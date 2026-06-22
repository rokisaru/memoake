use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf};
use tauri::Manager;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AppConfig {
    pub save_directory: String,
    pub filename_format: String,
    pub timestamp_heading_level: u8,
    pub timestamp_format: String,
}

impl AppConfig {
    pub fn default_for(app_handle: &tauri::AppHandle) -> Result<Self, String> {
        let mut save_dir = app_handle
            .path()
            .document_dir()
            .unwrap_or_else(|_| fallback_documents_dir());
        save_dir.push("memoake");

        Ok(Self {
            save_directory: save_dir.to_string_lossy().to_string(),
            filename_format: "%Y-%m-%d".to_string(),
            timestamp_heading_level: 2,
            timestamp_format: "%H:%M".to_string(),
        })
    }

    pub fn normalize(mut self, app_handle: &tauri::AppHandle) -> Result<Self, String> {
        if self.save_directory.trim().is_empty() {
            self.save_directory = Self::default_for(app_handle)?.save_directory;
        }

        if self.filename_format.trim().is_empty() {
            self.filename_format = "%Y-%m-%d".to_string();
        }

        if !(2..=4).contains(&self.timestamp_heading_level) {
            self.timestamp_heading_level = 2;
        }

        if self.timestamp_format.trim().is_empty() {
            self.timestamp_format = "%H:%M".to_string();
        }

        Ok(self)
    }
}

pub fn load(app_handle: &tauri::AppHandle) -> Result<AppConfig, String> {
    let path = config_path(app_handle)?;
    if !path.exists() {
        let config = AppConfig::default_for(app_handle)?;
        save(app_handle, &config)?;
        return Ok(config);
    }

    let text = fs::read_to_string(&path).map_err(|e| e.to_string())?;
    let config: AppConfig = serde_json::from_str(&text).map_err(|e| e.to_string())?;
    config.normalize(app_handle)
}

pub fn save(app_handle: &tauri::AppHandle, config: &AppConfig) -> Result<(), String> {
    validate(config)?;

    let path = config_path(app_handle)?;
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }

    let text = serde_json::to_string_pretty(config).map_err(|e| e.to_string())?;
    fs::write(path, format!("{text}\n")).map_err(|e| e.to_string())
}

pub fn validate(config: &AppConfig) -> Result<(), String> {
    if config.save_directory.trim().is_empty() {
        return Err("Save directory is required.".to_string());
    }

    let save_dir = PathBuf::from(&config.save_directory);
    if !save_dir.is_absolute() {
        return Err("Save directory must be an absolute path.".to_string());
    }

    if !(2..=4).contains(&config.timestamp_heading_level) {
        return Err("Timestamp heading level must be between 2 and 4.".to_string());
    }

    if config.filename_format.trim().is_empty() {
        return Err("Filename format is required.".to_string());
    }

    if config.timestamp_format.trim().is_empty() {
        return Err("Timestamp format is required.".to_string());
    }

    Ok(())
}

fn config_path(app_handle: &tauri::AppHandle) -> Result<PathBuf, String> {
    let mut config_dir = app_handle
        .path()
        .app_config_dir()
        .map_err(|e| e.to_string())?;
    config_dir.push("config.json");
    Ok(config_dir)
}

fn fallback_documents_dir() -> PathBuf {
    if let Some(user_profile) = std::env::var_os("USERPROFILE") {
        return PathBuf::from(user_profile).join("Documents");
    }

    if let Some(home) = std::env::var_os("HOME") {
        return PathBuf::from(home).join("Documents");
    }

    std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."))
}
