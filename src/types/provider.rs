use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum AgentProvider {
    #[default]
    ClaudeCode,
    Codex,
    Aider,
    Gemini,
    Amp,
    OpenCode,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DetectedProvider {
    pub provider: AgentProvider,
    pub path: PathBuf,
}

impl AgentProvider {
    #[must_use]
    pub const fn display_name(self) -> &'static str {
        match self {
            Self::ClaudeCode => "Claude Code",
            Self::Codex => "Codex",
            Self::Aider => "Aider",
            Self::Gemini => "Gemini",
            Self::Amp => "Amp",
            Self::OpenCode => "OpenCode",
        }
    }

    #[must_use]
    pub const fn command_name(self) -> &'static str {
        match self {
            Self::ClaudeCode => "claude",
            Self::Codex => "codex",
            Self::Aider => "aider",
            Self::Gemini => "gemini",
            Self::Amp => "amp",
            Self::OpenCode => "opencode",
        }
    }

    #[must_use]
    pub const fn all() -> &'static [Self] {
        &[
            Self::ClaudeCode,
            Self::Codex,
            Self::Aider,
            Self::Gemini,
            Self::Amp,
            Self::OpenCode,
        ]
    }

    #[must_use]
    pub fn detect(self) -> Option<DetectedProvider> {
        let output = std::process::Command::new("which")
            .arg(self.command_name())
            .output()
            .ok()?;

        if !output.status.success() {
            return None;
        }

        let path_string = String::from_utf8_lossy(&output.stdout);
        let path = PathBuf::from(path_string.trim());

        if path.exists() {
            Some(DetectedProvider {
                provider: self,
                path,
            })
        } else {
            None
        }
    }

    #[must_use]
    pub fn detect_all_available() -> Vec<DetectedProvider> {
        Self::all()
            .iter()
            .filter_map(|provider| provider.detect())
            .collect()
    }

    #[must_use]
    pub fn default_config(self) -> ProviderConfig {
        match self {
            Self::ClaudeCode => ProviderConfig {
                command: "claude".into(),
                arguments: vec![],
                environment: HashMap::new(),
                working_directory_argument: None,
                supports_worktree: true,
            },
            Self::Codex => ProviderConfig {
                command: "codex".into(),
                arguments: vec![],
                environment: HashMap::new(),
                working_directory_argument: None,
                supports_worktree: true,
            },
            Self::Aider => ProviderConfig {
                command: "aider".into(),
                arguments: vec![],
                environment: HashMap::new(),
                working_directory_argument: None,
                supports_worktree: true,
            },
            Self::Gemini => ProviderConfig {
                command: "gemini".into(),
                arguments: vec![],
                environment: HashMap::new(),
                working_directory_argument: None,
                supports_worktree: true,
            },
            Self::Amp => ProviderConfig {
                command: "amp".into(),
                arguments: vec![],
                environment: HashMap::new(),
                working_directory_argument: None,
                supports_worktree: true,
            },
            Self::OpenCode => ProviderConfig {
                command: "opencode".into(),
                arguments: vec![],
                environment: HashMap::new(),
                working_directory_argument: None,
                supports_worktree: true,
            },
        }
    }
}

impl AgentProvider {
    #[must_use]
    pub const fn cycle_next(self) -> Self {
        match self {
            Self::ClaudeCode => Self::Codex,
            Self::Codex => Self::Aider,
            Self::Aider => Self::Gemini,
            Self::Gemini => Self::Amp,
            Self::Amp => Self::OpenCode,
            Self::OpenCode => Self::ClaudeCode,
        }
    }
}

impl std::fmt::Display for AgentProvider {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{}", self.display_name())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderConfig {
    pub command: PathBuf,
    pub arguments: Vec<String>,
    pub environment: HashMap<String, String>,
    pub working_directory_argument: Option<String>,
    pub supports_worktree: bool,
}

impl ProviderConfig {
    #[must_use]
    pub fn build_spawn_command(&self, working_directory: &std::path::Path) -> SpawnCommand {
        let mut arguments = self.arguments.clone();

        if let Some(directory_argument) = &self.working_directory_argument {
            arguments.push(directory_argument.clone());
            arguments.push(working_directory.display().to_string());
        }

        SpawnCommand {
            command: self.command.clone(),
            arguments,
            environment: self.environment.clone(),
            working_directory: working_directory.to_path_buf(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct SpawnCommand {
    pub command: PathBuf,
    pub arguments: Vec<String>,
    pub environment: HashMap<String, String>,
    pub working_directory: PathBuf,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ProviderRegistry {
    pub configs: HashMap<AgentProvider, ProviderConfig>,
}

impl ProviderRegistry {
    #[must_use]
    pub fn new() -> Self {
        let mut configs = HashMap::new();
        for provider in AgentProvider::all() {
            configs.insert(*provider, provider.default_config());
        }
        Self { configs }
    }

    #[must_use]
    pub fn get_config(&self, provider: AgentProvider) -> ProviderConfig {
        self.configs
            .get(&provider)
            .cloned()
            .unwrap_or_else(|| provider.default_config())
    }

    pub fn set_config(&mut self, provider: AgentProvider, config: ProviderConfig) {
        self.configs.insert(provider, config);
    }
}
