use super::generator::{GeneratedRoadmap, RoadmapGenerationRequest};
use super::state::{RoadmapItem, RoadmapPriority, RoadmapState, RoadmapStatus};
use chrono::Utc;
use uuid::Uuid;

impl RoadmapState {
    pub fn add_item(
        &mut self,
        title: String,
        description: String,
        rationale: String,
        priority: RoadmapPriority,
    ) -> Uuid {
        let item = RoadmapItem::new(title, description, rationale, priority);
        let item_id = item.id;
        self.items.push(item);
        self.selected_item = Some(self.items.len() - 1);
        item_id
    }

    pub fn delete_item(&mut self, index: usize) -> Option<RoadmapItem> {
        if index < self.items.len() {
            let removed = self.items.remove(index);
            self.selected_item = if self.items.is_empty() {
                None
            } else if index >= self.items.len() {
                Some(self.items.len() - 1)
            } else {
                Some(index)
            };
            Some(removed)
        } else {
            None
        }
    }

    pub fn update_item_title(&mut self, index: usize, title: String) {
        if let Some(item) = self.items.get_mut(index) {
            item.title = title;
            item.updated_at = Utc::now();
        }
    }

    pub fn update_item_description(&mut self, index: usize, description: String) {
        if let Some(item) = self.items.get_mut(index) {
            item.description = description;
            item.updated_at = Utc::now();
        }
    }

    pub fn update_item_status(&mut self, index: usize, status: RoadmapStatus) {
        if let Some(item) = self.items.get_mut(index) {
            item.status = status;
            item.updated_at = Utc::now();
        }
    }

    pub fn update_item_priority(&mut self, index: usize, priority: RoadmapPriority) {
        if let Some(item) = self.items.get_mut(index) {
            item.priority = priority;
            item.updated_at = Utc::now();
        }
    }

    pub fn add_user_story(&mut self, index: usize, story: String) {
        if let Some(item) = self.items.get_mut(index) {
            item.user_stories.push(story);
            item.updated_at = Utc::now();
        }
    }

    pub fn add_acceptance_criterion(&mut self, index: usize, criterion: String) {
        if let Some(item) = self.items.get_mut(index) {
            item.acceptance_criteria.push(criterion);
            item.updated_at = Utc::now();
        }
    }

    pub fn add_dependency(&mut self, index: usize, dependency_id: Uuid) {
        if let Some(item) = self.items.get_mut(index) {
            if !item.dependencies.contains(&dependency_id) {
                item.dependencies.push(dependency_id);
                item.updated_at = Utc::now();
            }
        }
    }

    pub fn remove_dependency(&mut self, index: usize, dependency_id: Uuid) {
        if let Some(item) = self.items.get_mut(index) {
            item.dependencies.retain(|id| *id != dependency_id);
            item.updated_at = Utc::now();
        }
    }

    pub fn select_next(&mut self) {
        if self.items.is_empty() {
            self.selected_item = None;
            return;
        }

        self.selected_item = Some(match self.selected_item {
            Some(current) if current < self.items.len() - 1 => current + 1,
            Some(_) => self.items.len() - 1,
            None => 0,
        });
    }

    pub fn select_previous(&mut self) {
        if self.items.is_empty() {
            self.selected_item = None;
            return;
        }

        self.selected_item = Some(match self.selected_item {
            Some(current) if current > 0 => current - 1,
            Some(_) => 0,
            None => 0,
        });
    }

    pub fn find_item_by_id(&self, id: Uuid) -> Option<&RoadmapItem> {
        self.items.iter().find(|item| item.id == id)
    }

    pub fn start_generation(&mut self, project_path: String) {
        self.generation_request = Some(RoadmapGenerationRequest::spawn(project_path));
        self.mode = super::state::RoadmapMode::Generating;
    }

    pub fn poll_generation(&mut self) {
        if let Some(ref request) = self.generation_request {
            if let Some(result) = request.try_recv() {
                self.generation_request = None;
                self.mode = super::state::RoadmapMode::Normal;

                match result {
                    Ok(generated) => {
                        self.apply_generated_roadmap(generated);
                    }
                    Err(_) => {}
                }
            }
        }
    }

    fn apply_generated_roadmap(&mut self, generated: GeneratedRoadmap) {
        for generated_item in generated.items {
            let item = generated_item.to_roadmap_item();
            self.items.push(item);
        }

        if !self.items.is_empty() {
            self.selected_item = Some(0);
        }
    }
}
