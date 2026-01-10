use super::{InstanceMode, InstanceState};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers, MouseEvent, MouseEventKind};

pub fn handle_key_event(state: &mut InstanceState, key: KeyEvent) {
    match state.mode {
        InstanceMode::Normal => handle_navigation_mode(state, key),
        InstanceMode::Focused => handle_focused_mode(state, key),
    }
}

fn handle_navigation_mode(state: &mut InstanceState, key: KeyEvent) {
    match key.code {
        KeyCode::Left | KeyCode::Up => state.previous_pane(),
        KeyCode::Right | KeyCode::Down => state.next_pane(),
        KeyCode::Enter => {
            if !state.panes.is_empty() {
                state.mode = InstanceMode::Focused;
            }
        }
        KeyCode::Char('c') => {
            const DEFAULT_ROWS: u16 = 24;
            const DEFAULT_COLUMNS: u16 = 80;
            state.create_pane(DEFAULT_ROWS, DEFAULT_COLUMNS);
        }
        KeyCode::Char('x') => state.close_pane(),
        _ => {}
    }
}

const SCROLL_LINES: usize = 3;
const PAGE_SCROLL_LINES: usize = 10;

fn handle_focused_mode(state: &mut InstanceState, key: KeyEvent) {
    let is_shift_escape =
        key.code == KeyCode::Esc && key.modifiers.contains(KeyModifiers::SHIFT);
    if is_shift_escape {
        send_escape_to_instance(state);
        return;
    }

    if key.code == KeyCode::Esc {
        state.mode = InstanceMode::Normal;
        return;
    }

    let has_shift = key.modifiers.contains(KeyModifiers::SHIFT);
    if has_shift {
        match key.code {
            KeyCode::PageUp => {
                if let Some(pane) = state.selected_pane_mut() {
                    pane.scroll_up(PAGE_SCROLL_LINES);
                }
                return;
            }
            KeyCode::PageDown => {
                if let Some(pane) = state.selected_pane_mut() {
                    pane.scroll_down(PAGE_SCROLL_LINES);
                }
                return;
            }
            KeyCode::Home => {
                if let Some(pane) = state.selected_pane_mut()
                    && let Some(session) = &pane.pty_session
                {
                    pane.scroll_offset = session.scrollback_len();
                }
                return;
            }
            KeyCode::End => {
                if let Some(pane) = state.selected_pane_mut() {
                    pane.scroll_to_bottom();
                }
                return;
            }
            _ => {}
        }
    }

    if let Some(pane) = state.selected_pane_mut() {
        pane.scroll_to_bottom();
    }

    send_input_to_instance(state, key);
}

fn send_escape_to_instance(state: &mut InstanceState) {
    if let Some(pane) = state.selected_pane_mut() {
        pane.scroll_to_bottom();
        if let Some(session) = &mut pane.pty_session {
            let _ = session.write_input(b"\x1b");
        }
    }
}

fn send_input_to_instance(state: &mut InstanceState, key: KeyEvent) {
    if let Some(pane) = state.selected_pane_mut()
        && let Some(session) = &mut pane.pty_session
    {
        let data = match key.code {
            KeyCode::Char(c) => {
                if key.modifiers.contains(KeyModifiers::CONTROL) {
                    if c.is_ascii_alphabetic() {
                        let control_char = (c.to_ascii_lowercase() as u8) - b'a' + 1;
                        vec![control_char]
                    } else {
                        return;
                    }
                } else {
                    c.to_string().into_bytes()
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
            _ => return,
        };

        let _ = session.write_input(&data);
    }
}

pub fn handle_mouse_event(state: &mut InstanceState, mouse: MouseEvent) {
    let is_scroll_event = matches!(
        mouse.kind,
        MouseEventKind::ScrollUp | MouseEventKind::ScrollDown
    );
    if !is_scroll_event {
        return;
    }

    let Some(render_area) = state.last_render_area else {
        return;
    };

    let pane_areas =
        super::layout::calculate_pane_areas(render_area, state.layout_mode, state.panes.len());

    let pane_index = find_pane_at_position(&pane_areas, mouse.column, mouse.row);
    let Some(index) = pane_index else {
        return;
    };

    let Some(pane) = state.panes.get_mut(index) else {
        return;
    };

    match mouse.kind {
        MouseEventKind::ScrollUp => pane.scroll_up(SCROLL_LINES),
        MouseEventKind::ScrollDown => pane.scroll_down(SCROLL_LINES),
        _ => {}
    }
}

fn find_pane_at_position(
    pane_areas: &[ratatui::layout::Rect],
    mouse_x: u16,
    mouse_y: u16,
) -> Option<usize> {
    for (index, area) in pane_areas.iter().enumerate() {
        let is_within_x = mouse_x >= area.x && mouse_x < area.x + area.width;
        let is_within_y = mouse_y >= area.y && mouse_y < area.y + area.height;
        if is_within_x && is_within_y {
            return Some(index);
        }
    }
    None
}
