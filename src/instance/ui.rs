use super::InstanceState;
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
};

pub fn render(f: &mut Frame, state: &mut InstanceState, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(3)])
        .split(area);

    state.last_render_area = Some(chunks[0]);

    if state.panes.is_empty() {
        render_empty_state(f, chunks[0]);
    } else {
        render_panes(f, state, chunks[0]);
    }

    render_status_bar(f, state, chunks[1]);
}

fn render_empty_state(f: &mut Frame, area: Rect) {
    let block = Paragraph::new(vec![
        Line::from(""),
        Line::from(Span::styled(
            "No instance panes open",
            Style::default()
                .fg(Color::Gray)
                .add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
        Line::from(Span::styled(
            "Press 'c' to create a new instance pane",
            Style::default().fg(Color::DarkGray),
        )),
    ])
    .block(
        Block::default()
            .borders(Borders::ALL)
            .title("Instances")
            .border_style(Style::default().fg(Color::DarkGray)),
    )
    .style(Style::default())
    .alignment(ratatui::layout::Alignment::Center);

    f.render_widget(block, area);
}

fn render_panes(f: &mut Frame, state: &mut InstanceState, area: Rect) {
    let pane_areas =
        super::layout::calculate_pane_areas(area, state.layout_mode, state.panes.len());

    for (pane, pane_area) in state.panes.iter_mut().zip(pane_areas.iter()) {
        let inner_area = Block::default().borders(Borders::ALL).inner(*pane_area);

        let desired_rows = inner_area.height;
        let desired_columns = inner_area.width;

        if pane.rows != desired_rows || pane.columns != desired_columns {
            if let Some(session) = &mut pane.pty_session {
                let _ = session.resize(desired_rows, desired_columns);
            }
            pane.rows = desired_rows;
            pane.columns = desired_columns;
        }
    }

    for (index, (pane, pane_area)) in state.panes.iter().zip(pane_areas.iter()).enumerate() {
        let is_selected = index == state.selected_pane;
        let is_focused = is_selected && state.mode == super::InstanceMode::Focused;
        render_pane(f, pane, *pane_area, is_selected, is_focused, index);
    }
}

fn render_pane(
    f: &mut Frame,
    pane: &super::InstancePane,
    area: Rect,
    is_selected: bool,
    is_focused: bool,
    index: usize,
) {
    let border_color = if is_focused {
        Color::Green
    } else if is_selected {
        Color::Cyan
    } else {
        Color::DarkGray
    };

    let border_style = Style::default()
        .fg(border_color)
        .add_modifier(if is_selected {
            Modifier::BOLD
        } else {
            Modifier::empty()
        });

    let (state_indicator, indicator_color) = get_claude_state_indicator(pane.claude_state);

    let pane_name = if let Some(name) = &pane.name {
        name.clone()
    } else {
        format!("Pane {}", index + 1)
    };

    let title_prefix = if is_focused {
        "● "
    } else if is_selected {
        "→ "
    } else {
        "  "
    };

    let title_spans = vec![
        Span::styled(
            format!("{}{} ", title_prefix, pane_name),
            Style::default()
                .fg(border_color)
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled(
            state_indicator,
            Style::default()
                .fg(indicator_color)
                .add_modifier(Modifier::BOLD),
        ),
    ];

    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(border_style)
        .title(Line::from(title_spans));

    let inner_area = block.inner(area);
    f.render_widget(block, area);

    if let Some(session) = &pane.pty_session {
        render_instance_content(f, session, inner_area);
    } else {
        let message = "PTY session failed to start";
        let text = Paragraph::new(message).style(Style::default().fg(Color::Red));
        f.render_widget(text, inner_area);
    }
}

fn get_claude_state_indicator(state: super::ClaudeState) -> (&'static str, Color) {
    use std::time::{SystemTime, UNIX_EPOCH};

    const BLINK_DURATION_MS: u128 = 500;
    const BLINK_PHASES: u128 = 2;

    let should_flash = match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(duration) => (duration.as_millis() / BLINK_DURATION_MS) % BLINK_PHASES == 0,
        Err(_) => true,
    };

    match state {
        super::ClaudeState::Idle => (" ", Color::Gray),
        super::ClaudeState::Running if should_flash => ("●", Color::Rgb(255, 165, 0)),
        super::ClaudeState::Running => (" ", Color::Rgb(255, 165, 0)),
        super::ClaudeState::NeedsPermissions => ("●", Color::Rgb(138, 43, 226)),
        super::ClaudeState::Done => ("●", Color::Green),
    }
}

fn render_instance_content(f: &mut Frame, session: &super::pty::PtySession, area: Rect) {
    if let Ok(parser) = session.screen().lock() {
        let screen = parser.screen();
        let mut lines = Vec::new();

        let max_rows = area.height.min(screen.size().0);
        let max_cols = area.width.min(screen.size().1);

        for row in 0..max_rows {
            let mut line_spans = Vec::new();
            let mut current_text = String::new();
            let mut current_style = Style::default();
            let mut last_fg = vt100::Color::Default;
            let mut last_bg = vt100::Color::Default;
            let mut last_attrs = (false, false, false);

            for col in 0..max_cols {
                let cell = match screen.cell(row, col) {
                    Some(c) => c,
                    None => continue,
                };

                let fg = cell.fgcolor();
                let bg = cell.bgcolor();
                let attrs = (cell.bold(), cell.italic(), cell.underline());

                if fg != last_fg || bg != last_bg || attrs != last_attrs {
                    if !current_text.is_empty() {
                        line_spans.push(Span::styled(current_text.clone(), current_style));
                        current_text.clear();
                    }

                    current_style = Style::default()
                        .fg(convert_vt100_color(fg))
                        .bg(convert_vt100_color(bg));

                    if attrs.0 {
                        current_style = current_style.add_modifier(Modifier::BOLD);
                    }
                    if attrs.1 {
                        current_style = current_style.add_modifier(Modifier::ITALIC);
                    }
                    if attrs.2 {
                        current_style = current_style.add_modifier(Modifier::UNDERLINED);
                    }

                    last_fg = fg;
                    last_bg = bg;
                    last_attrs = attrs;
                }

                current_text.push_str(&cell.contents());
            }

            if !current_text.is_empty() {
                line_spans.push(Span::styled(current_text, current_style));
            }

            if line_spans.is_empty() {
                line_spans.push(Span::raw(""));
            }

            lines.push(Line::from(line_spans));
        }

        let text = Paragraph::new(lines);
        f.render_widget(text, area);
    }
}

fn convert_vt100_color(color: vt100::Color) -> Color {
    match color {
        vt100::Color::Default => Color::Reset,
        vt100::Color::Idx(idx) => match idx {
            0 => Color::Black,
            1 => Color::Red,
            2 => Color::Green,
            3 => Color::Yellow,
            4 => Color::Blue,
            5 => Color::Magenta,
            6 => Color::Cyan,
            7 => Color::Gray,
            8 => Color::DarkGray,
            9 => Color::LightRed,
            10 => Color::LightGreen,
            11 => Color::LightYellow,
            12 => Color::LightBlue,
            13 => Color::LightMagenta,
            14 => Color::LightCyan,
            15 => Color::White,
            _ => Color::Reset,
        },
        vt100::Color::Rgb(r, g, b) => Color::Rgb(r, g, b),
    }
}

fn render_status_bar(f: &mut Frame, state: &InstanceState, area: Rect) {
    const VERSION_TEXT: &str = "Chloe v0.1.0";
    const VERSION_TEXT_LENGTH: u16 = 13;
    const MINIMUM_SPACE_FOR_VERSION: u16 = 15;

    let (mode_text, mode_color) = match state.mode {
        super::InstanceMode::Normal => ("NAVIGATE", Color::Cyan),
        super::InstanceMode::Focused => ("FOCUSED", Color::Green),
    };

    let pane_count = state.panes.len();
    let layout_name = match state.layout_mode {
        super::LayoutMode::Single => "Single",
        super::LayoutMode::HorizontalSplit => "Horizontal",
        super::LayoutMode::VerticalSplit => "Vertical",
        super::LayoutMode::Grid => "Grid",
    };

    let help_text = match state.mode {
        super::InstanceMode::Normal => {
            if pane_count == 0 {
                "c:create"
            } else if area.width < 80 {
                "Arrows:nav  Enter:focus  c:create  x:close"
            } else {
                "Arrow-Keys:navigate  Enter:focus  c:create-pane  x:close-pane"
            }
        }
        super::InstanceMode::Focused => {
            if area.width < 80 {
                "Esc:unfocus"
            } else {
                "All keys sent to instance  Esc:back-to-navigation"
            }
        }
    };

    let inner_area = Block::default().borders(Borders::ALL).inner(area);
    let should_show_version = inner_area.width >= MINIMUM_SPACE_FOR_VERSION;

    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Min(0), Constraint::Length(VERSION_TEXT_LENGTH)])
        .split(inner_area);

    let status = Paragraph::new(Line::from(vec![
        Span::styled(
            format!("[{}] ", mode_text),
            Style::default().fg(mode_color).add_modifier(Modifier::BOLD),
        ),
        Span::styled(
            format!("Panes: {}  Layout: {}  ", pane_count, layout_name),
            Style::default().fg(Color::Gray),
        ),
        Span::styled(help_text, Style::default().fg(Color::DarkGray)),
    ]))
    .block(
        Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::DarkGray)),
    );

    f.render_widget(status, area);

    if should_show_version {
        let version = Paragraph::new(VERSION_TEXT)
            .style(Style::default().fg(Color::DarkGray))
            .alignment(ratatui::layout::Alignment::Right);
        f.render_widget(version, chunks[1]);
    }
}
