use crate::views::instances::ClaudeState;
use ratatui::style::Color;
use std::time::{SystemTime, UNIX_EPOCH};

const BLINK_DURATION_MS: u128 = 500;
const BLINK_PHASES: u128 = 2;

pub struct ClaudeIndicator {
    state: ClaudeState,
}

impl ClaudeIndicator {
    #[must_use]
    pub const fn new(state: ClaudeState) -> Self {
        Self { state }
    }

    fn should_blink() -> bool {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_or(true, |duration| {
                (duration.as_millis() / BLINK_DURATION_MS) % BLINK_PHASES == 0
            })
    }

    #[must_use]
    pub fn label(&self) -> (&'static str, Color) {
        let should_blink = Self::should_blink();

        match self.state {
            ClaudeState::Idle => ("Idle", Color::Gray),
            ClaudeState::Running if should_blink => ("Running", Color::Rgb(255, 165, 0)),
            ClaudeState::Running => ("Running", Color::Rgb(255, 165, 0)),
            ClaudeState::NeedsPermissions => ("Needs Permission", Color::Rgb(138, 43, 226)),
            ClaudeState::Done => ("Done", Color::Green),
        }
    }

    #[must_use]
    pub fn dot(&self) -> (&'static str, Color) {
        let should_blink = Self::should_blink();

        match self.state {
            ClaudeState::Idle => (" ", Color::Gray),
            ClaudeState::Running if should_blink => ("●", Color::Rgb(255, 165, 0)),
            ClaudeState::Running => (" ", Color::Rgb(255, 165, 0)),
            ClaudeState::NeedsPermissions => ("●", Color::Rgb(138, 43, 226)),
            ClaudeState::Done => ("●", Color::Green),
        }
    }

    #[must_use]
    pub fn dot_visible(&self) -> (&'static str, Color) {
        let should_blink = Self::should_blink();

        match self.state {
            ClaudeState::Idle => ("", Color::Gray),
            ClaudeState::Running if should_blink => ("●", Color::Rgb(255, 165, 0)),
            ClaudeState::Running => ("", Color::Rgb(255, 165, 0)),
            ClaudeState::NeedsPermissions => ("●", Color::Rgb(138, 43, 226)),
            ClaudeState::Done => ("●", Color::Green),
        }
    }
}

#[must_use]
pub fn label(state: ClaudeState) -> (&'static str, Color) {
    ClaudeIndicator::new(state).label()
}

#[must_use]
pub fn dot(state: ClaudeState) -> (&'static str, Color) {
    ClaudeIndicator::new(state).dot()
}

#[must_use]
pub fn dot_visible(state: ClaudeState) -> (&'static str, Color) {
    ClaudeIndicator::new(state).dot_visible()
}
