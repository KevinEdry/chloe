use super::tab_state::{WorktreeMode, WorktreeTabState};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame,
};

const HEADER_HEIGHT: u16 = 3;
const HELP_HEIGHT: u16 = 3;

pub fn render(frame: &mut Frame, area: Rect, state: &WorktreeTabState) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(HEADER_HEIGHT),
            Constraint::Min(0),
            Constraint::Length(HELP_HEIGHT),
        ])
        .split(area);

    render_header(frame, chunks[0]);
    render_worktree_list(frame, chunks[1], state);
    render_help(frame, chunks[2], state);

    if let WorktreeMode::ConfirmDelete { worktree_index } = state.mode {
        render_delete_confirmation(frame, area, state, worktree_index);
    }
}

fn render_header(frame: &mut Frame, area: Rect) {
    let title = Paragraph::new("Git Worktrees")
        .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL));

    frame.render_widget(title, area);
}

fn render_worktree_list(frame: &mut Frame, area: Rect, state: &WorktreeTabState) {
    if let Some(error) = &state.error_message {
        let error_text = Paragraph::new(error.as_str())
            .style(Style::default().fg(Color::Red))
            .alignment(Alignment::Center)
            .block(Block::default().borders(Borders::ALL).title("Error"));

        frame.render_widget(error_text, area);
        return;
    }

    if state.worktrees.is_empty() {
        let empty_text = Paragraph::new("No worktrees found. Press 'r' to refresh.")
            .style(Style::default().fg(Color::Gray))
            .alignment(Alignment::Center)
            .block(Block::default().borders(Borders::ALL).title("Worktrees"));

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

            let path_display = worktree
                .path
                .to_string_lossy()
                .to_string();

            let content = Line::from(vec![
                Span::styled(
                    format!("  {} ", worktree.branch_name),
                    branch_style.add_modifier(Modifier::BOLD),
                ),
                Span::styled(
                    format!("→ {}", path_display),
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
            .title(format!("Worktrees ({})", state.worktrees.len())),
    );

    frame.render_widget(list, area);
}

fn render_help(frame: &mut Frame, area: Rect, state: &WorktreeTabState) {
    let help_text = match state.mode {
        WorktreeMode::Normal => {
            "j/k: Navigate | r: Refresh | d: Delete | Tab: Switch tabs | q: Quit"
        }
        WorktreeMode::ConfirmDelete { .. } => "y: Confirm delete | n/Esc: Cancel",
    };

    let help = Paragraph::new(help_text)
        .style(Style::default().fg(Color::Gray))
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL));

    frame.render_widget(help, area);
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

    let worktree = match state.worktrees.get(worktree_index) {
        Some(wt) => wt,
        None => return,
    };

    let confirmation_text = vec![
        Line::from(""),
        Line::from(Span::styled(
            "Delete worktree?",
            Style::default()
                .fg(Color::Red)
                .add_modifier(Modifier::BOLD),
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
