pub mod ai_classifier;
pub mod dialogs;
pub mod events;
pub mod operations;
pub mod state;
pub mod views;

pub use events::{TasksAction, handle_key_event};
pub use operations::{get_active_tasks, get_done_tasks};
pub use state::{FocusPanel, Task, TaskType, TasksState, TasksViewMode};
