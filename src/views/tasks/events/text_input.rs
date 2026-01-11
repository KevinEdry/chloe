use crate::views::tasks::state::{TasksMode, TasksState};
use crossterm::event::{KeyCode, KeyEvent};

use super::TasksAction;

pub fn handle_adding_task_mode(state: &mut TasksState, key: KeyEvent) -> TasksAction {
    let (input, prompt) = match &state.mode {
        TasksMode::AddingTask { input, prompt } => (input.clone(), prompt.clone()),
        _ => return TasksAction::None,
    };

    match key.code {
        KeyCode::Char(character) => {
            let mut new_input = input;
            new_input.push(character);
            state.mode = TasksMode::AddingTask {
                input: new_input,
                prompt,
            };
            TasksAction::None
        }
        KeyCode::Backspace => {
            let mut new_input = input;
            new_input.pop();
            state.mode = TasksMode::AddingTask {
                input: new_input,
                prompt,
            };
            TasksAction::None
        }
        KeyCode::Enter => {
            state.mode = TasksMode::Normal;
            if input.trim().is_empty() {
                return TasksAction::None;
            }
            TasksAction::CreateTask { title: input }
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
            state.mode = TasksMode::Normal;
            if input.trim().is_empty() {
                TasksAction::None
            } else {
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
