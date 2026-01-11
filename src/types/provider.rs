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
    pub fn default_config(self) -> ProviderConfig {
        match self {
            Self::ClaudeCode => ProviderConfig {
                command: "claude".into(),
                arguments: vec![],
                environment: HashMap::new(),
                working_directory_argument: None,
            },
            Self::Codex => ProviderConfig {
                command: "codex".into(),
                arguments: vec![],
                environment: HashMap::new(),
                working_directory_argument: None,
            },
            Self::Aider => ProviderConfig {
                command: "aider".into(),
                arguments: vec![],
                environment: HashMap::new(),
                working_directory_argument: None,
            },
            Self::Gemini => ProviderConfig {
                command: "gemini".into(),
                arguments: vec![],
                environment: HashMap::new(),
                working_directory_argument: None,
            },
            Self::Amp => ProviderConfig {
                command: "amp".into(),
                arguments: vec![],
                environment: HashMap::new(),
                working_directory_argument: None,
            },
            Self::OpenCode => ProviderConfig {
                command: "opencode".into(),
                arguments: vec![],
                environment: HashMap::new(),
                working_directory_argument: None,
            },
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
