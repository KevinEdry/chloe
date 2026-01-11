use crate::views::tasks::state::{TasksMode, TasksState, WorktreeSelectionOption};
use crossterm::event::{KeyCode, KeyEvent};

use super::TasksAction;

pub fn handle_worktree_selection_mode(state: &mut TasksState, key: KeyEvent) -> TasksAction {
    let (task_id, task_title, selected_index, options) = match &state.mode {
        TasksMode::SelectWorktree {
            task_id,
            task_title,
            selected_index,
            options,
        } => (
            *task_id,
            task_title.clone(),
            *selected_index,
            options.clone(),
        ),
        _ => return TasksAction::None,
    };

    let last_index = options.len().saturating_sub(1);

    match key.code {
        KeyCode::Char('j') | KeyCode::Down => {
            let next_index = (selected_index + 1).min(last_index);
            state.mode = TasksMode::SelectWorktree {
                task_id,
                task_title,
                selected_index: next_index,
                options,
            };
            TasksAction::None
        }
        KeyCode::Char('k') | KeyCode::Up => {
            let previous_index = selected_index.saturating_sub(1);
            state.mode = TasksMode::SelectWorktree {
                task_id,
                task_title,
                selected_index: previous_index,
                options,
            };
            TasksAction::None
        }
        KeyCode::Enter => {
            state.mode = TasksMode::Normal;
            let selected_option = options
                .get(selected_index)
                .cloned()
                .unwrap_or(WorktreeSelectionOption::AutoCreate);
            TasksAction::WorktreeSelected {
                task_id,
                task_title,
                worktree_option: selected_option,
            }
        }
        KeyCode::Esc => {
            state.mode = TasksMode::Normal;
            TasksAction::None
        }
        _ => TasksAction::None,
    }
}
