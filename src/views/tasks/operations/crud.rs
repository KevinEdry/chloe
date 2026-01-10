use crate::views::tasks::state::{Task, TaskType, TasksState};
use uuid::Uuid;

impl TasksState {
    pub fn add_task_to_planning(
        &mut self,
        title: String,
        description: String,
        task_type: TaskType,
    ) {
        let task = Task::new(title, description, task_type);
        self.columns[0].tasks.push(task);
        self.kanban_selected_column = 0;
        self.kanban_selected_task = Some(self.columns[0].tasks.len() - 1);
    }

    pub fn delete_task_by_id(&mut self, task_id: Uuid) -> Option<Uuid> {
        for column in &mut self.columns {
            if let Some(position) = column.tasks.iter().position(|task| task.id == task_id) {
                let task = column.tasks.remove(position);
                return task.instance_id;
            }
        }
        None
    }

    pub fn update_task_title_by_id(&mut self, task_id: Uuid, new_title: String) -> bool {
        for column in &mut self.columns {
            for task in &mut column.tasks {
                if task.id == task_id {
                    task.title = new_title;
                    return true;
                }
            }
        }
        false
    }
}
