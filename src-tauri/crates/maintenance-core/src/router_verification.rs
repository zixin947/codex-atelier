//! Native target-state invariants recovered from AiMaMi arm64 100363040/10068704c.
//! Every filesystem/DB path is explicit; no home discovery or application execution.
pub use crate::transition_journal::TransitionTarget as TargetState;
use crate::{
    auth::{
        unlock::{self, LiveAuthState},
        AuthContext, RouterMode,
    },
    catalog::{self, builder, state::RelayStateDocument},
    config,
    threads::consistency,
    transition_journal::JournalStore,
};
use serde::{Deserialize, Serialize};
use std::{
    collections::{BTreeSet, HashSet},
    fs, io,
    net::{IpAddr, Ipv4Addr, SocketAddr, TcpStream},
    path::{Path, PathBuf},
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, Mutex,
    },
    time::Duration,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum InvariantDim {
    Config,
    Auth,
    Catalog,
    Thread,
    Proxy,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VerificationIssue {
    pub dim: InvariantDim,
    pub message: String,
}
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct VerificationReport {
    pub issues: Vec<VerificationIssue>,
}
impl VerificationReport {
    fn push(&mut self, dim: InvariantDim, message: impl Into<String>) {
        self.issues.push(VerificationIssue {
            dim,
            message: message.into(),
        });
    }
    pub fn is_satisfied(&self) -> bool {
        self.issues.is_empty()
    }
}
/// Native depth=1 skips provider convergence, but still inspects router model catalogs/DBs.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Depth {
    Full,
    SkipProviderConvergence,
}
#[derive(Debug, Clone)]
pub struct VerificationPaths {
    pub codex_home: PathBuf,
    pub relay_root: PathBuf,
    pub config_path: PathBuf,
    /// Actual state DB paths selected by the existing explicit-path discovery adapter.
    pub databases: Vec<PathBuf>,
}
impl VerificationPaths {
    pub fn from_home(home: &Path, databases: Vec<PathBuf>) -> Self {
        Self {
            codex_home: home.to_owned(),
            relay_root: home.join("codexmate/relay"),
            config_path: home.join("config.toml"),
            databases,
        }
    }
    pub fn catalog_path(&self) -> PathBuf {
        self.relay_root.join(catalog::CATALOG_FILE)
    }
}
/// Share the actual runtime's live flag. A true flag alone never passes verification:
/// the owned handle's loopback port must also accept a real connection.
#[derive(Clone)]
pub struct ProxyHandle {
    pub port: u16,
    pub alive: Arc<AtomicBool>,
}
pub struct VerificationContext<'a> {
    pub paths: &'a VerificationPaths,
    /// Retained Manager snapshot. Native invariant mode is determined from actual files,
    /// not from this snapshot's no-account/presence booleans.
    pub state: &'a RelayStateDocument,
    pub proxy: &'a Mutex<Option<ProxyHandle>>,
}
pub trait PortTransport {
    fn connect(&self, address: SocketAddr, timeout: Duration) -> io::Result<()>;
    fn pause(&self, duration: Duration);
}
#[derive(Clone, Copy, Default)]
pub struct LocalPortTransport;
impl PortTransport for LocalPortTransport {
    fn connect(&self, address: SocketAddr, timeout: Duration) -> io::Result<()> {
        TcpStream::connect_timeout(&address, timeout).map(drop)
    }
    fn pause(&self, duration: Duration) {
        std::thread::sleep(duration);
    }
}
pub fn proxy_handle_port_accepts(handle: &ProxyHandle, transport: &impl PortTransport) -> bool {
    if !handle.alive.load(Ordering::Acquire) {
        return false;
    }
    let address = SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), handle.port);
    let timeout = Duration::from_millis(300);
    if transport.connect(address, timeout).is_ok() {
        return true;
    }
    transport.pause(Duration::from_millis(50));
    transport.connect(address, timeout).is_ok()
}
pub fn read_catalog_slugs(path: &Path) -> Result<BTreeSet<String>, String> {
    let bytes = fs::read(path).map_err(|e| {
        format!(
            "Operation failed: read codex router catalog {} failed: {e}",
            path.display()
        )
    })?;
    let value: serde_json::Value = serde_json::from_slice(&bytes).map_err(|e| {
        format!(
            "Operation failed: parse codex router catalog {} failed: {e}",
            path.display()
        )
    })?;
    let models = value
        .get("models")
        .and_then(serde_json::Value::as_array)
        .ok_or_else(|| {
            format!(
                "Operation failed: codex router catalog {} has no models array",
                path.display()
            )
        })?;
    Ok(catalog::catalog_slugs(models))
}
pub fn read_catalog_no_account_mode(path: &Path) -> Result<bool, String> {
    let bytes = fs::read(path).map_err(|e| format!("IO error: {e}"))?;
    let value: serde_json::Value = serde_json::from_slice(&bytes)
        .map_err(|e| format!("Invalid data: invalid router catalog: {e}"))?;
    consistency::catalog_no_account_mode(&value)
}
/// Exact namespace check @100367e50: first '::', aimami_relay_ provider prefix,
/// actual portable-ID validation, and a nonblank model suffix.
pub fn is_relay_model_slug(slug: &str) -> bool {
    let Some((provider, model)) = slug.split_once("::") else {
        return false;
    };
    provider.starts_with("aimami_relay_")
        && builder::validate_relay_provider_id(provider).is_ok()
        && !model.trim().is_empty()
}
fn auth_context(paths: &VerificationPaths) -> AuthContext {
    AuthContext::new(&paths.codex_home, RouterMode::Off)
}
fn legacy_marker_exists(paths: &VerificationPaths) -> bool {
    fs::metadata(paths.relay_root.join("virtual-auth-marker.json")).is_ok()
}
fn unlock_marker_exists(paths: &VerificationPaths) -> bool {
    fs::metadata(paths.relay_root.join("router-unlock-auth-marker.json")).is_ok()
}
/// Existing auth helpers derive their relay directory from codex_home. Honor an explicit
/// relay root here as well; the live auth itself always comes from codex_home/auth.json.
fn unlock_residue(paths: &VerificationPaths, live: LiveAuthState) -> bool {
    unlock_marker_exists(paths)
        || fs::metadata(paths.relay_root.join("router-unlock-auth-backup.json")).is_ok()
        || matches!(
            live,
            LiveAuthState::VirtualApiKey | LiveAuthState::FlaggedUnlock
        )
}
fn example_suffix(examples: &[String]) -> String {
    if examples.is_empty() {
        String::new()
    } else {
        format!(" examples={}", examples.join("；"))
    }
}
fn verify_provider(paths: &VerificationPaths, target: &str, report: &mut VerificationReport) {
    let i = consistency::inspect_validated_provider_target(
        &paths.databases,
        target,
        &mut consistency::DiagnosticRolloutProviderCache::default(),
    );
    // Native wrapper @100362e6c excludes corrupt-only from its gate, though it prints it.
    if i.db_mismatch != 0
        || i.rollout_mismatch != 0
        || i.missing_rollouts != 0
        || !i.errors.is_empty()
    {
        report.push(InvariantDim::Thread,format!("thread provider convergence target={target} is not clean: db_mismatched={} rollout_mismatched={} missing_rollouts={} unreadable_rollouts={} read_errors={}{}",i.db_mismatch,i.rollout_mismatch,i.missing_rollouts,i.corrupt_rollouts,i.errors.len(),example_suffix(&i.examples)));
    }
}
/// Actual NativeOff invariants need no RelayState document. This is the same
/// native branch used by native verification, and also handles a genuinely absent
/// persisted router without inventing a default state or ignoring auth residues.
pub fn verify_native_off_with_depth(paths: &VerificationPaths, depth: Depth) -> VerificationReport {
    let mut report = VerificationReport::default();
    let text = match fs::read_to_string(&paths.config_path) {
        Ok(text) => text,
        Err(e) if e.kind() == io::ErrorKind::NotFound => String::new(),
        Err(e) => {
            report.push(
                InvariantDim::Config,
                format!("cannot read Codex config.toml: {e}"),
            );
            String::new()
        }
    };
    let surface = config::RouterConfigSurface::parse(&text, &paths.relay_root);
    for message in surface.violations(false, &paths.relay_root) {
        report.push(InvariantDim::Config, message);
    }
    let live = unlock::live_auth_state(&auth_context(paths));
    if [
        "router-takeover-backup.json",
        "codex_router_top_level_backup.json",
    ]
    .iter()
    .any(|p| fs::metadata(paths.relay_root.join(p)).is_ok())
    {
        report.push(
            InvariantDim::Config,
            "NativeOff must not keep takeover backup",
        );
    }
    if legacy_marker_exists(paths) {
        report.push(
            InvariantDim::Auth,
            "NativeOff must not keep a legacy virtual auth marker",
        );
    }
    let managed = matches!(
        live,
        LiveAuthState::VirtualApiKey | LiveAuthState::FlaggedUnlock
    );
    if managed {
        report.push(
            InvariantDim::Auth,
            "NativeOff must not keep virtual unlock auth active",
        );
    }
    if unlock_marker_exists(paths) {
        report.push(
            InvariantDim::Auth,
            "NativeOff must not keep virtual unlock auth marker",
        );
    }
    if unlock_residue(paths, live) && !unlock_marker_exists(paths) && !managed {
        report.push(
            InvariantDim::Auth,
            "NativeOff must not keep an orphaned virtual unlock auth backup",
        );
    }
    if depth == Depth::Full {
        match native_off_provider_target(&text) {
            Ok(target) => verify_provider(paths, &target, &mut report),
            Err(e) => report.push(InvariantDim::Thread, e),
        }
    }
    report
}

pub fn verify_invariants_with_depth(
    context: &VerificationContext<'_>,
    target: TargetState,
    depth: Depth,
) -> VerificationReport {
    let paths = context.paths;
    if target == TargetState::NativeOff {
        return verify_native_off_with_depth(paths, depth);
    }
    let mut report = VerificationReport::default();
    let text = match fs::read_to_string(&paths.config_path) {
        Ok(t) => t,
        Err(e) => {
            if e.kind() != io::ErrorKind::NotFound {
                report.push(
                    InvariantDim::Config,
                    format!("cannot read Codex config.toml: {e}"),
                );
            }
            String::new()
        }
    };
    let router_on = target == TargetState::RouterOn;
    let surface = config::RouterConfigSurface::parse(&text, &paths.relay_root);
    for message in surface.violations(router_on, &paths.relay_root) {
        report.push(InvariantDim::Config, message);
    }
    let live = unlock::live_auth_state(&auth_context(paths));
    let catalog_path = paths.catalog_path();
    if let (Ok(slugs), Some(model)) = (read_catalog_slugs(&catalog_path), surface.model.as_deref())
    {
        let slugs: HashSet<_> = slugs.into_iter().collect();
        if config::top_level_relay_model_requires_cleanup(model, true, &slugs) {
            report.push(
                InvariantDim::Config,
                format!("RouterOn top-level relay model must exist in the active catalog: {model}"),
            );
        }
    }
    let mode = read_catalog_no_account_mode(&catalog_path);
    let requires = toml::from_str::<toml::Value>(&text).ok().and_then(|v| {
        v.get("model_providers")?
            .get("aimai1")?
            .get("requires_openai_auth")?
            .as_bool()
    });
    match requires {
        None => report.push(
            InvariantDim::Config,
            "RouterOn requires a boolean requires_openai_auth provider setting",
        ),
        Some(requires)
            if mode
                .as_ref()
                .is_ok_and(|no_account| requires == *no_account) =>
        {
            report.push(
                InvariantDim::Config,
                "RouterOn requires_openai_auth must match the catalog mode",
            )
        }
        _ => {}
    }
    if legacy_marker_exists(paths) {
        report.push(
            InvariantDim::Auth,
            "RouterOn must not keep a legacy virtual auth marker",
        );
    }
    if unlock_residue(paths, live) {
        report.push(
            InvariantDim::Auth,
            "RouterOn must not keep legacy managed-auth files",
        );
    }
    if fs::metadata(&catalog_path).is_err() {
        report.push(
            InvariantDim::Catalog,
            format!("RouterOn requires catalog file: {}", catalog_path.display()),
        );
    } else {
        match read_catalog_no_account_mode(&catalog_path) {
            Err(e) => report.push(
                InvariantDim::Catalog,
                format!("invalid router catalog mode: {e}"),
            ),
            Ok(no_account) => match read_catalog_slugs(&catalog_path) {
                Err(e) => report.push(
                    InvariantDim::Catalog,
                    format!("RouterOn requires a readable model catalog: {e}"),
                ),
                Ok(slugs) => {
                    if no_account
                        && slugs.iter().any(|s| {
                            catalog::canonical_no_account_slot_slug(s).is_none()
                                && !is_relay_model_slug(s)
                        })
                    {
                        report.push(InvariantDim::Catalog,"RouterOn no-account catalog must contain relay models and legacy aliases only");
                    }
                }
            },
        }
    }
    if depth == Depth::Full {
        verify_provider(paths, "aimai1", &mut report);
    }
    // This inspection intentionally runs even at native depth=1.
    let i = consistency::inspect_router_thread_model_catalog(&paths.databases, &paths.relay_root);
    if i.inconsistent != 0 || !i.errors.is_empty() {
        report.push(InvariantDim::Thread,format!("RouterOn thread models must be catalog-valid or a routable official bare model: invalid={} missing_model={} read_errors={}{}",i.inconsistent,i.missing_models,i.errors.len(),example_suffix(&i.examples)));
    }
    report
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VerificationFlavor {
    Native,
    AtelierResponses,
}
/// An explicit protocol expectation, never an assertion that a configuration is valid.
#[derive(Debug, Clone, Copy)]
pub struct ExpectedConfig<'a> {
    pub flavor: VerificationFlavor,
    provider_id: &'a str,
    expected_base_url: Option<&'a str>,
    expected_auth: Option<bool>,
}
impl<'a> ExpectedConfig<'a> {
    pub fn native(expected_base_url: Option<&'a str>, expected_auth: Option<bool>) -> Self {
        Self {
            flavor: VerificationFlavor::Native,
            provider_id: "aimai1",
            expected_base_url,
            expected_auth,
        }
    }
    /// Atelier server forwards Responses at /v1 and reads ATELIER_GATEWAY_TOKEN;
    /// provider ownership/selection remains the caller's explicit routing policy.
    pub fn atelier_responses(provider_id: &'a str, expected_base_url: &'a str) -> Self {
        Self {
            flavor: VerificationFlavor::AtelierResponses,
            provider_id,
            expected_base_url: Some(expected_base_url),
            expected_auth: Some(false),
        }
    }
    pub fn provider_id(&self) -> &str {
        self.provider_id
    }
}
/// Validate actual parsed TOML against a concrete native or Atelier contract.
/// Native flavor preserves the original four-field algorithm. No network is sent.
pub fn config_contract_issues(
    text: &str,
    expected: &ExpectedConfig<'_>,
) -> Result<Vec<String>, String> {
    let value: toml::Value = toml::from_str(text).map_err(|e| e.to_string())?;
    let Some(provider) = value
        .get("model_providers")
        .and_then(|v| v.get(expected.provider_id))
    else {
        return Ok(vec![format!(
            "缺少 [model_providers.{}]",
            expected.provider_id
        )]);
    };
    let mut issues = vec![];
    let suffix = match expected.flavor {
        VerificationFlavor::Native => "/codex/router/v1",
        VerificationFlavor::AtelierResponses => "/v1",
    };
    let base = provider
        .get("base_url")
        .and_then(toml::Value::as_str)
        .unwrap_or("");
    if !base.trim_end_matches('/').ends_with(suffix) {
        issues.push(format!("base_url 未指向 {suffix}（当前：{base}）"));
    } else if let Some(url) = expected.expected_base_url {
        if base.trim_end_matches('/') != url.trim_end_matches('/') {
            issues.push(format!("base_url 端口漂移，期望 {url}，当前 {base}"));
        }
    }
    if provider.get("wire_api").and_then(toml::Value::as_str) != Some("responses") {
        issues.push("wire_api 必须为 responses".into());
    }
    match provider
        .get("requires_openai_auth")
        .and_then(toml::Value::as_bool)
    {
        None => issues.push("requires_openai_auth 必须为布尔值".into()),
        Some(current) => {
            if let Some(wanted) = expected.expected_auth {
                if current != wanted {
                    issues.push(format!(
                        "requires_openai_auth 必须为 {wanted}（当前：{current}）"
                    ));
                }
            }
        }
    }
    if provider
        .get("supports_websockets")
        .and_then(toml::Value::as_bool)
        != Some(false)
    {
        issues.push("supports_websockets 必须为 false".into());
    }
    if expected.flavor == VerificationFlavor::AtelierResponses
        && provider.get("env_key").and_then(toml::Value::as_str) != Some("ATELIER_GATEWAY_TOKEN")
    {
        issues.push("env_key 必须为 ATELIER_GATEWAY_TOKEN".into());
    }
    Ok(issues)
}
/// Exact native wrapper, without any Atelier namespace/path substitutions.
pub fn router_http_contract_issues(
    text: &str,
    expected_base_url: Option<&str>,
    expected_auth: Option<bool>,
) -> Result<Vec<String>, String> {
    config_contract_issues(
        text,
        &ExpectedConfig::native(expected_base_url, expected_auth),
    )
}
pub fn verify_router_on_http_contract(
    paths: &VerificationPaths,
    expected_base_url: &str,
) -> VerificationReport {
    let mut report = VerificationReport::default();
    let text = match fs::read_to_string(&paths.config_path) {
        Ok(t) => t,
        Err(e) => {
            report.push(
                InvariantDim::Config,
                format!("cannot read Codex config.toml for router HTTP contract: {e}"),
            );
            return report;
        }
    };
    let requires = read_catalog_no_account_mode(&paths.catalog_path())
        .map(|m| !m)
        .unwrap_or(true);
    match router_http_contract_issues(&text, Some(expected_base_url), Some(requires)) {
        Err(e) => report.push(
            InvariantDim::Config,
            format!("cannot parse Codex config.toml for router HTTP contract: {e}"),
        ),
        Ok(issues) if !issues.is_empty() => report.push(
            InvariantDim::Config,
            format!("RouterOn HTTP contract mismatch: {}", issues.join("; ")),
        ),
        _ => {}
    }
    report
}
pub fn verify_target_state_at_with_depth(
    context: &VerificationContext<'_>,
    target: TargetState,
    depth: Depth,
) -> VerificationReport {
    verify_target_state_with_transport(context, target, depth, &LocalPortTransport)
}
pub fn verify_target_state_with_transport(
    context: &VerificationContext<'_>,
    target: TargetState,
    depth: Depth,
    transport: &impl PortTransport,
) -> VerificationReport {
    let mut report = verify_invariants_with_depth(context, target, depth);
    if target == TargetState::NativeOff {
        return report;
    }
    let runtime = match context.proxy.lock() {
        Ok(g) => g,
        Err(_) => {
            report.push(
                InvariantDim::Proxy,
                "RouterOn cannot inspect the local proxy because its state lock is poisoned",
            );
            return report;
        }
    };
    let Some(handle) = runtime.as_ref() else {
        report.push(
            InvariantDim::Proxy,
            "RouterOn requires a live local proxy handle",
        );
        return report;
    };
    if !report.issues.iter().any(|i| i.dim == InvariantDim::Config) {
        report.issues.extend(
            verify_router_on_http_contract(
                context.paths,
                &format!("http://127.0.0.1:{}/codex/router/v1", handle.port),
            )
            .issues,
        );
    }
    if !proxy_handle_port_accepts(handle, transport) {
        report.push(
            InvariantDim::Proxy,
            format!(
                "RouterOn local proxy is not accepting connections on 127.0.0.1:{}",
                handle.port
            ),
        );
    }
    report
}
pub fn has_confirmed_target_state(context: &VerificationContext<'_>, target: TargetState) -> bool {
    has_confirmed_target_state_with_transport(context, target, &LocalPortTransport)
}
pub fn has_confirmed_target_state_with_transport(
    context: &VerificationContext<'_>,
    target: TargetState,
    transport: &impl PortTransport,
) -> bool {
    matches!(
        JournalStore::new(&context.paths.relay_root).commit_pending(),
        Ok(false)
    ) && verify_target_state_with_transport(context, target, Depth::Full, transport)
        .issues
        .is_empty()
}
/// run_diagnostics_with_mode uses depth=1 invariants plus actual config_stale.
/// It does not claim the Manager's proxy/TCP checks or complete target confirmation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PostconditionJournal {
    Clear,
    Committing,
    Unreadable(String),
}
pub struct RouterPostcondition {
    pub verification: VerificationReport,
    pub config_stale: config::DiagnosticItem,
    pub journal: PostconditionJournal,
}
impl RouterPostcondition {
    /// An unreadable checked journal cannot establish a complete target state. Native's
    /// diagnostic still runs its leaf checks after that error; the explicit journal field
    /// preserves that distinction instead of silently dropping the error as the UI did.
    pub fn is_satisfied(&self) -> bool {
        self.journal == PostconditionJournal::Clear
            && self.verification.issues.is_empty()
            && self.config_stale.status == config::Status::Ok
    }
}
pub fn router_postcondition(
    context: &VerificationContext<'_>,
) -> Result<RouterPostcondition, crate::catalog::state::StateError> {
    let state = context.state.diagnostic_state()?;
    let text = fs::read_to_string(&context.paths.config_path).ok();
    let journal = match JournalStore::new(&context.paths.relay_root).commit_pending() {
        Ok(false) => PostconditionJournal::Clear,
        Ok(true) => PostconditionJournal::Committing,
        Err(e) => PostconditionJournal::Unreadable(e.to_string()),
    };
    let verification = if journal == PostconditionJournal::Committing {
        VerificationReport::default()
    } else {
        verify_invariants_with_depth(
            context,
            if state.enabled {
                TargetState::RouterOn
            } else {
                TargetState::NativeOff
            },
            Depth::SkipProviderConvergence,
        )
    };
    Ok(RouterPostcondition {
        verification,
        journal,
        config_stale: config::check_config_stale(
            text.as_deref(),
            &context.paths.relay_root,
            &state,
        ),
    })
}

/// Explicit Atelier contract. This is protocol/inventory input, not a RelayState or a
/// claim that accounts, files, a process, or an upstream request are valid.
pub struct AtelierContract<'a> {
    pub provider_id: &'a str,
    pub expected_base_url: &'a str,
    pub catalog_path: &'a Path,
    pub expected_raw_models: &'a BTreeSet<String>,
    pub selected_model: &'a str,
}
pub enum VerificationContract<'a> {
    Native(&'a RelayStateDocument),
    AtelierResponses(AtelierContract<'a>),
}
pub struct ContractContext<'a> {
    pub paths: &'a VerificationPaths,
    pub proxy: &'a Mutex<Option<ProxyHandle>>,
}
/// Production verification strategy for real native or Atelier storage. Native defaults
/// remain unchanged; Atelier reads its actual raw-ID catalog, effective config, DBs,
/// rollout provider anchors and the owned handle's loopback port.
pub fn verify_with_contract(
    context: &ContractContext<'_>,
    contract: &VerificationContract<'_>,
    target: TargetState,
    depth: Depth,
) -> VerificationReport {
    verify_with_contract_and_transport(context, contract, target, depth, &LocalPortTransport)
}
pub fn verify_with_contract_and_transport(
    context: &ContractContext<'_>,
    contract: &VerificationContract<'_>,
    target: TargetState,
    depth: Depth,
    transport: &impl PortTransport,
) -> VerificationReport {
    let VerificationContract::AtelierResponses(expected) = contract else {
        let VerificationContract::Native(state) = contract else {
            unreachable!()
        };
        return verify_target_state_with_transport(
            &VerificationContext {
                paths: context.paths,
                state,
                proxy: context.proxy,
            },
            target,
            depth,
            transport,
        );
    };
    let mut report = VerificationReport::default();
    let text = match fs::read_to_string(&context.paths.config_path) {
        Ok(s) => s,
        Err(e) => {
            report.push(
                InvariantDim::Config,
                format!("cannot read Atelier Codex config: {e}"),
            );
            String::new()
        }
    };
    let config = match toml::from_str::<toml::Value>(&text) {
        Ok(v) => Some(v),
        Err(e) => {
            report.push(
                InvariantDim::Config,
                format!("cannot parse Atelier Codex config: {e}"),
            );
            None
        }
    };
    let effective = config.as_ref().and_then(effective_config_provider);
    if target == TargetState::NativeOff {
        if effective.is_some_and(atelier_provider_owned) {
            report.push(
                InvariantDim::Config,
                "Atelier NativeOff must not retain an active Atelier provider",
            );
        }
        if let Some(value) = config
            .as_ref()
            .and_then(|c| c.get("model_catalog_json"))
            .and_then(toml::Value::as_str)
        {
            let p = Path::new(value);
            let p = if p.is_absolute() {
                p.to_path_buf()
            } else {
                context.paths.codex_home.join(p)
            };
            if config::normalize_catalog_path(&p.to_string_lossy())
                == config::normalize_catalog_path(&expected.catalog_path.to_string_lossy())
            {
                report.push(
                    InvariantDim::Config,
                    "Atelier NativeOff must not retain its managed model_catalog_json",
                );
            }
        }
        if depth == Depth::Full {
            match native_off_provider_target(&text) {
                Ok(target) => verify_atelier_threads(context.paths, &target, None, &mut report),
                Err(e) => report.push(InvariantDim::Thread, e),
            }
        }
        return report;
    }
    if !atelier_provider_owned(expected.provider_id) {
        report.push(
            InvariantDim::Config,
            "expected Atelier provider ID is outside Atelier ownership",
        );
    }
    if effective != Some(expected.provider_id) {
        report.push(
            InvariantDim::Config,
            format!("effective model_provider must be {}", expected.provider_id),
        );
    }
    match config_contract_issues(
        &text,
        &ExpectedConfig::atelier_responses(expected.provider_id, expected.expected_base_url),
    ) {
        Ok(issues) => {
            for issue in issues {
                report.push(InvariantDim::Config, issue)
            }
        }
        Err(e) => report.push(InvariantDim::Config, e),
    }
    let configured = config
        .as_ref()
        .and_then(|c| c.get("model_catalog_json"))
        .and_then(toml::Value::as_str);
    if configured.is_none_or(|p| {
        let p = Path::new(p);
        let p = if p.is_absolute() {
            p.to_path_buf()
        } else {
            context.paths.codex_home.join(p)
        };
        config::normalize_catalog_path(&p.to_string_lossy())
            != config::normalize_catalog_path(&expected.catalog_path.to_string_lossy())
    }) {
        report.push(
            InvariantDim::Config,
            "Atelier model_catalog_json does not reference the actual managed catalog",
        );
    }
    match read_catalog_slugs(expected.catalog_path) {
        Err(e) => report.push(InvariantDim::Catalog, e),
        Ok(actual) if &actual != expected.expected_raw_models => report.push(
            InvariantDim::Catalog,
            "Atelier catalog raw model inventory differs from its preferred profile",
        ),
        Ok(_) => (),
    }
    if expected.selected_model.is_empty()
        || !expected
            .expected_raw_models
            .contains(expected.selected_model)
    {
        report.push(
            InvariantDim::Catalog,
            "Atelier selected model has no preferred-profile inventory",
        );
    }
    if let Some(config) = &config {
        let selected = effective_config_model(config);
        if selected != Some(expected.selected_model) {
            report.push(
                InvariantDim::Config,
                "effective model differs from Atelier gateway settings.model",
            );
        }
    }
    verify_atelier_threads(
        context.paths,
        expected.provider_id,
        Some(expected.expected_raw_models),
        &mut report,
    );
    match context.proxy.lock() {
        Err(_) => report.push(InvariantDim::Proxy, "Atelier proxy handle lock is poisoned"),
        Ok(guard) => match guard.as_ref() {
            None => report.push(
                InvariantDim::Proxy,
                "Atelier RouterOn requires its owned live proxy handle",
            ),
            Some(handle) => {
                if expected.expected_base_url.trim_end_matches('/')
                    != format!("http://127.0.0.1:{}/v1", handle.port)
                {
                    report.push(
                        InvariantDim::Proxy,
                        "Atelier owned proxy port differs from its configured gateway port",
                    );
                }
                if !proxy_handle_port_accepts(handle, transport) {
                    report.push(
                        InvariantDim::Proxy,
                        format!(
                            "Atelier local proxy is not accepting connections on 127.0.0.1:{}",
                            handle.port
                        ),
                    );
                }
            }
        },
    }
    report
}
pub fn atelier_provider_owned(id: &str) -> bool {
    id == "atelier" || id == "codex_atelier" || id.starts_with("atelier_")
}
/// Only an explicitly authenticated, unredirected official connection is an alias.
pub fn official_provider_target(config: &serde_json::Value) -> Result<String, String> {
    let provider = match config.get("model_provider") {
        None | Some(serde_json::Value::Null) => "openai",
        Some(serde_json::Value::String(s)) => s,
        _ => return Err("model_provider 类型无效".into()),
    };
    let official_url = |v: &serde_json::Value| v.is_null() || v.as_str().is_some_and(|s|
        s.trim_end_matches('/') == "https://chatgpt.com/backend-api/codex");
    let p = &config["model_providers"][provider];
    let plain = ["env_key", "experimental_bearer_token", "auth", "gateway_oauth", "model_catalog_url"]
        .iter().all(|k| p[*k].is_null())
        && ["http_headers", "env_http_headers", "query_params"].iter()
            .all(|k| p[*k].is_null() || p[*k].as_object().is_some_and(|v| v.is_empty()));
    if official_url(&config["openai_base_url"])
        && plain && official_url(&p["base_url"])
        && (provider == "openai" || p["requires_openai_auth"] == true)
    {
        Ok(provider.to_owned())
    } else {
        Err("无法确证官方 Provider；不自动接管第三方/Gateway 线程".into())
    }
}
pub fn native_off_provider_target(text: &str) -> Result<String, String> {
    let config: toml::Value = toml::from_str(text).map_err(|e| e.to_string())?;
    let mut effective = serde_json::to_value(&config).map_err(|e| e.to_string())?;
    if let Some(provider) = effective_config_provider(&config) {
        effective["model_provider"] = provider.into();
    }
    official_provider_target(&effective)
}
fn effective_config_provider(config: &toml::Value) -> Option<&str> {
    config
        .get("profile")
        .and_then(toml::Value::as_str)
        .and_then(|p| {
            config
                .get("profiles")?
                .get(p)?
                .get("model_provider")?
                .as_str()
        })
        .or_else(|| config.get("model_provider").and_then(toml::Value::as_str))
}
fn effective_config_model(config: &toml::Value) -> Option<&str> {
    config
        .get("profile")
        .and_then(toml::Value::as_str)
        .and_then(|p| config.get("profiles")?.get(p)?.get("model")?.as_str())
        .or_else(|| config.get("model").and_then(toml::Value::as_str))
}
fn verify_atelier_threads(
    paths: &VerificationPaths,
    target: &str,
    models: Option<&BTreeSet<String>>,
    report: &mut VerificationReport,
) {
    // NativeOff reuses the recovered native gate, including external-history and
    // corrupt-only policy. Atelier-owned rows are checked separately below.
    if models.is_none() {
        verify_provider(paths, target, report);
    }
    for db in &paths.databases {
        if !db.exists() {
            continue;
        }
        let inspected = (|| -> Result<(), String> {
            let c = rusqlite::Connection::open_with_flags(
                db,
                rusqlite::OpenFlags::SQLITE_OPEN_READ_ONLY
                    | rusqlite::OpenFlags::SQLITE_OPEN_NO_MUTEX,
            )
            .map_err(|e| e.to_string())?;
            c.busy_timeout(Duration::from_secs(3))
                .map_err(|e| e.to_string())?;
            let mut columns = c
                .prepare("PRAGMA table_info(threads)")
                .map_err(|e| e.to_string())?;
            let has_model = columns
                .query_map([], |r| r.get::<_, String>(1))
                .map_err(|e| e.to_string())?
                .collect::<rusqlite::Result<Vec<_>>>()
                .map_err(|e| e.to_string())?
                .iter()
                .any(|c| c == "model");
            let query=format!("SELECT id,COALESCE(model_provider,''),rollout_path,{},COALESCE(source,'') FROM threads WHERE COALESCE(archived,0)=0 ORDER BY id",if has_model{"model"}else{"NULL"});
            let mut statement = c.prepare(&query).map_err(|e| e.to_string())?;
            let rows = statement
                .query_map([], |r| {
                    Ok((
                        r.get::<_, String>(0)?,
                        r.get::<_, String>(1)?,
                        r.get::<_, Option<String>>(2)?,
                        r.get::<_, Option<String>>(3)?,
                        r.get::<_, String>(4)?,
                    ))
                })
                .map_err(|e| e.to_string())?;
            for row in rows {
                let (id, provider, rollout, model, source) = row.map_err(|e| e.to_string())?;
                if source.contains("\"subagent\"")
                    || (models.is_none() && !atelier_provider_owned(&provider))
                    || !(atelier_provider_owned(&provider)
                        || crate::threads::is_router_or_openai_provider(&provider))
                {
                    continue;
                }
                // 数据库 provider 是 Codex 的真实来源；不匹配才是实质问题。
                let db_converged = provider == target;
                if !db_converged {
                    report.push(
                        InvariantDim::Thread,
                        format!("Atelier thread {id} provider is {provider}, expected {target}"),
                    );
                }
                if let Some(models) = models {
                    if model.as_ref().is_none_or(|m| !models.contains(m)) {
                        report.push(
                            InvariantDim::Thread,
                            format!(
                                "Atelier thread {id} model is outside the routable raw inventory"
                            ),
                        );
                    }
                }
                let Some(path) = rollout else {
                    report.push(
                        InvariantDim::Thread,
                        format!("Atelier thread {id} has no rollout path"),
                    );
                    continue;
                };
                let path = Path::new(&path);
                let path = if path.is_absolute() {
                    path.to_path_buf()
                } else {
                    paths.codex_home.join(path)
                };
                if !crate::threads::rollout_file_belongs_to_thread(&path, &id) {
                    report.push(InvariantDim::Thread,format!("Atelier thread {id} rollout identity is missing, unreadable, or mismatched"));
                    continue;
                }
                match consistency::read_rollout_provider_anchors(&path) {
                    Ok(Some(a)) => {
                        let rollout_matches = [&a.first, &a.last, &a.native]
                            .into_iter()
                            .flatten()
                            .all(|p| p.as_str() == target);
                        if !rollout_matches {
                            report.push(
                                InvariantDim::Thread,
                                format!(
                                    "Atelier thread {id} rollout provider differs from {target}"
                                ),
                            );
                        }
                    }
                    Ok(None) => report.push(
                        InvariantDim::Thread,
                        format!("Atelier thread {id} has no session_meta"),
                    ),
                    Err(e) => report.push(
                        InvariantDim::Thread,
                        format!("Atelier thread {id} rollout cannot be inspected: {e:?}"),
                    ),
                }
            }
            Ok(())
        })();
        if let Err(e) = inspected {
            report.push(
                InvariantDim::Thread,
                format!(
                    "Atelier thread database {} cannot be inspected: {e}",
                    db.display()
                ),
            );
        }
    }
}

#[cfg(test)]
mod official_target_tests {
    use super::*;
    #[test]
    fn official_target_preserves_alias_and_rejects_unproven_connections() {
        assert_eq!(native_off_provider_target("model_provider='openai'").unwrap(), "openai");
        let alias = "model_provider='openai_https'\n[model_providers.openai_https]\nrequires_openai_auth=true\n";
        assert_eq!(native_off_provider_target(alias).unwrap(), "openai_https");
        assert!(native_off_provider_target("model_provider='third_party'").is_err());
        assert!(native_off_provider_target(&format!("{alias}base_url='https://example.invalid/v1'\n")).is_err());
        assert!(native_off_provider_target("model_provider='openai'\nopenai_base_url='https://example.invalid/v1'").is_err());
    }
}
