use super::super::RoadmapState;
use super::super::state::RoadmapItem;
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

    match state.get_selected_item() {
        Some(item) => render_item_details(f, item, inner_area),
        None => render_empty_state(f, inner_area),
    }
}

fn render_item_details(f: &mut Frame, item: &RoadmapItem, area: Rect) {
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

    append_text_section(&mut lines, "Description:", &item.description);
    append_text_section(&mut lines, "Rationale:", &item.rationale);
    append_list_section(&mut lines, "User Stories:", &item.user_stories);
    append_list_section(
        &mut lines,
        "Acceptance Criteria:",
        &item.acceptance_criteria,
    );
    append_tags_section(&mut lines, &item.tags);

    let paragraph = Paragraph::new(lines)
        .wrap(Wrap { trim: false })
        .scroll((0, 0));

    f.render_widget(paragraph, area);
}

fn render_empty_state(f: &mut Frame, area: Rect) {
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

    f.render_widget(empty_text, area);
}

fn append_text_section(lines: &mut Vec<Line<'_>>, title: &'static str, content: &str) {
    if content.is_empty() {
        return;
    }

    lines.push(Line::from(vec![Span::styled(
        title,
        Style::default()
            .fg(Color::Cyan)
            .add_modifier(Modifier::BOLD),
    )]));
    lines.push(Line::from(Span::styled(
        content.to_string(),
        Style::default().fg(Color::White),
    )));
    lines.push(Line::from(""));
}

fn append_list_section(lines: &mut Vec<Line<'_>>, title: &'static str, items: &[String]) {
    if items.is_empty() {
        return;
    }

    lines.push(Line::from(vec![Span::styled(
        title,
        Style::default()
            .fg(Color::Cyan)
            .add_modifier(Modifier::BOLD),
    )]));
    for item in items {
        lines.push(Line::from(vec![
            Span::raw("  • "),
            Span::styled(item.clone(), Style::default().fg(Color::White)),
        ]));
    }
    lines.push(Line::from(""));
}

fn append_tags_section(lines: &mut Vec<Line<'_>>, tags: &[String]) {
    if tags.is_empty() {
        return;
    }

    lines.push(Line::from(vec![Span::styled(
        "Tags:",
        Style::default()
            .fg(Color::Cyan)
            .add_modifier(Modifier::BOLD),
    )]));
    let tags_text = tags.join(", ");
    lines.push(Line::from(Span::styled(
        tags_text,
        Style::default().fg(Color::Magenta),
    )));
}
