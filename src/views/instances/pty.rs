use portable_pty::{Child, CommandBuilder, MasterPty, NativePtySystem, PtySize, PtySystem};
use std::io::{Read, Write};
use std::path::Path;
use std::sync::mpsc::{Receiver, channel};
use std::sync::{Arc, Mutex};
use std::thread;

pub struct PtySession {
    parser: Arc<Mutex<vt100::Parser>>,
    master: Box<dyn MasterPty + Send>,
    writer: Box<dyn Write + Send>,
    receiver: Receiver<Vec<u8>>,
    child: Option<Box<dyn Child + Send>>,
}

const SCROLLBACK_LINES: usize = 1000;

impl PtySession {
    pub fn spawn(working_directory: &Path, rows: u16, columns: u16) -> anyhow::Result<Self> {
        let pty_system = NativePtySystem::default();
        let pty_size = PtySize {
            rows,
            cols: columns,
            pixel_width: 0,
            pixel_height: 0,
        };

        let pair = pty_system.openpty(pty_size)?;

        let shell = if cfg!(target_os = "windows") {
            "cmd.exe".to_string()
        } else {
            std::env::var("SHELL").unwrap_or_else(|_| "/bin/sh".to_string())
        };

        let mut command = CommandBuilder::new(shell);
        command.cwd(working_directory);

        let child = pair.slave.spawn_command(command)?;
        drop(pair.slave);

        let writer = pair.master.take_writer()?;
        let mut reader = pair.master.try_clone_reader()?;
        let (sender, receiver) = channel();

        thread::spawn(move || {
            let mut buffer = [0u8; 4096];
            loop {
                match reader.read(&mut buffer) {
                    Ok(0) | Err(_) => break,
                    Ok(bytes_read) => {
                        if sender.send(buffer[..bytes_read].to_vec()).is_err() {
                            break;
                        }
                    }
                }
            }
        });

        let parser = Arc::new(Mutex::new(vt100::Parser::new(
            rows,
            columns,
            SCROLLBACK_LINES,
        )));

        Ok(Self {
            parser,
            master: pair.master,
            writer,
            receiver,
            child: Some(child),
        })
    }

    pub fn resize(&self, rows: u16, columns: u16) -> anyhow::Result<()> {
        let pty_size = PtySize {
            rows,
            cols: columns,
            pixel_width: 0,
            pixel_height: 0,
        };
        self.master.resize(pty_size)?;

        if let Ok(mut parser) = self.parser.lock() {
            parser.set_size(rows, columns);
        }

        Ok(())
    }

    pub fn read_output(&self) {
        while let Ok(data) = self.receiver.try_recv() {
            if let Ok(mut parser) = self.parser.lock() {
                parser.process(&data);
            }
        }
    }

    #[must_use]
    pub fn screen(&self) -> Arc<Mutex<vt100::Parser>> {
        Arc::clone(&self.parser)
    }

    #[must_use]
    pub fn scrollback_len(&self) -> usize {
        self.parser
            .lock()
            .map_or(0, |parser| parser.screen().scrollback())
    }

    pub fn write_input(&mut self, data: &[u8]) -> anyhow::Result<()> {
        self.writer.write_all(data)?;
        self.writer.flush()?;
        Ok(())
    }

    pub fn check_process_exit(&mut self) -> bool {
        if let Some(child) = &mut self.child {
            match child.try_wait() {
                Ok(Some(_exit_status)) => {
                    self.child = None;
                    true
                }
                Ok(None) => false,
                Err(_) => {
                    self.child = None;
                    true
                }
            }
        } else {
            false
        }
    }
}

impl std::fmt::Debug for PtySession {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PtySession").finish()
    }
}

impl Clone for PtySession {
    fn clone(&self) -> Self {
        panic!("PtySession cannot be cloned")
    }
}
