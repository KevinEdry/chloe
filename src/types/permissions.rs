use serde::{Deserialize, Serialize};
use std::collections::HashSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ToolPermission {
    Read,
    Edit,
    Write,
    MultiEdit,
    Glob,
    Grep,
    Bash,
    Skill,
    Task,
    NotebookEdit,
    WebFetch,
    WebSearch,
    KillShell,
    TodoWrite,
    ExitPlanMode,
    AskUserQuestion,
    EnterPlanMode,
}

impl ToolPermission {
    #[must_use]
    pub const fn all() -> &'static [Self] {
        &[
            Self::Read,
            Self::Edit,
            Self::Write,
            Self::MultiEdit,
            Self::Glob,
            Self::Grep,
            Self::Bash,
            Self::Skill,
            Self::Task,
            Self::NotebookEdit,
            Self::WebFetch,
            Self::WebSearch,
            Self::KillShell,
            Self::TodoWrite,
            Self::ExitPlanMode,
            Self::AskUserQuestion,
            Self::EnterPlanMode,
        ]
    }

    #[must_use]
    pub const fn display_name(&self) -> &'static str {
        match self {
            Self::Read => "Read",
            Self::Edit => "Edit",
            Self::Write => "Write",
            Self::MultiEdit => "MultiEdit",
            Self::Glob => "Glob",
            Self::Grep => "Grep",
            Self::Bash => "Bash",
            Self::Skill => "Skill",
            Self::Task => "Task",
            Self::NotebookEdit => "NotebookEdit",
            Self::WebFetch => "WebFetch",
            Self::WebSearch => "WebSearch",
            Self::KillShell => "KillShell",
            Self::TodoWrite => "TodoWrite",
            Self::ExitPlanMode => "ExitPlanMode",
            Self::AskUserQuestion => "AskUserQuestion",
            Self::EnterPlanMode => "EnterPlanMode",
        }
    }

    #[must_use]
    pub const fn description(&self) -> &'static str {
        match self {
            Self::Read => "Read files from the filesystem",
            Self::Edit => "Edit existing files with string replacement",
            Self::Write => "Create new files or overwrite existing ones",
            Self::MultiEdit => "Perform multiple edits across files",
            Self::Glob => "Search for files by pattern",
            Self::Grep => "Search file contents with regex",
            Self::Bash => "Execute shell commands",
            Self::Skill => "Execute predefined skills",
            Self::Task => "Launch background agents",
            Self::NotebookEdit => "Edit Jupyter notebook cells",
            Self::WebFetch => "Fetch content from URLs",
            Self::WebSearch => "Perform web searches",
            Self::KillShell => "Terminate background shells",
            Self::TodoWrite => "Create and manage todo lists",
            Self::ExitPlanMode => "Exit planning mode",
            Self::AskUserQuestion => "Ask user for clarification",
            Self::EnterPlanMode => "Enter planning mode",
        }
    }

    #[must_use]
    pub const fn category(&self) -> ToolCategory {
        match self {
            Self::Read | Self::Edit | Self::Write | Self::MultiEdit | Self::NotebookEdit => {
                ToolCategory::FileOperations
            }
            Self::Glob | Self::Grep => ToolCategory::Search,
            Self::Bash | Self::KillShell => ToolCategory::Shell,
            Self::Task | Self::Skill => ToolCategory::Agents,
            Self::WebFetch | Self::WebSearch => ToolCategory::Web,
            Self::TodoWrite | Self::ExitPlanMode | Self::AskUserQuestion | Self::EnterPlanMode => {
                ToolCategory::Workflow
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ToolCategory {
    FileOperations,
    Search,
    Shell,
    Agents,
    Web,
    Workflow,
}

impl ToolCategory {
    #[must_use]
    pub const fn display_name(self) -> &'static str {
        match self {
            Self::FileOperations => "File Operations",
            Self::Search => "Search",
            Self::Shell => "Shell",
            Self::Agents => "Agents",
            Self::Web => "Web",
            Self::Workflow => "Workflow",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SandboxConfig {
    pub enabled: bool,
    pub auto_allow_bash_if_sandboxed: bool,
}

impl Default for SandboxConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            auto_allow_bash_if_sandboxed: true,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PermissionConfig {
    pub allowed_tools: HashSet<ToolPermission>,
    pub sandbox: SandboxConfig,
}

impl PermissionConfig {
    #[must_use]
    pub fn restrictive() -> Self {
        let mut allowed_tools = HashSet::new();
        allowed_tools.insert(ToolPermission::Read);
        allowed_tools.insert(ToolPermission::Glob);
        allowed_tools.insert(ToolPermission::Grep);
        allowed_tools.insert(ToolPermission::AskUserQuestion);

        Self {
            allowed_tools,
            sandbox: SandboxConfig {
                enabled: true,
                auto_allow_bash_if_sandboxed: false,
            },
        }
    }

    #[must_use]
    pub fn balanced() -> Self {
        let mut allowed_tools = HashSet::new();
        allowed_tools.insert(ToolPermission::Read);
        allowed_tools.insert(ToolPermission::Edit);
        allowed_tools.insert(ToolPermission::Write);
        allowed_tools.insert(ToolPermission::Glob);
        allowed_tools.insert(ToolPermission::Grep);
        allowed_tools.insert(ToolPermission::Bash);
        allowed_tools.insert(ToolPermission::Skill);
        allowed_tools.insert(ToolPermission::AskUserQuestion);
        allowed_tools.insert(ToolPermission::TodoWrite);

        Self {
            allowed_tools,
            sandbox: SandboxConfig {
                enabled: true,
                auto_allow_bash_if_sandboxed: true,
            },
        }
    }

    #[must_use]
    pub fn permissive() -> Self {
        let allowed_tools = ToolPermission::all().iter().copied().collect();

        Self {
            allowed_tools,
            sandbox: SandboxConfig {
                enabled: true,
                auto_allow_bash_if_sandboxed: true,
            },
        }
    }

    #[must_use]
    pub fn to_provider_tool_list(&self) -> Vec<String> {
        self.allowed_tools
            .iter()
            .map(|tool| tool.display_name().to_string())
            .collect()
    }
}

impl Default for PermissionConfig {
    fn default() -> Self {
        Self::balanced()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PermissionPreset {
    Restrictive,
    Balanced,
    Permissive,
    Custom,
}

impl PermissionPreset {
    #[must_use]
    pub const fn all() -> &'static [Self] {
        &[Self::Restrictive, Self::Balanced, Self::Permissive]
    }

    #[must_use]
    pub const fn display_name(self) -> &'static str {
        match self {
            Self::Restrictive => "Restrictive",
            Self::Balanced => "Balanced",
            Self::Permissive => "Permissive",
            Self::Custom => "Custom",
        }
    }

    #[must_use]
    pub const fn description(self) -> &'static str {
        match self {
            Self::Restrictive => "Read-only access, requires approval for most actions",
            Self::Balanced => "Standard tools with sandbox enabled (recommended)",
            Self::Permissive => "All tools enabled, maximum agent autonomy",
            Self::Custom => "User-defined custom permission set",
        }
    }

    #[must_use]
    pub fn to_config(self) -> PermissionConfig {
        match self {
            Self::Restrictive => PermissionConfig::restrictive(),
            Self::Balanced => PermissionConfig::balanced(),
            Self::Permissive => PermissionConfig::permissive(),
            Self::Custom => PermissionConfig::default(),
        }
    }

    #[must_use]
    pub fn from_config(config: &PermissionConfig) -> Self {
        if config.allowed_tools == PermissionConfig::restrictive().allowed_tools
            && config.sandbox.enabled == PermissionConfig::restrictive().sandbox.enabled
            && config.sandbox.auto_allow_bash_if_sandboxed
                == PermissionConfig::restrictive()
                    .sandbox
                    .auto_allow_bash_if_sandboxed
        {
            return Self::Restrictive;
        }

        if config.allowed_tools == PermissionConfig::balanced().allowed_tools
            && config.sandbox.enabled == PermissionConfig::balanced().sandbox.enabled
            && config.sandbox.auto_allow_bash_if_sandboxed
                == PermissionConfig::balanced()
                    .sandbox
                    .auto_allow_bash_if_sandboxed
        {
            return Self::Balanced;
        }

        if config.allowed_tools == PermissionConfig::permissive().allowed_tools
            && config.sandbox.enabled == PermissionConfig::permissive().sandbox.enabled
            && config.sandbox.auto_allow_bash_if_sandboxed
                == PermissionConfig::permissive()
                    .sandbox
                    .auto_allow_bash_if_sandboxed
        {
            return Self::Permissive;
        }

        Self::Custom
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_restrictive_preset_has_minimal_permissions() {
        let config = PermissionConfig::restrictive();
        assert_eq!(config.allowed_tools.len(), 4);
        assert!(config.allowed_tools.contains(&ToolPermission::Read));
        assert!(!config.allowed_tools.contains(&ToolPermission::Write));
        assert!(!config.allowed_tools.contains(&ToolPermission::Bash));
    }

    #[test]
    fn test_balanced_preset_has_standard_permissions() {
        let config = PermissionConfig::balanced();
        assert!(config.allowed_tools.contains(&ToolPermission::Read));
        assert!(config.allowed_tools.contains(&ToolPermission::Write));
        assert!(config.allowed_tools.contains(&ToolPermission::Bash));
        assert!(config.sandbox.enabled);
        assert!(config.sandbox.auto_allow_bash_if_sandboxed);
    }

    #[test]
    fn test_permissive_preset_has_all_permissions() {
        let config = PermissionConfig::permissive();
        assert_eq!(config.allowed_tools.len(), ToolPermission::all().len());
    }

    #[test]
    fn test_preset_detection_from_config() {
        let restrictive_config = PermissionConfig::restrictive();
        assert_eq!(
            PermissionPreset::from_config(&restrictive_config),
            PermissionPreset::Restrictive
        );

        let balanced_config = PermissionConfig::balanced();
        assert_eq!(
            PermissionPreset::from_config(&balanced_config),
            PermissionPreset::Balanced
        );

        let permissive_config = PermissionConfig::permissive();
        assert_eq!(
            PermissionPreset::from_config(&permissive_config),
            PermissionPreset::Permissive
        );
    }

    #[test]
    fn test_tool_categories() {
        assert_eq!(
            ToolPermission::Read.category(),
            ToolCategory::FileOperations
        );
        assert_eq!(ToolPermission::Bash.category(), ToolCategory::Shell);
        assert_eq!(ToolPermission::WebFetch.category(), ToolCategory::Web);
    }

    #[test]
    fn test_to_provider_tool_list() {
        let config = PermissionConfig::balanced();
        let tool_list = config.to_provider_tool_list();
        assert!(tool_list.contains(&"Read".to_string()));
        assert!(tool_list.contains(&"Bash".to_string()));
    }
}
