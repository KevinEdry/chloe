use super::TasksAction;
use crate::views::tasks::state::{MergeTarget, ReviewAction, TasksMode, TasksState};
use crate::views::worktree::{find_repository_root, get_current_branch, get_worktree_status};
use crossterm::event::{KeyCode, KeyEvent};
use uuid::Uuid;

pub fn handle_review_popup_mode(
    state: &mut TasksState,
    key: KeyEvent,
    task_id: Uuid,
    scroll_offset: usize,
    selected_action: ReviewAction,
) -> TasksAction {
    match key.code {
        KeyCode::Esc | KeyCode::Char('q') => {
            state.mode = TasksMode::Normal;
            TasksAction::None
        }
        KeyCode::Char('j') => {
            state.mode = TasksMode::ReviewPopup {
                task_id,
                scroll_offset: scroll_offset.saturating_add(1),
                selected_action,
            };
            TasksAction::None
        }
        KeyCode::Char('k') => {
            state.mode = TasksMode::ReviewPopup {
                task_id,
                scroll_offset: scroll_offset.saturating_sub(1),
                selected_action,
            };
            TasksAction::None
        }
        KeyCode::PageDown => {
            state.mode = TasksMode::ReviewPopup {
                task_id,
                scroll_offset: scroll_offset.saturating_add(10),
                selected_action,
            };
            TasksAction::None
        }
        KeyCode::PageUp => {
            state.mode = TasksMode::ReviewPopup {
                task_id,
                scroll_offset: scroll_offset.saturating_sub(10),
                selected_action,
            };
            TasksAction::None
        }
        KeyCode::Left | KeyCode::Char('h') => {
            let actions = ReviewAction::all();
            let current_index = actions
                .iter()
                .position(|action| *action == selected_action)
                .unwrap_or(0);
            let new_index = if current_index == 0 {
                actions.len() - 1
            } else {
                current_index - 1
            };
            state.mode = TasksMode::ReviewPopup {
                task_id,
                scroll_offset,
                selected_action: actions[new_index],
            };
            TasksAction::None
        }
        KeyCode::Right | KeyCode::Char('l') | KeyCode::Tab => {
            let actions = ReviewAction::all();
            let current_index = actions
                .iter()
                .position(|action| *action == selected_action)
                .unwrap_or(0);
            let new_index = (current_index + 1) % actions.len();
            state.mode = TasksMode::ReviewPopup {
                task_id,
                scroll_offset,
                selected_action: actions[new_index],
            };
            TasksAction::None
        }
        KeyCode::Enter => execute_review_action(state, task_id, selected_action),
        _ => TasksAction::None,
    }
}

fn execute_review_action(
    state: &mut TasksState,
    task_id: Uuid,
    action: ReviewAction,
) -> TasksAction {
    let task = state.find_task_by_id(task_id);
    let worktree_info = task.and_then(|t| t.worktree_info.as_ref());

    let is_clean = worktree_info
        .and_then(|info| get_worktree_status(&info.worktree_path).ok())
        .is_none_or(|status| status.is_clean);

    match action {
        ReviewAction::ReviewInIDE => {
            state.mode = TasksMode::Normal;
            TasksAction::OpenInIDE(task_id)
        }
        ReviewAction::ReviewInTerminal => {
            state.mode = TasksMode::Normal;
            TasksAction::SwitchToTerminal(task_id)
        }
        ReviewAction::RequestChanges => {
            state.mode = TasksMode::ReviewRequestChanges {
                task_id,
                input: String::new(),
            };
            TasksAction::None
        }
        ReviewAction::CommitChanges => {
            if is_clean {
                return TasksAction::None;
            }
            state.mode = TasksMode::Normal;
            TasksAction::CommitChanges(task_id)
        }
        ReviewAction::MergeAndComplete => {
            if !is_clean {
                return TasksAction::None;
            }

            let worktree_branch = worktree_info
                .map(|info| info.branch_name.clone())
                .unwrap_or_default();

            let current_branch = std::env::current_dir()
                .ok()
                .and_then(|dir| find_repository_root(&dir).ok())
                .and_then(|root| get_current_branch(&root).ok())
                .unwrap_or_else(|| "main".to_string());

            let selected_target = if current_branch == "main" || current_branch == "master" {
                MergeTarget::MainBranch
            } else {
                MergeTarget::CurrentBranch(current_branch)
            };

            state.mode = TasksMode::MergeConfirmation {
                task_id,
                worktree_branch,
                selected_target,
            };
            TasksAction::None
        }
    }
}

pub fn handle_review_request_changes_mode(
    state: &mut TasksState,
    key: KeyEvent,
    task_id: Uuid,
) -> TasksAction {
    let TasksMode::ReviewRequestChanges { input, .. } = &mut state.mode else {
        return TasksAction::None;
    };

    match key.code {
        KeyCode::Char(character) => {
            input.push(character);
            TasksAction::None
        }
        KeyCode::Backspace => {
            input.pop();
            TasksAction::None
        }
        KeyCode::Enter => {
            let change_request = input.clone();
            state.mode = TasksMode::Normal;
            if change_request.is_empty() {
                TasksAction::None
            } else {
                TasksAction::RequestChanges {
                    task_id,
                    message: change_request,
                }
            }
        }
        KeyCode::Esc => {
            state.mode = TasksMode::Normal;
            TasksAction::None
        }
        _ => TasksAction::None,
    }
}

pub fn handle_merge_confirmation_mode(
    state: &mut TasksState,
    key: KeyEvent,
    task_id: Uuid,
    worktree_branch: String,
    selected_target: MergeTarget,
) -> TasksAction {
    match key.code {
        KeyCode::Esc | KeyCode::Char('q') => {
            state.mode = TasksMode::Normal;
            TasksAction::None
        }
        KeyCode::Up | KeyCode::Down | KeyCode::Char('k' | 'j') => {
            let current_branch = std::env::current_dir()
                .ok()
                .and_then(|dir| find_repository_root(&dir).ok())
                .and_then(|root| get_current_branch(&root).ok())
                .unwrap_or_else(|| "main".to_string());

            let new_target = match &selected_target {
                MergeTarget::CurrentBranch(_) => MergeTarget::MainBranch,
                MergeTarget::MainBranch => {
                    if current_branch == "main" || current_branch == "master" {
                        MergeTarget::MainBranch
                    } else {
                        MergeTarget::CurrentBranch(current_branch)
                    }
                }
            };

            state.mode = TasksMode::MergeConfirmation {
                task_id,
                worktree_branch,
                selected_target: new_target,
            };
            TasksAction::None
        }
        KeyCode::Enter => {
            state.mode = TasksMode::Normal;
            TasksAction::MergeBranch {
                task_id,
                target: selected_target,
            }
        }
        _ => TasksAction::None,
    }
}
