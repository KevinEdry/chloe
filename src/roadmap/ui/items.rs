use crate::roadmap::RoadmapState;
use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Padding},
};

const CARD_PADDING: u16 = 2;
const TITLE_ELLIPSIS_THRESHOLD: usize = 50;

pub fn render_items_list(f: &mut Frame, state: &RoadmapState, area: Rect) {
    let block = Block::default()
        .title("Roadmap")
        .title_style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Cyan))
        .padding(Padding::uniform(1));

    let inner_area = block.inner(area);
    f.render_widget(block, area);

    if state.items.is_empty() {
        let empty_message = List::new(vec![ListItem::new(Line::from(vec![Span::styled(
            "No roadmap items. Press 'a' to add one.",
            Style::default().fg(Color::DarkGray),
        )]))]);
        f.render_widget(empty_message, inner_area);
        return;
    }

    let items: Vec<ListItem> = state
        .items
        .iter()
        .enumerate()
        .map(|(index, item)| {
            let is_selected = state.selected_item == Some(index);

            let priority_badge = format!("[{}]", item.priority.label());
            let status_badge = format!("[{}]", item.status.label());

            let title_text = if item.title.len() > TITLE_ELLIPSIS_THRESHOLD {
                format!("{}...", &item.title[..TITLE_ELLIPSIS_THRESHOLD])
            } else {
                item.title.clone()
            };

            let line = Line::from(vec![
                Span::styled(
                    priority_badge,
                    Style::default().fg(item.priority.color()).add_modifier(Modifier::BOLD),
                ),
                Span::raw(" "),
                Span::styled(
                    status_badge,
                    Style::default().fg(item.status.color()),
                ),
                Span::raw(" "),
                Span::styled(
                    title_text,
                    if is_selected {
                        Style::default()
                            .fg(Color::White)
                            .add_modifier(Modifier::BOLD | Modifier::REVERSED)
                    } else {
                        Style::default().fg(Color::White)
                    },
                ),
            ]);

            ListItem::new(line)
        })
        .collect();

    let list = List::new(items);
    f.render_widget(list, inner_area);
}
