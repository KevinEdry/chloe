use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, Padding, Paragraph, Wrap},
};

const DIALOG_WIDTH_THRESHOLD: u16 = 80;
const DIALOG_WIDTH_SMALL: u16 = 80;
const DIALOG_WIDTH_NORMAL: u16 = 60;
const DIALOG_HEIGHT: u16 = 7;

pub fn render_input_dialog(frame: &mut Frame, title: &str, input: &str, area: Rect) {
    let dialog_width = if area.width < DIALOG_WIDTH_THRESHOLD {
        DIALOG_WIDTH_SMALL
    } else {
        DIALOG_WIDTH_NORMAL
    };

    let popup_area = centered_rect(dialog_width, DIALOG_HEIGHT, area);

    let block = Block::default()
        .title(title)
        .title_style(
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Cyan))
        .padding(Padding::uniform(1));

    let inner_area = block.inner(popup_area);

    frame.render_widget(Clear, popup_area);
    frame.render_widget(block, popup_area);

    let input_text = Paragraph::new(input)
        .style(Style::default().fg(Color::White))
        .wrap(Wrap { trim: false });

    frame.render_widget(input_text, inner_area);
}

pub fn render_confirm_dialog(frame: &mut Frame, title: &str, message: &str, area: Rect) {
    let dialog_width = if area.width < DIALOG_WIDTH_THRESHOLD {
        DIALOG_WIDTH_SMALL
    } else {
        DIALOG_WIDTH_NORMAL
    };

    let popup_area = centered_rect(dialog_width, DIALOG_HEIGHT, area);

    let block = Block::default()
        .title(title)
        .title_style(
            Style::default()
                .fg(Color::Red)
                .add_modifier(Modifier::BOLD),
        )
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Red))
        .padding(Padding::uniform(1));

    let inner_area = block.inner(popup_area);

    frame.render_widget(Clear, popup_area);
    frame.render_widget(block, popup_area);

    let text = Paragraph::new(vec![
        Line::from(""),
        Line::from(Span::styled(message, Style::default().fg(Color::White))),
    ])
    .alignment(Alignment::Center);

    frame.render_widget(text, inner_area);
}

fn centered_rect(width: u16, height: u16, area: Rect) -> Rect {
    let vertical_margin = area.height.saturating_sub(height) / 2;
    let horizontal_margin = area.width.saturating_sub(width) / 2;

    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(vertical_margin),
            Constraint::Length(height),
            Constraint::Min(0),
        ])
        .split(area);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Length(horizontal_margin),
            Constraint::Length(width),
            Constraint::Min(0),
        ])
        .split(popup_layout[1])[1]
}
