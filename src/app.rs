use crate::kanban::KanbanState;
use crate::terminal::TerminalState;
use crate::types::Config;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Tab {
    Kanban,
    Terminals,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct App {
    pub active_tab: Tab,
    pub kanban: KanbanState,
    pub terminals: TerminalState,
    #[serde(skip)]
    pub config: Config,
}

impl App {
    pub fn new() -> Self {
        Self {
            active_tab: Tab::Kanban,
            kanban: KanbanState::new(),
            terminals: TerminalState::new(),
            config: Config::default(),
        }
    }

    /// Load state from disk, or create new if it doesn't exist
    pub fn load_or_default() -> Self {
        match crate::persistence::storage::load_state() {
            Ok(mut app) => {
                // Restore config since it's skipped in serialization
                app.config = Config::default();
                app
            }
            Err(_) => Self::default(),
        }
    }

    /// Save the current state to disk
    pub fn save(&self) -> crate::types::Result<()> {
        crate::persistence::storage::save_state(self)
    }

    pub fn switch_tab(&mut self, tab: Tab) {
        self.active_tab = tab;
    }

    pub fn next_tab(&mut self) {
        self.active_tab = match self.active_tab {
            Tab::Kanban => Tab::Terminals,
            Tab::Terminals => Tab::Kanban,
        };
    }
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}
