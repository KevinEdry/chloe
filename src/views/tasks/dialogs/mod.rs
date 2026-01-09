mod exit_confirmation;
mod review_popup;

pub use exit_confirmation::render_exit_confirmation_dialog;
pub use review_popup::render_review_popup;

use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    widgets::Clear,
};

const PERCENTAGE_FULL: u16 = 100;

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
