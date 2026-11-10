//! AiMaMi 1.2.7 authentication maintenance recovered from the shipped arm64 binary.
//! Paths are explicit; this module never discovers a user's home or reads a keychain.
use base64::{engine::general_purpose, Engine};
use serde::Deserialize;
use serde_json::Value;
use std::{
    fs, io,
    path::{Path, PathBuf},
};

mod lexical;
pub mod takeover;
pub mod unlock;

pub const VIRTUAL_KEY_PREFIX: &str = "sk-aimami-relay-virtual";
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RouterMode {
    Off,
    Native,
    NoAccount,
}
impl RouterMode {
    pub fn enabled(self) -> bool {
        self != Self::Off
    }
}
#[derive(Clone, Debug)]
pub struct AuthContext {
    pub codex_home: PathBuf,
    pub router_mode: RouterMode,
}
impl AuthContext {
    pub fn new(codex_home: impl Into<PathBuf>, router_mode: RouterMode) -> Self {
        Self {
            codex_home: codex_home.into(),
            router_mode,
        }
    }
    pub fn auth_path(&self) -> PathBuf {
        self.codex_home.join("auth.json")
    }
    pub fn auth_backup_dir(&self) -> PathBuf {
        self.codex_home.join("accounts/backups")
    }
    pub fn relay_dir(&self) -> PathBuf {
        self.codex_home.join("codexmate/relay")
    }
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Level {
    Ok,
    Warning,
    Error,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AuthReason {
    Missing,
    Unreadable,
    Empty,
    InvalidJson,
    ManagedVirtual,
    PlaceholderToken,
    OAuthToken,
    MalformedToken,
    ApiKey,
    MissingToken,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AuthCheck {
    pub level: Level,
    pub reason: AuthReason,
    pub fixable: bool,
}
impl AuthCheck {
    pub fn label(&self) -> &'static str {
        match self.reason {
            AuthReason::Missing => "auth.json 不存在",
            AuthReason::Unreadable => "auth.json 无法读取",
            AuthReason::Empty => "auth.json 为空",
            AuthReason::InvalidJson => "auth.json JSON 格式损坏",
            AuthReason::ManagedVirtual => "auth.json 为旧版工具虚拟账号（非真实账号）",
            AuthReason::PlaceholderToken => "auth.json access_token 为占位符",
            AuthReason::OAuthToken => "auth.json 凭证正常",
            AuthReason::MalformedToken => "auth.json access_token 格式异常",
            AuthReason::ApiKey => "auth.json 正常（API Key 模式）",
            AuthReason::MissingToken => "auth.json access_token 为空",
        }
    }
}
fn check(reason: AuthReason, level: Level) -> AuthCheck {
    AuthCheck {
        reason,
        level,
        fixable: level != Level::Ok,
    }
}
pub fn check_auth_integrity(ctx: &AuthContext) -> AuthCheck {
    let p = ctx.auth_path();
    if fs::metadata(&p).is_err() {
        return check(AuthReason::Missing, Level::Warning);
    }
    match fs::read_to_string(p) {
        Ok(t) => check_auth_text(&t),
        Err(_) => check(AuthReason::Unreadable, Level::Error),
    }
}
pub fn check_auth_text(text: &str) -> AuthCheck {
    if text.trim().is_empty() {
        return check(AuthReason::Empty, Level::Error);
    }
    let Ok(v) = serde_json::from_str::<Value>(text) else {
        return check(AuthReason::InvalidJson, Level::Error);
    };
    if let Ok(a) = serde_json::from_value::<AuthFile>(v.clone()) {
        if is_managed_virtual_auth(&a) {
            return check(AuthReason::ManagedVirtual, Level::Ok);
        }
    }
    if let Some(t) = v
        .pointer("/tokens/access_token")
        .and_then(Value::as_str)
        .filter(|t| !t.is_empty())
    {
        if is_known_placeholder_token(t) {
            check(AuthReason::PlaceholderToken, Level::Error)
        } else if is_plausible_jwt(t) {
            check(AuthReason::OAuthToken, Level::Ok)
        } else {
            check(AuthReason::MalformedToken, Level::Warning)
        }
    } else if v
        .get("OPENAI_API_KEY")
        .and_then(Value::as_str)
        .is_some_and(|s| s.starts_with("sk-"))
    {
        check(AuthReason::ApiKey, Level::Ok)
    } else {
        check(AuthReason::MissingToken, Level::Error)
    }
}
pub fn is_known_placeholder_token(t: &str) -> bool {
    t.len() < 10
        || matches!(
            t.to_lowercase().as_str(),
            "fake"
                | "test"
                | "mock"
                | "null"
                | "none"
                | "invalid"
                | "undefined"
                | "placeholder"
                | "cc-switch"
                | "cc_switch"
                | "ccswitch"
                | "deecodex"
                | "sk-fake"
                | "sk-test"
        )
}
pub fn is_plausible_jwt(t: &str) -> bool {
    let p: Vec<_> = t.split('.').collect();
    p.len() == 3
        && general_purpose::URL_SAFE_NO_PAD.decode(p[0]).is_ok()
        && general_purpose::URL_SAFE_NO_PAD.decode(p[1]).is_ok()
}
#[derive(Clone, Deserialize, Default)]
pub struct AuthTokens {
    #[serde(default)]
    pub access_token: Option<String>,
    #[serde(default)]
    pub id_token: Option<String>,
    #[serde(default)]
    pub refresh_token: Option<String>,
    #[serde(default)]
    pub account_id: Option<String>,
}
#[derive(Clone, Deserialize, Default)]
pub struct AuthFile {
    #[serde(default)]
    pub auth_mode: Option<String>,
    #[serde(default, rename = "OPENAI_API_KEY")]
    pub api_key: Option<String>,
    #[serde(default)]
    pub aimami_router_unlock_auth: bool,
    #[serde(default)]
    pub tokens: AuthTokens,
    #[serde(default, deserialize_with = "deserialize_timestamp")]
    pub last_refresh: Option<String>,
}
fn deserialize_timestamp<'de, D: serde::Deserializer<'de>>(
    d: D,
) -> Result<Option<String>, D::Error> {
    let v = Value::deserialize(d)?;
    let text = match v {
        Value::Null => return Ok(None),
        Value::Number(n) => n.to_string(),
        Value::String(s) => s.trim().to_owned(),
        _ => {
            return Err(serde::de::Error::custom(
                "last_refresh must be a timestamp string or number",
            ))
        }
    };
    if text.is_empty() {
        return Ok(None);
    }
    let Ok(mut seconds) = text.parse::<f64>() else {
        return Ok(Some(text));
    };
    if seconds > 100_000_000_000.0 {
        seconds /= 1000.0
    }
    let timestamp =
        chrono::DateTime::<chrono::Utc>::from_timestamp(seconds as i64, 0).ok_or_else(|| {
            serde::de::Error::custom("last_refresh is outside the supported timestamp range")
        })?;
    Ok(Some(timestamp.to_rfc3339()))
}
fn claims(token: Option<&str>) -> Value {
    let Some(p) = token.and_then(|t| t.split('.').nth(1)) else {
        return Value::Null;
    };
    let b = p.replace('-', "+").replace('_', "/");
    let b = b.trim_end_matches('=');
    general_purpose::STANDARD_NO_PAD
        .decode(b)
        .or_else(|_| general_purpose::STANDARD.decode(b))
        .ok()
        .and_then(|b| serde_json::from_slice(&b).ok())
        .unwrap_or(Value::Null)
}
pub fn is_managed_virtual_auth(a: &AuthFile) -> bool {
    if a.aimami_router_unlock_auth {
        return true;
    }
    if a.api_key
        .as_deref()
        .is_some_and(|k| k.starts_with(VIRTUAL_KEY_PREFIX))
        && a.tokens.access_token.is_none()
        && a.tokens.id_token.is_none()
    {
        return true;
    }
    let access = claims(a.tokens.access_token.as_deref());
    let id = claims(a.tokens.id_token.as_deref());
    let c = access
        .get("https://api.openai.com/auth")
        .filter(|v| v.is_object())
        .or_else(|| {
            id.get("https://api.openai.com/auth")
                .filter(|v| v.is_object())
        });
    let Some(c) = c else { return false };
    let account = a
        .tokens
        .account_id
        .as_deref()
        .filter(|s| !s.is_empty())
        .or_else(|| c.get("chatgpt_account_id").and_then(Value::as_str));
    let user = ["chatgpt_user_id", "chatgpt_account_user_id", "user_id"]
        .iter()
        .find_map(|k| c.get(k).and_then(Value::as_str));
    match (user, account) {
        (Some(u), Some(a)) => format!("{u}::{a}").split_once("::").is_some_and(|(u, a)| {
            u.ends_with("-unlock") && a == "00000000-0000-4000-8000-000000000001"
        }),
        _ => false,
    }
}
pub fn is_switchable_oauth_auth(a: &AuthFile) -> bool {
    !is_managed_virtual_auth(a)
        && !a
            .auth_mode
            .as_deref()
            .is_some_and(|m| m.eq_ignore_ascii_case("apikey"))
        && !a.api_key.as_deref().is_some_and(|k| !k.trim().is_empty())
        && [
            a.tokens.access_token.as_deref(),
            a.tokens.id_token.as_deref(),
        ]
        .into_iter()
        .flatten()
        .any(|s| {
            let s = s.trim();
            !s.is_empty() && !s.starts_with("__") && s.split('.').count() > 2
        })
}
/// Credential bytes intentionally do not implement Debug or Serialize.
pub struct FileChange {
    pub path: PathBuf,
    pub before: Option<Vec<u8>>,
    pub after: Option<Vec<u8>>,
}
pub struct FileObservation {
    pub path: PathBuf,
    pub bytes: Option<Vec<u8>>,
}
pub struct AuthPlan {
    pub changes: Vec<FileChange>,
    pub selected_backup: Option<PathBuf>,
    pub observations: Vec<FileObservation>,
}
#[derive(Debug)]
pub enum AuthError {
    Io { path: PathBuf, source: io::Error },
    RouterEnabled,
    NoUsableBackup,
    ChangedSincePreview(PathBuf),
    UnsafeBackup(String),
    Unsupported(String),
    InvalidConfig(String),
}
impl std::fmt::Display for AuthError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io { path, source } => write!(f, "{}: {source}", path.display()),
            Self::RouterEnabled => f.write_str("请先关闭路由后再执行此修复"),
            Self::NoUsableBackup => f.write_str("没有可用的认证备份，请重新登录"),
            Self::ChangedSincePreview(p) => write!(f, "{} changed since preview", p.display()),
            Self::UnsafeBackup(s) | Self::Unsupported(s) | Self::InvalidConfig(s) => f.write_str(s),
        }
    }
}
impl std::error::Error for AuthError {}
pub(crate) fn ioerr(path: &Path, source: io::Error) -> AuthError {
    AuthError::Io {
        path: path.into(),
        source,
    }
}
pub(crate) fn snapshot(p: &Path) -> Result<Option<Vec<u8>>, AuthError> {
    match fs::read(p) {
        Ok(b) => Ok(Some(b)),
        Err(e) if e.kind() == io::ErrorKind::NotFound => Ok(None),
        Err(e) => Err(ioerr(p, e)),
    }
}
pub(crate) fn change(p: PathBuf, after: Option<Vec<u8>>) -> Result<FileChange, AuthError> {
    Ok(FileChange {
        before: snapshot(&p)?,
        path: p,
        after,
    })
}
/// The caller holds the native live-auth coordination lock for unlock cleanup.
/// All snapshots are checked before the first mutation; publications use mode 0600.
pub(crate) fn verify_plan(plan: &AuthPlan) -> Result<(), AuthError> {
    for o in &plan.observations {
        if snapshot(&o.path)? != o.bytes {
            return Err(AuthError::ChangedSincePreview(o.path.clone()));
        }
    }
    for c in &plan.changes {
        if snapshot(&c.path)? != c.before {
            return Err(AuthError::ChangedSincePreview(c.path.clone()));
        }
    }
    Ok(())
}
pub fn apply_plan(plan: &AuthPlan) -> Result<(), AuthError> {
    verify_plan(plan)?;
    for c in &plan.changes {
        match &c.after {
            Some(b) => write_private(&c.path, b)?,
            None => match fs::remove_file(&c.path) {
                Ok(()) => {}
                Err(e) if e.kind() == io::ErrorKind::NotFound => {}
                Err(e) => return Err(ioerr(&c.path, e)),
            },
        }
    }
    Ok(())
}
pub(crate) fn write_private(p: &Path, b: &[u8]) -> Result<(), AuthError> {
    use std::io::Write;
    #[cfg(unix)]
    use std::os::unix::fs::OpenOptionsExt;
    let parent = p
        .parent()
        .ok_or_else(|| AuthError::Unsupported("path has no parent".into()))?;
    let mut attempt = 0;
    loop {
        let temp = parent.join(format!(
            ".auth-maintenance-{}-{attempt}.tmp",
            std::process::id()
        ));
        let mut options = fs::OpenOptions::new();
        options.write(true).create_new(true);
        #[cfg(unix)]
        options.mode(0o600);
        match options.open(&temp) {
            Ok(mut f) => {
                let result = (|| {
                    f.write_all(b)?;
                    f.sync_all()?;
                    fs::rename(&temp, p)?;
                    fs::File::open(parent)?.sync_all()
                })();
                if result.is_err() {
                    let _ = fs::remove_file(&temp);
                }
                return result.map_err(|e| ioerr(p, e));
            }
            Err(e) if e.kind() == io::ErrorKind::AlreadyExists => attempt += 1,
            Err(e) => return Err(ioerr(p, e)),
        }
    }
}
/// Native scan: direct regular-file children with the case-sensitive `json` extension,
/// stable newest-mtime first; failed metadata/mtime uses UNIX_EPOCH.
pub fn auth_backup_candidates(ctx: &AuthContext) -> Vec<PathBuf> {
    use std::time::UNIX_EPOCH;
    let Ok(entries) = fs::read_dir(ctx.auth_backup_dir()) else {
        return Vec::new();
    };
    let mut files: Vec<_> = entries
        .filter_map(Result::ok)
        .map(|e| e.path())
        .filter(|p| p.is_file() && p.extension().is_some_and(|e| e == "json"))
        .collect();
    files.sort_by(|a, b| {
        let time = |p: &PathBuf| {
            fs::metadata(p)
                .and_then(|m| m.modified())
                .unwrap_or(UNIX_EPOCH)
        };
        time(b).cmp(&time(a))
    });
    files
}
pub fn plan_auth_integrity(ctx: &AuthContext) -> Result<AuthPlan, AuthError> {
    if ctx.router_mode.enabled() {
        return Err(AuthError::RouterEnabled);
    }
    for p in auth_backup_candidates(ctx) {
        let Ok(t) = fs::read_to_string(&p) else {
            continue;
        };
        let Ok(v) = serde_json::from_str::<Value>(&t) else {
            continue;
        };
        let access = v
            .pointer("/tokens/access_token")
            .and_then(Value::as_str)
            .is_some_and(|s| !s.is_empty() && !is_known_placeholder_token(s));
        let api = v
            .get("OPENAI_API_KEY")
            .and_then(Value::as_str)
            .is_some_and(|s| s.starts_with("sk-"));
        if access || api {
            return Ok(AuthPlan {
                changes: vec![change(ctx.auth_path(), Some(t.as_bytes().to_vec()))?],
                selected_backup: Some(p.clone()),
                observations: vec![FileObservation {
                    path: p,
                    bytes: Some(t.as_bytes().to_vec()),
                }],
            });
        }
    }
    Err(AuthError::NoUsableBackup)
}
/// A successful publication may yield Warning/Error: native code stops at this candidate.
pub fn apply_auth_integrity(ctx: &AuthContext, plan: &AuthPlan) -> Result<AuthCheck, AuthError> {
    if ctx.router_mode.enabled() {
        return Err(AuthError::RouterEnabled);
    }
    apply_plan(plan)?;
    Ok(check_auth_integrity(ctx))
}
