use super::TasksAction;
use crate::views::settings::VcsCommand;
use crate::views::tasks::operations::{
    get_active_task_count, get_active_tasks, get_done_task_count, get_done_tasks,
};
use crate::views::tasks::state::{FocusPanel, ReviewAction, ReviewPanel, TasksMode, TasksState};
use crossterm::event::{KeyCode, KeyEvent};
use uuid::Uuid;

#[allow(clippy::too_many_lines)]
pub fn handle_focus_normal_mode(
    state: &mut TasksState,
    key: KeyEvent,
    selected_instance_id: Option<Uuid>,
    vcs_command: &VcsCommand,
) -> TasksAction {
    let active_count = get_active_task_count(&state.columns);
    let done_count = get_done_task_count(&state.columns);

    let selected_task = match state.focus_panel {
        FocusPanel::ActiveTasks => {
            let tasks = get_active_tasks(&state.columns);
            tasks.into_iter().nth(state.focus_active_index)
        }
        FocusPanel::DoneTasks => {
            let tasks = get_done_tasks(&state.columns);
            tasks.into_iter().nth(state.focus_done_index)
        }
    };

    match key.code {
        KeyCode::Char('j') | KeyCode::Down => {
            state.focus_select_next();
            TasksAction::None
        }
        KeyCode::Char('k') | KeyCode::Up => {
            state.focus_select_previous();
            TasksAction::None
        }
        KeyCode::Tab => {
            match state.focus_panel {
                FocusPanel::ActiveTasks => state.focus_switch_to_done_panel(),
                FocusPanel::DoneTasks => state.focus_switch_to_active_panel(),
            }
            TasksAction::None
        }
        KeyCode::Char('a') => {
            state.begin_add_task();
            TasksAction::None
        }
        KeyCode::Char('e') => {
            if let Some(task_ref) = selected_task {
                state.mode = TasksMode::EditingTask {
                    task_id: task_ref.task.id,
                    input: task_ref.task.title.clone(),
                };
            }
            TasksAction::None
        }
        KeyCode::Char('d') => {
            if let Some(task_ref) = selected_task {
                state.mode = TasksMode::ConfirmDelete {
                    task_id: task_ref.task.id,
                };
            }
            TasksAction::None
        }
        KeyCode::Enter => {
            if let Some(task_ref) = selected_task {
                let is_planning = task_ref.column_index == 0;
                let is_in_progress = task_ref.column_index == 1;
                let is_review = task_ref.column_index == 2;

                if is_planning {
                    state.begin_worktree_selection_for_task(task_ref.task.id, vcs_command);
                    TasksAction::None
                } else if is_in_progress {
                    selected_instance_id.map_or(TasksAction::None, |instance_id| {
                        state.enter_terminal_mode();
                        TasksAction::SendToTerminal(instance_id, Vec::new())
                    })
                } else if is_review {
                    state.mode = TasksMode::ReviewPopup {
                        task_id: task_ref.task.id,
                        diff_scroll_offset: 0,
                        output_scroll_offset: 0,
                        selected_file_index: 0,
                        focused_panel: ReviewPanel::FileList,
                        selected_action: ReviewAction::ReviewInIDE,
                    };
                    TasksAction::None
                } else {
                    TasksAction::None
                }
            } else {
                TasksAction::None
            }
        }
        KeyCode::Char('t') => {
            selected_instance_id.map_or(TasksAction::None, TasksAction::JumpToInstance)
        }
        KeyCode::Char('s') => {
            if let Some(task_ref) = selected_task {
                let is_planning = task_ref.column_index == 0;
                if is_planning {
                    state.begin_worktree_selection_for_task(task_ref.task.id, vcs_command);
                }
            }
            TasksAction::None
        }
        KeyCode::Char('g') => {
            match state.focus_panel {
                FocusPanel::ActiveTasks => {
                    if active_count > 0 {
                        state.focus_active_index = 0;
                    }
                }
                FocusPanel::DoneTasks => {
                    if done_count > 0 {
                        state.focus_done_index = 0;
                    }
                }
            }
            TasksAction::None
        }
        KeyCode::Char('G') => {
            match state.focus_panel {
                FocusPanel::ActiveTasks => {
                    if active_count > 0 {
                        state.focus_active_index = active_count - 1;
                    }
                }
                FocusPanel::DoneTasks => {
                    if done_count > 0 {
                        state.focus_done_index = done_count - 1;
                    }
                }
            }
            TasksAction::None
        }
        KeyCode::Backspace => {
            if let Some(task_ref) = selected_task {
                let task_index_in_column = state.columns[task_ref.column_index]
                    .tasks
                    .iter()
                    .position(|task| task.id == task_ref.task.id);

                if let Some(task_index) = task_index_in_column {
                    state.kanban_selected_column = task_ref.column_index;
                    state.kanban_selected_task = Some(task_index);
                    state.move_task_previous();
                }
            }
            TasksAction::None
        }
        _ => TasksAction::None,
    }
}
