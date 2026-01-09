use ratatui::style::Color;

pub use crate::helpers::text::{truncate as truncate_string, wrap as wrap_text};
pub use crate::widgets::claude_indicator::dot as get_claude_state_indicator_for_card;

pub const COLUMN_COLORS: [Color; 4] = [
    Color::Cyan,    // Planning
    Color::Yellow,  // In Progress
    Color::Magenta, // Review
    Color::Green,   // Done
];

pub const COLUMN_COLORS_SELECTED: [Color; 4] = [
    Color::LightCyan,    // Planning
    Color::LightYellow,  // In Progress
    Color::LightMagenta, // Review
    Color::LightGreen,   // Done
];
