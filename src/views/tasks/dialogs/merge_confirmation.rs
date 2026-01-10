use super::{centered_rect, render_popup_background};
use crate::views::tasks::state::MergeTarget;
use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
};

const DIALOG_WIDTH_PERCENT: u16 = 50;
const DIALOG_HEIGHT_PERCENT: u16 = 30;

pub fn render_merge_confirmation(
    frame: &mut Frame,
    worktree_branch: &str,
    selected_target: &MergeTarget,
    area: Rect,
) {
    let dialog_area = centered_rect(DIALOG_WIDTH_PERCENT, DIALOG_HEIGHT_PERCENT, area);

    render_popup_background(frame, dialog_area);

    let block = Block::default()
        .title(Span::styled(
            " Merge Confirmation ",
            Style::default()
                .fg(Color::Green)
                .add_modifier(Modifier::BOLD),
        ))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Green));

    let inner_area = block.inner(dialog_area);
    frame.render_widget(block, dialog_area);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(2),
            Constraint::Length(1),
            Constraint::Min(0),
            Constraint::Length(2),
        ])
        .split(inner_area);

    let header = Paragraph::new(Line::from(vec![
        Span::styled("Merge ", Style::default().fg(Color::White)),
        Span::styled(
            format!("'{worktree_branch}'"),
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled(" into:", Style::default().fg(Color::White)),
    ]))
    .alignment(Alignment::Center);

    frame.render_widget(header, chunks[0]);

    let current_branch_name = match selected_target {
        MergeTarget::CurrentBranch(name) => Some(name.as_str()),
        MergeTarget::MainBranch => None,
    };

    let current_option_label = current_branch_name.map(|name| format!("{name} (current)"));

    let mut options = Vec::new();

    if let Some(label) = &current_option_label {
        let is_selected = matches!(selected_target, MergeTarget::CurrentBranch(_));
        options.push(render_option(label, is_selected));
    }

    let main_selected = matches!(selected_target, MergeTarget::MainBranch);
    options.push(render_option("main", main_selected));

    let options_area = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Constraint::Length(1); options.len()])
        .split(chunks[2]);

    for (index, option) in options.into_iter().enumerate() {
        if index < options_area.len() {
            frame.render_widget(option, options_area[index]);
        }
    }

    let footer = Paragraph::new(Line::from(Span::styled(
        "j/k: select  Enter: merge  Esc: cancel",
        Style::default().fg(Color::DarkGray),
    )))
    .alignment(Alignment::Center);

    frame.render_widget(footer, chunks[3]);
}

fn render_option(label: &str, is_selected: bool) -> Paragraph<'_> {
    let (prefix, style) = if is_selected {
        (
            "> ",
            Style::default()
                .fg(Color::Green)
                .add_modifier(Modifier::BOLD),
        )
    } else {
        ("  ", Style::default().fg(Color::DarkGray))
    };

    Paragraph::new(Line::from(vec![
        Span::styled(prefix, style),
        Span::styled(label, style),
    ]))
    .alignment(Alignment::Center)
}
