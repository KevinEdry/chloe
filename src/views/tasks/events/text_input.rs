use super::TasksAction;
use crate::views::tasks::state::{TasksMode, TasksState};
use crossterm::event::{KeyCode, KeyEvent};

pub fn handle_adding_task_mode(state: &mut TasksState, key: KeyEvent) -> TasksAction {
    let input = match &state.mode {
        TasksMode::AddingTask { input } => input.clone(),
        _ => return TasksAction::None,
    };

    match key.code {
        KeyCode::Char(character) => {
            let mut new_input = input;
            new_input.push(character);
            state.mode = TasksMode::AddingTask { input: new_input };
            TasksAction::None
        }
        KeyCode::Backspace => {
            let mut new_input = input;
            new_input.pop();
            state.mode = TasksMode::AddingTask { input: new_input };
            TasksAction::None
        }
        KeyCode::Enter => {
            if input.trim().is_empty() {
                state.mode = TasksMode::Normal;
                TasksAction::None
            } else {
                state.mode = TasksMode::Normal;
                TasksAction::CreateTask(input)
            }
        }
        KeyCode::Esc => {
            state.mode = TasksMode::Normal;
            TasksAction::None
        }
        _ => TasksAction::None,
    }
}

pub fn handle_editing_task_mode(state: &mut TasksState, key: KeyEvent) -> TasksAction {
    let (task_id, input) = match &state.mode {
        TasksMode::EditingTask { task_id, input } => (*task_id, input.clone()),
        _ => return TasksAction::None,
    };

    match key.code {
        KeyCode::Char(character) => {
            let mut new_input = input;
            new_input.push(character);
            state.mode = TasksMode::EditingTask {
                task_id,
                input: new_input,
            };
            TasksAction::None
        }
        KeyCode::Backspace => {
            let mut new_input = input;
            new_input.pop();
            state.mode = TasksMode::EditingTask {
                task_id,
                input: new_input,
            };
            TasksAction::None
        }
        KeyCode::Enter => {
            if input.trim().is_empty() {
                state.mode = TasksMode::Normal;
                TasksAction::None
            } else {
                state.mode = TasksMode::Normal;
                TasksAction::UpdateTask {
                    task_id,
                    new_title: input,
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
