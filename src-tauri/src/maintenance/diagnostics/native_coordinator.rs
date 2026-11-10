//! Concrete reviewed composition for config_global/profile + takeover/unlock + catalog.
//! This is a repair-stage coordinator, not a replacement RelayManager or routing intent engine.
use super::{err, native_auth, native_catalog, Context, Result};
use crate::gateway::storage::{atomic_private, open_private, private_dir};
use atelier_maintenance_core::{
    auth::RouterMode,
    catalog::{builder, publication, state::RelayStateDocument},
    config::{self, ConfigFs},
};
use fs2::FileExt;
use serde::Serialize;
use sha2::{Digest, Sha256};
use std::{
    cell::RefCell,
    collections::{BTreeMap, BTreeSet},
    fs, io,
    path::{Path, PathBuf},
    sync::Arc,
};
#[path = "native_coordinator_batch.rs"]
mod batch;
pub(super) use batch::*;

#[derive(Clone, PartialEq, Eq)]
struct Stamp {
    length: usize,
    sha256: String,
}
fn stamp_bytes(bytes: &[u8]) -> Stamp {
    Stamp {
        length: bytes.len(),
        sha256: format!("{:x}", Sha256::digest(bytes)),
    }
}
fn read(path: &Path) -> Result<Option<Vec<u8>>> {
    match fs::symlink_metadata(path) {
        Err(e) if e.kind() == io::ErrorKind::NotFound => Ok(None),
        Err(e) => Err(err(e)),
        Ok(m) => {
            if !m.is_file() || m.file_type().is_symlink() {
                return Err("组合修复依赖不是普通文件".into());
            }
            if m.len() > 32 * 1024 * 1024 {
                return Err("组合修复依赖超出32MiB上限".into());
            }
            let bytes = fs::read(path).map_err(err)?;
            if bytes.len() > 32 * 1024 * 1024 {
                return Err("组合修复依赖读取超出32MiB上限".into());
            }
            Ok(Some(bytes))
        }
    }
}
fn stamp(path: &Path) -> Result<Option<Stamp>> {
    Ok(read(path)?.as_deref().map(stamp_bytes))
}
fn insert_dependency(
    deps: &mut BTreeMap<PathBuf, Option<Stamp>>,
    path: PathBuf,
    value: Option<Stamp>,
) -> Result<()> {
    if deps.get(&path).is_some_and(|prior| prior != &value) {
        return Err("组合计划的初始依赖不一致".into());
    }
    deps.insert(path, value);
    Ok(())
}
fn tuple_stamp(value: Option<(usize, String)>) -> Option<Stamp> {
    value.map(|(length, sha256)| Stamp { length, sha256 })
}
#[derive(Default)]
struct ObservedMetadata(
    RefCell<BTreeMap<PathBuf, std::result::Result<(), io::ErrorKind>>>,
    RefCell<BTreeMap<PathBuf, Option<Stamp>>>,
);
impl ConfigFs for ObservedMetadata {
    fn metadata(&self, path: &Path) -> io::Result<()> {
        let result = fs::metadata(path).map(|_| ()).map_err(|e| e.kind());
        self.0.borrow_mut().insert(path.to_owned(), result);
        result.map_err(io::Error::from)
    }
}
pub(super) fn handles(id: &str) -> bool {
    matches!(
        id,
        "config_global_validity"
            | "config_third_party"
            | "config_profile_conflict"
            | "config_stale"
            | "router_http_contract"
    ) || native_auth::handles(id)
        || native_catalog::handles(id)
}
fn source_input(source: &native_catalog::Source) -> &Path {
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
fn catalog_config_patch(source: &native_catalog::Source, text: &str) -> Result<String> {
    let mut doc = text
        .parse::<toml_edit::DocumentMut>()
        .map_err(|_| "组合配置不是有效TOML")?;
    let static_path = match source {
        native_catalog::Source::Native { state_path, .. } => {
            let bytes = read(state_path)?.ok_or("原生状态不存在")?;
            RelayStateDocument::from_json(&bytes)
                .map_err(err)?
                .catalog_inputs()
                .map_err(err)?
                .no_account_mode
        }
        native_catalog::Source::Gateway { .. } => true,
    };
    if static_path {
        let path = source_root(source).join(atelier_maintenance_core::catalog::CATALOG_FILE);
        if !path.is_absolute() {
            return Err("组合目录路径需要显式绝对路径".into());
        }
        doc["model_catalog_json"] = toml_edit::value(path.to_str().ok_or("组合目录路径不是UTF8")?);
    } else {
        doc.remove("model_catalog_json");
    }
    Ok(doc.to_string())
}
fn gateway_config_patch(ctx: &Context, text: &str, observed: &ObservedMetadata) -> Result<String> {
    let path = ctx.app_root.join("gateway/catalog.json");
    let bytes = read(&path)?.ok_or("Atelier网关目录不存在")?;
    observed
        .1
        .borrow_mut()
        .insert(path, Some(stamp_bytes(&bytes)));
    let gateway: crate::gateway::types::Catalog =
        serde_json::from_slice(&bytes).map_err(|_| "Atelier网关目录不符合实际Catalog schema")?;
    if gateway.settings.port == 0 || gateway.host_id.is_empty() || gateway.profiles.len() > 64 {
        return Err("Atelier网关port/host/profiles超出实际支持范围".into());
    }
    let parsed: toml::Value = toml::from_str(text).map_err(|_| "配置不是有效TOML")?;
    let provider = super::inspect::provider(&parsed);
    if !(provider == "atelier" || provider == "codex_atelier" || provider.starts_with("atelier_")) {
        return Err("当前有效Provider不属于Atelier，未批准接管自定义连接".into());
    }
    let mut doc = text
        .parse::<toml_edit::DocumentMut>()
        .map_err(|_| "配置不是有效TOML")?;
    let providers = doc
        .get_mut("model_providers")
        .and_then(toml_edit::Item::as_table_like_mut)
        .ok_or("配置没有既有model_providers表")?;
    let section = providers
        .get_mut(&provider)
        .and_then(toml_edit::Item::as_table_like_mut)
        .ok_or("当前Atelier Provider不是既有配置表")?;
    section.insert(
        "base_url",
        toml_edit::value(format!("http://127.0.0.1:{}/v1", gateway.settings.port)),
    );
    section.insert("wire_api", toml_edit::value("responses"));
    section.insert("env_key", toml_edit::value("ATELIER_GATEWAY_TOKEN"));
    section.insert("requires_openai_auth", toml_edit::value(false));
    section.insert("supports_websockets", toml_edit::value(false));
    Ok(doc.to_string())
}
#[derive(Clone)]
pub(super) struct Prepared {
    home: PathBuf,
    app_root: PathBuf,
    backup_root: PathBuf,
    ids: Vec<String>,
    mode: RouterMode,
    config_before: Option<Vec<u8>>,
    config_after: Option<Vec<u8>>,
    auth: Vec<native_auth::PreparedAction>,
    catalog_source: Option<native_catalog::Source>,
    catalog: Option<native_catalog::PreparedAction>,
    dependencies: BTreeMap<PathBuf, Option<Stamp>>,
    metadata: BTreeMap<PathBuf, std::result::Result<(), io::ErrorKind>>,
}
impl Prepared {
    pub fn item_ids(&self) -> &[String] {
        &self.ids
    }
    pub fn config_before(&self) -> Option<&[u8]> {
        self.config_before.as_deref()
    }
    pub fn config_after(&self) -> Option<&[u8]> {
        self.config_after.as_deref()
    }
    pub fn config_path(&self) -> PathBuf {
        self.home.join("config.toml")
    }
    /// Parent generic-file transactions must exclude this path once composition owns it.
    pub fn owns_config(&self) -> bool {
        true
    }
}
fn composed_config(
    ctx: &Context,
    ids: &[String],
    before: Option<&[u8]>,
    auth: &[native_auth::PreparedAction],
    source: Option<&native_catalog::Source>,
    metadata: &ObservedMetadata,
) -> Result<Option<Vec<u8>>> {
    let mut text = std::str::from_utf8(before.unwrap_or(b""))
        .map_err(|_| "组合配置不是UTF8")?
        .to_owned();
    text.parse::<toml_edit::DocumentMut>()
        .map_err(|_| "先修复配置语法，再组合配置效果")?;
    for plan in auth {
        if let Some((path, original, after)) = native_auth::planned_config(plan) {
            if path != ctx.config() || original.as_deref() != before {
                return Err("认证配置计划不属于同一初始配置".into());
            }
            let after = after.ok_or("认证计划试图删除整个配置，未批准组合")?;
            text = String::from_utf8(after).map_err(|_| "认证恢复配置不是UTF8")?;
        }
    }
    if ids.iter().any(|id| id == "config_third_party") {
        let (known, dependencies) =
            super::native_config::known_provider_ids_with_dependencies(ctx)?;
        for (path, stamp) in dependencies {
            metadata.1.borrow_mut().insert(path, tuple_stamp(stamp));
        }
        text = config::fix_config_third_party_text(&text, &ctx.home, &known)
            .map_err(err)?
            .after;
    }
    if ids.iter().any(|id| id == "config_global_validity") {
        text = config::fix_config_global_validity_at(
            &text,
            &ctx.config(),
            &ctx.home.join("codexmate/relay"),
            metadata,
        )
        .map_err(err)?
        .after;
    }
    if ids.iter().any(|id| id == "config_profile_conflict") {
        text = config::fix_config_profile_conflict_text(&text, &ctx.home).after;
    }
    if ids
        .iter()
        .any(|id| matches!(id.as_str(), "config_stale" | "router_http_contract"))
    {
        text = gateway_config_patch(ctx, &text, metadata)?;
    }
    if let Some(source) = source {
        text = catalog_config_patch(source, &text)?;
    }
    text.parse::<toml_edit::DocumentMut>()
        .map_err(|_| "组合效果未产生有效TOML")?;
    Ok(if before.is_none() && text.is_empty() {
        None
    } else {
        Some(text.into_bytes())
    })
}
pub(super) fn prepare(
    ctx: &Context,
    ids: &[String],
    catalog_source: Option<&native_catalog::Source>,
    backup_root: &Path,
) -> Result<Prepared> {
    if ids.is_empty() || ids.iter().any(|id| !handles(id)) {
        return Err("组合器仅支持明确恢复的配置/认证/Catalog项目".into());
    }
    let mut seen = BTreeSet::new();
    let ids: Vec<_> = ids
        .iter()
        .filter(|id| seen.insert((*id).clone()))
        .cloned()
        .collect();
    let mode = native_auth::resolve_mode(ctx)?;
    let before = read(&ctx.config())?;
    let auth = native_auth::prepare_sequence(ctx, &ids, backup_root, mode)?;
    let mut non_config_targets: BTreeMap<PathBuf, String> = BTreeMap::new();
    for plan in &auth {
        let id = native_auth::item_id(plan);
        for path in native_auth::approved_paths(plan) {
            if path == ctx.config() {
                continue;
            }
            if let Some(prior) = non_config_targets.get(&path) {
                if !(path == ctx.home.join("auth.json")
                    && prior == "auth_integrity"
                    && id == "router_unlock_auth_residue")
                {
                    return Err("认证计划含未批准的重叠目标，需要独立重审".into());
                }
            } else {
                non_config_targets.insert(path, id.to_owned());
            }
        }
    }
    let wants_catalog = ids.iter().any(|id| native_catalog::handles(id));
    let source = if wants_catalog {
        Some(catalog_source.ok_or("目录组合需要实际类型化来源")?.clone())
    } else {
        None
    };
    let catalog = source.as_ref().map(native_catalog::prepare).transpose()?;
    let metadata = ObservedMetadata::default();
    let after = composed_config(
        ctx,
        &ids,
        before.as_deref(),
        &auth,
        source.as_ref(),
        &metadata,
    )?;
    let mut dependencies = BTreeMap::new();
    insert_dependency(
        &mut dependencies,
        ctx.config(),
        before.as_deref().map(stamp_bytes),
    )?;
    for plan in &auth {
        for (path, value) in native_auth::approved_dependencies(plan) {
            insert_dependency(&mut dependencies, path, tuple_stamp(value))?;
        }
    }
    if let (Some(source), Some(catalog)) = (&source, &catalog) {
        insert_dependency(
            &mut dependencies,
            source_input(source).to_owned(),
            stamp(source_input(source))?,
        )?;
        insert_dependency(
            &mut dependencies,
            catalog.destination(),
            stamp(&catalog.destination())?,
        )?;
        native_catalog::validate(catalog)?;
    }
    for (path, value) in metadata.1.into_inner() {
        insert_dependency(&mut dependencies, path, value)?;
    }
    let plan = Prepared {
        home: ctx.home.clone(),
        app_root: ctx.app_root.clone(),
        backup_root: backup_root.to_owned(),
        ids,
        mode,
        config_before: before,
        config_after: after,
        auth,
        catalog_source: source,
        catalog,
        dependencies,
        metadata: metadata.0.into_inner(),
    };
    validate_initial(ctx, &plan)?;
    Ok(plan)
}
fn validate_ledger(
    expected: &BTreeMap<PathBuf, Option<Stamp>>,
    metadata: &BTreeMap<PathBuf, std::result::Result<(), io::ErrorKind>>,
) -> Result<()> {
    for (path, value) in expected {
        if stamp(path)? != *value {
            return Err(format!("组合依赖已变化：{}；请重新预览", path.display()));
        }
    }
    for (path, value) in metadata {
        if fs::metadata(path).map(|_| ()).map_err(|e| e.kind()) != *value {
            return Err(format!("配置metadata依赖已变化：{}", path.display()));
        }
    }
    Ok(())
}
pub(super) fn preview_lines(plan: &Prepared) -> Vec<String> {
    let mut lines=vec![format!("组合审阅{}个项目；takeover恢复、global/profile清理、Atelier连接契约与catalog路径效果只提交一次config。",plan.ids.len()),
        format!("配置目标：{}；保留未涉及字段和原始字节备份。",plan.config_path().display()),
        "先校验全部原始依赖，再发布目录、唯一配置和已批准的认证清理；后续失败保留已完成阶段明细。".into()];
    if plan.catalog.is_some() {
        lines.push("目录使用真实类型化来源和构造器；不扩展Provider孤儿或未知网关协议。".into());
    }
    if plan.ids.iter().any(|id| {
        matches!(
            id.as_str(),
            "config_third_party"
                | "config_profile_conflict"
                | "config_stale"
                | "router_http_contract"
        )
    }) {
        lines.push("配置效果与leaf复检不代替完整Manager路由重收敛或真实HTTP请求验证。".into());
    }
    lines
}
pub(super) fn validate_initial(ctx: &Context, plan: &Prepared) -> Result<()> {
    if ctx.home != plan.home
        || ctx.app_root != plan.app_root
        || native_auth::resolve_mode(ctx)? != plan.mode
    {
        return Err("组合上下文或真实路由模式已变化".into());
    }
    validate_ledger(&plan.dependencies, &plan.metadata)?;
    for auth in &plan.auth {
        native_auth::validate(ctx, auth, plan.mode)?;
    }
    if let Some(catalog) = &plan.catalog {
        native_catalog::validate(catalog)?;
    }
    Ok(())
}
#[derive(Clone, Serialize)]
pub(super) struct StageResult {
    pub stage: String,
    pub success: bool,
    pub changed: usize,
    pub detail: String,
    pub backups: Vec<PathBuf>,
    pub partial_effects_possible: bool,
}
#[derive(Clone, Serialize)]
pub(super) struct Outcome {
    pub completed: Vec<StageResult>,
    pub failure: Option<String>,
    pub backup_path: PathBuf,
    pub diagnostics: Vec<super::DiagnosticItem>,
    pub requires_manager_reconcile: bool,
}
pub(super) struct Session {
    plan: Arc<Prepared>,
    expected: BTreeMap<PathBuf, Option<Stamp>>,
    metadata: BTreeMap<PathBuf, std::result::Result<(), io::ErrorKind>>,
    outcome: Outcome,
    integrity_done: bool,
    catalog_done: bool,
    config_done: bool,
    auth_done: bool,
    _gate: Option<fs::File>,
}
fn backup_initial_config(plan: &Prepared) -> Result<PathBuf> {
    let root = plan
        .backup_root
        .join(format!("composed-config-{}", uuid::Uuid::new_v4()));
    private_dir(&root)?;
    if let Some(bytes) = &plan.config_before {
        atomic_private(&root.join("config.toml.before"), bytes)?;
        if read(&root.join("config.toml.before"))?.as_deref() != Some(bytes.as_slice()) {
            return Err("组合配置原始备份读回失败".into());
        }
    }
    atomic_private(&root.join("manifest.json"),&serde_json::to_vec_pretty(&serde_json::json!({"path":plan.config_path(),"present":plan.config_before.is_some(),
        "backup":plan.config_before.as_ref().map(|_|"config.toml.before"),"beforeSha256":plan.config_before.as_deref().map(|b|stamp_bytes(b).sha256),
        "afterSha256":plan.config_after.as_deref().map(|b|stamp_bytes(b).sha256),"itemIds":plan.ids})).map_err(err)?)?;
    Ok(root)
}
/// Parent repair driver already owns codex-write.lock and may run its thread stages first.
/// It calls validate_initial before any stage, then begins this concrete subset under that gate.
pub(super) fn begin_under_existing_gate(ctx: &Context, plan: &Prepared) -> Result<Session> {
    validate_initial(ctx, plan)?;
    let backup = backup_initial_config(plan)?;
    validate_initial(ctx, plan)?;
    Ok(Session {
        plan: Arc::new(plan.clone()),
        expected: plan.dependencies.clone(),
        metadata: plan.metadata.clone(),
        outcome: Outcome {
            completed: vec![],
            failure: None,
            backup_path: backup,
            diagnostics: vec![],
            requires_manager_reconcile: plan.ids.iter().any(|id| {
                matches!(
                    id.as_str(),
                    "config_third_party"
                        | "config_profile_conflict"
                        | "config_stale"
                        | "router_http_contract"
                )
            }),
        },
        integrity_done: false,
        catalog_done: false,
        config_done: false,
        auth_done: false,
        _gate: None,
    })
}
impl Session {
    pub fn outcome(&self) -> &Outcome {
        &self.outcome
    }
    pub fn into_outcome(self) -> Outcome {
        self.outcome
    }
    fn record(&mut self, stage: StageResult) {
        if !stage.success {
            self.outcome.failure = Some(stage.detail.clone());
        }
        self.outcome.completed.push(stage);
        let bytes = match serde_json::to_vec_pretty(&self.outcome) {
            Ok(bytes) => bytes,
            Err(e) => {
                self.outcome.failure = Some(format!("阶段结果保留在内存；恢复记录编码失败：{e}"));
                return;
            }
        };
        if let Err(e) = atomic_private(&self.outcome.backup_path.join("stage-results.json"), &bytes)
        {
            self.outcome.failure = Some(format!("阶段结果保留在内存；写入恢复记录失败：{e}"));
        }
    }
    fn fail(&mut self, stage: &str, error: String, partial: bool) {
        self.record(StageResult {
            stage: stage.into(),
            success: false,
            changed: 0,
            detail: error,
            backups: vec![],
            partial_effects_possible: partial,
        });
    }
    fn check(&self) -> Result<()> {
        validate_ledger(&self.expected, &self.metadata)
    }
}
fn stamp_tuple(stamp: &Option<Stamp>) -> Option<(usize, String)> {
    stamp.as_ref().map(|v| (v.length, v.sha256.clone()))
}
fn apply_reviewed_auth(
    ctx: &Context,
    session: &mut Session,
    plan: &native_auth::PreparedAction,
    config_after: Option<&[u8]>,
) -> bool {
    if let Err(e) = session.check() {
        session.fail("auth", e, false);
        return false;
    }
    if native_auth::resolve_mode(ctx).ok() != Some(session.plan.mode) {
        session.fail(
            "auth",
            "真实路由模式改变；保留已完成效果，需Manager重收敛".into(),
            false,
        );
        return false;
    }
    let paths = native_auth::approved_dependencies(plan)
        .into_iter()
        .map(|(p, _)| p)
        .collect::<BTreeSet<_>>();
    let prefix = session
        .expected
        .iter()
        .filter_map(|(path, after)| {
            if path == &ctx.config() || !paths.contains(path) {
                return None;
            }
            let before = session.plan.dependencies.get(path)?;
            (before != after).then(|| (path.clone(), stamp_tuple(before), stamp_tuple(after)))
        })
        .collect::<Vec<_>>();
    let applied = native_auth::apply_after_approved_effects(
        ctx,
        plan,
        session.plan.mode,
        session.plan.config_before.as_deref(),
        config_after,
        &prefix,
    );
    match applied {
        Err(e) => {
            session.fail(native_auth::item_id(plan), e, true);
            false
        }
        Ok((result, effects)) => {
            let exact = effects
                .iter()
                .filter(|(p, _)| p != &ctx.config())
                .all(|(path, after)| stamp(path).ok() == Some(tuple_stamp(after.clone())));
            if exact {
                for (path, after) in effects {
                    if path == ctx.config() {
                        continue;
                    }
                    let after = tuple_stamp(after);
                    session.expected.insert(path.clone(), after);
                    if session.metadata.contains_key(&path) {
                        let observed = fs::metadata(&path).map(|_| ()).map_err(|e| e.kind());
                        session.metadata.insert(path, observed);
                    }
                }
            }
            let success = result.success && exact;
            session.record(StageResult {
                stage: result.id,
                success,
                changed: result.changed,
                detail: if exact {
                    result.detail
                } else {
                    "实际fresh认证结果不等于已批准effect；保留此前完成结果".into()
                },
                backups: result.backups,
                partial_effects_possible: !success,
            });
            success
        }
    }
}
/// fix_all ASM: 1003a6d54 -> fix_auth_integrity, before unlock at 1003a720c.
/// This prefix runs before the composed config can change auth-store/routing gates.
pub(super) fn auth_integrity_stage(ctx: &Context, session: &mut Session) {
    if session.integrity_done || session.outcome.failure.is_some() {
        return;
    }
    if ctx.home != session.plan.home || ctx.app_root != session.plan.app_root {
        session.fail("auth_integrity", "组合上下文已变化".into(), false);
        return;
    }
    if let Some(plan) = session
        .plan
        .auth
        .iter()
        .find(|p| native_auth::item_id(p) == "auth_integrity")
        .cloned()
    {
        let initial = session.plan.config_before.clone();
        if !apply_reviewed_auth(ctx, session, &plan, initial.as_deref()) {
            return;
        }
    }
    session.integrity_done = true;
}
pub(super) fn catalog_stage(ctx: &Context, session: &mut Session) {
    auth_integrity_stage(ctx, session);
    if session.catalog_done || session.outcome.failure.is_some() {
        return;
    }
    if ctx.home != session.plan.home || ctx.app_root != session.plan.app_root {
        session.fail("catalog", "组合上下文已变化".into(), false);
        return;
    }
    let action = (|| -> Result<Option<native_catalog::Outcome>> {
        session.check()?;
        let Some(source) = &session.plan.catalog_source else {
            return Ok(None);
        };
        let reviewed = session.plan.catalog.as_ref().ok_or("缺少目录初始计划")?;
        let fresh = native_catalog::prepare(source)?;
        if fresh.revision != reviewed.revision
            || fresh.document() != reviewed.document()
            || fresh.destination() != reviewed.destination()
        {
            return Err("fresh目录计划超出已批准内容".into());
        }
        native_catalog::apply(fresh).map(Some)
    })();
    match action {
        Err(e) => session.fail("catalog", e, true),
        Ok(None) => session.catalog_done = true,
        Ok(Some(result)) => {
            let reviewed = session.plan.catalog.as_ref().unwrap();
            let bytes = builder::encode_prepared_catalog(reviewed.document()).unwrap();
            let approved = Some(stamp_bytes(&bytes));
            if stamp(&result.destination).ok() != Some(approved.clone()) {
                session.fail("catalog", "目录实际读回不等于已批准effect".into(), true);
                return;
            }
            session
                .expected
                .insert(result.destination.clone(), approved);
            if session.metadata.contains_key(&result.destination) {
                session.metadata.insert(result.destination.clone(), Ok(()));
            }
            session.catalog_done = true;
            session.record(StageResult {
                stage: "catalog".into(),
                success: true,
                changed: 1,
                detail: "已发布实际构造目录，未重写共享config".into(),
                backups: vec![result.backup_path],
                partial_effects_possible: false,
            });
        }
    }
}
pub(super) fn config_stage(ctx: &Context, session: &mut Session) {
    auth_integrity_stage(ctx, session);
    if session.config_done || session.outcome.failure.is_some() {
        return;
    }
    if ctx.home != session.plan.home || ctx.app_root != session.plan.app_root {
        session.fail("config", "组合上下文已变化".into(), false);
        return;
    }
    if !session.catalog_done {
        if session.plan.catalog.is_some() {
            session.fail("config", "必须先完成获批目录stage".into(), false);
            return;
        }
        session.catalog_done = true;
    }
    let action = (|| -> Result<bool> {
        session.check()?;
        let observed = ObservedMetadata::default();
        let fresh = composed_config(
            ctx,
            &session.plan.ids,
            session.plan.config_before.as_deref(),
            &session.plan.auth,
            session.plan.catalog_source.as_ref(),
            &observed,
        )?;
        for (path, value) in observed.1.borrow().iter() {
            if session.expected.get(path) != Some(value) {
                return Err("fresh配置引用了未批准的gateway输入".into());
            }
        }
        for (path, value) in observed.0.borrow().iter() {
            if session.metadata.get(path) != Some(value) {
                return Err("fresh配置metadata不属于已批准effect".into());
            }
        }
        if fresh != session.plan.config_after {
            return Err("fresh配置效果不等于已批准的组合字节".into());
        }
        if session.plan.config_before == fresh {
            return Ok(false);
        }
        match &fresh {
            Some(bytes) => {
                publication::write_atomic_report(&ctx.config(), bytes)
                    .map_err(err)?
                    .acknowledge();
            }
            None => {
                fs::remove_file(ctx.config()).map_err(err)?;
            }
        }
        if read(&ctx.config())? != fresh {
            return Err("组合配置发布后读回不一致".into());
        }
        Ok(true)
    })();
    match action {
        Err(e) => session.fail("config", e, true),
        Ok(changed) => {
            session.expected.insert(
                ctx.config(),
                session.plan.config_after.as_deref().map(stamp_bytes),
            );
            session.config_done = true;
            session.record(StageResult {
                stage: "config".into(),
                success: true,
                changed: usize::from(changed),
                detail: "takeover/global/profile/catalog效果已合并并只发布一次config".into(),
                backups: vec![session.outcome.backup_path.clone()],
                partial_effects_possible: false,
            });
        }
    }
}
pub(super) fn auth_stage(ctx: &Context, session: &mut Session) {
    if session.auth_done || session.outcome.failure.is_some() {
        return;
    }
    if ctx.home != session.plan.home || ctx.app_root != session.plan.app_root {
        session.fail("auth", "组合上下文已变化".into(), false);
        return;
    }
    if !session.config_done {
        session.fail("auth", "必须先提交获批唯一组合config".into(), false);
        return;
    }
    for plan in session.plan.auth.clone() {
        if native_auth::item_id(&plan) == "auth_integrity" {
            continue;
        }
        let final_config = session.plan.config_after.clone();
        if !apply_reviewed_auth(ctx, session, &plan, final_config.as_deref()) {
            return;
        }
    }
    session.auth_done = true;
}
/// Reread the actual subset report. Config and catalog leaf checks do not execute
/// a Manager transition or a live HTTP request, which remains explicit in the receipt.
pub(super) fn postcheck(ctx: &Context, session: &mut Session) {
    let result = (|| -> Result<Vec<super::DiagnosticItem>> {
        let bytes = read(&ctx.config())?;
        let text = bytes
            .as_deref()
            .map(std::str::from_utf8)
            .transpose()
            .map_err(|_| "最终配置不是UTF8")?;
        let mode = native_auth::resolve_mode(ctx)?;
        let mut diagnostics = Vec::new();
        if session
            .plan
            .ids
            .iter()
            .any(|id| id == "config_global_validity")
        {
            diagnostics.push(super::native_config::global(
                text,
                &ctx.home.join("codexmate/relay"),
                false,
            )?);
        }
        if session.plan.ids.iter().any(|id| id == "config_third_party") {
            diagnostics.push(super::native_config::third_party(ctx, text, false)?);
        }
        if session
            .plan
            .ids
            .iter()
            .any(|id| id == "config_profile_conflict")
        {
            diagnostics.push(super::native_config::profile(text, mode.enabled(), false)?);
        }
        if session
            .plan
            .ids
            .iter()
            .any(|id| matches!(id.as_str(), "config_stale" | "router_http_contract"))
        {
            let actual = text.unwrap_or("");
            let expected = gateway_config_patch(ctx, actual, &ObservedMetadata::default())?;
            let actual: toml::Value = toml::from_str(actual).map_err(|_| "最终配置不是有效TOML")?;
            let expected: toml::Value =
                toml::from_str(&expected).map_err(|_| "最终连接契约不是有效TOML")?;
            let ok = actual == expected;
            for id in &session.plan.ids {
                if matches!(id.as_str(), "config_stale" | "router_http_contract") {
                    diagnostics.push(super::DiagnosticItem{id:id.clone(),group:"config".into(),status:if ok{"ok"}else{"warning"}.into(),
                        label:"Atelier Responses连接配置字段复检".into(),detail:"已按真实gateway端口复检配置字段；未发送HTTP探测，Manager路由重收敛由上层完成。".into(),fixable:!ok});
                }
            }
        }
        if let Some(source) = &session.plan.catalog_source {
            diagnostics.extend(
                native_catalog::inspect(source, text, false)?
                    .into_iter()
                    .filter(|d| session.plan.ids.contains(&d.id)),
            );
        }
        if !session.plan.auth.is_empty() {
            diagnostics.extend(
                native_auth::inspect(ctx, mode)
                    .into_iter()
                    .filter(|d| session.plan.ids.contains(&d.id)),
            );
        }
        Ok(diagnostics)
    })();
    match result {
        Ok(diagnostics) => {
            if session.outcome.failure.is_none() && diagnostics.iter().any(|d| d.status != "ok") {
                session.outcome.failure =
                    Some("实际subset复检仍有问题；已保留具体完成阶段和备份".into());
            }
            session.outcome.diagnostics = diagnostics;
        }
        Err(e) => {
            if session.outcome.failure.is_none() {
                session.outcome.failure = Some(format!("已完成阶段保留；最终subset复检失败：{e}"));
            }
        }
    }
    match serde_json::to_vec_pretty(&session.outcome)
        .map_err(err)
        .and_then(|bytes| {
            atomic_private(
                &session.outcome.backup_path.join("stage-results.json"),
                &bytes,
            )
        }) {
        Ok(()) => {}
        Err(e) => {
            let previous = session.outcome.failure.take().unwrap_or_default();
            session.outcome.failure = Some(format!(
                "{previous}；阶段结果仍在内存，恢复记录写入失败：{e}"
            ));
        }
    }
}
pub(super) fn apply(ctx: &Context, plan: &Prepared) -> Result<Outcome> {
    let dir = ctx.app_root.join("maintenance");
    private_dir(&dir)?;
    let gate = open_private(&dir.join("codex-write.lock"))?;
    gate.try_lock_exclusive()
        .map_err(|_| "其它维护操作正在修改配置")?;
    let mut session = begin_under_existing_gate(ctx, plan)?;
    session._gate = Some(gate);
    auth_integrity_stage(ctx, &mut session);
    catalog_stage(ctx, &mut session);
    config_stage(ctx, &mut session);
    auth_stage(ctx, &mut session);
    postcheck(ctx, &mut session);
    Ok(session.into_outcome())
}

#[cfg(test)]
#[path = "native_coordinator_tests.rs"]
mod tests;
