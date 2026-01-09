use super::TasksAction;
use crate::views::tasks::state::{ReviewAction, TasksMode, TasksState};
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
        ReviewAction::MergeToBranch => {
            state.mode = TasksMode::Normal;
            TasksAction::MergeBranch(task_id)
        }
    }
}

pub fn handle_review_request_changes_mode(
    state: &mut TasksState,
    key: KeyEvent,
    task_id: Uuid,
) -> TasksAction {
    let input = match &mut state.mode {
        TasksMode::ReviewRequestChanges { input, .. } => input,
        _ => return TasksAction::None,
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
