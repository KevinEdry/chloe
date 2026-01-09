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
mod common;
pub mod events;
mod helpers;
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
        None => run_tui(),
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

#[allow(clippy::too_many_lines)]
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
                    let instances_terminal_focused = app.active_tab == Tab::Instances
                        && app.instances.mode == views::instances::InstanceMode::Focused;
                    let tasks_terminal_focused =
                        app.active_tab == Tab::Tasks && app.tasks.is_terminal_focused();
                    let terminal_is_focused = instances_terminal_focused || tasks_terminal_focused;

                    let tasks_is_typing =
                        app.active_tab == Tab::Tasks && app.tasks.is_typing_mode();

                    if app.showing_exit_confirmation {
                        match key.code {
                            KeyCode::Char('y' | 'Y') => {
                                return Ok(());
                            }
                            KeyCode::Char('n' | 'N') | KeyCode::Esc => {
                                app.showing_exit_confirmation = false;
                            }
                            _ => {}
                        }
                        continue;
                    }

                    match key.code {
                        KeyCode::Char('q' | 'Q') => {
                            if !terminal_is_focused {
                                app.showing_exit_confirmation = true;
                            } else if instances_terminal_focused {
                                views::instances::events::handle_key_event(&mut app.instances, key);
                            } else {
                                polling::process_tasks_event(app, key);
                            }
                        }
                        KeyCode::Char('c')
                            if key.modifiers.contains(event::KeyModifiers::CONTROL) =>
                        {
                            if !terminal_is_focused {
                                app.showing_exit_confirmation = true;
                            } else if instances_terminal_focused {
                                views::instances::events::handle_key_event(&mut app.instances, key);
                            } else {
                                polling::process_tasks_event(app, key);
                            }
                        }
                        KeyCode::Tab | KeyCode::BackTab => {
                            if !terminal_is_focused {
                                match key.code {
                                    KeyCode::Tab => app.next_tab(),
                                    KeyCode::BackTab => app.previous_tab(),
                                    _ => {}
                                }
                            } else if instances_terminal_focused {
                                views::instances::events::handle_key_event(&mut app.instances, key);
                            } else {
                                polling::process_tasks_event(app, key);
                            }
                        }
                        KeyCode::Char('1') if !terminal_is_focused && !tasks_is_typing => {
                            app.switch_tab(Tab::Tasks);
                        }
                        KeyCode::Char('2') if !terminal_is_focused && !tasks_is_typing => {
                            app.switch_tab(Tab::Instances);
                        }
                        KeyCode::Char('3') if !terminal_is_focused && !tasks_is_typing => {
                            app.switch_tab(Tab::Roadmap);
                        }
                        KeyCode::Char('4') if !terminal_is_focused && !tasks_is_typing => {
                            app.switch_tab(Tab::Worktree);
                        }
                        _ => match app.active_tab {
                            Tab::Tasks => {
                                let is_normal_mode = app.tasks.is_normal_mode();
                                let is_jump_to_instance_key = key.code == KeyCode::Char('t')
                                    || key.code == KeyCode::Char('T');

                                if is_normal_mode && is_jump_to_instance_key {
                                    app.jump_to_task_instance();
                                } else {
                                    polling::process_tasks_event(app, key);
                                }
                            }
                            Tab::Instances => {
                                views::instances::events::handle_key_event(&mut app.instances, key);
                            }
                            Tab::Roadmap => {
                                let action =
                                    views::roadmap::events::handle_key_event(&mut app.roadmap, key);
                                polling::process_roadmap_action(app, &action);
                            }
                            Tab::Worktree => {
                                app.worktree.handle_key_event(key);
                                polling::process_worktree_pending_actions(app);
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
