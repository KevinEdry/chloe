use crate::types::AgentProvider;
use alacritty_terminal::grid::Dimensions;
use ratatui::layout::Rect;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SplitDirection {
    Horizontal,
    Vertical,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PaneNode {
    Leaf(Box<InstancePane>),
    Split {
        direction: SplitDirection,
        ratio: f32,
        first: Box<Self>,
        second: Box<Self>,
    },
}

impl PaneNode {
    #[must_use]
    pub fn collect_panes(&self) -> Vec<&InstancePane> {
        match self {
            Self::Leaf(pane) => vec![pane],
            Self::Split { first, second, .. } => {
                let mut panes = first.collect_panes();
                panes.extend(second.collect_panes());
                panes
            }
        }
    }

    #[must_use]
    pub fn find_pane(&self, id: Uuid) -> Option<&InstancePane> {
        match self {
            Self::Leaf(pane) if pane.id == id => Some(pane),
            Self::Leaf(_) => None,
            Self::Split { first, second, .. } => {
                first.find_pane(id).or_else(|| second.find_pane(id))
            }
        }
    }

    pub fn find_pane_mut(&mut self, id: Uuid) -> Option<&mut InstancePane> {
        match self {
            Self::Leaf(pane) if pane.id == id => Some(pane),
            Self::Leaf(_) => None,
            Self::Split { first, second, .. } => {
                first.find_pane_mut(id).or_else(|| second.find_pane_mut(id))
            }
        }
    }

    #[must_use]
    pub fn first_pane_id(&self) -> Uuid {
        match self {
            Self::Leaf(pane) => pane.id,
            Self::Split { first, .. } => first.first_pane_id(),
        }
    }

    #[must_use]
    pub fn pane_count(&self) -> usize {
        match self {
            Self::Leaf(_) => 1,
            Self::Split { first, second, .. } => first.pane_count() + second.pane_count(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstanceState {
    pub root: Option<PaneNode>,
    pub selected_pane_id: Option<Uuid>,
    pub mode: InstanceMode,
    #[serde(skip)]
    pub last_render_area: Option<Rect>,
    #[serde(skip, default)]
    pub pane_areas: Vec<(Uuid, Rect)>,
}

impl InstanceState {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            root: None,
            selected_pane_id: None,
            mode: InstanceMode::Normal,
            last_render_area: None,
            pane_areas: Vec::new(),
        }
    }

    pub fn selected_pane_mut(&mut self) -> Option<&mut InstancePane> {
        let id = self.selected_pane_id?;
        self.root.as_mut()?.find_pane_mut(id)
    }

    #[must_use]
    pub fn pane_count(&self) -> usize {
        self.root.as_ref().map_or(0, PaneNode::pane_count)
    }

    #[must_use]
    pub fn collect_panes(&self) -> Vec<&InstancePane> {
        self.root
            .as_ref()
            .map_or_else(Vec::new, PaneNode::collect_panes)
    }

    #[must_use]
    pub fn find_pane(&self, id: Uuid) -> Option<&InstancePane> {
        self.root.as_ref()?.find_pane(id)
    }

    pub fn find_pane_mut(&mut self, id: Uuid) -> Option<&mut InstancePane> {
        self.root.as_mut()?.find_pane_mut(id)
    }

    #[must_use]
    pub fn get_pane_area(&self, id: Uuid) -> Option<Rect> {
        self.pane_areas
            .iter()
            .find(|(pane_id, _)| *pane_id == id)
            .map(|(_, area)| *area)
    }
}

impl Default for InstanceState {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum ClaudeState {
    #[default]
    Idle,
    Running,
    NeedsPermissions,
    Done,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstancePane {
    pub id: Uuid,
    pub name: Option<String>,
    pub working_directory: PathBuf,
    #[serde(default)]
    pub provider: AgentProvider,
    pub rows: u16,
    pub columns: u16,
    #[serde(skip)]
    pub pty_session: Option<super::pty::PtySession>,
    #[serde(skip, default)]
    pub pty_spawn_error: Option<String>,
    #[serde(default)]
    pub claude_state: ClaudeState,
    #[serde(skip, default)]
    pub scroll_offset: usize,
}

impl InstancePane {
    #[must_use]
    pub fn new(working_directory: PathBuf, rows: u16, columns: u16) -> Self {
        Self::with_provider(working_directory, rows, columns, AgentProvider::default())
    }

    #[must_use]
    pub fn with_provider(
        working_directory: PathBuf,
        rows: u16,
        columns: u16,
        provider: AgentProvider,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            name: None,
            working_directory,
            provider,
            rows,
            columns,
            pty_session: None,
            pty_spawn_error: None,
            claude_state: ClaudeState::Idle,
            scroll_offset: 0,
        }
    }

    pub fn scroll_up(&mut self, lines: usize, max_scrollback: usize) {
        self.scroll_offset = (self.scroll_offset + lines).min(max_scrollback);
    }

    pub const fn scroll_down(&mut self, lines: usize) {
        self.scroll_offset = self.scroll_offset.saturating_sub(lines);
    }

    pub const fn scroll_to_bottom(&mut self) {
        self.scroll_offset = 0;
    }

    #[must_use]
    pub fn scrollback_len(&self) -> usize {
        let Some(session) = &self.pty_session else {
            return 0;
        };
        let term_mutex = session.term();
        let Ok(term) = term_mutex.lock() else {
            return 0;
        };
        term.grid().history_size()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum InstanceMode {
    Normal,
    Focused,
    Scroll,
}
