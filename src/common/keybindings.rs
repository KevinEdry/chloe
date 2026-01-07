use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

/// Centralized keybinding configuration
pub struct Keybindings;

impl Keybindings {
    /// Check if key event is tab switch (Tab key or number keys 1-2)
    pub const fn is_tab_switch(key: &KeyEvent) -> bool {
        matches!(
            key.code,
            KeyCode::Tab | KeyCode::Char('1') | KeyCode::Char('2')
        )
    }

    /// Check if key event is quit command
    pub const fn is_quit(key: &KeyEvent) -> bool {
        matches!(key.code, KeyCode::Char('q'))
            || (matches!(key.code, KeyCode::Char('c'))
                && key.modifiers.contains(KeyModifiers::CONTROL))
    }

    /// Check if key event is help overlay toggle
    pub const fn is_help(key: &KeyEvent) -> bool {
        matches!(key.code, KeyCode::Char('?'))
    }
}
