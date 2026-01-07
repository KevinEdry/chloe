use crate::app::App;
use crate::types::Result;
use std::fs;

/// Save the application state to disk
pub fn save_state(app: &App) -> Result<()> {
    let path = super::paths::get_state_path();

    // Create parent directory if it doesn't exist
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }

    // Serialize to JSON
    let json = serde_json::to_string_pretty(app)?;

    // Write to file
    fs::write(path, json)?;

    Ok(())
}

/// Load the application state from disk
pub fn load_state() -> Result<App> {
    let path = super::paths::get_state_path();

    if !path.exists() {
        // No saved state, return default
        return Ok(App::default());
    }

    // Read file
    let json = fs::read_to_string(path)?;

    // Deserialize
    let app: App = serde_json::from_str(&json)?;

    Ok(app)
}
