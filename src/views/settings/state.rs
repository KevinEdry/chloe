use crate::types::{AgentProvider, DetectedProvider, ProviderRegistry};
use serde::{Deserialize, Serialize};

const SECTION_COUNT: usize = 4;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SettingsSection {
    ShellAndTerminal,
    EditorAndIde,
    Agent,
    Persistence,
}

impl SettingsSection {
    pub const ALL: [Self; SECTION_COUNT] = [
        Self::ShellAndTerminal,
        Self::EditorAndIde,
        Self::Agent,
        Self::Persistence,
    ];

    #[must_use]
    pub const fn from_index(index: usize) -> Option<Self> {
        match index {
            0 => Some(Self::ShellAndTerminal),
            1 => Some(Self::EditorAndIde),
            2 => Some(Self::Agent),
            3 => Some(Self::Persistence),
            _ => None,
        }
    }

    #[must_use]
    pub const fn icon(self) -> &'static str {
        match self {
            Self::ShellAndTerminal => ">_",
            Self::EditorAndIde => "<>",
            Self::Agent => "@",
            Self::Persistence => "[]",
        }
    }

    #[must_use]
    pub const fn title(self) -> &'static str {
        match self {
            Self::ShellAndTerminal => "Shell & Terminal",
            Self::EditorAndIde => "Editor & IDE",
            Self::Agent => "Agent",
            Self::Persistence => "Persistence",
        }
    }

    #[must_use]
    pub const fn description(self) -> &'static str {
        match self {
            Self::ShellAndTerminal => "Configure shell and terminal preferences",
            Self::EditorAndIde => "Set up your preferred code editor",
            Self::Agent => "Configure AI agent providers",
            Self::Persistence => "Auto-save and data settings",
        }
    }

    #[must_use]
    pub const fn items(self) -> &'static [SettingItem] {
        match self {
            Self::ShellAndTerminal => &[
                SettingItem::DefaultShell,
                SettingItem::TerminalCommand,
                SettingItem::VcsCommand,
            ],
            Self::EditorAndIde => &[SettingItem::IdeCommand],
            Self::Agent => &[SettingItem::DefaultProvider],
            Self::Persistence => &[SettingItem::AutoSaveInterval],
        }
    }

    #[must_use]
    pub const fn count() -> usize {
        SECTION_COUNT
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SettingsFocus {
    #[default]
    Sidebar,
    Content,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Settings {
    pub default_shell: String,
    pub auto_save_interval_seconds: u64,
    pub ide_command: IdeCommand,
    pub terminal_command: TerminalCommand,
    #[serde(default)]
    pub vcs_command: VcsCommand,
    #[serde(default)]
    pub default_provider: AgentProvider,
    #[serde(default)]
    pub skip_provider_selection: bool,
    #[serde(default)]
    pub provider_registry: ProviderRegistry,
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
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TerminalCommand {
    AppleTerminal,
    ITerm2,
    Custom(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum VcsCommand {
    Git,
    Jujutsu,
}

impl Default for VcsCommand {
    fn default() -> Self {
        Self::detect()
    }
}

impl VcsCommand {
    #[must_use]
    pub fn detect() -> Self {
        if std::process::Command::new("jj")
            .arg("--version")
            .output()
            .is_ok()
        {
            return Self::Jujutsu;
        }

        Self::Git
    }

    #[must_use]
    pub const fn command_name(&self) -> &str {
        match self {
            Self::Git => "git",
            Self::Jujutsu => "jj",
        }
    }

    #[must_use]
    pub const fn supports_worktrees(&self) -> bool {
        match self {
            Self::Git => true,
            Self::Jujutsu => false,
        }
    }

    #[must_use]
    pub const fn display_name(&self) -> &str {
        match self {
            Self::Git => "Git",
            Self::Jujutsu => "Jujutsu (jj)",
        }
    }

    #[must_use]
    pub const fn workspace_term(&self) -> &str {
        match self {
            Self::Git => "Worktree",
            Self::Jujutsu => "Workspace",
        }
    }

    #[must_use]
    pub const fn workspace_term_plural(&self) -> &str {
        match self {
            Self::Git => "Worktrees",
            Self::Jujutsu => "Workspaces",
        }
    }

    #[must_use]
    pub const fn full_title(&self) -> &str {
        match self {
            Self::Git => "Git Worktrees",
            Self::Jujutsu => "Jujutsu Workspaces",
        }
    }

    #[must_use]
    pub const fn description(&self) -> &str {
        match self {
            Self::Git => "Isolated working directories for parallel development",
            Self::Jujutsu => "Isolated workspaces for parallel development",
        }
    }

    #[must_use]
    pub fn available_commands() -> Vec<Self> {
        let mut commands = Vec::new();

        if std::process::Command::new("git")
            .arg("--version")
            .output()
            .is_ok()
        {
            commands.push(Self::Git);
        }

        if std::process::Command::new("jj")
            .arg("--version")
            .output()
            .is_ok()
        {
            commands.push(Self::Jujutsu);
        }

        if commands.is_empty() {
            commands.push(Self::Git);
        }

        commands
    }
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
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            default_shell: std::env::var("SHELL").unwrap_or_else(|_| "/bin/sh".to_string()),
            auto_save_interval_seconds: 30,
            ide_command: IdeCommand::detect(),
            terminal_command: TerminalCommand::detect(),
            vcs_command: VcsCommand::detect(),
            default_provider: AgentProvider::default(),
            skip_provider_selection: false,
            provider_registry: ProviderRegistry::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum SettingsMode {
    #[default]
    Normal,
    EditingShell {
        initial_value: String,
    },
    EditingAutoSave {
        initial_value: u64,
    },
    SelectingProvider {
        selected_index: usize,
    },
    SelectingIde {
        selected_index: usize,
    },
    SelectingTerminal {
        selected_index: usize,
    },
    SelectingVcs {
        selected_index: usize,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SettingItem {
    DefaultShell,
    AutoSaveInterval,
    IdeCommand,
    TerminalCommand,
    VcsCommand,
    DefaultProvider,
}

impl SettingItem {
    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::DefaultShell => "Default Shell",
            Self::AutoSaveInterval => "Auto-save Interval",
            Self::IdeCommand => "IDE Command",
            Self::TerminalCommand => "Terminal",
            Self::VcsCommand => "Version Control",
            Self::DefaultProvider => "Default Agent",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SettingsState {
    pub settings: Settings,
    #[serde(skip)]
    pub selected_section: usize,
    #[serde(skip)]
    pub selected_item_in_section: usize,
    #[serde(skip)]
    pub focus: SettingsFocus,
    #[serde(skip)]
    pub mode: SettingsMode,
    #[serde(skip)]
    pub edit_buffer: String,
    #[serde(skip)]
    pub detected_providers: Vec<DetectedProvider>,
}

impl SettingsState {
    #[must_use]
    pub fn new() -> Self {
        Self {
            settings: Settings::default(),
            selected_section: 0,
            selected_item_in_section: 0,
            focus: SettingsFocus::default(),
            mode: SettingsMode::Normal,
            edit_buffer: String::new(),
            detected_providers: AgentProvider::detect_all_available(),
        }
    }

    #[must_use]
    pub fn with_settings(settings: Settings) -> Self {
        Self {
            settings,
            selected_section: 0,
            selected_item_in_section: 0,
            focus: SettingsFocus::default(),
            mode: SettingsMode::Normal,
            edit_buffer: String::new(),
            detected_providers: AgentProvider::detect_all_available(),
        }
    }

    pub const fn toggle_focus(&mut self) {
        self.focus = match self.focus {
            SettingsFocus::Sidebar => SettingsFocus::Content,
            SettingsFocus::Content => SettingsFocus::Sidebar,
        };
    }

    pub const fn navigate_up(&mut self) {
        match self.focus {
            SettingsFocus::Sidebar => {
                self.selected_section = self.selected_section.saturating_sub(1);
                self.selected_item_in_section = 0;
            }
            SettingsFocus::Content => {
                self.selected_item_in_section = self.selected_item_in_section.saturating_sub(1);
            }
        }
    }

    pub fn navigate_down(&mut self) {
        match self.focus {
            SettingsFocus::Sidebar => {
                self.selected_section =
                    (self.selected_section + 1).min(SettingsSection::count() - 1);
                self.selected_item_in_section = 0;
            }
            SettingsFocus::Content => {
                if let Some(section) = SettingsSection::from_index(self.selected_section) {
                    let item_count = section.items().len();
                    self.selected_item_in_section =
                        (self.selected_item_in_section + 1).min(item_count - 1);
                }
            }
        }
    }

    pub fn enter_content(&mut self) {
        if self.focus == SettingsFocus::Sidebar {
            self.focus = SettingsFocus::Content;
        }
    }

    #[must_use]
    pub const fn selected_section(&self) -> Option<SettingsSection> {
        SettingsSection::from_index(self.selected_section)
    }

    #[must_use]
    pub fn selected_item(&self) -> Option<SettingItem> {
        self.selected_section()
            .and_then(|section| section.items().get(self.selected_item_in_section).copied())
    }

    pub fn start_editing(&mut self) {
        let Some(item) = self.selected_item() else {
            return;
        };

        match item {
            SettingItem::DefaultShell => {
                self.edit_buffer = self.settings.default_shell.clone();
                self.mode = SettingsMode::EditingShell {
                    initial_value: self.settings.default_shell.clone(),
                };
            }
            SettingItem::AutoSaveInterval => {
                self.edit_buffer = self.settings.auto_save_interval_seconds.to_string();
                self.mode = SettingsMode::EditingAutoSave {
                    initial_value: self.settings.auto_save_interval_seconds,
                };
            }
            SettingItem::IdeCommand => {
                let current_index = self.get_current_ide_index();
                self.mode = SettingsMode::SelectingIde {
                    selected_index: current_index,
                };
            }
            SettingItem::TerminalCommand => {
                let current_index = self.get_current_terminal_index();
                self.mode = SettingsMode::SelectingTerminal {
                    selected_index: current_index,
                };
            }
            SettingItem::VcsCommand => {
                let current_index = self.get_current_vcs_index();
                self.mode = SettingsMode::SelectingVcs {
                    selected_index: current_index,
                };
            }
            SettingItem::DefaultProvider => {
                if self.detected_providers.len() <= 1 {
                    if let Some(detected) = self.detected_providers.first() {
                        self.settings.default_provider = detected.provider;
                    }
                } else {
                    self.mode = SettingsMode::SelectingProvider { selected_index: 0 };
                }
            }
        }
    }

    const fn get_current_ide_index(&self) -> usize {
        match &self.settings.ide_command {
            IdeCommand::Cursor => 0,
            IdeCommand::VSCode => 1,
            IdeCommand::WebStorm => 2,
            IdeCommand::Custom(_) => 3,
        }
    }

    const fn get_current_terminal_index(&self) -> usize {
        match &self.settings.terminal_command {
            TerminalCommand::AppleTerminal => 0,
            TerminalCommand::ITerm2 => 1,
            TerminalCommand::Custom(_) => 2,
        }
    }

    const fn get_current_vcs_index(&self) -> usize {
        match &self.settings.vcs_command {
            VcsCommand::Git => 0,
            VcsCommand::Jujutsu => 1,
        }
    }

    pub fn select_ide(&mut self, index: usize) {
        self.settings.ide_command = match index {
            0 => IdeCommand::Cursor,
            1 => IdeCommand::VSCode,
            2 => IdeCommand::WebStorm,
            _ => return,
        };
        self.mode = SettingsMode::Normal;
    }

    pub fn select_terminal(&mut self, index: usize) {
        self.settings.terminal_command = match index {
            0 => TerminalCommand::AppleTerminal,
            1 => TerminalCommand::ITerm2,
            _ => return,
        };
        self.mode = SettingsMode::Normal;
    }

    pub fn select_vcs(&mut self, index: usize) {
        self.settings.vcs_command = match index {
            0 => VcsCommand::Git,
            1 => VcsCommand::Jujutsu,
            _ => return,
        };
        self.mode = SettingsMode::Normal;
    }

    pub fn confirm_edit(&mut self) {
        match self.mode {
            SettingsMode::Normal
            | SettingsMode::SelectingProvider { .. }
            | SettingsMode::SelectingIde { .. }
            | SettingsMode::SelectingTerminal { .. }
            | SettingsMode::SelectingVcs { .. } => {}
            SettingsMode::EditingShell { .. } => {
                if !self.edit_buffer.is_empty() {
                    self.settings.default_shell = self.edit_buffer.clone();
                }
                self.mode = SettingsMode::Normal;
                self.edit_buffer.clear();
            }
            SettingsMode::EditingAutoSave { .. } => {
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
            SettingsMode::Normal
            | SettingsMode::SelectingProvider { .. }
            | SettingsMode::SelectingIde { .. }
            | SettingsMode::SelectingTerminal { .. }
            | SettingsMode::SelectingVcs { .. } => {}
            SettingsMode::EditingShell { .. } => {
                self.edit_buffer.push(character);
            }
            SettingsMode::EditingAutoSave { .. } => {
                if character.is_ascii_digit() {
                    self.edit_buffer.push(character);
                }
            }
        }
    }

    pub fn handle_edit_backspace(&mut self) {
        self.edit_buffer.pop();
    }
}

impl Default for SettingsState {
    fn default() -> Self {
        Self::new()
    }
}
