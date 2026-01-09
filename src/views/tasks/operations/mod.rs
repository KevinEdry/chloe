mod classification;
mod crud;
mod movement;
mod navigation;
mod queries;
mod worktree;

use super::state::Task;

pub use queries::{get_active_task_count, get_active_tasks, get_done_task_count, get_done_tasks};

pub struct TaskReference<'a> {
    pub task: &'a Task,
    pub column_name: &'a str,
    pub column_index: usize,
}
