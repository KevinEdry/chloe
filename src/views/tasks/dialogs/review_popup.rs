use super::{centered_rect, render_popup_background};
use crate::app::App;
use crate::views::tasks::state::ReviewAction;
use crate::views::worktree::{WorktreeStatus, get_worktree_status};
use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Wrap},
};
use uuid::Uuid;

const REVIEW_POPUP_WIDTH_PERCENT: u16 = 90;
const REVIEW_POPUP_HEIGHT_PERCENT: u16 = 90;

const BUTTON_COUNT: usize = 5;
const BUTTON_WIDTH_PERCENT: u16 = 20;

const STATUS_HEADER_HEIGHT: u16 = 8;
const BUTTON_ROW_HEIGHT: u16 = 3;

pub struct ReviewInfo {
    pub branch_name: Option<String>,
    pub worktree_status: WorktreeStatus,
    pub task_title: String,
}

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

    let review_info = get_review_info(app, task_id);
    let is_clean = review_info.worktree_status.is_clean;

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(STATUS_HEADER_HEIGHT),
            Constraint::Min(0),
            Constraint::Length(BUTTON_ROW_HEIGHT),
        ])
        .split(dialog_area);

    render_status_header(frame, &review_info, chunks[0]);
    render_output_section(frame, app, task_id, scroll_offset, chunks[1]);
    render_action_buttons(frame, selected_action, is_clean, chunks[2]);
}

fn get_review_info(app: &App, task_id: Uuid) -> ReviewInfo {
    let task = app.tasks.find_task_by_id(task_id);

    let default_info = ReviewInfo {
        branch_name: None,
        worktree_status: WorktreeStatus::default(),
        task_title: "Unknown Task".to_string(),
    };

    let Some(task) = task else {
        return default_info;
    };

    let task_title = task.title.clone();

    let Some(worktree_info) = &task.worktree_info else {
        return ReviewInfo {
            branch_name: None,
            worktree_status: WorktreeStatus {
                is_clean: true,
                ..WorktreeStatus::default()
            },
            task_title,
        };
    };

    let worktree_status = get_worktree_status(&worktree_info.worktree_path)
        .unwrap_or_default();

    ReviewInfo {
        branch_name: Some(worktree_info.branch_name.clone()),
        worktree_status,
        task_title,
    }
}

fn render_status_header(frame: &mut Frame, info: &ReviewInfo, area: Rect) {
    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Cyan))
        .title(Span::styled(
            format!(" Review: {} ", info.task_title),
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        ));

    let inner_area = block.inner(area);
    frame.render_widget(block, area);

    let mut lines = Vec::new();

    if let Some(branch) = &info.branch_name {
        lines.push(Line::from(vec![
            Span::styled("Branch: ", Style::default().fg(Color::DarkGray)),
            Span::styled(branch, Style::default().fg(Color::White)),
        ]));
    } else {
        lines.push(Line::from(Span::styled(
            "No worktree associated",
            Style::default().fg(Color::DarkGray),
        )));
    }

    let status = &info.worktree_status;
    if status.is_clean {
        lines.push(Line::from(Span::styled(
            "Status: Ready to merge",
            Style::default().fg(Color::Green),
        )));
    } else if status.has_conflicts {
        lines.push(Line::from(Span::styled(
            "Status: Has merge conflicts",
            Style::default().fg(Color::Red),
        )));
    } else {
        let count = status.uncommitted_count();
        lines.push(Line::from(Span::styled(
            format!("Status: {} uncommitted change{}", count, if count == 1 { "" } else { "s" }),
            Style::default().fg(Color::Yellow),
        )));
    }

    if !status.is_clean {
        lines.push(Line::from(""));

        let max_files_to_show = 3;
        let mut shown = 0;

        for file in &status.modified_files {
            if shown >= max_files_to_show {
                break;
            }
            lines.push(Line::from(vec![
                Span::styled("  M ", Style::default().fg(Color::Yellow)),
                Span::styled(file, Style::default().fg(Color::White)),
            ]));
            shown += 1;
        }

        for file in &status.untracked_files {
            if shown >= max_files_to_show {
                break;
            }
            lines.push(Line::from(vec![
                Span::styled("  ? ", Style::default().fg(Color::Green)),
                Span::styled(file, Style::default().fg(Color::White)),
            ]));
            shown += 1;
        }

        let total = status.uncommitted_count();
        if total > max_files_to_show {
            lines.push(Line::from(Span::styled(
                format!("  ... and {} more", total - max_files_to_show),
                Style::default().fg(Color::DarkGray),
            )));
        }
    }

    let text = Paragraph::new(lines);
    frame.render_widget(text, inner_area);
}

fn render_output_section(
    frame: &mut Frame,
    app: &App,
    task_id: Uuid,
    scroll_offset: usize,
    area: Rect,
) {
    let task = app.tasks.find_task_by_id(task_id);

    let output_text = task.map_or("Task not found", |task| {
        task.instance_id
            .map_or("No instance associated with this task", |instance_id| {
                app.get_instance_output(instance_id)
                    .unwrap_or("No output available")
            })
    });

    let output_lines: Vec<&str> = output_text.lines().collect();
    let total_lines = output_lines.len();

    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::DarkGray))
        .title(Span::styled(
            " Output ",
            Style::default().fg(Color::DarkGray),
        ))
        .title_bottom(
            Line::from(vec![Span::styled(
                format!(
                    " Lines {}-{} of {} ",
                    scroll_offset + 1,
                    (scroll_offset + area.height as usize).min(total_lines),
                    total_lines
                ),
                Style::default().fg(Color::DarkGray),
            )])
            .alignment(Alignment::Right),
        );

    let inner_area = block.inner(area);
    frame.render_widget(block, area);

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
}

fn render_action_buttons(
    frame: &mut Frame,
    selected_action: ReviewAction,
    is_clean: bool,
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
        let is_enabled = action.is_enabled(is_clean);

        let style = if !is_enabled {
            Style::default().fg(Color::DarkGray).bg(Color::Black)
        } else if is_selected {
            Style::default()
                .fg(Color::Black)
                .bg(Color::Cyan)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Color::Cyan).bg(Color::Black)
        };

        let border_style = if !is_enabled {
            Style::default().fg(Color::DarkGray)
        } else if is_selected {
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Color::DarkGray)
        };

        let label = action.label();
        let button = Paragraph::new(label)
            .alignment(Alignment::Center)
            .style(style)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(border_style),
            );

        frame.render_widget(button, button_areas[index]);
    }
}
