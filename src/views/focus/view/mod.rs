mod details_panel;
mod task_list;
mod terminal_panel;

use super::operations::get_ordered_tasks;
use super::state::FocusMode;
use crate::app::App;
use crate::views::StatusBarContent;
use crate::widgets::dialogs;
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::Color,
};

const LEFT_PANEL_PERCENT: u16 = 35;
const RIGHT_PANEL_PERCENT: u16 = 65;
const DETAILS_PANEL_PERCENT: u16 = 40;
const TERMINAL_PANEL_PERCENT: u16 = 60;
const STATUS_BAR_WIDTH_THRESHOLD: u16 = 80;

pub fn render(frame: &mut Frame, app: &App, area: Rect) {
    let horizontal_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(LEFT_PANEL_PERCENT),
            Constraint::Percentage(RIGHT_PANEL_PERCENT),
        ])
        .split(area);

    task_list::render(frame, app, horizontal_chunks[0]);

    let right_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(DETAILS_PANEL_PERCENT),
            Constraint::Percentage(TERMINAL_PANEL_PERCENT),
        ])
        .split(horizontal_chunks[1]);

    details_panel::render(
        frame,
        &app.kanban.columns,
        app.focus.selected_index,
        right_chunks[0],
    );

    let tasks = get_ordered_tasks(&app.kanban.columns);
    let selected_task = tasks.get(app.focus.selected_index);
    let instance_pane = selected_task
        .and_then(|task_ref| task_ref.task.instance_id)
        .and_then(|instance_id| {
            app.instances
                .panes
                .iter()
                .find(|pane| pane.id == instance_id)
        });

    let is_terminal_focused = matches!(app.focus.mode, FocusMode::TerminalFocused);
    terminal_panel::render(frame, instance_pane, is_terminal_focused, right_chunks[1]);

    render_dialogs(frame, &app.focus.mode, area);
}

fn render_dialogs(frame: &mut Frame, mode: &FocusMode, area: Rect) {
    match mode {
        FocusMode::AddingTask { input } => {
            dialogs::render_input_dialog(frame, "Add Task", input, area);
        }
        FocusMode::EditingTask { input, .. } => {
            dialogs::render_input_dialog(frame, "Edit Task", input, area);
        }
        FocusMode::ConfirmDelete { .. } => {
            dialogs::render_confirm_dialog(
                frame,
                "Delete Task",
                "Are you sure? (y/n)",
                area,
            );
        }
        FocusMode::ConfirmStartTask { .. } => {
            dialogs::render_confirm_dialog(
                frame,
                "Start Task",
                "Move task to In Progress? (y/n)",
                area,
            );
        }
        FocusMode::Normal | FocusMode::TerminalFocused => {}
    }
}

pub fn get_status_bar_content(app: &App, width: u16) -> StatusBarContent {
    let state = &app.focus;

    let (mode_text, mode_color) = match &state.mode {
        FocusMode::Normal => ("NAVIGATE", Color::Cyan),
        FocusMode::TerminalFocused => ("TERMINAL", Color::Green),
        FocusMode::AddingTask { .. } => ("ADD TASK", Color::Yellow),
        FocusMode::EditingTask { .. } => ("EDIT TASK", Color::Yellow),
        FocusMode::ConfirmDelete { .. } => ("DELETE", Color::Red),
        FocusMode::ConfirmStartTask { .. } => ("START", Color::Green),
    };

    let total_tasks: usize = app.kanban.columns.iter().map(|c| c.tasks.len()).sum();

    let help_text = if width < STATUS_BAR_WIDTH_THRESHOLD {
        match &state.mode {
            FocusMode::Normal => "jk:nav  a:add  e:edit  d:del  s:start  t:jump",
            FocusMode::TerminalFocused => "Esc:back  Keys→terminal",
            FocusMode::AddingTask { .. } | FocusMode::EditingTask { .. } => "Enter:save  Esc:cancel",
            FocusMode::ConfirmDelete { .. } | FocusMode::ConfirmStartTask { .. } => "y:confirm  n:cancel",
        }
    } else {
        match &state.mode {
            FocusMode::Normal => "↑↓/jk:navigate  a:add  e:edit  d:delete  s:start  Enter:focus-terminal  t:jump-to-instances",
            FocusMode::TerminalFocused => "All keys sent to terminal  Esc:back-to-navigation",
            FocusMode::AddingTask { .. } | FocusMode::EditingTask { .. } => "Type task title  Enter:save  Esc:cancel",
            FocusMode::ConfirmDelete { .. } | FocusMode::ConfirmStartTask { .. } => "Press y to confirm, n or Esc to cancel",
        }
    };

    StatusBarContent {
        mode_text: mode_text.to_string(),
        mode_color,
        extra_info: Some(format!("Tasks: {}  ", total_tasks)),
        help_text: help_text.to_string(),
    }
}
