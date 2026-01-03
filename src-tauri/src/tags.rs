use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use tauri::AppHandle;
use serde::{Deserialize, Serialize};
use chrono::Utc;

use crate::config::get_diary_dir;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Tag {
    pub id: String,
    pub name: String,
    pub color: String,
    #[serde(rename = "createdAt")]
    pub created_at: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct TagsData {
    pub tags: Vec<Tag>,
    #[serde(rename = "entryTags", default)]
    pub entry_tags: HashMap<String, Vec<String>>,
}

/// Get the path to tags.json in the diary directory
pub async fn get_tags_path(app: &AppHandle) -> Result<PathBuf, String> {
    let diary_dir = get_diary_dir(app).await?;
    Ok(diary_dir.join("tags.json"))
}

/// Read tags data from file
pub async fn read_tags_data(app: &AppHandle) -> Result<TagsData, String> {
    let tags_path = get_tags_path(app).await?;

    if tags_path.exists() {
        let content = fs::read_to_string(&tags_path)
            .map_err(|e| format!("Failed to read tags: {}", e))?;
        serde_json::from_str(&content)
            .map_err(|e| format!("Invalid tags format: {}", e))
    } else {
        Ok(TagsData::default())
    }
}

/// Write tags data to file
pub async fn write_tags_data(app: &AppHandle, data: &TagsData) -> Result<(), String> {
    let tags_path = get_tags_path(app).await?;
    let json = serde_json::to_string_pretty(data)
        .map_err(|e| format!("Failed to serialize tags: {}", e))?;

    fs::write(&tags_path, json)
        .map_err(|e| format!("Failed to write tags: {}", e))?;

    Ok(())
}

/// Generate a simple unique ID (timestamp-based)
fn generate_id() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let duration = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap();
    format!("{:x}-{:x}", duration.as_secs(), duration.subsec_nanos())
}

/// Create a new tag
pub async fn create_tag(app: &AppHandle, name: String, color: String) -> Result<Tag, String> {
    let mut data = read_tags_data(app).await?;

    // Check for duplicate name
    if data.tags.iter().any(|t| t.name.to_lowercase() == name.to_lowercase()) {
        return Err("Tag with this name already exists".to_string());
    }

    let tag = Tag {
        id: generate_id(),
        name,
        color,
        created_at: Utc::now().to_rfc3339(),
    };

    data.tags.push(tag.clone());
    write_tags_data(app, &data).await?;

    Ok(tag)
}

/// Update an existing tag
pub async fn update_tag(app: &AppHandle, id: String, name: String, color: String) -> Result<Tag, String> {
    let mut data = read_tags_data(app).await?;

    // Check for duplicate name (excluding current tag)
    if data.tags.iter().any(|t| t.id != id && t.name.to_lowercase() == name.to_lowercase()) {
        return Err("Tag with this name already exists".to_string());
    }

    let tag = data.tags.iter_mut()
        .find(|t| t.id == id)
        .ok_or_else(|| "Tag not found".to_string())?;

    tag.name = name;
    tag.color = color;

    let updated_tag = tag.clone();
    write_tags_data(app, &data).await?;

    Ok(updated_tag)
}

/// Delete a tag
pub async fn delete_tag(app: &AppHandle, id: String) -> Result<(), String> {
    let mut data = read_tags_data(app).await?;

    // Remove from tags list
    data.tags.retain(|t| t.id != id);

    // Remove from all entry associations
    for tag_ids in data.entry_tags.values_mut() {
        tag_ids.retain(|tid| tid != &id);
    }

    write_tags_data(app, &data).await?;

    Ok(())
}

/// Get all tags
pub async fn get_all_tags(app: &AppHandle) -> Result<Vec<Tag>, String> {
    let data = read_tags_data(app).await?;
    Ok(data.tags)
}

/// Set tags for a diary entry
pub async fn set_entry_tags(app: &AppHandle, date: String, tag_ids: Vec<String>) -> Result<(), String> {
    let mut data = read_tags_data(app).await?;

    // Validate that all tag_ids exist
    for tid in &tag_ids {
        if !data.tags.iter().any(|t| &t.id == tid) {
            return Err(format!("Tag with id {} not found", tid));
        }
    }

    if tag_ids.is_empty() {
        data.entry_tags.remove(&date);
    } else {
        data.entry_tags.insert(date, tag_ids);
    }

    write_tags_data(app, &data).await?;

    Ok(())
}

/// Get tags for a diary entry
pub async fn get_entry_tags(app: &AppHandle, date: String) -> Result<Vec<Tag>, String> {
    let data = read_tags_data(app).await?;

    let tag_ids = data.entry_tags.get(&date).cloned().unwrap_or_default();

    let tags: Vec<Tag> = data.tags.iter()
        .filter(|t| tag_ids.contains(&t.id))
        .cloned()
        .collect();

    Ok(tags)
}

/// Get all dates that have a specific tag
pub async fn get_entries_by_tag(app: &AppHandle, tag_id: String) -> Result<Vec<String>, String> {
    let data = read_tags_data(app).await?;

    let dates: Vec<String> = data.entry_tags.iter()
        .filter(|(_, ids)| ids.contains(&tag_id))
        .map(|(date, _)| date.clone())
        .collect();

    Ok(dates)
}

/// Tag statistics data
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TagStat {
    pub tag: Tag,
    pub count: usize,
    #[serde(rename = "lastUsed")]
    pub last_used: Option<String>,
}

/// Get statistics for all tags
pub async fn get_tag_stats(app: &AppHandle) -> Result<Vec<TagStat>, String> {
    let data = read_tags_data(app).await?;

    let mut stats: Vec<TagStat> = data.tags.iter().map(|tag| {
        // Count entries with this tag
        let count = data.entry_tags.iter()
            .filter(|(_, ids)| ids.contains(&tag.id))
            .count();

        // Find the most recent date with this tag
        let last_used = data.entry_tags.iter()
            .filter(|(_, ids)| ids.contains(&tag.id))
            .map(|(date, _)| date.clone())
            .max();

        TagStat {
            tag: tag.clone(),
            count,
            last_used,
        }
    }).collect();

    // Sort by count descending
    stats.sort_by(|a, b| b.count.cmp(&a.count));

    Ok(stats)
}
