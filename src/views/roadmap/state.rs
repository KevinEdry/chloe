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
    #[serde(skip)]
    pub spinner_frame: usize,
}

impl RoadmapState {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            items: Vec::new(),
            selected_item: None,
            mode: RoadmapMode::Normal,
            generation_request: None,
            spinner_frame: 0,
        }
    }

    pub fn sort_items_by_priority(&mut self) {
        self.items.sort_by(|a, b| {
            use std::cmp::Ordering;
            match (a.priority, b.priority) {
                (RoadmapPriority::High, RoadmapPriority::High)
                | (RoadmapPriority::Medium, RoadmapPriority::Medium)
                | (RoadmapPriority::Low, RoadmapPriority::Low) => Ordering::Equal,
                (RoadmapPriority::High, RoadmapPriority::Medium | RoadmapPriority::Low)
                | (RoadmapPriority::Medium, RoadmapPriority::Low) => Ordering::Less,
                (RoadmapPriority::Medium | RoadmapPriority::Low, RoadmapPriority::High)
                | (RoadmapPriority::Low, RoadmapPriority::Medium) => Ordering::Greater,
            }
        });
    }

    pub const fn advance_spinner(&mut self) {
        self.spinner_frame = (self.spinner_frame + 1) % 10;
    }

    #[must_use]
    pub const fn get_spinner_char(&self) -> char {
        const SPINNER_FRAMES: [char; 10] = ['⠋', '⠙', '⠹', '⠸', '⠼', '⠴', '⠦', '⠧', '⠇', '⠏'];
        SPINNER_FRAMES[self.spinner_frame]
    }

    #[must_use]
    pub fn get_selected_item(&self) -> Option<&RoadmapItem> {
        self.selected_item.and_then(|index| self.items.get(index))
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
    #[must_use]
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

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum RoadmapStatus {
    #[default]
    Planned,
    InProgress,
    Completed,
    Cancelled,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum RoadmapPriority {
    High,
    #[default]
    Medium,
    Low,
}

impl RoadmapPriority {
    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::High => "High",
            Self::Medium => "Medium",
            Self::Low => "Low",
        }
    }

    #[must_use]
    pub const fn color(self) -> Color {
        match self {
            Self::High => Color::Red,
            Self::Medium => Color::Yellow,
            Self::Low => Color::Gray,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum RoadmapMode {
    Normal,
    AddingItem { input: String },
    EditingItem { item_index: usize, input: String },
    ConfirmDelete { item_index: usize },
    ConvertToTask { item_index: usize },
    Generating,
}
