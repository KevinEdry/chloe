use crate::types::AgentProvider;
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Padding, Paragraph},
};

use super::{centered_rect, render_popup_background};

const POPUP_WIDTH_PERCENT: u16 = 50;
const POPUP_HEIGHT_PERCENT: u16 = 60;
const TASK_BLOCK_HEIGHT: u16 = 5;
const VERTICAL_GAP: u16 = 1;

pub struct ProviderSelectionViewState<'a> {
    pub task_title: &'a str,
    pub selected_index: usize,
    pub default_provider: AgentProvider,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProviderSelectionResult {
    Provider(AgentProvider),
    ProviderAndRemember(AgentProvider),
}

impl ProviderSelectionResult {
    #[must_use]
    pub const fn provider(self) -> AgentProvider {
        match self {
            Self::Provider(provider) | Self::ProviderAndRemember(provider) => provider,
        }
    }

    #[must_use]
    pub const fn should_remember(self) -> bool {
        matches!(self, Self::ProviderAndRemember(_))
    }
}

#[must_use]
pub fn get_selection_result_with_default(
    selected_index: usize,
    default_provider: AgentProvider,
) -> Option<ProviderSelectionResult> {
    let providers = AgentProvider::all();
    let provider_count = providers.len();

    match selected_index.cmp(&provider_count) {
        std::cmp::Ordering::Less => {
            Some(ProviderSelectionResult::Provider(providers[selected_index]))
        }
        std::cmp::Ordering::Equal => {
            Some(ProviderSelectionResult::ProviderAndRemember(default_provider))
        }
        std::cmp::Ordering::Greater => None,
    }
}

#[must_use]
pub const fn get_option_count() -> usize {
    AgentProvider::all().len() + 1
}

pub fn render_provider_selection(
    frame: &mut Frame,
    state: &ProviderSelectionViewState<'_>,
    area: Rect,
) {
    let popup_area = centered_rect(POPUP_WIDTH_PERCENT, POPUP_HEIGHT_PERCENT, area);
    render_popup_background(frame, popup_area);

    let outer_block = Block::default()
        .title(" Select AI Provider ")
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
        ])
        .split(inner_area);

    render_task_block(frame, layout[0], state.task_title);
    render_selection_block(frame, layout[2], state);
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

fn render_selection_block(frame: &mut Frame, area: Rect, state: &ProviderSelectionViewState<'_>) {
    let block = Block::default()
        .title(" Choose Provider ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::DarkGray))
        .padding(Padding::horizontal(1));

    let inner = block.inner(area);
    frame.render_widget(block, area);

    let list = build_provider_list(state);
    frame.render_widget(list, inner);
}

fn build_provider_list(state: &ProviderSelectionViewState<'_>) -> List<'static> {
    let providers = AgentProvider::all();
    let mut items: Vec<ListItem> = providers
        .iter()
        .enumerate()
        .map(|(index, provider)| {
            render_provider_option(index, *provider, state.selected_index, state.default_provider)
        })
        .collect();

    items.push(render_remember_option(
        providers.len(),
        state.selected_index,
        state.default_provider,
    ));

    List::new(items)
        .block(Block::default().borders(Borders::NONE))
        .highlight_style(
            Style::default()
                .bg(Color::DarkGray)
                .add_modifier(Modifier::BOLD),
        )
}

fn render_provider_option(
    index: usize,
    provider: AgentProvider,
    selected_index: usize,
    default_provider: AgentProvider,
) -> ListItem<'static> {
    let is_selected = index == selected_index;
    let is_default = provider == default_provider;

    let mut spans = vec![Span::styled(
        provider.display_name().to_string(),
        Style::default()
            .fg(Color::White)
            .add_modifier(Modifier::BOLD),
    )];

    if is_default {
        spans.push(Span::styled(
            " (default)",
            Style::default().fg(Color::DarkGray),
        ));
    }

    let content = Line::from(spans);
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

fn render_remember_option(
    index: usize,
    selected_index: usize,
    default_provider: AgentProvider,
) -> ListItem<'static> {
    let is_selected = index == selected_index;

    let content = Line::from(vec![
        Span::styled(
            format!("Use {} and don't ask again", default_provider.display_name()),
            Style::default()
                .fg(Color::Green)
                .add_modifier(Modifier::BOLD),
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
}
