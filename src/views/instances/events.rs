use super::TerminalAction;
use super::operations::NavigationDirection;
use super::state::{InstanceMode, InstanceState};
use crate::events::{AppAction, EventHandler, EventResult};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

const DEFAULT_PTY_ROWS: u16 = 24;
const DEFAULT_PTY_COLUMNS: u16 = 80;
const SCROLL_LINES_SINGLE: usize = 1;
const SCROLL_LINES_HALF_PAGE: usize = 12;

impl EventHandler for InstanceState {
    fn handle_key(&mut self, key: KeyEvent) -> EventResult {
        match self.mode {
            InstanceMode::Normal => self.handle_navigation_mode(key),
            InstanceMode::Focused => self.handle_focused_mode(key),
            InstanceMode::Scroll => self.handle_scroll_mode(key),
            InstanceMode::ActivitySummary => self.handle_activity_summary_mode(key),
        }
    }
}

impl InstanceState {
    fn handle_navigation_mode(&mut self, key: KeyEvent) -> EventResult {
        match key.code {
            KeyCode::Char('h') | KeyCode::Left => {
                self.navigate_to_pane_in_direction(NavigationDirection::Left);
                EventResult::Consumed
            }
            KeyCode::Char('j') | KeyCode::Down => {
                self.navigate_to_pane_in_direction(NavigationDirection::Down);
                EventResult::Consumed
            }
            KeyCode::Char('k') | KeyCode::Up => {
                self.navigate_to_pane_in_direction(NavigationDirection::Up);
                EventResult::Consumed
            }
            KeyCode::Char('l') | KeyCode::Right => {
                self.navigate_to_pane_in_direction(NavigationDirection::Right);
                EventResult::Consumed
            }
            KeyCode::Enter => {
                if self.pane_count() > 0 {
                    self.mode = InstanceMode::Focused;
                }
                EventResult::Consumed
            }
            KeyCode::Char('c') => {
                self.create_pane(DEFAULT_PTY_ROWS, DEFAULT_PTY_COLUMNS);
                EventResult::Consumed
            }
            KeyCode::Char('x') => {
                self.close_pane();
                EventResult::Consumed
            }
            KeyCode::Tab => {
                self.next_pane();
                EventResult::Consumed
            }
            KeyCode::BackTab => {
                self.previous_pane();
                EventResult::Consumed
            }
            KeyCode::Char('A') => {
                if self.selected_pane_id.is_some() {
                    self.activity_summary_scroll_offset = 0;
                    self.mode = InstanceMode::ActivitySummary;
                }
                EventResult::Consumed
            }
            _ => EventResult::Ignored,
        }
    }

    fn handle_focused_mode(&mut self, key: KeyEvent) -> EventResult {
        let is_shift_escape =
            key.code == KeyCode::Esc && key.modifiers.contains(KeyModifiers::SHIFT);
        if is_shift_escape {
            return self.send_escape_to_terminal();
        }

        if key.code == KeyCode::Esc {
            self.mode = InstanceMode::Normal;
            return EventResult::Consumed;
        }

        let is_ctrl_s =
            key.code == KeyCode::Char('s') && key.modifiers.contains(KeyModifiers::CONTROL);
        if is_ctrl_s {
            self.mode = InstanceMode::Scroll;
            return EventResult::Consumed;
        }

        if let Some(pane) = self.selected_pane_mut() {
            pane.scroll_to_bottom();
        }

        self.send_input_to_terminal(key)
    }

    fn handle_scroll_mode(&mut self, key: KeyEvent) -> EventResult {
        if key.code == KeyCode::Esc || key.code == KeyCode::Char('q') {
            if let Some(pane) = self.selected_pane_mut() {
                pane.scroll_to_bottom();
            }
            self.mode = InstanceMode::Focused;
            return EventResult::Consumed;
        }

        let has_ctrl = key.modifiers.contains(KeyModifiers::CONTROL);

        match key.code {
            KeyCode::Char('j') | KeyCode::Down => {
                if let Some(pane) = self.selected_pane_mut() {
                    pane.scroll_down(SCROLL_LINES_SINGLE);
                }
                EventResult::Consumed
            }
            KeyCode::Char('k') | KeyCode::Up => {
                if let Some(pane) = self.selected_pane_mut() {
                    let max_scrollback = pane.scrollback_len();
                    pane.scroll_up(SCROLL_LINES_SINGLE, max_scrollback);
                }
                EventResult::Consumed
            }
            KeyCode::Char('d') if has_ctrl => {
                if let Some(pane) = self.selected_pane_mut() {
                    pane.scroll_down(SCROLL_LINES_HALF_PAGE);
                }
                EventResult::Consumed
            }
            KeyCode::Char('u') if has_ctrl => {
                if let Some(pane) = self.selected_pane_mut() {
                    let max_scrollback = pane.scrollback_len();
                    pane.scroll_up(SCROLL_LINES_HALF_PAGE, max_scrollback);
                }
                EventResult::Consumed
            }
            KeyCode::Char('\x04') => {
                if let Some(pane) = self.selected_pane_mut() {
                    pane.scroll_down(SCROLL_LINES_HALF_PAGE);
                }
                EventResult::Consumed
            }
            KeyCode::Char('\x15') => {
                if let Some(pane) = self.selected_pane_mut() {
                    let max_scrollback = pane.scrollback_len();
                    pane.scroll_up(SCROLL_LINES_HALF_PAGE, max_scrollback);
                }
                EventResult::Consumed
            }
            KeyCode::Char('g') => {
                if let Some(pane) = self.selected_pane_mut() {
                    pane.scroll_offset = pane.scrollback_len();
                }
                EventResult::Consumed
            }
            KeyCode::Char('G') => {
                if let Some(pane) = self.selected_pane_mut() {
                    pane.scroll_to_bottom();
                }
                EventResult::Consumed
            }
            _ => EventResult::Ignored,
        }
    }

    fn handle_activity_summary_mode(&mut self, key: KeyEvent) -> EventResult {
        if key.code == KeyCode::Esc || key.code == KeyCode::Char('q') {
            if let Some(pane) = self.selected_pane_mut() {
                pane.mark_viewed();
            }
            self.activity_summary_scroll_offset = 0;
            self.mode = InstanceMode::Normal;
            return EventResult::Consumed;
        }

        match key.code {
            KeyCode::Char('j') | KeyCode::Down => {
                self.activity_summary_scroll_offset =
                    self.activity_summary_scroll_offset.saturating_add(1);
                EventResult::Consumed
            }
            KeyCode::Char('k') | KeyCode::Up => {
                self.activity_summary_scroll_offset =
                    self.activity_summary_scroll_offset.saturating_sub(1);
                EventResult::Consumed
            }
            KeyCode::Char('d') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                self.activity_summary_scroll_offset = self
                    .activity_summary_scroll_offset
                    .saturating_add(SCROLL_LINES_HALF_PAGE);
                EventResult::Consumed
            }
            KeyCode::Char('u') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                self.activity_summary_scroll_offset = self
                    .activity_summary_scroll_offset
                    .saturating_sub(SCROLL_LINES_HALF_PAGE);
                EventResult::Consumed
            }
            KeyCode::Char('g') => {
                self.activity_summary_scroll_offset = 0;
                EventResult::Consumed
            }
            _ => EventResult::Ignored,
        }
    }

    fn send_escape_to_terminal(&mut self) -> EventResult {
        let Some(pane) = self.selected_pane_mut() else {
            return EventResult::Ignored;
        };

        pane.scroll_to_bottom();

        let Some(pane_id) = self.selected_pane_id else {
            return EventResult::Ignored;
        };

        EventResult::Action(AppAction::Terminal(TerminalAction::SendInput {
            instance_id: pane_id,
            data: b"\x1b".to_vec(),
        }))
    }

    fn send_input_to_terminal(&self, key: KeyEvent) -> EventResult {
        let Some(pane_id) = self.selected_pane_id else {
            return EventResult::Ignored;
        };

        let data = match key.code {
            KeyCode::Char(character) => {
                if key.modifiers.contains(KeyModifiers::CONTROL) {
                    if character.is_ascii_alphabetic() {
                        let control_char = (character.to_ascii_lowercase() as u8) - b'a' + 1;
                        vec![control_char]
                    } else {
                        return EventResult::Ignored;
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
            _ => return EventResult::Ignored,
        };

        EventResult::Action(AppAction::Terminal(TerminalAction::SendInput {
            instance_id: pane_id,
            data,
        }))
    }
}
