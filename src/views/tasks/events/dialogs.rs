use super::TasksAction;
use crate::views::tasks::state::{TasksMode, TasksState};
use crossterm::event::{KeyCode, KeyEvent};
use uuid::Uuid;

pub fn handle_confirm_delete_mode(
    state: &mut TasksState,
    key: KeyEvent,
    task_id: Uuid,
) -> TasksAction {
    match key.code {
        KeyCode::Char('y' | 'Y') => {
            state.mode = TasksMode::Normal;
            TasksAction::DeleteTask(task_id)
        }
        KeyCode::Char('n' | 'N') | KeyCode::Esc => {
            state.mode = TasksMode::Normal;
            TasksAction::None
        }
        _ => TasksAction::None,
    }
}

pub fn handle_confirm_move_back_mode(
    state: &mut TasksState,
    key: KeyEvent,
    _task_id: Uuid,
) -> TasksAction {
    match key.code {
        KeyCode::Char('y' | 'Y') => {
            state.execute_move_task_previous();
            state.mode = TasksMode::Normal;
            TasksAction::None
        }
        KeyCode::Char('n' | 'N') | KeyCode::Esc => {
            state.mode = TasksMode::Normal;
            TasksAction::None
        }
        _ => TasksAction::None,
    }
}
