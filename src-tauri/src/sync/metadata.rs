use serde::{Deserialize, Serialize};
use sha2::{Sha256, Digest};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use tauri::{AppHandle, Manager};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SyncSettings {
    pub enabled: bool,
    pub sync_mode: String, // "auto" or "manual"
    pub sync_interval_minutes: u32,
}

impl Default for SyncSettings {
    fn default() -> Self {
        Self {
            enabled: false,
            sync_mode: "manual".to_string(),
            sync_interval_minutes: 15,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FileMetadata {
    pub local_modified: String,
    pub remote_id: Option<String>,
    pub remote_modified: Option<String>,
    pub synced_hash: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SyncMetadata {
    pub settings: SyncSettings,
    pub last_sync_time: Option<String>,
    pub drive_folder_id: Option<String>,
    pub entries_folder_id: Option<String>,
    pub images_folder_id: Option<String>,
    pub files: HashMap<String, FileMetadata>,
}

impl SyncMetadata {
    fn get_path(app: &AppHandle) -> Result<PathBuf, std::io::Error> {
        let app_data_dir = app.path().app_data_dir()
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::NotFound, e.to_string()))?;
        fs::create_dir_all(&app_data_dir)?;
        Ok(app_data_dir.join("sync_meta.json"))
    }

    pub fn load(app: &AppHandle) -> Result<Self, std::io::Error> {
        let path = Self::get_path(app)?;
        if path.exists() {
            let content = fs::read_to_string(&path)?;
            serde_json::from_str(&content)
                .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e.to_string()))
        } else {
            Ok(Self::default())
        }
    }

    pub fn save(&self, app: &AppHandle) -> Result<(), std::io::Error> {
        let path = Self::get_path(app)?;
        let content = serde_json::to_string_pretty(self)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e.to_string()))?;
        fs::write(&path, content)
    }

    pub fn update_file_metadata(
        &mut self,
        relative_path: &str,
        local_modified: &str,
        remote_id: Option<String>,
        remote_modified: Option<String>,
        content_hash: &str,
    ) {
        self.files.insert(
            relative_path.to_string(),
            FileMetadata {
                local_modified: local_modified.to_string(),
                remote_id,
                remote_modified,
                synced_hash: content_hash.to_string(),
            },
        );
    }

    pub fn remove_file_metadata(&mut self, relative_path: &str) {
        self.files.remove(relative_path);
    }

    pub fn get_file_metadata(&self, relative_path: &str) -> Option<&FileMetadata> {
        self.files.get(relative_path)
    }

    pub fn update_last_sync_time(&mut self) {
        self.last_sync_time = Some(chrono::Utc::now().to_rfc3339());
    }
}

pub fn calculate_file_hash(path: &PathBuf) -> Result<String, std::io::Error> {
    let content = fs::read(path)?;
    let mut hasher = Sha256::new();
    hasher.update(&content);
    Ok(hex::encode(hasher.finalize()))
}

pub fn calculate_content_hash(content: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(content);
    hex::encode(hasher.finalize())
}
