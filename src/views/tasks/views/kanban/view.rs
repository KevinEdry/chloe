use super::columns::render_columns;
use crate::app::App;
use crate::views::StatusBarContent;
use crate::views::tasks::dialogs;
use crate::views::tasks::state::{TasksMode, TasksViewMode};
use crate::widgets::dialogs::{ConfirmDialog, DialogStyle, InputDialog, LoadingDialog};
use ratatui::{Frame, layout::Rect, style::Color};

const STATUS_BAR_WIDTH_THRESHOLD: u16 = 100;

pub fn render(frame: &mut Frame, app: &App, area: Rect) {
    let state = &app.tasks;

    render_columns(frame, app, area);

    match &state.mode {
        TasksMode::AddingTask { input } => {
            frame.render_widget(InputDialog::new("Add Task to Planning", input), area);
        }
        TasksMode::EditingTask { input, .. } => {
            frame.render_widget(InputDialog::new("Edit Task", input), area);
        }
        TasksMode::ConfirmDelete { .. } => {
            frame.render_widget(
                ConfirmDialog::new("Delete Task", "Delete this task? (y/n)")
                    .style(DialogStyle::Danger),
                area,
            );
        }
        TasksMode::ConfirmMoveBack { .. } => {
            frame.render_widget(
                ConfirmDialog::new(
                    "Move Back",
                    "Move back to Planning? This will terminate the Claude Code instance. (y/n)",
                )
                .style(DialogStyle::Danger),
                area,
            );
        }
        TasksMode::ConfirmStartTask { .. } => {
            frame.render_widget(
                ConfirmDialog::new("Start Task", "Move task to In Progress? (y/n)")
                    .style(DialogStyle::Success),
                area,
            );
        }
        TasksMode::ClassifyingTask { raw_input, .. } => {
            frame.render_widget(LoadingDialog::new("Loading", raw_input), area);
        }
        TasksMode::ReviewPopup {
            task_id,
            scroll_offset,
            selected_action,
        } => {
            dialogs::render_review_popup(
                frame,
                app,
                *task_id,
                *scroll_offset,
                *selected_action,
                area,
            );
        }
        TasksMode::ReviewRequestChanges { input, .. } => {
            frame.render_widget(InputDialog::new("Request Changes", input), area);
        }
        TasksMode::Normal | TasksMode::TerminalFocused => {}
    }
}

#[must_use]
pub fn get_status_bar_content(app: &App, width: u16) -> StatusBarContent {
    let state = &app.tasks;

    let mode_color = match &state.mode {
        TasksMode::Normal | TasksMode::ReviewPopup { .. } => Color::Cyan,
        TasksMode::TerminalFocused
        | TasksMode::AddingTask { .. }
        | TasksMode::ConfirmStartTask { .. } => Color::Green,
        TasksMode::EditingTask { .. } | TasksMode::ReviewRequestChanges { .. } => Color::Yellow,
        TasksMode::ConfirmDelete { .. } => Color::Red,
        TasksMode::ConfirmMoveBack { .. } => Color::LightRed,
        TasksMode::ClassifyingTask { .. } => Color::Magenta,
    };

    let mode_text = match &state.mode {
        TasksMode::Normal => "KANBAN",
        TasksMode::TerminalFocused => "TERMINAL",
        TasksMode::AddingTask { .. } => "ADD TO PLANNING",
        TasksMode::EditingTask { .. } => "EDIT TASK",
        TasksMode::ConfirmDelete { .. } => "CONFIRM DELETE",
        TasksMode::ConfirmMoveBack { .. } => "CONFIRM MOVE BACK",
        TasksMode::ConfirmStartTask { .. } => "START TASK",
        TasksMode::ClassifyingTask { .. } => "LOADING",
        TasksMode::ReviewPopup { .. } => "REVIEW OUTPUT",
        TasksMode::ReviewRequestChanges { .. } => "REQUEST CHANGES",
    };

    let view_indicator = match state.view_mode {
        TasksViewMode::Focus => "[Focus]",
        TasksViewMode::Kanban => "[Kanban]",
    };

    let help_text = if width < STATUS_BAR_WIDTH_THRESHOLD {
        match &state.mode {
            TasksMode::Normal => "hjkl/arrows:navigate  a:add  e:edit  d:delete  /:view",
            TasksMode::ClassifyingTask { .. } => "Esc:cancel",
            TasksMode::AddingTask { .. }
            | TasksMode::EditingTask { .. }
            | TasksMode::ReviewRequestChanges { .. } => "Enter:save  Esc:cancel",
            TasksMode::ConfirmDelete { .. }
            | TasksMode::ConfirmMoveBack { .. }
            | TasksMode::ConfirmStartTask { .. } => "y:yes  n:no",
            TasksMode::ReviewPopup { .. } => "jk:scroll  hl/Tab:button  Enter:action",
            TasksMode::TerminalFocused => "Esc:back",
        }
    } else {
        match &state.mode {
            TasksMode::Normal => {
                "↑↓/jk:task  ←→/hl:column  a:add-to-planning  e:edit  d:delete  Enter:move→  Backspace:move←  /:switch-view"
            }
            TasksMode::AddingTask { .. } | TasksMode::EditingTask { .. } => {
                "Type to enter text  Enter:save  Esc:cancel"
            }
            TasksMode::ConfirmDelete { .. }
            | TasksMode::ConfirmMoveBack { .. }
            | TasksMode::ConfirmStartTask { .. } => "y:yes  n:no  Esc:cancel",
            TasksMode::ClassifyingTask { .. } => "Press Esc to cancel",
            TasksMode::ReviewPopup { .. } => {
                "jk:scroll  ←→/hl/Tab:select-button  Enter:execute-action  q/Esc:close"
            }
            TasksMode::ReviewRequestChanges { .. } => {
                "Type your change request  Enter:save  Esc:cancel"
            }
            TasksMode::TerminalFocused => "Esc:back-to-navigation",
        }
    };

    StatusBarContent {
        mode_text: mode_text.to_string(),
        mode_color,
        extra_info: Some(format!("{view_indicator} ")),
        help_text: help_text.to_string(),
    }
}
