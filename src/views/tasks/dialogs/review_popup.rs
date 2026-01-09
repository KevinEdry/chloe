use super::{centered_rect, render_popup_background};
use crate::app::App;
use crate::views::tasks::state::ReviewAction;
use crate::views::worktree::operations::{check_merge_conflicts, get_default_branch};
use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Wrap},
};
use std::path::Path;
use uuid::Uuid;

const REVIEW_POPUP_WIDTH_PERCENT: u16 = 90;
const REVIEW_POPUP_HEIGHT_PERCENT: u16 = 90;

const BUTTON_COUNT: usize = 4;
const BUTTON_WIDTH_PERCENT: u16 = 25;

pub fn render_review_popup(
    frame: &mut Frame,
    app: &App,
    task_id: Uuid,
    scroll_offset: usize,
    selected_action: ReviewAction,
    area: Rect,
) {
    let dialog_area = centered_rect(
        REVIEW_POPUP_WIDTH_PERCENT,
        REVIEW_POPUP_HEIGHT_PERCENT,
        area,
    );

    render_popup_background(frame, dialog_area);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(3)])
        .split(dialog_area);

    let task = app.tasks.find_task_by_id(task_id);

    let output_text = if let Some(task) = task {
        if let Some(instance_id) = task.instance_id {
            app.get_instance_output(instance_id)
                .unwrap_or("No output available")
        } else {
            "No instance associated with this task"
        }
    } else {
        "Task not found"
    };

    let output_lines: Vec<&str> = output_text.lines().collect();
    let total_lines = output_lines.len();

    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )
        .title(Span::styled(
            " Claude Code Output ",
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        ))
        .title_bottom(
            Line::from(vec![Span::styled(
                format!(
                    " Lines {}-{} of {} ",
                    scroll_offset + 1,
                    (scroll_offset + chunks[0].height as usize).min(total_lines),
                    total_lines
                ),
                Style::default().fg(Color::DarkGray),
            )])
            .alignment(Alignment::Right),
        );

    let inner_area = block.inner(chunks[0]);
    frame.render_widget(block, chunks[0]);

    let visible_lines: Vec<Line> = output_lines
        .iter()
        .skip(scroll_offset)
        .take(inner_area.height as usize)
        .map(|line| Line::from(Span::styled(*line, Style::default().fg(Color::White))))
        .collect();

    let text = Paragraph::new(visible_lines)
        .alignment(Alignment::Left)
        .wrap(Wrap { trim: false });

    frame.render_widget(text, inner_area);

    let (default_branch, has_conflicts) = get_merge_info(app, task_id);
    render_action_buttons(
        frame,
        selected_action,
        default_branch.as_deref(),
        has_conflicts,
        chunks[1],
    );
}

fn get_merge_info(app: &App, task_id: Uuid) -> (Option<String>, bool) {
    let task = app.tasks.find_task_by_id(task_id);

    let Some(task) = task else {
        return (None, false);
    };

    let Some(worktree_info) = &task.worktree_info else {
        return (None, false);
    };

    let repository_path = Path::new(".");
    let default_branch = get_default_branch(repository_path).ok();
    let has_conflicts = check_merge_conflicts(repository_path, worktree_info)
        .ok()
        .flatten()
        .is_some();

    (default_branch, has_conflicts)
}

fn render_action_buttons(
    frame: &mut Frame,
    selected_action: ReviewAction,
    default_branch: Option<&str>,
    has_conflicts: bool,
    area: Rect,
) {
    let actions = ReviewAction::all();
    let button_constraints = vec![Constraint::Percentage(BUTTON_WIDTH_PERCENT); BUTTON_COUNT];

    let button_areas = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(button_constraints)
        .split(area);

    for (index, action) in actions.iter().enumerate() {
        let is_selected = *action == selected_action;
        let style = if is_selected {
            Style::default()
                .fg(Color::Black)
                .bg(Color::Cyan)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Color::Cyan).bg(Color::Black)
        };

        let label = action.label(default_branch, has_conflicts);
        let button = Paragraph::new(label)
            .alignment(Alignment::Center)
            .style(style)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(if is_selected {
                        Style::default()
                            .fg(Color::Cyan)
                            .add_modifier(Modifier::BOLD)
                    } else {
                        Style::default().fg(Color::DarkGray)
                    }),
            );

        frame.render_widget(button, button_areas[index]);
    }
}
