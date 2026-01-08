use crate::app::App;
use crate::types::Result;
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
