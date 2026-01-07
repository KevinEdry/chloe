use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KanbanState {
    pub columns: Vec<Column>,
    pub selected_column: usize,
    pub selected_task: Option<usize>,
    pub mode: KanbanMode,
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
        }
    }

    pub fn selected_column(&self) -> &Column {
        &self.columns[self.selected_column]
    }

    pub fn selected_column_mut(&mut self) -> &mut Column {
        &mut self.columns[self.selected_column]
    }

    pub fn get_selected_task(&self) -> Option<&Task> {
        self.selected_task.and_then(|idx| {
            self.columns[self.selected_column].tasks.get(idx)
        })
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub created_at: DateTime<Utc>,
}

impl Task {
    pub fn new(title: String, description: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            title,
            description,
            created_at: Utc::now(),
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
}
