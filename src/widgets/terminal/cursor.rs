use ratatui::style::{Color, Modifier, Style};

#[derive(Debug, Clone)]
pub struct Cursor {
    visibility: bool,
    symbol: String,
    style: Style,
    blinking: bool,
}

impl Default for Cursor {
    fn default() -> Self {
        Self {
            visibility: true,
            symbol: "▌".to_string(),
            style: Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::SLOW_BLINK),
            blinking: true,
        }
    }
}

impl Cursor {
    pub const fn visibility(mut self, visibility: bool) -> Self {
        self.visibility = visibility;
        self
    }

    pub const fn is_visible(&self) -> bool {
        self.visibility
    }

    pub fn get_symbol(&self) -> &str {
        if self.symbol.is_empty() {
            "▌"
        } else {
            &self.symbol
        }
    }

    pub fn get_style(&self) -> Style {
        if self.blinking {
            self.style.add_modifier(Modifier::SLOW_BLINK)
        } else {
            self.style
        }
    }
}
