use serde::{Deserialize, Serialize};

const SETTINGS_COUNT: usize = 4;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Settings {
    pub default_shell: String,
    pub auto_save_interval_seconds: u64,
    pub ide_command: IdeCommand,
    pub terminal_command: TerminalCommand,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum IdeCommand {
    Cursor,
    VSCode,
    WebStorm,
    Custom(String),
}

impl IdeCommand {
    #[must_use]
    pub fn detect() -> Self {
        if std::process::Command::new("cursor")
            .arg("--version")
            .output()
            .is_ok()
        {
            return Self::Cursor;
        }

        if std::process::Command::new("code")
            .arg("--version")
            .output()
            .is_ok()
        {
            return Self::VSCode;
        }

        if std::process::Command::new("webstorm")
            .arg("--version")
            .output()
            .is_ok()
        {
            return Self::WebStorm;
        }

        Self::Cursor
    }

    #[must_use]
    pub fn command_name(&self) -> &str {
        match self {
            Self::Cursor => "cursor",
            Self::VSCode => "code",
            Self::WebStorm => "webstorm",
            Self::Custom(command) => command,
        }
    }

    #[must_use]
    pub fn display_name(&self) -> &str {
        match self {
            Self::Cursor => "Cursor",
            Self::VSCode => "VS Code",
            Self::WebStorm => "WebStorm",
            Self::Custom(command) => command,
        }
    }

    #[must_use]
    pub const fn cycle_next(&self) -> Self {
        match self {
            Self::Cursor => Self::VSCode,
            Self::VSCode => Self::WebStorm,
            Self::WebStorm | Self::Custom(_) => Self::Cursor,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TerminalCommand {
    AppleTerminal,
    ITerm2,
    Custom(String),
}

impl TerminalCommand {
    #[must_use]
    pub const fn detect() -> Self {
        Self::AppleTerminal
    }

    pub fn open_at_path(&self, path: &std::path::Path) -> std::io::Result<()> {
        match self {
            Self::AppleTerminal => {
                let script = format!(
                    "tell application \"Terminal\"\n    \
                     do script \"cd '{}'\"\n    \
                     activate\n\
                     end tell",
                    path.display()
                );
                std::process::Command::new("osascript")
                    .arg("-e")
                    .arg(script)
                    .spawn()?;
            }
            Self::ITerm2 => {
                let script = format!(
                    "tell application \"iTerm\"\n    \
                     create window with default profile\n    \
                     tell current session of current window\n        \
                     write text \"cd '{}'\"\n    \
                     end tell\n    \
                     activate\n\
                     end tell",
                    path.display()
                );
                std::process::Command::new("osascript")
                    .arg("-e")
                    .arg(script)
                    .spawn()?;
            }
            Self::Custom(command) => {
                std::process::Command::new(command).arg(path).spawn()?;
            }
        }
        Ok(())
    }

    #[must_use]
    pub fn display_name(&self) -> &str {
        match self {
            Self::AppleTerminal => "Apple Terminal",
            Self::ITerm2 => "iTerm2",
            Self::Custom(command) => command,
        }
    }

    #[must_use]
    pub const fn cycle_next(&self) -> Self {
        match self {
            Self::AppleTerminal => Self::ITerm2,
            Self::ITerm2 | Self::Custom(_) => Self::AppleTerminal,
        }
    }
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            default_shell: std::env::var("SHELL").unwrap_or_else(|_| "/bin/sh".to_string()),
            auto_save_interval_seconds: 30,
            ide_command: IdeCommand::detect(),
            terminal_command: TerminalCommand::detect(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SettingsMode {
    #[default]
    Normal,
    EditingShell,
    EditingAutoSave,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SettingItem {
    DefaultShell,
    AutoSaveInterval,
    IdeCommand,
    TerminalCommand,
}

impl SettingItem {
    #[must_use]
    pub const fn from_index(index: usize) -> Option<Self> {
        match index {
            0 => Some(Self::DefaultShell),
            1 => Some(Self::AutoSaveInterval),
            2 => Some(Self::IdeCommand),
            3 => Some(Self::TerminalCommand),
            _ => None,
        }
    }

    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::DefaultShell => "Default Shell",
            Self::AutoSaveInterval => "Auto-save Interval",
            Self::IdeCommand => "IDE Command",
            Self::TerminalCommand => "Terminal",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SettingsState {
    pub settings: Settings,
    #[serde(skip)]
    pub selected_index: usize,
    #[serde(skip)]
    pub mode: SettingsMode,
    #[serde(skip)]
    pub edit_buffer: String,
}

impl SettingsState {
    #[must_use]
    pub fn new() -> Self {
        Self {
            settings: Settings::default(),
            selected_index: 0,
            mode: SettingsMode::Normal,
            edit_buffer: String::new(),
        }
    }

    #[must_use]
    pub const fn with_settings(settings: Settings) -> Self {
        Self {
            settings,
            selected_index: 0,
            mode: SettingsMode::Normal,
            edit_buffer: String::new(),
        }
    }

    pub fn select_next(&mut self) {
        self.selected_index = (self.selected_index + 1).min(SETTINGS_COUNT - 1);
    }

    pub const fn select_previous(&mut self) {
        self.selected_index = self.selected_index.saturating_sub(1);
    }

    pub const fn select_first(&mut self) {
        self.selected_index = 0;
    }

    pub const fn select_last(&mut self) {
        self.selected_index = SETTINGS_COUNT - 1;
    }

    #[must_use]
    pub const fn selected_item(&self) -> Option<SettingItem> {
        SettingItem::from_index(self.selected_index)
    }

    pub fn start_editing(&mut self) {
        let Some(item) = self.selected_item() else {
            return;
        };

        match item {
            SettingItem::DefaultShell => {
                self.edit_buffer = self.settings.default_shell.clone();
                self.mode = SettingsMode::EditingShell;
            }
            SettingItem::AutoSaveInterval => {
                self.edit_buffer = self.settings.auto_save_interval_seconds.to_string();
                self.mode = SettingsMode::EditingAutoSave;
            }
            SettingItem::IdeCommand => {
                self.settings.ide_command = self.settings.ide_command.cycle_next();
            }
            SettingItem::TerminalCommand => {
                self.settings.terminal_command = self.settings.terminal_command.cycle_next();
            }
        }
    }

    pub fn confirm_edit(&mut self) {
        match self.mode {
            SettingsMode::Normal => {}
            SettingsMode::EditingShell => {
                if !self.edit_buffer.is_empty() {
                    self.settings.default_shell = self.edit_buffer.clone();
                }
                self.mode = SettingsMode::Normal;
                self.edit_buffer.clear();
            }
            SettingsMode::EditingAutoSave => {
                if let Ok(value) = self.edit_buffer.parse::<u64>()
                    && value > 0
                {
                    self.settings.auto_save_interval_seconds = value;
                }
                self.mode = SettingsMode::Normal;
                self.edit_buffer.clear();
            }
        }
    }

    pub fn cancel_edit(&mut self) {
        self.mode = SettingsMode::Normal;
        self.edit_buffer.clear();
    }

    pub fn handle_edit_input(&mut self, character: char) {
        match self.mode {
            SettingsMode::Normal => {}
            SettingsMode::EditingShell => {
                self.edit_buffer.push(character);
            }
            SettingsMode::EditingAutoSave => {
                if character.is_ascii_digit() {
                    self.edit_buffer.push(character);
                }
            }
        }
    }

    pub fn handle_edit_backspace(&mut self) {
        self.edit_buffer.pop();
    }

    #[must_use]
    pub const fn is_editing(&self) -> bool {
        !matches!(self.mode, SettingsMode::Normal)
    }
}

impl Default for SettingsState {
    fn default() -> Self {
        Self::new()
    }
}
