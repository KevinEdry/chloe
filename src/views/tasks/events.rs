use super::operations::{
    get_active_task_count, get_active_tasks, get_done_task_count, get_done_tasks,
};
use super::state::{FocusPanel, ReviewAction, TasksMode, TasksState, TasksViewMode};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use uuid::Uuid;

pub enum TasksAction {
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
    state: &mut TasksState,
    key: KeyEvent,
    selected_instance_id: Option<Uuid>,
) -> TasksAction {
    if state.mode == TasksMode::Normal && key.code == KeyCode::Char('/') {
        state.toggle_view_mode();
        return TasksAction::None;
    }

    match &state.mode {
        TasksMode::Normal => match state.view_mode {
            TasksViewMode::Focus => handle_focus_normal_mode(state, key, selected_instance_id),
            TasksViewMode::Kanban => {
                handle_kanban_normal_mode(state, key);
                TasksAction::None
            }
        },
        TasksMode::TerminalFocused => {
            handle_terminal_focused_mode(state, key, selected_instance_id)
        }
        TasksMode::AddingTask { .. } => handle_adding_task_mode(state, key),
        TasksMode::EditingTask { .. } => handle_editing_task_mode(state, key),
        TasksMode::ConfirmDelete { task_id } => handle_confirm_delete_mode(state, key, *task_id),
        TasksMode::ConfirmStartTask { task_id } => {
            handle_confirm_start_task_mode(state, key, *task_id)
        }
        TasksMode::ConfirmMoveBack { task_id } => {
            handle_confirm_move_back_mode(state, key, *task_id)
        }
        TasksMode::ClassifyingTask { .. } => handle_classifying_task_mode(state, key),
        TasksMode::ReviewPopup {
            task_id,
            scroll_offset,
            selected_action,
        } => handle_review_popup_mode(state, key, *task_id, *scroll_offset, *selected_action),
        TasksMode::ReviewRequestChanges { task_id, .. } => {
            handle_review_request_changes_mode(state, key, *task_id)
        }
    }
}

fn handle_focus_normal_mode(
    state: &mut TasksState,
    key: KeyEvent,
    selected_instance_id: Option<Uuid>,
) -> TasksAction {
    let active_count = get_active_task_count(&state.columns);
    let done_count = get_done_task_count(&state.columns);

    let selected_task = match state.focus_panel {
        FocusPanel::ActiveTasks => {
            let tasks = get_active_tasks(&state.columns);
            tasks.into_iter().nth(state.focus_active_index)
        }
        FocusPanel::DoneTasks => {
            let tasks = get_done_tasks(&state.columns);
            tasks.into_iter().nth(state.focus_done_index)
        }
    };

    match key.code {
        KeyCode::Char('j') | KeyCode::Down => {
            state.focus_select_next();
            TasksAction::None
        }
        KeyCode::Char('k') | KeyCode::Up => {
            state.focus_select_previous();
            TasksAction::None
        }
        KeyCode::Tab => {
            match state.focus_panel {
                FocusPanel::ActiveTasks => state.focus_switch_to_done_panel(),
                FocusPanel::DoneTasks => state.focus_switch_to_active_panel(),
            }
            TasksAction::None
        }
        KeyCode::Char('a') => {
            state.mode = TasksMode::AddingTask {
                input: String::new(),
            };
            TasksAction::None
        }
        KeyCode::Char('e') => {
            if let Some(task_ref) = selected_task {
                state.mode = TasksMode::EditingTask {
                    task_id: task_ref.task.id,
                    input: task_ref.task.title.clone(),
                };
            }
            TasksAction::None
        }
        KeyCode::Char('d') => {
            if let Some(task_ref) = selected_task {
                state.mode = TasksMode::ConfirmDelete {
                    task_id: task_ref.task.id,
                };
            }
            TasksAction::None
        }
        KeyCode::Enter => {
            if let Some(task_ref) = selected_task {
                let is_planning = task_ref.column_index == 0;
                let is_in_progress = task_ref.column_index == 1;
                let is_review = task_ref.column_index == 2;

                if is_planning {
                    state.mode = TasksMode::ConfirmStartTask {
                        task_id: task_ref.task.id,
                    };
                    TasksAction::None
                } else if is_in_progress {
                    if let Some(instance_id) = selected_instance_id {
                        state.enter_terminal_mode();
                        TasksAction::SendToTerminal(instance_id, Vec::new())
                    } else {
                        TasksAction::None
                    }
                } else if is_review {
                    state.mode = TasksMode::ReviewPopup {
                        task_id: task_ref.task.id,
                        scroll_offset: 0,
                        selected_action: ReviewAction::ReviewInIDE,
                    };
                    TasksAction::None
                } else {
                    TasksAction::None
                }
            } else {
                TasksAction::None
            }
        }
        KeyCode::Char('t') => {
            if let Some(instance_id) = selected_instance_id {
                TasksAction::JumpToInstance(instance_id)
            } else {
                TasksAction::None
            }
        }
        KeyCode::Char('s') => {
            if let Some(task_ref) = selected_task {
                let is_planning = task_ref.column_index == 0;
                if is_planning {
                    state.mode = TasksMode::ConfirmStartTask {
                        task_id: task_ref.task.id,
                    };
                }
            }
            TasksAction::None
        }
        KeyCode::Char('g') => {
            match state.focus_panel {
                FocusPanel::ActiveTasks => {
                    if active_count > 0 {
                        state.focus_active_index = 0;
                    }
                }
                FocusPanel::DoneTasks => {
                    if done_count > 0 {
                        state.focus_done_index = 0;
                    }
                }
            }
            TasksAction::None
        }
        KeyCode::Char('G') => {
            match state.focus_panel {
                FocusPanel::ActiveTasks => {
                    if active_count > 0 {
                        state.focus_active_index = active_count - 1;
                    }
                }
                FocusPanel::DoneTasks => {
                    if done_count > 0 {
                        state.focus_done_index = done_count - 1;
                    }
                }
            }
            TasksAction::None
        }
        _ => TasksAction::None,
    }
}

fn handle_kanban_normal_mode(state: &mut TasksState, key: KeyEvent) {
    match key.code {
        KeyCode::Left | KeyCode::Char('h') => state.previous_column(),
        KeyCode::Right | KeyCode::Char('l') => state.next_column(),
        KeyCode::Up | KeyCode::Char('k') => state.previous_task(),
        KeyCode::Down | KeyCode::Char('j') => state.next_task(),
        KeyCode::Char('a') => {
            state.mode = TasksMode::AddingTask {
                input: String::new(),
            };
        }
        KeyCode::Char('e') => {
            if let Some(task) = state.get_kanban_selected_task() {
                let task_id = task.id;
                let title = task.title.clone();
                state.mode = TasksMode::EditingTask {
                    task_id,
                    input: title,
                };
            }
        }
        KeyCode::Char('d') => {
            if let Some(task) = state.get_kanban_selected_task() {
                state.mode = TasksMode::ConfirmDelete { task_id: task.id };
            }
        }
        KeyCode::Enter => {
            let is_review_column = state.kanban_selected_column == 2;
            let is_in_progress_column = state.kanban_selected_column == 1;

            if is_review_column {
                if let Some(task) = state.get_kanban_selected_task() {
                    state.mode = TasksMode::ReviewPopup {
                        task_id: task.id,
                        scroll_offset: 0,
                        selected_action: ReviewAction::ReviewInIDE,
                    };
                }
            } else if !is_in_progress_column {
                state.move_task_next();
            }
        }
        KeyCode::Backspace => {
            state.move_task_previous();
        }
        _ => {}
    }
}

fn handle_adding_task_mode(state: &mut TasksState, key: KeyEvent) -> TasksAction {
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

fn handle_editing_task_mode(state: &mut TasksState, key: KeyEvent) -> TasksAction {
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

fn handle_confirm_delete_mode(state: &mut TasksState, key: KeyEvent, task_id: Uuid) -> TasksAction {
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

fn handle_confirm_start_task_mode(
    state: &mut TasksState,
    key: KeyEvent,
    task_id: Uuid,
) -> TasksAction {
    match key.code {
        KeyCode::Char('y' | 'Y') | KeyCode::Enter => {
            state.mode = TasksMode::Normal;
            TasksAction::StartTask(task_id)
        }
        KeyCode::Char('n' | 'N') | KeyCode::Esc => {
            state.mode = TasksMode::Normal;
            TasksAction::None
        }
        _ => TasksAction::None,
    }
}

fn handle_confirm_move_back_mode(
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

fn handle_classifying_task_mode(state: &mut TasksState, key: KeyEvent) -> TasksAction {
    if key.code == KeyCode::Esc {
        state.mode = TasksMode::Normal;
        return TasksAction::CancelClassification;
    }
    TasksAction::None
}

fn handle_terminal_focused_mode(
    state: &mut TasksState,
    key: KeyEvent,
    selected_instance_id: Option<Uuid>,
) -> TasksAction {
    if key.code == KeyCode::Esc {
        state.exit_terminal_mode();
        return TasksAction::None;
    }

    let Some(instance_id) = selected_instance_id else {
        state.exit_terminal_mode();
        return TasksAction::None;
    };

    let data = convert_key_to_bytes(key);
    if data.is_empty() {
        return TasksAction::None;
    }

    TasksAction::SendToTerminal(instance_id, data)
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

fn handle_review_request_changes_mode(
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
