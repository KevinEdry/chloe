use super::state::{RoadmapMode, RoadmapPriority, RoadmapState};
use crossterm::event::{KeyCode, KeyEvent};

pub enum RoadmapAction {
    None,
    ConvertToTask(usize),
    SaveState,
    GenerateRoadmap,
}

pub fn handle_key_event(state: &mut RoadmapState, key: KeyEvent) -> RoadmapAction {
    match &state.mode {
        RoadmapMode::Normal => handle_normal_mode(state, key),
        RoadmapMode::AddingItem { .. } => handle_adding_item(state, key),
        RoadmapMode::EditingItem { .. } => handle_editing_item(state, key),
        RoadmapMode::ConfirmDelete { item_index } => handle_confirm_delete(state, key, *item_index),
        RoadmapMode::ConvertToTask { item_index } => {
            handle_convert_to_task(state, key, *item_index)
        }
        RoadmapMode::Generating => handle_generating_mode(state, key),
    }
}

fn handle_normal_mode(state: &mut RoadmapState, key: KeyEvent) -> RoadmapAction {
    match key.code {
        KeyCode::Char('j') | KeyCode::Down => {
            state.select_next();
            RoadmapAction::None
        }
        KeyCode::Char('k') | KeyCode::Up => {
            state.select_previous();
            RoadmapAction::None
        }
        KeyCode::Char('a') => {
            state.mode = RoadmapMode::AddingItem {
                input: String::new(),
            };
            RoadmapAction::None
        }
        KeyCode::Char('e') => {
            if let Some(index) = state.selected_item {
                if let Some(item) = state.items.get(index) {
                    state.mode = RoadmapMode::EditingItem {
                        item_index: index,
                        input: item.title.clone(),
                    };
                }
            }
            RoadmapAction::None
        }
        KeyCode::Char('d') => {
            if let Some(index) = state.selected_item {
                state.mode = RoadmapMode::ConfirmDelete { item_index: index };
            }
            RoadmapAction::None
        }
        KeyCode::Char('t') => {
            if let Some(index) = state.selected_item {
                state.mode = RoadmapMode::ConvertToTask { item_index: index };
            }
            RoadmapAction::None
        }
        KeyCode::Char('p') => {
            if let Some(index) = state.selected_item {
                if let Some(item) = state.items.get(index) {
                    let new_priority = match item.priority {
                        RoadmapPriority::Low => RoadmapPriority::Medium,
                        RoadmapPriority::Medium => RoadmapPriority::High,
                        RoadmapPriority::High => RoadmapPriority::Low,
                    };
                    state.update_item_priority(index, new_priority);
                    return RoadmapAction::SaveState;
                }
            }
            RoadmapAction::None
        }
        KeyCode::Char('g' | 'G') => RoadmapAction::GenerateRoadmap,
        _ => RoadmapAction::None,
    }
}

fn handle_adding_item(state: &mut RoadmapState, key: KeyEvent) -> RoadmapAction {
    let input = if let RoadmapMode::AddingItem { input } = &state.mode {
        input.clone()
    } else {
        return RoadmapAction::None;
    };

    match key.code {
        KeyCode::Enter => {
            if !input.trim().is_empty() {
                state.add_item(
                    input.trim().to_string(),
                    String::new(),
                    String::new(),
                    RoadmapPriority::Medium,
                );
                state.mode = RoadmapMode::Normal;
                return RoadmapAction::SaveState;
            }
            RoadmapAction::None
        }
        KeyCode::Esc => {
            state.mode = RoadmapMode::Normal;
            RoadmapAction::None
        }
        KeyCode::Char(character) => {
            let mut new_input = input;
            new_input.push(character);
            state.mode = RoadmapMode::AddingItem { input: new_input };
            RoadmapAction::None
        }
        KeyCode::Backspace => {
            let mut new_input = input;
            new_input.pop();
            state.mode = RoadmapMode::AddingItem { input: new_input };
            RoadmapAction::None
        }
        _ => RoadmapAction::None,
    }
}

fn handle_editing_item(state: &mut RoadmapState, key: KeyEvent) -> RoadmapAction {
    let (item_index, input) = if let RoadmapMode::EditingItem { item_index, input } = &state.mode {
        (*item_index, input.clone())
    } else {
        return RoadmapAction::None;
    };

    match key.code {
        KeyCode::Enter => {
            if !input.trim().is_empty() {
                state.update_item_title(item_index, input.trim().to_string());
                state.mode = RoadmapMode::Normal;
                return RoadmapAction::SaveState;
            }
            RoadmapAction::None
        }
        KeyCode::Esc => {
            state.mode = RoadmapMode::Normal;
            RoadmapAction::None
        }
        KeyCode::Char(character) => {
            let mut new_input = input;
            new_input.push(character);
            state.mode = RoadmapMode::EditingItem {
                item_index,
                input: new_input,
            };
            RoadmapAction::None
        }
        KeyCode::Backspace => {
            let mut new_input = input;
            new_input.pop();
            state.mode = RoadmapMode::EditingItem {
                item_index,
                input: new_input,
            };
            RoadmapAction::None
        }
        _ => RoadmapAction::None,
    }
}

fn handle_confirm_delete(
    state: &mut RoadmapState,
    key: KeyEvent,
    item_index: usize,
) -> RoadmapAction {
    match key.code {
        KeyCode::Char('y' | 'Y') => {
            state.delete_item(item_index);
            state.mode = RoadmapMode::Normal;
            RoadmapAction::SaveState
        }
        KeyCode::Char('n' | 'N') | KeyCode::Esc => {
            state.mode = RoadmapMode::Normal;
            RoadmapAction::None
        }
        _ => RoadmapAction::None,
    }
}

fn handle_convert_to_task(
    state: &mut RoadmapState,
    key: KeyEvent,
    item_index: usize,
) -> RoadmapAction {
    match key.code {
        KeyCode::Char('y' | 'Y') | KeyCode::Enter => {
            state.mode = RoadmapMode::Normal;
            RoadmapAction::ConvertToTask(item_index)
        }
        KeyCode::Char('n' | 'N') | KeyCode::Esc => {
            state.mode = RoadmapMode::Normal;
            RoadmapAction::None
        }
        _ => RoadmapAction::None,
    }
}

fn handle_generating_mode(state: &mut RoadmapState, key: KeyEvent) -> RoadmapAction {
    match key.code {
        KeyCode::Esc => {
            state.generation_request = None;
            state.mode = RoadmapMode::Normal;
            RoadmapAction::None
        }
        _ => RoadmapAction::None,
    }
}
