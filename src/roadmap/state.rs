use chrono::{DateTime, Utc};
use ratatui::style::Color;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoadmapState {
    pub items: Vec<RoadmapItem>,
    pub selected_item: Option<usize>,
    pub mode: RoadmapMode,
    #[serde(skip)]
    pub generation_request: Option<super::generator::RoadmapGenerationRequest>,
}

impl RoadmapState {
    pub fn new() -> Self {
        Self {
            items: Vec::new(),
            selected_item: None,
            mode: RoadmapMode::Normal,
            generation_request: None,
        }
    }

    pub fn get_selected_item(&self) -> Option<&RoadmapItem> {
        self.selected_item
            .and_then(|index| self.items.get(index))
    }

    pub fn get_selected_item_mut(&mut self) -> Option<&mut RoadmapItem> {
        self.selected_item
            .and_then(|index| self.items.get_mut(index))
    }
}

impl Default for RoadmapState {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoadmapItem {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub rationale: String,
    pub user_stories: Vec<String>,
    pub acceptance_criteria: Vec<String>,
    pub status: RoadmapStatus,
    pub priority: RoadmapPriority,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    #[serde(default)]
    pub dependencies: Vec<Uuid>,
    #[serde(default)]
    pub tags: Vec<String>,
}

impl RoadmapItem {
    pub fn new(
        title: String,
        description: String,
        rationale: String,
        priority: RoadmapPriority,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            title,
            description,
            rationale,
            user_stories: Vec::new(),
            acceptance_criteria: Vec::new(),
            status: RoadmapStatus::Planned,
            priority,
            created_at: now,
            updated_at: now,
            dependencies: Vec::new(),
            tags: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum RoadmapStatus {
    Planned,
    InProgress,
    Completed,
    Cancelled,
}

impl RoadmapStatus {
    pub const fn label(self) -> &'static str {
        match self {
            Self::Planned => "Planned",
            Self::InProgress => "In Progress",
            Self::Completed => "Completed",
            Self::Cancelled => "Cancelled",
        }
    }

    pub const fn color(self) -> Color {
        match self {
            Self::Planned => Color::Gray,
            Self::InProgress => Color::Cyan,
            Self::Completed => Color::Green,
            Self::Cancelled => Color::Red,
        }
    }
}

impl Default for RoadmapStatus {
    fn default() -> Self {
        Self::Planned
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum RoadmapPriority {
    High,
    Medium,
    Low,
}

impl RoadmapPriority {
    pub const fn label(self) -> &'static str {
        match self {
            Self::High => "High",
            Self::Medium => "Medium",
            Self::Low => "Low",
        }
    }

    pub const fn color(self) -> Color {
        match self {
            Self::High => Color::Red,
            Self::Medium => Color::Yellow,
            Self::Low => Color::Gray,
        }
    }
}

impl Default for RoadmapPriority {
    fn default() -> Self {
        Self::Medium
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum RoadmapMode {
    Normal,
    AddingItem {
        input: String,
    },
    EditingItem {
        item_index: usize,
        input: String,
    },
    ViewingDetails {
        item_index: usize,
        scroll_offset: usize,
    },
    ConfirmDelete {
        item_index: usize,
    },
    ConvertToTask {
        item_index: usize,
    },
    Generating,
}
