use super::AppEvent;
use serde::{Deserialize, Serialize};
use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::path::PathBuf;
use std::thread;
use tokio::sync::mpsc;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HookEvent {
    pub event: String,
    pub worktree_id: Uuid,
    pub timestamp: u128,
    #[serde(default)]
    pub hook_data: serde_json::Value,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EventType {
    Start,
    End,
    Permission,
    Unknown(String),
}

impl From<&str> for EventType {
    fn from(string: &str) -> Self {
        match string {
            "start" => Self::Start,
            "end" => Self::End,
            "permission" => Self::Permission,
            other => Self::Unknown(other.to_string()),
        }
    }
}

impl HookEvent {
    #[must_use]
    pub fn event_type(&self) -> EventType {
        EventType::from(self.event.as_str())
    }
}

#[must_use]
pub fn get_port_file_path() -> PathBuf {
    std::env::temp_dir().join("chloe.port")
}

pub struct EventListener {
    _marker: (),
}

impl EventListener {
    pub fn start(event_sender: Option<mpsc::UnboundedSender<AppEvent>>) -> std::io::Result<Self> {
        let listener = TcpListener::bind("127.0.0.1:0")?;
        let local_port = listener.local_addr()?.port();
        listener.set_nonblocking(true)?;

        let port_file_path = get_port_file_path();
        std::fs::write(&port_file_path, local_port.to_string())?;

        thread::spawn(move || {
            run_listener(&listener, event_sender.as_ref());
        });

        Ok(Self { _marker: () })
    }
}

impl Drop for EventListener {
    fn drop(&mut self) {
        let port_file_path = get_port_file_path();
        let _ = std::fs::remove_file(port_file_path);
    }
}

fn run_listener(listener: &TcpListener, event_sender: Option<&mpsc::UnboundedSender<AppEvent>>) {
    loop {
        match listener.accept() {
            Ok((stream, _)) => {
                handle_connection(stream, event_sender);
            }
            Err(error) if error.kind() == std::io::ErrorKind::WouldBlock => {
                thread::sleep(std::time::Duration::from_millis(50));
            }
            Err(_) => {
                thread::sleep(std::time::Duration::from_millis(100));
            }
        }
    }
}

fn handle_connection(stream: TcpStream, event_sender: Option<&mpsc::UnboundedSender<AppEvent>>) {
    let Some(sender) = event_sender else {
        return;
    };

    let reader = BufReader::new(stream);

    for line in reader.lines() {
        let Ok(line) = line else {
            break;
        };

        if line.is_empty() {
            continue;
        }

        if let Ok(event) = serde_json::from_str::<HookEvent>(&line) {
            let _ = sender.send(AppEvent::HookReceived(event));
        }
    }
}

pub fn send_event(event: &HookEvent) -> std::io::Result<()> {
    let port_file_path = get_port_file_path();
    let port_string = std::fs::read_to_string(&port_file_path)?;
    let port: u16 = port_string
        .trim()
        .parse()
        .map_err(|error| std::io::Error::new(std::io::ErrorKind::InvalidData, error))?;

    let mut stream = TcpStream::connect(("127.0.0.1", port))?;

    let json = serde_json::to_string(event)
        .map_err(|error| std::io::Error::new(std::io::ErrorKind::InvalidData, error))?;

    writeln!(stream, "{json}")?;
    stream.flush()?;

    Ok(())
}
