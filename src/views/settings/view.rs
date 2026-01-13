use super::state::{
    IdeCommand, SettingItem, SettingsFocus, SettingsMode, SettingsSection, SettingsState,
    TerminalCommand,
};
use crate::views::StatusBarContent;
use crate::views::tasks::dialogs::{
    ProviderSelectionViewState, centered_rect, render_popup_background, render_provider_selection,
};
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Margin, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, List, ListItem, Padding, Paragraph, Wrap},
};

const SIDEBAR_WIDTH_PERCENT: u16 = 30;
const SIDEBAR_MIN_WIDTH: u16 = 25;
const SIDEBAR_MAX_WIDTH: u16 = 40;
const STATUS_BAR_WIDTH_THRESHOLD: u16 = 80;
const SELECTION_POPUP_WIDTH_PERCENT: u16 = 40;
const SELECTION_POPUP_HEIGHT_PERCENT: u16 = 40;

pub fn render(frame: &mut Frame, state: &SettingsState, area: Rect) {
    let layout = create_main_layout(area);
    let sidebar_area = layout[0];
    let content_area = layout[1];

    render_sidebar(frame, sidebar_area, state);
    render_content_panel(frame, content_area, state);
    render_dialogs(frame, area, state);
}

fn create_main_layout(area: Rect) -> std::rc::Rc<[Rect]> {
    let sidebar_width = calculate_sidebar_width(area.width);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Length(sidebar_width), Constraint::Min(0)])
        .split(area)
}

fn calculate_sidebar_width(total_width: u16) -> u16 {
    let percent_width = total_width * SIDEBAR_WIDTH_PERCENT / 100;
    percent_width.clamp(SIDEBAR_MIN_WIDTH, SIDEBAR_MAX_WIDTH)
}

fn render_sidebar(frame: &mut Frame, area: Rect, state: &SettingsState) {
    let is_focused = state.focus == SettingsFocus::Sidebar;
    let border_color = if is_focused {
        Color::Cyan
    } else {
        Color::DarkGray
    };

    let block = Block::default()
        .borders(Borders::ALL)
        .title(" Categories ")
        .border_style(Style::default().fg(border_color))
        .padding(Padding::new(1, 1, 1, 0));

    let inner_area = block.inner(area);
    frame.render_widget(block, area);

    let items: Vec<ListItem> = SettingsSection::ALL
        .iter()
        .enumerate()
        .map(|(index, section)| render_sidebar_item(*section, index, state))
        .collect();

    let list = List::new(items);
    frame.render_widget(list, inner_area);
}

fn render_sidebar_item(
    section: SettingsSection,
    index: usize,
    state: &SettingsState,
) -> ListItem<'static> {
    let is_selected = index == state.selected_section;
    let is_sidebar_focused = state.focus == SettingsFocus::Sidebar;
    let is_active = is_selected && is_sidebar_focused;

    let icon_style = if is_active {
        Style::default()
            .fg(Color::Cyan)
            .add_modifier(Modifier::BOLD)
    } else if is_selected {
        Style::default().fg(Color::Cyan)
    } else {
        Style::default().fg(Color::DarkGray)
    };

    let title_style = if is_active {
        Style::default()
            .fg(Color::White)
            .add_modifier(Modifier::BOLD)
    } else if is_selected {
        Style::default().fg(Color::White)
    } else {
        Style::default().fg(Color::Gray)
    };

    let line = Line::from(vec![
        Span::styled(format!(" {} ", section.icon()), icon_style),
        Span::styled(section.title().to_string(), title_style),
    ]);

    let mut item = ListItem::new(vec![line, Line::from("")]);

    if is_selected {
        let background_color = if is_sidebar_focused {
            Color::Rgb(40, 40, 50)
        } else {
            Color::Rgb(30, 30, 35)
        };
        item = item.style(Style::default().bg(background_color));
    }

    item
}

fn render_content_panel(frame: &mut Frame, area: Rect, state: &SettingsState) {
    let is_focused = state.focus == SettingsFocus::Content;
    let border_color = if is_focused {
        Color::Cyan
    } else {
        Color::DarkGray
    };

    let Some(section) = SettingsSection::from_index(state.selected_section) else {
        return;
    };

    let block = Block::default()
        .borders(Borders::ALL)
        .title(format!(" {} ", section.title()))
        .border_style(Style::default().fg(border_color))
        .padding(Padding::new(2, 2, 1, 1));

    let inner_area = block.inner(area);
    frame.render_widget(block, area);

    render_section_description(frame, inner_area, section);
    render_section_items(frame, inner_area, state, section);
}

fn render_section_description(frame: &mut Frame, area: Rect, section: SettingsSection) {
    let description = Paragraph::new(section.description())
        .style(Style::default().fg(Color::DarkGray))
        .wrap(Wrap { trim: true });

    let description_area = Rect {
        x: area.x,
        y: area.y,
        width: area.width,
        height: 2,
    };

    frame.render_widget(description, description_area);
}

fn render_section_items(
    frame: &mut Frame,
    area: Rect,
    state: &SettingsState,
    section: SettingsSection,
) {
    let items = section.items();
    let items_start_y = area.y + 3;

    for (index, &item) in items.iter().enumerate() {
        let is_selected =
            state.focus == SettingsFocus::Content && index == state.selected_item_in_section;

        let item_area = Rect {
            x: area.x,
            y: items_start_y.saturating_add(u16::try_from(index).unwrap_or(0).saturating_mul(3)),
            width: area.width,
            height: 3,
        };

        render_setting_item(frame, item_area, item, state, is_selected);
    }
}

fn render_setting_item(
    frame: &mut Frame,
    area: Rect,
    item: SettingItem,
    state: &SettingsState,
    is_selected: bool,
) {
    let label = item.label();
    let value = get_setting_value_text(item, state);
    let item_type = get_item_type_indicator(item);

    let label_style = if is_selected {
        Style::default()
            .fg(Color::White)
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(Color::Gray)
    };

    let value_style = if is_selected {
        Style::default().fg(Color::Cyan)
    } else {
        Style::default().fg(Color::DarkGray)
    };

    let indicator_style = Style::default().fg(Color::DarkGray);

    let selector = if is_selected { "> " } else { "  " };
    let selector_style = if is_selected {
        Style::default()
            .fg(Color::Cyan)
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(Color::DarkGray)
    };

    let line = Line::from(vec![
        Span::styled(selector, selector_style),
        Span::styled(label.to_string(), label_style),
    ]);

    let value_line = Line::from(vec![
        Span::styled("   ", Style::default()),
        Span::styled(value, value_style),
        Span::styled(format!(" {item_type}"), indicator_style),
    ]);

    let content = vec![line, value_line];
    let mut paragraph = Paragraph::new(content);

    if is_selected {
        paragraph = paragraph.style(Style::default().bg(Color::Rgb(35, 35, 45)));
    }

    frame.render_widget(paragraph, area);
}

const fn get_item_type_indicator(item: SettingItem) -> &'static str {
    match item {
        SettingItem::DefaultShell => "[text]",
        SettingItem::AutoSaveInterval => "[number]",
        SettingItem::IdeCommand | SettingItem::TerminalCommand | SettingItem::DefaultProvider => {
            "[select]"
        }
    }
}

fn get_setting_value_text(item: SettingItem, state: &SettingsState) -> String {
    match item {
        SettingItem::DefaultShell => state.settings.default_shell.clone(),
        SettingItem::AutoSaveInterval => {
            format!("{} seconds", state.settings.auto_save_interval_seconds)
        }
        SettingItem::IdeCommand => state.settings.ide_command.display_name().to_string(),
        SettingItem::TerminalCommand => state.settings.terminal_command.display_name().to_string(),
        SettingItem::DefaultProvider => state.settings.default_provider.display_name().to_string(),
    }
}

fn render_dialogs(frame: &mut Frame, area: Rect, state: &SettingsState) {
    match state.mode {
        SettingsMode::SelectingProvider { selected_index } => {
            let dialog_state = ProviderSelectionViewState {
                selected_index,
                default_provider: state.settings.default_provider,
                detected_providers: &state.detected_providers,
            };
            render_provider_selection(frame, &dialog_state, area);
        }
        SettingsMode::SelectingIde { selected_index } => {
            render_ide_selection_dialog(frame, area, state, selected_index);
        }
        SettingsMode::SelectingTerminal { selected_index } => {
            render_terminal_selection_dialog(frame, area, state, selected_index);
        }
        SettingsMode::EditingShell { .. } => {
            render_text_input_dialog(frame, area, "Edit Default Shell", &state.edit_buffer);
        }
        SettingsMode::EditingAutoSave { .. } => {
            render_text_input_dialog(
                frame,
                area,
                "Edit Auto-save Interval (seconds)",
                &state.edit_buffer,
            );
        }
        SettingsMode::Normal => {}
    }
}

fn render_ide_selection_dialog(
    frame: &mut Frame,
    area: Rect,
    state: &SettingsState,
    selected_index: usize,
) {
    let popup_area = centered_rect(
        SELECTION_POPUP_WIDTH_PERCENT,
        SELECTION_POPUP_HEIGHT_PERCENT,
        area,
    );
    render_popup_background(frame, popup_area);

    let block = Block::default()
        .title(" Select IDE ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Cyan))
        .padding(Padding::uniform(1));

    let inner_area = block.inner(popup_area);
    frame.render_widget(block, popup_area);

    let options = [
        ("Cursor", "AI-native code editor"),
        ("VS Code", "Visual Studio Code"),
        ("WebStorm", "JetBrains WebStorm"),
    ];

    let current_ide = &state.settings.ide_command;
    let items: Vec<ListItem> = options
        .iter()
        .enumerate()
        .map(|(index, (name, description))| {
            render_selection_option(
                name,
                description,
                index,
                selected_index,
                is_current_ide(current_ide, index),
            )
        })
        .collect();

    let list = List::new(items);
    frame.render_widget(list, inner_area);
}

const fn is_current_ide(command: &IdeCommand, index: usize) -> bool {
    matches!(
        (command, index),
        (IdeCommand::Cursor, 0) | (IdeCommand::VSCode, 1) | (IdeCommand::WebStorm, 2)
    )
}

fn render_terminal_selection_dialog(
    frame: &mut Frame,
    area: Rect,
    state: &SettingsState,
    selected_index: usize,
) {
    let popup_area = centered_rect(
        SELECTION_POPUP_WIDTH_PERCENT,
        SELECTION_POPUP_HEIGHT_PERCENT,
        area,
    );
    render_popup_background(frame, popup_area);

    let block = Block::default()
        .title(" Select Terminal ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Cyan))
        .padding(Padding::uniform(1));

    let inner_area = block.inner(popup_area);
    frame.render_widget(block, popup_area);

    let options = [
        ("Apple Terminal", "macOS built-in terminal"),
        ("iTerm2", "Popular macOS terminal replacement"),
    ];

    let current_terminal = &state.settings.terminal_command;
    let items: Vec<ListItem> = options
        .iter()
        .enumerate()
        .map(|(index, (name, description))| {
            render_selection_option(
                name,
                description,
                index,
                selected_index,
                is_current_terminal(current_terminal, index),
            )
        })
        .collect();

    let list = List::new(items);
    frame.render_widget(list, inner_area);
}

const fn is_current_terminal(command: &TerminalCommand, index: usize) -> bool {
    matches!(
        (command, index),
        (TerminalCommand::AppleTerminal, 0) | (TerminalCommand::ITerm2, 1)
    )
}

fn render_selection_option(
    name: &str,
    description: &str,
    index: usize,
    selected_index: usize,
    is_current: bool,
) -> ListItem<'static> {
    let is_selected = index == selected_index;

    let mut name_spans = vec![Span::styled(
        name.to_string(),
        Style::default()
            .fg(Color::White)
            .add_modifier(Modifier::BOLD),
    )];

    if is_current {
        name_spans.push(Span::styled(
            " (current)",
            Style::default().fg(Color::Green),
        ));
    }

    let description_line = Line::from(vec![Span::styled(
        format!("  {description}"),
        Style::default().fg(Color::DarkGray),
    )]);

    let content = vec![Line::from(name_spans), description_line];
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

fn render_text_input_dialog(frame: &mut Frame, area: Rect, title: &str, value: &str) {
    let dialog_width = 60;
    let dialog_height = 7;

    let dialog_area = Rect {
        x: area.width.saturating_sub(dialog_width) / 2,
        y: area.height.saturating_sub(dialog_height) / 2,
        width: dialog_width,
        height: dialog_height,
    };

    frame.render_widget(Clear, dialog_area);

    let dialog_block = Block::default()
        .title(title)
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Cyan));

    let inner_area = dialog_block.inner(dialog_area);
    frame.render_widget(dialog_block, dialog_area);

    let input_area = Rect {
        x: inner_area.x,
        y: inner_area.y,
        width: inner_area.width,
        height: 3,
    };

    let input_block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Gray));

    let input_text = format!("{value}|");
    let input_paragraph = Paragraph::new(input_text)
        .style(Style::default().fg(Color::Yellow))
        .wrap(Wrap { trim: false });

    frame.render_widget(input_block, input_area);
    frame.render_widget(
        input_paragraph,
        input_area.inner(Margin {
            vertical: 0,
            horizontal: 1,
        }),
    );

    let help_text = "Enter: confirm  Esc: cancel";
    let help_area = Rect {
        x: inner_area.x,
        y: inner_area.y + 4,
        width: inner_area.width,
        height: 1,
    };

    let help_paragraph = Paragraph::new(help_text)
        .style(Style::default().fg(Color::DarkGray))
        .alignment(ratatui::layout::Alignment::Center);

    frame.render_widget(help_paragraph, help_area);
}

#[must_use]
pub fn get_status_bar_content(state: &SettingsState, width: u16) -> StatusBarContent {
    let is_in_dialog = !matches!(state.mode, SettingsMode::Normal);

    let mode_color = if is_in_dialog {
        Color::Yellow
    } else {
        Color::White
    };

    let mode_text = match state.mode {
        SettingsMode::Normal => {
            if state.focus == SettingsFocus::Sidebar {
                "SIDEBAR"
            } else {
                "CONTENT"
            }
        }
        SettingsMode::EditingShell { .. } | SettingsMode::EditingAutoSave { .. } => "EDITING",
        SettingsMode::SelectingProvider { .. }
        | SettingsMode::SelectingIde { .. }
        | SettingsMode::SelectingTerminal { .. } => "SELECT",
    };

    let help_text = if width < STATUS_BAR_WIDTH_THRESHOLD {
        match state.mode {
            SettingsMode::Normal => "jk:nav  Tab:focus  Enter:edit",
            SettingsMode::SelectingProvider { .. }
            | SettingsMode::SelectingIde { .. }
            | SettingsMode::SelectingTerminal { .. } => "jk:navigate  Enter:select  Esc:cancel",
            _ => "Enter:confirm  Esc:cancel",
        }
    } else {
        match state.mode {
            SettingsMode::EditingShell { .. } => {
                "Enter: confirm  Esc: cancel  Type to edit shell path"
            }
            SettingsMode::EditingAutoSave { .. } => {
                "Enter: confirm  Esc: cancel  Type numbers to set interval"
            }
            SettingsMode::SelectingProvider { .. }
            | SettingsMode::SelectingIde { .. }
            | SettingsMode::SelectingTerminal { .. } => {
                "jk/arrows: navigate  Enter: select  Esc: cancel"
            }
            SettingsMode::Normal => {
                "jk: navigate  Tab: switch focus  Enter/Space: edit  h/l: focus sidebar/content"
            }
        }
    };

    StatusBarContent {
        mode_text: mode_text.to_string(),
        mode_color,
        extra_info: None,
        help_text: help_text.to_string(),
    }
}
