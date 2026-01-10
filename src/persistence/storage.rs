use crate::app::App;
use crate::types::Result;
use crate::views::settings::state::Settings;
use std::fs;

pub fn save_state(app: &App) -> Result<()> {
    let path = super::paths::get_state_path();

    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }

    let json = serde_json::to_string_pretty(app)?;
    fs::write(path, json)?;

    Ok(())
}

pub fn load_state() -> Result<App> {
    let path = super::paths::get_state_path();

    if !path.exists() {
        return Ok(App::default());
    }

    let json = fs::read_to_string(path)?;
    let app: App = serde_json::from_str(&json)?;

    Ok(app)
}

pub fn save_settings(settings: &Settings) -> Result<()> {
    let path = super::paths::get_settings_path();

    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }

    let json = serde_json::to_string_pretty(settings)?;
    fs::write(path, json)?;

    Ok(())
}

pub fn load_settings() -> Result<Settings> {
    let path = super::paths::get_settings_path();

    if !path.exists() {
        return Ok(Settings::default());
    }

    let json = fs::read_to_string(path)?;
    let settings: Settings = serde_json::from_str(&json)?;

    Ok(settings)
}
