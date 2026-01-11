use super::state::{SettingItem, SettingsMode, SettingsState};
use crate::views::StatusBarContent;
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem},
};

const STATUS_BAR_WIDTH_THRESHOLD: u16 = 80;
const SETTINGS_COUNT: usize = 5;
const LABEL_WIDTH: u16 = 25;

pub fn render(frame: &mut Frame, state: &SettingsState, area: Rect) {
    let block = Block::default()
        .borders(Borders::ALL)
        .title("Settings")
        .border_style(Style::default().fg(Color::DarkGray));

    let inner = block.inner(area);
    frame.render_widget(block, area);

    let content_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0)])
        .margin(1)
        .split(inner);

    render_settings_list(frame, content_layout[0], state);
}

fn render_settings_list(frame: &mut Frame, area: Rect, state: &SettingsState) {
    let items: Vec<ListItem> = (0..SETTINGS_COUNT)
        .filter_map(|index| {
            let item = SettingItem::from_index(index)?;
            Some(create_list_item(index, item, state))
        })
        .collect();

    let list = List::new(items);
    frame.render_widget(list, area);
}

fn create_list_item(index: usize, item: SettingItem, state: &SettingsState) -> ListItem<'static> {
    let is_selected = state.selected_index == index;
    let is_editing = state.is_editing() && is_selected;

    let selector = if is_selected { "> " } else { "  " };
    let label = format!("{:width$}", item.label(), width = LABEL_WIDTH as usize);

    let value_text = get_setting_value_text(item, state, is_editing);

    let selector_style = if is_selected {
        Style::default()
            .fg(Color::Cyan)
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(Color::DarkGray)
    };

    let label_style = if is_selected {
        Style::default()
            .fg(Color::White)
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(Color::Gray)
    };

    let value_style = if is_editing {
        Style::default()
            .fg(Color::Yellow)
            .add_modifier(Modifier::UNDERLINED)
    } else if is_selected {
        Style::default().fg(Color::Cyan)
    } else {
        Style::default().fg(Color::DarkGray)
    };

    let content = Line::from(vec![
        Span::styled(selector, selector_style),
        Span::styled(label, label_style),
        Span::styled(value_text, value_style),
    ]);

    let mut list_item = ListItem::new(content);

    if is_selected {
        list_item = list_item.style(Style::default().bg(Color::Rgb(30, 30, 30)));
    }

    list_item
}

fn get_setting_value_text(item: SettingItem, state: &SettingsState, is_editing: bool) -> String {
    match item {
        SettingItem::DefaultShell => {
            if is_editing {
                format!("{}|", state.edit_buffer)
            } else {
                state.settings.default_shell.clone()
            }
        }
        SettingItem::AutoSaveInterval => {
            if is_editing {
                format!("{}| seconds", state.edit_buffer)
            } else {
                format!("{} seconds", state.settings.auto_save_interval_seconds)
            }
        }
        SettingItem::IdeCommand => state.settings.ide_command.display_name().to_string(),
        SettingItem::TerminalCommand => state.settings.terminal_command.display_name().to_string(),
        SettingItem::DefaultProvider => state.settings.default_provider.display_name().to_string(),
    }
}

#[must_use]
pub fn get_status_bar_content(state: &SettingsState, width: u16) -> StatusBarContent {
    let mode_color = if state.is_editing() {
        Color::Yellow
    } else {
        Color::White
    };

    let mode_text = if state.is_editing() {
        "EDITING"
    } else {
        "NORMAL"
    };

    let help_text = if width < STATUS_BAR_WIDTH_THRESHOLD {
        if state.is_editing() {
            "Enter:confirm  Esc:cancel"
        } else {
            "jk:navigate  Enter:edit"
        }
    } else if state.is_editing() {
        match state.mode {
            SettingsMode::EditingShell => "Enter:confirm  Esc:cancel  Type to edit shell path",
            SettingsMode::EditingAutoSave => {
                "Enter:confirm  Esc:cancel  Type numbers to set interval"
            }
            SettingsMode::Normal => "Enter:confirm  Esc:cancel",
        }
    } else {
        "jk/arrows:navigate  Enter/Space:edit  Tab:switch-tabs  q:quit"
    };

    StatusBarContent {
        mode_text: mode_text.to_string(),
        mode_color,
        extra_info: None,
        help_text: help_text.to_string(),
    }
}
