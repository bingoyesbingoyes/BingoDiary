use reqwest::{multipart, Client};
use serde::{Deserialize, Serialize};
use std::path::Path;
use std::time::Duration;
use tokio::fs;

const DRIVE_API_BASE: &str = "https://www.googleapis.com/drive/v3";
const UPLOAD_API_BASE: &str = "https://www.googleapis.com/upload/drive/v3";

// Timeout settings
const REQUEST_TIMEOUT_SECS: u64 = 30;
const CONNECT_TIMEOUT_SECS: u64 = 10;

// App folder name in Google Drive
pub const APP_FOLDER_NAME: &str = "BingoDiary";
pub const ENTRIES_FOLDER_NAME: &str = "entries";
pub const IMAGES_FOLDER_NAME: &str = "images";

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DriveFile {
    pub id: String,
    pub name: String,
    pub mime_type: Option<String>,
    pub modified_time: Option<String>,
    pub size: Option<String>,
    pub md5_checksum: Option<String>,
    pub parents: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct FileListResponse {
    files: Vec<DriveFile>,
    next_page_token: Option<String>,
}

pub struct DriveClient {
    http_client: Client,
    access_token: String,
}

impl DriveClient {
    pub fn new(access_token: String) -> Self {
        let http_client = Client::builder()
            .timeout(Duration::from_secs(REQUEST_TIMEOUT_SECS))
            .connect_timeout(Duration::from_secs(CONNECT_TIMEOUT_SECS))
            .build()
            .unwrap_or_else(|_| Client::new());

        Self {
            http_client,
            access_token,
        }
    }

    fn auth_header(&self) -> String {
        format!("Bearer {}", self.access_token)
    }

    /// Find a folder by name under a parent (or root if parent is None)
    pub async fn find_folder(&self, name: &str, parent_id: Option<&str>) -> Result<Option<DriveFile>, String> {
        let parent_query = match parent_id {
            Some(id) => format!("'{}' in parents", id),
            None => "'root' in parents".to_string(),
        };

        let query = format!(
            "name = '{}' and mimeType = 'application/vnd.google-apps.folder' and {} and trashed = false",
            name, parent_query
        );

        let url = format!(
            "{}/files?q={}&fields=files(id,name,mimeType,modifiedTime)",
            DRIVE_API_BASE,
            urlencoding::encode(&query)
        );

        let response = self.http_client
            .get(&url)
            .header("Authorization", self.auth_header())
            .send()
            .await
            .map_err(|e| format!("Failed to search for folder: {}", e))?;

        if !response.status().is_success() {
            let error = response.text().await.unwrap_or_default();
            return Err(format!("Drive API error: {}", error));
        }

        let list: FileListResponse = response.json().await
            .map_err(|e| format!("Failed to parse response: {}", e))?;

        Ok(list.files.into_iter().next())
    }

    /// Create a folder
    pub async fn create_folder(&self, name: &str, parent_id: Option<&str>) -> Result<DriveFile, String> {
        let mut metadata = serde_json::json!({
            "name": name,
            "mimeType": "application/vnd.google-apps.folder"
        });

        if let Some(id) = parent_id {
            metadata["parents"] = serde_json::json!([id]);
        }

        let response = self.http_client
            .post(&format!("{}/files", DRIVE_API_BASE))
            .header("Authorization", self.auth_header())
            .header("Content-Type", "application/json")
            .body(metadata.to_string())
            .send()
            .await
            .map_err(|e| format!("Failed to create folder: {}", e))?;

        if !response.status().is_success() {
            let error = response.text().await.unwrap_or_default();
            return Err(format!("Failed to create folder: {}", error));
        }

        response.json().await
            .map_err(|e| format!("Failed to parse response: {}", e))
    }

    /// Ensure app folder structure exists, returns (app_folder_id, entries_folder_id, images_folder_id)
    pub async fn ensure_folder_structure(&self) -> Result<(String, String, String), String> {
        // Find or create main app folder
        let app_folder = match self.find_folder(APP_FOLDER_NAME, None).await? {
            Some(folder) => folder,
            None => self.create_folder(APP_FOLDER_NAME, None).await?,
        };

        // Find or create entries subfolder
        let entries_folder = match self.find_folder(ENTRIES_FOLDER_NAME, Some(&app_folder.id)).await? {
            Some(folder) => folder,
            None => self.create_folder(ENTRIES_FOLDER_NAME, Some(&app_folder.id)).await?,
        };

        // Find or create images subfolder
        let images_folder = match self.find_folder(IMAGES_FOLDER_NAME, Some(&app_folder.id)).await? {
            Some(folder) => folder,
            None => self.create_folder(IMAGES_FOLDER_NAME, Some(&app_folder.id)).await?,
        };

        Ok((app_folder.id, entries_folder.id, images_folder.id))
    }

    /// List all files in a folder
    pub async fn list_files(&self, folder_id: &str) -> Result<Vec<DriveFile>, String> {
        let mut all_files = Vec::new();
        let mut page_token: Option<String> = None;

        loop {
            let query = format!("'{}' in parents and trashed = false", folder_id);
            let mut url = format!(
                "{}/files?q={}&fields=files(id,name,mimeType,modifiedTime,size,md5Checksum)&pageSize=1000",
                DRIVE_API_BASE,
                urlencoding::encode(&query)
            );

            if let Some(ref token) = page_token {
                url.push_str(&format!("&pageToken={}", token));
            }

            let response = self.http_client
                .get(&url)
                .header("Authorization", self.auth_header())
                .send()
                .await
                .map_err(|e| format!("Failed to list files: {}", e))?;

            if !response.status().is_success() {
                let error = response.text().await.unwrap_or_default();
                return Err(format!("Drive API error: {}", error));
            }

            let list: FileListResponse = response.json().await
                .map_err(|e| format!("Failed to parse response: {}", e))?;

            all_files.extend(list.files);

            match list.next_page_token {
                Some(token) => page_token = Some(token),
                None => break,
            }
        }

        Ok(all_files)
    }

    /// Upload a file to Drive
    pub async fn upload_file(
        &self,
        local_path: &Path,
        file_name: &str,
        folder_id: &str,
        existing_file_id: Option<&str>,
    ) -> Result<DriveFile, String> {
        let content = fs::read(local_path).await
            .map_err(|e| format!("Failed to read local file: {}", e))?;

        let mime_type = mime_guess::from_path(local_path)
            .first_or_octet_stream()
            .to_string();

        if let Some(file_id) = existing_file_id {
            // Update existing file
            self.update_file_content(file_id, &content, &mime_type).await
        } else {
            // Create new file
            self.create_file(file_name, folder_id, &content, &mime_type).await
        }
    }

    /// Upload content directly (for text files)
    pub async fn upload_content(
        &self,
        content: &[u8],
        file_name: &str,
        folder_id: &str,
        mime_type: &str,
        existing_file_id: Option<&str>,
    ) -> Result<DriveFile, String> {
        if let Some(file_id) = existing_file_id {
            self.update_file_content(file_id, content, mime_type).await
        } else {
            self.create_file(file_name, folder_id, content, mime_type).await
        }
    }

    async fn create_file(
        &self,
        file_name: &str,
        folder_id: &str,
        content: &[u8],
        mime_type: &str,
    ) -> Result<DriveFile, String> {
        let metadata = serde_json::json!({
            "name": file_name,
            "parents": [folder_id]
        });

        let form = multipart::Form::new()
            .part(
                "metadata",
                multipart::Part::text(metadata.to_string())
                    .mime_str("application/json")
                    .map_err(|e| e.to_string())?,
            )
            .part(
                "file",
                multipart::Part::bytes(content.to_vec())
                    .mime_str(mime_type)
                    .map_err(|e| e.to_string())?,
            );

        let url = format!(
            "{}/files?uploadType=multipart&fields=id,name,mimeType,modifiedTime,size,md5Checksum",
            UPLOAD_API_BASE
        );

        let response = self.http_client
            .post(&url)
            .header("Authorization", self.auth_header())
            .multipart(form)
            .send()
            .await
            .map_err(|e| format!("Failed to upload file: {}", e))?;

        if !response.status().is_success() {
            let error = response.text().await.unwrap_or_default();
            return Err(format!("Upload failed: {}", error));
        }

        response.json().await
            .map_err(|e| format!("Failed to parse response: {}", e))
    }

    async fn update_file_content(
        &self,
        file_id: &str,
        content: &[u8],
        mime_type: &str,
    ) -> Result<DriveFile, String> {
        let url = format!(
            "{}/files/{}?uploadType=media&fields=id,name,mimeType,modifiedTime,size,md5Checksum",
            UPLOAD_API_BASE, file_id
        );

        let response = self.http_client
            .patch(&url)
            .header("Authorization", self.auth_header())
            .header("Content-Type", mime_type)
            .body(content.to_vec())
            .send()
            .await
            .map_err(|e| format!("Failed to update file: {}", e))?;

        if !response.status().is_success() {
            let error = response.text().await.unwrap_or_default();
            return Err(format!("Update failed: {}", error));
        }

        response.json().await
            .map_err(|e| format!("Failed to parse response: {}", e))
    }

    /// Download a file from Drive
    pub async fn download_file(&self, file_id: &str) -> Result<Vec<u8>, String> {
        let url = format!("{}/files/{}?alt=media", DRIVE_API_BASE, file_id);

        let response = self.http_client
            .get(&url)
            .header("Authorization", self.auth_header())
            .send()
            .await
            .map_err(|e| format!("Failed to download file: {}", e))?;

        if !response.status().is_success() {
            let error = response.text().await.unwrap_or_default();
            return Err(format!("Download failed: {}", error));
        }

        response.bytes().await
            .map(|b| b.to_vec())
            .map_err(|e| format!("Failed to read response: {}", e))
    }

    /// Delete a file from Drive
    pub async fn delete_file(&self, file_id: &str) -> Result<(), String> {
        let url = format!("{}/files/{}", DRIVE_API_BASE, file_id);

        let response = self.http_client
            .delete(&url)
            .header("Authorization", self.auth_header())
            .send()
            .await
            .map_err(|e| format!("Failed to delete file: {}", e))?;

        if !response.status().is_success() && response.status().as_u16() != 404 {
            let error = response.text().await.unwrap_or_default();
            return Err(format!("Delete failed: {}", error));
        }

        Ok(())
    }

    /// Get file metadata
    pub async fn get_file_metadata(&self, file_id: &str) -> Result<DriveFile, String> {
        let url = format!(
            "{}/files/{}?fields=id,name,mimeType,modifiedTime,size,md5Checksum,parents",
            DRIVE_API_BASE, file_id
        );

        let response = self.http_client
            .get(&url)
            .header("Authorization", self.auth_header())
            .send()
            .await
            .map_err(|e| format!("Failed to get file metadata: {}", e))?;

        if !response.status().is_success() {
            let error = response.text().await.unwrap_or_default();
            return Err(format!("Get metadata failed: {}", error));
        }

        response.json().await
            .map_err(|e| format!("Failed to parse response: {}", e))
    }

    /// Find a file by name in a folder
    pub async fn find_file(&self, name: &str, folder_id: &str) -> Result<Option<DriveFile>, String> {
        let query = format!(
            "name = '{}' and '{}' in parents and trashed = false",
            name, folder_id
        );

        let url = format!(
            "{}/files?q={}&fields=files(id,name,mimeType,modifiedTime,size,md5Checksum)",
            DRIVE_API_BASE,
            urlencoding::encode(&query)
        );

        let response = self.http_client
            .get(&url)
            .header("Authorization", self.auth_header())
            .send()
            .await
            .map_err(|e| format!("Failed to search for file: {}", e))?;

        if !response.status().is_success() {
            let error = response.text().await.unwrap_or_default();
            return Err(format!("Drive API error: {}", error));
        }

        let list: FileListResponse = response.json().await
            .map_err(|e| format!("Failed to parse response: {}", e))?;

        Ok(list.files.into_iter().next())
    }
}
