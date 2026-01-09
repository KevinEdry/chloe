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
    pub classification_request: Option<super::ai_classifier::ClassificationRequest>,
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
}

impl TasksState {
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
            classification_request: None,
            pending_instance_termination: None,
            pending_worktree_deletion: None,
            pending_instance_creation: None,
            pending_ide_open: None,
            pending_terminal_switch: None,
            pending_change_request: None,
        }
    }

    pub fn toggle_view_mode(&mut self) {
        self.view_mode = match self.view_mode {
            TasksViewMode::Focus => TasksViewMode::Kanban,
            TasksViewMode::Kanban => TasksViewMode::Focus,
        };
    }

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

    pub fn is_normal_mode(&self) -> bool {
        matches!(self.mode, TasksMode::Normal)
    }

    pub fn is_terminal_focused(&self) -> bool {
        matches!(self.mode, TasksMode::TerminalFocused)
    }

    pub fn is_typing_mode(&self) -> bool {
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

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum TaskType {
    Feature,
    Bug,
    Chore,
    Task,
}

impl TaskType {
    pub const fn badge_text(self) -> &'static str {
        match self {
            Self::Feature => "FEAT",
            Self::Bug => "BUG",
            Self::Chore => "CHORE",
            Self::Task => "TASK",
        }
    }

    pub const fn color(self) -> Color {
        match self {
            Self::Feature => Color::Green,
            Self::Bug => Color::Red,
            Self::Chore => Color::Yellow,
            Self::Task => Color::Cyan,
        }
    }
}

impl Default for TaskType {
    fn default() -> Self {
        Self::Task
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub created_at: DateTime<Utc>,
    #[serde(default)]
    pub task_type: TaskType,
    #[serde(default)]
    pub instance_id: Option<Uuid>,
    #[serde(default)]
    pub is_paused: bool,
    #[serde(default)]
    pub worktree_info: Option<WorktreeInfo>,
}

impl Task {
    pub fn new(title: String, description: String, task_type: TaskType) -> Self {
        Self {
            id: Uuid::new_v4(),
            title,
            description,
            created_at: Utc::now(),
            task_type,
            instance_id: None,
            is_paused: false,
            worktree_info: None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ReviewAction {
    ReviewInIDE,
    ReviewInTerminal,
    RequestChanges,
    MergeToBranch,
}

impl ReviewAction {
    pub const fn all() -> [Self; 4] {
        [
            Self::ReviewInIDE,
            Self::ReviewInTerminal,
            Self::RequestChanges,
            Self::MergeToBranch,
        ]
    }

    pub fn label(&self, branch_name: Option<&str>, has_conflicts: bool) -> String {
        match self {
            Self::ReviewInIDE => "Review in IDE".to_string(),
            Self::ReviewInTerminal => "Review in Terminal".to_string(),
            Self::RequestChanges => "Request Changes".to_string(),
            Self::MergeToBranch => {
                if has_conflicts {
                    "Resolve Conflicts".to_string()
                } else {
                    match branch_name {
                        Some(name) => format!("Merge to {}", name),
                        None => "Mark Complete".to_string(),
                    }
                }
            }
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
    ClassifyingTask {
        raw_input: String,
        edit_task_id: Option<Uuid>,
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
}
