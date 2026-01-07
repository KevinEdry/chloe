use super::TerminalState;
use ratatui::{Frame, widgets::{Block, Borders, Paragraph}, style::{Color, Style}};

pub fn render(f: &mut Frame, _state: &TerminalState) {
    let area = f.area();

    let block = Paragraph::new("Terminals Tab - Coming Soon")
        .block(Block::default().borders(Borders::ALL).title("Terminals"))
        .style(Style::default().fg(Color::Green));

    f.render_widget(block, area);
}
