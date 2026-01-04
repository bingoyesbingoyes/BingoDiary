use std::fs;
use tauri::{AppHandle, Manager};
use crate::{diary, config, tags, sync};
use config::Config;
use diary::{DiaryEntry, ScheduleEvent};
use tags::{Tag, TagStat};
use sync::{SyncStatus, SyncReport, SyncSettings};

#[tauri::command]
pub async fn save_diary(date: String, content: String, app: AppHandle) -> Result<(), String> {
    let diary_dir = config::get_diary_dir(&app).await?;
    let file_path = diary_dir.join(format!("{}.txt", date));

    fs::write(&file_path, content)
        .map_err(|e| format!("Failed to save diary: {}", e))?;

    Ok(())
}

#[tauri::command]
pub async fn load_diary(date: String, app: AppHandle) -> Result<Option<String>, String> {
    let diary_dir = config::get_diary_dir(&app).await?;
    let file_path = diary_dir.join(format!("{}.txt", date));

    if file_path.exists() {
        let content = fs::read_to_string(&file_path)
            .map_err(|e| format!("Failed to read diary: {}", e))?;
        Ok(Some(content))
    } else {
        Ok(None)
    }
}

#[tauri::command]
pub async fn get_all_diaries(app: AppHandle) -> Result<Vec<DiaryEntry>, String> {
    diary::get_all_diaries(&app).await
}

#[tauri::command]
pub async fn save_image(
    filename: String,
    data: Vec<u8>,
    app: AppHandle
) -> Result<String, String> {
    diary::save_image(filename, data, &app).await
}

#[tauri::command]
pub async fn export_json(app: AppHandle) -> Result<(), String> {
    use tauri_plugin_dialog::{DialogExt, FilePath};

    // Get all diaries
    let diaries = get_all_diaries(app.clone()).await?;
    let json_data = serde_json::to_string_pretty(&diaries)
        .map_err(|e| format!("Failed to serialize: {}", e))?;

    // Show save dialog
    let file_path = app.dialog()
        .file()
        .set_title("Export diaries as JSON")
        .set_file_name("diaries.json")
        .add_filter("JSON Files", &["json"])
        .blocking_save_file();

    if let Some(FilePath::Path(path)) = file_path {
        fs::write(path, json_data)
            .map_err(|e| format!("Failed to write file: {}", e))?;
    }

    Ok(())
}

#[tauri::command]
pub async fn import_json(app: AppHandle) -> Result<String, String> {
    use tauri_plugin_dialog::{DialogExt, FilePath};

    // Show open dialog
    let file_path = app.dialog()
        .file()
        .set_title("Select a JSON file")
        .add_filter("JSON Files", &["json"])
        .blocking_pick_file();

    if let Some(FilePath::Path(path)) = file_path {
        let content = fs::read_to_string(&path)
            .map_err(|e| format!("Failed to read file: {}", e))?;

        let diaries: Vec<DiaryEntry> = serde_json::from_str(&content)
            .map_err(|e| format!("Invalid JSON format: {}", e))?;

        // Import each diary entry
        let diary_dir = config::get_diary_dir(&app).await?;
        let mut imported = 0;

        for entry in diaries {
            let file_path = diary_dir.join(format!("{}.txt", entry.date));
            if fs::write(&file_path, &entry.content).is_ok() {
                imported += 1;
            }
        }

        Ok(format!("Successfully imported {} diaries", imported))
    } else {
        Err("No file selected".to_string())
    }
}

#[tauri::command]
pub async fn export_images(app: AppHandle) -> Result<String, String> {
    diary::export_images(&app).await
}

#[tauri::command]
pub async fn import_images(app: AppHandle) -> Result<String, String> {
    diary::import_images(&app).await
}

#[tauri::command]
pub async fn get_storage_path(app: AppHandle) -> Result<String, String> {
    let diary_dir = config::get_diary_dir(&app).await?;
    Ok(diary_dir.to_string_lossy().to_string())
}

#[tauri::command]
pub async fn change_storage_path(app: AppHandle) -> Result<String, String> {
    config::change_storage_path(&app).await
}

#[tauri::command]
pub async fn get_app_data_dir(app: AppHandle) -> Result<String, String> {
    let app_data = app.path().app_data_dir()
        .map_err(|e| format!("Failed to get app data dir: {}", e))?;
    Ok(app_data.to_string_lossy().to_string())
}

#[tauri::command]
pub async fn export_pdf(app: AppHandle) -> Result<(), String> {
    use tauri_plugin_dialog::{DialogExt, FilePath};
    use crate::pdf;

    // Get diary directory for resolving relative image paths
    let diary_dir = config::get_diary_dir(&app).await?;

    // Get all diaries
    let diaries = get_all_diaries(app.clone()).await?;

    // Build markdown content (oldest first)
    let mut markdown = String::new();
    let mut valid_entries: Vec<_> = diaries.iter().filter_map(|entry| {
        let mut content = entry.content.clone();
        let first_line = content.lines().next().unwrap_or("").trim();

        // Remove date from first line if present
        if first_line == entry.date || first_line.chars().all(|c| c.is_ascii_digit() || c == '-') {
            content = content.lines().skip(1).collect::<Vec<_>>().join("\n").trim().to_string();
        }

        if content.is_empty() {
            None
        } else {
            Some((entry.date.clone(), content))
        }
    }).collect();
    valid_entries.reverse(); // Sort oldest first for export

    let total = valid_entries.len();
    for (i, (date, content)) in valid_entries.into_iter().enumerate() {
        markdown.push_str(&format!("## {}\n\n{}\n\n", date, content));
        // Add separator between entries, but not after the last one
        if i < total - 1 {
            markdown.push_str("---\n\n");
        }
    }

    // Show save dialog
    let file_path = app.dialog()
        .file()
        .set_title("Export diaries as PDF")
        .set_file_name("diaries.pdf")
        .add_filter("PDF Files", &["pdf"])
        .blocking_save_file();

    if let Some(FilePath::Path(path)) = file_path {
        pdf::export_to_pdf(&markdown, &path, Some(&diary_dir))?;
    }

    Ok(())
}

// ============== Settings Commands ==============

#[tauri::command]
pub fn get_config(app: AppHandle) -> Result<Config, String> {
    config::read_config(&app)
}

#[tauri::command]
pub fn save_config(
    diary_dir: Option<String>,
    bg_type: Option<String>,
    bg_value: Option<String>,
    language: Option<String>,
    app: AppHandle
) -> Result<(), String> {
    let mut cfg = config::read_config(&app)?;
    cfg.diary_dir = diary_dir;
    cfg.bg_type = bg_type;
    cfg.bg_value = bg_value;
    cfg.language = language;
    config::write_config(&app, &cfg)
}

#[tauri::command]
pub fn set_password(password: String, app: AppHandle) -> Result<(), String> {
    use bcrypt::{hash, DEFAULT_COST};

    let hashed = hash(password, DEFAULT_COST)
        .map_err(|e| format!("Failed to hash password: {}", e))?;

    let mut cfg = config::read_config(&app)?;
    cfg.password_hash = Some(hashed);
    config::write_config(&app, &cfg)
}

#[tauri::command]
pub fn verify_password(password: String, app: AppHandle) -> Result<bool, String> {
    use bcrypt::verify;

    let cfg = config::read_config(&app)?;

    match cfg.password_hash {
        Some(hash) => {
            verify(password, &hash)
                .map_err(|e| format!("Failed to verify password: {}", e))
        }
        None => Ok(true) // No password set, always pass
    }
}

#[tauri::command]
pub fn clear_password(app: AppHandle) -> Result<(), String> {
    let mut cfg = config::read_config(&app)?;
    cfg.password_hash = None;
    config::write_config(&app, &cfg)
}

#[tauri::command]
pub fn has_password(app: AppHandle) -> Result<bool, String> {
    let cfg = config::read_config(&app)?;
    Ok(cfg.password_hash.is_some())
}

#[tauri::command]
pub fn save_background_image(
    data: Vec<u8>,
    ext: String,
    app: AppHandle
) -> Result<String, String> {
    config::save_bg_image(&app, &data, &ext)
}

// ============== Tag Commands ==============

#[tauri::command]
pub async fn create_tag(name: String, color: String, app: AppHandle) -> Result<Tag, String> {
    tags::create_tag(&app, name, color).await
}

#[tauri::command]
pub async fn update_tag(id: String, name: String, color: String, app: AppHandle) -> Result<Tag, String> {
    tags::update_tag(&app, id, name, color).await
}

#[tauri::command]
pub async fn delete_tag(id: String, app: AppHandle) -> Result<(), String> {
    tags::delete_tag(&app, id).await
}

#[tauri::command]
pub async fn get_all_tags(app: AppHandle) -> Result<Vec<Tag>, String> {
    tags::get_all_tags(&app).await
}

#[tauri::command]
pub async fn set_entry_tags(date: String, tag_ids: Vec<String>, app: AppHandle) -> Result<(), String> {
    tags::set_entry_tags(&app, date, tag_ids).await
}

#[tauri::command]
pub async fn get_entry_tags(date: String, app: AppHandle) -> Result<Vec<Tag>, String> {
    tags::get_entry_tags(&app, date).await
}

#[tauri::command]
pub async fn get_entries_by_tag(tag_id: String, app: AppHandle) -> Result<Vec<String>, String> {
    tags::get_entries_by_tag(&app, tag_id).await
}

#[tauri::command]
pub async fn get_tag_stats(app: AppHandle) -> Result<Vec<TagStat>, String> {
    tags::get_tag_stats(&app).await
}

// ============== Schedule Event Commands ==============

#[tauri::command]
pub async fn get_events_for_date(date: String, app: AppHandle) -> Result<Vec<ScheduleEvent>, String> {
    diary::get_events_for_date(&date, &app).await
}

#[tauri::command]
pub async fn save_event(date: String, event: ScheduleEvent, app: AppHandle) -> Result<(), String> {
    diary::save_event(&date, event, &app).await
}

#[tauri::command]
pub async fn delete_event(date: String, event_id: String, app: AppHandle) -> Result<(), String> {
    diary::delete_event(&date, &event_id, &app).await
}

// ============== Sync Commands ==============

#[tauri::command]
pub async fn get_sync_status(app: AppHandle) -> Result<SyncStatus, String> {
    sync::get_status(&app).await
}

#[tauri::command]
pub async fn start_sync(app: AppHandle) -> Result<SyncReport, String> {
    let mut auth = sync::GoogleAuth::load(&app)?;

    if !auth.is_authenticated() {
        return Err("Not authenticated with Google".to_string());
    }

    let access_token = auth.get_valid_access_token().await?;
    let diary_dir = config::get_diary_dir(&app).await?;

    let engine = sync::SyncEngine::new(app.clone(), diary_dir);
    engine.sync(&access_token).await
}

#[tauri::command]
pub async fn get_google_auth_url(is_mobile: bool, app: AppHandle) -> Result<String, String> {
    let auth = sync::GoogleAuth::load(&app)?;
    auth.get_auth_url(is_mobile).await
}

#[tauri::command]
pub async fn handle_oauth_callback(code: String, is_mobile: bool, app: AppHandle) -> Result<(), String> {
    let mut auth = sync::GoogleAuth::load(&app)?;
    auth.exchange_code(&code, is_mobile).await
}

#[tauri::command]
pub async fn disconnect_google(app: AppHandle) -> Result<(), String> {
    let mut auth = sync::GoogleAuth::load(&app)?;
    auth.disconnect()
}

#[tauri::command]
pub async fn get_sync_settings(app: AppHandle) -> Result<SyncSettings, String> {
    sync::get_settings(&app).await
}

#[tauri::command]
pub async fn save_sync_settings(settings: SyncSettings, app: AppHandle) -> Result<(), String> {
    sync::save_settings(&app, settings).await
}

#[tauri::command]
pub async fn save_google_client_id(client_id: String, app: AppHandle) -> Result<(), String> {
    let auth = sync::GoogleAuth::load(&app)?;
    auth.save_client_id(&client_id)
}

#[tauri::command]
pub async fn save_google_credentials(client_id: String, client_secret: Option<String>, app: AppHandle) -> Result<(), String> {
    let auth = sync::GoogleAuth::load(&app)?;
    auth.save_client_credentials(&client_id, client_secret.as_deref())
}

#[tauri::command]
pub async fn get_google_client_id(app: AppHandle) -> Result<Option<String>, String> {
    let auth = sync::GoogleAuth::load(&app)?;
    let client_id = auth.get_client_id().to_string();
    if client_id == "YOUR_CLIENT_ID.apps.googleusercontent.com" {
        Ok(None)
    } else {
        Ok(Some(client_id))
    }
}

#[tauri::command]
pub async fn clear_sync_credentials(app: AppHandle) -> Result<(), String> {
    let mut auth = sync::GoogleAuth::load(&app)?;
    auth.clear_all_credentials()
}

// Desktop-only: Start OAuth callback server
#[cfg(desktop)]
#[tauri::command]
pub async fn start_oauth_callback_server() -> Result<String, String> {
    // Run in blocking thread pool since it uses sync I/O
    tokio::task::spawn_blocking(|| {
        sync::auth::callback_server::start_callback_server()
    })
    .await
    .map_err(|e| format!("Task failed: {}", e))?
}
