// Placeholder for config directory paths
use std::path::PathBuf;

pub fn get_config_dir() -> PathBuf {
    dirs::config_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("chloe")
}

pub fn get_state_path() -> PathBuf {
    get_config_dir().join("state.json")
}

pub fn get_config_path() -> PathBuf {
    get_config_dir().join("config.toml")
}
