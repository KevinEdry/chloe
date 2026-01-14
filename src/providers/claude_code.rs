use super::{GeneratedFile, OneShotPromptStyle, PromptStyle, ProviderSpec};
use crate::types::PermissionConfig;
use std::path::Path;
use uuid::Uuid;

pub static SPEC: ProviderSpec = ProviderSpec {
    command: "claude",
    prompt_style: PromptStyle::Direct,
    oneshot_style: OneShotPromptStyle::Flag("-p"),
    generate_files,
};

fn generate_files(
    task_id: Uuid,
    working_directory: &Path,
    permission_config: &PermissionConfig,
) -> Vec<GeneratedFile> {
    let notify_start = format!("chloe notify start --worktree-id {task_id}");
    let notify_end = format!("chloe notify end --worktree-id {task_id}");
    let notify_permission = format!("chloe notify permission --worktree-id {task_id}");

    let allowed_tools = permission_config.to_provider_tool_list();

    let settings = serde_json::json!({
        "permissions": {
            "allow": allowed_tools
        },
        "sandbox": {
            "enabled": permission_config.sandbox.enabled,
            "autoAllowBashIfSandboxed": permission_config.sandbox.auto_allow_bash_if_sandboxed
        },
        "includeCoAuthoredBy": false,
        "gitAttribution": false,
        "hooks": {
            "UserPromptSubmit": [
                {
                    "hooks": [
                        { "type": "command", "command": notify_start }
                    ]
                }
            ],
            "PermissionRequest": [
                {
                    "matcher": "*",
                    "hooks": [
                        { "type": "command", "command": notify_permission }
                    ]
                }
            ],
            "Stop": [
                {
                    "hooks": [
                        { "type": "command", "command": notify_end }
                    ]
                }
            ]
        }
    });

    vec![GeneratedFile {
        path: working_directory
            .join(".claude")
            .join("settings.local.json"),
        content: serde_json::to_string_pretty(&settings).unwrap_or_default(),
    }]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spec_values() {
        assert_eq!(SPEC.command, "claude");
    }

    #[test]
    fn test_generate_files_creates_settings() {
        let task_id = Uuid::new_v4();
        let working_dir = Path::new("/tmp/test");
        let permission_config = PermissionConfig::default();

        let files = generate_files(task_id, working_dir, &permission_config);

        assert_eq!(files.len(), 1);
        assert_eq!(
            files[0].path,
            working_dir.join(".claude").join("settings.local.json")
        );
        assert!(files[0].content.contains("hooks"));
        assert!(files[0].content.contains(&task_id.to_string()));
    }
}
