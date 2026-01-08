#![allow(clippy::all, clippy::pedantic, clippy::nursery, clippy::restriction)]

use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use std::io::Read;
use uuid::Uuid;

#[derive(Parser)]
#[command(name = "chloe")]
#[command(about = "Auto Claude CLI - Task management with Claude Code integration")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Handle Claude Code hook events (internal use)
    Notify {
        /// Event type: start, end, permission
        event_type: String,

        /// Worktree ID associated with this event
        #[arg(long)]
        worktree_id: Uuid,
    },
}

/// Handle the notify subcommand
/// Reads hook data from stdin and writes an event file to ~/.chloe/events/
pub fn handle_notify_command(event_type: String, worktree_id: Uuid) -> Result<()> {
    let mut hook_data = String::new();
    std::io::stdin()
        .read_to_string(&mut hook_data)
        .context("Failed to read hook data from stdin")?;

    let events_dir = std::env::current_dir()
        .context("Could not determine current directory")?
        .join(".chloe")
        .join("events");

    std::fs::create_dir_all(&events_dir).context("Failed to create events directory")?;

    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .context("System time error")?
        .as_nanos();

    let unique_id = Uuid::new_v4();
    let event_file = events_dir.join(format!("{}-{}-{}.json", worktree_id, timestamp, unique_id));

    let event_data = serde_json::json!({
        "event": event_type,
        "worktree_id": worktree_id,
        "timestamp": timestamp,
        "hook_data": serde_json::from_str::<serde_json::Value>(&hook_data)
            .unwrap_or_else(|_| serde_json::Value::String(hook_data.clone()))
    });

    std::fs::write(&event_file, serde_json::to_string_pretty(&event_data)?)
        .context("Failed to write event file")?;

    Ok(())
}
