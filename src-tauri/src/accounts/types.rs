use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Window {
    pub used_percent: Option<f64>,
    pub duration_mins: Option<u64>,
    pub resets_at: Option<u64>,
}
#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResetCredit {
    pub id: String,
    pub reset_type: String,
    pub status: String,
    pub granted_at: Option<u64>,
    pub expires_at: Option<u64>,
    pub no_expiration: bool,
    pub title: Option<String>,
    pub description: Option<String>,
}
#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResetCredits {
    pub available_count: Option<u64>,
    pub credits: Option<Vec<ResetCredit>>,
    pub details_checked_at: Option<u64>,
}
#[derive(Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Quota {
    pub primary: Option<Window>,
    pub secondary: Option<Window>,
    pub checked_at: u64,
    #[serde(default)]
    pub reset_credits: Option<ResetCredits>,
}
#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountMetadata {
    pub account_name: Option<String>,
    pub workspace_name: Option<String>,
    pub profile_name: Option<String>,
    pub plan_snapshot: Option<String>,
    pub has_active_subscription: Option<bool>,
    pub subscription_expires_at: Option<u64>,
    pub subscription_will_renew: Option<bool>,
    pub is_delinquent: Option<bool>,
    pub source: String,
    pub source_at: Option<u64>,
    pub error: Option<String>,
}
impl Default for AccountMetadata {
    fn default() -> Self {
        Self {
            account_name: None,
            workspace_name: None,
            profile_name: None,
            plan_snapshot: None,
            has_active_subscription: None,
            subscription_expires_at: None,
            subscription_will_renew: None,
            is_delinquent: None,
            source: "none".into(),
            source_at: None,
            error: None,
        }
    }
}
#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CurrentObservation {
    pub identity: String,
    pub path: String,
    pub auth_signature: String,
    pub config_signature: String,
    pub checked_at: u64,
}
#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CurrentAccount {
    pub account_id: Option<String>,
    pub state: String,
    pub path: Option<String>,
    pub checked_at: Option<u64>,
    pub message: String,
}
#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Account {
    pub id: String,
    pub name: String,
    pub email: Option<String>,
    pub plan: Option<String>,
    pub group: String,
    pub note: String,
    pub status: String,
    pub quota: Option<Quota>,
    pub error: Option<String>,
    pub created_at: u64,
    // Local deduplication hint, never an authentication proof.
    #[serde(default)]
    pub identity: String,
    #[serde(default)]
    pub metadata: AccountMetadata,
}
#[derive(Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Catalog {
    pub revision: String,
    pub accounts: Vec<Account>,
    pub last_selected_id: Option<String>,
    #[serde(default)]
    pub current_observation: Option<CurrentObservation>,
}
#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Snapshot {
    pub available: bool,
    pub reason: Option<String>,
    pub revision: String,
    pub accounts: Vec<Account>,
    pub last_selected_id: Option<String>,
    pub rollback_available: bool,
    pub pending_login: Option<Login>,
    pub current_account: CurrentAccount,
}
#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Login {
    pub id: String,
    pub state: String,
    pub url: Option<String>,
    pub message: Option<String>,
    pub expires_at: u64,
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Edit {
    pub id: String,
    pub name: String,
    pub group: String,
    pub note: String,
}
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ItemResult {
    pub id: String,
    pub ok: bool,
    pub message: String,
}
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BatchResult {
    pub snapshot: Snapshot,
    pub results: Vec<ItemResult>,
}
#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SwitchPreview {
    pub ticket: String,
    pub target: String,
    pub before: String,
    pub after: String,
    pub expires_at: u64,
    pub rollback: bool,
    pub warning: String,
}
#[derive(Clone, Serialize, Deserialize)]
pub struct Backup {
    pub target: String,
    pub before: Option<Vec<u8>>,
    pub after_hash: String,
    pub config_hash: String,
    pub selected_before: Option<String>,
}
