use crate::events::AppEvent;
use alacritty_terminal::event::{Event, EventListener, OnResize, WindowSize};
use alacritty_terminal::grid::Dimensions;
use alacritty_terminal::term::{Config, Term};
use alacritty_terminal::tty::{self, Options, Pty, Shell};
use alacritty_terminal::vte::ansi::{Processor, StdSyncHandler};
use std::io::{Read, Write};
use std::path::Path;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use tokio::sync::mpsc;
use uuid::Uuid;

#[cfg(windows)]
use alacritty_terminal::tty::{EventedPty, EventedReadWrite};

const DEFAULT_SCROLLBACK_LINES: usize = 10000;
const READ_BUFFER_BYTES: usize = 4096;
const READ_POLL_DELAY_MS: u64 = 10;

pub struct EventProxy;

struct TerminalSize {
    columns: usize,
    screen_lines: usize,
}

impl Dimensions for TerminalSize {
    fn columns(&self) -> usize {
        self.columns
    }

    fn screen_lines(&self) -> usize {
        self.screen_lines
    }

    fn total_lines(&self) -> usize {
        self.screen_lines
    }
}

impl EventListener for EventProxy {
    fn send_event(&self, _event: Event) {}
}

pub struct PtySession {
    term: Arc<Mutex<Term<EventProxy>>>,
    #[cfg(unix)]
    pty: Pty,
    #[cfg(windows)]
    pty: Arc<Mutex<Pty>>,
}

pub struct SpawnOptions {
    pub pane_id: Uuid,
    pub working_directory: std::path::PathBuf,
    pub rows: u16,
    pub columns: u16,
    pub command: Option<String>,
    pub arguments: Vec<String>,
    pub environment: std::collections::HashMap<String, String>,
    pub event_sender: mpsc::UnboundedSender<AppEvent>,
}

impl SpawnOptions {
    #[must_use]
    pub fn new(
        pane_id: Uuid,
        working_directory: std::path::PathBuf,
        rows: u16,
        columns: u16,
        event_sender: mpsc::UnboundedSender<AppEvent>,
    ) -> Self {
        Self {
            pane_id,
            working_directory,
            rows,
            columns,
            command: None,
            arguments: Vec::new(),
            environment: std::collections::HashMap::new(),
            event_sender,
        }
    }

    #[must_use]
    pub fn with_command(mut self, command: String, arguments: Vec<String>) -> Self {
        self.command = Some(command);
        self.arguments = arguments;
        self
    }

    #[must_use]
    pub fn with_environment(
        mut self,
        environment: std::collections::HashMap<String, String>,
    ) -> Self {
        self.environment = environment;
        self
    }
}

impl PtySession {
    pub fn spawn(
        pane_id: Uuid,
        working_directory: &Path,
        rows: u16,
        columns: u16,
        event_sender: mpsc::UnboundedSender<AppEvent>,
    ) -> anyhow::Result<Self> {
        let options = SpawnOptions::new(
            pane_id,
            working_directory.to_path_buf(),
            rows,
            columns,
            event_sender,
        );
        Self::spawn_with_options(options)
    }

    pub fn spawn_with_options(options: SpawnOptions) -> anyhow::Result<Self> {
        tty::setup_env();

        let shell = options
            .command
            .clone()
            .map(|command| Shell::new(command, options.arguments.clone()));

        let tty_options = Options {
            shell,
            working_directory: Some(options.working_directory.clone()),
            env: options.environment.clone(),
            drain_on_exit: true,
            #[cfg(windows)]
            escape_args: false,
        };

        let window_size = WindowSize {
            cell_width: 1,
            cell_height: 1,
            num_cols: options.columns,
            num_lines: options.rows,
        };

        let pty = tty::new(&tty_options, window_size, 0)?;

        let config = Config {
            scrolling_history: DEFAULT_SCROLLBACK_LINES,
            ..Config::default()
        };

        let term_size = TerminalSize {
            columns: usize::from(options.columns),
            screen_lines: usize::from(options.rows),
        };
        let term = Term::new(config, &term_size, EventProxy);

        let term = Arc::new(Mutex::new(term));

        let pane_id = options.pane_id;
        let event_sender = options.event_sender;
        let term_for_thread = Arc::clone(&term);

        #[cfg(unix)]
        {
            let reader = pty.file().try_clone()?;
            spawn_unix_reader_thread(reader, pane_id, event_sender, term_for_thread);
            Ok(Self { term, pty })
        }

        #[cfg(windows)]
        {
            let pty = Arc::new(Mutex::new(pty));
            let pty_for_thread = Arc::clone(&pty);
            spawn_windows_reader_thread(pty_for_thread, pane_id, event_sender, term_for_thread);
            Ok(Self { term, pty })
        }
    }

    #[allow(clippy::needless_pass_by_ref_mut)]
    pub fn resize(&mut self, rows: u16, columns: u16) {
        let window_size = WindowSize {
            cell_width: 1,
            cell_height: 1,
            num_cols: columns,
            num_lines: rows,
        };

        #[cfg(unix)]
        self.pty.on_resize(window_size);

        #[cfg(windows)]
        if let Ok(mut pty) = self.pty.lock() {
            pty.on_resize(window_size);
        }

        if let Ok(mut term) = self.term.lock() {
            let term_size = TerminalSize {
                columns: usize::from(columns),
                screen_lines: usize::from(rows),
            };
            term.resize(term_size);
        }
    }

    #[must_use]
    pub fn term(&self) -> Arc<Mutex<Term<EventProxy>>> {
        Arc::clone(&self.term)
    }

    pub fn write_input(&self, data: &[u8]) -> anyhow::Result<()> {
        #[cfg(unix)]
        {
            let mut writer = self.pty.file().try_clone()?;
            writer.write_all(data)?;
            writer.flush()?;
        }

        #[cfg(windows)]
        {
            let mut pty = self
                .pty
                .lock()
                .map_err(|error| anyhow::anyhow!("PTY lock poisoned: {error}"))?;
            pty.writer().write_all(data)?;
            pty.writer().flush()?;
        }

        Ok(())
    }
}

#[cfg(unix)]
fn spawn_unix_reader_thread(
    reader: std::fs::File,
    pane_id: Uuid,
    event_sender: mpsc::UnboundedSender<AppEvent>,
    term: Arc<Mutex<Term<EventProxy>>>,
) {
    thread::spawn(move || {
        let mut reader = reader;
        let mut buffer = [0u8; READ_BUFFER_BYTES];
        let mut processor: Processor<StdSyncHandler> = Processor::new();

        loop {
            match reader.read(&mut buffer) {
                Ok(0) => {
                    let _ = event_sender.send(AppEvent::PtyExit { pane_id });
                    break;
                }
                Ok(bytes_read) => {
                    let data = buffer[..bytes_read].to_vec();

                    if let Ok(mut term) = term.lock() {
                        processor.advance(&mut *term, &data);
                    }

                    if event_sender
                        .send(AppEvent::PtyOutput { pane_id, data })
                        .is_err()
                    {
                        break;
                    }
                }
                Err(error) if error.kind() == std::io::ErrorKind::WouldBlock => {
                    thread::sleep(Duration::from_millis(READ_POLL_DELAY_MS));
                }
                Err(_) => {
                    let _ = event_sender.send(AppEvent::PtyExit { pane_id });
                    break;
                }
            }
        }
    });
}

#[cfg(windows)]
fn spawn_windows_reader_thread(
    pty: Arc<Mutex<Pty>>,
    pane_id: Uuid,
    event_sender: mpsc::UnboundedSender<AppEvent>,
    term: Arc<Mutex<Term<EventProxy>>>,
) {
    thread::spawn(move || {
        let mut buffer = [0u8; READ_BUFFER_BYTES];
        let mut processor: Processor<StdSyncHandler> = Processor::new();

        loop {
            let read_result = {
                let Ok(mut pty) = pty.lock() else {
                    break;
                };

                if let Some(alacritty_terminal::tty::ChildEvent::Exited(_)) =
                    pty.next_child_event()
                {
                    let _ = event_sender.send(AppEvent::PtyExit { pane_id });
                    break;
                }

                pty.reader().read(&mut buffer)
            };

            match read_result {
                Ok(0) => {
                    thread::sleep(Duration::from_millis(READ_POLL_DELAY_MS));
                }
                Ok(bytes_read) => {
                    let data = buffer[..bytes_read].to_vec();

                    if let Ok(mut term) = term.lock() {
                        processor.advance(&mut *term, &data);
                    }

                    if event_sender
                        .send(AppEvent::PtyOutput { pane_id, data })
                        .is_err()
                    {
                        break;
                    }
                }
                Err(error) if error.kind() == std::io::ErrorKind::WouldBlock => {
                    thread::sleep(Duration::from_millis(READ_POLL_DELAY_MS));
                }
                Err(_) => {
                    let _ = event_sender.send(AppEvent::PtyExit { pane_id });
                    break;
                }
            }
        }
    });
}

impl std::fmt::Debug for PtySession {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.debug_struct("PtySession").finish()
    }
}

impl Clone for PtySession {
    fn clone(&self) -> Self {
        panic!("PtySession cannot be cloned")
    }
}
