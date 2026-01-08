use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
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
            "start" => EventType::Start,
            "end" => EventType::End,
            "permission" => EventType::Permission,
            other => EventType::Unknown(other.to_string()),
        }
    }
}

impl HookEvent {
    #[must_use]
    pub fn event_type(&self) -> EventType {
        EventType::from(self.event.as_str())
    }
}

/// Poll for new hook events from the events directory
/// Returns a list of events and deletes processed files
pub fn poll_events() -> Result<Vec<HookEvent>> {
    let events_dir = match std::env::current_dir() {
        Ok(cwd) => cwd.join(".chloe").join("events"),
        Err(_) => return Ok(Vec::new()),
    };

    if !events_dir.exists() {
        return Ok(Vec::new());
    }

    let mut events = Vec::new();

    let entries = std::fs::read_dir(&events_dir)
        .context("Failed to read events directory")?;

    for entry in entries.flatten() {
        let path = entry.path();

        if !path.is_file() || path.extension().and_then(|s| s.to_str()) != Some("json") {
            continue;
        }

        match read_and_delete_event_file(&path) {
            Ok(event) => events.push(event),
            Err(error) => {
                eprintln!("Warning: Failed to process event file {}: {}", path.display(), error);
            }
        }
    }

    Ok(events)
}

fn read_and_delete_event_file(path: &PathBuf) -> Result<HookEvent> {
    let content = std::fs::read_to_string(path)
        .context("Failed to read event file")?;

    let event: HookEvent = serde_json::from_str(&content)
        .context("Failed to parse event JSON")?;

    std::fs::remove_file(path)
        .context("Failed to delete event file")?;

    Ok(event)
}
