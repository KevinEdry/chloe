use super::{GeneratedFile, PromptStyle, ProviderSpec};
use crate::types::AgentProvider;
use std::path::Path;
use uuid::Uuid;

pub static SPEC: ProviderSpec = ProviderSpec {
    provider: AgentProvider::OpenCode,
    command: "opencode",
    prompt_style: PromptStyle::Flag("--prompt"),
    generate_files: generate_files,
};

fn generate_files(task_id: Uuid, working_directory: &Path) -> Vec<GeneratedFile> {
    let log_path = working_directory.join(".opencode").join("chloe-plugin.log");
    let log_path_str = log_path.to_string_lossy();

    let plugin_content = format!(
        r#"// Chloe integration plugin for OpenCode
import {{ appendFileSync }} from "fs";

const log = (msg) => appendFileSync("{log_path_str}", new Date().toISOString() + " " + msg + "\n");

export const ChloeNotifier = async () => {{
  const {{ spawn }} = await import("child_process");
  const taskId = "{task_id}";

  const notify = (type) => {{
    spawn("chloe", ["notify", type, "--worktree-id", taskId], {{
      detached: true,
      stdio: "ignore",
    }}).unref();
  }};

  log("Plugin loaded, taskId: " + taskId);

  return {{
    event: async ({{ event }}) => {{
      log("event: " + event.type);
      if (event.type === "permission.updated") {{
        notify("permission");
      }}
      if (event.type === "session.updated") {{
        notify("start");
      }}
      if (event.type === "session.idle") {{
        log("session.idle detected, notifying end");
        notify("end");
      }}
    }},
  }};
}};
"#
    );

    vec![GeneratedFile {
        path: working_directory
            .join(".opencode")
            .join("plugin")
            .join("chloe-hooks.js"),
        content: plugin_content,
    }]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spec_values() {
        assert_eq!(SPEC.provider, AgentProvider::OpenCode);
        assert_eq!(SPEC.command, "opencode");
    }

    #[test]
    fn test_generate_files_creates_plugin() {
        let task_id = Uuid::new_v4();
        let working_dir = Path::new("/tmp/test");

        let files = generate_files(task_id, working_dir);

        assert_eq!(files.len(), 1);
        assert_eq!(
            files[0].path,
            working_dir
                .join(".opencode")
                .join("plugin")
                .join("chloe-hooks.js")
        );
        assert!(files[0].content.contains("ChloeNotifier"));
        assert!(files[0].content.contains(&task_id.to_string()));
    }
}
