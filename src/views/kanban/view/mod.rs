mod columns;
pub mod dialogs;
mod helpers;

use super::{KanbanMode, KanbanState};
use crate::app::App;
use crate::views::StatusBarContent;
use columns::render_columns;
use dialogs::{
    render_classifying_dialog, render_confirm_dialog, render_input_dialog, render_review_popup,
};
use ratatui::{Frame, layout::Rect, style::Color};

const STATUS_BAR_WIDTH_THRESHOLD: u16 = 100;

pub fn render(f: &mut Frame, app: &App, area: Rect) {
    let state = &app.kanban;

    render_columns(f, app, area);

    match &state.mode {
        KanbanMode::AddingTask { input } => {
            render_input_dialog(f, "Add Task to Planning", input, area);
        }
        KanbanMode::EditingTask { input, .. } => {
            render_input_dialog(f, "Edit Task", input, area);
        }
        KanbanMode::ConfirmDelete { .. } => {
            render_confirm_dialog(f, "Delete this task? (y/n)", area);
        }
        KanbanMode::ConfirmMoveBack { .. } => {
            render_confirm_dialog(
                f,
                "Move back to Planning? This will terminate the Claude Code instance. (y/n)",
                area,
            );
        }
        KanbanMode::ClassifyingTask { raw_input, .. } => {
            render_classifying_dialog(f, raw_input, area);
        }
        KanbanMode::ReviewPopup {
            task_index,
            scroll_offset,
            selected_action,
        } => {
            render_review_popup(f, app, *task_index, *scroll_offset, *selected_action, area);
        }
        KanbanMode::ReviewRequestChanges { input, .. } => {
            render_input_dialog(f, "Request Changes", input, area);
        }
        KanbanMode::Normal => {}
    }
}

pub fn get_status_bar_content(state: &KanbanState, width: u16) -> StatusBarContent {
    let mode_color = match &state.mode {
        KanbanMode::Normal => Color::Cyan,
        KanbanMode::AddingTask { .. } => Color::Green,
        KanbanMode::EditingTask { .. } => Color::Yellow,
        KanbanMode::ConfirmDelete { .. } => Color::Red,
        KanbanMode::ConfirmMoveBack { .. } => Color::LightRed,
        KanbanMode::ClassifyingTask { .. } => Color::Magenta,
        KanbanMode::ReviewPopup { .. } => Color::Cyan,
        KanbanMode::ReviewRequestChanges { .. } => Color::Yellow,
    };

    let mode_text = match &state.mode {
        KanbanMode::Normal => "NORMAL",
        KanbanMode::AddingTask { .. } => "ADD TO PLANNING",
        KanbanMode::EditingTask { .. } => "EDIT TASK",
        KanbanMode::ConfirmDelete { .. } => "CONFIRM DELETE",
        KanbanMode::ConfirmMoveBack { .. } => "CONFIRM MOVE BACK",
        KanbanMode::ClassifyingTask { .. } => "AI CLASSIFYING",
        KanbanMode::ReviewPopup { .. } => "REVIEW OUTPUT",
        KanbanMode::ReviewRequestChanges { .. } => "REQUEST CHANGES",
    };

    let help_text = if width < STATUS_BAR_WIDTH_THRESHOLD {
        match &state.mode {
            KanbanMode::Normal => "hjkl/arrows:navigate  a:add  e:edit  d:delete",
            KanbanMode::ClassifyingTask { .. } => "Esc:cancel",
            KanbanMode::AddingTask { .. } | KanbanMode::EditingTask { .. } => {
                "Enter:save  Esc:cancel"
            }
            KanbanMode::ConfirmDelete { .. } => "y:yes  n:no",
            KanbanMode::ConfirmMoveBack { .. } => "y:yes  n:no",
            KanbanMode::ReviewPopup { .. } => "jk:scroll  hl/Tab:button  Enter:action",
            KanbanMode::ReviewRequestChanges { .. } => "Enter:save  Esc:cancel",
        }
    } else {
        match &state.mode {
            KanbanMode::Normal => {
                "↑↓/jk:task  ←→/hl:column  a:add-to-planning  e:edit  d:delete  Enter:move→  Backspace:move←  q:quit"
            }
            KanbanMode::AddingTask { .. } | KanbanMode::EditingTask { .. } => {
                "Type to enter text  Enter:save  Esc:cancel"
            }
            KanbanMode::ConfirmDelete { .. } => "y:yes  n:no  Esc:cancel",
            KanbanMode::ConfirmMoveBack { .. } => "y:yes  n:no  Esc:cancel",
            KanbanMode::ClassifyingTask { .. } => "Press Esc to cancel classification",
            KanbanMode::ReviewPopup { .. } => {
                "jk:scroll  ←→/hl/Tab:select-button  Enter:execute-action  q/Esc:close"
            }
            KanbanMode::ReviewRequestChanges { .. } => {
                "Type your change request  Enter:save  Esc:cancel"
            }
        }
    };

    StatusBarContent {
        mode_text: mode_text.to_string(),
        mode_color,
        extra_info: None,
        help_text: help_text.to_string(),
    }
}
