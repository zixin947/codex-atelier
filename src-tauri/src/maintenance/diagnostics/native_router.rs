//! Explicit production router primitives for Native RelayState and Atelier Catalog.
//! No discovery, process launch, credential refresh, or implicit namespace conversion.
use super::{err, inspect, native_auth, native_catalog, Context, DiagnosticItem, Result};
use crate::gateway::{
    storage::{atomic_private, private_dir, read_private},
    types::{Catalog, Credential, ProfileKind},
};
use atelier_maintenance_core::{
    auth::RouterMode,
    catalog::{self, publication, state::RelayStateDocument},
    config,
    router_verification::{
        self as core, AtelierContract, ContractContext, Depth, InvariantDim, ProxyHandle,
        TargetState, VerificationContract, VerificationIssue, VerificationPaths,
        VerificationReport,
    },
};
use sha2::{Digest, Sha256};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs, io,
    path::{Path, PathBuf},
    sync::Mutex,
};

#[derive(Clone)]
pub(super) enum Snapshot {
    Native {
        source: native_catalog::Source,
        document: RelayStateDocument,
        bytes: Vec<u8>,
    },
    Gateway {
        source: native_catalog::Source,
        catalog: Catalog,
        target: TargetState,
        provider: Option<String>,
        intent_before: Option<Vec<u8>>,
        bytes: Vec<u8>,
    },
}
impl Snapshot {
    pub fn target(&self) -> TargetState {
        match self {
            Self::Native { document, .. } => {
                if document
                    .catalog_inputs()
                    .expect("validated native snapshot")
                    .router_enabled
                {
                    TargetState::RouterOn
                } else {
                    TargetState::NativeOff
                }
            }
            Self::Gateway { target, .. } => *target,
        }
    }
    pub fn source(&self) -> &native_catalog::Source {
        match self {
            Self::Native { source, .. } | Self::Gateway { source, .. } => source,
        }
    }
    pub fn native_document(&self) -> Option<&RelayStateDocument> {
        match self {
            Self::Native { document, .. } => Some(document),
            _ => None,
        }
    }
    pub fn gateway_catalog(&self) -> Option<&Catalog> {
        match self {
            Self::Gateway { catalog, .. } => Some(catalog),
            _ => None,
        }
    }
    pub fn source_revision(&self) -> String {
        format!("{:x}", Sha256::digest(self.bytes()))
    }
    pub fn source_dependency(&self) -> (PathBuf, Option<(usize, String)>) {
        (
            source_path(self.source()).to_owned(),
            effect_stamp(Some(self.bytes())),
        )
    }
    pub fn intent_dependency(&self, ctx: &Context) -> Option<(PathBuf, Option<(usize, String)>)> {
        match self {
            Self::Gateway { intent_before, .. } => Some((
                super::router_intent::path(ctx),
                effect_stamp(intent_before.as_deref()),
            )),
            _ => None,
        }
    }
    fn bytes(&self) -> &[u8] {
        match self {
            Self::Native { bytes, .. } | Self::Gateway { bytes, .. } => bytes,
        }
    }
}
impl atelier_maintenance_core::manager::RepairSnapshot for Snapshot {
    fn target(&self) -> TargetState {
        Snapshot::target(self)
    }
}

fn source_path(source: &native_catalog::Source) -> &Path {
    match source {
        native_catalog::Source::Native { state_path, .. } => state_path,
        native_catalog::Source::Gateway { catalog_path, .. } => catalog_path,
    }
}
fn source_root(source: &native_catalog::Source) -> &Path {
    match source {
        native_catalog::Source::Native { catalog_root, .. }
        | native_catalog::Source::Gateway { catalog_root, .. } => catalog_root,
    }
}
fn read(path: &Path) -> Result<Option<Vec<u8>>> {
    match fs::symlink_metadata(path) {
        Err(e) if e.kind() == io::ErrorKind::NotFound => Ok(None),
        Err(e) => Err(err(e)),
        Ok(m) => {
            if !m.is_file() || m.file_type().is_symlink() || m.len() > 8 * 1024 * 1024 {
                return Err("路由依赖不是8MiB以内普通文件".into());
            }
            let bytes = fs::read(path).map_err(err)?;
            if bytes.len() > 8 * 1024 * 1024 {
                return Err("路由依赖读取超出上限".into());
            }
            Ok(Some(bytes))
        }
    }
}
pub(super) fn snapshot(ctx: &Context, source: &native_catalog::Source) -> Result<Snapshot> {
    let bytes = read(source_path(source))?.ok_or("路由来源不存在，未构造默认成功状态")?;
    match source {
        native_catalog::Source::Native { .. } => {
            let v: serde_json::Value = serde_json::from_slice(&bytes).map_err(err)?;
            if v.get("hostId").is_some() || v.get("profiles").is_some() {
                return Err("Atelier Catalog不能作为原生RelayState读取".into());
            }
            let document = RelayStateDocument::from_json(&bytes).map_err(err)?;
            document.catalog_inputs().map_err(err)?;
            Ok(Snapshot::Native {
                source: source.clone(),
                document,
                bytes,
            })
        }
        native_catalog::Source::Gateway { .. } => {
            let catalog: Catalog = serde_json::from_slice(&bytes)
                .map_err(|_| "Atelier路由来源不符合实际Catalog schema")?;
            if catalog.host_id.is_empty()
                || catalog.profiles.len() > 64
                || catalog.settings.port == 0
            {
                return Err("Atelier Catalog host/profiles/port无效".into());
            }
            let intent = super::router_intent::prepare(ctx)?;
            Ok(Snapshot::Gateway {
                source: source.clone(),
                catalog,
                target: intent.observation().record.target,
                provider: intent.observation().record.provider.clone(),
                intent_before: read(&super::router_intent::path(ctx))?,
                bytes,
            })
        }
    }
}
fn paths(ctx: &Context, snapshot: &Snapshot) -> Result<VerificationPaths> {
    let text = read(&ctx.config())?
        .map(|b| String::from_utf8(b).map_err(err))
        .transpose()?;
    let configured = atelier_maintenance_core::state_db::configured_sqlite_home(
        text.as_deref(),
        ctx.sqlite_home_env.as_deref(),
        ctx.working_directory.as_deref(),
    );
    let dirs = configured
        .map(|d| vec![d])
        .unwrap_or_else(|| vec![ctx.home.join("sqlite"), ctx.home.clone()]);
    let mut databases = vec![];
    for dir in dirs {
        if let Some(db) = atelier_maintenance_core::state_db::highest_state_db(&dir) {
            if !databases.contains(&db) {
                databases.push(db);
            }
        }
    }
    Ok(VerificationPaths {
        codex_home: ctx.home.clone(),
        relay_root: source_root(snapshot.source()).to_path_buf(),
        config_path: ctx.config(),
        databases,
    })
}
fn push(report: &mut VerificationReport, dim: InvariantDim, message: impl Into<String>) {
    report.issues.push(VerificationIssue {
        dim,
        message: message.into(),
    });
}
fn provider(text: &str) -> Result<String> {
    let c = toml::from_str(text).map_err(|_| "Codex配置不是有效TOML")?;
    Ok(inspect::provider(&c))
}
fn native_proxy(ctx: &Context, snapshot: &Snapshot) -> Option<ProxyHandle> {
    if matches!(snapshot, Snapshot::Gateway { .. }) {
        ctx.proxy_handle.clone()
    } else {
        None
    }
}
/// Default runtime ownership is Gateway-only. A native Manager must supply its own actual
/// live handle through verify_with_proxy; persisted state.proxy never substitutes for it.
pub(super) fn verify(
    ctx: &Context,
    snapshot: &Snapshot,
    depth: Depth,
) -> Result<VerificationReport> {
    verify_with_proxy(ctx, snapshot, depth, native_proxy(ctx, snapshot))
}
pub(super) fn verify_with_proxy(
    ctx: &Context,
    snapshot: &Snapshot,
    depth: Depth,
    actual_proxy: Option<ProxyHandle>,
) -> Result<VerificationReport> {
    verify_target_with_proxy(ctx, snapshot, snapshot.target(), depth, actual_proxy)
}
pub(super) fn verify_target(
    ctx: &Context,
    snapshot: &Snapshot,
    target: TargetState,
    depth: Depth,
) -> Result<VerificationReport> {
    verify_target_with_proxy(ctx, snapshot, target, depth, native_proxy(ctx, snapshot))
}
pub(super) fn verify_target_with_proxy(
    ctx: &Context,
    snapshot: &Snapshot,
    target: TargetState,
    depth: Depth,
    actual_proxy: Option<ProxyHandle>,
) -> Result<VerificationReport> {
    let paths = paths(ctx, snapshot)?;
    super::snapshot::with_verification_paths(&paths, |projected| {
        let mut report = verify_projected(
            ctx,
            snapshot,
            target,
            depth,
            actual_proxy.clone(),
            projected,
        )?;
        for issue in &mut report.issues {
            for (copy, source) in projected.databases.iter().zip(&paths.databases) {
                issue.message = issue.message.replace(
                    copy.to_string_lossy().as_ref(),
                    source.to_string_lossy().as_ref(),
                );
            }
        }
        Ok(report)
    })
}
fn verify_projected(
    ctx: &Context,
    snapshot: &Snapshot,
    target: TargetState,
    depth: Depth,
    actual_proxy: Option<ProxyHandle>,
    paths: &VerificationPaths,
) -> Result<VerificationReport> {
    let proxy = Mutex::new(actual_proxy);
    let context = ContractContext {
        paths,
        proxy: &proxy,
    };
    let mut report = match snapshot {
        Snapshot::Native { document, .. } => core::verify_with_contract(
            &context,
            &VerificationContract::Native(document),
            target,
            depth,
        ),
        Snapshot::Gateway {
            catalog,
            source,
            provider: expected_provider,
            ..
        } => {
            let text = read(&ctx.config())?
                .map(|b| String::from_utf8(b).map_err(err))
                .transpose()?
                .unwrap_or_default();
            let actual_provider = provider(&text)?;
            let provider = expected_provider.clone().unwrap_or_else(|| {
                if core::atelier_provider_owned(&actual_provider) {
                    actual_provider
                } else {
                    "atelier".into()
                }
            });
            let base = format!("http://127.0.0.1:{}/v1", catalog.settings.port);
            let inventory = catalog
                .profiles
                .iter()
                .find(|p| Some(&p.id) == catalog.settings.preferred_profile_id.as_ref())
                .map(|p| {
                    p.models
                        .iter()
                        .map(|m| m.id.clone())
                        .collect::<BTreeSet<_>>()
                })
                .unwrap_or_default();
            let path = source_root(source).join(catalog::CATALOG_FILE);
            core::verify_with_contract(
                &context,
                &VerificationContract::AtelierResponses(AtelierContract {
                    provider_id: &provider,
                    expected_base_url: &base,
                    catalog_path: &path,
                    expected_raw_models: &inventory,
                    selected_model: &catalog.settings.model,
                }),
                target,
                depth,
            )
        }
    };
    if read(source_path(snapshot.source()))?.as_deref() != Some(snapshot.bytes()) {
        push(
            &mut report,
            InvariantDim::Config,
            "路由来源在snapshot后变化，需重新读取再验证",
        );
    }
    if let Snapshot::Gateway { intent_before, .. } = snapshot {
        if &read(&super::router_intent::path(ctx))? != intent_before {
            push(
                &mut report,
                InvariantDim::Config,
                "路由意图在snapshot后变化，需重新读取再验证",
            );
        }
    }
    if target == TargetState::RouterOn {
        availability(snapshot, &mut report, &mut vec![]);
    }
    Ok(report)
}
fn availability(snapshot: &Snapshot, report: &mut VerificationReport, warnings: &mut Vec<String>) {
    match snapshot {
        Snapshot::Native { document, .. } => {
            let inputs = match document.catalog_inputs() {
                Ok(i) => i,
                Err(e) => {
                    push(report, InvariantDim::Auth, e.to_string());
                    return;
                }
            };
            if !inputs.providers.iter().any(|p| {
                inputs.active_provider_ids.contains(&p.id)
                    && (!p.model.is_empty() || !p.models.is_empty())
            }) {
                push(
                    report,
                    InvariantDim::Catalog,
                    "原生路由没有可路由的active provider模型",
                );
            }
            if inputs
                .providers
                .iter()
                .any(|p| inputs.active_provider_ids.contains(&p.id))
            {
                push(
                    report,
                    InvariantDim::Auth,
                    "原生上游凭据尚未由指定凭据存储确认",
                );
            }
        }
        Snapshot::Gateway {
            catalog, source, ..
        } => {
            let Some(selected) = catalog
                .profiles
                .iter()
                .find(|p| Some(&p.id) == catalog.settings.preferred_profile_id.as_ref())
            else {
                push(
                    report,
                    InvariantDim::Auth,
                    "Atelier没有preferred profile，不能生成",
                );
                return;
            };
            let now = crate::gateway::types::now_ms();
            let mut candidates = catalog
                .profiles
                .iter()
                .filter(|p| {
                    p.kind == selected.kind
                        && p.enabled
                        && (p.auth_status == "ready"
                            || (p.kind == ProfileKind::ApiKey && p.auth_status == "warning"))
                        && p.cooldown_until.is_none_or(|t| t <= now)
                        && !catalog.settings.model.is_empty()
                        && p.models.iter().any(|m| m.id == catalog.settings.model)
                        && (catalog.settings.auto_switch || p.id == selected.id)
                })
                .collect::<Vec<_>>();
            candidates.sort_by_key(|p| (p.id != selected.id, p.priority));
            candidates.truncate(usize::from(catalog.settings.max_attempts));
            if candidates.is_empty() {
                push(
                    report,
                    InvariantDim::Auth,
                    "Atelier没有符合模型、启用、授权与冷却条件的候选账号",
                );
                return;
            }
            let root = source_path(source)
                .parent()
                .unwrap_or_else(|| Path::new(""));
            let mut details = vec![];
            for profile in candidates {
                if uuid::Uuid::parse_str(&profile.id).is_err() {
                    details.push("候选账号ID无效");
                    continue;
                }
                if profile.kind == ProfileKind::Codex {
                    warnings.push("Codex 登录账号由账号模块提供凭据；此处未在线核验".into());
                    continue;
                }
                let path = root
                    .join("credentials")
                    .join(format!("{}.json", profile.id));
                let credential = read_private(&path, 128 * 1024).and_then(|b| {
                    serde_json::from_slice::<Credential>(&b)
                        .map_err(|_| "账号凭据JSON格式无效".into())
                });
                let credential = match credential {
                    Ok(c) => c,
                    Err(_) => {
                        details.push("候选账号凭据不可读或格式无效");
                        continue;
                    }
                };
                let ready = match profile.kind {
                    ProfileKind::Codex => false,
                    ProfileKind::ApiKey => {
                        credential.api_key.as_ref().is_some_and(|k| !k.is_empty())
                    }
                    ProfileKind::Chatgpt => {
                        credential
                            .expires_at
                            .is_some_and(|t| t > now.saturating_add(60_000))
                            && credential.access_token.is_some()
                    }
                };
                if ready {
                    return;
                }
                if profile.kind == ProfileKind::Chatgpt
                    && credential.refresh_token.is_some()
                    && credential.client_id.is_some()
                {
                    warnings
                        .push("ChatGPT候选的本地刷新材料齐备；在线授权与刷新结果尚未核验".into());
                    return;
                }
                details.push("候选账号缺少本地认证材料");
            }
            push(report, InvariantDim::Auth, details.join("；"));
        }
    }
}
pub(super) fn availability_warnings(snapshot: &Snapshot) -> Vec<String> {
    let mut warnings = vec![];
    if snapshot.target() == TargetState::RouterOn {
        availability(snapshot, &mut VerificationReport::default(), &mut warnings);
    }
    warnings
}
pub(super) fn stale_reason(ctx: &Context, snapshot: &Snapshot) -> Result<Option<String>> {
    stale_reason_for_target(ctx, snapshot, snapshot.target())
}
pub(super) fn stale_reason_for_target(
    ctx: &Context,
    snapshot: &Snapshot,
    target: TargetState,
) -> Result<Option<String>> {
    let report =
        verify_target_with_proxy(ctx, snapshot, target, Depth::SkipProviderConvergence, None)?;
    let reasons = report
        .issues
        .into_iter()
        .filter(|i| matches!(i.dim, InvariantDim::Config | InvariantDim::Catalog))
        .map(|i| i.message)
        .collect::<Vec<_>>();
    Ok((!reasons.is_empty()).then(|| reasons.join("；")))
}
pub(super) fn journal_root(ctx: &Context, snapshot: &Snapshot) -> PathBuf {
    match snapshot {
        Snapshot::Native { source, .. } => source_root(source).to_owned(),
        Snapshot::Gateway { .. } => ctx.app_root.join("gateway"),
    }
}
pub(super) fn diagnose(ctx: &Context, source: &native_catalog::Source) -> DiagnosticItem {
    let checked = snapshot(ctx, source).and_then(|s| {
        let journal =
            atelier_maintenance_core::transition_journal::JournalStore::new(&journal_root(ctx, &s));
        let pending = journal.commit_pending();
        let mut r = if matches!(pending, Ok(true)) {
            VerificationReport::default()
        } else {
            verify(ctx, &s, Depth::Full)?
        };
        match pending {
            Ok(true) => push(
                &mut r,
                InvariantDim::Config,
                "ROUTER_COMMIT_RECOVERY_REQUIRED: 路由提交尚未确认",
            ),
            Err(e) => push(
                &mut r,
                InvariantDim::Config,
                format!("路由恢复记录不可读：{e}"),
            ),
            _ => (),
        }
        Ok((r, availability_warnings(&s), s.target()))
    });
    match checked {
        Ok((r, warnings, target)) => DiagnosticItem {
            id: "router_postcondition".into(),
            group: "router".into(),
            label: "路由完整目标状态".into(),
            status: if r.issues.is_empty() { "ok" } else { "error" }.into(),
            detail: if r.issues.is_empty() {
                if target == TargetState::NativeOff {
                    "路由关闭目标的实际配置与线程检查通过。".into()
                } else {
                    format!(
                        "实际配置、目录、账号本地材料、线程与owned proxy TCP检查通过。{}",
                        warnings.join("；")
                    )
                }
            } else {
                r.issues
                    .iter()
                    .map(|i| format!("{:?}: {}", i.dim, i.message))
                    .collect::<Vec<_>>()
                    .join("；")
            },
            fixable: !r.issues.is_empty(),
        },
        Err(e) => DiagnosticItem {
            id: "router_postcondition".into(),
            group: "router".into(),
            label: "路由完整目标状态".into(),
            status: "error".into(),
            detail: e,
            fixable: false,
        },
    }
}

#[derive(Clone)]
pub(super) struct ReviewedPlan {
    snapshot: Snapshot,
    target: TargetState,
    home: PathBuf,
    app_root: PathBuf,
    backup_root: PathBuf,
    config_before: Option<Vec<u8>>,
    config_after: Option<Vec<u8>>,
    catalog: Option<native_catalog::PreparedAction>,
    auth: Vec<native_auth::PreparedAction>,
    dependencies: BTreeMap<PathBuf, Option<Vec<u8>>>,
    mode: RouterMode,
    actual_proxy: Option<ProxyHandle>,
    cli_signature: Option<publication::CliSignature>,
    native_primary: Option<NativePrimaryComposition>,
    batch_seed_only: bool,
}
impl ReviewedPlan {
    pub fn config_effect(&self) -> (PathBuf, Option<&[u8]>, Option<&[u8]>) {
        (
            self.home.join("config.toml"),
            self.config_before.as_deref(),
            self.config_after.as_deref(),
        )
    }
    pub fn approved_dependencies(&self) -> Vec<(PathBuf, Option<(usize, String)>)> {
        self.dependencies
            .iter()
            .map(|(p, b)| (p.clone(), effect_stamp(b.as_deref())))
            .collect()
    }
    pub fn config_before(&self) -> Option<&[u8]> {
        self.config_before.as_deref()
    }
    pub fn config_after(&self) -> Option<&[u8]> {
        self.config_after.as_deref()
    }
    pub fn source(&self) -> &native_catalog::Source {
        self.snapshot.source()
    }
}
fn mode(snapshot: &Snapshot, target: TargetState) -> Result<RouterMode> {
    Ok(match snapshot {
        Snapshot::Native { document, .. } => {
            let i = document.catalog_inputs().map_err(err)?;
            if target == TargetState::NativeOff {
                RouterMode::Off
            } else if i.no_account_mode {
                RouterMode::NoAccount
            } else {
                RouterMode::Native
            }
        }
        Snapshot::Gateway { .. } => {
            if target == TargetState::RouterOn {
                RouterMode::Native
            } else {
                RouterMode::Off
            }
        }
    })
}
fn provider_table<'a>(
    doc: &'a mut toml_edit::DocumentMut,
    id: &str,
) -> Result<&'a mut dyn toml_edit::TableLike> {
    if !doc.contains_key("model_providers") {
        doc.insert(
            "model_providers",
            toml_edit::Item::Table(toml_edit::Table::new()),
        );
    }
    let providers = doc
        .get_mut("model_providers")
        .and_then(toml_edit::Item::as_table_like_mut)
        .ok_or("model_providers不是配置表，未覆盖原值")?;
    if providers.get(id).is_none() {
        providers.insert(id, toml_edit::Item::Table(toml_edit::Table::new()));
    }
    providers
        .get_mut(id)
        .and_then(toml_edit::Item::as_table_like_mut)
        .ok_or_else(|| "Provider不是配置表，未覆盖原值".into())
}
fn native_off_config(text: &str, root: &Path) -> Result<String> {
    let clean = config::strip_all_managed_blocks(text);
    let surface = config::RouterConfigSurface::parse(&clean, root);
    let mut doc = clean
        .parse::<toml_edit::DocumentMut>()
        .map_err(|_| "原生退出配置不是有效TOML")?;
    if surface
        .provider
        .as_deref()
        .is_some_and(atelier_maintenance_core::threads::is_aimami_provider)
    {
        doc.remove("model_provider");
    }
    if surface
        .profile
        .as_deref()
        .is_some_and(atelier_maintenance_core::threads::is_aimami_provider)
    {
        doc.remove("profile");
    }
    if surface.legacy_url {
        doc.remove("openai_base_url");
    }
    if surface.own_catalog {
        doc.remove("model_catalog_json");
    }
    if surface.model.as_deref().is_some_and(|m| {
        config::top_level_relay_model_requires_cleanup(m, false, &std::collections::HashSet::new())
    }) {
        doc.remove("model");
    }
    for name in ["model_providers", "profiles"] {
        if let Some(table) = doc
            .get_mut(name)
            .and_then(toml_edit::Item::as_table_like_mut)
        {
            let owned = table
                .iter()
                .filter(|(key, _)| atelier_maintenance_core::threads::is_aimami_provider(key))
                .map(|(key, _)| key.to_owned())
                .collect::<Vec<_>>();
            for key in owned {
                table.remove(&key);
            }
        }
    }
    Ok(doc.to_string())
}
fn config_patch(
    ctx: &Context,
    snapshot: &Snapshot,
    target: TargetState,
    text: &str,
    proxy: Option<&ProxyHandle>,
) -> Result<String> {
    let on = target == TargetState::RouterOn;
    let root = source_root(snapshot.source());
    let mut doc = text
        .parse::<toml_edit::DocumentMut>()
        .map_err(|_| "配置不是有效TOML，未批准重写")?;
    match snapshot {
        Snapshot::Native { document, .. } => {
            if !on {
                return native_off_config(text, root);
            }
            let handle = proxy.ok_or("原生proxy所有权未知，不能根据持久化proxy状态猜端口")?;
            let input = document.catalog_inputs().map_err(err)?;
            doc.remove("profile");
            doc["model_provider"] = toml_edit::value("aimai1");
            let provider = provider_table(&mut doc, "aimai1")?;
            provider.insert(
                "base_url",
                toml_edit::value(format!("http://127.0.0.1:{}/codex/router/v1", handle.port)),
            );
            provider.insert("wire_api", toml_edit::value("responses"));
            provider.insert(
                "requires_openai_auth",
                toml_edit::value(!input.no_account_mode),
            );
            provider.insert("supports_websockets", toml_edit::value(false));
            if input.no_account_mode {
                doc["model_catalog_json"] = toml_edit::value(
                    root.join(catalog::CATALOG_FILE)
                        .to_str()
                        .ok_or("目录路径不是UTF8")?,
                );
            } else {
                doc.remove("model_catalog_json");
            }
            let current = doc.get("model").and_then(toml_edit::Item::as_str);
            let state = document.diagnostic_state().map_err(err)?;
            let slugs = catalog::expected_slugs(&state)
                .into_iter()
                .collect::<std::collections::HashSet<_>>();
            if current
                .is_some_and(|m| config::top_level_relay_model_requires_cleanup(m, true, &slugs))
            {
                doc.remove("model");
            }
        }
        Snapshot::Gateway {
            catalog: gateway,
            provider: approved_provider,
            ..
        } => {
            let existing = provider(text)?;
            if on {
                let expected = approved_provider
                    .as_deref()
                    .filter(|p| core::atelier_provider_owned(p))
                    .unwrap_or(&existing)
                    .to_owned();
                if !core::atelier_provider_owned(&expected) {
                    return Err(
                        "当前有效Provider不属于Atelier；路由修复未批准接管自定义/官方连接".into(),
                    );
                }
                if existing != expected {
                    doc.remove("profile");
                    doc["model_provider"] = toml_edit::value(&expected);
                }
                let p = provider_table(&mut doc, &expected)?;
                p.insert(
                    "base_url",
                    toml_edit::value(format!("http://127.0.0.1:{}/v1", gateway.settings.port)),
                );
                p.insert("wire_api", toml_edit::value("responses"));
                p.insert("env_key", toml_edit::value("ATELIER_GATEWAY_TOKEN"));
                p.insert("requires_openai_auth", toml_edit::value(false));
                p.insert("supports_websockets", toml_edit::value(false));
                // Preserve selected user profile; update its actual effective model if present.
                let selected = doc
                    .get("profile")
                    .and_then(toml_edit::Item::as_str)
                    .map(str::to_owned);
                if let Some(profile) = selected {
                    doc.get_mut("profiles")
                        .and_then(|p| p.get_mut(&profile))
                        .and_then(toml_edit::Item::as_table_like_mut)
                        .ok_or("当前profile不是既有配置表")?
                        .insert("model", toml_edit::value(&gateway.settings.model));
                } else {
                    doc["model"] = toml_edit::value(&gateway.settings.model);
                }
                doc["model_catalog_json"] = toml_edit::value(
                    root.join(catalog::CATALOG_FILE)
                        .to_str()
                        .ok_or("目录路径不是UTF8")?,
                );
            } else {
                if core::atelier_provider_owned(&existing) {
                    if let Some(selected) = doc
                        .get("profile")
                        .and_then(toml_edit::Item::as_str)
                        .map(str::to_owned)
                    {
                        if doc
                            .get("profiles")
                            .and_then(|p| p.get(&selected))
                            .and_then(|p| p.get("model_provider"))
                            .and_then(toml_edit::Item::as_str)
                            .is_some_and(core::atelier_provider_owned)
                        {
                            doc.remove("profile");
                        }
                    }
                    if doc
                        .get("model_provider")
                        .and_then(toml_edit::Item::as_str)
                        .is_some_and(core::atelier_provider_owned)
                    {
                        doc.remove("model_provider");
                    }
                }
                if let Some(path) = doc
                    .get("model_catalog_json")
                    .and_then(toml_edit::Item::as_str)
                {
                    let p = Path::new(path);
                    let p = if p.is_absolute() {
                        p.to_path_buf()
                    } else {
                        ctx.home.join(p)
                    };
                    if config::normalize_catalog_path(&p.to_string_lossy())
                        == config::normalize_catalog_path(
                            &root.join(catalog::CATALOG_FILE).to_string_lossy(),
                        )
                    {
                        doc.remove("model_catalog_json");
                    }
                }
            }
        }
    }
    Ok(doc.to_string())
}
/// Pure Gateway config composition over already-approved primary bytes. No file is
/// published. Native auth/takeover effects require their original complete plan.
pub(super) fn compose_config_after(
    ctx: &Context,
    snapshot: &Snapshot,
    target: TargetState,
    config_after_primary: Option<&[u8]>,
) -> Result<Option<Vec<u8>>> {
    if matches!(snapshot, Snapshot::Native { .. }) {
        if read(source_path(snapshot.source()))?.as_deref() != Some(snapshot.bytes()) {
            return Err("Native来源在组合预览期间变化".into());
        }
        if read(&ctx.config())?.as_deref() != config_after_primary {
            return Err("Native认证/接管与primary配置效果尚未完成安全组合；需保留原计划".into());
        }
        let plan = prepare_reconcile_for_target(
            ctx,
            snapshot.source(),
            target,
            &ctx.app_root.join("maintenance/router-compose-preview"),
        )?;
        return Ok(plan.config_after);
    }
    let text = std::str::from_utf8(config_after_primary.unwrap_or_default())
        .map_err(|_| "primary批准配置不是UTF8")?;
    let after = config_patch(
        ctx,
        snapshot,
        target,
        text,
        native_proxy(ctx, snapshot).as_ref(),
    )?;
    Ok(if config_after_primary.is_none() && after.is_empty() {
        None
    } else {
        Some(after.into_bytes())
    })
}
/// Reconstruct only an ordered prefix of the original approved effects. A file
/// matching a later stage while an earlier changed file is still before is rejected.
pub(super) fn resume_after_approved_effects(
    ctx: &Context,
    reviewed: &ReviewedPlan,
    backup_root: &Path,
) -> Result<ReviewedPlan> {
    validate_owner(ctx, reviewed)?;
    let mut current = BTreeMap::new();
    for (path, _) in &reviewed.dependencies {
        current.insert(path.clone(), read(path)?);
    }
    let actual = current
        .iter()
        .map(|(p, b)| (p.clone(), effect_stamp(b.as_deref())))
        .collect::<BTreeMap<_, _>>();
    let mut expected = reviewed
        .dependencies
        .iter()
        .map(|(p, b)| (p.clone(), effect_stamp(b.as_deref())))
        .collect::<BTreeMap<_, _>>();
    let mut effects = vec![];
    if let Some(catalog) = &reviewed.catalog {
        let payload = catalog::builder::encode_prepared_catalog(catalog.document()).map_err(err)?;
        effects.push((catalog.destination(), effect_stamp(Some(&payload))));
    }
    effects.push((ctx.config(), effect_stamp(reviewed.config_after.as_deref())));
    for p in &reviewed.auth {
        effects.extend(
            native_auth::approved_effects(p)
                .into_iter()
                .filter(|(path, _)| path != &ctx.config()),
        );
    }
    let mut accepted = (actual == expected).then_some(0);
    for (index, (path, after)) in effects.iter().enumerate() {
        if !expected.contains_key(path) {
            return Err("原计划effect缺少原始依赖，不能恢复".into());
        }
        expected.insert(path.clone(), after.clone());
        if actual == expected {
            accepted = Some(index + 1);
        }
    }
    let prefix_len = accepted.ok_or("当前文件不是任何有序获批效果前缀；未覆盖外部变化")?;
    let mut ledger = BTreeMap::new();
    for (path, after) in effects.iter().take(prefix_len) {
        ledger.insert(
            path.clone(),
            (
                effect_stamp(reviewed.dependencies.get(path).and_then(|b| b.as_deref())),
                after.clone(),
            ),
        );
    }
    let prefix = ledger
        .into_iter()
        .map(|(path, (before, after))| (path, before, after))
        .collect::<Vec<_>>();
    let config_before = current
        .get(&ctx.config())
        .cloned()
        .ok_or("恢复计划缺少原始配置依赖")?;
    let mut auth = vec![];
    for p in &reviewed.auth {
        auth.push(native_auth::replan_after_approved_effects(
            ctx,
            p,
            reviewed.mode,
            reviewed.config_before.as_deref(),
            config_before.as_deref(),
            &prefix,
        )?);
    }
    let mut text = if let Some(primary) = &reviewed.native_primary {
        validate_primary_metadata(&primary.metadata)?;
        let observed = PrimaryMetadata::default();
        let text = apply_primary_items(ctx, &primary.restored_input, &primary.ids, &observed)?;
        if observed.unstable.get() {
            return Err("Native primary元数据在恢复期间变化".into());
        }
        for (path, value) in observed.observed.into_inner() {
            if primary.metadata.get(&path) != Some(&value) {
                return Err("Native primary恢复引入未批准的元数据依赖".into());
            }
        }
        text
    } else {
        String::from_utf8(config_before.clone().unwrap_or_default())
            .map_err(|_| "当前获批配置不是UTF8")?
    };
    if reviewed.native_primary.is_none()
        && matches!(reviewed.snapshot, Snapshot::Native { .. })
        && reviewed.mode == RouterMode::Off
    {
        let native = atelier_maintenance_core::auth::takeover::plan_takeover_backup_orphan(
            &atelier_maintenance_core::auth::AuthContext::new(&ctx.home, RouterMode::Off),
        )
        .map_err(err)?;
        if let Some(c) = native.files.changes.iter().find(|c| c.path == ctx.config()) {
            if c.before != config_before {
                return Err("恢复配置在重新规划期间变化".into());
            }
            text =
                String::from_utf8(c.after.clone().ok_or("恢复候选试图删除配置")?).map_err(err)?;
        }
    }
    let after = config_patch(
        ctx,
        &reviewed.snapshot,
        reviewed.target,
        &text,
        reviewed.actual_proxy.as_ref(),
    )?;
    let config_after = if config_before.is_none() && after.is_empty() {
        None
    } else {
        Some(after.into_bytes())
    };
    if config_after != reviewed.config_after {
        return Err("重新规划后的最终配置不等于原批准效果".into());
    }
    let catalog = if let Some(original) = &reviewed.catalog {
        let fresh = native_catalog::prepare(reviewed.snapshot.source())?;
        if catalog::builder::encode_prepared_catalog(fresh.document()).map_err(err)?
            != catalog::builder::encode_prepared_catalog(original.document()).map_err(err)?
        {
            return Err("重新规划后的目录payload不等于原批准效果".into());
        }
        Some(fresh)
    } else {
        None
    };
    let next = ReviewedPlan {
        snapshot: reviewed.snapshot.clone(),
        target: reviewed.target,
        home: reviewed.home.clone(),
        app_root: reviewed.app_root.clone(),
        backup_root: backup_root.to_owned(),
        config_before,
        config_after,
        catalog,
        auth,
        dependencies: current,
        mode: reviewed.mode,
        actual_proxy: reviewed.actual_proxy.clone(),
        cli_signature: reviewed.cli_signature.clone(),
        native_primary: reviewed.native_primary.clone(),
        batch_seed_only: reviewed.batch_seed_only,
    };
    validate(ctx, &next)?;
    Ok(next)
}

/// Reuse the reviewed target/provider/source after an approved primary config effect.
/// All other dependencies retain their original stamps; the caller also compares
/// config_after with the concrete expected final bytes shown in its initial preview.
pub(super) fn rebase_after_approved_config(
    ctx: &Context,
    reviewed: &ReviewedPlan,
    approved_primary_after: Option<&[u8]>,
    backup_root: &Path,
) -> Result<ReviewedPlan> {
    if reviewed.home != ctx.home || reviewed.app_root != ctx.app_root {
        return Err("后继路由计划目标目录已变化".into());
    }
    if matches!(reviewed.snapshot, Snapshot::Native { .. }) {
        if reviewed.config_before.as_deref() != approved_primary_after {
            return Err("Native认证/接管与primary配置效果尚未完成安全组合；需保留原计划".into());
        }
        validate(ctx, reviewed)?;
        return Ok(reviewed.clone());
    }
    if read(&ctx.config())?.as_deref() != approved_primary_after {
        return Err("当前配置不等于获批primary效果，未重建后继计划".into());
    }
    for (path, before) in &reviewed.dependencies {
        if path == &ctx.config() {
            continue;
        }
        if &read(path)? != before {
            return Err(format!("后继路由计划未批准的依赖变化：{}", path.display()));
        }
    }
    let after = compose_config_after(
        ctx,
        &reviewed.snapshot,
        reviewed.target,
        approved_primary_after,
    )?;
    let mut next = reviewed.clone();
    next.config_before = approved_primary_after.map(|b| b.to_vec());
    next.config_after = after;
    next.backup_root = backup_root.to_owned();
    next.dependencies
        .insert(ctx.config(), next.config_before.clone());
    validate(ctx, &next)?;
    Ok(next)
}

pub(super) fn prepare_reconcile(
    ctx: &Context,
    source: &native_catalog::Source,
    backup_root: &Path,
) -> Result<ReviewedPlan> {
    let snap = snapshot(ctx, source)?;
    let proxy = native_proxy(ctx, &snap);
    prepare_reconcile_with_proxy(ctx, source, backup_root, proxy)
}
pub(super) fn prepare_reconcile_with_proxy(
    ctx: &Context,
    source: &native_catalog::Source,
    backup_root: &Path,
    actual_proxy: Option<ProxyHandle>,
) -> Result<ReviewedPlan> {
    let snap = snapshot(ctx, source)?;
    prepare_reconcile_for_target_with_proxy(ctx, source, snap.target(), backup_root, actual_proxy)
}
pub(super) fn prepare_reconcile_for_target(
    ctx: &Context,
    source: &native_catalog::Source,
    target: TargetState,
    backup_root: &Path,
) -> Result<ReviewedPlan> {
    let snap = snapshot(ctx, source)?;
    prepare_reconcile_for_target_with_proxy(
        ctx,
        source,
        target,
        backup_root,
        native_proxy(ctx, &snap),
    )
}
pub(super) fn prepare_reconcile_for_target_with_proxy(
    ctx: &Context,
    source: &native_catalog::Source,
    target: TargetState,
    backup_root: &Path,
    actual_proxy: Option<ProxyHandle>,
) -> Result<ReviewedPlan> {
    let snapshot = snapshot(ctx, source)?;
    if matches!(snapshot, Snapshot::Native { .. })
        && actual_proxy
            .as_ref()
            .zip(ctx.proxy_handle.as_ref())
            .is_some_and(|(a, b)| std::sync::Arc::ptr_eq(&a.alive, &b.alive))
    {
        return Err("Native预审不能借用Atelier Gateway proxy所有权".into());
    }
    let mode = mode(&snapshot, target)?;
    let config_before = read(&ctx.config())?;
    let mut text = String::from_utf8(config_before.clone().unwrap_or_default())
        .map_err(|_| "原始配置不是UTF8")?;
    // Gateway authentication belongs to Gateway's credentials. Native residual plans
    // are only applicable to the explicit Native document, never to Gateway auth.json.
    let auth = if matches!(snapshot, Snapshot::Native { .. }) && mode != RouterMode::NoAccount {
        let ids = if mode == RouterMode::Off {
            vec![
                "router_unlock_auth_residue".into(),
                "takeover_backup_orphan".into(),
            ]
        } else {
            vec!["router_unlock_auth_residue".into()]
        };
        native_auth::prepare_sequence(ctx, &ids, backup_root, mode)?
    } else {
        vec![]
    };
    for p in &auth {
        if let Some((_, before, after)) = native_auth::planned_config(p) {
            if before != config_before {
                return Err("认证配置计划不是同一原始版本".into());
            }
            text = String::from_utf8(after.ok_or("未批准删除整个配置")?).map_err(err)?;
        }
    }
    text = config_patch(ctx, &snapshot, target, &text, actual_proxy.as_ref())?;
    let config_after = if config_before.is_none() && text.is_empty() {
        None
    } else {
        Some(text.into_bytes())
    };
    let catalog = if target == TargetState::RouterOn {
        Some(native_catalog::prepare(source)?)
    } else {
        None
    };
    let mut dependencies = BTreeMap::new();
    dependencies.insert(
        source_path(source).to_owned(),
        Some(snapshot.bytes().to_vec()),
    );
    dependencies.insert(ctx.config(), config_before.clone());
    if let Some(c) = &catalog {
        dependencies.insert(c.destination(), read(&c.destination())?);
    }
    if let Snapshot::Gateway { intent_before, .. } = &snapshot {
        dependencies.insert(super::router_intent::path(ctx), intent_before.clone());
    }
    for p in &auth {
        for (path, stamp) in native_auth::approved_dependencies(p) {
            let current = read(&path)?;
            if stamp != effect_stamp(current.as_deref()) {
                return Err("认证依赖在计划期间变化".into());
            }
            if dependencies
                .get(&path)
                .is_some_and(|prior| prior != &current)
            {
                return Err("路由计划初始依赖不一致".into());
            }
            dependencies.insert(path, current);
        }
    }
    let cli_signature = if catalog.is_some() {
        Some(catalog_cli_signature(source))
    } else {
        None
    };
    let plan = ReviewedPlan {
        snapshot,
        target,
        home: ctx.home.clone(),
        app_root: ctx.app_root.clone(),
        backup_root: backup_root.to_owned(),
        config_before,
        config_after,
        catalog,
        auth,
        dependencies,
        mode,
        actual_proxy,
        cli_signature,
        native_primary: None,
        batch_seed_only: false,
    };
    validate(ctx, &plan)?;
    Ok(plan)
}
pub(super) fn preview_lines(plan: &ReviewedPlan) -> Vec<String> {
    let mut lines = vec![
        format!(
            "路由来源：{}",
            if matches!(plan.snapshot, Snapshot::Native { .. }) {
                "Native RelayState"
            } else {
                "Atelier Responses Gateway"
            }
        ),
        format!(
            "配置：{}（{} → {}字节）",
            plan.home.join("config.toml").display(),
            plan.config_before.as_ref().map_or(0, Vec::len),
            plan.config_after.as_ref().map_or(0, Vec::len)
        ),
    ];
    if let Some(c) = &plan.catalog {
        lines.push(format!("实际模型目录：{}", c.destination().display()));
    }
    for p in &plan.auth {
        lines.push(format!("原生认证残留阶段：{}", native_auth::item_id(p)));
    }
    lines
}
fn catalog_cli_signature(source: &native_catalog::Source) -> publication::CliSignature {
    let bundle = match source {
        native_catalog::Source::Native { bundle, .. }
        | native_catalog::Source::Gateway { bundle, .. } => bundle.as_deref(),
    };
    publication::current_bundled_cli_signature(bundle)
}
fn validate_owner(ctx: &Context, plan: &ReviewedPlan) -> Result<()> {
    if ctx.home != plan.home || ctx.app_root != plan.app_root {
        return Err("路由维护上下文已变化".into());
    }
    if matches!(plan.snapshot, Snapshot::Gateway { .. }) {
        match (&plan.actual_proxy, &ctx.proxy_handle) {
            (Some(a), Some(b))
                if a.port == b.port && std::sync::Arc::ptr_eq(&a.alive, &b.alive) =>
            {
                ()
            }
            (None, None) => (),
            _ => return Err("已审核proxy所有权已变化，请重新预览".into()),
        }
    }
    if plan
        .actual_proxy
        .as_ref()
        .is_some_and(|h| !h.alive.load(std::sync::atomic::Ordering::Acquire))
    {
        return Err("已审核proxy已停止，未写入旧端口配置".into());
    }
    if let Some(primary) = &plan.native_primary {
        validate_primary_metadata(&primary.metadata)?;
    }
    if plan
        .cli_signature
        .as_ref()
        .is_some_and(|s| *s != catalog_cli_signature(plan.snapshot.source()))
    {
        return Err("已审核CLI签名已变化".into());
    }
    Ok(())
}
pub(super) fn validate(ctx: &Context, plan: &ReviewedPlan) -> Result<()> {
    validate_owner(ctx, plan)?;
    for (path, before) in &plan.dependencies {
        if &read(path)? != before {
            return Err(format!("已审核路由依赖变化：{}", path.display()));
        }
    }
    for p in &plan.auth {
        native_auth::validate(ctx, p, plan.mode)?;
    }
    if let Some(p) = &plan.catalog {
        native_catalog::validate(p)?;
    }
    Ok(())
}
fn effect_stamp(bytes: Option<&[u8]>) -> Option<(usize, String)> {
    bytes.map(|b| (b.len(), format!("{:x}", Sha256::digest(b))))
}
#[derive(Clone, Debug)]
pub(super) struct Outcome {
    pub completed: Vec<String>,
    pub failure: Option<String>,
    pub backups: Vec<PathBuf>,
    pub verification: VerificationReport,
    pub warnings: Vec<String>,
}
pub(super) fn apply(ctx: &Context, plan: &ReviewedPlan) -> Result<Outcome> {
    let guard = super::repair::acquire(ctx)?;
    apply_under_manager(ctx, plan, &guard)
}
pub(super) fn apply_under_manager(
    ctx: &Context,
    plan: &ReviewedPlan,
    guard: &super::repair::MutationGuard,
) -> Result<Outcome> {
    apply_under_manager_using(ctx, plan, guard, &mut |path, bytes| {
        publication::write_atomic_private_report(path, bytes)
    })
}
fn apply_under_manager_using(
    ctx: &Context,
    plan: &ReviewedPlan,
    guard: &super::repair::MutationGuard,
    publish: &mut dyn FnMut(&Path, &[u8]) -> io::Result<publication::Publication>,
) -> Result<Outcome> {
    guard.validate(ctx)?;
    if plan.batch_seed_only {
        return Err("batch种子计划只能由获批prefix重基后执行".into());
    }
    validate(ctx, plan)?;
    private_dir(&plan.backup_root)?;
    let mut outcome = Outcome {
        completed: vec![],
        failure: None,
        backups: vec![],
        verification: VerificationReport::default(),
        warnings: vec![],
    };
    let execute = (|| -> Result<()> {
        let backup = plan.backup_root.join("router-config-original.json");
        let manifest = serde_json::json!({"path":ctx.config(),"beforeBase64":plan.config_before.as_ref().map(|b|{use base64::Engine;base64::engine::general_purpose::STANDARD.encode(b)})});
        let bytes = serde_json::to_vec(&manifest).map_err(err)?;
        atomic_private(&backup, &bytes)?;
        if fs::read(&backup).map_err(err)? != bytes {
            return Err("原始配置备份回读不一致".into());
        }
        outcome.backups.push(backup);
        guard.validate(ctx)?;
        validate(ctx, plan)?;
        if let Some(p) = &plan.catalog {
            let applied = native_catalog::apply(p.clone())?;
            outcome.backups.push(applied.backup_path);
            outcome.completed.push("catalog".into());
            if let Some(kind) = applied.directory_sync_error {
                outcome
                    .warnings
                    .push(format!("模型目录已发布并回读，目录同步尚未确认：{kind:?}"));
            }
        }
        for (path, before) in &plan.dependencies {
            if plan
                .catalog
                .as_ref()
                .is_some_and(|c| c.destination() == *path)
            {
                continue;
            }
            if &read(path)? != before {
                return Err(format!(
                    "目录阶段后已审核依赖变化，已保留完成结果：{}",
                    path.display()
                ));
            }
        }
        guard.validate(ctx)?;
        // Publish the reviewed composition once; no auth stage rewrites this config.
        if read(&ctx.config())? != plan.config_before {
            return Err("目录阶段后配置被外部修改，已保留完成结果".into());
        }
        if plan.config_before != plan.config_after {
            match &plan.config_after {
                Some(b) => {
                    let published = publish(&ctx.config(), b).map_err(err)?;
                    if let Err(e) = published.require_sync() {
                        outcome
                            .completed
                            .push("config_published_sync_unconfirmed".into());
                        return Err(err(e));
                    }
                }
                None => {
                    if ctx.config().exists() {
                        fs::remove_file(ctx.config()).map_err(err)?;
                    }
                }
            }
        }
        if read(&ctx.config())? != plan.config_after {
            return Err("路由配置发布回读不一致".into());
        }
        if plan.config_before == plan.config_after {
            // Resumed publication may have failed only its previous directory sync.
            // Confirm the actual retained bytes durably before consuming auth backups.
            if plan.config_after.is_some() {
                fs::File::open(ctx.config())
                    .and_then(|f| f.sync_all())
                    .map_err(err)?;
            }
            fs::File::open(&ctx.home)
                .and_then(|f| f.sync_all())
                .map_err(err)?;
            if read(&ctx.config())? != plan.config_after {
                return Err("恢复配置同步期间字节变化".into());
            }
        }
        outcome.completed.push("config".into());
        let mut prefix = vec![];
        for p in &plan.auth {
            let (result, effects) = native_auth::apply_after_approved_effects(
                ctx,
                p,
                plan.mode,
                plan.config_before.as_deref(),
                plan.config_after.as_deref(),
                &prefix,
            )?;
            outcome.backups.extend(result.backups);
            if !result.success {
                return Err(result.detail);
            }
            for (path, after) in effects {
                let before = plan
                    .dependencies
                    .get(&path)
                    .and_then(|b| effect_stamp(b.as_deref()));
                prefix.push((path, before, after));
            }
            outcome.completed.push(native_auth::item_id(p).into());
        }
        Ok(())
    })();
    if let Err(e) = execute {
        outcome.failure = Some(e);
    }
    outcome.verification = verify_target_with_proxy(
        ctx,
        &plan.snapshot,
        plan.target,
        Depth::Full,
        plan.actual_proxy.clone(),
    )
    .unwrap_or_else(|e| VerificationReport {
        issues: vec![VerificationIssue {
            dim: InvariantDim::Config,
            message: e,
        }],
    });
    if plan.target == TargetState::RouterOn {
        availability(
            &plan.snapshot,
            &mut VerificationReport::default(),
            &mut outcome.warnings,
        );
    }
    Ok(outcome)
}
#[cfg(test)]
#[path = "native_router_tests.rs"]
mod tests;

/// Exactly the two recovered Native config cleanup algorithms, reviewed before the
/// primary mutation. Config bytes are private; this token is not Debug/Serialize.
#[derive(Clone)]
pub(super) struct NativePrimaryComposition {
    ids: Vec<String>,
    primary_after: Option<Vec<u8>>,
    final_after: Option<Vec<u8>>,
    restored_input: String,
    dependencies: Vec<(PathBuf, Option<(usize, String)>)>,
    auth_effects: Vec<(String, Vec<(PathBuf, native_auth::EffectStamp)>)>,
    plain_after: native_auth::EffectStamp,
    target: TargetState,
    metadata: BTreeMap<PathBuf, std::result::Result<(), io::ErrorKind>>,
}
impl NativePrimaryComposition {
    pub fn config_after(&self) -> Option<&[u8]> {
        self.final_after.as_deref()
    }
    pub fn primary_after(&self) -> Option<&[u8]> {
        self.primary_after.as_deref()
    }
    pub fn item_ids(&self) -> &[String] {
        &self.ids
    }
    pub fn metadata_dependencies(&self) -> Vec<(PathBuf, std::result::Result<(), io::ErrorKind>)> {
        self.metadata.iter().map(|(p, s)| (p.clone(), *s)).collect()
    }
}
#[derive(Default)]
struct PrimaryMetadata {
    observed: std::cell::RefCell<BTreeMap<PathBuf, std::result::Result<(), io::ErrorKind>>>,
    unstable: std::cell::Cell<bool>,
}
impl config::ConfigFs for PrimaryMetadata {
    fn metadata(&self, path: &Path) -> io::Result<()> {
        let actual = fs::metadata(path).map(|_| ()).map_err(|e| e.kind());
        if self
            .observed
            .borrow()
            .get(path)
            .is_some_and(|prior| prior != &actual)
        {
            self.unstable.set(true);
        }
        self.observed.borrow_mut().insert(path.to_owned(), actual);
        actual.map_err(io::Error::from)
    }
}
fn validate_primary_metadata(
    metadata: &BTreeMap<PathBuf, std::result::Result<(), io::ErrorKind>>,
) -> Result<()> {
    for (path, expected) in metadata {
        if fs::metadata(path).map(|_| ()).map_err(|e| e.kind()) != *expected {
            return Err(format!(
                "Native primary已审核元数据变化：{}",
                path.display()
            ));
        }
    }
    Ok(())
}
fn primary_ids(ids: &[String]) -> Result<Vec<String>> {
    const ORDER: [&str; 2] = ["config_global_validity", "config_profile_conflict"];
    if ids.iter().any(|id| !ORDER.contains(&id.as_str())) {
        return Err(
            "Native primary组合只支持已批准global/profile；认证或其它primary效果需独立确切ledger"
                .into(),
        );
    }
    Ok(ORDER
        .into_iter()
        .filter(|id| ids.iter().any(|i| i == id))
        .map(str::to_owned)
        .collect())
}
fn apply_primary_items(
    ctx: &Context,
    text: &str,
    ids: &[String],
    observed: &PrimaryMetadata,
) -> Result<String> {
    let mut after = text.to_owned();
    if ids.iter().any(|id| id == "config_global_validity") {
        after = config::fix_config_global_validity_at(
            &after,
            &ctx.config(),
            &ctx.home.join("codexmate/relay"),
            observed,
        )
        .map_err(err)?
        .after;
    }
    if ids.iter().any(|id| id == "config_profile_conflict") {
        after = config::fix_config_profile_conflict_text(&after, &ctx.home).after;
    }
    Ok(after)
}
fn optional_config(before: Option<&[u8]>, text: String) -> Option<Vec<u8>> {
    if before.is_none() && text.is_empty() {
        None
    } else {
        Some(text.into_bytes())
    }
}
fn build_native_primary(
    ctx: &Context,
    reviewed: &ReviewedPlan,
    ids: &[String],
    approved_primary_after: Option<&[u8]>,
) -> Result<NativePrimaryComposition> {
    if !matches!(reviewed.snapshot, Snapshot::Native { .. }) {
        return Err("该组合token仅用于真实Native来源".into());
    }
    let ids = primary_ids(ids)?;
    let original = std::str::from_utf8(reviewed.config_before.as_deref().unwrap_or_default())
        .map_err(|_| "Native原始配置不是UTF8")?;
    let observed = PrimaryMetadata::default();
    let primary = optional_config(
        reviewed.config_before.as_deref(),
        apply_primary_items(ctx, original, &ids, &observed)?,
    );
    if primary.as_deref() != approved_primary_after {
        return Err("primary配置字节不是原始输入执行获批global/profile的确切效果".into());
    }
    let mut restored = original.to_owned();
    for action in &reviewed.auth {
        if let Some((path, before, after)) = native_auth::planned_config(action) {
            if path != ctx.config() || before != reviewed.config_before {
                return Err("Native接管候选不属于同一原始配置".into());
            }
            restored =
                String::from_utf8(after.ok_or("Native接管候选试图删除配置")?).map_err(err)?;
        }
    }
    let cleaned = apply_primary_items(ctx, &restored, &ids, &observed)?;
    let final_text = config_patch(
        ctx,
        &reviewed.snapshot,
        reviewed.target,
        &cleaned,
        reviewed.actual_proxy.as_ref(),
    )?;
    if observed.unstable.get() {
        return Err("Native primary元数据在组合预览期间变化".into());
    }
    let metadata = observed.observed.into_inner();
    validate_primary_metadata(&metadata)?;
    Ok(NativePrimaryComposition {
        ids,
        primary_after: primary,
        final_after: optional_config(reviewed.config_before.as_deref(), final_text),
        restored_input: restored,
        dependencies: reviewed.approved_dependencies(),
        auth_effects: reviewed
            .auth
            .iter()
            .map(|p| {
                (
                    native_auth::item_id(p).to_owned(),
                    native_auth::approved_effects(p),
                )
            })
            .collect(),
        plain_after: effect_stamp(reviewed.config_after()),
        target: reviewed.target,
        metadata,
    })
}
/// Prove primary bytes with the exact recovered algorithms, restore the approved
/// takeover candidate, reapply those cleanups and render the actual Native route.
/// This method performs no publication or virtual-filesystem substitution.
pub(super) fn prepare_native_primary_composition(
    ctx: &Context,
    reviewed: &ReviewedPlan,
    approved_item_ids: &[String],
    approved_primary_after: Option<&[u8]>,
) -> Result<NativePrimaryComposition> {
    validate(ctx, reviewed)?;
    let token = build_native_primary(ctx, reviewed, approved_item_ids, approved_primary_after)?;
    validate(ctx, reviewed)?;
    validate_primary_metadata(&token.metadata)?;
    Ok(token)
}
/// After exact primary publication, retain all original nonconfig sources and pinned
/// auth effects. Consume fresh auth config candidates to preserve the reviewed final.
pub(super) fn rebase_native_primary(
    ctx: &Context,
    reviewed: &ReviewedPlan,
    token: &NativePrimaryComposition,
    backup_root: &Path,
) -> Result<ReviewedPlan> {
    validate_owner(ctx, reviewed)?;
    let effects = reviewed
        .auth
        .iter()
        .map(|p| {
            (
                native_auth::item_id(p).to_owned(),
                native_auth::approved_effects(p),
            )
        })
        .collect::<Vec<_>>();
    if reviewed.target != token.target
        || reviewed.approved_dependencies() != token.dependencies
        || effect_stamp(reviewed.config_after()) != token.plain_after
        || effects != token.auth_effects
    {
        return Err("Native primary token不属于原审核计划".into());
    }
    if read(&ctx.config())?.as_deref() != token.primary_after() {
        return Err("当前配置不等于批准primary效果".into());
    }
    for (path, before) in &reviewed.dependencies {
        if path != &ctx.config() && &read(path)? != before {
            return Err(format!(
                "Native primary之外的原始依赖变化：{}",
                path.display()
            ));
        }
    }
    validate_primary_metadata(&token.metadata)?;
    let rebuilt = build_native_primary(ctx, reviewed, &token.ids, token.primary_after())?;
    if rebuilt.final_after != token.final_after || rebuilt.metadata != token.metadata {
        return Err("Native后继候选不等于最初获批final/元数据范围".into());
    }
    let mut auth = vec![];
    for action in &reviewed.auth {
        auth.push(native_auth::replan_after_approved_effects(
            ctx,
            action,
            reviewed.mode,
            reviewed.config_before.as_deref(),
            token.primary_after(),
            &[],
        )?);
    }
    let mut next = reviewed.clone();
    next.config_before = token.primary_after.clone();
    next.config_after = token.final_after.clone();
    next.dependencies
        .insert(ctx.config(), next.config_before.clone());
    next.auth = auth;
    next.backup_root = backup_root.to_owned();
    next.native_primary = Some(token.clone());
    validate(ctx, &next)?;
    Ok(next)
}

/// Approval for the finite, concrete Native full-batch config candidates. Neither
/// file bytes nor an effect ledger can be supplied by the caller at execution.
/// The batch owns the original Integrity -> Unlock -> Takeover plans and alone can
/// produce the opaque readback proof consumed here.
#[derive(Clone)]
pub(super) struct BoundBatchResyncToken {
    batch: super::native_coordinator::PreparedBatch,
    original: Option<ReviewedPlan>,
    observed_off: Option<ObservedOff>,
    target: TargetState,
    mode: RouterMode,
    router_dependencies: Vec<(PathBuf, native_auth::EffectStamp)>,
    router_after: native_auth::EffectStamp,
    router_auth: Vec<(String, Vec<(PathBuf, native_auth::EffectStamp)>)>,
    initial: BTreeMap<PathBuf, Option<Vec<u8>>>,
    candidates: BTreeMap<usize, Option<Vec<u8>>>,
    final_candidates: BTreeMap<usize, Result<Option<Vec<u8>>>>,
    catalog_payload: Option<Vec<u8>>,
    initial_modified: BTreeMap<PathBuf, Option<std::time::SystemTime>>,
}
impl BoundBatchResyncToken {
    pub fn validate_initial(&self, ctx: &Context) -> Result<()> {
        if let Some(original) = &self.original {
            validate(ctx, original)?;
        }
        if let Some(off) = &self.observed_off {
            off.validate(ctx)?;
        }
        self.batch.validate_initial(ctx)?;
        for (path, before) in &self.initial {
            if read(path)? != *before
                || bound_modified(path)? != self.initial_modified.get(path).copied().flatten()
            {
                return Err("Router batch token初始union已变化".into());
            }
        }
        Ok(())
    }
    pub fn catalog_effect(&self) -> Option<(PathBuf, Option<Vec<u8>>)> {
        self.original
            .as_ref()
            .and_then(|original| original.catalog.as_ref())
            .map(|p| (p.destination(), self.catalog_payload.clone()))
    }
    /// Read-only exact final alternatives for the review UI. An invalid TOML branch
    /// retains its deterministic failure rather than acquiring an arbitrary fallback.
    pub fn config_candidates(&self) -> Vec<(usize, Result<Option<Vec<u8>>>)> {
        self.final_candidates
            .iter()
            .map(|(key, value)| (*key, value.clone()))
            .collect()
    }
    pub fn approved_dependencies(&self) -> Vec<(PathBuf, native_auth::EffectStamp)> {
        self.initial
            .iter()
            .map(|(path, bytes)| (path.clone(), effect_stamp(bytes.as_deref())))
            .collect()
    }
}
fn router_auth_signature(
    plan: &ReviewedPlan,
) -> Vec<(String, Vec<(PathBuf, native_auth::EffectStamp)>)> {
    plan.auth
        .iter()
        .map(|action| {
            (
                native_auth::item_id(action).to_owned(),
                native_auth::approved_effects(action),
            )
        })
        .collect()
}
fn bound_catalog_payload(plan: &ReviewedPlan) -> Result<Option<Vec<u8>>> {
    plan.catalog
        .as_ref()
        .map(|p| catalog::builder::encode_prepared_catalog(p.document()).map_err(err))
        .transpose()
}
/// Pure preview over original plans; initial source/dependencies are read and
/// validated twice. No temporary mutation or re-preview of changed current bytes.
pub(super) fn prepare_bound_batch_resync(
    ctx: &Context,
    reviewed: &ReviewedPlan,
    batch: &super::native_coordinator::PreparedBatch,
) -> Result<BoundBatchResyncToken> {
    validate(ctx, reviewed)?;
    batch.validate_initial(ctx)?;
    if reviewed.mode != batch.mode() || reviewed.config_before() != batch.config_before() {
        return Err("batch与Router计划不是同一原始Native配置/模式".into());
    }
    let mut initial = reviewed.dependencies.clone();
    for (path, expected) in batch.original_dependencies() {
        let bytes = read(&path)?;
        if effect_stamp(bytes.as_deref()) != expected {
            return Err("batch初始依赖在Router token预审期间变化".into());
        }
        if initial.get(&path).is_some_and(|prior| prior != &bytes) {
            return Err("batch与Router原始依赖不一致".into());
        }
        initial.insert(path, bytes);
    }
    // These are actual opaque batch plans, not reconstructed desired booleans.
    let mut order = 0;
    for action in batch.auth_actions() {
        let next = match native_auth::item_id(action) {
            "auth_integrity" => 1,
            "router_unlock_auth_residue" => 2,
            "takeover_backup_orphan" => 3,
            _ => return Err("batch含未审核的认证阶段".into()),
        };
        if next <= order {
            return Err("batch认证顺序不是Integrity/Unlock/Takeover".into());
        }
        order = next;
        native_auth::validate(ctx, action, reviewed.mode)?;
        for (path, expected) in native_auth::approved_dependencies(action) {
            if initial.get(&path).map(|b| effect_stamp(b.as_deref())) != Some(expected) {
                return Err("batch认证计划有未纳入原始union的依赖".into());
            }
        }
    }
    let mut candidates = BTreeMap::new();
    let mut final_candidates = BTreeMap::new();
    for (key, bytes) in batch.config_candidates() {
        if candidates.insert(key, bytes.clone()).is_some() {
            return Err("batch候选key重复".into());
        }
        let rendered = (|| {
            let text = std::str::from_utf8(bytes.as_deref().unwrap_or_default())
                .map_err(|_| "batch候选配置不是UTF8")?;
            let text = config_patch(
                ctx,
                &reviewed.snapshot,
                reviewed.target,
                text,
                reviewed.actual_proxy.as_ref(),
            )?;
            Ok(optional_config(bytes.as_deref(), text))
        })();
        final_candidates.insert(key, rendered);
    }
    if candidates.is_empty() {
        return Err("batch没有预审配置候选".into());
    }
    let initial_modified = initial
        .keys()
        .map(|p| Ok((p.clone(), bound_modified(p)?)))
        .collect::<Result<_>>()?;
    let token = BoundBatchResyncToken {
        batch: batch.clone(),
        original: Some(reviewed.clone()),
        observed_off: None,
        target: reviewed.target,
        mode: reviewed.mode,
        router_dependencies: reviewed.approved_dependencies(),
        router_after: effect_stamp(reviewed.config_after()),
        router_auth: router_auth_signature(reviewed),
        initial,
        candidates,
        final_candidates,
        catalog_payload: bound_catalog_payload(reviewed)?,
        initial_modified,
    };
    validate(ctx, reviewed)?;
    batch.validate_initial(ctx)?;
    for (path, expected) in &token.initial {
        if read(path)? != *expected {
            return Err("Router batch union在预审结束前变化".into());
        }
    }
    Ok(token)
}
/// Bind real, read-back batch effects to one of the previously approved alternatives.
/// Source/target/proxy stay those of the original review. Auth fresh plans can only
/// shrink within their original scope; catalog bytes must remain exactly reviewed.
pub(super) fn rebase_bound_batch_resync(
    ctx: &Context,
    reviewed: &ReviewedPlan,
    token: &BoundBatchResyncToken,
    prefix: &super::native_coordinator::ApprovedBatchPrefix,
    backup_root: &Path,
) -> Result<ReviewedPlan> {
    validate_owner(ctx, reviewed)?;
    if token.original.is_none() {
        return Err("ObservedOff token不能套用Native ReviewedPlan".into());
    }
    if !token.batch.same_plan(prefix.plan())
        || reviewed.target != token.target
        || reviewed.mode != token.mode
        || reviewed.approved_dependencies() != token.router_dependencies
        || effect_stamp(reviewed.config_after()) != token.router_after
        || router_auth_signature(reviewed) != token.router_auth
        || bound_catalog_payload(reviewed)? != token.catalog_payload
    {
        return Err("batch效果凭据/token不属于原Router审核计划".into());
    }
    prefix.validate_current(ctx)?;
    let staged = token
        .candidates
        .get(&prefix.candidate_key())
        .ok_or("batch实际候选key未预审")?;
    if staged.as_deref() != prefix.config_after() || read(&ctx.config())? != *staged {
        return Err("batch实际配置不等于预审中间候选".into());
    }
    let final_after = token
        .final_candidates
        .get(&prefix.candidate_key())
        .ok_or("batch实际分支没有预审Router效果")?
        .clone()?;
    let mut current = BTreeMap::new();
    let mut auth_prefix = vec![];
    for (path, before) in &token.initial {
        let bytes = read(path)?;
        let original = effect_stamp(before.as_deref());
        let expected = if path == &ctx.config() {
            effect_stamp(staged.as_deref())
        } else if let Some((initial, after)) = prefix.effect(path) {
            if initial != original {
                return Err("opaque batch效果的before不属于原始union".into());
            }
            if initial != after {
                auth_prefix.push((path.clone(), initial, after.clone()));
            }
            after
        } else {
            original
        };
        if effect_stamp(bytes.as_deref()) != expected {
            return Err(format!(
                "batch未批准的Router来源/依赖变化：{}",
                path.display()
            ));
        }
        current.insert(path.clone(), bytes);
    }
    // A batch auth effect cannot authorize rewriting the actual Native state source.
    let source = source_path(reviewed.snapshot.source());
    if current.get(source) != token.initial.get(source) {
        return Err("batch效果不能修改原审核Router来源".into());
    }
    let mut auth = vec![];
    for action in token.batch.auth_actions() {
        auth.push(native_auth::replan_after_approved_effects(
            ctx,
            action,
            reviewed.mode,
            token.batch.config_before(),
            staged.as_deref(),
            &auth_prefix,
        )?);
    }
    let catalog = if let Some(original) = &reviewed.catalog {
        let fresh = native_catalog::prepare(reviewed.snapshot.source())?;
        let bytes = catalog::builder::encode_prepared_catalog(fresh.document()).map_err(err)?;
        if Some(bytes) != token.catalog_payload || fresh.destination() != original.destination() {
            return Err("batch后的目录payload/目标不等于原批准效果".into());
        }
        Some(fresh)
    } else {
        None
    };
    // Never render a new candidate from post-mutation current data here.
    let mut next = reviewed.clone();
    next.config_before = staged.clone();
    next.config_after = final_after;
    next.dependencies = current;
    next.auth = auth;
    next.catalog = catalog;
    next.backup_root = backup_root.to_owned();
    next.native_primary = None;
    next.batch_seed_only = false;
    prefix.validate_current(ctx)?;
    validate(ctx, &next)?;
    Ok(next)
}

fn bound_modified(path: &Path) -> Result<Option<std::time::SystemTime>> {
    match fs::symlink_metadata(path) {
        Ok(m) => Ok(m.modified().ok()),
        Err(e) if e.kind() == io::ErrorKind::NotFound => Ok(None),
        Err(e) => Err(err(e)),
    }
}
/// An invalid original TOML may have an approved Syntax candidate. Build a source
/// seed without rendering or modifying that original. The seed cannot be applied
/// directly; only a proved candidate can produce an executable ReviewedPlan.
pub(super) fn prepare_bound_batch_resync_from_source(
    ctx: &Context,
    source: &native_catalog::Source,
    target: TargetState,
    backup_root: &Path,
    actual_proxy: Option<ProxyHandle>,
    batch: &super::native_coordinator::PreparedBatch,
) -> Result<(ReviewedPlan, BoundBatchResyncToken)> {
    let snapshot = snapshot(ctx, source)?;
    // Context's handle belongs to Atelier Gateway, never to a Native host.
    // An explicit independently-owned Native host handle remains supported.
    if matches!(snapshot, Snapshot::Native { .. })
        && actual_proxy
            .as_ref()
            .zip(ctx.proxy_handle.as_ref())
            .is_some_and(|(a, b)| std::sync::Arc::ptr_eq(&a.alive, &b.alive))
    {
        return Err("Native预审不能借用Atelier Gateway proxy所有权".into());
    }
    let mode = mode(&snapshot, target)?;
    if mode != batch.mode() {
        return Err("batch种子与实际目标模式不一致".into());
    }
    if target == TargetState::RouterOn && actual_proxy.is_none() {
        return Err("启用态批次重同步需要实际proxy所有权".into());
    }
    batch.validate_initial(ctx)?;
    let before = read(&ctx.config())?;
    if before.as_deref() != batch.config_before() {
        return Err("batch种子不是原始配置版本".into());
    }
    let catalog = if target == TargetState::RouterOn {
        Some(native_catalog::prepare(source)?)
    } else {
        None
    };
    let mut dependencies = BTreeMap::new();
    dependencies.insert(
        source_path(source).to_owned(),
        Some(snapshot.bytes().to_vec()),
    );
    dependencies.insert(ctx.config(), before.clone());
    if let Snapshot::Gateway { intent_before, .. } = &snapshot {
        dependencies.insert(super::router_intent::path(ctx), intent_before.clone());
    }
    if let Some(catalog) = &catalog {
        dependencies.insert(catalog.destination(), read(&catalog.destination())?);
    }
    for (path, expected) in batch.original_dependencies() {
        let bytes = read(&path)?;
        if effect_stamp(bytes.as_deref()) != expected {
            return Err("batch种子原始依赖变化".into());
        }
        if dependencies.get(&path).is_some_and(|prior| prior != &bytes) {
            return Err("batch种子依赖冲突".into());
        }
        dependencies.insert(path, bytes);
    }
    let cli_signature = catalog.as_ref().map(|_| catalog_cli_signature(source));
    let reviewed = ReviewedPlan {
        snapshot,
        target,
        home: ctx.home.clone(),
        app_root: ctx.app_root.clone(),
        backup_root: backup_root.to_owned(),
        // This is a non-executable marker, never an approved postcondition.
        config_before: before.clone(),
        config_after: before,
        catalog,
        auth: batch.auth_actions().to_vec(),
        dependencies,
        mode,
        actual_proxy,
        cli_signature,
        native_primary: None,
        batch_seed_only: true,
    };
    let token = prepare_bound_batch_resync(ctx, &reviewed, batch)?;
    Ok((reviewed, token))
}

/// Only this module creates the exact resync receipt. A failed/uncertain apply
/// preserves its Outcome but yields a poisoned receipt, unusable by later stages.
#[derive(Clone)]
pub(super) struct ApprovedResyncEffects {
    batch: super::native_coordinator::PreparedBatch,
    home: PathBuf,
    app_root: PathBuf,
    expected: BTreeMap<PathBuf, native_auth::EffectStamp>,
    effects: BTreeMap<
        PathBuf,
        (
            Option<super::native_orphans::EffectStamp>,
            Option<super::native_orphans::EffectStamp>,
        ),
    >,
    poisoned: bool,
}
impl ApprovedResyncEffects {
    pub fn plan(&self) -> &super::native_coordinator::PreparedBatch {
        &self.batch
    }
    pub fn paths(&self) -> Vec<PathBuf> {
        self.effects.keys().cloned().collect()
    }
    pub fn effect_stamp(
        &self,
        path: &Path,
    ) -> Option<(
        Option<super::native_orphans::EffectStamp>,
        Option<super::native_orphans::EffectStamp>,
    )> {
        self.effects.get(path).cloned()
    }
    pub fn validate_current(&self, ctx: &Context) -> Result<()> {
        if self.poisoned {
            return Err("Router重同步部分效果未完成确认，禁止后续批次写入".into());
        }
        if ctx.home != self.home
            || ctx.app_root != self.app_root
            || native_auth::resolve_mode(ctx)? != self.batch.mode()
        {
            return Err("Router重同步凭据上下文/模式变化".into());
        }
        for (path, expected) in &self.expected {
            if effect_stamp(read(path)?.as_deref()) != *expected {
                return Err("Router重同步已批准效果被外部修改".into());
            }
            if let Some((_, after)) = self.effects.get(path) {
                if after.as_ref().and_then(|s| s.2) != bound_modified(path)? {
                    return Err("Router重同步效果时间戳变化".into());
                }
            }
        }
        Ok(())
    }
}
pub(super) fn apply_bound_batch_resync(
    ctx: &Context,
    reviewed: &ReviewedPlan,
    token: &BoundBatchResyncToken,
    prefix: &super::native_coordinator::ApprovedBatchPrefix,
    backup_root: &Path,
    guard: &super::repair::MutationGuard,
) -> Result<(Outcome, ApprovedResyncEffects)> {
    apply_bound_batch_resync_using(
        ctx,
        reviewed,
        token,
        prefix,
        backup_root,
        guard,
        &mut |path, bytes| publication::write_atomic_private_report(path, bytes),
    )
}
fn apply_bound_batch_resync_using(
    ctx: &Context,
    reviewed: &ReviewedPlan,
    token: &BoundBatchResyncToken,
    prefix: &super::native_coordinator::ApprovedBatchPrefix,
    backup_root: &Path,
    guard: &super::repair::MutationGuard,
    publish: &mut dyn FnMut(&Path, &[u8]) -> io::Result<publication::Publication>,
) -> Result<(Outcome, ApprovedResyncEffects)> {
    let next = rebase_bound_batch_resync(ctx, reviewed, token, prefix, backup_root)?;
    let mut expected = token
        .initial
        .iter()
        .map(|(p, b)| (p.clone(), effect_stamp(b.as_deref())))
        .collect::<BTreeMap<_, _>>();
    for (path, _, after) in prefix.effects() {
        if !expected.contains_key(&path) {
            return Err("batch效果不属于原Router union".into());
        }
        expected.insert(path, after);
    }
    expected.insert(ctx.config(), effect_stamp(next.config_after()));
    if let Some(catalog) = &next.catalog {
        expected.insert(
            catalog.destination(),
            effect_stamp(token.catalog_payload.as_deref()),
        );
    }
    for action in &next.auth {
        for (path, after) in native_auth::approved_effects(action) {
            expected.insert(path, after);
        }
    }
    let outcome = apply_under_manager_using(ctx, &next, guard, publish)?;
    finish_bound_receipt(ctx, token, expected, outcome)
}
fn finish_bound_receipt(
    ctx: &Context,
    token: &BoundBatchResyncToken,
    expected: BTreeMap<PathBuf, native_auth::EffectStamp>,
    mut outcome: Outcome,
) -> Result<(Outcome, ApprovedResyncEffects)> {
    let mut receipt = ApprovedResyncEffects {
        batch: token.batch.clone(),
        home: ctx.home.clone(),
        app_root: ctx.app_root.clone(),
        expected,
        effects: BTreeMap::new(),
        poisoned: outcome.failure.is_some(),
    };
    if !receipt.poisoned {
        let check = (|| -> Result<()> {
            for (path, after) in &receipt.expected {
                // Validate actual bytes against the originally approved after,
                // then attach observed metadata; never derive an approved hash
                // from whatever the filesystem happens to contain.
                if effect_stamp(read(path)?.as_deref()) != *after {
                    return Err("Router重同步最终效果不等于预审字节".into());
                }
                let before = effect_stamp(token.initial.get(path).and_then(|b| b.as_deref()));
                let initial_time = token.initial_modified.get(path).copied().flatten();
                let before = before.map(|(len, hash)| (len as u64, hash, initial_time));
                let observed_time = bound_modified(path)?;
                let after = after
                    .clone()
                    .map(|(len, hash)| (len as u64, hash, observed_time));
                // Include O->O read-only dependencies as well: a pure stage can
                // change a file that resync restores to the original bytes.
                receipt.effects.insert(path.clone(), (before, after));
            }
            receipt.validate_current(ctx)
        })();
        if let Err(error) = check {
            receipt.poisoned = true;
            outcome.failure = Some(error);
        }
    }
    Ok((outcome, receipt))
}

/// No persisted native or Gateway source exists. This is an observed Off scope,
/// not a RelayState default and not a synthetic Native Snapshot.
#[derive(Clone)]
struct ObservedOff {
    home: PathBuf,
    app_root: PathBuf,
}
impl ObservedOff {
    fn validate(&self, ctx: &Context) -> Result<()> {
        if ctx.home != self.home || ctx.app_root != self.app_root {
            return Err("ObservedOff目录范围变化".into());
        }
        for path in [
            native_catalog::native_state_path(&ctx.home),
            ctx.app_root.join("gateway/catalog.json"),
        ] {
            if read(&path)?.is_some() {
                return Err("ObservedOff已出现实际路由来源，请重新审阅".into());
            }
        }
        let intent = super::router_intent::read(ctx)?;
        if intent.as_ref().is_some_and(|record| {
            record.target
                == atelier_maintenance_core::transition_journal::TransitionTarget::RouterOn
        }) {
            return Err("实际RouterIntent仍为On，不能作为ObservedOff".into());
        }
        if intent.is_none() {
            let bytes = read(&ctx.config())?.unwrap_or_default();
            let text = std::str::from_utf8(&bytes).map_err(|_| "ObservedOff配置不是UTF8")?;
            if catalog::read_top_level_string_value(text, "model_provider")
                .as_deref()
                .is_some_and(core::atelier_provider_owned)
            {
                return Err("无实际来源但配置明确属于Atelier，不能降级ObservedOff".into());
            }
        }
        Ok(())
    }
}
/// The batch's actual mode observer owns malformed-TOML/default semantics. This
/// entry additionally fences all absent source paths and never creates them.
pub(super) fn prepare_bound_observed_off(
    ctx: &Context,
    batch: &super::native_coordinator::PreparedBatch,
) -> Result<BoundBatchResyncToken> {
    batch.validate_initial(ctx)?;
    if batch.mode() != RouterMode::Off {
        return Err("无source但实际模式非Off，不能降级清理".into());
    }
    let observed_off = ObservedOff {
        home: ctx.home.clone(),
        app_root: ctx.app_root.clone(),
    };
    observed_off.validate(ctx)?;
    let mut initial = BTreeMap::new();
    for (path, expected) in batch.original_dependencies() {
        let bytes = read(&path)?;
        if effect_stamp(bytes.as_deref()) != expected {
            return Err("ObservedOff批次初始依赖变化".into());
        }
        initial.insert(path, bytes);
    }
    for path in [
        native_catalog::native_state_path(&ctx.home),
        ctx.app_root.join("gateway/catalog.json"),
        super::router_intent::path(ctx),
        ctx.home.join("auth.json"),
        ctx.home
            .join("codexmate/relay/router-unlock-auth-marker.json"),
        ctx.home
            .join("codexmate/relay/router-unlock-auth-backup.json"),
        ctx.home.join("codexmate/relay/router-takeover-backup.json"),
        ctx.home
            .join("codexmate/relay/codex_router_top_level_backup.json"),
    ] {
        let bytes = read(&path)?;
        if initial
            .get(&path)
            .is_some_and(|original| original != &bytes)
        {
            return Err("ObservedOff初始union不一致".into());
        }
        initial.insert(path, bytes);
    }
    if initial.get(&ctx.config()).and_then(|b| b.as_deref()) != batch.config_before() {
        return Err("ObservedOff批次缺少同一原始配置".into());
    }
    let mut candidates = BTreeMap::new();
    let mut final_candidates = BTreeMap::new();
    for (key, bytes) in batch.config_candidates() {
        if candidates.insert(key, bytes.clone()).is_some() {
            return Err("ObservedOff候选key重复".into());
        }
        let rendered = (|| {
            let text = std::str::from_utf8(bytes.as_deref().unwrap_or_default())
                .map_err(|_| "ObservedOff候选不是UTF8")?;
            let after = observed_off_config(ctx, text)?;
            Ok(optional_config(bytes.as_deref(), after))
        })();
        final_candidates.insert(key, rendered);
    }
    let initial_modified = initial
        .keys()
        .map(|p| Ok((p.clone(), bound_modified(p)?)))
        .collect::<Result<_>>()?;
    let token = BoundBatchResyncToken {
        batch: batch.clone(),
        original: None,
        observed_off: Some(observed_off),
        target: TargetState::NativeOff,
        mode: RouterMode::Off,
        router_dependencies: vec![],
        router_after: None,
        router_auth: vec![],
        initial,
        candidates,
        final_candidates,
        catalog_payload: None,
        initial_modified,
    };
    token.validate_initial(ctx)?;
    Ok(token)
}
fn observed_off_paths(ctx: &Context) -> Result<VerificationPaths> {
    let text = read(&ctx.config())?
        .map(|b| String::from_utf8(b).map_err(err))
        .transpose()?;
    let configured = atelier_maintenance_core::state_db::configured_sqlite_home(
        text.as_deref(),
        ctx.sqlite_home_env.as_deref(),
        ctx.working_directory.as_deref(),
    );
    let dirs = configured
        .map(|d| vec![d])
        .unwrap_or_else(|| vec![ctx.home.join("sqlite"), ctx.home.clone()]);
    let mut databases = vec![];
    for dir in dirs {
        if let Some(db) = atelier_maintenance_core::state_db::highest_state_db(&dir) {
            if !databases.contains(&db) {
                databases.push(db);
            }
        }
    }
    Ok(VerificationPaths {
        codex_home: ctx.home.clone(),
        relay_root: ctx.home.join("codexmate/relay"),
        config_path: ctx.config(),
        databases,
    })
}
fn verify_observed_off_snapshot(ctx: &Context) -> Result<VerificationReport> {
    let paths = observed_off_paths(ctx)?;
    super::snapshot::with_verification_paths(&paths, |projected| {
        let mut report = core::verify_native_off_with_depth(projected, Depth::Full);
        for issue in &mut report.issues {
            for (copy, source) in projected.databases.iter().zip(&paths.databases) {
                issue.message = issue.message.replace(
                    copy.to_string_lossy().as_ref(),
                    source.to_string_lossy().as_ref(),
                );
            }
        }
        Ok(report)
    })
}
/// Executes the exact source-free Off candidate; no catalog/state/account source
/// is synthesized. Authentication remains the original batch's concrete actions.
pub(super) fn apply_bound_observed_off(
    ctx: &Context,
    token: &BoundBatchResyncToken,
    prefix: &super::native_coordinator::ApprovedBatchPrefix,
    backup_root: &Path,
    guard: &super::repair::MutationGuard,
) -> Result<(Outcome, ApprovedResyncEffects)> {
    guard.validate(ctx)?;
    let scope = token
        .observed_off
        .as_ref()
        .ok_or("该token不是ObservedOff")?;
    if token.original.is_some()
        || token.target != TargetState::NativeOff
        || token.mode != RouterMode::Off
        || !token.batch.same_plan(prefix.plan())
    {
        return Err("ObservedOff批次审批身份/目标不一致".into());
    }
    scope.validate(ctx)?;
    prefix.validate_current(ctx)?;
    let before = token
        .candidates
        .get(&prefix.candidate_key())
        .ok_or("ObservedOff中间候选未预审")?;
    if before.as_deref() != prefix.config_after() {
        return Err("ObservedOff候选字节未预审".into());
    }
    let after = token
        .final_candidates
        .get(&prefix.candidate_key())
        .ok_or("ObservedOff最终分支未预审")?
        .clone()?;
    let mut expected = BTreeMap::new();
    let mut auth_prefix = vec![];
    for (path, original) in &token.initial {
        let original = effect_stamp(original.as_deref());
        let wanted = if path == &ctx.config() {
            effect_stamp(before.as_deref())
        } else if let Some((initial, after)) = prefix.effect(path) {
            if original != initial {
                return Err("ObservedOff前序before不属于原始union".into());
            }
            auth_prefix.push((path.clone(), initial, after.clone()));
            after
        } else {
            original
        };
        if effect_stamp(read(path)?.as_deref()) != wanted {
            return Err("ObservedOff原始来源或前序效果变化".into());
        }
        expected.insert(path.clone(), wanted);
    }
    let mut auth = vec![];
    for p in token.batch.auth_actions() {
        auth.push(native_auth::replan_after_approved_effects(
            ctx,
            p,
            RouterMode::Off,
            token.batch.config_before(),
            before.as_deref(),
            &auth_prefix,
        )?);
    }
    let mut outcome = Outcome {
        completed: vec![],
        failure: None,
        backups: vec![],
        verification: VerificationReport::default(),
        warnings: vec![],
    };
    let execute = (|| -> Result<()> {
        private_dir(backup_root)?;
        let manifest = backup_root.join("observed-off-config-original.json");
        let bytes=serde_json::to_vec(&serde_json::json!({"path":ctx.config(),"beforeBase64":before.as_ref().map(|b|{use base64::Engine;base64::engine::general_purpose::STANDARD.encode(b)})})).map_err(err)?;
        atomic_private(&manifest, &bytes)?;
        if fs::read(&manifest).map_err(err)? != bytes {
            return Err("ObservedOff备份回读失败".into());
        }
        outcome.backups.push(manifest);
        guard.validate(ctx)?;
        scope.validate(ctx)?;
        prefix.validate_current(ctx)?;
        for (path, wanted) in &expected {
            if effect_stamp(read(path)?.as_deref()) != *wanted {
                return Err("ObservedOff发布前依赖变化".into());
            }
        }
        if before != &after {
            match &after {
                Some(bytes) => {
                    let p = publication::write_atomic_private_report(&ctx.config(), bytes)
                        .map_err(err)?;
                    if let Err(e) = p.require_sync() {
                        outcome
                            .completed
                            .push("config_published_sync_unconfirmed".into());
                        return Err(err(e));
                    }
                }
                None => {
                    if read(&ctx.config())?.is_some() {
                        fs::remove_file(ctx.config()).map_err(err)?;
                    }
                }
            }
        }
        if read(&ctx.config())? != after {
            return Err("ObservedOff配置回读不等于预审效果".into());
        }
        if after.is_some() {
            fs::File::open(ctx.config())
                .and_then(|f| f.sync_all())
                .map_err(err)?;
        }
        fs::File::open(&ctx.home)
            .and_then(|f| f.sync_all())
            .map_err(err)?;
        expected.insert(ctx.config(), effect_stamp(after.as_deref()));
        outcome.completed.push("config".into());
        let mut fresh_prefix = vec![];
        for p in &auth {
            let (result, effects) = native_auth::apply_after_approved_effects(
                ctx,
                p,
                RouterMode::Off,
                before.as_deref(),
                after.as_deref(),
                &fresh_prefix,
            )?;
            outcome.backups.extend(result.backups);
            if !result.success {
                return Err(result.detail);
            }
            for (path, value) in effects {
                let before = expected
                    .get(&path)
                    .cloned()
                    .ok_or("ObservedOff认证effect缺少初始依赖")?;
                fresh_prefix.push((path.clone(), before, value.clone()));
                expected.insert(path, value);
            }
            outcome.completed.push(native_auth::item_id(p).into());
        }
        Ok(())
    })();
    if let Err(error) = execute {
        outcome.failure = Some(error);
    }
    outcome.verification = verify_observed_off_snapshot(ctx)?;
    if provider(&fs::read_to_string(ctx.config()).unwrap_or_default())
        .is_ok_and(|p| core::atelier_provider_owned(&p))
    {
        push(
            &mut outcome.verification,
            InvariantDim::Config,
            "ObservedOff still selects Atelier provider",
        );
    }
    finish_bound_receipt(ctx, token, expected, outcome)
}

/// Finite end-of-batch alternatives, sealed before any mutation. Branch identity
/// includes Global's exact producer branch, not just its possibly-equal bytes.
pub(super) struct BoundBatchSuccessor {
    seed: BoundBatchResyncToken,
    candidates: Vec<(Option<usize>, Option<Vec<u8>>, Result<Option<Vec<u8>>>)>,
    members: BTreeMap<PathBuf, BTreeSet<PathBuf>>,
}
fn successor_members(ctx: &Context) -> Result<BTreeMap<PathBuf, BTreeSet<PathBuf>>> {
    let mut result = BTreeMap::new();
    for directory in [
        ctx.home.join("accounts/backups"),
        ctx.app_root.join("gateway/credentials"),
    ] {
        let entries = match fs::read_dir(&directory) {
            Ok(entries) => entries
                .map(|e| e.map(|e| e.path()).map_err(err))
                .collect::<Result<BTreeSet<_>>>()?,
            Err(e) if e.kind() == io::ErrorKind::NotFound => BTreeSet::new(),
            Err(e) => return Err(err(e)),
        };
        result.insert(directory, entries);
    }
    Ok(result)
}
impl BoundBatchSuccessor {
    pub fn validate_initial(&self, ctx: &Context) -> Result<()> {
        self.seed.validate_initial(ctx)?;
        self.validate_members(ctx)
    }
    fn validate_members(&self, ctx: &Context) -> Result<()> {
        if successor_members(ctx)? != self.members {
            return Err("批次后继认证目录成员变化，未批准新来源".into());
        }
        Ok(())
    }
    pub fn outputs(&self) -> Vec<Option<Vec<u8>>> {
        self.candidates
            .iter()
            .filter_map(|(_, _, after)| after.clone().ok())
            .collect()
    }
    pub fn extend_expected(
        &self,
        expected: &mut BTreeMap<PathBuf, Option<super::native_orphans::EffectStamp>>,
    ) -> Result<()> {
        for (path, bytes) in &self.seed.initial {
            let stamp = effect_stamp(bytes.as_deref()).map(|(n, h)| {
                (
                    n as u64,
                    h,
                    self.seed.initial_modified.get(path).copied().flatten(),
                )
            });
            expected.entry(path.clone()).or_insert(stamp);
        }
        Ok(())
    }
}
pub(super) fn prepare_batch_successor(
    ctx: &Context,
    token: &BoundBatchResyncToken,
    global: Option<&super::native_coordinator::PreparedGlobal>,
) -> Result<BoundBatchSuccessor> {
    token.validate_initial(ctx)?;
    let mut inputs = token
        .candidates
        .values()
        .cloned()
        .map(|b| (None, b))
        .collect::<Vec<_>>();
    inputs.extend(
        token
            .final_candidates
            .values()
            .filter_map(|b| b.clone().ok())
            .map(|b| (None, b)),
    );
    if let Some(global) = global {
        inputs.extend(
            global
                .successor_inputs()
                .into_iter()
                .map(|(key, b)| (Some(key), b)),
        );
    }
    let candidates = inputs
        .into_iter()
        .map(|(key, before)| {
            let after = (|| {
                let text = std::str::from_utf8(before.as_deref().unwrap_or_default())
                    .map_err(|_| "后继配置不是UTF8")?;
                let after = if let Some(plan) = &token.original {
                    config_patch(
                        ctx,
                        &plan.snapshot,
                        plan.target,
                        text,
                        plan.actual_proxy.as_ref(),
                    )?
                } else {
                    observed_off_config(ctx, text)?
                };
                Ok(optional_config(before.as_deref(), after))
            })();
            (key, before, after)
        })
        .collect();
    let mut seed = token.clone();
    let members = successor_members(ctx)?;
    // Credential contents as well as membership are read-only dependencies.
    for paths in members.values() {
        for path in paths {
            if fs::symlink_metadata(path).map_err(err)?.is_file() {
                seed.initial.entry(path.clone()).or_insert(read(path)?);
                seed.initial_modified
                    .entry(path.clone())
                    .or_insert(bound_modified(path)?);
            }
        }
    }
    let plan = BoundBatchSuccessor {
        seed,
        candidates,
        members,
    };
    plan.validate_initial(ctx)?;
    Ok(plan)
}
/// Executes only after the caller has validated its opaque final BatchReceipt.
/// No current TOML is rendered here; auth is shrunk through the existing original
/// plan validator, and catalog publication remains byte-for-byte preapproved.
pub(super) fn apply_batch_successor(
    ctx: &Context,
    token: &BoundBatchSuccessor,
    receipt: &super::native_batch::BatchReceipt,
    global: Option<&super::native_coordinator::PreparedGlobal>,
    backup: &Path,
    guard: &super::repair::MutationGuard,
) -> Result<(
    Outcome,
    BTreeMap<PathBuf, Option<super::native_orphans::EffectStamp>>,
    Option<Vec<u8>>,
)> {
    guard.validate(ctx)?;
    token.validate_members(ctx)?;
    let seed = &token.seed;
    if let Some(plan) = &seed.original {
        validate_owner(ctx, plan)?;
    }
    if let Some(off) = &seed.observed_off {
        off.validate(ctx)?;
    }
    if let Some(key) = receipt.global_branch() {
        global
            .ok_or("缺少原Global分支审批")?
            .validate_branch(ctx, key, receipt.config_after())?;
    }
    let (_, before, after) = token
        .candidates
        .iter()
        .find(|(key, b, _)| {
            *key == receipt.global_branch() && b.as_deref() == receipt.config_after()
        })
        .ok_or("批次最终凭据不属于已封存后继分支")?;
    let after = after.clone()?;
    let mut dependencies = BTreeMap::new();
    let mut prefix = vec![];
    for (path, original) in &seed.initial {
        let approved = receipt.effect(path).ok_or("后继依赖缺少累计凭据")?;
        if super::repair::ordinary_stamp(path)? != approved {
            return Err("后继累计依赖已变化".into());
        }
        let current = read(path)?;
        let original_stamp = effect_stamp(original.as_deref());
        let approved_stamp = approved.map(|(n, h, _)| (n as usize, h));
        if path != &ctx.config() && approved_stamp != original_stamp {
            prefix.push((path.clone(), original_stamp, approved_stamp));
        }
        dependencies.insert(path.clone(), current);
    }
    if dependencies.get(&ctx.config()) != Some(before) {
        return Err("后继配置不等于累计凭据".into());
    }
    // Source and intent are immutable within the live operation; a producer ledger
    // cannot grant authority to adopt another routing source.
    for path in [
        native_catalog::native_state_path(&ctx.home),
        ctx.app_root.join("gateway/catalog.json"),
        super::router_intent::path(ctx),
    ] {
        if dependencies.get(&path) != seed.initial.get(&path) {
            return Err("后继路由来源/intent变化".into());
        }
    }
    let auth = seed
        .batch
        .auth_actions()
        .iter()
        .map(|p| {
            native_auth::replan_after_approved_effects(
                ctx,
                p,
                seed.mode,
                seed.batch.config_before(),
                before.as_deref(),
                &prefix,
            )
        })
        .collect::<Result<Vec<_>>>()?;
    let mut wanted = dependencies
        .iter()
        .map(|(p, b)| (p.clone(), effect_stamp(b.as_deref())))
        .collect::<BTreeMap<_, _>>();
    wanted.insert(ctx.config(), effect_stamp(after.as_deref()));
    let outcome = if let Some(original) = &seed.original {
        let mut next = original.clone();
        next.config_before = before.clone();
        next.config_after = after.clone();
        next.dependencies = dependencies;
        for p in &auth {
            for (path, b) in native_auth::approved_effects(p) {
                wanted.insert(path, b);
            }
        }
        next.auth = auth;
        next.batch_seed_only = false;
        next.backup_root = backup.to_owned();
        if let Some(catalog) = &original.catalog {
            let fresh = native_catalog::prepare(original.snapshot.source())?;
            if fresh.destination() != catalog.destination()
                || Some(catalog::builder::encode_prepared_catalog(fresh.document()).map_err(err)?)
                    != seed.catalog_payload
            {
                return Err("后继目录不等于原批准payload".into());
            }
            wanted.insert(
                fresh.destination(),
                effect_stamp(seed.catalog_payload.as_deref()),
            );
            next.catalog = Some(fresh);
        }
        apply_under_manager(ctx, &next, guard)?
    } else {
        // Source-free Off has no catalog/state/host to invent. Reuse its exact
        // publication path with a private certified prefix produced below.
        apply_successor_off(ctx, seed, before, &after, auth, &mut wanted, backup, guard)?
    };
    if outcome.failure.is_some() || !outcome.verification.issues.is_empty() {
        return Ok((outcome, BTreeMap::new(), after));
    }
    let mut effects = BTreeMap::new();
    for (path, expected) in wanted {
        if effect_stamp(read(&path)?.as_deref()) != expected {
            return Err("后继效果不等于原批准字节".into());
        }
        effects.insert(
            path.clone(),
            expected.map(|(n, h)| (n as u64, h, bound_modified(&path).ok().flatten())),
        );
    }
    Ok((outcome, effects, after))
}
fn apply_successor_off(
    ctx: &Context,
    seed: &BoundBatchResyncToken,
    before: &Option<Vec<u8>>,
    after: &Option<Vec<u8>>,
    auth: Vec<native_auth::PreparedAction>,
    wanted: &mut BTreeMap<PathBuf, native_auth::EffectStamp>,
    backup: &Path,
    guard: &super::repair::MutationGuard,
) -> Result<Outcome> {
    let mut outcome = Outcome {
        completed: vec![],
        failure: None,
        backups: vec![],
        verification: VerificationReport::default(),
        warnings: vec![],
    };
    let execute = (|| -> Result<()> {
        private_dir(backup)?;
        let manifest = backup.join("successor-config-original.json");
        use base64::Engine;
        atomic_private(&manifest,&serde_json::to_vec(&serde_json::json!({"beforeBase64":before.as_ref().map(|b|base64::engine::general_purpose::STANDARD.encode(b))})).map_err(err)?)?;
        outcome.backups.push(manifest);
        guard.validate(ctx)?;
        seed.observed_off
            .as_ref()
            .ok_or("缺少ObservedOff")?
            .validate(ctx)?;
        if read(&ctx.config())? != *before {
            return Err("后继发布前配置变化".into());
        }
        if before != after {
            match after {
                Some(bytes) => {
                    publication::write_atomic_private_report(&ctx.config(), bytes)
                        .map_err(err)?
                        .require_sync()
                        .map_err(err)?;
                }
                None => {
                    if read(&ctx.config())?.is_some() {
                        fs::remove_file(ctx.config()).map_err(err)?;
                    }
                }
            }
        }
        if read(&ctx.config())? != *after {
            return Err("后继配置回读不匹配".into());
        }
        outcome.completed.push("config".into());
        let mut prefix = vec![];
        for p in &auth {
            let (result, effects) = native_auth::apply_after_approved_effects(
                ctx,
                p,
                seed.mode,
                before.as_deref(),
                after.as_deref(),
                &prefix,
            )?;
            outcome.backups.extend(result.backups);
            if !result.success {
                return Err(result.detail);
            }
            for (path, value) in effects {
                prefix.push((
                    path.clone(),
                    wanted.get(&path).cloned().ok_or("认证后继缺少依赖")?,
                    value.clone(),
                ));
                wanted.insert(path, value);
            }
            outcome.completed.push(native_auth::item_id(p).into());
        }
        Ok(())
    })();
    if let Err(error) = execute {
        outcome.failure = Some(error);
    }
    outcome.verification = verify_observed_off_snapshot(ctx)?;
    Ok(outcome)
}

// The same already-reviewed GatewayOff ownership rules, without requiring a
// fabricated Gateway Catalog merely to deactivate its leftover configuration.
fn observed_off_config(ctx: &Context, text: &str) -> Result<String> {
    let clean = native_off_config(text, &ctx.home.join("codexmate/relay"))?;
    let mut doc = clean
        .parse::<toml_edit::DocumentMut>()
        .map_err(|_| "ObservedOff候选不是有效TOML")?;
    let effective = provider(&clean)?;
    if core::atelier_provider_owned(&effective) {
        if let Some(selected) = doc
            .get("profile")
            .and_then(toml_edit::Item::as_str)
            .map(str::to_owned)
        {
            if doc
                .get("profiles")
                .and_then(|p| p.get(&selected))
                .and_then(|p| p.get("model_provider"))
                .and_then(toml_edit::Item::as_str)
                .is_some_and(core::atelier_provider_owned)
            {
                doc.remove("profile");
            }
        }
        if doc
            .get("model_provider")
            .and_then(toml_edit::Item::as_str)
            .is_some_and(core::atelier_provider_owned)
        {
            doc.remove("model_provider");
        }
    }
    if let Some(path) = doc
        .get("model_catalog_json")
        .and_then(toml_edit::Item::as_str)
    {
        let path = Path::new(path);
        let path = if path.is_absolute() {
            path.to_path_buf()
        } else {
            ctx.home.join(path)
        };
        if config::normalize_catalog_path(&path.to_string_lossy())
            == config::normalize_catalog_path(
                &ctx.home.join(catalog::CATALOG_FILE).to_string_lossy(),
            )
        {
            doc.remove("model_catalog_json");
        }
    }
    Ok(doc.to_string())
}
