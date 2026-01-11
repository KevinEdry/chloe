use crate::views::tasks::state::WorktreeSelectionOption;
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Padding, Paragraph},
};

use super::{centered_rect, render_popup_background};

const POPUP_WIDTH_PERCENT: u16 = 70;
const POPUP_HEIGHT_PERCENT: u16 = 80;
const TASK_BLOCK_HEIGHT: u16 = 5;
const DISCLAIMER_BLOCK_HEIGHT: u16 = 9;
const VERTICAL_GAP: u16 = 1;

pub struct WorktreeSelectionViewState<'a> {
    pub task_title: &'a str,
    pub selected_index: usize,
    pub options: &'a [WorktreeSelectionOption],
}

pub fn render_worktree_selection(
    frame: &mut Frame,
    state: &WorktreeSelectionViewState<'_>,
    area: Rect,
) {
    let popup_area = centered_rect(POPUP_WIDTH_PERCENT, POPUP_HEIGHT_PERCENT, area);
    render_popup_background(frame, popup_area);

    let outer_block = Block::default()
        .title(" Select Worktree ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Cyan))
        .padding(Padding::uniform(1));

    let inner_area = outer_block.inner(popup_area);
    frame.render_widget(outer_block, popup_area);

    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(TASK_BLOCK_HEIGHT),
            Constraint::Length(VERTICAL_GAP),
            Constraint::Min(0),
            Constraint::Length(VERTICAL_GAP),
            Constraint::Length(DISCLAIMER_BLOCK_HEIGHT),
        ])
        .split(inner_area);

    render_task_block(frame, layout[0], state.task_title);
    render_selection_block(frame, layout[2], state);
    render_disclaimer_block(frame, layout[4]);
}

fn render_task_block(frame: &mut Frame, area: Rect, task_title: &str) {
    let block = Block::default()
        .title(" Task ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::DarkGray))
        .padding(Padding::horizontal(1));

    let inner = block.inner(area);
    frame.render_widget(block, area);

    let content = vec![
        Line::from(""),
        Line::from(vec![
            Span::styled(" ", Style::default().fg(Color::Yellow)),
            Span::styled("  ", Style::default()),
            Span::styled(
                task_title,
                Style::default()
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
    ];

    frame.render_widget(Paragraph::new(content), inner);
}

fn render_selection_block(frame: &mut Frame, area: Rect, state: &WorktreeSelectionViewState<'_>) {
    let block = Block::default()
        .title(" Choose Worktree ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::DarkGray))
        .padding(Padding::horizontal(1));

    let inner = block.inner(area);
    frame.render_widget(block, area);

    let list = build_options_list(state);
    frame.render_widget(list, inner);
}

fn render_disclaimer_block(frame: &mut Frame, area: Rect) {
    let block = Block::default()
        .title(" Why Worktrees? ")
        .title_style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::DarkGray))
        .padding(Padding::horizontal(1));

    let inner = block.inner(area);
    frame.render_widget(block, area);

    let content = vec![
        Line::from(""),
        Line::from(Span::styled(
            "Git worktrees let each task have its own isolated working",
            Style::default().fg(Color::Gray),
        )),
        Line::from(Span::styled(
            "directory with a separate branch. AI agents can work on",
            Style::default().fg(Color::Gray),
        )),
        Line::from(Span::styled(
            "multiple tasks simultaneously without code conflicts.",
            Style::default().fg(Color::Gray),
        )),
        Line::from(""),
        Line::from(Span::styled(
            "Your main branch stays clean and you can review each",
            Style::default().fg(Color::DarkGray),
        )),
        Line::from(Span::styled(
            "task's work independently before merging.",
            Style::default().fg(Color::DarkGray),
        )),
    ];

    frame.render_widget(Paragraph::new(content), inner);
}

fn build_options_list(state: &WorktreeSelectionViewState<'_>) -> List<'static> {
    let items: Vec<ListItem> = state
        .options
        .iter()
        .enumerate()
        .map(|(index, option)| render_option(index, option, state.selected_index))
        .collect();

    List::new(items)
        .block(Block::default().borders(Borders::NONE))
        .highlight_style(
            Style::default()
                .bg(Color::DarkGray)
                .add_modifier(Modifier::BOLD),
        )
}

fn render_option(
    index: usize,
    option: &WorktreeSelectionOption,
    selected_index: usize,
) -> ListItem<'static> {
    let is_selected = index == selected_index;

    let content = match option {
        WorktreeSelectionOption::AutoCreate => Line::from(vec![Span::styled(
            "Create a new worktree",
            Style::default()
                .fg(Color::Green)
                .add_modifier(Modifier::BOLD),
        )]),
        WorktreeSelectionOption::Existing {
            branch_name,
            worktree_path,
        } => {
            let path_display = worktree_path.to_string_lossy().to_string();
            Line::from(vec![
                Span::styled(
                    format!("{branch_name} "),
                    Style::default()
                        .fg(Color::White)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::styled(
                    format!("→ {path_display}"),
                    Style::default().fg(Color::Gray),
                ),
            ])
        }
    };

    let mut item = ListItem::new(content);

    if is_selected {
        item = item.style(
            Style::default()
                .bg(Color::DarkGray)
                .add_modifier(Modifier::BOLD),
        );
    }

    item
}
