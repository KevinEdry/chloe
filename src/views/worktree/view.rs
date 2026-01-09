use super::tab_state::{WorktreeMode, WorktreeTabState};
use crate::views::StatusBarContent;
use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph},
};

const STATUS_BAR_WIDTH_THRESHOLD: u16 = 100;

pub fn render(frame: &mut Frame, area: Rect, state: &WorktreeTabState) {
    render_worktree_list(frame, area, state);

    if let WorktreeMode::ConfirmDelete { worktree_index } = state.mode {
        render_delete_confirmation(frame, area, state, worktree_index);
    }
}

fn render_worktree_list(frame: &mut Frame, area: Rect, state: &WorktreeTabState) {
    if let Some(error) = &state.error_message {
        let error_text = Paragraph::new(error.as_str())
            .style(Style::default().fg(Color::Red))
            .alignment(Alignment::Center)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Git Worktrees - Error")
                    .border_style(Style::default().fg(Color::DarkGray)),
            );

        frame.render_widget(error_text, area);
        return;
    }

    if state.worktrees.is_empty() {
        let empty_text = Paragraph::new("No worktrees found.")
            .style(Style::default().fg(Color::Gray))
            .alignment(Alignment::Center)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Git Worktrees")
                    .border_style(Style::default().fg(Color::DarkGray)),
            );

        frame.render_widget(empty_text, area);
        return;
    }

    let items: Vec<ListItem> = state
        .worktrees
        .iter()
        .enumerate()
        .map(|(index, worktree)| {
            let is_selected = state.selected_index == Some(index);

            let branch_style = if worktree.is_detached {
                Style::default().fg(Color::Yellow)
            } else {
                Style::default().fg(Color::Green)
            };

            let path_display = worktree.path.to_string_lossy().to_string();

            let content = Line::from(vec![
                Span::styled(
                    format!("  {} ", worktree.branch_name),
                    branch_style.add_modifier(Modifier::BOLD),
                ),
                Span::styled(
                    format!("→ {path_display}"),
                    Style::default().fg(Color::Gray),
                ),
            ]);

            let mut item = ListItem::new(content);

            if is_selected {
                item = item.style(
                    Style::default()
                        .bg(Color::DarkGray)
                        .add_modifier(Modifier::BOLD),
                );
            }

            item
        })
        .collect();

    let list = List::new(items).block(
        Block::default()
            .borders(Borders::ALL)
            .title(format!("Git Worktrees ({})", state.worktrees.len()))
            .border_style(Style::default().fg(Color::DarkGray)),
    );

    frame.render_widget(list, area);
}

#[must_use]
pub fn get_status_bar_content(state: &WorktreeTabState, width: u16) -> StatusBarContent {
    let mode_color = match state.mode {
        WorktreeMode::Normal => Color::Cyan,
        WorktreeMode::ConfirmDelete { .. } => Color::Red,
    };

    let mode_text = match state.mode {
        WorktreeMode::Normal => "NORMAL",
        WorktreeMode::ConfirmDelete { .. } => "CONFIRM DELETE",
    };

    let help_text = if width < STATUS_BAR_WIDTH_THRESHOLD {
        match state.mode {
            WorktreeMode::Normal => "jk:navigate  o:open  r:refresh  d:delete",
            WorktreeMode::ConfirmDelete { .. } => "y:yes  n:no",
        }
    } else {
        match state.mode {
            WorktreeMode::Normal => {
                "↑↓/jk:navigate  o:open  r:refresh  d:delete  Tab:switch-tabs  q:quit"
            }
            WorktreeMode::ConfirmDelete { .. } => "y:yes  n:no  Esc:cancel",
        }
    };

    StatusBarContent {
        mode_text: mode_text.to_string(),
        mode_color,
        extra_info: None,
        help_text: help_text.to_string(),
    }
}

fn render_delete_confirmation(
    frame: &mut Frame,
    area: Rect,
    state: &WorktreeTabState,
    worktree_index: usize,
) {
    const POPUP_WIDTH_PERCENT: u16 = 60;
    const POPUP_HEIGHT: u16 = 7;

    let popup_area = centered_rect(POPUP_WIDTH_PERCENT, POPUP_HEIGHT, area);

    let Some(worktree) = state.worktrees.get(worktree_index) else {
        return;
    };

    let confirmation_text = vec![
        Line::from(""),
        Line::from(Span::styled(
            "Delete worktree?",
            Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
        Line::from(Span::styled(
            format!("Branch: {}", worktree.branch_name),
            Style::default().fg(Color::Yellow),
        )),
        Line::from(Span::styled(
            format!("Path: {}", worktree.path.display()),
            Style::default().fg(Color::Gray),
        )),
    ];

    let paragraph = Paragraph::new(confirmation_text)
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Confirm Delete")
                .style(Style::default().bg(Color::Black)),
        );

    frame.render_widget(paragraph, popup_area);
}

fn centered_rect(percent_width: u16, height: u16, area: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length((area.height.saturating_sub(height)) / 2),
            Constraint::Length(height),
            Constraint::Min(0),
        ])
        .split(area);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_width) / 2),
            Constraint::Percentage(percent_width),
            Constraint::Min(0),
        ])
        .split(popup_layout[1])[1]
}
