use super::super::RoadmapState;
use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Wrap},
};

pub fn render_details_panel(f: &mut Frame, state: &RoadmapState, area: Rect) {
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
    f.render_widget(block, area);

    if let Some(item) = state.get_selected_item() {
        let mut lines = vec![
            Line::from(vec![Span::styled(
                &item.title,
                Style::default()
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD),
            )]),
            Line::from(""),
            Line::from(vec![
                Span::styled("Priority: ", Style::default().fg(Color::DarkGray)),
                Span::styled(
                    item.priority.label(),
                    Style::default()
                        .fg(item.priority.color())
                        .add_modifier(Modifier::BOLD),
                ),
            ]),
            Line::from(""),
        ];

        if !item.description.is_empty() {
            lines.push(Line::from(vec![Span::styled(
                "Description:",
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            )]));
            lines.push(Line::from(Span::styled(
                &item.description,
                Style::default().fg(Color::White),
            )));
            lines.push(Line::from(""));
        }

        if !item.rationale.is_empty() {
            lines.push(Line::from(vec![Span::styled(
                "Rationale:",
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            )]));
            lines.push(Line::from(Span::styled(
                &item.rationale,
                Style::default().fg(Color::White),
            )));
            lines.push(Line::from(""));
        }

        if !item.user_stories.is_empty() {
            lines.push(Line::from(vec![Span::styled(
                "User Stories:",
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            )]));
            for story in &item.user_stories {
                lines.push(Line::from(vec![
                    Span::raw("  • "),
                    Span::styled(story, Style::default().fg(Color::White)),
                ]));
            }
            lines.push(Line::from(""));
        }

        if !item.acceptance_criteria.is_empty() {
            lines.push(Line::from(vec![Span::styled(
                "Acceptance Criteria:",
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            )]));
            for criterion in &item.acceptance_criteria {
                lines.push(Line::from(vec![
                    Span::raw("  • "),
                    Span::styled(criterion, Style::default().fg(Color::White)),
                ]));
            }
            lines.push(Line::from(""));
        }

        if !item.tags.is_empty() {
            lines.push(Line::from(vec![Span::styled(
                "Tags:",
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            )]));
            let tags_text = item.tags.join(", ");
            lines.push(Line::from(Span::styled(
                tags_text,
                Style::default().fg(Color::Magenta),
            )));
        }

        let paragraph = Paragraph::new(lines)
            .wrap(Wrap { trim: false })
            .scroll((0, 0));

        f.render_widget(paragraph, inner_area);
    } else {
        let empty_text = Paragraph::new(vec![
            Line::from(""),
            Line::from(""),
            Line::from(vec![Span::styled(
                "No item selected",
                Style::default().fg(Color::DarkGray),
            )]),
            Line::from(""),
            Line::from(vec![Span::styled(
                "Press 'a' to add a new item",
                Style::default().fg(Color::DarkGray),
            )]),
            Line::from(vec![Span::styled(
                "Press 'g' to generate with AI",
                Style::default().fg(Color::DarkGray),
            )]),
        ])
        .wrap(Wrap { trim: false });

        f.render_widget(empty_text, inner_area);
    }
}
