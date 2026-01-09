use crate::views::focus::operations::TaskReference;
use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Wrap},
};

pub fn render(frame: &mut Frame, selected_task: Option<&TaskReference<'_>>, area: Rect) {
    let block = Block::default()
        .title("Details")
        .title_style(
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Cyan));

    let inner_area = block.inner(area);
    frame.render_widget(block, area);

    if let Some(task_ref) = selected_task {
        render_task_details(frame, task_ref, inner_area);
    } else {
        render_no_selection(frame, inner_area);
    }
}

fn render_task_details(frame: &mut Frame, task_ref: &TaskReference<'_>, area: Rect) {
    let task = task_ref.task;

    let mut lines = vec![
        Line::from(vec![Span::styled(
            &task.title,
            Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        )]),
        Line::from(""),
        Line::from(vec![
            Span::styled("Type: ", Style::default().fg(Color::DarkGray)),
            Span::styled(
                task.task_type.badge_text(),
                Style::default()
                    .fg(task.task_type.color())
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw("  "),
            Span::styled("Stage: ", Style::default().fg(Color::DarkGray)),
            Span::styled(task_ref.column_name, Style::default().fg(Color::Cyan)),
        ]),
        Line::from(""),
    ];

    if !task.description.is_empty() {
        lines.push(Line::from(vec![Span::styled(
            "Description:",
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )]));

        for line in task.description.lines() {
            lines.push(Line::from(Span::styled(
                line,
                Style::default().fg(Color::White),
            )));
        }
        lines.push(Line::from(""));
    }

    if let Some(worktree_info) = &task.worktree_info {
        lines.push(Line::from(vec![
            Span::styled("Path: ", Style::default().fg(Color::DarkGray)),
            Span::styled(
                worktree_info.worktree_path.display().to_string(),
                Style::default().fg(Color::Yellow),
            ),
        ]));
        lines.push(Line::from(""));
    }

    lines.push(Line::from(vec![
        Span::styled("Created: ", Style::default().fg(Color::DarkGray)),
        Span::styled(
            task.created_at.format("%Y-%m-%d %H:%M").to_string(),
            Style::default().fg(Color::Gray),
        ),
    ]));

    if task.instance_id.is_some() {
        lines.push(Line::from(""));
        lines.push(Line::from(vec![Span::styled(
            "Terminal active - press Enter to interact",
            Style::default().fg(Color::Green),
        )]));
    }

    let paragraph = Paragraph::new(lines)
        .wrap(Wrap { trim: false })
        .scroll((0, 0));

    frame.render_widget(paragraph, area);
}

fn render_no_selection(frame: &mut Frame, area: Rect) {
    let text = Paragraph::new(vec![
        Line::from(""),
        Line::from(vec![Span::styled(
            "No task selected",
            Style::default().fg(Color::DarkGray),
        )]),
        Line::from(""),
        Line::from(vec![Span::styled(
            "Navigate with j/k or arrow keys",
            Style::default().fg(Color::DarkGray),
        )]),
    ])
    .wrap(Wrap { trim: false });

    frame.render_widget(text, area);
}
