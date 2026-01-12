mod amp;
mod claude_code;
mod gemini;
mod opencode;

use crate::types::AgentProvider;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use uuid::Uuid;

pub struct ProviderSpec {
    pub provider: AgentProvider,
    pub command: &'static str,
    pub prompt_style: PromptStyle,
    pub generate_files: fn(Uuid, &Path) -> Vec<GeneratedFile>,
}

#[derive(Debug, Clone, Copy)]
pub enum PromptStyle {
    Direct,
    Flag(&'static str),
}

#[derive(Debug, Clone)]
pub struct GeneratedFile {
    pub path: PathBuf,
    pub content: String,
}

pub struct ProviderCommand {
    pub program: String,
    pub arguments: Vec<String>,
    pub environment: HashMap<String, String>,
}

impl ProviderSpec {
    #[must_use]
    pub fn build_command(&self, prompt: &str) -> ProviderCommand {
        let mut arguments = Vec::new();

        if !prompt.is_empty() {
            match self.prompt_style {
                PromptStyle::Direct => arguments.push(prompt.to_string()),
                PromptStyle::Flag(flag) => {
                    arguments.push(flag.to_string());
                    arguments.push(prompt.to_string());
                }
            }
        }

        ProviderCommand {
            program: self.command.to_string(),
            arguments,
            environment: HashMap::new(),
        }
    }

    #[must_use]
    pub fn build_files(&self, task_id: Uuid, working_directory: &Path) -> Vec<GeneratedFile> {
        (self.generate_files)(task_id, working_directory)
    }
}

pub fn get_spec(provider: AgentProvider) -> &'static ProviderSpec {
    match provider {
        AgentProvider::ClaudeCode => &claude_code::SPEC,
        AgentProvider::Gemini => &gemini::SPEC,
        AgentProvider::Amp => &amp::SPEC,
        AgentProvider::OpenCode => &opencode::SPEC,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_command_direct_prompt() {
        let spec = get_spec(AgentProvider::ClaudeCode);
        let command = spec.build_command("Fix the bug");

        assert_eq!(command.program, "claude");
        assert_eq!(command.arguments, vec!["Fix the bug"]);
    }

    #[test]
    fn test_build_command_flag_prompt() {
        let spec = get_spec(AgentProvider::OpenCode);
        let command = spec.build_command("Fix the bug");

        assert_eq!(command.program, "opencode");
        assert_eq!(command.arguments, vec!["--prompt", "Fix the bug"]);
    }

    #[test]
    fn test_build_command_empty_prompt() {
        let spec = get_spec(AgentProvider::ClaudeCode);
        let command = spec.build_command("");

        assert_eq!(command.program, "claude");
        assert!(command.arguments.is_empty());
    }
}
