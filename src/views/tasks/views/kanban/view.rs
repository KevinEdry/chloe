use super::columns::render_columns;
use crate::app::App;
use crate::views::StatusBarContent;
use crate::views::tasks::dialogs;
use crate::views::tasks::state::{TasksMode, TasksViewMode};
use crate::widgets::dialogs::{ConfirmDialog, DialogStyle, ErrorDialog, InputDialog};
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
        TasksMode::MergeConfirmation {
            worktree_branch,
            selected_target,
            ..
        } => {
            dialogs::render_merge_confirmation(frame, worktree_branch, selected_target, area);
        }
        TasksMode::Normal | TasksMode::TerminalFocused | TasksMode::TerminalScroll => {}
    }

    if let Some(error) = &state.error_message {
        frame.render_widget(ErrorDialog::new("Error", error), area);
    }
}

#[must_use]
pub fn get_status_bar_content(app: &App, width: u16) -> StatusBarContent {
    let state = &app.tasks;

    let mode_color = match &state.mode {
        TasksMode::Normal | TasksMode::ReviewPopup { .. } => Color::Cyan,
        TasksMode::TerminalFocused
        | TasksMode::AddingTask { .. }
        | TasksMode::ConfirmStartTask { .. }
        | TasksMode::MergeConfirmation { .. } => Color::Green,
        TasksMode::TerminalScroll
        | TasksMode::EditingTask { .. }
        | TasksMode::ReviewRequestChanges { .. } => Color::Yellow,
        TasksMode::ConfirmDelete { .. } => Color::Red,
        TasksMode::ConfirmMoveBack { .. } => Color::LightRed,
    };

    let mode_text = match &state.mode {
        TasksMode::Normal => "KANBAN",
        TasksMode::TerminalFocused => "TERMINAL",
        TasksMode::TerminalScroll => "SCROLL",
        TasksMode::AddingTask { .. } => "ADD TO PLANNING",
        TasksMode::EditingTask { .. } => "EDIT TASK",
        TasksMode::ConfirmDelete { .. } => "CONFIRM DELETE",
        TasksMode::ConfirmMoveBack { .. } => "CONFIRM MOVE BACK",
        TasksMode::ConfirmStartTask { .. } => "START TASK",
        TasksMode::ReviewPopup { .. } => "REVIEW OUTPUT",
        TasksMode::ReviewRequestChanges { .. } => "REQUEST CHANGES",
        TasksMode::MergeConfirmation { .. } => "MERGE",
    };

    let view_indicator = match state.view_mode {
        TasksViewMode::Focus => "[Focus]",
        TasksViewMode::Kanban => "[Kanban]",
    };

    let help_text = if width < STATUS_BAR_WIDTH_THRESHOLD {
        match &state.mode {
            TasksMode::Normal => "hjkl/arrows:navigate  a:add  e:edit  d:delete  /:view",
            TasksMode::AddingTask { .. }
            | TasksMode::EditingTask { .. }
            | TasksMode::ReviewRequestChanges { .. } => "Enter:save  Esc:cancel",
            TasksMode::ConfirmDelete { .. }
            | TasksMode::ConfirmMoveBack { .. }
            | TasksMode::ConfirmStartTask { .. } => "y:yes  n:no",
            TasksMode::ReviewPopup { .. } => "jk:scroll  hl/Tab:button  Enter:action",
            TasksMode::TerminalFocused => "Ctrl+s:scroll  Esc:back",
            TasksMode::TerminalScroll => "jk:line  Ctrl+d/u:page  g/G:top/bottom  q:exit",
            TasksMode::MergeConfirmation { .. } => "jk:select  Enter:merge  Esc:cancel",
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
            TasksMode::ReviewPopup { .. } => {
                "jk:scroll  ←→/hl/Tab:select-button  Enter:execute-action  q/Esc:close"
            }
            TasksMode::ReviewRequestChanges { .. } => {
                "Type your change request  Enter:save  Esc:cancel"
            }
            TasksMode::TerminalFocused => "Ctrl+s:scroll-mode  Esc:back-to-navigation",
            TasksMode::TerminalScroll => {
                "j/k:scroll-line  Ctrl+d/u:half-page  g/G:top/bottom  q/Esc:exit-scroll"
            }
            TasksMode::MergeConfirmation { .. } => "↑↓/jk:select-branch  Enter:merge  Esc/q:cancel",
        }
    };

    StatusBarContent {
        mode_text: mode_text.to_string(),
        mode_color,
        extra_info: Some(format!("{view_indicator} ")),
        help_text: help_text.to_string(),
    }
}
