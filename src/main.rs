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
#![warn(clippy::magic_numbers)]

mod app;
mod common;
mod instance;
mod kanban;
mod persistence;
mod types;
mod ui;

use app::{App, Tab};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{Terminal, backend::CrosstermBackend};
use std::io;

fn main() -> Result<(), io::Error> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Load state from disk
    let mut app = App::load_or_default();
    let res = run_app(&mut terminal, &mut app);

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    // Save state before exiting
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
) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui::render(f, app))?;

        // Poll for AI classification completion on every loop iteration
        if app.active_tab == Tab::Kanban {
            app.kanban.poll_classification();
        }

        // Poll for instance PTY output on every loop iteration
        // Always poll instances, not just when tab is active, to catch output from background instances
        app.instances.poll_pty_output();

        // Auto-transition completed tasks from In Progress to Review
        app.auto_transition_completed_tasks();

        if event::poll(std::time::Duration::from_millis(100))? {
            match event::read()? {
                Event::Key(key) => {
                    // Check if instance is focused - if so, don't catch Ctrl+C globally
                    let instance_is_focused = app.active_tab == Tab::Instances
                        && app.instances.mode == instance::InstanceMode::Focused;

                    // Global keybindings
                    match key.code {
                        KeyCode::Char('q') | KeyCode::Char('Q') => {
                            if !instance_is_focused {
                                return Ok(());
                            }
                            instance::events::handle_key_event(&mut app.instances, key);
                        }
                        KeyCode::Char('c')
                            if key.modifiers.contains(event::KeyModifiers::CONTROL) =>
                        {
                            if !instance_is_focused {
                                return Ok(());
                            }
                            instance::events::handle_key_event(&mut app.instances, key);
                        }
                        KeyCode::Tab | KeyCode::BackTab => {
                            if !instance_is_focused && key.code == KeyCode::Tab {
                                app.next_tab();
                            } else {
                                instance::events::handle_key_event(&mut app.instances, key);
                            }
                        }
                        KeyCode::Char('1') if !instance_is_focused => {
                            app.switch_tab(Tab::Kanban);
                        }
                        KeyCode::Char('2') if !instance_is_focused => {
                            app.switch_tab(Tab::Instances);
                        }
                        _ => {
                            // Route to active tab
                            match app.active_tab {
                                Tab::Kanban => {
                                    // Handle 'T' key to jump to task instance, but only in Normal mode
                                    let is_normal_mode =
                                        app.kanban.mode == kanban::KanbanMode::Normal;
                                    if is_normal_mode
                                        && (key.code == KeyCode::Char('t')
                                            || key.code == KeyCode::Char('T'))
                                    {
                                        app.jump_to_task_instance();
                                    } else {
                                        kanban::events::handle_key_event(&mut app.kanban, key);

                                        // Check if an instance needs to be terminated
                                        if let Some(instance_id) =
                                            app.kanban.pending_instance_termination.take()
                                        {
                                            app.instances.close_pane_by_id(instance_id);
                                        }

                                        // Handle pending IDE open action
                                        if let Some(task_idx) = app.kanban.pending_ide_open.take() {
                                            app.open_task_in_ide(task_idx);
                                        }

                                        // Handle pending terminal switch action
                                        if let Some(task_idx) =
                                            app.kanban.pending_terminal_switch.take()
                                        {
                                            app.switch_to_task_instance(task_idx);
                                        }

                                        // Handle pending change request
                                        if let Some((task_idx, change_request)) =
                                            app.kanban.pending_change_request.take()
                                        {
                                            let instance_id =
                                                app.kanban.move_task_to_in_progress(task_idx);
                                            if let Some(instance_id) = instance_id {
                                                app.instances.send_input_to_instance(
                                                    instance_id,
                                                    &change_request,
                                                );
                                            }
                                        }

                                        // Auto-create instances for tasks in "In Progress"
                                        app.sync_task_instances();
                                    }
                                }
                                Tab::Instances => {
                                    instance::events::handle_key_event(&mut app.instances, key)
                                }
                            }
                        }
                    }
                }
                Event::Mouse(mouse_event) => {
                    if app.active_tab == Tab::Instances {
                        instance::events::handle_mouse_event(&mut app.instances, mouse_event);
                    }
                }
                _ => {}
            }
        }
    }
}
