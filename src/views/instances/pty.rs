use alacritty_terminal::event::{Event, EventListener, OnResize, WindowSize};
use alacritty_terminal::tty::EventedPty;
use alacritty_terminal::grid::Dimensions;
use alacritty_terminal::term::{Config, Term};
use alacritty_terminal::tty::{self, Options, Pty};
use alacritty_terminal::vte::ansi::Processor;
use std::io::{Read, Write};
use std::path::Path;
use std::sync::mpsc::{Receiver, channel};
use std::sync::{Arc, Mutex};
use std::thread;

const DEFAULT_SCROLLBACK_LINES: usize = 10000;

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
    processor: Arc<Mutex<Processor>>,
    pty: Pty,
    receiver: Receiver<Vec<u8>>,
}

impl PtySession {
    pub fn spawn(working_directory: &Path, rows: u16, columns: u16) -> anyhow::Result<Self> {
        let options = Options {
            shell: None,
            working_directory: Some(working_directory.to_path_buf()),
            env: std::collections::HashMap::new(),
            drain_on_exit: true,
        };

        let window_size = WindowSize {
            cell_width: 1,
            cell_height: 1,
            num_cols: columns,
            num_lines: rows,
        };

        let pty = tty::new(&options, window_size, 0)?;

        let config = Config {
            scrolling_history: DEFAULT_SCROLLBACK_LINES,
            ..Config::default()
        };

        let term_size = TerminalSize {
            columns: usize::from(columns),
            screen_lines: usize::from(rows),
        };
        let term = Term::new(config, &term_size, EventProxy);

        let term = Arc::new(Mutex::new(term));
        let processor = Arc::new(Mutex::new(Processor::new()));

        let (sender, receiver) = channel();
        let reader = pty.file().try_clone()?;

        thread::spawn(move || {
            let mut reader = reader;
            let mut buffer = [0u8; 4096];
            loop {
                match reader.read(&mut buffer) {
                    Ok(0) => break,
                    Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                        thread::sleep(std::time::Duration::from_millis(10));
                        continue;
                    }
                    Err(_) => break,
                    Ok(bytes_read) => {
                        if sender.send(buffer[..bytes_read].to_vec()).is_err() {
                            break;
                        }
                    }
                }
            }
        });

        Ok(Self {
            term,
            processor,
            pty,
            receiver,
        })
    }

    pub fn resize(&mut self, rows: u16, columns: u16) {
        let window_size = WindowSize {
            cell_width: 1,
            cell_height: 1,
            num_cols: columns,
            num_lines: rows,
        };
        self.pty.on_resize(window_size);

        if let Ok(mut term) = self.term.lock() {
            let term_size = TerminalSize {
                columns: usize::from(columns),
                screen_lines: usize::from(rows),
            };
            term.resize(term_size);
        }
    }

    pub fn read_output(&self) {
        while let Ok(data) = self.receiver.try_recv() {
            let Ok(mut term) = self.term.lock() else {
                continue;
            };
            let Ok(mut processor) = self.processor.lock() else {
                continue;
            };
            processor.advance(&mut *term, &data);
        }
    }

    #[must_use]
    pub fn term(&self) -> Arc<Mutex<Term<EventProxy>>> {
        Arc::clone(&self.term)
    }

    pub fn write_input(&self, data: &[u8]) -> anyhow::Result<()> {
        let mut writer = self.pty.file().try_clone()?;
        writer.write_all(data)?;
        writer.flush()?;
        Ok(())
    }

    pub fn check_process_exit(&mut self) -> bool {
        matches!(
            self.pty.next_child_event(),
            Some(alacritty_terminal::tty::ChildEvent::Exited(_))
        )
    }
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
