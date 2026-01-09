use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::Color,
    widgets::Clear,
};

pub use crate::helpers::text::{truncate as truncate_string, wrap as wrap_text};
pub use crate::widgets::claude_indicator::dot as get_claude_state_indicator_for_card;

const PERCENTAGE_FULL: u16 = 100;

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

pub fn centered_rect(percent_x: u16, percent_y: u16, area: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((PERCENTAGE_FULL - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((PERCENTAGE_FULL - percent_y) / 2),
        ])
        .split(area);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((PERCENTAGE_FULL - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((PERCENTAGE_FULL - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}

pub fn render_popup_background(frame: &mut Frame, area: Rect) {
    frame.render_widget(Clear, area);
}
