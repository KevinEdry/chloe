mod details_panel;
mod dialogs;
mod items;

use super::{RoadmapMode, RoadmapState};
use crate::app::App;
use crate::views::StatusBarContent;
use details_panel::render_details_panel;
use dialogs::{
    render_confirm_dialog, render_convert_dialog, render_input_dialog, render_loading_dialog,
};
use items::render_items_list;
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::Color,
};

const STATUS_BAR_WIDTH_THRESHOLD: u16 = 100;

pub fn render(f: &mut Frame, app: &App, area: Rect) {
    let state = &app.roadmap;

    let content_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(35), Constraint::Percentage(65)])
        .split(area);

    render_items_list(f, state, content_chunks[0]);
    render_details_panel(f, state, content_chunks[1]);

    match &state.mode {
        RoadmapMode::AddingItem { input } => {
            render_input_dialog(f, "Add Roadmap Item", input, area);
        }
        RoadmapMode::EditingItem { input, .. } => {
            render_input_dialog(f, "Edit Roadmap Item", input, area);
        }
        RoadmapMode::ConfirmDelete { .. } => {
            render_confirm_dialog(f, "Delete this roadmap item? (y/n)", area);
        }
        RoadmapMode::ConvertToTask { item_index } => {
            render_convert_dialog(f, state, *item_index, area);
        }
        RoadmapMode::Generating => {
            render_loading_dialog(f, state, area);
        }
        RoadmapMode::Normal => {}
    }
}

#[must_use]
pub fn get_status_bar_content(state: &RoadmapState, width: u16) -> StatusBarContent {
    let mode_color = match &state.mode {
        RoadmapMode::Normal => Color::Cyan,
        RoadmapMode::AddingItem { .. } => Color::Green,
        RoadmapMode::EditingItem { .. } => Color::Yellow,
        RoadmapMode::ConfirmDelete { .. } => Color::Red,
        RoadmapMode::ConvertToTask { .. } | RoadmapMode::Generating => Color::Magenta,
    };

    let mode_text = match &state.mode {
        RoadmapMode::Normal => "NORMAL",
        RoadmapMode::AddingItem { .. } => "ADD ITEM",
        RoadmapMode::EditingItem { .. } => "EDIT ITEM",
        RoadmapMode::ConfirmDelete { .. } => "CONFIRM DELETE",
        RoadmapMode::ConvertToTask { .. } => "CONVERT TO TASK",
        RoadmapMode::Generating => "GENERATING",
    };

    let help_text = if width < STATUS_BAR_WIDTH_THRESHOLD {
        match &state.mode {
            RoadmapMode::Normal => "jk:navigate  a:add  g:generate  e:edit  d:delete  t:convert",
            RoadmapMode::AddingItem { .. } | RoadmapMode::EditingItem { .. } => {
                "Enter:save  Esc:cancel"
            }
            RoadmapMode::ConfirmDelete { .. } | RoadmapMode::ConvertToTask { .. } => "y:yes  n:no",
            RoadmapMode::Generating => "Esc:cancel",
        }
    } else {
        match &state.mode {
            RoadmapMode::Normal => {
                "↑↓/jk:navigate  a:add  g:generate-with-ai  e:edit  d:delete  t:convert-to-task  p:priority  q:quit"
            }
            RoadmapMode::AddingItem { .. } | RoadmapMode::EditingItem { .. } => {
                "Type to enter text  Enter:save  Esc:cancel"
            }
            RoadmapMode::ConfirmDelete { .. } | RoadmapMode::ConvertToTask { .. } => {
                "y:yes  n:no  Esc:cancel"
            }
            RoadmapMode::Generating => "AI is analyzing your project... Press Esc to cancel",
        }
    };

    StatusBarContent {
        mode_text: mode_text.to_string(),
        mode_color,
        extra_info: None,
        help_text: help_text.to_string(),
    }
}
