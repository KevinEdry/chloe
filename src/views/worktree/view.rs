use super::tab_state::{WorktreeMode, WorktreeTabState};
use crate::views::StatusBarContent;
use crate::views::settings::VcsCommand;
use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Padding, Paragraph},
};

const STATUS_BAR_WIDTH_THRESHOLD: u16 = 100;
const HEADER_HEIGHT: u16 = 5;

const BRANCH_ICON: &str = "";
const BARE_ICON: &str = "󰋊";
const DETACHED_ICON: &str = "";
const PATH_ICON: &str = "";
const SELECTED_INDICATOR: &str = "▶";

pub fn render(frame: &mut Frame, area: Rect, state: &WorktreeTabState, vcs_command: &VcsCommand) {
    render_worktree_list(frame, area, state, vcs_command);

    if let WorktreeMode::ConfirmDelete { worktree_index } = state.mode {
        render_delete_confirmation(frame, area, state, worktree_index, vcs_command);
    }
}

fn render_worktree_list(
    frame: &mut Frame,
    area: Rect,
    state: &WorktreeTabState,
    vcs_command: &VcsCommand,
) {
    let outer_block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::DarkGray))
        .padding(Padding::horizontal(1));

    let inner_area = outer_block.inner(area);
    frame.render_widget(outer_block, area);

    if let Some(error) = &state.error_message {
        render_error_state(frame, inner_area, error, vcs_command);
        return;
    }

    if state.worktrees.is_empty() {
        render_empty_state(frame, inner_area, vcs_command);
        return;
    }

    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(HEADER_HEIGHT), Constraint::Min(0)])
        .split(inner_area);

    render_header(frame, layout[0], state.worktrees.len(), vcs_command);
    render_worktree_items(frame, layout[1], state);
}

fn render_header(frame: &mut Frame, area: Rect, count: usize, vcs_command: &VcsCommand) {
    let header_lines = vec![
        Line::from(""),
        Line::from(vec![
            Span::styled(
                format!("  {}", vcs_command.full_title()),
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(format!("  ({count})"), Style::default().fg(Color::DarkGray)),
        ]),
        Line::from(Span::styled(
            format!("  {}", vcs_command.description()),
            Style::default().fg(Color::Gray),
        )),
        Line::from(Span::styled(
            "  ─────────────────────────────────────────────────────────────",
            Style::default().fg(Color::DarkGray),
        )),
    ];

    frame.render_widget(Paragraph::new(header_lines), area);
}

fn render_error_state(frame: &mut Frame, area: Rect, error: &str, vcs_command: &VcsCommand) {
    let error_lines = vec![
        Line::from(""),
        Line::from(Span::styled(
            format!("  ⚠ Error Loading {}", vcs_command.workspace_term_plural()),
            Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
        Line::from(Span::styled(
            format!("  {error}"),
            Style::default().fg(Color::Red),
        )),
        Line::from(""),
        Line::from(Span::styled(
            "  Press 'r' to retry",
            Style::default().fg(Color::Gray),
        )),
    ];

    frame.render_widget(Paragraph::new(error_lines), area);
}

fn render_empty_state(frame: &mut Frame, area: Rect, vcs_command: &VcsCommand) {
    let workspace_plural = vcs_command.workspace_term_plural();
    let workspace_singular = vcs_command.workspace_term();

    let empty_lines = vec![
        Line::from(""),
        Line::from(Span::styled(
            format!("  No {workspace_plural} Found"),
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
        Line::from(Span::styled(
            format!("  {workspace_plural} will appear here when you start tasks."),
            Style::default().fg(Color::Gray),
        )),
        Line::from(Span::styled(
            "  Each task can have its own isolated working directory.",
            Style::default().fg(Color::Gray),
        )),
        Line::from(""),
        Line::from(Span::styled(
            format!(
                "  Tip: Start a task in the Tasks tab to create a {}.",
                workspace_singular.to_lowercase()
            ),
            Style::default().fg(Color::DarkGray),
        )),
    ];

    frame.render_widget(Paragraph::new(empty_lines), area);
}

fn render_worktree_items(frame: &mut Frame, area: Rect, state: &WorktreeTabState) {
    let items: Vec<ListItem> = state
        .worktrees
        .iter()
        .enumerate()
        .flat_map(|(index, worktree)| {
            let is_selected = state.selected_index == Some(index);
            build_worktree_item(index, worktree, is_selected)
        })
        .collect();

    let list = List::new(items);
    frame.render_widget(list, area);
}

fn build_worktree_item(
    _index: usize,
    worktree: &super::state::Worktree,
    is_selected: bool,
) -> Vec<ListItem<'static>> {
    let (branch_icon, branch_color) = if worktree.is_bare {
        (BARE_ICON, Color::Blue)
    } else if worktree.is_detached {
        (DETACHED_ICON, Color::Yellow)
    } else {
        (BRANCH_ICON, Color::Green)
    };

    let path_display = worktree.path.to_string_lossy().to_string();

    let selection_indicator = if is_selected { SELECTED_INDICATOR } else { " " };

    let branch_line = Line::from(vec![
        Span::styled(
            format!("  {selection_indicator} "),
            Style::default().fg(Color::Cyan),
        ),
        Span::styled(format!("{branch_icon} "), Style::default().fg(branch_color)),
        Span::styled(
            worktree.branch_name.clone(),
            Style::default()
                .fg(branch_color)
                .add_modifier(Modifier::BOLD),
        ),
        build_status_badge(worktree),
    ]);

    let path_line = Line::from(vec![
        Span::styled("      ", Style::default()),
        Span::styled(
            format!("{PATH_ICON} "),
            Style::default().fg(Color::DarkGray),
        ),
        Span::styled(path_display, Style::default().fg(Color::Gray)),
    ]);

    let separator_line = Line::from(Span::styled(
        "      ┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄",
        Style::default().fg(Color::Rgb(50, 50, 50)),
    ));

    let base_style = if is_selected {
        Style::default().bg(Color::Rgb(40, 40, 50))
    } else {
        Style::default()
    };

    vec![
        ListItem::new(branch_line).style(base_style),
        ListItem::new(path_line).style(base_style),
        ListItem::new(separator_line),
    ]
}

fn build_status_badge(worktree: &super::state::Worktree) -> Span<'static> {
    if worktree.is_bare {
        Span::styled(
            "  [bare]",
            Style::default().fg(Color::Blue).add_modifier(Modifier::DIM),
        )
    } else if worktree.is_detached {
        Span::styled(
            "  [detached]",
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::DIM),
        )
    } else {
        Span::styled("", Style::default())
    }
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
    vcs_command: &VcsCommand,
) {
    const POPUP_WIDTH_PERCENT: u16 = 60;
    const POPUP_HEIGHT: u16 = 7;

    let popup_area = centered_rect(POPUP_WIDTH_PERCENT, POPUP_HEIGHT, area);

    let Some(worktree) = state.worktrees.get(worktree_index) else {
        return;
    };

    let workspace_singular = vcs_command.workspace_term().to_lowercase();

    let confirmation_text = vec![
        Line::from(""),
        Line::from(Span::styled(
            format!("Delete {workspace_singular}?"),
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
