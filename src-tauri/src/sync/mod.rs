pub mod auth;
pub mod drive;
pub mod engine;
pub mod metadata;

use serde::{Deserialize, Serialize};
use tauri::AppHandle;

pub use auth::GoogleAuth;
pub use drive::DriveClient;
pub use engine::SyncEngine;
pub use metadata::{SyncMetadata, SyncSettings};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SyncStatus {
    pub connected: bool,
    pub syncing: bool,
    pub last_sync: Option<String>,
    pub error: Option<String>,
    pub account_email: Option<String>,
}

impl Default for SyncStatus {
    fn default() -> Self {
        Self {
            connected: false,
            syncing: false,
            last_sync: None,
            error: None,
            account_email: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SyncReport {
    pub uploaded: Vec<String>,
    pub downloaded: Vec<String>,
    pub deleted_local: Vec<String>,
    pub deleted_remote: Vec<String>,
    pub conflicts_resolved: Vec<String>,
    pub errors: Vec<String>,
    pub duration_ms: u64,
}

impl Default for SyncReport {
    fn default() -> Self {
        Self {
            uploaded: Vec::new(),
            downloaded: Vec::new(),
            deleted_local: Vec::new(),
            deleted_remote: Vec::new(),
            conflicts_resolved: Vec::new(),
            errors: Vec::new(),
            duration_ms: 0,
        }
    }
}

pub async fn get_status(app: &AppHandle) -> Result<SyncStatus, String> {
    let auth = GoogleAuth::load(app).map_err(|e| e.to_string())?;
    let metadata = SyncMetadata::load(app).unwrap_or_default();

    Ok(SyncStatus {
        connected: auth.is_authenticated(),
        syncing: false,
        last_sync: metadata.last_sync_time,
        error: None,
        account_email: auth.get_email(),
    })
}

pub async fn get_settings(app: &AppHandle) -> Result<SyncSettings, String> {
    SyncMetadata::load(app)
        .map(|m| m.settings)
        .map_err(|e| e.to_string())
}

pub async fn save_settings(app: &AppHandle, settings: SyncSettings) -> Result<(), String> {
    let mut metadata = SyncMetadata::load(app).unwrap_or_default();
    metadata.settings = settings;
    metadata.save(app).map_err(|e| e.to_string())
}
