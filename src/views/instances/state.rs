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
    Leaf(InstancePane),
    Split {
        direction: SplitDirection,
        ratio: f32,
        first: Box<PaneNode>,
        second: Box<PaneNode>,
    },
}

impl PaneNode {
    pub fn collect_panes(&self) -> Vec<&InstancePane> {
        match self {
            PaneNode::Leaf(pane) => vec![pane],
            PaneNode::Split { first, second, .. } => {
                let mut panes = first.collect_panes();
                panes.extend(second.collect_panes());
                panes
            }
        }
    }

    pub fn find_pane(&self, id: Uuid) -> Option<&InstancePane> {
        match self {
            PaneNode::Leaf(pane) => {
                if pane.id == id {
                    Some(pane)
                } else {
                    None
                }
            }
            PaneNode::Split { first, second, .. } => {
                first.find_pane(id).or_else(|| second.find_pane(id))
            }
        }
    }

    pub fn find_pane_mut(&mut self, id: Uuid) -> Option<&mut InstancePane> {
        match self {
            PaneNode::Leaf(pane) => {
                if pane.id == id {
                    Some(pane)
                } else {
                    None
                }
            }
            PaneNode::Split { first, second, .. } => {
                first.find_pane_mut(id).or_else(|| second.find_pane_mut(id))
            }
        }
    }

    pub fn first_pane_id(&self) -> Uuid {
        match self {
            PaneNode::Leaf(pane) => pane.id,
            PaneNode::Split { first, .. } => first.first_pane_id(),
        }
    }

    pub fn pane_count(&self) -> usize {
        match self {
            PaneNode::Leaf(_) => 1,
            PaneNode::Split { first, second, .. } => first.pane_count() + second.pane_count(),
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

    pub fn pane_count(&self) -> usize {
        self.root.as_ref().map_or(0, PaneNode::pane_count)
    }

    pub fn collect_panes(&self) -> Vec<&InstancePane> {
        self.root
            .as_ref()
            .map_or_else(Vec::new, PaneNode::collect_panes)
    }

    pub fn find_pane(&self, id: Uuid) -> Option<&InstancePane> {
        self.root.as_ref()?.find_pane(id)
    }

    pub fn find_pane_mut(&mut self, id: Uuid) -> Option<&mut InstancePane> {
        self.root.as_mut()?.find_pane_mut(id)
    }

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
    pub rows: u16,
    pub columns: u16,
    #[serde(skip)]
    pub pty_session: Option<super::pty::PtySession>,
    #[serde(default)]
    pub claude_state: ClaudeState,
    #[serde(skip, default)]
    pub output_buffer: String,
    #[serde(skip, default)]
    pub scroll_offset: usize,
}

impl InstancePane {
    #[must_use]
    pub fn new(working_directory: PathBuf, rows: u16, columns: u16) -> Self {
        Self {
            id: Uuid::new_v4(),
            name: None,
            working_directory,
            rows,
            columns,
            pty_session: None,
            claude_state: ClaudeState::Idle,
            output_buffer: String::new(),
            scroll_offset: 0,
        }
    }

    pub fn scroll_up(&mut self, lines: usize) {
        if let Some(session) = &self.pty_session {
            let scrollback_len = session.scrollback_len();
            self.scroll_offset = (self.scroll_offset + lines).min(scrollback_len);
        }
    }

    pub const fn scroll_down(&mut self, lines: usize) {
        self.scroll_offset = self.scroll_offset.saturating_sub(lines);
    }

    pub const fn scroll_to_bottom(&mut self) {
        self.scroll_offset = 0;
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum InstanceMode {
    Normal,
    Focused,
    Scroll,
}
