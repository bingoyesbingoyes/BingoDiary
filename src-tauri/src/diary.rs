use std::fs;
use tauri::AppHandle;
use serde::{Deserialize, Serialize};
use crate::config;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DiaryEntry {
    pub date: String,
    pub content: String,
}

// ============== Schedule Event Types ==============

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ScheduleEvent {
    pub id: String,
    pub time: String,
    #[serde(rename = "endTime", skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    pub title: String,
    pub category: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Frontmatter {
    #[serde(default)]
    pub events: Vec<ScheduleEvent>,
}

/// Parse YAML frontmatter from diary content
/// Returns (content_without_frontmatter, events)
pub fn parse_diary_with_events(content: &str) -> (String, Vec<ScheduleEvent>) {
    if content.starts_with("---") {
        // Find the closing ---
        if let Some(end_idx) = content[3..].find("\n---") {
            let yaml_content = &content[3..end_idx + 3];
            let body = &content[end_idx + 7..]; // Skip past closing ---\n

            // Try to parse YAML
            if let Ok(frontmatter) = serde_yaml::from_str::<Frontmatter>(yaml_content) {
                return (body.trim_start_matches('\n').to_string(), frontmatter.events);
            }
        }
    }
    (content.to_string(), Vec::new())
}

/// Serialize events back into frontmatter format
pub fn serialize_with_events(content: &str, events: &[ScheduleEvent]) -> String {
    if events.is_empty() {
        // Strip any existing frontmatter if no events
        let (clean_content, _) = parse_diary_with_events(content);
        return clean_content;
    }

    let frontmatter = Frontmatter { events: events.to_vec() };
    let yaml = serde_yaml::to_string(&frontmatter).unwrap_or_default();

    // Get content without any existing frontmatter
    let (clean_content, _) = parse_diary_with_events(content);

    format!("---\n{}---\n{}", yaml, clean_content)
}

/// Get events for a specific date
pub async fn get_events_for_date(date: &str, app: &AppHandle) -> Result<Vec<ScheduleEvent>, String> {
    let diary_dir = config::get_diary_dir(app).await?;
    let file_path = diary_dir.join(format!("{}.txt", date));

    if file_path.exists() {
        let content = fs::read_to_string(&file_path)
            .map_err(|e| format!("Failed to read diary: {}", e))?;
        let (_, events) = parse_diary_with_events(&content);
        Ok(events)
    } else {
        Ok(Vec::new())
    }
}

/// Save or update an event for a specific date
pub async fn save_event(date: &str, event: ScheduleEvent, app: &AppHandle) -> Result<(), String> {
    let diary_dir = config::get_diary_dir(app).await?;
    let file_path = diary_dir.join(format!("{}.txt", date));

    let (content, mut events) = if file_path.exists() {
        let raw = fs::read_to_string(&file_path)
            .map_err(|e| format!("Failed to read diary: {}", e))?;
        parse_diary_with_events(&raw)
    } else {
        (format!("{}\n", date), Vec::new())
    };

    // Update existing event or add new one
    if let Some(idx) = events.iter().position(|e| e.id == event.id) {
        events[idx] = event;
    } else {
        events.push(event);
    }

    // Sort events by time
    events.sort_by(|a, b| a.time.cmp(&b.time));

    let new_content = serialize_with_events(&content, &events);
    fs::write(&file_path, new_content)
        .map_err(|e| format!("Failed to save: {}", e))?;

    Ok(())
}

/// Delete an event by ID
pub async fn delete_event(date: &str, event_id: &str, app: &AppHandle) -> Result<(), String> {
    let diary_dir = config::get_diary_dir(app).await?;
    let file_path = diary_dir.join(format!("{}.txt", date));

    if !file_path.exists() {
        return Ok(());
    }

    let raw = fs::read_to_string(&file_path)
        .map_err(|e| format!("Failed to read diary: {}", e))?;
    let (content, mut events) = parse_diary_with_events(&raw);

    events.retain(|e| e.id != event_id);

    let new_content = serialize_with_events(&content, &events);
    fs::write(&file_path, new_content)
        .map_err(|e| format!("Failed to save: {}", e))?;

    Ok(())
}

pub async fn get_all_diaries(app: &AppHandle) -> Result<Vec<DiaryEntry>, String> {
    let diary_dir = config::get_diary_dir(app).await?;

    if !diary_dir.exists() {
        return Ok(Vec::new());
    }

    let mut diaries = Vec::new();

    for entry in fs::read_dir(&diary_dir).map_err(|e| format!("Failed to read directory: {}", e))? {
        let entry = entry.map_err(|e| format!("Failed to read entry: {}", e))?;
        let path = entry.path();

        if path.is_file() {
            if let Some(ext) = path.extension() {
                if ext == "txt" || ext == "md" {
                    if let Some(file_name) = path.file_stem() {
                        let date = file_name.to_string_lossy().to_string();
                        let content = fs::read_to_string(&path)
                            .map_err(|e| format!("Failed to read file: {}", e))?;

                        diaries.push(DiaryEntry { date, content });
                    }
                }
            }
        }
    }

    // Sort by date descending
    diaries.sort_by(|a, b| b.date.cmp(&a.date));

    Ok(diaries)
}

pub async fn save_image(
    filename: String,
    data: Vec<u8>,
    app: &AppHandle
) -> Result<String, String> {
    let diary_dir = config::get_diary_dir(app).await?;
    let images_dir = diary_dir.join("images");

    // Ensure directory exists
    if !images_dir.exists() {
        fs::create_dir_all(&images_dir)
            .map_err(|e| format!("Failed to create images directory: {}", e))?;
    }

    // Sanitize filename (keep only safe characters)
    let clean_name = filename
        .chars()
        .filter(|c| c.is_alphanumeric() || *c == '-' || *c == '_' || *c == '.')
        .collect::<String>();

    // Generate unique filename
    let timestamp = chrono::Local::now().timestamp();
    let new_filename = format!("{}-{}", timestamp, clean_name);
    let file_path = images_dir.join(&new_filename);

    // Write file
    fs::write(&file_path, data)
        .map_err(|e| format!("Failed to write image: {}", e))?;

    // Return full path
    Ok(file_path.to_string_lossy().to_string())
}

#[cfg(not(target_os = "android"))]
pub async fn export_images(app: &AppHandle) -> Result<String, String> {
    use tauri_plugin_dialog::{DialogExt, FilePath};

    let diary_dir = config::get_diary_dir(app).await?;
    let images_dir = diary_dir.join("images");

    if !images_dir.exists() {
        return Err("No images directory found".to_string());
    }

    let files: Vec<_> = fs::read_dir(&images_dir)
        .map_err(|e| format!("Failed to read images directory: {}", e))?
        .filter_map(|e| e.ok())
        .filter(|e| e.path().is_file())
        .collect();

    if files.is_empty() {
        return Err("The images directory is empty".to_string());
    }

    // Select target directory
    let target_dir = app.dialog()
        .file()
        .set_title("Select a folder to export images")
        .blocking_pick_folder();

    if let Some(FilePath::Path(target_path)) = target_dir {
        if !target_path.exists() {
            fs::create_dir_all(&target_path)
                .map_err(|e| format!("Failed to create target directory: {}", e))?;
        }

        let mut copied = 0;
        for file in files {
            let src = file.path();
            let file_name = src.file_name().unwrap();
            let dest = target_path.join(file_name);

            fs::copy(&src, &dest)
                .map_err(|e| format!("Failed to copy file: {}", e))?;
            copied += 1;
        }

        Ok(format!("Exported {} images to:\n{}", copied, target_path.display()))
    } else {
        Err("No folder selected".to_string())
    }
}

#[cfg(target_os = "android")]
pub async fn export_images(_app: &AppHandle) -> Result<String, String> {
    Err("Image export is not supported on Android".to_string())
}

#[cfg(not(target_os = "android"))]
pub async fn import_images(app: &AppHandle) -> Result<String, String> {
    use tauri_plugin_dialog::{DialogExt, FilePath};

    // Select source directory
    let source_dir = app.dialog()
        .file()
        .set_title("Select a folder containing images")
        .blocking_pick_folder();

    if let Some(FilePath::Path(source_path)) = source_dir {
        if !source_path.exists() {
            return Err("Selected directory does not exist".to_string());
        }

        let diary_dir = config::get_diary_dir(app).await?;
        let target_dir = diary_dir.join("images");

        if !target_dir.exists() {
            fs::create_dir_all(&target_dir)
                .map_err(|e| format!("Failed to create images directory: {}", e))?;
        }

        let allowed_exts = ["png", "jpg", "jpeg", "gif", "webp", "bmp"];
        let mut imported = 0;
        let mut skipped = 0;

        for entry in fs::read_dir(&source_path)
            .map_err(|e| format!("Failed to read source directory: {}", e))?
        {
            let entry = entry.map_err(|e| format!("Failed to read entry: {}", e))?;
            let path = entry.path();

            if !path.is_file() {
                skipped += 1;
                continue;
            }

            let ext = path.extension()
                .and_then(|e| e.to_str())
                .map(|e| e.to_lowercase());

            if let Some(ext_str) = ext {
                if !allowed_exts.contains(&ext_str.as_str()) {
                    skipped += 1;
                    continue;
                }
            } else {
                skipped += 1;
                continue;
            }

            // Generate target filename (avoid conflicts)
            let file_name = path.file_name().unwrap();
            let mut dest_path = target_dir.join(file_name);

            if dest_path.exists() {
                let timestamp = chrono::Local::now().timestamp();
                let new_name = format!("{}-{}", timestamp, file_name.to_string_lossy());
                dest_path = target_dir.join(new_name);
            }

            fs::copy(&path, &dest_path)
                .map_err(|e| format!("Failed to copy file: {}", e))?;
            imported += 1;
        }

        let mut message = format!("Successfully imported {} images", imported);
        if skipped > 0 {
            message.push_str(&format!("\nSkipped {} non-image files", skipped));
        }
        Ok(message)
    } else {
        Err("No folder selected".to_string())
    }
}

#[cfg(target_os = "android")]
pub async fn import_images(_app: &AppHandle) -> Result<String, String> {
    Err("Image import is not supported on Android".to_string())
}
