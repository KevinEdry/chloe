//! Chloe - Auto Claude CLI
//!
//! # Safety Policy
//!
//! This project maintains a **STRICT NO UNSAFE CODE** policy:
//! - No `unsafe` blocks anywhere in the codebase
//! - No unsafe threading patterns
//! - All dependencies must use safe Rust APIs
//! - Static analysis enforces this via `#![forbid(unsafe_code)]`
//!
//! This ensures memory safety, thread safety, and eliminates entire classes of bugs.

#![forbid(unsafe_code)]

mod app;
mod cli;
pub mod events;
mod helpers;
mod persistence;
mod providers;
mod types;
mod views;
mod widgets;

use app::App;
use clap::Parser;
use cli::{Cli, Commands};
use crossterm::{
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use events::EventLoop;
use ratatui::{Terminal, backend::CrosstermBackend};
use std::io;

#[tokio::main]
async fn main() -> Result<(), io::Error> {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Init) => {
            if let Err(error) = cli::handle_init_command() {
                eprintln!("Error: {error}");
                std::process::exit(1);
            }
            Ok(())
        }
        Some(Commands::Notify {
            event_type,
            worktree_id,
        }) => {
            if let Err(error) = cli::handle_notify_command(event_type, worktree_id) {
                eprintln!("Error handling notify command: {error}");
                std::process::exit(1);
            }
            Ok(())
        }
        None => run_tui().await,
    }
}

#[allow(clippy::future_not_send)]
async fn run_tui() -> Result<(), io::Error> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::load_or_default();
    let mut event_loop = EventLoop::new();

    app.set_event_sender(event_loop.event_sender());
    let _event_listener = events::EventListener::start(app.event_sender())?;

    let result = event_loop.run(&mut terminal, &mut app).await;

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    if let Err(save_error) = app.save() {
        eprintln!("Warning: Failed to save state: {save_error}");
    }

    if let Err(error) = result {
        println!("Error: {error:?}");
    }

    Ok(())
}
