use crate::helpers::text;
use crate::views::instances::ClaudeState;
use crate::views::tasks::TaskType;
use crate::widgets::{claude_indicator, spinner};
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
    is_classifying: bool,
    claude_state: Option<ClaudeState>,
    title_max_length: usize,
    selection_color: Color,
    badge_color_override: Option<Color>,
    spinner_frame: usize,
}

impl TaskItem {
    #[must_use]
    pub fn new(title: impl Into<String>, task_type: TaskType) -> Self {
        Self {
            title: title.into(),
            task_type,
            is_selected: false,
            is_classifying: false,
            claude_state: None,
            title_max_length: DEFAULT_TITLE_MAX_LENGTH,
            selection_color: Color::Cyan,
            badge_color_override: None,
            spinner_frame: 0,
        }
    }

    #[must_use]
    pub const fn selected(mut self, is_selected: bool) -> Self {
        self.is_selected = is_selected;
        self
    }

    #[must_use]
    pub const fn classifying(mut self, is_classifying: bool) -> Self {
        self.is_classifying = is_classifying;
        self
    }

    #[must_use]
    pub const fn claude_state(mut self, state: Option<ClaudeState>) -> Self {
        self.claude_state = state;
        self
    }

    #[must_use]
    pub const fn selection_color(mut self, color: Color) -> Self {
        self.selection_color = color;
        self
    }

    #[must_use]
    pub const fn badge_color(mut self, color: Color) -> Self {
        self.badge_color_override = Some(color);
        self
    }

    #[must_use]
    pub const fn title_max_length(mut self, length: usize) -> Self {
        self.title_max_length = length;
        self
    }

    #[must_use]
    pub const fn spinner_frame(mut self, frame: usize) -> Self {
        self.spinner_frame = frame;
        self
    }

    #[must_use]
    pub fn build(self) -> ListItem<'static> {
        let truncated_title = text::truncate(&self.title, self.title_max_length);

        let badge_text = self.task_type.badge_text();
        let badge_color = if self.is_classifying {
            Color::Yellow
        } else {
            self.badge_color_override
                .unwrap_or_else(|| self.task_type.color())
        };

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

        let mut spans = vec![self.build_selection_indicator()];

        if self.is_classifying {
            spans.push(spinner::spinner_span(self.spinner_frame, "Planning..."));
        } else {
            spans.push(Span::styled(
                format!("[{badge_text}]"),
                Style::default().fg(badge_color),
            ));
        }
        spans.push(Span::raw(" "));

        spans.push(Span::styled(
            truncated_title,
            Style::default()
                .fg(title_color)
                .add_modifier(title_modifier),
        ));

        if !self.is_classifying
            && let Some(state) = self.claude_state
        {
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
