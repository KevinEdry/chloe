use serde::{Deserialize, Serialize};

// Placeholder for terminal state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TerminalState {
    // TODO: Add actual terminal state when implemented
}

impl TerminalState {
    pub const fn new() -> Self {
        Self {}
    }
}

impl Default for TerminalState {
    fn default() -> Self {
        Self::new()
    }
}

pub struct TerminalPane;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LayoutMode {
    Single,
}
