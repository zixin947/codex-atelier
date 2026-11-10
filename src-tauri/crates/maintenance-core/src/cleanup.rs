//! Installed AiMaMi 1.2.7 account-data cleanup; every path is explicit.
//! No background cleanup, home discovery, client process control or keychain access.
use serde::{Deserialize, Serialize};
use std::{
    cell::Cell,
    collections::{BTreeMap, BTreeSet},
    fs::{self, File, OpenOptions},
    io::{self, Write},
    path::{Path, PathBuf},
    sync::atomic::{AtomicU64, Ordering},
    time::{Duration, Instant, SystemTime, UNIX_EPOCH},
};

#[derive(Clone, Debug)]
pub struct CleanupPaths {
    /// Remaining directories created by native persist_registry, in original order.
    pub auxiliary_directories: Vec<PathBuf>,
    pub accounts_directory: PathBuf,
    pub snapshots_directory: PathBuf,
    pub registry_path: PathBuf,
    pub quota_store_path: PathBuf,
    pub auth_backups_directory: PathBuf,
    pub registry_backups_directory: PathBuf,
}
impl CleanupPaths {
    /// Pure path construction; unlike original CodexPaths initialization, no legacy migration.
    pub fn from_home(home: &Path) -> Self {
        let accounts = home.join("accounts");
        let base = home.join("codexmate");
        Self {
            auxiliary_directories: vec![
                base.clone(),
                base.join("skill-backups"),
                base.join("custom-instructions"),
                base.join("custom-instructions/history"),
                base.join("relay"),
            ],
            snapshots_directory: accounts.join("snapshots"),
            registry_path: accounts.join("registry.json"),
            auth_backups_directory: accounts.join("backups"),
            registry_backups_directory: accounts.join("registry-backups"),
            quota_store_path: base.join("quota-store.json"),
            accounts_directory: accounts,
        }
    }
    fn directories(&self) -> Vec<PathBuf> {
        let mut paths = vec![
            self.accounts_directory.clone(),
            self.snapshots_directory.clone(),
            self.auth_backups_directory.clone(),
            self.registry_backups_directory.clone(),
        ];
        paths.extend(self.auxiliary_directories.iter().cloned());
        paths
    }
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CleanupWarning {
    pub code: String,
    pub message: String,
}
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CleanPayload {
    pub auth_backups_removed: i32,
    pub registry_backups_removed: i32,
    pub stale_entries_removed: i32,
}
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct BackupCleanupReport {
    pub removed: i32,
    pub failed: i32,
    pub warnings: Vec<CleanupWarning>,
}
/// No synthesized healthy status: the default implementation reads and mutates real paths.
pub trait CleanupFs {
    fn metadata_exists(&self, path: &Path) -> bool;
    fn canonicalize(&self, path: &Path) -> io::Result<PathBuf>;
    fn regular_file_without_symlink(&self, path: &Path) -> bool;
    fn read_directory(&self, path: &Path) -> io::Result<Vec<io::Result<PathBuf>>>;
    fn remove_file(&self, path: &Path) -> io::Result<()>;
    fn read(&self, path: &Path) -> io::Result<Vec<u8>>;
    fn write_atomic_private(&self, path: &Path, bytes: &[u8]) -> io::Result<()>;
    fn create_directory(&self, path: &Path) -> io::Result<()>;
}
pub struct StdCleanupFs;
impl CleanupFs for StdCleanupFs {
    fn canonicalize(&self, path: &Path) -> io::Result<PathBuf> {
        fs::canonicalize(path)
    }
    fn regular_file_without_symlink(&self, path: &Path) -> bool {
        fs::symlink_metadata(path).is_ok_and(|m| m.is_file())
    }
    fn metadata_exists(&self, path: &Path) -> bool {
        fs::metadata(path).is_ok()
    }
    fn read_directory(&self, path: &Path) -> io::Result<Vec<io::Result<PathBuf>>> {
        Ok(fs::read_dir(path)?
            .map(|entry| entry.map(|entry| entry.path()))
            .collect())
    }
    fn remove_file(&self, path: &Path) -> io::Result<()> {
        fs::remove_file(path)
    }
    fn read(&self, path: &Path) -> io::Result<Vec<u8>> {
        fs::read(path)
    }
    fn create_directory(&self, path: &Path) -> io::Result<()> {
        fs::create_dir_all(path)
    }
    fn write_atomic_private(&self, path: &Path, bytes: &[u8]) -> io::Result<()> {
        static SEQUENCE: AtomicU64 = AtomicU64::new(0);
        let parent = path.parent().ok_or_else(|| {
            io::Error::new(io::ErrorKind::InvalidInput, "missing destination parent")
        })?;
        fs::create_dir_all(parent)?;
        let temporary = parent.join(format!(
            ".cleanup-{}-{}-{}.tmp",
            std::process::id(),
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_nanos(),
            SEQUENCE.fetch_add(1, Ordering::Relaxed)
        ));
        let publish = || {
            let mut options = OpenOptions::new();
            options.create_new(true).write(true);
            #[cfg(unix)]
            {
                use std::os::unix::fs::OpenOptionsExt;
                options.mode(0o600);
            }
            let mut file = options.open(&temporary)?;
            file.write_all(bytes)?;
            file.sync_all()?;
            drop(file);
            fs::rename(&temporary, path)?;
            File::open(parent)?.sync_all()
        };
        let result = publish();
        if result.is_err() {
            let _ = fs::remove_file(&temporary);
        }
        result
    }
}
/// @10082fb00: metadata failure is silently empty, read-dir failure warns, each immediate
/// entry is remove_file-attempted without name/type filtering or recursive deletion.
pub fn remove_account_backup_files_with_fs(
    fs: &impl CleanupFs,
    directory: &Path,
    warning_code: &str,
    label: &str,
) -> BackupCleanupReport {
    let mut report = BackupCleanupReport::default();
    if !fs.metadata_exists(directory) {
        return report;
    }
    let entries = match fs.read_directory(directory) {
        Ok(entries) => entries,
        Err(_) => {
            report.warnings.push(CleanupWarning {
                code: warning_code.into(),
                message: format!("Some {label} backups could not be inspected."),
            });
            return report;
        }
    };
    for entry in entries {
        match entry.and_then(|path| fs.remove_file(&path)) {
            Ok(()) => report.removed = report.removed.wrapping_add(1),
            Err(_) => report.failed = report.failed.wrapping_add(1),
        }
    }
    if report.failed != 0 {
        report.warnings.push(CleanupWarning {
            code: warning_code.into(),
            message: format!("{} {label} backup(s) could not be removed.", report.failed),
        });
    }
    report
}
pub fn remove_account_backup_files(
    directory: &Path,
    warning_code: &str,
    label: &str,
) -> BackupCleanupReport {
    remove_account_backup_files_with_fs(&StdCleanupFs, directory, warning_code, label)
}
/// Native clean holds accounts/.account-domain.lock for registry, quota and both directories.
pub fn acquire_account_domain_lock(paths: &CleanupPaths) -> io::Result<File> {
    acquire_account_domain_lock_for(paths, Duration::from_secs(20))
}
fn acquire_account_domain_lock_for(paths: &CleanupPaths, timeout: Duration) -> io::Result<File> {
    fs::create_dir_all(&paths.accounts_directory)?;
    let path = paths.accounts_directory.join(".account-domain.lock");
    let mut options = OpenOptions::new();
    options.read(true).write(true).create(true);
    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt;
        options.mode(0o600);
    }
    let file = options.open(path)?;
    let deadline = Instant::now() + timeout;
    loop {
        match file.try_lock() {
            Ok(()) => return Ok(file),
            Err(fs::TryLockError::WouldBlock) => {
                if Instant::now() >= deadline {
                    return Err(io::Error::new(
                        io::ErrorKind::TimedOut,
                        "timed out waiting for account state lock",
                    ));
                }
                std::thread::sleep(Duration::from_millis(25));
            }
            Err(fs::TryLockError::Error(e)) if e.kind() == io::ErrorKind::Interrupted => {}
            Err(fs::TryLockError::Error(e)) => return Err(e),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Window {
    used_percent: f64,
    remaining_percent: i32,
    window_minutes: Option<i32>,
    resets_at: Option<i64>,
}
#[derive(Clone, Serialize, Deserialize)]
enum PlanStateSource {
    #[serde(rename = "jwt")]
    Jwt,
    #[serde(rename = "remote")]
    Remote,
    #[serde(rename = "usage")]
    Usage,
}
#[derive(Clone, Serialize, Deserialize)]
enum SubscriptionStateSource {
    #[serde(rename = "jwt")]
    Jwt,
    #[serde(rename = "livePlan")]
    LivePlan,
    #[serde(rename = "remote")]
    Remote,
}
#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
enum UsageSource {
    Local,
    Api,
}
#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
enum TokenStatusCode {
    Fresh,
    Refreshed,
    NoRefreshToken,
    RefreshReused,
    RefreshFailed,
}
#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct AccountTokenStatus {
    code: TokenStatusCode,
    #[serde(skip_serializing_if = "Option::is_none")]
    credential_generation: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    access_token_expires_at: Option<i64>,
}
#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct RegistryItem {
    account_key: String,
    snapshot_path: String,
    email: String,
    alias: String,
    account_name: Option<String>,
    workspace_name: Option<String>,
    profile_name: Option<String>,
    plan: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    plan_state_source: Option<PlanStateSource>,
    plan_state_updated_at: Option<i64>,
    auth_mode: String,
    has_active_subscription: Option<bool>,
    subscription_expires_at: Option<i64>,
    subscription_will_renew: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    subscription_state_source: Option<SubscriptionStateSource>,
    subscription_last_attempt_at: Option<i64>,
    subscription_last_checked_at: Option<i64>,
    subscription_state_updated_at: Option<i64>,
    created_at: i64,
    last_used_at: Option<i64>,
    last_usage_at: Option<i64>,
    cached_primary_window: Option<Window>,
    cached_secondary_window: Option<Window>,
}
fn default_five() -> i32 {
    15
}
fn default_week() -> i32 {
    10
}
fn default_true() -> bool {
    true
}
#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct AutoSwitch {
    #[serde(default)]
    revision: i64,
    #[serde(default)]
    enabled: bool,
    #[serde(default = "default_five")]
    threshold5h_percent: i32,
    #[serde(default = "default_week")]
    threshold_weekly_percent: i32,
}
impl Default for AutoSwitch {
    fn default() -> Self {
        Self {
            revision: 0,
            enabled: false,
            threshold5h_percent: 15,
            threshold_weekly_percent: 10,
        }
    }
}
#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ApiConfig {
    #[serde(default = "default_true")]
    usage_refresh_enabled: bool,
}
impl Default for ApiConfig {
    fn default() -> Self {
        Self {
            usage_refresh_enabled: true,
        }
    }
}
#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Registry {
    schema_version: i32,
    updated_at: i64,
    active_account_key: Option<String>,
    items: Vec<RegistryItem>,
    auto_switch: Option<AutoSwitch>,
    api: Option<ApiConfig>,
}
impl Registry {
    fn empty(now: i64) -> Self {
        Self {
            schema_version: 4,
            updated_at: now,
            active_account_key: None,
            items: vec![],
            auto_switch: Some(AutoSwitch::default()),
            api: Some(ApiConfig::default()),
        }
    }
}
#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct QuotaItem {
    account_key: String,
    captured_at: i64,
    usage_source: UsageSource,
    primary_window: Option<Window>,
    secondary_window: Option<Window>,
    available_reset_credits: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    token_status: Option<AccountTokenStatus>,
    #[serde(skip_serializing_if = "Option::is_none")]
    token_status_updated_at: Option<i64>,
}
#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct QuotaStore {
    schema_version: i32,
    updated_at: i64,
    items: Vec<QuotaItem>,
}
impl QuotaStore {
    fn empty() -> Self {
        Self {
            schema_version: 3,
            updated_at: 0,
            items: vec![],
        }
    }
    fn normalize(&mut self, now: i64) {
        self.schema_version = self.schema_version.max(3);
        for item in &mut self.items {
            if item.primary_window.as_ref().is_some_and(|w| {
                w.resets_at.is_some_and(|t| t <= now)
                    || !w.window_minutes.is_some_and(|m| (1..=360).contains(&m))
            }) {
                item.primary_window = None;
            }
            if item.secondary_window.as_ref().is_some_and(|w| {
                w.resets_at.is_some_and(|t| t <= now) || w.window_minutes.is_some_and(|m| m < 8640)
            }) {
                item.secondary_window = None;
            }
        }
    }
}
#[derive(Debug)]
pub enum CleanupError {
    Io {
        operation: &'static str,
        path: PathBuf,
        source: io::Error,
    },
    Json {
        path: PathBuf,
        source: serde_json::Error,
    },
    StalePlan,
}
impl std::fmt::Display for CleanupError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io {
                operation,
                path,
                source,
            } => write!(f, "{operation} {}: {source}", path.display()),
            Self::Json { path, source } => {
                write!(f, "invalid account registry {}: {source}", path.display())
            }
            Self::StalePlan => write!(
                f,
                "cleanup inputs or targets changed after preview; prepare a new preview"
            ),
        }
    }
}
impl std::error::Error for CleanupError {}
fn io_error(operation: &'static str, path: &Path, source: io::Error) -> CleanupError {
    CleanupError::Io {
        operation,
        path: path.to_path_buf(),
        source,
    }
}
fn load_registry(
    fs: &impl CleanupFs,
    paths: &CleanupPaths,
    now: i64,
) -> Result<Registry, CleanupError> {
    // load_registry_for_mutation checks metadata, not just ErrorKind::NotFound.
    if !fs.metadata_exists(&paths.registry_path) {
        return Ok(Registry::empty(now));
    }
    let bytes = fs
        .read(&paths.registry_path)
        .map_err(|e| io_error("read registry", &paths.registry_path, e))?;
    serde_json::from_slice(&bytes).map_err(|source| CleanupError::Json {
        path: paths.registry_path.clone(),
        source,
    })
}
fn unlocked_key(key: &str) -> bool {
    key.split_once("::").is_some_and(|(user, account)| {
        user.ends_with("-unlock") && account == "00000000-0000-4000-8000-000000000001"
    })
}
fn switchable_snapshot(fs: &impl CleanupFs, item: &RegistryItem, root: &Path) -> bool {
    let path = Path::new(&item.snapshot_path);
    if !fs.metadata_exists(path) || unlocked_key(&item.account_key) {
        return false;
    }
    let Some(parent) = path.parent() else {
        return false;
    };
    let (Ok(parent), Ok(root)) = (fs.canonicalize(parent), fs.canonicalize(root)) else {
        return false;
    };
    if parent != root || !fs.regular_file_without_symlink(path) {
        return false;
    }
    let Ok(bytes) = fs.read(path) else {
        return false;
    };
    let Ok(auth) = serde_json::from_slice::<crate::auth::AuthFile>(&bytes) else {
        return false;
    };
    crate::auth::is_switchable_oauth_auth(&auth)
}
fn load_quota(fs: &impl CleanupFs, path: &Path, now: i64) -> QuotaStore {
    let mut store = fs
        .read(path)
        .ok()
        .and_then(|b| serde_json::from_slice::<QuotaStore>(&b).ok())
        .unwrap_or_else(QuotaStore::empty);
    store.normalize(now);
    store
}
#[derive(Clone, Debug, Default)]
pub struct CleanupReport {
    pub data: CleanPayload,
    pub warnings: Vec<CleanupWarning>,
    pub quota_items_removed: usize,
    pub registry_published: bool,
    /// Atelier ticket boundary; native direct clean has no preview scope protocol.
    pub scope_changed: bool,
}
fn persist_registry(
    fs: &impl CleanupFs,
    paths: &CleanupPaths,
    registry: &Registry,
) -> Result<(), CleanupError> {
    for path in paths.directories() {
        fs.create_directory(&path)
            .map_err(|e| io_error("create account directory", &path, e))?;
    }
    let mut registry = registry.clone();
    registry.schema_version = registry.schema_version.max(4);
    registry.auto_switch.get_or_insert_with(AutoSwitch::default);
    registry.api.get_or_insert_with(ApiConfig::default);
    let bytes = serde_json::to_vec_pretty(&registry).map_err(|source| CleanupError::Json {
        path: paths.registry_path.clone(),
        source,
    })?;
    // clean passes create_backup=false: do not recreate the just-cleaned registry backups.
    fs.write_atomic_private(&paths.registry_path, &bytes)
        .map_err(|e| io_error("publish account registry", &paths.registry_path, e))
}
/// Actual installed ordering, intended for an adapter already holding the account-domain lock.
/// Registry failures stop; disposable quota errors are ignored on read and warnings on write;
/// backup deletion failures aggregate warnings and never roll back published registry state.
pub fn clean_under_existing_lock_with_fs(
    fs: &impl CleanupFs,
    paths: &CleanupPaths,
    now: i64,
) -> Result<CleanupReport, CleanupError> {
    let mut registry = load_registry(fs, paths, now)?;
    let before = registry.items.len();
    registry
        .items
        .retain(|item| switchable_snapshot(fs, item, &paths.snapshots_directory));
    let keys = registry
        .items
        .iter()
        .map(|i| i.account_key.clone())
        .collect::<BTreeSet<_>>();
    if !registry
        .active_account_key
        .as_ref()
        .is_some_and(|key| keys.contains(key))
    {
        registry.active_account_key = None;
    }
    registry.updated_at = now;
    persist_registry(fs, paths, &registry)?;
    let mut report = CleanupReport {
        data: CleanPayload {
            stale_entries_removed: (before - registry.items.len()) as i32,
            ..Default::default()
        },
        registry_published: true,
        ..Default::default()
    };
    let mut quota = load_quota(fs, &paths.quota_store_path, now);
    let count = quota.items.len();
    quota.items.retain(|item| keys.contains(&item.account_key));
    let prune_count = count - quota.items.len();
    let mut quota_warning = None;
    if prune_count != 0 {
        quota.updated_at = now;
        let save = (|| {
            if let Some(parent) = paths.quota_store_path.parent() {
                fs.create_directory(parent)?;
            }
            let bytes = serde_json::to_vec_pretty(&quota).map_err(io::Error::other)?;
            fs.write_atomic_private(&paths.quota_store_path, &bytes)
        })();
        match save {
            Ok(()) => report.quota_items_removed = prune_count,
            Err(error) => quota_warning = Some(CleanupWarning {
                code: "QUOTA_CACHE_CLEANUP_INCOMPLETE".into(),
                message: format!("Account cleanup completed, but the disposable quota cache could not be trimmed: {error}"),
            }),
        }
    }
    let auth = remove_account_backup_files_with_fs(
        fs,
        &paths.auth_backups_directory,
        "AUTH_BACKUP_CLEANUP_INCOMPLETE",
        "authentication",
    );
    let backups = remove_account_backup_files_with_fs(
        fs,
        &paths.registry_backups_directory,
        "REGISTRY_BACKUP_CLEANUP_INCOMPLETE",
        "registry",
    );
    report.data.auth_backups_removed = auth.removed;
    report.data.registry_backups_removed = backups.removed;
    report.warnings.extend(auth.warnings);
    report.warnings.extend(backups.warnings);
    report.warnings.extend(quota_warning);
    Ok(report)
}
fn now_seconds() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs() as i64
}
pub fn clean(paths: &CleanupPaths) -> Result<CleanupReport, CleanupError> {
    let _lock = acquire_account_domain_lock(paths)
        .map_err(|e| io_error("lock account state", &paths.accounts_directory, e))?;
    clean_under_existing_lock_with_fs(&StdCleanupFs, paths, now_seconds())
}
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct CleanupPreview {
    pub stale_entries: usize,
    pub quota_items_to_remove: usize,
    pub inspection_warnings: Vec<CleanupWarning>,
    /// Native attempts every direct entry; directories can fail and are never recursed.
    pub auth_backup_targets: Vec<PathBuf>,
    pub registry_backup_targets: Vec<PathBuf>,
}
#[derive(Clone, PartialEq, Eq)]
struct Dependency {
    path: PathBuf,
    read_contents: bool,
    bytes: Option<Vec<u8>>,
    exists: bool,
    regular: bool,
    canonical: Option<PathBuf>,
}
/// Contains private credential/registry dependency bytes; deliberately no Debug/Serialize.
#[derive(Clone)]
pub struct CleanupPlan {
    paths: CleanupPaths,
    preview: CleanupPreview,
    dependencies: Vec<Dependency>,
}
impl CleanupPlan {
    pub fn preview(&self) -> &CleanupPreview {
        &self.preview
    }
    pub fn paths(&self) -> &CleanupPaths {
        &self.paths
    }
}
fn dependencies(
    fs: &impl CleanupFs,
    paths: &CleanupPaths,
    registry: &Registry,
    preview: &CleanupPreview,
) -> Vec<Dependency> {
    let mut reads = BTreeMap::from([
        (paths.registry_path.clone(), true),
        (paths.quota_store_path.clone(), true),
        (paths.snapshots_directory.clone(), false),
    ]);
    let root = fs.canonicalize(&paths.snapshots_directory).ok();
    for item in &registry.items {
        let path = PathBuf::from(&item.snapshot_path);
        // Native rejects outside/unlock/symlink snapshots before reading credentials.
        // Preview dependency capture must respect that same read boundary.
        let can_read = !unlocked_key(&item.account_key)
            && fs.regular_file_without_symlink(&path)
            && root.as_ref().is_some_and(|root| {
                path.parent().and_then(|p| fs.canonicalize(p).ok()).as_ref() == Some(root)
            });
        reads
            .entry(path)
            .and_modify(|read| *read |= can_read)
            .or_insert(can_read);
    }
    for path in preview
        .auth_backup_targets
        .iter()
        .chain(&preview.registry_backup_targets)
    {
        // A backup symlink is an approved unlink; its external target contents are irrelevant.
        reads.insert(path.clone(), fs.regular_file_without_symlink(path));
    }
    reads
        .into_iter()
        .map(|(path, read_contents)| observe(fs, &path, read_contents))
        .collect()
}
fn targets(
    fs: &impl CleanupFs,
    path: &Path,
    code: &str,
    label: &str,
) -> (Vec<PathBuf>, Vec<CleanupWarning>) {
    if !fs.metadata_exists(path) {
        return (vec![], vec![]);
    }
    let mut files = vec![];
    let mut warnings = vec![];
    match fs.read_directory(path) {
        Ok(entries) => {
            let mut failures = 0;
            for entry in entries {
                match entry {
                    Ok(path) => files.push(path),
                    Err(_) => failures += 1,
                }
            }
            if failures > 0 {
                warnings.push(CleanupWarning {
                    code: code.into(),
                    message: format!(
                        "{failures} {label} backup entry(s) could not be inspected during preview."
                    ),
                });
            }
        }
        Err(_) => warnings.push(CleanupWarning {
            code: code.into(),
            message: format!("Some {label} backups could not be inspected."),
        }),
    }
    files.sort();
    (files, warnings)
}
pub fn prepare_with_fs(
    fs: &impl CleanupFs,
    paths: &CleanupPaths,
    now: i64,
) -> Result<CleanupPlan, CleanupError> {
    let registry = load_registry(fs, paths, now)?;
    let keys = registry
        .items
        .iter()
        .filter(|item| switchable_snapshot(fs, item, &paths.snapshots_directory))
        .map(|item| item.account_key.clone())
        .collect::<BTreeSet<_>>();
    let retained = registry
        .items
        .iter()
        .filter(|item| switchable_snapshot(fs, item, &paths.snapshots_directory))
        .count();
    let quota = load_quota(fs, &paths.quota_store_path, now);
    let (auth_backup_targets, mut inspection_warnings) = targets(
        fs,
        &paths.auth_backups_directory,
        "AUTH_BACKUP_CLEANUP_INCOMPLETE",
        "authentication",
    );
    let (registry_backup_targets, registry_warnings) = targets(
        fs,
        &paths.registry_backups_directory,
        "REGISTRY_BACKUP_CLEANUP_INCOMPLETE",
        "registry",
    );
    inspection_warnings.extend(registry_warnings);
    let preview = CleanupPreview {
        stale_entries: registry.items.len() - retained,
        quota_items_to_remove: quota
            .items
            .iter()
            .filter(|i| !keys.contains(&i.account_key))
            .count(),
        auth_backup_targets,
        registry_backup_targets,
        inspection_warnings,
    };
    let dependencies = dependencies(fs, paths, &registry, &preview);
    Ok(CleanupPlan {
        paths: paths.clone(),
        preview,
        dependencies,
    })
}
pub fn prepare(paths: &CleanupPaths) -> Result<CleanupPlan, CleanupError> {
    prepare_with_fs(&StdCleanupFs, paths, now_seconds())
}
pub fn validate_with_fs(
    fs: &impl CleanupFs,
    plan: &CleanupPlan,
    now: i64,
) -> Result<(), CleanupError> {
    let fresh = prepare_with_fs(fs, &plan.paths, now)?;
    if fresh.preview != plan.preview || fresh.dependencies != plan.dependencies {
        return Err(CleanupError::StalePlan);
    }
    Ok(())
}
fn observe(fs: &impl CleanupFs, path: &Path, read_contents: bool) -> Dependency {
    Dependency {
        path: path.to_path_buf(),
        read_contents,
        bytes: if read_contents {
            fs.read(path).ok()
        } else {
            None
        },
        exists: fs.metadata_exists(path),
        regular: fs.regular_file_without_symlink(path),
        canonical: fs.canonicalize(path).ok(),
    }
}
struct ScopedFs<'a, F> {
    fs: &'a F,
    plan: &'a CleanupPlan,
    changed: Cell<bool>,
}
impl<F: CleanupFs> ScopedFs<'_, F> {
    fn unchanged(&self, path: &Path) -> bool {
        let same = self
            .plan
            .dependencies
            .iter()
            .find(|d| d.path == path)
            .is_some_and(|d| observe(self.fs, path, d.read_contents) == *d);
        if !same {
            self.changed.set(true);
        }
        same
    }
    fn check_observed(&self, path: &Path) {
        if self.plan.dependencies.iter().any(|d| d.path == path) {
            self.unchanged(path);
        }
    }
}
impl<F: CleanupFs> CleanupFs for ScopedFs<'_, F> {
    fn metadata_exists(&self, path: &Path) -> bool {
        self.check_observed(path);
        self.fs.metadata_exists(path)
    }
    fn canonicalize(&self, path: &Path) -> io::Result<PathBuf> {
        self.check_observed(path);
        self.fs.canonicalize(path)
    }
    fn regular_file_without_symlink(&self, path: &Path) -> bool {
        self.check_observed(path);
        self.fs.regular_file_without_symlink(path)
    }
    fn read(&self, path: &Path) -> io::Result<Vec<u8>> {
        self.check_observed(path);
        self.fs.read(path)
    }
    fn create_directory(&self, path: &Path) -> io::Result<()> {
        self.fs.create_directory(path)
    }
    fn write_atomic_private(&self, path: &Path, bytes: &[u8]) -> io::Result<()> {
        if !self.unchanged(path) || self.changed.get() {
            return Err(io::Error::other("approved cleanup dependency changed"));
        }
        self.fs.write_atomic_private(path, bytes)
    }
    fn read_directory(&self, path: &Path) -> io::Result<Vec<io::Result<PathBuf>>> {
        let entries = self.fs.read_directory(path)?;
        let approved = if path == self.plan.paths.auth_backups_directory {
            &self.plan.preview.auth_backup_targets
        } else if path == self.plan.paths.registry_backups_directory {
            &self.plan.preview.registry_backup_targets
        } else {
            self.changed.set(true);
            return Err(io::Error::other("unapproved cleanup directory"));
        };
        if entries
            .iter()
            .filter_map(|entry| entry.as_ref().ok())
            .any(|p| !approved.contains(p))
        {
            self.changed.set(true);
            return Err(io::Error::other(
                "new backup targets appeared after preview",
            ));
        }
        Ok(entries)
    }
    fn remove_file(&self, path: &Path) -> io::Result<()> {
        if !self.unchanged(path) {
            return Err(io::Error::other("approved backup file changed"));
        }
        self.fs.remove_file(path)
    }
}
/// Applies a ticket while the caller holds the account-domain lock. New backup targets
/// cannot enter the approved scope after earlier registry/quota publications.
pub fn apply_under_existing_lock_with_fs(
    fs: &impl CleanupFs,
    plan: &CleanupPlan,
    now: i64,
) -> Result<CleanupReport, CleanupError> {
    validate_with_fs(fs, plan, now)?;
    let scoped = ScopedFs {
        fs,
        plan,
        changed: Cell::new(false),
    };
    let result = clean_under_existing_lock_with_fs(&scoped, &plan.paths, now);
    match result {
        Ok(mut report) => {
            report.scope_changed = scoped.changed.get();
            if report.scope_changed {
                report.warnings.push(CleanupWarning{code:"CLEANUP_APPROVED_SCOPE_CHANGED".into(),message:"Backup targets or inputs changed during cleanup; changed/new files were preserved. Earlier registry/cache publications remain; prepare a new preview.".into()});
            }
            Ok(report)
        }
        Err(_) if scoped.changed.get() => Err(CleanupError::StalePlan),
        Err(error) => Err(error),
    }
}
pub fn apply(plan: &CleanupPlan) -> Result<CleanupReport, CleanupError> {
    let _lock = acquire_account_domain_lock(&plan.paths)
        .map_err(|e| io_error("lock account state", &plan.paths.accounts_directory, e))?;
    apply_under_existing_lock_with_fs(&StdCleanupFs, plan, now_seconds())
}
