use crate::types::AgentProvider;
use crate::views::tasks::ai_classifier::{ClassificationRequest, ClassifiedTask};
use crate::views::tasks::state::{Task, TaskType, TasksState};
use uuid::Uuid;

const PLANNING_COLUMN_INDEX: usize = 0;

impl TasksState {
    pub fn start_classification(&mut self, raw_input: String, provider: AgentProvider) -> Uuid {
        let task = Task::new_classifying(raw_input.clone());
        let task_id = task.id;

        self.columns[PLANNING_COLUMN_INDEX].tasks.push(task);
        self.kanban_selected_column = PLANNING_COLUMN_INDEX;
        self.kanban_selected_task = Some(self.columns[PLANNING_COLUMN_INDEX].tasks.len() - 1);

        let request = ClassificationRequest::spawn(raw_input, task_id, provider);
        self.pending_classifications.push(request);

        task_id
    }

    pub fn poll_classification(&mut self) {
        let mut completed_indices = Vec::new();

        for (index, request) in self.pending_classifications.iter().enumerate() {
            if let Some(result) = request.try_recv() {
                completed_indices.push((index, request.task_id, result));
            }
        }

        for (index, task_id, result) in completed_indices.into_iter().rev() {
            self.pending_classifications.remove(index);

            match result {
                Ok(classified) => {
                    self.apply_classification_to_task(task_id, classified);
                }
                Err(_) => {
                    self.mark_task_classification_failed(task_id);
                }
            }
        }
    }

    fn apply_classification_to_task(&mut self, task_id: Uuid, classified: ClassifiedTask) {
        let task_type = match classified.task_type.to_lowercase().as_str() {
            "feature" => TaskType::Feature,
            "bug" => TaskType::Bug,
            "chore" => TaskType::Chore,
            _ => TaskType::Task,
        };

        for column in &mut self.columns {
            for task in &mut column.tasks {
                if task.id == task_id {
                    task.title = classified.title;
                    task.description = classified.description;
                    task.kind = task_type;
                    task.is_classifying = false;
                    return;
                }
            }
        }
    }

    fn mark_task_classification_failed(&mut self, task_id: Uuid) {
        for column in &mut self.columns {
            for task in &mut column.tasks {
                if task.id == task_id {
                    task.is_classifying = false;
                    return;
                }
            }
        }
    }
}
