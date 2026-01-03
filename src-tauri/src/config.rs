use std::fs;
use std::path::PathBuf;
use tauri::{AppHandle, Manager};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Config {
    #[serde(rename = "diaryDir")]
    pub diary_dir: Option<String>,

    // Background settings
    #[serde(rename = "bgType", default)]
    pub bg_type: Option<String>,      // "preset" | "color" | "image"

    #[serde(rename = "bgValue", default)]
    pub bg_value: Option<String>,     // preset name / color hex / image path

    #[serde(rename = "bgFit", default)]
    pub bg_fit: Option<String>,       // "cover" | "contain" | "fill" | "auto"

    #[serde(rename = "bgOpacity", default)]
    pub bg_opacity: Option<f64>,      // 0.1 to 1.0

    #[serde(rename = "leftWidth", default)]
    pub left_width: Option<u32>,      // sidebar width in pixels

    // Window settings
    #[serde(rename = "windowWidth", default)]
    pub window_width: Option<u32>,

    #[serde(rename = "windowHeight", default)]
    pub window_height: Option<u32>,

    // Password settings
    #[serde(rename = "passwordHash", default)]
    pub password_hash: Option<String>, // bcrypt hash

    // Language settings
    #[serde(rename = "language", default)]
    pub language: Option<String>,      // "en" | "zh"
}

pub async fn get_diary_dir(app: &AppHandle) -> Result<PathBuf, String> {
    let config = read_config(app)?;

    if let Some(dir) = config.diary_dir {
        let path = PathBuf::from(dir);
        if path.exists() {
            return Ok(path);
        }
    }

    // Default directory
    let app_data = app.path().app_data_dir()
        .map_err(|e| format!("Failed to get app data dir: {}", e))?;
    let default_dir = app_data.join("diary");

    if !default_dir.exists() {
        fs::create_dir_all(&default_dir)
            .map_err(|e| format!("Failed to create diary dir: {}", e))?;
    }

    Ok(default_dir)
}

#[cfg(not(target_os = "android"))]
pub async fn change_storage_path(app: &AppHandle) -> Result<String, String> {
    use tauri_plugin_dialog::{DialogExt, FilePath};

    let folder = app.dialog()
        .file()
        .set_title("Select a folder to store diaries")
        .blocking_pick_folder();

    if let Some(FilePath::Path(path)) = folder {
        let mut config = read_config(app)?;
        config.diary_dir = Some(path.to_string_lossy().to_string());
        write_config(app, &config)?;

        Ok(path.to_string_lossy().to_string())
    } else {
        Err("No folder selected".to_string())
    }
}

#[cfg(target_os = "android")]
pub async fn change_storage_path(_app: &AppHandle) -> Result<String, String> {
    Err("Changing storage path is not supported on Android".to_string())
}

pub fn read_config(app: &AppHandle) -> Result<Config, String> {
    let config_path = get_config_path(app)?;

    if config_path.exists() {
        let content = fs::read_to_string(&config_path)
            .map_err(|e| format!("Failed to read config: {}", e))?;
        serde_json::from_str(&content)
            .map_err(|e| format!("Invalid config format: {}", e))
    } else {
        Ok(Config::default())
    }
}

pub fn write_config(app: &AppHandle, config: &Config) -> Result<(), String> {
    let config_path = get_config_path(app)?;
    let json = serde_json::to_string_pretty(config)
        .map_err(|e| format!("Failed to serialize config: {}", e))?;

    fs::write(&config_path, json)
        .map_err(|e| format!("Failed to write config: {}", e))?;

    Ok(())
}

pub fn get_config_path(app: &AppHandle) -> Result<PathBuf, String> {
    let app_data = app.path().app_data_dir()
        .map_err(|e| format!("Failed to get app data dir: {}", e))?;

    if !app_data.exists() {
        fs::create_dir_all(&app_data)
            .map_err(|e| format!("Failed to create app data dir: {}", e))?;
    }

    Ok(app_data.join("config.json"))
}

/// Save background image to app data directory
pub fn save_bg_image(app: &AppHandle, data: &[u8], ext: &str) -> Result<String, String> {
    // Validate and sanitize extension
    let valid_extensions = ["png", "jpg", "jpeg", "gif", "webp"];
    let sanitized_ext = ext.to_lowercase();
    let final_ext = if valid_extensions.contains(&sanitized_ext.as_str()) {
        sanitized_ext
    } else {
        "png".to_string()
    };

    let app_data = app.path().app_data_dir()
        .map_err(|e| format!("Failed to get app data dir: {}", e))?;

    // Ensure app data directory exists first
    if !app_data.exists() {
        fs::create_dir_all(&app_data)
            .map_err(|e| format!("Failed to create app data dir: {}", e))?;
    }

    let bg_dir = app_data.join("backgrounds");
    // Use create_dir_all to ensure all parent directories are created
    fs::create_dir_all(&bg_dir)
        .map_err(|e| format!("Failed to create backgrounds dir: {}", e))?;

    let filename = format!("bg-{}.{}", chrono::Local::now().timestamp(), final_ext);
    let file_path = bg_dir.join(&filename);

    fs::write(&file_path, data)
        .map_err(|e| format!("Failed to write background image: {}", e))?;

    Ok(file_path.to_string_lossy().to_string())
}
