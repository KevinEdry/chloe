/// Input mode determines how keyboard events are handled
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputMode {
    /// Normal navigation mode - app handles keybindings
    Normal,
}

impl Default for InputMode {
    fn default() -> Self {
        Self::Normal
    }
}
