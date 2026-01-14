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
        TasksMode::AddingTask { input, prompt } => {
            let dialog_state = dialogs::AddTaskDialogState { input, prompt };
            dialogs::render_add_task_dialog(frame, &dialog_state, area);
        }
        TasksMode::SelectWorktree {
            task_title,
            selected_index,
            options,
            ..
        } => {
            let dialog_state = dialogs::WorktreeSelectionViewState {
                task_title,
                selected_index: *selected_index,
                options,
            };
            dialogs::render_worktree_selection(frame, &dialog_state, area);
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
                    "Move back to previous column? This will terminate the Claude Code instance and clean up resources. (y/n)",
                )
                .style(DialogStyle::Danger),
                area,
            );
        }
        TasksMode::ReviewPopup {
            task_id,
            diff_scroll_offset,
            output_scroll_offset,
            selected_file_index,
            focused_panel,
            selected_action,
        } => {
            let popup_state = dialogs::ReviewPopupViewState {
                task_id: *task_id,
                diff_scroll_offset: *diff_scroll_offset,
                output_scroll_offset: *output_scroll_offset,
                selected_file_index: *selected_file_index,
                focused_panel: *focused_panel,
                selected_action: *selected_action,
            };
            dialogs::render_review_popup(frame, app, &popup_state, area);
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
        TasksMode::SelectProvider {
            selected_index,
            detected_providers,
            ..
        } => {
            let dialog_state = dialogs::ProviderSelectionViewState {
                selected_index: *selected_index,
                default_provider: app.settings.settings.default_provider,
                detected_providers,
            };
            dialogs::render_provider_selection(frame, &dialog_state, area);
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
        | TasksMode::SelectWorktree { .. }
        | TasksMode::MergeConfirmation { .. } => Color::Green,
        TasksMode::TerminalScroll
        | TasksMode::EditingTask { .. }
        | TasksMode::ReviewRequestChanges { .. }
        | TasksMode::SelectProvider { .. } => Color::Yellow,
        TasksMode::ConfirmDelete { .. } => Color::Red,
        TasksMode::ConfirmMoveBack { .. } => Color::LightRed,
    };

    let mode_text = match &state.mode {
        TasksMode::Normal => "KANBAN",
        TasksMode::TerminalFocused => "TERMINAL",
        TasksMode::TerminalScroll => "SCROLL",
        TasksMode::AddingTask { .. } => "ADD TO PLANNING",
        TasksMode::SelectWorktree { .. } => "SELECT WORKTREE",
        TasksMode::SelectProvider { .. } => "SELECT PROVIDER",
        TasksMode::EditingTask { .. } => "EDIT TASK",
        TasksMode::ConfirmDelete { .. } => "CONFIRM DELETE",
        TasksMode::ConfirmMoveBack { .. } => "CONFIRM MOVE BACK",
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
            TasksMode::SelectWorktree { .. } | TasksMode::SelectProvider { .. } => {
                "jk:select  Enter:choose  Esc:cancel"
            }
            TasksMode::ConfirmDelete { .. } | TasksMode::ConfirmMoveBack { .. } => "y:yes  n:no",
            TasksMode::ReviewPopup { .. } => "Tab:panel  jk:move/scroll  hl:button  Enter:action",
            TasksMode::TerminalFocused => "Ctrl+s:scroll  Esc:back",
            TasksMode::TerminalScroll => "jk:line  Ctrl+d/u:page  g/G:top/bottom  q:exit",
            TasksMode::MergeConfirmation { .. } => "jk:select  Enter:merge  Esc:cancel",
        }
    } else {
        match &state.mode {
            TasksMode::Normal => {
                "↑↓/jk:task  ←→/hl:column  a:add-to-planning  e:edit  d:delete  Enter:move→  Backspace:move←  /:switch-view"
            }
            TasksMode::AddingTask { .. } => "Type task title  Enter:save  Esc:cancel",
            TasksMode::SelectWorktree { .. } | TasksMode::SelectProvider { .. } => {
                "↑↓/jk:select  Enter:choose  Esc:cancel"
            }
            TasksMode::EditingTask { .. } => "Type to enter text  Enter:save  Esc:cancel",
            TasksMode::ConfirmDelete { .. } | TasksMode::ConfirmMoveBack { .. } => {
                "y:yes  n:no  Esc:cancel"
            }
            TasksMode::ReviewPopup { .. } => {
                "Tab:panel  jk:move/scroll  ←→/hl:select-button  Enter:execute-action  q/Esc:close"
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
