use ratatui::layout::Rect;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstanceState {
    pub panes: Vec<InstancePane>,
    pub selected_pane: usize,
    pub layout_mode: LayoutMode,
    pub mode: InstanceMode,
    #[serde(skip)]
    pub last_render_area: Option<Rect>,
}

impl InstanceState {
    pub fn new() -> Self {
        Self {
            panes: Vec::new(),
            selected_pane: 0,
            layout_mode: LayoutMode::Grid,
            mode: InstanceMode::Normal,
            last_render_area: None,
        }
    }

    pub fn selected_pane_mut(&mut self) -> Option<&mut InstancePane> {
        self.panes.get_mut(self.selected_pane)
    }
}

impl Default for InstanceState {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ClaudeState {
    Idle,
    Running,
    NeedsPermissions,
    Done,
}

impl Default for ClaudeState {
    fn default() -> Self {
        Self::Idle
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstancePane {
    pub id: Uuid,
    pub working_directory: PathBuf,
    pub rows: u16,
    pub columns: u16,
    #[serde(skip)]
    pub pty_session: Option<crate::instance::pty::PtySession>,
    #[serde(default)]
    pub claude_state: ClaudeState,
    #[serde(skip, default)]
    pub output_buffer: String,
}

impl InstancePane {
    pub fn new(working_directory: PathBuf, rows: u16, columns: u16) -> Self {
        Self {
            id: Uuid::new_v4(),
            working_directory,
            rows,
            columns,
            pty_session: None,
            claude_state: ClaudeState::Idle,
            output_buffer: String::new(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LayoutMode {
    Single,
    HorizontalSplit,
    VerticalSplit,
    Grid,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum InstanceMode {
    Normal,
    Focused,
}
