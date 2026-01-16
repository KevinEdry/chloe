mod dialogs;
mod focus_navigation;
mod kanban_navigation;
mod provider_selection;
mod terminal;
mod text_input;
mod worktree_selection;

use super::dialogs::review;
use super::state::{MergeTarget, TasksMode, TasksState, TasksViewMode, WorktreeSelectionOption};
use crate::events::{AppAction, EventHandler, EventResult, TaskAction, TerminalAction};
use crate::types::AgentProvider;
use crate::views::settings::VcsCommand;
use crossterm::event::{KeyCode, KeyEvent};
use uuid::Uuid;

pub enum TasksAction {
    None,
    JumpToInstance(Uuid),
    SendToTerminal(Uuid, Vec<u8>),
    ScrollTerminal {
        instance_id: Uuid,
        delta: isize,
    },
    ScrollTerminalToTop(Uuid),
    ScrollTerminalToBottom(Uuid),
    CreateTask {
        title: String,
    },
    UpdateTask {
        task_id: Uuid,
        new_title: String,
    },
    DeleteTask(Uuid),
    OpenInIDE(Uuid),
    SwitchToTerminal(Uuid),
    RequestChanges {
        task_id: Uuid,
        message: String,
    },
    CommitChanges(Uuid),
    MergeBranch {
        task_id: Uuid,
        target: MergeTarget,
    },
    WorktreeSelected {
        task_id: Uuid,
        worktree_option: WorktreeSelectionOption,
    },
    ProviderSelected {
        task_id: Uuid,
        provider: AgentProvider,
        worktree_option: WorktreeSelectionOption,
        remember: bool,
    },
}

pub fn handle_key_event(
    state: &mut TasksState,
    key: KeyEvent,
    selected_instance_id: Option<Uuid>,
    default_provider: AgentProvider,
    vcs_command: &VcsCommand,
) -> TasksAction {
    if state.error_message.is_some() {
        state.error_message = None;
        return TasksAction::None;
    }

    if state.mode == TasksMode::Normal && key.code == KeyCode::Char('/') {
        state.toggle_view_mode();
        return TasksAction::None;
    }

    match &state.mode {
        TasksMode::Normal => match state.view_mode {
            TasksViewMode::Focus => focus_navigation::handle_focus_normal_mode(
                state,
                key,
                selected_instance_id,
                vcs_command,
            ),
            TasksViewMode::Kanban => {
                kanban_navigation::handle_kanban_normal_mode(state, key, vcs_command);
                TasksAction::None
            }
        },
        TasksMode::TerminalFocused => {
            terminal::handle_terminal_focused_mode(state, key, selected_instance_id)
        }
        TasksMode::TerminalScroll => {
            terminal::handle_terminal_scroll_mode(state, key, selected_instance_id)
        }
        TasksMode::AddingTask { .. } => text_input::handle_adding_task_mode(state, key),
        TasksMode::SelectWorktree { .. } => {
            worktree_selection::handle_worktree_selection_mode(state, key)
        }
        TasksMode::SelectProvider { .. } => {
            provider_selection::handle_provider_selection_mode(state, key, default_provider)
        }
        TasksMode::EditingTask { .. } => text_input::handle_editing_task_mode(state, key),
        TasksMode::ConfirmDelete { task_id } => {
            dialogs::handle_confirm_delete_mode(state, key, *task_id)
        }

        TasksMode::ConfirmMoveBack { task_id } => {
            dialogs::handle_confirm_move_back_mode(state, key, *task_id)
        }
        TasksMode::ReviewPopup {
            task_id,
            diff_scroll_offset,
            output_scroll_offset,
            selected_file_index,
            focused_panel,
            selected_action,
        } => {
            let popup_state = review::ReviewPopupState {
                task_id: *task_id,
                diff_scroll_offset: *diff_scroll_offset,
                output_scroll_offset: *output_scroll_offset,
                selected_file_index: *selected_file_index,
                focused_panel: *focused_panel,
                selected_action: *selected_action,
            };
            review::handle_review_popup_mode(state, key, popup_state)
        }
        TasksMode::ReviewRequestChanges { task_id, .. } => {
            review::handle_review_request_changes_mode(state, key, *task_id)
        }
        TasksMode::MergeConfirmation {
            task_id,
            worktree_branch,
            selected_target,
        } => review::handle_merge_confirmation_mode(
            state,
            key,
            *task_id,
            worktree_branch.clone(),
            selected_target.clone(),
        ),
    }
}

impl EventHandler for TasksState {
    fn handle_key(&mut self, key: KeyEvent) -> EventResult {
        let selected_instance_id = self.selected_instance_id();
        let default_provider = AgentProvider::default();
        let vcs_command = VcsCommand::default();

        let action = handle_key_event(
            self,
            key,
            selected_instance_id,
            default_provider,
            &vcs_command,
        );

        convert_tasks_action_to_event_result(action)
    }
}

fn convert_tasks_action_to_event_result(action: TasksAction) -> EventResult {
    match action {
        TasksAction::None => EventResult::Consumed,
        TasksAction::JumpToInstance(instance_id) => EventResult::Action(AppAction::Terminal(
            TerminalAction::JumpToInstance(instance_id),
        )),
        TasksAction::SendToTerminal(instance_id, data) => {
            EventResult::Action(AppAction::Terminal(TerminalAction::SendInput {
                instance_id,
                data,
            }))
        }
        TasksAction::ScrollTerminal { instance_id, delta } => {
            EventResult::Action(AppAction::Terminal(TerminalAction::Scroll {
                instance_id,
                delta,
            }))
        }
        TasksAction::ScrollTerminalToTop(instance_id) => EventResult::Action(AppAction::Terminal(
            TerminalAction::ScrollToTop(instance_id),
        )),
        TasksAction::ScrollTerminalToBottom(instance_id) => EventResult::Action(
            AppAction::Terminal(TerminalAction::ScrollToBottom(instance_id)),
        ),
        TasksAction::CreateTask { title } => {
            EventResult::Action(AppAction::Task(TaskAction::Create { title }))
        }
        TasksAction::UpdateTask { task_id, new_title } => {
            EventResult::Action(AppAction::Task(TaskAction::Update { task_id, new_title }))
        }
        TasksAction::DeleteTask(task_id) => {
            EventResult::Action(AppAction::Task(TaskAction::Delete(task_id)))
        }
        TasksAction::OpenInIDE(task_id) => {
            EventResult::Action(AppAction::Task(TaskAction::OpenInIde(task_id)))
        }
        TasksAction::SwitchToTerminal(task_id) => {
            EventResult::Action(AppAction::Task(TaskAction::OpenInTerminal(task_id)))
        }
        TasksAction::RequestChanges { task_id, message } => {
            EventResult::Action(AppAction::Task(TaskAction::RequestChanges {
                task_id,
                message,
            }))
        }
        TasksAction::CommitChanges(task_id) => {
            EventResult::Action(AppAction::Task(TaskAction::CommitChanges(task_id)))
        }
        TasksAction::MergeBranch { task_id, target } => {
            EventResult::Action(AppAction::Task(TaskAction::MergeBranch { task_id, target }))
        }
        TasksAction::WorktreeSelected {
            task_id,
            worktree_option,
        } => EventResult::Action(AppAction::Task(TaskAction::WorktreeSelected {
            task_id,
            worktree_option,
        })),
        TasksAction::ProviderSelected {
            task_id,
            provider,
            worktree_option,
            remember,
        } => EventResult::Action(AppAction::Task(TaskAction::ProviderSelected {
            task_id,
            provider,
            worktree_option,
            remember,
        })),
    }
}
