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

pub fn handle_notify_command(event_type: String, worktree_id: Uuid) -> Result<(), String> {
    let mut hook_data = String::new();
    std::io::stdin()
        .read_to_string(&mut hook_data)
        .map_err(|error| format!("Failed to read hook data from stdin: {error}"))?;

    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_err(|error| format!("System time error: {error}"))?
        .as_nanos();

    let hook_data_value = serde_json::from_str::<serde_json::Value>(&hook_data)
        .unwrap_or_else(|_| serde_json::Value::String(hook_data));

    let event = crate::events::HookEvent {
        event: event_type,
        worktree_id,
        timestamp,
        hook_data: hook_data_value,
    };

    // Silently ignore errors - Chloe TUI may not be running
    let _ = crate::events::send_event(&event);

    Ok(())
}
