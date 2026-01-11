use crate::types::{AgentProvider, DetectedProvider};
use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Padding},
};

use super::{centered_rect, render_popup_background};

const POPUP_WIDTH_PERCENT: u16 = 50;
const POPUP_HEIGHT_PERCENT: u16 = 50;

pub struct ProviderSelectionViewState<'a> {
    pub selected_index: usize,
    pub default_provider: AgentProvider,
    pub detected_providers: &'a [DetectedProvider],
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
pub fn get_selection_result(
    selected_index: usize,
    detected_providers: &[DetectedProvider],
    default_provider: AgentProvider,
) -> Option<ProviderSelectionResult> {
    let provider_count = detected_providers.len();

    match selected_index.cmp(&provider_count) {
        std::cmp::Ordering::Less => Some(ProviderSelectionResult::Provider(
            detected_providers[selected_index].provider,
        )),
        std::cmp::Ordering::Equal => Some(ProviderSelectionResult::ProviderAndRemember(
            default_provider,
        )),
        std::cmp::Ordering::Greater => None,
    }
}

#[must_use]
pub const fn get_option_count(detected_providers: &[DetectedProvider]) -> usize {
    detected_providers.len() + 1
}

pub fn render_provider_selection(
    frame: &mut Frame,
    state: &ProviderSelectionViewState<'_>,
    area: Rect,
) {
    let popup_area = centered_rect(POPUP_WIDTH_PERCENT, POPUP_HEIGHT_PERCENT, area);
    render_popup_background(frame, popup_area);

    let block = Block::default()
        .title(" Select AI Provider ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Cyan))
        .padding(Padding::uniform(1));

    let inner_area = block.inner(popup_area);
    frame.render_widget(block, popup_area);

    let list = build_provider_list(state);
    frame.render_widget(list, inner_area);
}

fn build_provider_list(state: &ProviderSelectionViewState<'_>) -> List<'static> {
    let mut items: Vec<ListItem> = state
        .detected_providers
        .iter()
        .enumerate()
        .map(|(index, detected)| {
            render_provider_option(
                index,
                detected,
                state.selected_index,
                state.default_provider,
            )
        })
        .collect();

    items.push(render_remember_option(
        state.detected_providers.len(),
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
    detected: &DetectedProvider,
    selected_index: usize,
    default_provider: AgentProvider,
) -> ListItem<'static> {
    let is_selected = index == selected_index;
    let is_default = detected.provider == default_provider;

    let mut name_spans = vec![Span::styled(
        detected.provider.display_name().to_string(),
        Style::default()
            .fg(Color::White)
            .add_modifier(Modifier::BOLD),
    )];

    if is_default {
        name_spans.push(Span::styled(
            " (default)",
            Style::default().fg(Color::DarkGray),
        ));
    }

    let path_line = Line::from(vec![Span::styled(
        format!("  {}", detected.path.display()),
        Style::default().fg(Color::DarkGray),
    )]);

    let content = vec![Line::from(name_spans), path_line];
    let mut item = ListItem::new(content);

    if is_selected {
        item = item.style(
            Style::default()
                .bg(Color::Rgb(40, 40, 40))
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

    let content = vec![
        Line::from(vec![Span::styled(
            format!(
                "Use {} and don't ask again",
                default_provider.display_name()
            ),
            Style::default()
                .fg(Color::Green)
                .add_modifier(Modifier::BOLD),
        )]),
        Line::from(vec![Span::styled(
            "  Skip this dialog in the future",
            Style::default().fg(Color::DarkGray),
        )]),
    ];

    let mut item = ListItem::new(content);

    if is_selected {
        item = item.style(
            Style::default()
                .bg(Color::Rgb(40, 40, 40))
                .add_modifier(Modifier::BOLD),
        );
    }

    item
}
