use super::TasksAction;
use crate::types::AgentProvider;
use crate::views::tasks::dialogs::{
    ProviderSelectionResult, get_option_count, get_selection_result,
};
use crate::views::tasks::state::{TasksMode, TasksState};
use crossterm::event::{KeyCode, KeyEvent};

pub fn handle_provider_selection_mode(
    state: &mut TasksState,
    key: KeyEvent,
    default_provider: AgentProvider,
) -> TasksAction {
    let (task_id, selected_index, worktree_option, detected_providers) = match &mut state.mode {
        TasksMode::SelectProvider {
            task_id,
            selected_index,
            worktree_option,
            detected_providers,
            ..
        } => (
            *task_id,
            selected_index,
            worktree_option.clone(),
            detected_providers.clone(),
        ),
        _ => return TasksAction::None,
    };

    let option_count = get_option_count(&detected_providers);

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
            let result = get_selection_result(current_index, &detected_providers, default_provider);

            state.mode = TasksMode::Normal;

            result.map_or(TasksAction::None, |selection: ProviderSelectionResult| {
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
