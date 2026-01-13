use super::{details_panel, done_tasks, task_list, terminal_panel};
use crate::app::App;
use crate::views::StatusBarContent;
use crate::views::tasks::dialogs;
use crate::views::tasks::operations::{TaskReference, get_active_tasks, get_done_tasks};
use crate::views::tasks::state::{FocusPanel, TasksMode, TasksViewMode};
use crate::widgets::dialogs::{ConfirmDialog, DialogStyle, ErrorDialog, InputDialog};
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::Color,
};

const LEFT_PANEL_PERCENT: u16 = 35;
const RIGHT_PANEL_PERCENT: u16 = 65;
const ACTIVE_TASKS_PANEL_PERCENT: u16 = 65;
const DONE_TASKS_PANEL_PERCENT: u16 = 35;
const DETAILS_PANEL_PERCENT: u16 = 30;
const TERMINAL_PANEL_PERCENT: u16 = 70;
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
    let instance_pane = instance_id.and_then(|id| app.instances.find_pane_mut(id));

    let is_terminal_focused = matches!(
        app.tasks.mode,
        TasksMode::TerminalFocused | TasksMode::TerminalScroll
    );
    let is_scroll_mode = matches!(app.tasks.mode, TasksMode::TerminalScroll);
    terminal_panel::render(
        frame,
        instance_pane,
        is_terminal_focused,
        is_scroll_mode,
        right_chunks[1],
    );

    render_dialogs(frame, app, &app.tasks.mode, area);
}

fn get_selected_task(app: &App) -> Option<TaskReference<'_>> {
    match app.tasks.focus_panel {
        FocusPanel::ActiveTasks => {
            let tasks = get_active_tasks(&app.tasks.columns);
            tasks.into_iter().nth(app.tasks.focus_active_index)
        }
        FocusPanel::DoneTasks => {
            let tasks = get_done_tasks(&app.tasks.columns);
            tasks.into_iter().nth(app.tasks.focus_done_index)
        }
    }
}

fn render_dialogs(frame: &mut Frame, app: &App, mode: &TasksMode, area: Rect) {
    match mode {
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
                ConfirmDialog::new("Delete Task", "Are you sure? (y/n)").style(DialogStyle::Danger),
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

    if let Some(error) = &app.tasks.error_message {
        frame.render_widget(ErrorDialog::new("Error", error), area);
    }
}

#[must_use]
pub fn get_status_bar_content(app: &App, width: u16) -> StatusBarContent {
    let state = &app.tasks;

    let (mode_text, mode_color) = match &state.mode {
        TasksMode::Normal => match state.focus_panel {
            FocusPanel::ActiveTasks => ("FOCUS", Color::Cyan),
            FocusPanel::DoneTasks => ("DONE", Color::Green),
        },
        TasksMode::TerminalFocused => ("TERMINAL", Color::Green),
        TasksMode::TerminalScroll => ("SCROLL", Color::Yellow),
        TasksMode::AddingTask { .. } => ("ADD TASK", Color::Yellow),
        TasksMode::SelectWorktree { .. } => ("SELECT WORKTREE", Color::Yellow),
        TasksMode::EditingTask { .. } => ("EDIT TASK", Color::Yellow),
        TasksMode::ConfirmDelete { .. } => ("DELETE", Color::Red),
        TasksMode::ConfirmMoveBack { .. } => ("MOVE BACK", Color::Red),
        TasksMode::ReviewPopup { .. } => ("REVIEW", Color::Magenta),
        TasksMode::ReviewRequestChanges { .. } => ("REQUEST CHANGES", Color::Yellow),
        TasksMode::MergeConfirmation { .. } => ("MERGE", Color::Green),
        TasksMode::SelectProvider { .. } => ("SELECT PROVIDER", Color::Yellow),
    };

    let active_count: usize = state
        .columns
        .iter()
        .take(3)
        .map(|column| column.tasks.len())
        .sum();
    let done_count = state.columns.get(3).map_or(0, |column| column.tasks.len());

    let view_indicator = match state.view_mode {
        TasksViewMode::Focus => "[Focus]",
        TasksViewMode::Kanban => "[Kanban]",
    };

    let help_text = if width < STATUS_BAR_WIDTH_THRESHOLD {
        match &state.mode {
            TasksMode::Normal => {
                "jk:nav  Tab:panel  a:add  e:edit  d:del  s:start  Bksp:back  /:view"
            }
            TasksMode::TerminalFocused => "Ctrl+s:scroll  Esc:back",
            TasksMode::TerminalScroll => "jk:line  Ctrl+d/u:page  g/G:top/bottom  q:exit",
            TasksMode::AddingTask { .. } | TasksMode::EditingTask { .. } => {
                "Enter:save  Esc:cancel"
            }
            TasksMode::SelectWorktree { .. } | TasksMode::SelectProvider { .. } => {
                "jk:select  Enter:choose  Esc:cancel"
            }
            TasksMode::ConfirmDelete { .. } | TasksMode::ConfirmMoveBack { .. } => {
                "y:confirm  n:cancel"
            }
            TasksMode::ReviewPopup { .. } => {
                "Tab:panel  jk:move/scroll  hl:buttons  Enter:select  Esc:close"
            }
            TasksMode::ReviewRequestChanges { .. } => "Enter:send  Esc:cancel",
            TasksMode::MergeConfirmation { .. } => "jk:select  Enter:merge  Esc:cancel",
        }
    } else {
        match &state.mode {
            TasksMode::Normal => {
                "↑↓/jk:navigate  Tab:switch-panel  a:add  e:edit  d:delete  s:start  Backspace:move-back  /:switch-view"
            }
            TasksMode::TerminalFocused => "Ctrl+s:scroll-mode  Esc:back-to-navigation",
            TasksMode::TerminalScroll => {
                "j/k:scroll-line  Ctrl+d/u:half-page  g/G:top/bottom  q/Esc:exit-scroll"
            }
            TasksMode::AddingTask { .. } | TasksMode::EditingTask { .. } => {
                "Type task title  Enter:save  Esc:cancel"
            }
            TasksMode::SelectWorktree { .. } | TasksMode::SelectProvider { .. } => {
                "↑↓/jk:select  Enter:choose  Esc:cancel"
            }
            TasksMode::ConfirmDelete { .. } | TasksMode::ConfirmMoveBack { .. } => {
                "Press y to confirm, n or Esc to cancel"
            }
            TasksMode::ReviewPopup { .. } => {
                "Tab:panel  h/l:switch-buttons  j/k:move/scroll  Enter:select-action  Esc/q:close"
            }
            TasksMode::ReviewRequestChanges { .. } => {
                "Type your change request  Enter:send  Esc:cancel"
            }
            TasksMode::MergeConfirmation { .. } => "↑↓/jk:select-branch  Enter:merge  Esc/q:cancel",
        }
    };

    StatusBarContent {
        mode_text: mode_text.to_string(),
        mode_color,
        extra_info: Some(format!(
            "{view_indicator} Active: {active_count}  Done: {done_count}  "
        )),
        help_text: help_text.to_string(),
    }
}
