mod details_panel;
mod done_tasks;
mod review_dialog;
mod task_list;
mod terminal_panel;

use super::operations::{get_active_tasks, get_done_tasks};
use super::state::{FocusMode, FocusPanel};
use crate::app::App;
use crate::views::StatusBarContent;
use crate::widgets::dialogs::{ConfirmDialog, DialogStyle, InputDialog, LoadingDialog};
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::Color,
};

const LEFT_PANEL_PERCENT: u16 = 35;
const RIGHT_PANEL_PERCENT: u16 = 65;
const ACTIVE_TASKS_PANEL_PERCENT: u16 = 65;
const DONE_TASKS_PANEL_PERCENT: u16 = 35;
const DETAILS_PANEL_PERCENT: u16 = 20;
const TERMINAL_PANEL_PERCENT: u16 = 80;
const STATUS_BAR_WIDTH_THRESHOLD: u16 = 80;

pub fn render(frame: &mut Frame, app: &mut App, area: Rect) {
    let horizontal_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(LEFT_PANEL_PERCENT),
            Constraint::Percentage(RIGHT_PANEL_PERCENT),
        ])
        .split(area);

    let left_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(ACTIVE_TASKS_PANEL_PERCENT),
            Constraint::Percentage(DONE_TASKS_PANEL_PERCENT),
        ])
        .split(horizontal_chunks[0]);

    task_list::render(frame, app, left_chunks[0]);
    done_tasks::render(frame, app, left_chunks[1]);

    let right_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(DETAILS_PANEL_PERCENT),
            Constraint::Percentage(TERMINAL_PANEL_PERCENT),
        ])
        .split(horizontal_chunks[1]);

    let selected_task = get_selected_task(app);
    details_panel::render(frame, selected_task.as_ref(), right_chunks[0]);

    let instance_id = selected_task.and_then(|task_ref| task_ref.task.instance_id);
    let instance_pane = instance_id.and_then(|id| {
        app.instances.panes.iter_mut().find(|pane| pane.id == id)
    });

    let is_terminal_focused = matches!(app.focus.mode, FocusMode::TerminalFocused);
    terminal_panel::render(frame, instance_pane, is_terminal_focused, right_chunks[1]);

    render_dialogs(frame, app, &app.focus.mode, area);
}

fn get_selected_task(app: &App) -> Option<super::operations::TaskReference<'_>> {
    match app.focus.focused_panel {
        FocusPanel::ActiveTasks => {
            let tasks = get_active_tasks(&app.kanban.columns);
            tasks.into_iter().nth(app.focus.active_selected_index)
        }
        FocusPanel::DoneTasks => {
            let tasks = get_done_tasks(&app.kanban.columns);
            tasks.into_iter().nth(app.focus.done_selected_index)
        }
    }
}

fn render_dialogs(frame: &mut Frame, app: &App, mode: &FocusMode, area: Rect) {
    match mode {
        FocusMode::AddingTask { input } => {
            frame.render_widget(InputDialog::new("Add Task", input), area);
        }
        FocusMode::EditingTask { input, .. } => {
            frame.render_widget(InputDialog::new("Edit Task", input), area);
        }
        FocusMode::ConfirmDelete { .. } => {
            frame.render_widget(
                ConfirmDialog::new("Delete Task", "Are you sure? (y/n)")
                    .style(DialogStyle::Danger),
                area,
            );
        }
        FocusMode::ConfirmStartTask { .. } => {
            frame.render_widget(
                ConfirmDialog::new("Start Task", "Move task to In Progress? (y/n)")
                    .style(DialogStyle::Success),
                area,
            );
        }
        FocusMode::ClassifyingTask { raw_input } => {
            frame.render_widget(LoadingDialog::new("Loading", raw_input), area);
        }
        FocusMode::ReviewPopup {
            task_id,
            scroll_offset,
            selected_action,
        } => {
            review_dialog::render(frame, app, *task_id, *scroll_offset, *selected_action, area);
        }
        FocusMode::ReviewRequestChanges { input, .. } => {
            frame.render_widget(InputDialog::new("Request Changes", input), area);
        }
        FocusMode::Normal | FocusMode::TerminalFocused => {}
    }
}

pub fn get_status_bar_content(app: &App, width: u16) -> StatusBarContent {
    let state = &app.focus;

    let (mode_text, mode_color) = match &state.mode {
        FocusMode::Normal => match state.focused_panel {
            FocusPanel::ActiveTasks => ("TASKS", Color::Cyan),
            FocusPanel::DoneTasks => ("DONE", Color::Green),
        },
        FocusMode::TerminalFocused => ("TERMINAL", Color::Green),
        FocusMode::AddingTask { .. } => ("ADD TASK", Color::Yellow),
        FocusMode::EditingTask { .. } => ("EDIT TASK", Color::Yellow),
        FocusMode::ConfirmDelete { .. } => ("DELETE", Color::Red),
        FocusMode::ConfirmStartTask { .. } => ("START", Color::Green),
        FocusMode::ClassifyingTask { .. } => ("CLASSIFYING", Color::Yellow),
        FocusMode::ReviewPopup { .. } => ("REVIEW", Color::Magenta),
        FocusMode::ReviewRequestChanges { .. } => ("REQUEST CHANGES", Color::Yellow),
    };

    let active_count: usize = app
        .kanban
        .columns
        .iter()
        .take(3)
        .map(|c| c.tasks.len())
        .sum();
    let done_count = app
        .kanban
        .columns
        .get(3)
        .map(|c| c.tasks.len())
        .unwrap_or(0);

    let help_text = if width < STATUS_BAR_WIDTH_THRESHOLD {
        match &state.mode {
            FocusMode::Normal => "jk:nav  Tab:panel  a:add  e:edit  d:del  s:start",
            FocusMode::TerminalFocused => "Esc:back  Keys→terminal",
            FocusMode::AddingTask { .. } | FocusMode::EditingTask { .. } => {
                "Enter:save  Esc:cancel"
            }
            FocusMode::ConfirmDelete { .. } | FocusMode::ConfirmStartTask { .. } => {
                "y:confirm  n:cancel"
            }
            FocusMode::ClassifyingTask { .. } => "Esc:cancel",
            FocusMode::ReviewPopup { .. } => "hl:buttons  jk:scroll  Enter:select  Esc:close",
            FocusMode::ReviewRequestChanges { .. } => "Enter:send  Esc:cancel",
        }
    } else {
        match &state.mode {
            FocusMode::Normal => {
                "↑↓/jk:navigate  Tab:switch-panel  a:add  e:edit  d:delete  s:start  Enter:focus-terminal"
            }
            FocusMode::TerminalFocused => "All keys sent to terminal  Esc:back-to-navigation",
            FocusMode::AddingTask { .. } | FocusMode::EditingTask { .. } => {
                "Type task title  Enter:save  Esc:cancel"
            }
            FocusMode::ConfirmDelete { .. } | FocusMode::ConfirmStartTask { .. } => {
                "Press y to confirm, n or Esc to cancel"
            }
            FocusMode::ClassifyingTask { .. } => "AI is classifying your task...  Esc:cancel",
            FocusMode::ReviewPopup { .. } => {
                "h/l:switch-buttons  j/k:scroll  Enter:select-action  Esc/q:close"
            }
            FocusMode::ReviewRequestChanges { .. } => {
                "Type your change request  Enter:send  Esc:cancel"
            }
        }
    };

    StatusBarContent {
        mode_text: mode_text.to_string(),
        mode_color,
        extra_info: Some(format!("Active: {}  Done: {}  ", active_count, done_count)),
        help_text: help_text.to_string(),
    }
}
