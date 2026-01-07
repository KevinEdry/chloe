mod dialogs;
mod items;

use crate::app::App;
use crate::roadmap::{RoadmapMode, RoadmapState};
use dialogs::{render_confirm_dialog, render_convert_dialog, render_details_view, render_input_dialog};
use items::render_items_list;
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
};

const STATUS_BAR_WIDTH_THRESHOLD: u16 = 100;

pub fn render(f: &mut Frame, app: &App, area: Rect) {
    let state = &app.roadmap;
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(3)])
        .split(area);

    render_items_list(f, state, chunks[0]);
    render_status_bar(f, state, chunks[1]);

    match &state.mode {
        RoadmapMode::AddingItem { input } => {
            render_input_dialog(f, "Add Roadmap Item", input, area);
        }
        RoadmapMode::EditingItem { input, .. } => {
            render_input_dialog(f, "Edit Roadmap Item", input, area);
        }
        RoadmapMode::ViewingDetails {
            item_index,
            scroll_offset,
        } => {
            render_details_view(f, state, *item_index, *scroll_offset, area);
        }
        RoadmapMode::ConfirmDelete { .. } => {
            render_confirm_dialog(f, "Delete this roadmap item? (y/n)", area);
        }
        RoadmapMode::ConvertToTask { item_index } => {
            render_convert_dialog(f, state, *item_index, area);
        }
        RoadmapMode::Generating => {
            render_confirm_dialog(f, "Generating roadmap with AI... (press Esc to cancel)", area);
        }
        RoadmapMode::Normal => {}
    }
}

fn render_status_bar(f: &mut Frame, state: &RoadmapState, area: Rect) {
    let mode_color = match &state.mode {
        RoadmapMode::Normal => Color::Cyan,
        RoadmapMode::AddingItem { .. } => Color::Green,
        RoadmapMode::EditingItem { .. } => Color::Yellow,
        RoadmapMode::ViewingDetails { .. } => Color::Cyan,
        RoadmapMode::ConfirmDelete { .. } => Color::Red,
        RoadmapMode::ConvertToTask { .. } => Color::Magenta,
        RoadmapMode::Generating => Color::Magenta,
    };

    let mode_text = match &state.mode {
        RoadmapMode::Normal => "NORMAL",
        RoadmapMode::AddingItem { .. } => "ADD ITEM",
        RoadmapMode::EditingItem { .. } => "EDIT ITEM",
        RoadmapMode::ViewingDetails { .. } => "VIEW DETAILS",
        RoadmapMode::ConfirmDelete { .. } => "CONFIRM DELETE",
        RoadmapMode::ConvertToTask { .. } => "CONVERT TO TASK",
        RoadmapMode::Generating => "GENERATING",
    };

    let help_text = if area.width < STATUS_BAR_WIDTH_THRESHOLD {
        match &state.mode {
            RoadmapMode::Normal => "jk:navigate  a:add  g:generate  e:edit  d:delete  t:convert",
            RoadmapMode::AddingItem { .. } | RoadmapMode::EditingItem { .. } => {
                "Enter:save  Esc:cancel"
            }
            RoadmapMode::ViewingDetails { .. } => "jk:scroll  q/Esc:close",
            RoadmapMode::ConfirmDelete { .. } => "y:yes  n:no",
            RoadmapMode::ConvertToTask { .. } => "y:yes  n:no",
            RoadmapMode::Generating => "Esc:cancel",
        }
    } else {
        match &state.mode {
            RoadmapMode::Normal => {
                "↑↓/jk:navigate  a:add  g:generate-with-ai  e:edit  d:delete  Enter:details  t:convert-to-task  p:priority  s:status  q:quit"
            }
            RoadmapMode::AddingItem { .. } | RoadmapMode::EditingItem { .. } => {
                "Type to enter text  Enter:save  Esc:cancel"
            }
            RoadmapMode::ViewingDetails { .. } => "↑↓/jk:scroll  q/Esc:close",
            RoadmapMode::ConfirmDelete { .. } => "y:yes  n:no  Esc:cancel",
            RoadmapMode::ConvertToTask { .. } => "y:yes  n:no  Esc:cancel",
            RoadmapMode::Generating => "AI is analyzing your project... Press Esc to cancel",
        }
    };

    let status = Paragraph::new(Line::from(vec![
        Span::styled(
            format!("[{}] ", mode_text),
            Style::default().fg(mode_color).add_modifier(Modifier::BOLD),
        ),
        Span::styled(help_text, Style::default().fg(Color::DarkGray)),
    ]))
    .block(
        Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::DarkGray)),
    );

    f.render_widget(status, area);
}
