use crate::views::roadmap::GeneratedRoadmap;
use crate::views::tasks::ai_classifier::ClassifiedTask;
use uuid::Uuid;

use super::HookEvent;

#[derive(Debug, Clone)]
pub enum AppEvent {
    PtyOutput {
        pane_id: Uuid,
        data: Vec<u8>,
    },
    PtyExit {
        pane_id: Uuid,
    },

    ClassificationCompleted {
        task_id: Uuid,
        result: Result<ClassifiedTask, String>,
    },

    RoadmapGenerationCompleted {
        result: Result<GeneratedRoadmap, String>,
    },

    HookReceived(HookEvent),
}
