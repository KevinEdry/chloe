use super::TasksAction;
use crate::views::tasks::state::TasksState;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use uuid::Uuid;

pub fn handle_terminal_focused_mode(
    state: &mut TasksState,
    key: KeyEvent,
    selected_instance_id: Option<Uuid>,
) -> TasksAction {
    let is_shift_escape =
        key.code == KeyCode::Esc && key.modifiers.contains(KeyModifiers::SHIFT);
    if is_shift_escape {
        let Some(instance_id) = selected_instance_id else {
            return TasksAction::None;
        };
        return TasksAction::SendToTerminal(instance_id, b"\x1b".to_vec());
    }

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
