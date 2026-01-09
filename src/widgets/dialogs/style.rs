use ratatui::style::Color;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum DialogStyle {
    #[default]
    Normal,
    Danger,
    Success,
}

impl DialogStyle {
    pub const fn color(self) -> Color {
        match self {
            Self::Normal => Color::Cyan,
            Self::Danger => Color::Red,
            Self::Success => Color::Green,
        }
    }
}
