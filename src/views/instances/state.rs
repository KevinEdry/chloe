use crate::types::AgentProvider;
use alacritty_terminal::grid::Dimensions;
use chrono::{DateTime, Utc};
use ratatui::layout::Rect;
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use std::path::PathBuf;
use uuid::Uuid;

const MAX_ACTIVITY_EVENTS: usize = 100;

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
    pub last_viewed_at: Option<DateTime<Utc>>,
    #[serde(default)]
    pub activity_events: VecDeque<ActivityEvent>,
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
            last_viewed_at: Some(Utc::now()),
            activity_events: VecDeque::new(),
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

    pub fn mark_viewed(&mut self) {
        self.last_viewed_at = Some(Utc::now());
    }

    pub fn add_activity_event(
        &mut self,
        event_type: ActivityEventType,
        description: String,
        metadata: Option<String>,
    ) {
        let event = ActivityEvent {
            timestamp: Utc::now(),
            event_type,
            description,
            metadata,
        };

        self.activity_events.push_back(event);

        while self.activity_events.len() > MAX_ACTIVITY_EVENTS {
            self.activity_events.pop_front();
        }
    }

    #[must_use]
    #[allow(dead_code)]
    pub fn get_events_since(&self, since: DateTime<Utc>) -> Vec<&ActivityEvent> {
        self.activity_events
            .iter()
            .filter(|event| event.timestamp > since)
            .collect()
    }

    #[must_use]
    #[allow(dead_code)]
    pub fn generate_activity_summary(&self) -> Option<ActivitySummary> {
        let since = self.last_viewed_at?;
        let events = self.get_events_since(since);

        if events.is_empty() {
            return None;
        }

        let mut commands_executed = Vec::new();
        let mut files_changed = Vec::new();
        let mut errors = Vec::new();
        let mut notifications = Vec::new();
        let mut tasks_completed = 0;

        for event in &events {
            match event.event_type {
                ActivityEventType::CommandExecuted => {
                    commands_executed.push(event.description.clone());
                }
                ActivityEventType::FileChanged => {
                    files_changed.push(event.description.clone());
                }
                ActivityEventType::ErrorOccurred => {
                    errors.push(event.description.clone());
                }
                ActivityEventType::ProviderNotification => {
                    notifications.push(event.description.clone());
                }
                ActivityEventType::TaskCompleted => {
                    tasks_completed += 1;
                }
            }
        }

        let elapsed = Utc::now().signed_duration_since(since);

        Some(ActivitySummary {
            since,
            elapsed_seconds: elapsed.num_seconds(),
            commands_executed,
            files_changed,
            errors,
            notifications,
            tasks_completed,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum InstanceMode {
    Normal,
    Focused,
    Scroll,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ActivityEventType {
    CommandExecuted,
    FileChanged,
    TaskCompleted,
    ErrorOccurred,
    ProviderNotification,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActivityEvent {
    pub timestamp: DateTime<Utc>,
    pub event_type: ActivityEventType,
    pub description: String,
    pub metadata: Option<String>,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct ActivitySummary {
    pub since: DateTime<Utc>,
    pub elapsed_seconds: i64,
    pub commands_executed: Vec<String>,
    pub files_changed: Vec<String>,
    pub errors: Vec<String>,
    pub notifications: Vec<String>,
    pub tasks_completed: usize,
}

impl ActivitySummary {
    #[must_use]
    #[allow(dead_code)]
    pub fn format_as_text(&self) -> String {
        use std::fmt::Write;
        let mut output = String::new();

        output.push_str("Activity Summary\n");
        let _ = write!(
            output,
            "Since: {} ({} seconds ago)\n\n",
            self.since.format("%H:%M:%S"),
            self.elapsed_seconds
        );

        if !self.commands_executed.is_empty() {
            let _ = writeln!(
                output,
                "Commands executed ({}):",
                self.commands_executed.len()
            );
            for command in &self.commands_executed {
                let _ = writeln!(output, "  • {command}");
            }
            output.push('\n');
        }

        if !self.files_changed.is_empty() {
            let _ = writeln!(output, "Files changed ({}):", self.files_changed.len());
            for file in &self.files_changed {
                let _ = writeln!(output, "  • {file}");
            }
            output.push('\n');
        }

        if self.tasks_completed > 0 {
            let _ = write!(output, "Tasks completed: {}\n\n", self.tasks_completed);
        }

        if !self.errors.is_empty() {
            let _ = writeln!(output, "Errors ({}):", self.errors.len());
            for error in &self.errors {
                let _ = writeln!(output, "  • {error}");
            }
            output.push('\n');
        }

        if !self.notifications.is_empty() {
            let _ = writeln!(output, "Notifications ({}):", self.notifications.len());
            for notification in &self.notifications {
                let _ = writeln!(output, "  • {notification}");
            }
        }

        output
    }

    #[must_use]
    #[allow(dead_code)]
    pub fn format_as_summary_line(&self) -> String {
        let mut parts = Vec::new();

        if !self.commands_executed.is_empty() {
            parts.push(format!("{} commands", self.commands_executed.len()));
        }

        if !self.files_changed.is_empty() {
            parts.push(format!("{} files changed", self.files_changed.len()));
        }

        if self.tasks_completed > 0 {
            parts.push(format!("{} tasks done", self.tasks_completed));
        }

        if !self.errors.is_empty() {
            parts.push(format!("{} errors", self.errors.len()));
        }

        if parts.is_empty() {
            "No significant activity".to_string()
        } else {
            parts.join(", ")
        }
    }
}
