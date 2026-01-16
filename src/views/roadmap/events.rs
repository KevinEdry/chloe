use super::state::{RoadmapMode, RoadmapPriority, RoadmapState};
use super::RoadmapAction as AppRoadmapAction;
use crate::events::{AppAction, EventHandler, EventResult, SettingsAction};
use crossterm::event::{KeyCode, KeyEvent};

enum RoadmapAction {
    None,
    ConvertToTask(usize),
    SaveState,
    GenerateRoadmap,
}

impl EventHandler for RoadmapState {
    fn handle_key(&mut self, key: KeyEvent) -> EventResult {
        let action = match &self.mode {
            RoadmapMode::Normal => self.handle_normal_mode(key),
            RoadmapMode::AddingItem { .. } => self.handle_adding_item(key),
            RoadmapMode::EditingItem { .. } => self.handle_editing_item(key),
            RoadmapMode::ConfirmDelete { item_index } => {
                self.handle_confirm_delete(key, *item_index)
            }
            RoadmapMode::ConvertToTask { item_index } => {
                self.handle_convert_to_task(key, *item_index)
            }
            RoadmapMode::Generating => self.handle_generating_mode(key),
        };

        match action {
            RoadmapAction::None => EventResult::Consumed,
            RoadmapAction::ConvertToTask(index) => EventResult::Action(AppAction::Roadmap(
                AppRoadmapAction::ConvertToTask(index),
            )),
            RoadmapAction::SaveState => {
                EventResult::Action(AppAction::Settings(SettingsAction::SaveState))
            }
            RoadmapAction::GenerateRoadmap => {
                EventResult::Action(AppAction::Roadmap(AppRoadmapAction::Generate))
            }
        }
    }
}

impl RoadmapState {
    fn handle_normal_mode(&mut self, key: KeyEvent) -> RoadmapAction {
        match key.code {
            KeyCode::Char('j') | KeyCode::Down => {
                self.select_next();
                RoadmapAction::None
            }
            KeyCode::Char('k') | KeyCode::Up => {
                self.select_previous();
                RoadmapAction::None
            }
            KeyCode::Char('a') => {
                self.mode = RoadmapMode::AddingItem {
                    input: String::new(),
                };
                RoadmapAction::None
            }
            KeyCode::Char('e') => {
                if let Some(index) = self.selected_item
                    && let Some(item) = self.items.get(index)
                {
                    self.mode = RoadmapMode::EditingItem {
                        item_index: index,
                        input: item.title.clone(),
                    };
                }
                RoadmapAction::None
            }
            KeyCode::Char('d') => {
                if let Some(index) = self.selected_item {
                    self.mode = RoadmapMode::ConfirmDelete { item_index: index };
                }
                RoadmapAction::None
            }
            KeyCode::Char('t') => {
                if let Some(index) = self.selected_item {
                    self.mode = RoadmapMode::ConvertToTask { item_index: index };
                }
                RoadmapAction::None
            }
            KeyCode::Char('p') => {
                if let Some(index) = self.selected_item
                    && let Some(item) = self.items.get(index)
                {
                    let new_priority = match item.priority {
                        RoadmapPriority::Low => RoadmapPriority::Medium,
                        RoadmapPriority::Medium => RoadmapPriority::High,
                        RoadmapPriority::High => RoadmapPriority::Low,
                    };
                    self.update_item_priority(index, new_priority);
                    return RoadmapAction::SaveState;
                }
                RoadmapAction::None
            }
            KeyCode::Char('g' | 'G') => RoadmapAction::GenerateRoadmap,
            _ => RoadmapAction::None,
        }
    }

    fn handle_adding_item(&mut self, key: KeyEvent) -> RoadmapAction {
        let input = if let RoadmapMode::AddingItem { input } = &self.mode {
            input.clone()
        } else {
            return RoadmapAction::None;
        };

        match key.code {
            KeyCode::Enter => {
                if !input.trim().is_empty() {
                    self.add_item(
                        input.trim().to_string(),
                        String::new(),
                        String::new(),
                        RoadmapPriority::Medium,
                    );
                    self.mode = RoadmapMode::Normal;
                    return RoadmapAction::SaveState;
                }
                RoadmapAction::None
            }
            KeyCode::Esc => {
                self.mode = RoadmapMode::Normal;
                RoadmapAction::None
            }
            KeyCode::Char(character) => {
                let mut new_input = input;
                new_input.push(character);
                self.mode = RoadmapMode::AddingItem { input: new_input };
                RoadmapAction::None
            }
            KeyCode::Backspace => {
                let mut new_input = input;
                new_input.pop();
                self.mode = RoadmapMode::AddingItem { input: new_input };
                RoadmapAction::None
            }
            _ => RoadmapAction::None,
        }
    }

    fn handle_editing_item(&mut self, key: KeyEvent) -> RoadmapAction {
        let (item_index, input) = if let RoadmapMode::EditingItem { item_index, input } = &self.mode
        {
            (*item_index, input.clone())
        } else {
            return RoadmapAction::None;
        };

        match key.code {
            KeyCode::Enter => {
                if !input.trim().is_empty() {
                    self.update_item_title(item_index, input.trim().to_string());
                    self.mode = RoadmapMode::Normal;
                    return RoadmapAction::SaveState;
                }
                RoadmapAction::None
            }
            KeyCode::Esc => {
                self.mode = RoadmapMode::Normal;
                RoadmapAction::None
            }
            KeyCode::Char(character) => {
                let mut new_input = input;
                new_input.push(character);
                self.mode = RoadmapMode::EditingItem {
                    item_index,
                    input: new_input,
                };
                RoadmapAction::None
            }
            KeyCode::Backspace => {
                let mut new_input = input;
                new_input.pop();
                self.mode = RoadmapMode::EditingItem {
                    item_index,
                    input: new_input,
                };
                RoadmapAction::None
            }
            _ => RoadmapAction::None,
        }
    }

    fn handle_confirm_delete(&mut self, key: KeyEvent, item_index: usize) -> RoadmapAction {
        match key.code {
            KeyCode::Char('y' | 'Y') => {
                self.delete_item(item_index);
                self.mode = RoadmapMode::Normal;
                RoadmapAction::SaveState
            }
            KeyCode::Char('n' | 'N') | KeyCode::Esc => {
                self.mode = RoadmapMode::Normal;
                RoadmapAction::None
            }
            _ => RoadmapAction::None,
        }
    }

    fn handle_convert_to_task(&mut self, key: KeyEvent, item_index: usize) -> RoadmapAction {
        match key.code {
            KeyCode::Char('y' | 'Y') | KeyCode::Enter => {
                self.mode = RoadmapMode::Normal;
                RoadmapAction::ConvertToTask(item_index)
            }
            KeyCode::Char('n' | 'N') | KeyCode::Esc => {
                self.mode = RoadmapMode::Normal;
                RoadmapAction::None
            }
            _ => RoadmapAction::None,
        }
    }

    fn handle_generating_mode(&mut self, key: KeyEvent) -> RoadmapAction {
        match key.code {
            KeyCode::Esc => {
                self.mode = RoadmapMode::Normal;
                RoadmapAction::None
            }
            _ => RoadmapAction::None,
        }
    }
}
