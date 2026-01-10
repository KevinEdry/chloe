use chrono::{DateTime, Utc};
use ratatui::style::Color;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::views::worktree::WorktreeInfo;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum TasksViewMode {
    #[default]
    Focus,
    Kanban,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum FocusPanel {
    #[default]
    ActiveTasks,
    DoneTasks,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TasksState {
    pub columns: Vec<Column>,
    pub mode: TasksMode,
    pub view_mode: TasksViewMode,

    pub kanban_selected_column: usize,
    pub kanban_selected_task: Option<usize>,

    pub focus_active_index: usize,
    pub focus_done_index: usize,
    pub focus_panel: FocusPanel,
    pub focus_details_scroll: u16,

    #[serde(skip)]
    pub pending_classifications: Vec<super::ai_classifier::ClassificationRequest>,
    #[serde(skip)]
    pub pending_instance_termination: Option<Uuid>,
    #[serde(skip)]
    pub pending_worktree_deletion: Option<WorktreeInfo>,
    #[serde(skip)]
    pub pending_instance_creation: Option<Uuid>,
    #[serde(skip)]
    pub pending_ide_open: Option<Uuid>,
    #[serde(skip)]
    pub pending_terminal_switch: Option<Uuid>,
    #[serde(skip)]
    pub pending_change_request: Option<(Uuid, String)>,
    #[serde(skip)]
    pub error_message: Option<String>,
}

impl TasksState {
    #[must_use]
    pub fn new() -> Self {
        Self {
            columns: vec![
                Column {
                    name: "Planning".to_string(),
                    tasks: Vec::new(),
                },
                Column {
                    name: "In Progress".to_string(),
                    tasks: Vec::new(),
                },
                Column {
                    name: "Review".to_string(),
                    tasks: Vec::new(),
                },
                Column {
                    name: "Done".to_string(),
                    tasks: Vec::new(),
                },
            ],
            mode: TasksMode::Normal,
            view_mode: TasksViewMode::default(),
            kanban_selected_column: 0,
            kanban_selected_task: None,
            focus_active_index: 0,
            focus_done_index: 0,
            focus_panel: FocusPanel::default(),
            focus_details_scroll: 0,
            pending_classifications: Vec::new(),
            pending_instance_termination: None,
            pending_worktree_deletion: None,
            pending_instance_creation: None,
            pending_ide_open: None,
            pending_terminal_switch: None,
            pending_change_request: None,
            error_message: None,
        }
    }

    pub const fn toggle_view_mode(&mut self) {
        self.view_mode = match self.view_mode {
            TasksViewMode::Focus => TasksViewMode::Kanban,
            TasksViewMode::Kanban => TasksViewMode::Focus,
        };
    }

    #[must_use]
    pub fn get_kanban_selected_task(&self) -> Option<&Task> {
        self.kanban_selected_task
            .and_then(|index| self.columns[self.kanban_selected_column].tasks.get(index))
    }

    pub fn link_task_to_instance(&mut self, task_id: Uuid, instance_id: Uuid) {
        for column in &mut self.columns {
            for task in &mut column.tasks {
                if task.id == task_id {
                    task.instance_id = Some(instance_id);
                    return;
                }
            }
        }
    }

    #[must_use]
    pub fn find_task_by_id(&self, task_id: Uuid) -> Option<&Task> {
        for column in &self.columns {
            for task in &column.tasks {
                if task.id == task_id {
                    return Some(task);
                }
            }
        }
        None
    }

    #[must_use]
    pub const fn is_normal_mode(&self) -> bool {
        matches!(self.mode, TasksMode::Normal)
    }

    #[must_use]
    pub const fn is_terminal_focused(&self) -> bool {
        matches!(self.mode, TasksMode::TerminalFocused)
    }

    #[must_use]
    pub const fn is_typing_mode(&self) -> bool {
        matches!(
            self.mode,
            TasksMode::AddingTask { .. }
                | TasksMode::EditingTask { .. }
                | TasksMode::ReviewRequestChanges { .. }
        )
    }
}

impl Default for TasksState {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Column {
    pub name: String,
    pub tasks: Vec<Task>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum TaskType {
    Feature,
    Bug,
    Chore,
    #[default]
    Task,
}

impl TaskType {
    #[must_use]
    pub const fn badge_text(self) -> &'static str {
        match self {
            Self::Feature => "FEAT",
            Self::Bug => "BUG",
            Self::Chore => "CHORE",
            Self::Task => "TASK",
        }
    }

    #[must_use]
    pub const fn color(self) -> Color {
        match self {
            Self::Feature => Color::Green,
            Self::Bug => Color::Red,
            Self::Chore => Color::Yellow,
            Self::Task => Color::Cyan,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub created_at: DateTime<Utc>,
    #[serde(default)]
    pub kind: TaskType,
    #[serde(default)]
    pub instance_id: Option<Uuid>,
    #[serde(default)]
    pub is_paused: bool,
    #[serde(default)]
    pub worktree_info: Option<WorktreeInfo>,
    #[serde(skip)]
    pub is_classifying: bool,
}

impl Task {
    #[must_use]
    pub fn new(title: String, description: String, kind: TaskType) -> Self {
        Self {
            id: Uuid::new_v4(),
            title,
            description,
            created_at: Utc::now(),
            kind,
            instance_id: None,
            is_paused: false,
            worktree_info: None,
            is_classifying: false,
        }
    }

    #[must_use]
    pub fn new_classifying(raw_input: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            title: raw_input,
            description: String::new(),
            created_at: Utc::now(),
            kind: TaskType::Task,
            instance_id: None,
            is_paused: false,
            worktree_info: None,
            is_classifying: true,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ReviewAction {
    ReviewInIDE,
    ReviewInTerminal,
    RequestChanges,
    CommitChanges,
    MergeAndComplete,
}

impl ReviewAction {
    #[must_use]
    pub const fn all() -> [Self; 5] {
        [
            Self::ReviewInIDE,
            Self::ReviewInTerminal,
            Self::RequestChanges,
            Self::CommitChanges,
            Self::MergeAndComplete,
        ]
    }

    #[must_use]
    pub fn label(self) -> String {
        match self {
            Self::ReviewInIDE => "Review in IDE".to_string(),
            Self::ReviewInTerminal => "Review in Terminal".to_string(),
            Self::RequestChanges => "Request Changes".to_string(),
            Self::CommitChanges => "Commit".to_string(),
            Self::MergeAndComplete => "Merge & Complete".to_string(),
        }
    }

    #[must_use]
    pub const fn is_enabled(self, is_clean: bool) -> bool {
        match self {
            Self::ReviewInIDE | Self::ReviewInTerminal | Self::RequestChanges => true,
            Self::CommitChanges => !is_clean,
            Self::MergeAndComplete => is_clean,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum MergeTarget {
    CurrentBranch(String),
    MainBranch,
}

impl MergeTarget {
    #[must_use]
    pub fn branch_name(&self) -> &str {
        match self {
            Self::CurrentBranch(name) => name,
            Self::MainBranch => "main",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TasksMode {
    Normal,
    TerminalFocused,
    AddingTask {
        input: String,
    },
    EditingTask {
        task_id: Uuid,
        input: String,
    },
    ConfirmDelete {
        task_id: Uuid,
    },
    ConfirmStartTask {
        task_id: Uuid,
    },
    ConfirmMoveBack {
        task_id: Uuid,
    },
    ReviewPopup {
        task_id: Uuid,
        scroll_offset: usize,
        selected_action: ReviewAction,
    },
    ReviewRequestChanges {
        task_id: Uuid,
        input: String,
    },
    MergeConfirmation {
        task_id: Uuid,
        worktree_branch: String,
        selected_target: MergeTarget,
    },
}
