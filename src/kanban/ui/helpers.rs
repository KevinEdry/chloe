use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::Color,
    widgets::Clear,
};

const PERCENTAGE_FULL: u16 = 100;
const CLAUDE_STATE_BLINK_DURATION_MS: u128 = 500;
const CLAUDE_STATE_BLINK_PHASES: u128 = 2;

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

pub fn truncate_string(s: &str, max_len: usize) -> String {
    if s.len() <= max_len {
        s.to_string()
    } else if max_len > 3 {
        format!("{}...", &s[..max_len - 3])
    } else {
        String::new()
    }
}

pub fn wrap_text(text: &str, max_width: usize) -> Vec<String> {
    let mut lines = Vec::new();
    let words: Vec<&str> = text.split_whitespace().collect();

    if words.is_empty() {
        return lines;
    }

    let mut current_line = String::new();

    for word in words {
        let word_len = word.len();
        let space_len = if current_line.is_empty() { 0 } else { 1 };

        if current_line.len() + space_len + word_len <= max_width {
            if !current_line.is_empty() {
                current_line.push(' ');
            }
            current_line.push_str(word);
        } else {
            if !current_line.is_empty() {
                lines.push(current_line);
                current_line = String::new();
            }

            if word_len <= max_width {
                current_line.push_str(word);
            } else {
                current_line.push_str(&word[..max_width.saturating_sub(3)]);
                current_line.push_str("...");
            }
        }
    }

    if !current_line.is_empty() {
        lines.push(current_line);
    }

    lines
}

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

pub fn render_popup_background(f: &mut Frame, area: Rect) {
    f.render_widget(Clear, area);
}

pub fn get_claude_state_indicator_for_card(
    state: crate::instance::ClaudeState,
) -> (&'static str, Color) {
    use std::time::{SystemTime, UNIX_EPOCH};

    let should_flash = match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(duration) => {
            (duration.as_millis() / CLAUDE_STATE_BLINK_DURATION_MS) % CLAUDE_STATE_BLINK_PHASES == 0
        }
        Err(_) => true,
    };

    match state {
        crate::instance::ClaudeState::Idle => (" ", Color::Gray),
        crate::instance::ClaudeState::Running if should_flash => ("●", Color::Rgb(255, 165, 0)),
        crate::instance::ClaudeState::Running => (" ", Color::Rgb(255, 165, 0)),
        crate::instance::ClaudeState::NeedsPermissions => ("●", Color::Rgb(138, 43, 226)),
        crate::instance::ClaudeState::Done => ("●", Color::Green),
    }
}
