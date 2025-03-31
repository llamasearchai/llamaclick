pub mod settings;

use directories::ProjectDirs;
use settings::Settings;
use std::fs;
use std::path::PathBuf;
use crate::error::{Result, config_error};

/// Load settings from the configuration file
pub fn load_settings() -> Result<Settings> {
    let config_path = get_config_path()?;
    
    // Create config directory if it doesn't exist
    if let Some(config_dir) = config_path.parent() {
        fs::create_dir_all(config_dir)?;
    }
    
    // If config file doesn't exist, create a default one
    if !config_path.exists() {
        let default_settings = Settings::default();
        save_settings(&default_settings)?;
        return Ok(default_settings);
    }
    
    // Read and parse the config file
    let config_content = fs::read_to_string(&config_path)?;
    let settings: Settings = toml::from_str(&config_content)
        .map_err(|e| config_error(format!("Failed to parse config file: {}", e)))?;
    
    Ok(settings)
}

/// Save settings to the configuration file
pub fn save_settings(settings: &Settings) -> Result<()> {
    let config_path = get_config_path()?;
    
    // Create config directory if it doesn't exist
    if let Some(config_dir) = config_path.parent() {
        fs::create_dir_all(config_dir)?;
    }
    
    // Serialize settings to TOML
    let toml_content = toml::to_string_pretty(settings)
        .map_err(|e| config_error(format!("Failed to serialize settings: {}", e)))?;
    
    // Write to file
    fs::write(&config_path, toml_content)?;
    
    Ok(())
}

/// Get the path to the configuration file
fn get_config_path() -> Result<PathBuf> {
    let project_dirs = ProjectDirs::from("ai", "llamasearch", "llamaclick")
        .ok_or_else(|| config_error("Failed to determine config directory"))?;
    
    let config_dir = project_dirs.config_dir();
    let config_path = config_dir.join("config.toml");
    
    Ok(config_path)
} 