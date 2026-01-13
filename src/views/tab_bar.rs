use crate::app::{App, Tab};
use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
};

const TAB_COLORS: [Color; 6] = [
    Color::LightBlue, // Tasks
    Color::Yellow,    // Instances
    Color::Magenta,   // Roadmap
    Color::Green,     // Worktree
    Color::Cyan,      // PullRequests
    Color::White,     // Settings
];

const TAB_COLORS_INACTIVE: [Color; 6] = [
    Color::DarkGray,
    Color::DarkGray,
    Color::DarkGray,
    Color::DarkGray,
    Color::DarkGray,
    Color::DarkGray,
];

fn get_tab_name(tab_index: usize, vcs_command: &crate::views::settings::VcsCommand) -> String {
    match tab_index {
        0 => "Tasks".to_string(),
        1 => "Instances".to_string(),
        2 => "Roadmap".to_string(),
        3 => vcs_command.workspace_term().to_string(),
        4 => "PRs".to_string(),
        5 => "Settings".to_string(),
        _ => "Unknown".to_string(),
    }
}

const DIRECTORY_PADDING: u16 = 2;

pub fn render(frame: &mut Frame, app: &App, area: Rect) {
    let current_directory = std::env::current_dir()
        .ok()
        .and_then(|path| path.to_str().map(String::from))
        .unwrap_or_else(|| String::from("?"));

    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::DarkGray));
    let inner = block.inner(area);
    frame.render_widget(block, area);

    let directory_width =
        u16::try_from(current_directory.len()).unwrap_or(u16::MAX) + DIRECTORY_PADDING;

    let layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Min(0), Constraint::Length(directory_width)])
        .split(inner);

    let selected_index = match app.active_tab {
        Tab::Tasks => 0,
        Tab::Instances => 1,
        Tab::Roadmap => 2,
        Tab::Worktree => 3,
        Tab::PullRequests => 4,
        Tab::Settings => 5,
    };

    let vcs_command = &app.settings.settings.vcs_command;
    let tab_spans: Vec<Span> = (0..6)
        .flat_map(|index| {
            let is_selected = index == selected_index;
            let tab_number = index + 1;
            let name = get_tab_name(index, vcs_command);
            let color = if is_selected {
                TAB_COLORS[index]
            } else {
                TAB_COLORS_INACTIVE[index]
            };

            let tab_style = if is_selected {
                Style::default()
                    .fg(Color::Black)
                    .bg(color)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(color)
            };

            vec![Span::styled(format!(" {tab_number}:{name} "), tab_style)]
        })
        .collect();

    let tabs_line = Paragraph::new(Line::from(tab_spans));

    let directory_display = Paragraph::new(Line::from(vec![
        Span::styled(" ", Style::default()),
        Span::styled(
            current_directory,
            Style::default()
                .fg(Color::DarkGray)
                .add_modifier(Modifier::ITALIC),
        ),
    ]))
    .alignment(Alignment::Right);

    frame.render_widget(tabs_line, layout[0]);
    frame.render_widget(directory_display, layout[1]);
}
