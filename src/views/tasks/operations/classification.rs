use crate::events::AppEvent;
use crate::types::AgentProvider;
use crate::views::tasks::ai_classifier::{ClassifiedTask, spawn_classification};
use crate::views::tasks::state::{Task, TaskType, TasksState};
use tokio::sync::mpsc;
use uuid::Uuid;

const PLANNING_COLUMN_INDEX: usize = 0;

impl TasksState {
    pub fn start_classification(
        &mut self,
        raw_input: String,
        provider: AgentProvider,
        event_sender: mpsc::UnboundedSender<AppEvent>,
    ) -> Uuid {
        let task = Task::new_classifying(raw_input.clone());
        let task_id = task.id;

        self.columns[PLANNING_COLUMN_INDEX].tasks.push(task);
        self.kanban_selected_column = PLANNING_COLUMN_INDEX;
        self.kanban_selected_task = Some(self.columns[PLANNING_COLUMN_INDEX].tasks.len() - 1);

        self.pending_classifications.insert(task_id);
        spawn_classification(raw_input, task_id, provider, event_sender);

        task_id
    }

    pub fn handle_classification_completed(
        &mut self,
        task_id: Uuid,
        result: Result<ClassifiedTask, String>,
    ) {
        self.pending_classifications.remove(&task_id);

        match result {
            Ok(classified) => {
                self.apply_classification_to_task(task_id, classified);
            }
            Err(_) => {
                self.mark_task_classification_failed(task_id);
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
