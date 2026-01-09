use crate::views::instances::ClaudeState;
use ratatui::style::Color;
use std::time::{SystemTime, UNIX_EPOCH};

const BLINK_DURATION_MS: u128 = 500;
const BLINK_PHASES: u128 = 2;

pub fn get_indicator(state: ClaudeState) -> (&'static str, Color) {
    let should_flash = match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(duration) => (duration.as_millis() / BLINK_DURATION_MS) % BLINK_PHASES == 0,
        Err(_) => true,
    };

    match state {
        ClaudeState::Idle => ("Idle", Color::Gray),
        ClaudeState::Running if should_flash => ("Running", Color::Rgb(255, 165, 0)),
        ClaudeState::Running => ("Running", Color::Rgb(255, 165, 0)),
        ClaudeState::NeedsPermissions => ("Needs Permission", Color::Rgb(138, 43, 226)),
        ClaudeState::Done => ("Done", Color::Green),
    }
}

pub fn get_indicator_dot(state: ClaudeState) -> (&'static str, Color) {
    let should_flash = match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(duration) => (duration.as_millis() / BLINK_DURATION_MS) % BLINK_PHASES == 0,
        Err(_) => true,
    };

    match state {
        ClaudeState::Idle => (" ", Color::Gray),
        ClaudeState::Running if should_flash => ("●", Color::Rgb(255, 165, 0)),
        ClaudeState::Running => (" ", Color::Rgb(255, 165, 0)),
        ClaudeState::NeedsPermissions => ("●", Color::Rgb(138, 43, 226)),
        ClaudeState::Done => ("●", Color::Green),
    }
}
