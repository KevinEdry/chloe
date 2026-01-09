use super::operations::{
    TaskReference, get_active_task_count, get_active_tasks, get_done_task_count, get_done_tasks,
};
use super::state::{FocusMode, FocusPanel, FocusReviewAction, FocusState};
use crate::views::kanban::Column;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use uuid::Uuid;

pub enum FocusAction {
    None,
    JumpToInstance(Uuid),
    SendToTerminal(Uuid, Vec<u8>),
    CreateTask(String),
    UpdateTask { task_id: Uuid, new_title: String },
    DeleteTask(Uuid),
    StartTask(Uuid),
    CancelClassification,
    OpenInIDE(Uuid),
    SwitchToTerminal(Uuid),
    RequestChanges { task_id: Uuid, message: String },
    MergeBranch(Uuid),
}

pub fn handle_key_event(
    state: &mut FocusState,
    key: KeyEvent,
    columns: &[Column],
    selected_instance_id: Option<Uuid>,
) -> FocusAction {
    let active_count = get_active_task_count(columns);
    let done_count = get_done_task_count(columns);

    let selected_task = match state.focused_panel {
        FocusPanel::ActiveTasks => {
            let tasks = get_active_tasks(columns);
            tasks.into_iter().nth(state.active_selected_index)
        }
        FocusPanel::DoneTasks => {
            let tasks = get_done_tasks(columns);
            tasks.into_iter().nth(state.done_selected_index)
        }
    };

    match &state.mode {
        FocusMode::Normal => handle_normal_mode(
            state,
            key,
            active_count,
            done_count,
            selected_task.as_ref(),
            selected_instance_id,
        ),
        FocusMode::TerminalFocused => {
            handle_terminal_focused_mode(state, key, selected_instance_id)
        }
        FocusMode::AddingTask { .. } => handle_adding_task_mode(state, key),
        FocusMode::EditingTask { .. } => handle_editing_task_mode(state, key),
        FocusMode::ConfirmDelete { task_id } => handle_confirm_delete_mode(state, key, *task_id),
        FocusMode::ConfirmStartTask { task_id } => {
            handle_confirm_start_task_mode(state, key, *task_id)
        }
        FocusMode::ClassifyingTask { .. } => handle_classifying_task_mode(state, key),
        FocusMode::ReviewPopup {
            task_id,
            scroll_offset,
            selected_action,
        } => handle_review_popup_mode(state, key, *task_id, *scroll_offset, *selected_action),
        FocusMode::ReviewRequestChanges { task_id, .. } => {
            handle_review_request_changes_mode(state, key, *task_id)
        }
    }
}

fn handle_normal_mode(
    state: &mut FocusState,
    key: KeyEvent,
    active_count: usize,
    done_count: usize,
    selected_task: Option<&TaskReference<'_>>,
    selected_instance_id: Option<Uuid>,
) -> FocusAction {
    match key.code {
        KeyCode::Char('j') | KeyCode::Down => {
            state.select_next(active_count, done_count);
            FocusAction::None
        }
        KeyCode::Char('k') | KeyCode::Up => {
            state.select_previous();
            FocusAction::None
        }
        KeyCode::Tab => {
            match state.focused_panel {
                FocusPanel::ActiveTasks => state.switch_to_done_panel(done_count),
                FocusPanel::DoneTasks => state.switch_to_active_panel(active_count),
            }
            FocusAction::None
        }
        KeyCode::Char('a') => {
            state.mode = FocusMode::AddingTask {
                input: String::new(),
            };
            FocusAction::None
        }
        KeyCode::Char('e') => {
            if let Some(task_ref) = selected_task {
                state.mode = FocusMode::EditingTask {
                    task_id: task_ref.task.id,
                    input: task_ref.task.title.clone(),
                };
            }
            FocusAction::None
        }
        KeyCode::Char('d') => {
            if let Some(task_ref) = selected_task {
                state.mode = FocusMode::ConfirmDelete {
                    task_id: task_ref.task.id,
                };
            }
            FocusAction::None
        }
        KeyCode::Enter => {
            if let Some(task_ref) = selected_task {
                let is_planning = task_ref.column_index == 0;
                let is_in_progress = task_ref.column_index == 1;
                let is_review = task_ref.column_index == 2;

                if is_planning {
                    state.mode = FocusMode::ConfirmStartTask {
                        task_id: task_ref.task.id,
                    };
                    FocusAction::None
                } else if is_in_progress {
                    if let Some(instance_id) = selected_instance_id {
                        state.enter_terminal_mode();
                        FocusAction::SendToTerminal(instance_id, Vec::new())
                    } else {
                        FocusAction::None
                    }
                } else if is_review {
                    state.mode = FocusMode::ReviewPopup {
                        task_id: task_ref.task.id,
                        scroll_offset: 0,
                        selected_action: FocusReviewAction::ReviewInIDE,
                    };
                    FocusAction::None
                } else {
                    FocusAction::None
                }
            } else {
                FocusAction::None
            }
        }
        KeyCode::Char('t') => {
            if let Some(instance_id) = selected_instance_id {
                FocusAction::JumpToInstance(instance_id)
            } else {
                FocusAction::None
            }
        }
        KeyCode::Char('s') => {
            if let Some(task_ref) = selected_task {
                let is_planning = task_ref.column_index == 0;
                if is_planning {
                    state.mode = FocusMode::ConfirmStartTask {
                        task_id: task_ref.task.id,
                    };
                }
            }
            FocusAction::None
        }
        _ => FocusAction::None,
    }
}

fn handle_adding_task_mode(state: &mut FocusState, key: KeyEvent) -> FocusAction {
    let input = match &state.mode {
        FocusMode::AddingTask { input } => input.clone(),
        _ => return FocusAction::None,
    };

    match key.code {
        KeyCode::Char(character) => {
            let mut new_input = input;
            new_input.push(character);
            state.mode = FocusMode::AddingTask { input: new_input };
            FocusAction::None
        }
        KeyCode::Backspace => {
            let mut new_input = input;
            new_input.pop();
            state.mode = FocusMode::AddingTask { input: new_input };
            FocusAction::None
        }
        KeyCode::Enter => {
            if input.trim().is_empty() {
                state.mode = FocusMode::Normal;
                FocusAction::None
            } else {
                state.mode = FocusMode::Normal;
                FocusAction::CreateTask(input)
            }
        }
        KeyCode::Esc => {
            state.mode = FocusMode::Normal;
            FocusAction::None
        }
        _ => FocusAction::None,
    }
}

fn handle_editing_task_mode(state: &mut FocusState, key: KeyEvent) -> FocusAction {
    let (task_id, input) = match &state.mode {
        FocusMode::EditingTask { task_id, input } => (*task_id, input.clone()),
        _ => return FocusAction::None,
    };

    match key.code {
        KeyCode::Char(character) => {
            let mut new_input = input;
            new_input.push(character);
            state.mode = FocusMode::EditingTask {
                task_id,
                input: new_input,
            };
            FocusAction::None
        }
        KeyCode::Backspace => {
            let mut new_input = input;
            new_input.pop();
            state.mode = FocusMode::EditingTask {
                task_id,
                input: new_input,
            };
            FocusAction::None
        }
        KeyCode::Enter => {
            if input.trim().is_empty() {
                state.mode = FocusMode::Normal;
                FocusAction::None
            } else {
                state.mode = FocusMode::Normal;
                FocusAction::UpdateTask {
                    task_id,
                    new_title: input,
                }
            }
        }
        KeyCode::Esc => {
            state.mode = FocusMode::Normal;
            FocusAction::None
        }
        _ => FocusAction::None,
    }
}

fn handle_confirm_delete_mode(state: &mut FocusState, key: KeyEvent, task_id: Uuid) -> FocusAction {
    match key.code {
        KeyCode::Char('y' | 'Y') => {
            state.mode = FocusMode::Normal;
            FocusAction::DeleteTask(task_id)
        }
        KeyCode::Char('n' | 'N') | KeyCode::Esc => {
            state.mode = FocusMode::Normal;
            FocusAction::None
        }
        _ => FocusAction::None,
    }
}

fn handle_confirm_start_task_mode(
    state: &mut FocusState,
    key: KeyEvent,
    task_id: Uuid,
) -> FocusAction {
    match key.code {
        KeyCode::Char('y' | 'Y') | KeyCode::Enter => {
            state.mode = FocusMode::Normal;
            FocusAction::StartTask(task_id)
        }
        KeyCode::Char('n' | 'N') | KeyCode::Esc => {
            state.mode = FocusMode::Normal;
            FocusAction::None
        }
        _ => FocusAction::None,
    }
}

fn handle_classifying_task_mode(state: &mut FocusState, key: KeyEvent) -> FocusAction {
    if key.code == KeyCode::Esc {
        state.mode = FocusMode::Normal;
        return FocusAction::CancelClassification;
    }
    FocusAction::None
}

fn handle_terminal_focused_mode(
    state: &mut FocusState,
    key: KeyEvent,
    selected_instance_id: Option<Uuid>,
) -> FocusAction {
    if key.code == KeyCode::Esc {
        state.exit_terminal_mode();
        return FocusAction::None;
    }

    let Some(instance_id) = selected_instance_id else {
        state.exit_terminal_mode();
        return FocusAction::None;
    };

    let data = convert_key_to_bytes(key);
    if data.is_empty() {
        return FocusAction::None;
    }

    FocusAction::SendToTerminal(instance_id, data)
}

fn convert_key_to_bytes(key: KeyEvent) -> Vec<u8> {
    match key.code {
        KeyCode::Char(character) => {
            if key.modifiers.contains(KeyModifiers::CONTROL) {
                if character.is_ascii_alphabetic() {
                    let control_char = (character.to_ascii_lowercase() as u8) - b'a' + 1;
                    vec![control_char]
                } else {
                    Vec::new()
                }
            } else {
                character.to_string().into_bytes()
            }
        }
        KeyCode::Enter => b"\r".to_vec(),
        KeyCode::Backspace => b"\x7f".to_vec(),
        KeyCode::Left => b"\x1b[D".to_vec(),
        KeyCode::Right => b"\x1b[C".to_vec(),
        KeyCode::Up => b"\x1b[A".to_vec(),
        KeyCode::Down => b"\x1b[B".to_vec(),
        KeyCode::Home => b"\x1b[H".to_vec(),
        KeyCode::End => b"\x1b[F".to_vec(),
        KeyCode::PageUp => b"\x1b[5~".to_vec(),
        KeyCode::PageDown => b"\x1b[6~".to_vec(),
        KeyCode::Tab => b"\t".to_vec(),
        KeyCode::BackTab => b"\x1b[Z".to_vec(),
        KeyCode::Delete => b"\x1b[3~".to_vec(),
        KeyCode::Insert => b"\x1b[2~".to_vec(),
        KeyCode::Esc => b"\x1b".to_vec(),
        _ => Vec::new(),
    }
}

fn handle_review_popup_mode(
    state: &mut FocusState,
    key: KeyEvent,
    task_id: Uuid,
    scroll_offset: usize,
    selected_action: FocusReviewAction,
) -> FocusAction {
    match key.code {
        KeyCode::Esc | KeyCode::Char('q') => {
            state.mode = FocusMode::Normal;
            FocusAction::None
        }
        KeyCode::Char('j') => {
            state.mode = FocusMode::ReviewPopup {
                task_id,
                scroll_offset: scroll_offset.saturating_add(1),
                selected_action,
            };
            FocusAction::None
        }
        KeyCode::Char('k') => {
            state.mode = FocusMode::ReviewPopup {
                task_id,
                scroll_offset: scroll_offset.saturating_sub(1),
                selected_action,
            };
            FocusAction::None
        }
        KeyCode::PageDown => {
            state.mode = FocusMode::ReviewPopup {
                task_id,
                scroll_offset: scroll_offset.saturating_add(10),
                selected_action,
            };
            FocusAction::None
        }
        KeyCode::PageUp => {
            state.mode = FocusMode::ReviewPopup {
                task_id,
                scroll_offset: scroll_offset.saturating_sub(10),
                selected_action,
            };
            FocusAction::None
        }
        KeyCode::Left | KeyCode::Char('h') => {
            let actions = FocusReviewAction::all();
            let current_index = actions
                .iter()
                .position(|action| *action == selected_action)
                .unwrap_or(0);
            let new_index = if current_index == 0 {
                actions.len() - 1
            } else {
                current_index - 1
            };
            state.mode = FocusMode::ReviewPopup {
                task_id,
                scroll_offset,
                selected_action: actions[new_index],
            };
            FocusAction::None
        }
        KeyCode::Right | KeyCode::Char('l') | KeyCode::Tab => {
            let actions = FocusReviewAction::all();
            let current_index = actions
                .iter()
                .position(|action| *action == selected_action)
                .unwrap_or(0);
            let new_index = (current_index + 1) % actions.len();
            state.mode = FocusMode::ReviewPopup {
                task_id,
                scroll_offset,
                selected_action: actions[new_index],
            };
            FocusAction::None
        }
        KeyCode::Enter => execute_review_action(state, task_id, selected_action),
        _ => FocusAction::None,
    }
}

fn execute_review_action(
    state: &mut FocusState,
    task_id: Uuid,
    action: FocusReviewAction,
) -> FocusAction {
    match action {
        FocusReviewAction::ReviewInIDE => {
            state.mode = FocusMode::Normal;
            FocusAction::OpenInIDE(task_id)
        }
        FocusReviewAction::ReviewInTerminal => {
            state.mode = FocusMode::Normal;
            FocusAction::SwitchToTerminal(task_id)
        }
        FocusReviewAction::RequestChanges => {
            state.mode = FocusMode::ReviewRequestChanges {
                task_id,
                input: String::new(),
            };
            FocusAction::None
        }
        FocusReviewAction::MergeToBranch => {
            state.mode = FocusMode::Normal;
            FocusAction::MergeBranch(task_id)
        }
    }
}

fn handle_review_request_changes_mode(
    state: &mut FocusState,
    key: KeyEvent,
    task_id: Uuid,
) -> FocusAction {
    let input = match &mut state.mode {
        FocusMode::ReviewRequestChanges { input, .. } => input,
        _ => return FocusAction::None,
    };

    match key.code {
        KeyCode::Char(character) => {
            input.push(character);
            FocusAction::None
        }
        KeyCode::Backspace => {
            input.pop();
            FocusAction::None
        }
        KeyCode::Enter => {
            let change_request = input.clone();
            state.mode = FocusMode::Normal;
            if change_request.is_empty() {
                FocusAction::None
            } else {
                FocusAction::RequestChanges {
                    task_id,
                    message: change_request,
                }
            }
        }
        KeyCode::Esc => {
            state.mode = FocusMode::Normal;
            FocusAction::None
        }
        _ => FocusAction::None,
    }
}
