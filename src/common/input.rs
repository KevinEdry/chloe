/// Input mode determines how keyboard events are handled
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputMode {
    /// Normal navigation mode - app handles keybindings
    Normal,
    /// Text input mode - for adding/editing tasks
    TextEntry,
    /// Terminal mode - all input goes to PTY
    Terminal,
}

impl Default for InputMode {
    fn default() -> Self {
        Self::Normal
    }
}
