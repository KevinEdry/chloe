use crate::views::tasks::ai_classifier::{ClassificationRequest, ClassifiedTask};
use crate::views::tasks::state::{TaskType, TasksMode, TasksState};
use uuid::Uuid;

impl TasksState {
    pub fn start_classification(&mut self, raw_input: String) {
        self.start_classification_internal(raw_input, None);
    }

    fn start_classification_internal(&mut self, raw_input: String, edit_task_id: Option<Uuid>) {
        let request = ClassificationRequest::spawn(raw_input.clone());
        self.classification_request = Some(request);
        self.mode = TasksMode::ClassifyingTask {
            raw_input,
            edit_task_id,
        };
    }

    pub fn poll_classification(&mut self) {
        if let Some(request) = &self.classification_request {
            if let Some(result) = request.try_recv() {
                self.classification_request = None;

                match result {
                    Ok(classified) => {
                        self.apply_classification(classified);
                    }
                    Err(_) => {
                        if let TasksMode::ClassifyingTask { raw_input, .. } = &self.mode {
                            self.fallback_to_manual(raw_input.clone());
                        }
                    }
                }
            }
        }
    }

    pub fn cancel_classification(&mut self) {
        self.classification_request = None;
        self.mode = TasksMode::Normal;
    }

    pub fn apply_classification(&mut self, classified: ClassifiedTask) {
        let task_type = match classified.task_type.to_lowercase().as_str() {
            "feature" => TaskType::Feature,
            "bug" => TaskType::Bug,
            "chore" => TaskType::Chore,
            _ => TaskType::Task,
        };

        if let TasksMode::ClassifyingTask { edit_task_id, .. } = self.mode {
            if let Some(task_id) = edit_task_id {
                self.edit_task_by_id(
                    task_id,
                    classified.title,
                    classified.description,
                    Some(task_type),
                );
            } else {
                self.add_task_to_planning(classified.title, classified.description, task_type);
            }
        }

        self.mode = TasksMode::Normal;
    }

    pub fn fallback_to_manual(&mut self, raw_input: String) {
        if let TasksMode::ClassifyingTask { edit_task_id, .. } = self.mode {
            if let Some(task_id) = edit_task_id {
                self.edit_task_by_id(task_id, raw_input, String::new(), None);
            } else {
                self.add_task_to_planning(raw_input, String::new(), TaskType::Task);
            }
        }
        self.mode = TasksMode::Normal;
    }
}
