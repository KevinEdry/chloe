use crate::helpers::text;
use crate::views::instances::ClaudeState;
use crate::views::tasks::TaskType;
use crate::widgets::claude_indicator;
use ratatui::{
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::ListItem,
};

const DEFAULT_TITLE_MAX_LENGTH: usize = 25;

pub struct TaskItem {
    title: String,
    task_type: TaskType,
    is_selected: bool,
    claude_state: Option<ClaudeState>,
    title_max_length: usize,
    selection_color: Color,
    badge_color_override: Option<Color>,
}

impl TaskItem {
    pub fn new(title: impl Into<String>, task_type: TaskType) -> Self {
        Self {
            title: title.into(),
            task_type,
            is_selected: false,
            claude_state: None,
            title_max_length: DEFAULT_TITLE_MAX_LENGTH,
            selection_color: Color::Cyan,
            badge_color_override: None,
        }
    }

    pub const fn selected(mut self, is_selected: bool) -> Self {
        self.is_selected = is_selected;
        self
    }

    pub const fn claude_state(mut self, state: Option<ClaudeState>) -> Self {
        self.claude_state = state;
        self
    }

    pub const fn selection_color(mut self, color: Color) -> Self {
        self.selection_color = color;
        self
    }

    pub const fn badge_color(mut self, color: Color) -> Self {
        self.badge_color_override = Some(color);
        self
    }

    pub fn build(self) -> ListItem<'static> {
        let truncated_title = text::truncate(&self.title, self.title_max_length);

        let badge_text = self.task_type.badge_text();
        let badge_color = self
            .badge_color_override
            .unwrap_or_else(|| self.task_type.color());

        let title_color = if self.is_selected {
            Color::White
        } else {
            Color::Gray
        };

        let title_modifier = if self.is_selected {
            Modifier::BOLD
        } else {
            Modifier::empty()
        };

        let mut spans = vec![
            self.build_selection_indicator(),
            Span::styled(format!("[{badge_text}]"), Style::default().fg(badge_color)),
            Span::raw(" "),
            Span::styled(
                truncated_title,
                Style::default()
                    .fg(title_color)
                    .add_modifier(title_modifier),
            ),
        ];

        if let Some(state) = self.claude_state {
            let (indicator, color) = claude_indicator::dot_visible(state);
            if !indicator.is_empty() {
                spans.push(Span::raw(" "));
                spans.push(Span::styled(
                    indicator.to_string(),
                    Style::default().fg(color),
                ));
            }
        }

        ListItem::new(Line::from(spans))
    }

    fn build_selection_indicator(&self) -> Span<'static> {
        if self.is_selected {
            Span::styled("▶ ", Style::default().fg(self.selection_color))
        } else {
            Span::raw("  ")
        }
    }
}
