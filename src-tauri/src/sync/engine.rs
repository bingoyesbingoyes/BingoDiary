use super::drive::DriveClient;
use super::metadata::{calculate_file_hash, SyncMetadata};
use super::SyncReport;
use serde::Serialize;
use std::collections::HashMap;
use std::path::PathBuf;
use tauri::{AppHandle, Emitter, Manager};
use tokio::fs;
use walkdir::WalkDir;

#[derive(Clone, Serialize)]
struct SyncProgress {
    stage: String,
    current: u32,
    total: u32,
    message: String,
}

const TAGS_FILE: &str = "tags.json";

#[derive(Debug)]
enum SyncAction {
    Upload { local_path: PathBuf, remote_name: String, remote_id: Option<String> },
    Download { remote_id: String, remote_name: String, local_path: PathBuf },
    DeleteLocal { local_path: PathBuf },
    DeleteRemote { remote_id: String, remote_name: String },
}

pub struct SyncEngine {
    app: AppHandle,
    diary_dir: PathBuf,
}

impl SyncEngine {
    pub fn new(app: AppHandle, diary_dir: PathBuf) -> Self {
        Self { app, diary_dir }
    }

    fn emit_progress(&self, stage: &str, current: u32, total: u32, message: &str) {
        let progress = SyncProgress {
            stage: stage.to_string(),
            current,
            total,
            message: message.to_string(),
        };
        self.app.emit("sync-progress", &progress).ok();
    }

    pub async fn sync(&self, access_token: &str) -> Result<SyncReport, String> {
        let start_time = std::time::Instant::now();
        let mut report = SyncReport::default();

        println!("[Sync] Starting sync...");

        // Emit sync started event
        self.app.emit("sync-started", ()).ok();
        self.emit_progress("init", 0, 4, "Initializing...");

        // Create Drive client
        let drive = DriveClient::new(access_token.to_string());

        // Load sync metadata
        let mut metadata = SyncMetadata::load(&self.app).unwrap_or_default();

        // Ensure folder structure exists on Drive
        println!("[Sync] Ensuring folder structure...");
        self.emit_progress("init", 1, 4, "Connecting to Google Drive...");
        let (app_folder_id, entries_folder_id, images_folder_id) =
            drive.ensure_folder_structure().await?;
        println!("[Sync] Folder structure ready");

        metadata.drive_folder_id = Some(app_folder_id.clone());
        metadata.entries_folder_id = Some(entries_folder_id.clone());
        metadata.images_folder_id = Some(images_folder_id.clone());

        // Sync diary entries
        println!("[Sync] Syncing entries...");
        match self.sync_entries(&drive, &mut metadata, &entries_folder_id).await {
            Ok((mut uploaded, mut downloaded, mut errors)) => {
                report.uploaded.append(&mut uploaded);
                report.downloaded.append(&mut downloaded);
                report.errors.append(&mut errors);
            }
            Err(e) => {
                report.errors.push(format!("Entry sync failed: {}", e));
            }
        }

        // Sync tags.json
        println!("[Sync] Syncing tags...");
        self.emit_progress("tags", 0, 1, "Syncing tags...");
        match self.sync_tags(&drive, &mut metadata, &app_folder_id).await {
            Ok(synced) => {
                if !synced.is_empty() {
                    if synced.starts_with("uploaded") {
                        report.uploaded.push("tags.json".to_string());
                    } else if synced.starts_with("downloaded") {
                        report.downloaded.push("tags.json".to_string());
                    }
                }
            }
            Err(e) => {
                report.errors.push(format!("Tags sync failed: {}", e));
            }
        }

        // Sync images
        println!("[Sync] Syncing images...");
        match self.sync_images(&drive, &mut metadata, &images_folder_id).await {
            Ok((mut uploaded, mut downloaded, mut errors)) => {
                report.uploaded.append(&mut uploaded);
                report.downloaded.append(&mut downloaded);
                report.errors.append(&mut errors);
            }
            Err(e) => {
                report.errors.push(format!("Images sync failed: {}", e));
            }
        }

        // Update sync time and save metadata
        metadata.update_last_sync_time();
        metadata.save(&self.app).map_err(|e| e.to_string())?;

        report.duration_ms = start_time.elapsed().as_millis() as u64;

        println!("[Sync] Sync completed in {}ms - uploaded: {}, downloaded: {}, errors: {}",
            report.duration_ms,
            report.uploaded.len(),
            report.downloaded.len(),
            report.errors.len()
        );

        // Emit sync completed event
        self.app.emit("sync-completed", &report).ok();

        Ok(report)
    }

    async fn sync_entries(
        &self,
        drive: &DriveClient,
        metadata: &mut SyncMetadata,
        folder_id: &str,
    ) -> Result<(Vec<String>, Vec<String>, Vec<String>), String> {
        let mut uploaded = Vec::new();
        let mut downloaded = Vec::new();
        let mut errors = Vec::new();

        // Get local entries
        self.emit_progress("entries", 0, 1, "Scanning local entries...");
        let local_entries = self.get_local_entries().await?;

        // Get remote entries
        self.emit_progress("entries", 0, 1, "Fetching remote entries...");
        let remote_files = drive.list_files(folder_id).await?;
        let remote_entries: HashMap<String, _> = remote_files
            .into_iter()
            .filter(|f| f.name.ends_with(".txt"))
            .map(|f| (f.name.clone(), f))
            .collect();

        // Determine sync actions using last-write-wins
        let mut actions = Vec::new();

        // Check local files
        for (name, local_path) in &local_entries {
            let local_modified = get_file_modified_time(local_path).await?;
            let local_hash = calculate_file_hash(local_path).map_err(|e| e.to_string())?;
            let local_size = fs::metadata(local_path).await.map(|m| m.len()).unwrap_or(0);
            // Consider file "empty" if it's very small (just date line or whitespace)
            let local_is_empty = local_size < 20;

            if let Some(remote) = remote_entries.get(name) {
                // File exists both locally and remotely
                let remote_modified = remote.modified_time.as_ref()
                    .map(|t| t.clone())
                    .unwrap_or_default();
                let remote_size = remote.size.as_ref()
                    .and_then(|s| s.parse::<u64>().ok())
                    .unwrap_or(0);
                let remote_is_empty = remote_size < 20;

                // Special handling for empty vs non-empty files
                if local_is_empty && !remote_is_empty {
                    // Local is empty, remote has content - always download
                    actions.push(SyncAction::Download {
                        remote_id: remote.id.clone(),
                        remote_name: name.clone(),
                        local_path: local_path.clone(),
                    });
                    continue;
                } else if !local_is_empty && remote_is_empty {
                    // Local has content, remote is empty - always upload
                    actions.push(SyncAction::Upload {
                        local_path: local_path.clone(),
                        remote_name: name.clone(),
                        remote_id: Some(remote.id.clone()),
                    });
                    continue;
                }

                // Both have content (or both empty) - use normal sync logic
                // Check if we've synced this before
                let meta_key = format!("entries/{}", name);
                if let Some(file_meta) = metadata.get_file_metadata(&meta_key) {
                    // Compare with last synced version
                    let local_changed = file_meta.synced_hash != local_hash;
                    let remote_changed = file_meta.remote_modified.as_ref() != Some(&remote_modified);

                    if local_changed && remote_changed {
                        // Conflict! Use last-write-wins
                        if local_modified > remote_modified {
                            actions.push(SyncAction::Upload {
                                local_path: local_path.clone(),
                                remote_name: name.clone(),
                                remote_id: Some(remote.id.clone()),
                            });
                        } else {
                            actions.push(SyncAction::Download {
                                remote_id: remote.id.clone(),
                                remote_name: name.clone(),
                                local_path: local_path.clone(),
                            });
                        }
                    } else if local_changed {
                        actions.push(SyncAction::Upload {
                            local_path: local_path.clone(),
                            remote_name: name.clone(),
                            remote_id: Some(remote.id.clone()),
                        });
                    } else if remote_changed {
                        actions.push(SyncAction::Download {
                            remote_id: remote.id.clone(),
                            remote_name: name.clone(),
                            local_path: local_path.clone(),
                        });
                    }
                } else {
                    // First time syncing this file - compare timestamps
                    if local_modified > remote_modified {
                        actions.push(SyncAction::Upload {
                            local_path: local_path.clone(),
                            remote_name: name.clone(),
                            remote_id: Some(remote.id.clone()),
                        });
                    } else {
                        actions.push(SyncAction::Download {
                            remote_id: remote.id.clone(),
                            remote_name: name.clone(),
                            local_path: local_path.clone(),
                        });
                    }
                }
            } else {
                // Only exists locally - upload only if has content
                if !local_is_empty {
                    actions.push(SyncAction::Upload {
                        local_path: local_path.clone(),
                        remote_name: name.clone(),
                        remote_id: None,
                    });
                }
            }
        }

        // Check for remote-only files (need to download)
        for (name, remote) in &remote_entries {
            if !local_entries.contains_key(name) {
                let local_path = self.diary_dir.join(name);
                actions.push(SyncAction::Download {
                    remote_id: remote.id.clone(),
                    remote_name: name.clone(),
                    local_path,
                });
            }
        }

        // Execute sync actions
        let total_actions = actions.len() as u32;
        for (i, action) in actions.into_iter().enumerate() {
            let current = (i + 1) as u32;
            match action {
                SyncAction::Upload { local_path, remote_name, remote_id } => {
                    self.emit_progress("entries", current, total_actions, &format!("Uploading {}...", remote_name));
                    match self.upload_entry(drive, &local_path, &remote_name, folder_id, remote_id.as_deref(), metadata).await {
                        Ok(_) => uploaded.push(remote_name),
                        Err(e) => errors.push(format!("Upload {} failed: {}", remote_name, e)),
                    }
                }
                SyncAction::Download { remote_id, remote_name, local_path } => {
                    self.emit_progress("entries", current, total_actions, &format!("Downloading {}...", remote_name));
                    match self.download_entry(drive, &remote_id, &remote_name, &local_path, metadata).await {
                        Ok(_) => downloaded.push(remote_name),
                        Err(e) => errors.push(format!("Download {} failed: {}", remote_name, e)),
                    }
                }
                _ => {}
            }
        }

        if total_actions == 0 {
            self.emit_progress("entries", 1, 1, "Entries up to date");
        }

        Ok((uploaded, downloaded, errors))
    }

    async fn get_local_entries(&self) -> Result<HashMap<String, PathBuf>, String> {
        let mut entries = HashMap::new();

        let read_dir = fs::read_dir(&self.diary_dir).await
            .map_err(|e| format!("Failed to read diary dir: {}", e))?;

        let mut reader = read_dir;
        while let Ok(Some(entry)) = reader.next_entry().await {
            let path = entry.path();
            if path.is_file() && path.extension().map(|e| e == "txt").unwrap_or(false) {
                if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                    entries.insert(name.to_string(), path);
                }
            }
        }

        Ok(entries)
    }

    async fn upload_entry(
        &self,
        drive: &DriveClient,
        local_path: &PathBuf,
        remote_name: &str,
        folder_id: &str,
        existing_id: Option<&str>,
        metadata: &mut SyncMetadata,
    ) -> Result<(), String> {
        let content = fs::read(local_path).await
            .map_err(|e| format!("Failed to read file: {}", e))?;

        let result = drive.upload_content(
            &content,
            remote_name,
            folder_id,
            "text/plain",
            existing_id,
        ).await?;

        let hash = super::metadata::calculate_content_hash(&content);
        let modified = get_file_modified_time(local_path).await?;

        metadata.update_file_metadata(
            &format!("entries/{}", remote_name),
            &modified,
            Some(result.id),
            result.modified_time,
            &hash,
        );

        Ok(())
    }

    async fn download_entry(
        &self,
        drive: &DriveClient,
        remote_id: &str,
        remote_name: &str,
        local_path: &PathBuf,
        metadata: &mut SyncMetadata,
    ) -> Result<(), String> {
        let content = drive.download_file(remote_id).await?;

        // Create parent directory if needed
        if let Some(parent) = local_path.parent() {
            fs::create_dir_all(parent).await
                .map_err(|e| format!("Failed to create directory: {}", e))?;
        }

        fs::write(local_path, &content).await
            .map_err(|e| format!("Failed to write file: {}", e))?;

        let hash = super::metadata::calculate_content_hash(&content);
        let modified = get_file_modified_time(local_path).await?;

        let remote_meta = drive.get_file_metadata(remote_id).await?;

        metadata.update_file_metadata(
            &format!("entries/{}", remote_name),
            &modified,
            Some(remote_id.to_string()),
            remote_meta.modified_time,
            &hash,
        );

        Ok(())
    }

    async fn sync_tags(
        &self,
        drive: &DriveClient,
        metadata: &mut SyncMetadata,
        folder_id: &str,
    ) -> Result<String, String> {
        let local_path = self.diary_dir.join(TAGS_FILE);
        let remote_file = drive.find_file(TAGS_FILE, folder_id).await?;

        let local_exists = local_path.exists();
        let remote_exists = remote_file.is_some();

        if !local_exists && !remote_exists {
            return Ok(String::new());
        }

        if local_exists && !remote_exists {
            // Upload local
            let content = fs::read(&local_path).await
                .map_err(|e| format!("Failed to read tags: {}", e))?;

            let result = drive.upload_content(
                &content,
                TAGS_FILE,
                folder_id,
                "application/json",
                None,
            ).await?;

            let hash = super::metadata::calculate_content_hash(&content);
            let modified = get_file_modified_time(&local_path).await?;

            metadata.update_file_metadata(
                TAGS_FILE,
                &modified,
                Some(result.id),
                result.modified_time,
                &hash,
            );

            return Ok("uploaded".to_string());
        }

        if !local_exists && remote_exists {
            // Download remote
            let remote = remote_file.unwrap();
            let content = drive.download_file(&remote.id).await?;

            fs::write(&local_path, &content).await
                .map_err(|e| format!("Failed to write tags: {}", e))?;

            let hash = super::metadata::calculate_content_hash(&content);
            let modified = get_file_modified_time(&local_path).await?;

            metadata.update_file_metadata(
                TAGS_FILE,
                &modified,
                Some(remote.id),
                remote.modified_time,
                &hash,
            );

            return Ok("downloaded".to_string());
        }

        // Both exist - compare and sync
        let remote = remote_file.unwrap();
        let local_modified = get_file_modified_time(&local_path).await?;
        let remote_modified = remote.modified_time.clone().unwrap_or_default();

        let local_hash = calculate_file_hash(&local_path).map_err(|e| e.to_string())?;

        let meta_key = TAGS_FILE;
        let should_upload = if let Some(file_meta) = metadata.get_file_metadata(meta_key) {
            let local_changed = file_meta.synced_hash != local_hash;
            let remote_changed = file_meta.remote_modified.as_ref() != Some(&remote_modified);

            if local_changed && remote_changed {
                local_modified > remote_modified
            } else {
                local_changed
            }
        } else {
            local_modified > remote_modified
        };

        if should_upload {
            let content = fs::read(&local_path).await
                .map_err(|e| format!("Failed to read tags: {}", e))?;

            let result = drive.upload_content(
                &content,
                TAGS_FILE,
                folder_id,
                "application/json",
                Some(&remote.id),
            ).await?;

            let hash = super::metadata::calculate_content_hash(&content);

            metadata.update_file_metadata(
                TAGS_FILE,
                &local_modified,
                Some(result.id),
                result.modified_time,
                &hash,
            );

            Ok("uploaded".to_string())
        } else {
            // Download
            let content = drive.download_file(&remote.id).await?;

            fs::write(&local_path, &content).await
                .map_err(|e| format!("Failed to write tags: {}", e))?;

            let hash = super::metadata::calculate_content_hash(&content);
            let modified = get_file_modified_time(&local_path).await?;

            metadata.update_file_metadata(
                TAGS_FILE,
                &modified,
                Some(remote.id),
                remote_modified.into(),
                &hash,
            );

            Ok("downloaded".to_string())
        }
    }

    async fn sync_images(
        &self,
        drive: &DriveClient,
        metadata: &mut SyncMetadata,
        folder_id: &str,
    ) -> Result<(Vec<String>, Vec<String>, Vec<String>), String> {
        let mut uploaded = Vec::new();
        let mut downloaded = Vec::new();
        let mut errors = Vec::new();

        let images_dir = self.diary_dir.join("images");

        // Create images directory if it doesn't exist
        if !images_dir.exists() {
            fs::create_dir_all(&images_dir).await.ok();
        }

        // Get local images
        self.emit_progress("images", 0, 1, "Scanning local images...");
        let mut local_images: HashMap<String, PathBuf> = HashMap::new();
        if images_dir.exists() {
            for entry in WalkDir::new(&images_dir).max_depth(1) {
                if let Ok(entry) = entry {
                    let path = entry.path().to_path_buf();
                    if path.is_file() {
                        if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                            local_images.insert(name.to_string(), path);
                        }
                    }
                }
            }
        }

        // Get remote images
        self.emit_progress("images", 0, 1, "Fetching remote images...");
        let remote_files = drive.list_files(folder_id).await?;
        let remote_images: HashMap<String, _> = remote_files
            .into_iter()
            .map(|f| (f.name.clone(), f))
            .collect();

        // Collect images to upload/download
        let images_to_upload: Vec<_> = local_images.iter()
            .filter(|(name, _)| !remote_images.contains_key(*name))
            .collect();
        let images_to_download: Vec<_> = remote_images.iter()
            .filter(|(name, _)| !local_images.contains_key(*name))
            .collect();

        let total_images = (images_to_upload.len() + images_to_download.len()) as u32;
        let mut current = 0u32;

        // Upload local-only images
        for (name, local_path) in images_to_upload {
            current += 1;
            self.emit_progress("images", current, total_images, &format!("Uploading {}...", name));
            match drive.upload_file(local_path, name, folder_id, None).await {
                Ok(result) => {
                    let hash = calculate_file_hash(local_path).unwrap_or_default();
                    let modified = get_file_modified_time(local_path).await.unwrap_or_default();

                    metadata.update_file_metadata(
                        &format!("images/{}", name),
                        &modified,
                        Some(result.id),
                        result.modified_time,
                        &hash,
                    );
                    uploaded.push(format!("images/{}", name));
                }
                Err(e) => errors.push(format!("Upload image {} failed: {}", name, e)),
            }
        }

        // Download remote-only images
        for (name, remote) in images_to_download {
            current += 1;
            self.emit_progress("images", current, total_images, &format!("Downloading {}...", name));
            let local_path = images_dir.join(name);
            match drive.download_file(&remote.id).await {
                Ok(content) => {
                    if let Err(e) = fs::write(&local_path, &content).await {
                        errors.push(format!("Write image {} failed: {}", name, e));
                        continue;
                    }

                    let hash = super::metadata::calculate_content_hash(&content);
                    let modified = get_file_modified_time(&local_path).await.unwrap_or_default();

                    metadata.update_file_metadata(
                        &format!("images/{}", name),
                        &modified,
                        Some(remote.id.clone()),
                        remote.modified_time.clone(),
                        &hash,
                    );
                    downloaded.push(format!("images/{}", name));
                }
                Err(e) => errors.push(format!("Download image {} failed: {}", name, e)),
            }
        }

        if total_images == 0 {
            self.emit_progress("images", 1, 1, "Images up to date");
        }

        Ok((uploaded, downloaded, errors))
    }
}

impl SyncEngine {
    /// Force upload all local files to cloud, overwriting remote versions
    pub async fn force_upload_sync(&self, access_token: &str) -> Result<SyncReport, String> {
        let start_time = std::time::Instant::now();
        let mut report = SyncReport::default();

        println!("[Sync] Starting FORCE UPLOAD sync...");

        self.app.emit("sync-started", ()).ok();
        self.emit_progress("init", 0, 4, "Force uploading to cloud...");

        let drive = DriveClient::new(access_token.to_string());
        let mut metadata = SyncMetadata::load(&self.app).unwrap_or_default();

        // Ensure folder structure
        self.emit_progress("init", 1, 4, "Connecting to Google Drive...");
        let (app_folder_id, entries_folder_id, images_folder_id) =
            drive.ensure_folder_structure().await?;

        metadata.drive_folder_id = Some(app_folder_id.clone());
        metadata.entries_folder_id = Some(entries_folder_id.clone());
        metadata.images_folder_id = Some(images_folder_id.clone());

        // Force upload all diary entries
        self.emit_progress("entries", 0, 1, "Uploading diary entries...");
        let local_entries = self.get_local_entries().await?;
        let remote_files = drive.list_files(&entries_folder_id).await?;
        let remote_entries: HashMap<String, _> = remote_files
            .into_iter()
            .filter(|f| f.name.ends_with(".txt"))
            .map(|f| (f.name.clone(), f))
            .collect();

        let total = local_entries.len() as u32;
        for (i, (name, local_path)) in local_entries.iter().enumerate() {
            let current = (i + 1) as u32;
            self.emit_progress("entries", current, total, &format!("Uploading {}...", name));

            let existing_id = remote_entries.get(name).map(|r| r.id.as_str());
            match self.upload_entry(&drive, local_path, name, &entries_folder_id, existing_id, &mut metadata).await {
                Ok(_) => report.uploaded.push(name.clone()),
                Err(e) => report.errors.push(format!("Upload {} failed: {}", name, e)),
            }
        }

        // Force upload tags.json
        self.emit_progress("tags", 0, 1, "Uploading tags...");
        let tags_path = self.diary_dir.join("tags.json");
        if tags_path.exists() {
            let content = fs::read(&tags_path).await
                .map_err(|e| format!("Failed to read tags: {}", e))?;

            let existing_tag = drive.find_file("tags.json", &app_folder_id).await?;
            let result = drive.upload_content(
                &content,
                "tags.json",
                &app_folder_id,
                "application/json",
                existing_tag.as_ref().map(|f| f.id.as_str()),
            ).await?;

            let hash = super::metadata::calculate_content_hash(&content);
            let modified = get_file_modified_time(&tags_path).await?;
            metadata.update_file_metadata("tags.json", &modified, Some(result.id), result.modified_time, &hash);
            report.uploaded.push("tags.json".to_string());
        }

        // Force upload images
        self.emit_progress("images", 0, 1, "Uploading images...");
        let images_dir = self.diary_dir.join("images");
        if images_dir.exists() {
            let mut local_images: HashMap<String, PathBuf> = HashMap::new();
            for entry in walkdir::WalkDir::new(&images_dir).max_depth(1) {
                if let Ok(entry) = entry {
                    let path = entry.path().to_path_buf();
                    if path.is_file() {
                        if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                            local_images.insert(name.to_string(), path);
                        }
                    }
                }
            }

            let remote_images_list = drive.list_files(&images_folder_id).await?;
            let remote_images: HashMap<String, _> = remote_images_list
                .into_iter()
                .map(|f| (f.name.clone(), f))
                .collect();

            let total = local_images.len() as u32;
            for (i, (name, local_path)) in local_images.iter().enumerate() {
                let current = (i + 1) as u32;
                self.emit_progress("images", current, total, &format!("Uploading {}...", name));

                let existing_id = remote_images.get(name).map(|r| r.id.as_str());
                match drive.upload_file(local_path, name, &images_folder_id, existing_id).await {
                    Ok(result) => {
                        let hash = calculate_file_hash(local_path).unwrap_or_default();
                        let modified = get_file_modified_time(local_path).await.unwrap_or_default();
                        metadata.update_file_metadata(
                            &format!("images/{}", name),
                            &modified,
                            Some(result.id),
                            result.modified_time,
                            &hash,
                        );
                        report.uploaded.push(format!("images/{}", name));
                    }
                    Err(e) => report.errors.push(format!("Upload image {} failed: {}", name, e)),
                }
            }
        }

        // Update sync time
        metadata.update_last_sync_time();
        metadata.save(&self.app).map_err(|e| e.to_string())?;

        report.duration_ms = start_time.elapsed().as_millis() as u64;

        println!("[Sync] Force upload completed in {}ms - uploaded: {}, errors: {}",
            report.duration_ms, report.uploaded.len(), report.errors.len());

        self.app.emit("sync-completed", &report).ok();

        Ok(report)
    }
}

async fn get_file_modified_time(path: &PathBuf) -> Result<String, String> {
    let metadata = fs::metadata(path).await
        .map_err(|e| format!("Failed to get file metadata: {}", e))?;

    let modified = metadata.modified()
        .map_err(|e| format!("Failed to get modified time: {}", e))?;

    let datetime: chrono::DateTime<chrono::Utc> = modified.into();
    Ok(datetime.to_rfc3339())
}
