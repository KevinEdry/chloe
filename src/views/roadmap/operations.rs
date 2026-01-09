use super::generator::{GeneratedRoadmap, RoadmapGenerationRequest};
use super::state::{RoadmapItem, RoadmapPriority, RoadmapState};
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
        self.sort_by_priority();
        self.select_item_by_id(item_id);
        item_id
    }

    fn sort_by_priority(&mut self) {
        self.items.sort_by(|a, b| {
            use std::cmp::Ordering;
            match (a.priority, b.priority) {
                (RoadmapPriority::High, RoadmapPriority::High)
                | (RoadmapPriority::Medium, RoadmapPriority::Medium)
                | (RoadmapPriority::Low, RoadmapPriority::Low) => Ordering::Equal,
                (RoadmapPriority::High, _) | (RoadmapPriority::Medium, RoadmapPriority::Low) => {
                    Ordering::Less
                }
                (_, RoadmapPriority::High) | (RoadmapPriority::Low, RoadmapPriority::Medium) => {
                    Ordering::Greater
                }
            }
        });
    }

    fn select_item_by_id(&mut self, id: Uuid) {
        self.selected_item = self.items.iter().position(|item| item.id == id);
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

    pub fn update_item_priority(&mut self, index: usize, priority: RoadmapPriority) {
        if let Some(item) = self.items.get_mut(index) {
            let item_id = item.id;
            item.priority = priority;
            item.updated_at = Utc::now();
            self.sort_by_priority();
            self.select_item_by_id(item_id);
        }
    }

    pub const fn select_next(&mut self) {
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

    pub const fn select_previous(&mut self) {
        if self.items.is_empty() {
            self.selected_item = None;
            return;
        }

        self.selected_item = Some(match self.selected_item {
            Some(current) if current > 0 => current - 1,
            Some(_) | None => 0,
        });
    }

    pub fn start_generation(&mut self, project_path: String) {
        self.generation_request = Some(RoadmapGenerationRequest::spawn(project_path));
        self.mode = super::state::RoadmapMode::Generating;
    }

    pub fn poll_generation(&mut self) {
        if let Some(ref request) = self.generation_request
            && let Some(result) = request.try_recv()
        {
            self.generation_request = None;
            self.mode = super::state::RoadmapMode::Normal;

            if let Ok(generated) = result {
                self.apply_generated_roadmap(generated);
            }
        }
    }

    fn apply_generated_roadmap(&mut self, generated: GeneratedRoadmap) {
        for generated_item in generated.items {
            let item = generated_item.into_roadmap_item();
            self.items.push(item);
        }

        self.sort_by_priority();

        if !self.items.is_empty() {
            self.selected_item = Some(0);
        }
    }
}
