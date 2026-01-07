use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub theme: Theme,
    pub default_shell: String,
    pub auto_save_interval_secs: u64,
    pub ide_command: IdeCommand,
    pub terminal_command: TerminalCommand,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Theme {
    Dark,
    Light,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum IdeCommand {
    Cursor,
    VSCode,
    Custom(String),
}

impl IdeCommand {
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

        Self::Cursor
    }

    pub fn command_name(&self) -> &str {
        match self {
            Self::Cursor => "cursor",
            Self::VSCode => "code",
            Self::Custom(cmd) => cmd,
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
            Self::Custom(cmd) => {
                std::process::Command::new(cmd).arg(path).spawn()?;
            }
        }
        Ok(())
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            theme: Theme::Dark,
            default_shell: std::env::var("SHELL").unwrap_or_else(|_| "/bin/sh".to_string()),
            auto_save_interval_secs: 30,
            ide_command: IdeCommand::detect(),
            terminal_command: TerminalCommand::detect(),
        }
    }
}
