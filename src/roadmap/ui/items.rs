use crate::roadmap::RoadmapState;
use ratatui::{
    Frame,
    layout::{Alignment, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem},
};

const TITLE_ELLIPSIS_THRESHOLD: usize = 40;

pub fn render_items_list(f: &mut Frame, state: &RoadmapState, area: Rect) {
    let mut block = Block::default()
        .title("Roadmap Items")
        .title_style(
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Cyan));

    if !state.items.is_empty() {
        if let Some(selected_idx) = state.selected_item {
            let position_text = format!(" {} of {} ", selected_idx + 1, state.items.len());
            block = block.title_bottom(
                Line::from(vec![Span::styled(
                    position_text,
                    Style::default()
                        .fg(Color::White)
                        .add_modifier(Modifier::BOLD),
                )])
                .alignment(Alignment::Right),
            );
        }
    }

    let inner_area = block.inner(area);
    f.render_widget(block, area);

    if state.items.is_empty() {
        let empty_message = List::new(vec![
            ListItem::new(Line::from("")),
            ListItem::new(Line::from(vec![Span::styled(
                "No items yet",
                Style::default().fg(Color::DarkGray),
            )])),
            ListItem::new(Line::from("")),
            ListItem::new(Line::from(vec![Span::styled(
                "Press 'a' to add",
                Style::default().fg(Color::DarkGray),
            )])),
            ListItem::new(Line::from(vec![Span::styled(
                "Press 'g' to generate",
                Style::default().fg(Color::DarkGray),
            )])),
        ]);
        f.render_widget(empty_message, inner_area);
        return;
    }

    let items: Vec<ListItem> = state
        .items
        .iter()
        .enumerate()
        .map(|(index, item)| {
            let is_selected = state.selected_item == Some(index);

            let priority_char = match item.priority {
                crate::roadmap::RoadmapPriority::High => "H",
                crate::roadmap::RoadmapPriority::Medium => "M",
                crate::roadmap::RoadmapPriority::Low => "L",
            };

            let title_text = if item.title.len() > TITLE_ELLIPSIS_THRESHOLD {
                format!("{}...", &item.title[..TITLE_ELLIPSIS_THRESHOLD])
            } else {
                item.title.clone()
            };

            let line = if is_selected {
                Line::from(vec![
                    Span::styled("▶ ", Style::default().fg(Color::Cyan)),
                    Span::styled(
                        priority_char,
                        Style::default()
                            .fg(item.priority.color())
                            .add_modifier(Modifier::BOLD),
                    ),
                    Span::raw(" "),
                    Span::styled(
                        title_text,
                        Style::default()
                            .fg(Color::White)
                            .add_modifier(Modifier::BOLD),
                    ),
                ])
            } else {
                Line::from(vec![
                    Span::raw("  "),
                    Span::styled(priority_char, Style::default().fg(item.priority.color())),
                    Span::raw(" "),
                    Span::styled(title_text, Style::default().fg(Color::Gray)),
                ])
            };

            ListItem::new(line)
        })
        .collect();

    let list = List::new(items);
    f.render_widget(list, inner_area);
}
