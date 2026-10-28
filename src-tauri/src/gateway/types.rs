use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ProfileKind {
    Chatgpt,
    ApiKey,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GatewayModel {
    pub id: String,
    pub name: String,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Profile {
    pub id: String,
    pub kind: ProfileKind,
    pub name: String,
    pub email: Option<String>,
    pub enabled: bool,
    pub priority: u16,
    pub base_url: String,
    pub auth_status: String,
    pub models: Vec<GatewayModel>,
    pub cooldown_until: Option<u64>,
    pub last_error: Option<String>,
    pub requests: u64,
    pub input_tokens: u64,
    pub output_tokens: u64,
    pub created_at: u64,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Settings {
    pub port: u16,
    pub auto_switch: bool,
    pub max_attempts: u8,
    pub preferred_profile_id: Option<String>,
    pub model: String,
}
impl Default for Settings {
    fn default() -> Self {
        Self {
            port: 18910,
            auto_switch: false,
            max_attempts: 2,
            preferred_profile_id: None,
            model: String::new(),
        }
    }
}
#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Catalog {
    pub revision: String,
    pub host_id: String,
    pub profiles: Vec<Profile>,
    pub settings: Settings,
}
/// Secrets never cross the Tauri IPC boundary. Atomically replace the whole token set.
#[derive(Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Credential {
    pub api_key: Option<String>,
    pub access_token: Option<String>,
    pub refresh_token: Option<String>,
    pub id_token: Option<String>,
    pub client_id: Option<String>,
    pub subject: Option<String>,
    pub issuer: Option<String>,
    pub scopes: Vec<String>,
    pub expires_at: Option<u64>,
}
#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LoginAttempt {
    pub id: String,
    pub state: String,
    pub auth_url: Option<String>,
    pub expires_at: u64,
    pub message: Option<String>,
}
#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RouteEvent {
    pub at: u64,
    pub profile_id: Option<String>,
    pub status: String,
    pub code: String,
    pub attempts: usize,
}
#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RuntimeView {
    pub state: String,
    pub address: Option<String>,
    pub active_requests: usize,
    pub completed_requests: u64,
    pub failed_requests: u64,
    pub message: Option<String>,
}
impl Default for RuntimeView {
    fn default() -> Self {
        Self {
            state: "stopped".into(),
            address: None,
            active_requests: 0,
            completed_requests: 0,
            failed_requests: 0,
            message: None,
        }
    }
}
#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Snapshot {
    pub available: bool,
    pub reason: Option<String>,
    pub revision: String,
    pub profiles: Vec<Profile>,
    pub settings: Settings,
    pub runtime: RuntimeView,
    pub events: Vec<RouteEvent>,
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProviderInput {
    pub id: Option<String>,
    pub name: String,
    pub base_url: String,
    pub api_key: Option<String>,
    pub models: Vec<String>,
    pub priority: u16,
}
#[derive(Deserialize)]
pub struct ProfileUpdate {
    pub id: String,
    pub name: String,
    pub enabled: bool,
    pub priority: u16,
}
pub fn now_ms() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis() as u64
}
