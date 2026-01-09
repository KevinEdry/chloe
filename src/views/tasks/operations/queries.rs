use super::TaskReference;
use crate::views::tasks::state::{Column, TasksState};

impl TasksState {
    pub fn find_task_index_by_id(&self, task_id: uuid::Uuid) -> Option<usize> {
        let review_column_index = 2;
        self.columns
            .get(review_column_index)?
            .tasks
            .iter()
            .position(|task| task.id == task_id)
    }
}

pub fn get_active_task_count(columns: &[Column]) -> usize {
    columns
        .iter()
        .take(3)
        .map(|column| column.tasks.len())
        .sum()
}

pub fn get_done_task_count(columns: &[Column]) -> usize {
    columns.get(3).map(|column| column.tasks.len()).unwrap_or(0)
}

pub fn get_active_tasks(columns: &[Column]) -> Vec<TaskReference<'_>> {
    let mut tasks = Vec::new();
    let active_column_indices = [0, 1, 2];

    for &column_index in &active_column_indices {
        if let Some(column) = columns.get(column_index) {
            for task in &column.tasks {
                tasks.push(TaskReference {
                    task,
                    column_name: &column.name,
                    column_index,
                });
            }
        }
    }

    tasks
}

pub fn get_done_tasks(columns: &[Column]) -> Vec<TaskReference<'_>> {
    let mut tasks = Vec::new();
    let done_column_index = 3;

    if let Some(column) = columns.get(done_column_index) {
        for task in &column.tasks {
            tasks.push(TaskReference {
                task,
                column_name: &column.name,
                column_index: done_column_index,
            });
        }
    }

    tasks
}
