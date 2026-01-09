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
#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

mod app;
mod cli;
mod common;
pub mod events;
mod persistence;
mod polling;
mod types;
mod views;
mod widgets;

use app::{App, Tab};
use clap::Parser;
use cli::{Cli, Commands};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{Terminal, backend::CrosstermBackend};
use std::io;

fn main() -> Result<(), io::Error> {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Notify {
            event_type,
            worktree_id,
        }) => {
            if let Err(error) = cli::handle_notify_command(event_type, worktree_id) {
                eprintln!("Error handling notify command: {error}");
                std::process::exit(1);
            }
            return Ok(());
        }
        None => {
            return run_tui();
        }
    }
}

fn run_tui() -> Result<(), io::Error> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let event_listener = events::EventListener::start()?;
    let mut app = App::load_or_default();
    let res = run_app(&mut terminal, &mut app, &event_listener);

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(save_err) = app.save() {
        eprintln!("Warning: Failed to save state: {save_err}");
    }

    if let Err(err) = res {
        println!("Error: {err:?}");
    }

    Ok(())
}

fn run_app<B: ratatui::backend::Backend>(
    terminal: &mut Terminal<B>,
    app: &mut App,
    event_listener: &events::EventListener,
) -> io::Result<()> {
    loop {
        terminal.draw(|f| views::render(f, app))?;

        polling::poll_background_tasks(app, event_listener);

        if event::poll(std::time::Duration::from_millis(100))? {
            match event::read()? {
                Event::Key(key) => {
                    let instance_is_focused = app.active_tab == Tab::Instances
                        && app.instances.mode == views::instances::InstanceMode::Focused;

                    if app.showing_exit_confirmation {
                        match key.code {
                            KeyCode::Char('y') | KeyCode::Char('Y') => {
                                return Ok(());
                            }
                            KeyCode::Char('n') | KeyCode::Char('N') | KeyCode::Esc => {
                                app.showing_exit_confirmation = false;
                            }
                            _ => {}
                        }
                        continue;
                    }

                    match key.code {
                        KeyCode::Char('q') | KeyCode::Char('Q') => {
                            if !instance_is_focused {
                                app.showing_exit_confirmation = true;
                            } else {
                                views::instances::events::handle_key_event(&mut app.instances, key);
                            }
                        }
                        KeyCode::Char('c')
                            if key.modifiers.contains(event::KeyModifiers::CONTROL) =>
                        {
                            if !instance_is_focused {
                                app.showing_exit_confirmation = true;
                            } else {
                                views::instances::events::handle_key_event(&mut app.instances, key);
                            }
                        }
                        KeyCode::Tab | KeyCode::BackTab => {
                            if !instance_is_focused && key.code == KeyCode::Tab {
                                app.next_tab();
                            } else {
                                views::instances::events::handle_key_event(&mut app.instances, key);
                            }
                        }
                        KeyCode::Char('1') if !instance_is_focused => {
                            app.switch_tab(Tab::Kanban);
                        }
                        KeyCode::Char('2') if !instance_is_focused => {
                            app.switch_tab(Tab::Instances);
                        }
                        KeyCode::Char('3') if !instance_is_focused => {
                            app.switch_tab(Tab::Roadmap);
                        }
                        KeyCode::Char('4') if !instance_is_focused => {
                            app.switch_tab(Tab::Worktree);
                        }
                        KeyCode::Char('5') if !instance_is_focused => {
                            app.switch_tab(Tab::Focus);
                        }
                        _ => match app.active_tab {
                            Tab::Kanban => {
                                let is_normal_mode =
                                    app.kanban.mode == views::kanban::KanbanMode::Normal;
                                let is_jump_to_instance_key = key.code == KeyCode::Char('t')
                                    || key.code == KeyCode::Char('T');

                                if is_normal_mode && is_jump_to_instance_key {
                                    app.jump_to_task_instance();
                                } else {
                                    views::kanban::events::handle_key_event(&mut app.kanban, key);
                                    polling::process_kanban_pending_actions(app);
                                }
                            }
                            Tab::Instances => {
                                views::instances::events::handle_key_event(&mut app.instances, key);
                            }
                            Tab::Roadmap => {
                                let action =
                                    views::roadmap::events::handle_key_event(&mut app.roadmap, key);
                                polling::process_roadmap_action(app, action);
                            }
                            Tab::Worktree => {
                                app.worktree.handle_key_event(key);
                                polling::process_worktree_pending_actions(app);
                            }
                            Tab::Focus => {
                                polling::process_focus_event(app, key);
                            }
                        },
                    }
                }
                Event::Mouse(mouse_event) => {
                    if app.active_tab == Tab::Instances {
                        views::instances::events::handle_mouse_event(
                            &mut app.instances,
                            mouse_event,
                        );
                    }
                }
                _ => {}
            }
        }
    }
}
