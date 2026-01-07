use chrono::{DateTime, Utc};
use ratatui::style::Color;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KanbanState {
    pub columns: Vec<Column>,
    pub selected_column: usize,
    pub selected_task: Option<usize>,
    pub mode: KanbanMode,
    #[serde(skip)]
    pub classification_request: Option<super::ai_classifier::ClassificationRequest>,
    #[serde(skip)]
    pub pending_instance_termination: Option<Uuid>,
}

impl KanbanState {
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
            selected_column: 0,
            selected_task: None,
            mode: KanbanMode::Normal,
            classification_request: None,
            pending_instance_termination: None,
        }
    }

    pub fn selected_column_mut(&mut self) -> &mut Column {
        &mut self.columns[self.selected_column]
    }

    pub fn get_selected_task(&self) -> Option<&Task> {
        self.selected_task
            .and_then(|idx| self.columns[self.selected_column].tasks.get(idx))
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
}

impl Default for KanbanState {
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
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum KanbanMode {
    Normal,
    AddingTask {
        input: String,
    },
    EditingTask {
        task_idx: usize,
        input: String,
    },
    ConfirmDelete {
        task_idx: usize,
    },
    ConfirmMoveBack {
        task_idx: usize,
    },
    ClassifyingTask {
        raw_input: String,
    },
    ReviewPopup {
        task_idx: usize,
        scroll_offset: usize,
    },
}
