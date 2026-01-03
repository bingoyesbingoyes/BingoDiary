use oauth2::{
    AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken, PkceCodeChallenge, PkceCodeVerifier,
    RedirectUrl, Scope, TokenResponse, TokenUrl,
    basic::BasicClient,
    reqwest::async_http_client,
};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use std::sync::Arc;
use tauri::{AppHandle, Manager};
use tokio::sync::Mutex;

// Google OAuth2 endpoints
const AUTH_URL: &str = "https://accounts.google.com/o/oauth2/v2/auth";
const TOKEN_URL: &str = "https://oauth2.googleapis.com/token";

// Google Drive API scope - only access files created by this app
const DRIVE_SCOPE: &str = "https://www.googleapis.com/auth/drive.file";
const EMAIL_SCOPE: &str = "https://www.googleapis.com/auth/userinfo.email";

// Default localhost port for desktop OAuth callback
const DESKTOP_REDIRECT_PORT: u16 = 8234;

// Replace with your Google Cloud Console Client ID
// This should be configured by the user or stored in a config file
const DEFAULT_CLIENT_ID: &str = "YOUR_CLIENT_ID.apps.googleusercontent.com";

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TokenSet {
    pub access_token: String,
    pub refresh_token: Option<String>,
    pub expires_at: Option<i64>,
    pub email: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct AuthConfig {
    pub client_id: Option<String>,
    pub client_secret: Option<String>,
}

pub struct GoogleAuth {
    client_id: String,
    client_secret: Option<String>,
    tokens: Option<TokenSet>,
    pkce_verifier: Arc<Mutex<Option<PkceCodeVerifier>>>,
    app_data_dir: PathBuf,
}

impl GoogleAuth {
    pub fn new(app: &AppHandle) -> Result<Self, String> {
        let app_data_dir = app.path().app_data_dir()
            .map_err(|e| e.to_string())?;
        fs::create_dir_all(&app_data_dir).map_err(|e| e.to_string())?;

        let auth_config = Self::load_auth_config(&app_data_dir);
        let client_id = auth_config.client_id.unwrap_or_else(|| DEFAULT_CLIENT_ID.to_string());
        let client_secret = auth_config.client_secret;

        let tokens = Self::load_tokens(&app_data_dir);

        // Load PKCE verifier from file if exists
        let pkce_verifier = Self::load_pkce_verifier(&app_data_dir);

        Ok(Self {
            client_id,
            client_secret,
            tokens,
            pkce_verifier: Arc::new(Mutex::new(pkce_verifier)),
            app_data_dir,
        })
    }

    pub fn load(app: &AppHandle) -> Result<Self, String> {
        Self::new(app)
    }

    fn get_pkce_path(app_data_dir: &PathBuf) -> PathBuf {
        app_data_dir.join("pkce_verifier.txt")
    }

    fn load_pkce_verifier(app_data_dir: &PathBuf) -> Option<PkceCodeVerifier> {
        let path = Self::get_pkce_path(app_data_dir);
        fs::read_to_string(&path)
            .ok()
            .map(|s| PkceCodeVerifier::new(s.trim().to_string()))
    }

    fn save_pkce_verifier(&self, verifier: &PkceCodeVerifier) -> Result<(), String> {
        let path = Self::get_pkce_path(&self.app_data_dir);
        fs::write(&path, verifier.secret()).map_err(|e| e.to_string())
    }

    fn clear_pkce_verifier(&self) {
        let path = Self::get_pkce_path(&self.app_data_dir);
        let _ = fs::remove_file(&path);
    }

    fn load_auth_config(app_data_dir: &PathBuf) -> AuthConfig {
        let config_path = app_data_dir.join("google_auth_config.json");
        if config_path.exists() {
            fs::read_to_string(&config_path)
                .ok()
                .and_then(|content| serde_json::from_str(&content).ok())
                .unwrap_or_default()
        } else {
            AuthConfig::default()
        }
    }

    pub fn save_client_id(&self, client_id: &str) -> Result<(), String> {
        // Load existing config to preserve client_secret if any
        let mut config = Self::load_auth_config(&self.app_data_dir);
        config.client_id = Some(client_id.to_string());
        let config_path = self.app_data_dir.join("google_auth_config.json");
        let content = serde_json::to_string_pretty(&config)
            .map_err(|e| e.to_string())?;
        fs::write(&config_path, content).map_err(|e| e.to_string())
    }

    pub fn save_client_credentials(&self, client_id: &str, client_secret: Option<&str>) -> Result<(), String> {
        let config = AuthConfig {
            client_id: Some(client_id.to_string()),
            client_secret: client_secret.map(|s| s.to_string()),
        };
        let config_path = self.app_data_dir.join("google_auth_config.json");
        let content = serde_json::to_string_pretty(&config)
            .map_err(|e| e.to_string())?;
        fs::write(&config_path, content).map_err(|e| e.to_string())
    }

    fn get_tokens_path(app_data_dir: &PathBuf) -> PathBuf {
        app_data_dir.join("google_auth.json")
    }

    fn load_tokens(app_data_dir: &PathBuf) -> Option<TokenSet> {
        let path = Self::get_tokens_path(app_data_dir);
        if path.exists() {
            fs::read_to_string(&path)
                .ok()
                .and_then(|content| serde_json::from_str(&content).ok())
        } else {
            None
        }
    }

    fn save_tokens(&self) -> Result<(), String> {
        let path = Self::get_tokens_path(&self.app_data_dir);
        if let Some(ref tokens) = self.tokens {
            let content = serde_json::to_string_pretty(tokens)
                .map_err(|e| e.to_string())?;
            fs::write(&path, content).map_err(|e| e.to_string())
        } else {
            // Delete tokens file if disconnecting
            if path.exists() {
                fs::remove_file(&path).map_err(|e| e.to_string())
            } else {
                Ok(())
            }
        }
    }

    pub fn is_authenticated(&self) -> bool {
        self.tokens.is_some()
    }

    pub fn get_email(&self) -> Option<String> {
        self.tokens.as_ref().and_then(|t| t.email.clone())
    }

    pub fn get_client_id(&self) -> &str {
        &self.client_id
    }

    pub fn set_client_id(&mut self, client_id: String) {
        self.client_id = client_id;
    }

    fn create_oauth_client(&self, _is_mobile: bool) -> Result<BasicClient, String> {
        // Always use localhost redirect - works for both desktop and mobile
        let redirect_url = format!("http://localhost:{}/callback", DESKTOP_REDIRECT_PORT);

        // Use client_secret if available (required for Desktop apps)
        let client_secret = self.client_secret.as_ref()
            .map(|s| ClientSecret::new(s.clone()));

        let client = BasicClient::new(
            ClientId::new(self.client_id.clone()),
            client_secret,
            AuthUrl::new(AUTH_URL.to_string()).map_err(|e| e.to_string())?,
            Some(TokenUrl::new(TOKEN_URL.to_string()).map_err(|e| e.to_string())?),
        )
        .set_redirect_uri(RedirectUrl::new(redirect_url).map_err(|e| e.to_string())?);

        Ok(client)
    }

    pub async fn get_auth_url(&self, is_mobile: bool) -> Result<String, String> {
        let client = self.create_oauth_client(is_mobile)?;

        // Generate PKCE challenge
        let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();

        // Store verifier for later use - both in memory and on disk
        println!("[OAuth] Saving PKCE verifier...");
        self.save_pkce_verifier(&pkce_verifier)?;
        {
            let mut verifier_guard = self.pkce_verifier.lock().await;
            *verifier_guard = Some(pkce_verifier);
        }

        // Build authorization URL
        let (auth_url, _csrf_token) = client
            .authorize_url(CsrfToken::new_random)
            .add_scope(Scope::new(DRIVE_SCOPE.to_string()))
            .add_scope(Scope::new(EMAIL_SCOPE.to_string()))
            .set_pkce_challenge(pkce_challenge)
            .add_extra_param("access_type", "offline") // Request refresh token
            .add_extra_param("prompt", "consent") // Force consent to get refresh token
            .url();

        println!("[OAuth] Auth URL generated");
        Ok(auth_url.to_string())
    }

    pub async fn exchange_code(&mut self, code: &str, is_mobile: bool) -> Result<(), String> {
        println!("[OAuth] Starting token exchange...");
        let client = self.create_oauth_client(is_mobile)?;

        // Get stored PKCE verifier
        let pkce_verifier = {
            let mut verifier_guard = self.pkce_verifier.lock().await;
            verifier_guard.take().ok_or_else(|| {
                println!("[OAuth] Error: No PKCE verifier found");
                "No PKCE verifier found".to_string()
            })?
        };
        println!("[OAuth] Got PKCE verifier: {}...", &pkce_verifier.secret()[..pkce_verifier.secret().len().min(10)]);

        // Exchange authorization code for tokens
        println!("[OAuth] Exchanging code for tokens...");
        println!("[OAuth] Code (first 20 chars): {}...", &code[..code.len().min(20)]);
        let token_result = client
            .exchange_code(AuthorizationCode::new(code.to_string()))
            .set_pkce_verifier(pkce_verifier)
            .request_async(async_http_client)
            .await
            .map_err(|e| {
                // Get detailed error info
                let error_msg = match &e {
                    oauth2::RequestTokenError::ServerResponse(resp) => {
                        format!("Server error: {:?}, description: {:?}",
                            resp.error(),
                            resp.error_description())
                    },
                    oauth2::RequestTokenError::Request(req_err) => {
                        format!("Request error: {}", req_err)
                    },
                    oauth2::RequestTokenError::Parse(parse_err, body) => {
                        format!("Parse error: {}, body: {:?}", parse_err,
                            String::from_utf8_lossy(body))
                    },
                    oauth2::RequestTokenError::Other(msg) => {
                        format!("Other error: {}", msg)
                    },
                };
                println!("[OAuth] Token exchange failed: {}", error_msg);
                format!("Token exchange failed: {}", error_msg)
            })?;
        println!("[OAuth] Token exchange successful");

        // Calculate expiry time
        let expires_at = token_result.expires_in().map(|duration| {
            chrono::Utc::now().timestamp() + duration.as_secs() as i64
        });

        // Fetch user email
        let email = self.fetch_user_email(token_result.access_token().secret()).await.ok();

        self.tokens = Some(TokenSet {
            access_token: token_result.access_token().secret().clone(),
            refresh_token: token_result.refresh_token().map(|t| t.secret().clone()),
            expires_at,
            email: email.clone(),
        });

        println!("[OAuth] Saving tokens (email: {:?})...", email);
        let result = self.save_tokens();
        println!("[OAuth] Token save result: {:?}", result.is_ok());

        // Clear PKCE verifier file after successful exchange
        self.clear_pkce_verifier();

        result
    }

    async fn fetch_user_email(&self, access_token: &str) -> Result<String, String> {
        let client = reqwest::Client::new();
        let response = client
            .get("https://www.googleapis.com/oauth2/v2/userinfo")
            .bearer_auth(access_token)
            .send()
            .await
            .map_err(|e| e.to_string())?;

        #[derive(Deserialize)]
        struct UserInfo {
            email: String,
        }

        let user_info: UserInfo = response.json().await.map_err(|e| e.to_string())?;
        Ok(user_info.email)
    }

    pub async fn get_valid_access_token(&mut self) -> Result<String, String> {
        let tokens = self.tokens.as_ref().ok_or("Not authenticated")?;

        // Check if token is expired or will expire in next 5 minutes
        if let Some(expires_at) = tokens.expires_at {
            let now = chrono::Utc::now().timestamp();
            if now >= expires_at - 300 {
                // Token expired or expiring soon, refresh it
                self.refresh_token().await?;
            }
        }

        self.tokens
            .as_ref()
            .map(|t| t.access_token.clone())
            .ok_or_else(|| "No access token available".to_string())
    }

    async fn refresh_token(&mut self) -> Result<(), String> {
        let tokens = self.tokens.as_ref().ok_or("Not authenticated")?;
        let refresh_token = tokens.refresh_token.as_ref()
            .ok_or("No refresh token available")?;

        let client = reqwest::Client::new();

        // Build params - include client_secret if available (required for Desktop apps)
        let mut params = vec![
            ("client_id", self.client_id.as_str()),
            ("refresh_token", refresh_token.as_str()),
            ("grant_type", "refresh_token"),
        ];

        // For Desktop OAuth clients, client_secret is required for token refresh
        let client_secret_owned = self.client_secret.clone();
        if let Some(ref secret) = client_secret_owned {
            params.push(("client_secret", secret.as_str()));
        }

        let response = client
            .post(TOKEN_URL)
            .form(&params)
            .send()
            .await
            .map_err(|e| format!("Refresh request failed: {}", e))?;

        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_default();
            return Err(format!("Token refresh failed: {}", error_text));
        }

        #[derive(Deserialize)]
        struct RefreshResponse {
            access_token: String,
            expires_in: Option<u64>,
        }

        let refresh_response: RefreshResponse = response.json().await
            .map_err(|e| format!("Failed to parse refresh response: {}", e))?;

        let expires_at = refresh_response.expires_in.map(|duration| {
            chrono::Utc::now().timestamp() + duration as i64
        });

        // Update tokens (keep existing refresh token and email)
        if let Some(ref mut tokens) = self.tokens {
            tokens.access_token = refresh_response.access_token;
            tokens.expires_at = expires_at;
        }

        self.save_tokens()
    }

    pub fn disconnect(&mut self) -> Result<(), String> {
        self.tokens = None;
        self.save_tokens()
    }

    pub fn clear_all_credentials(&mut self) -> Result<(), String> {
        // Clear tokens
        self.tokens = None;
        let tokens_path = Self::get_tokens_path(&self.app_data_dir);
        if tokens_path.exists() {
            fs::remove_file(&tokens_path).ok();
        }

        // Clear auth config (client_id, client_secret)
        let config_path = self.app_data_dir.join("google_auth_config.json");
        if config_path.exists() {
            fs::remove_file(&config_path).ok();
        }

        // Clear PKCE verifier
        self.clear_pkce_verifier();

        // Reset to defaults
        self.client_id = DEFAULT_CLIENT_ID.to_string();
        self.client_secret = None;

        Ok(())
    }
}

// Desktop-only: HTTP server for OAuth callback
#[cfg(desktop)]
pub mod callback_server {
    use super::DESKTOP_REDIRECT_PORT;
    use std::io::{Read, Write};
    use std::net::TcpListener;
    use std::time::Duration;

    pub fn start_callback_server() -> Result<String, String> {
        println!("[OAuth] Starting callback server on port {}...", DESKTOP_REDIRECT_PORT);

        // Try binding to 0.0.0.0 first (accepts connections on all interfaces including localhost)
        // This helps with Windows where localhost might resolve differently
        let listener = TcpListener::bind(format!("0.0.0.0:{}", DESKTOP_REDIRECT_PORT))
            .or_else(|_| TcpListener::bind(format!("127.0.0.1:{}", DESKTOP_REDIRECT_PORT)))
            .map_err(|e| {
                println!("[OAuth] Failed to bind: {}", e);
                format!("Failed to start callback server: {}", e)
            })?;

        println!("[OAuth] Server bound successfully, waiting for connection...");
        listener.set_nonblocking(false).map_err(|e| e.to_string())?;

        // Wait for callback (with timeout)
        let (mut stream, addr) = listener.accept()
            .map_err(|e| {
                println!("[OAuth] Failed to accept: {}", e);
                format!("Failed to accept connection: {}", e)
            })?;

        println!("[OAuth] Connection accepted from: {}", addr);
        stream.set_read_timeout(Some(Duration::from_secs(30))).ok();

        let mut buffer = [0; 4096];
        let bytes_read = stream.read(&mut buffer)
            .map_err(|e| {
                println!("[OAuth] Failed to read: {}", e);
                format!("Failed to read request: {}", e)
            })?;

        println!("[OAuth] Read {} bytes", bytes_read);
        let request = String::from_utf8_lossy(&buffer[..bytes_read]);
        println!("[OAuth] Request: {}", request.lines().next().unwrap_or(""));

        // Extract authorization code from URL
        let code = extract_code_from_request(&request)
            .ok_or_else(|| {
                println!("[OAuth] No code found in request");
                "No authorization code found in callback".to_string()
            })?;

        println!("[OAuth] Got authorization code");

        // Send success response
        let response = r#"HTTP/1.1 200 OK
Content-Type: text/html; charset=utf-8

<!DOCTYPE html>
<html>
<head><title>BingoDiary - Authorization Successful</title></head>
<body style="font-family: system-ui; display: flex; justify-content: center; align-items: center; height: 100vh; margin: 0;">
<div style="text-align: center;">
<h1 style="color: #4CAF50;">Authorization Successful!</h1>
<p>You can close this window and return to BingoDiary.</p>
</div>
</body>
</html>"#;

        stream.write_all(response.as_bytes()).ok();

        Ok(code)
    }

    fn extract_code_from_request(request: &str) -> Option<String> {
        // Parse GET /callback?code=xxx&... HTTP/1.1
        let first_line = request.lines().next()?;
        let path = first_line.split_whitespace().nth(1)?;

        if let Some(query_start) = path.find('?') {
            let query = &path[query_start + 1..];
            for param in query.split('&') {
                if let Some((key, value)) = param.split_once('=') {
                    if key == "code" {
                        return Some(urlencoding::decode(value).ok()?.into_owned());
                    }
                }
            }
        }
        None
    }
}
