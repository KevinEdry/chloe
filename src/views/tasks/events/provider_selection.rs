use super::TasksAction;
use crate::types::AgentProvider;
use crate::views::tasks::dialogs::{get_option_count, get_selection_result_with_default};
use crate::views::tasks::state::{TasksMode, TasksState};
use crossterm::event::{KeyCode, KeyEvent};

pub fn handle_provider_selection_mode(state: &mut TasksState, key: KeyEvent) -> TasksAction {
    let (task_id, selected_index, worktree_option) = match &mut state.mode {
        TasksMode::SelectProvider {
            task_id,
            selected_index,
            worktree_option,
            ..
        } => (*task_id, selected_index, worktree_option.clone()),
        _ => return TasksAction::None,
    };

    let option_count = get_option_count();

    match key.code {
        KeyCode::Esc | KeyCode::Char('q') => {
            state.mode = TasksMode::Normal;
            TasksAction::None
        }
        KeyCode::Up | KeyCode::Char('k') => {
            *selected_index = selected_index.saturating_sub(1);
            TasksAction::None
        }
        KeyCode::Down | KeyCode::Char('j') => {
            *selected_index = (*selected_index + 1).min(option_count - 1);
            TasksAction::None
        }
        KeyCode::Enter => {
            let current_index = *selected_index;
            let result =
                get_selection_result_with_default(current_index, AgentProvider::default());

            state.mode = TasksMode::Normal;

            result.map_or(TasksAction::None, |selection| {
                TasksAction::ProviderSelected {
                    task_id,
                    provider: selection.provider(),
                    worktree_option: worktree_option.clone(),
                    remember: selection.should_remember(),
                }
            })
        }
        _ => TasksAction::None,
    }
}
