use std::env;
use std::path::PathBuf;

pub fn get_config_dir() -> PathBuf {
    env::current_dir()
        .unwrap_or_else(|_| PathBuf::from("."))
        .join(".chloe")
}

pub fn get_state_path() -> PathBuf {
    get_config_dir().join("state.json")
}
