//! Reviewed adapter for native unlock and takeover recovery. Credential bytes
//! stay in private, non-Debug/non-Serialize plans; previews contain counts only.
use super::*;
use crate::gateway::storage::{atomic_private, private_dir};
use atelier_maintenance_core::auth::{
    self, takeover, unlock, AuthContext, AuthPlan, Level, RouterMode,
};
use sha2::{Digest, Sha256};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    sync::Arc,
};
const IDS: [&str; 3] = [
    "auth_integrity",
    "router_unlock_auth_residue",
    "takeover_backup_orphan",
];
pub(super) fn handles(id: &str) -> bool {
    IDS.contains(&id)
}
#[derive(Clone, Eq, PartialEq)]
struct Fingerprint {
    len: usize,
    sha256: String,
}
fn fingerprint(path: &Path) -> Result<Option<Fingerprint>> {
    match fs::symlink_metadata(path) {
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(None),
        Err(_) => Err("无法核对认证恢复依赖文件".into()),
        Ok(m) => {
            if !m.is_file() || m.file_type().is_symlink() {
                return Err("认证恢复依赖不是普通文件，已保留".into());
            }
            let bytes = read_bounded(path, inspect::LIMIT)?;
            Ok(Some(Fingerprint {
                len: bytes.len(),
                sha256: format!("{:x}", Sha256::digest(&bytes)),
            }))
        }
    }
}
fn bytes_fingerprint(bytes: &Option<Vec<u8>>) -> Option<Fingerprint> {
    bytes.as_ref().map(|b| Fingerprint {
        len: b.len(),
        sha256: format!("{:x}", Sha256::digest(b)),
    })
}
enum Body {
    Integrity(AuthPlan),
    Unlock(AuthPlan),
    Takeover(takeover::TakeoverPlan),
}
impl Body {
    fn files(&self) -> &AuthPlan {
        match self {
            Self::Integrity(p) | Self::Unlock(p) => p,
            Self::Takeover(p) => &p.files,
        }
    }
}
#[derive(Clone)]
pub(super) struct PreparedAction {
    id: String,
    home: PathBuf,
    app_root: PathBuf,
    backup_root: PathBuf,
    mode: RouterMode,
    dependencies: BTreeMap<PathBuf, Option<Fingerprint>>,
    scope: BTreeSet<PathBuf>,
    body: Arc<Body>,
}
#[derive(Clone, Debug)]
pub(super) struct Outcome {
    pub id: String,
    pub success: bool,
    pub changed: usize,
    pub detail: String,
    pub backups: Vec<PathBuf>,
    pub diagnostic: DiagnosticItem,
}
/// Native home/codexmate/relay/state.json is proven by CodexPaths::from_home
/// @1008efb84 (directory joins lines 116-130 and state filename lines 246-249).
/// Native enabled/no-account flags take precedence; Atelier routing is observed
/// from its persisted intent/effective provider, independently of its listener.
pub(super) fn resolve_mode(ctx: &Context) -> Result<RouterMode> {
    if let Some(native) = super::route_mode::native(ctx)? {
        if native.enabled() {
            return Ok(native);
        }
    }
    let config = inspect::config(ctx)?;
    super::route_mode::atelier(ctx, &config)
}
fn context(ctx: &Context, mode: RouterMode) -> AuthContext {
    AuthContext::new(&ctx.home, mode)
}
fn dependency_paths(c: &AuthContext, id: &str) -> Vec<PathBuf> {
    let mut paths = vec![
        c.codex_home.join("config.toml"),
        c.relay_dir().join("state.json"),
    ];
    match id {
        "auth_integrity" => {
            paths.push(c.auth_path());
            paths.extend(auth::auth_backup_candidates(c));
        }
        "router_unlock_auth_residue" => paths.extend([
            c.auth_path(),
            unlock::marker_path(c),
            unlock::backup_path(c),
            unlock::legacy_marker_path(c),
            unlock::legacy_backup_path(c),
        ]),
        "takeover_backup_orphan" => paths.extend([
            c.relay_dir().join("router-takeover-backup.json"),
            c.relay_dir().join("codex_router_top_level_backup.json"),
        ]),
        _ => {}
    }
    paths
}
fn prepare_internal(
    ctx: &Context,
    id: &str,
    backup_root: &Path,
    mode: RouterMode,
    pin: Option<&PreparedAction>,
) -> Result<PreparedAction> {
    prepare_internal_with_prefix(ctx, id, backup_root, mode, pin, &[])
}
fn prepare_internal_with_prefix(
    ctx: &Context,
    id: &str,
    backup_root: &Path,
    mode: RouterMode,
    pin: Option<&PreparedAction>,
    approved_prefix: &[(PathBuf, EffectStamp, EffectStamp)],
) -> Result<PreparedAction> {
    if !handles(id) {
        return Err("不是已恢复的认证维护项目".into());
    }
    if id == "auth_integrity" {
        let config = inspect::config(ctx)?;
        if non_file_auth(&config) {
            return Err("当前认证存储不是文件模式，不恢复 auth.json".into());
        }
        if mode.enabled() || residual_cleanup::route_enabled(ctx, &config)? {
            return Err("路由启用时不能恢复文件认证".into());
        }
    }
    let c = context(ctx, mode);
    let mut dependencies = BTreeMap::new();
    for path in dependency_paths(&c, id) {
        dependencies.insert(path.clone(), fingerprint(&path)?);
    }
    let intent_path = super::router_intent::path(ctx);
    dependencies.insert(intent_path.clone(), fingerprint(&intent_path)?);
    let mut body = match id {
        "auth_integrity" => {
            let mut plan = auth::plan_auth_integrity(&c).map_err(err)?;
            plan.changes.retain(|c| c.before != c.after);
            Body::Integrity(plan)
        }
        "router_unlock_auth_residue" => {
            Body::Unlock(unlock::plan_router_unlock_auth_residue(&c).map_err(err)?)
        }
        "takeover_backup_orphan" => {
            Body::Takeover(takeover::plan_takeover_backup_orphan(&c).map_err(err)?)
        }
        _ => unreachable!(),
    };
    // Native quarantine names contain a timestamp. Replanning must retain the exact
    // already-reviewed destination for the same damaged source, not approve a new one.
    if let (Some(prior), Body::Takeover(fresh)) = (pin, &mut body) {
        if let Body::Takeover(prior) = prior.body.as_ref() {
            for (source, target) in &mut fresh.quarantine_pairs {
                if let Some((_, approved)) =
                    prior.quarantine_pairs.iter().find(|(p, _)| p == source)
                {
                    let generated = target.clone();
                    for change in &mut fresh.files.changes {
                        if change.path == generated {
                            change.path = approved.clone();
                            change.before = prior
                                .files
                                .changes
                                .iter()
                                .find(|c| c.path == *approved)
                                .and_then(|c| c.before.clone());
                            // Only a verified pre-approved copy may already occupy the pinned
                            // destination during partial recovery. Normal prepare has no prefix.
                            if approved_prefix.iter().any(|(path, before, after)| {
                                path == approved
                                    && before.is_none()
                                    && *after
                                        == bytes_fingerprint(&change.after)
                                            .map(|f| (f.len, f.sha256))
                            }) {
                                change.before = change.after.clone();
                            }
                        }
                    }
                    for path in &mut fresh.report.quarantined_files {
                        if path == &generated {
                            *path = approved.clone()
                        }
                    }
                    *target = approved.clone();
                }
            }
        }
    }
    let mut scope = BTreeSet::new();
    for o in &body.files().observations {
        let actual = fingerprint(&o.path)?;
        if actual != bytes_fingerprint(&o.bytes) {
            return Err("认证恢复依赖在预览期间变化，请重试".into());
        }
        dependencies.insert(o.path.clone(), actual);
    }
    for change in &body.files().changes {
        let actual = fingerprint(&change.path)?;
        if actual != bytes_fingerprint(&change.before) {
            return Err("认证恢复目标在预览期间变化，请重试".into());
        }
        dependencies.insert(change.path.clone(), actual);
        scope.insert(change.path.clone());
    }
    for (path, before) in &dependencies {
        if fingerprint(path)? != *before {
            return Err("认证恢复依赖在预览期间变化，请重试".into());
        }
    }
    Ok(PreparedAction {
        id: id.into(),
        home: ctx.home.clone(),
        app_root: ctx.app_root.clone(),
        backup_root: backup_root.into(),
        mode,
        dependencies,
        scope,
        body: Arc::new(body),
    })
}
/// Mode must come from the shared effective/native-state resolver, not backup contents.
pub(super) fn prepare(
    ctx: &Context,
    id: &str,
    backup_root: &Path,
    mode: RouterMode,
) -> Result<PreparedAction> {
    prepare_internal(ctx, id, backup_root, mode, None)
}
pub(super) fn prepare_with_backup(
    ctx: &Context,
    id: &str,
    backup_root: &Path,
    mode: RouterMode,
) -> Result<PreparedAction> {
    prepare(ctx, id, backup_root, mode)
}
pub(super) fn preview_lines(p: &PreparedAction) -> Vec<String> {
    let files = p.body.files();
    let writes = files.changes.iter().filter(|c| c.after.is_some()).count();
    let deletes = files.changes.len() - writes;
    let description = match p.body.as_ref() {
        Body::Integrity(_) => {
            "从 accounts/backups 按原生候选排序恢复认证，原件逐字私有备份，不展示凭据"
        }
        Body::Unlock(_) => {
            "按原生认证标记恢复真实用户凭据，保留当前用户登录，并清理受管虚拟认证及旧版残留"
        }
        Body::Takeover(_) => {
            "按原生接管备份恢复缺失的原始配置，保留当前已有键和重叠区段；损坏备份先隔离验证后移除"
        }
    };
    let mut lines=vec![format!("{description}；写入 {writes} 项，移除 {deletes} 项。"),"确认后先保存私有原件备份；认证清理使用 accounts/.live-auth.lock，凭据不会出现在预览或结果中。".into()];
    if let Body::Takeover(plan) = p.body.as_ref() {
        lines.push(format!(
            "恢复顶层配置 {} 项、配置区段 {} 项；保留冲突 {} 项；隔离损坏备份 {} 项。",
            plan.report.restored_top_level_keys.len(),
            plan.report.restored_sections.len(),
            plan.report.skipped_top_level_keys.len() + plan.report.skipped_sections.len(),
            plan.quarantine_pairs.len()
        ));
    }
    lines
}
fn non_file_auth(config: &toml::Value) -> bool {
    matches!(
        config
            .get("cli_auth_credentials_store")
            .and_then(|v| v.as_str()),
        Some("keyring" | "ephemeral")
    )
}
fn integrity_diagnostic(ctx: &Context, mode: RouterMode) -> DiagnosticItem {
    let config = match inspect::config(ctx) {
        Ok(c) => c,
        Err(_) => {
            return DiagnosticItem {
                id: "auth_integrity".into(),
                group: "auth".into(),
                status: "warning".into(),
                label: "文件认证完整性".into(),
                detail: "配置未通过解析，未恢复认证。".into(),
                fixable: false,
            }
        }
    };
    if non_file_auth(&config) {
        return DiagnosticItem {
            id: "auth_integrity".into(),
            group: "auth".into(),
            status: "ok".into(),
            label: "文件认证完整性".into(),
            detail: "当前认证存储不是文件模式；不读取或恢复 auth.json，不访问钥匙串。".into(),
            fixable: false,
        };
    }
    let check = auth::check_auth_integrity(&context(ctx, mode));
    DiagnosticItem {
        id: "auth_integrity".into(),
        group: "auth".into(),
        status: match check.level {
            Level::Ok => "ok",
            Level::Warning => "warning",
            Level::Error => "error",
        }
        .into(),
        label: check.label().into(),
        detail:
            "本地文件检查不验证在线登录、签名或额度；按 accounts/backups 原生候选恢复，不展示凭据。"
                .into(),
        fixable: check.fixable
            && prepare(
                ctx,
                "auth_integrity",
                &ctx.app_root.join("maintenance/auth-repairs"),
                mode,
            )
            .is_ok_and(|p| !p.body.files().changes.is_empty()),
    }
}
fn diagnostic(ctx: &Context, id: &str, mode: RouterMode) -> DiagnosticItem {
    if id == "auth_integrity" {
        return integrity_diagnostic(ctx, mode);
    }
    let c = context(ctx, mode);
    let level = match id {
        "router_unlock_auth_residue" => unlock::check_router_unlock_auth_residue(&c),
        _ => takeover::check_takeover_backup_orphan(&c),
    };
    let (group, label) = if id == "router_unlock_auth_residue" {
        ("auth", "无虚拟账号残留")
    } else {
        ("config", "无 takeover 备份残留")
    };
    let(status,detail)=match(id,level){(_,Level::Ok)=>("ok","原生残留检查通过。"),("router_unlock_auth_residue",_)=>("error","发现原生路由认证标记、认证备份或受管虚拟认证；恢复将按当前登录状态与备份内容逐项判断。"),_=>("warning","路由关闭后仍有原生配置接管备份；恢复会保留当前冲突值，损坏备份需隔离。")};
    let fixable = level != Level::Ok
        && prepare(
            ctx,
            id,
            &ctx.app_root.join("maintenance/auth-repairs"),
            mode,
        )
        .is_ok_and(|p| !p.body.files().changes.is_empty());
    let detail = if level != Level::Ok && mode == RouterMode::NoAccount {
        "无账号路由启用时保留认证和接管备份；请先关闭该模式。"
    } else {
        detail
    };
    DiagnosticItem {
        id: id.into(),
        group: group.into(),
        status: status.into(),
        label: label.into(),
        detail: detail.into(),
        fixable,
    }
}
pub(super) fn inspect(ctx: &Context, mode: RouterMode) -> Vec<DiagnosticItem> {
    IDS.iter().map(|id| diagnostic(ctx, id, mode)).collect()
}
pub(super) fn validate(ctx: &Context, p: &PreparedAction, mode: RouterMode) -> Result<()> {
    if ctx.home != p.home || ctx.app_root != p.app_root || mode != p.mode {
        return Err("认证维护上下文或路由模式已变化，请重新预览".into());
    }
    validate_candidate_membership(ctx, p)?;
    validate_dependencies(&p.dependencies)
}
fn validate_candidate_membership(ctx: &Context, p: &PreparedAction) -> Result<()> {
    if p.id != "auth_integrity" {
        return Ok(());
    }
    let c = context(ctx, p.mode);
    let dir = c.auth_backup_dir();
    let before = p
        .dependencies
        .keys()
        .filter(|path| {
            path.parent() == Some(dir.as_path()) && path.extension().is_some_and(|e| e == "json")
        })
        .cloned()
        .collect::<BTreeSet<_>>();
    let now = auth::auth_backup_candidates(&c)
        .into_iter()
        .collect::<BTreeSet<_>>();
    if before != now {
        return Err("认证备份候选集合已变化，请重新预览".into());
    }
    Ok(())
}
fn validate_dependencies(dependencies: &BTreeMap<PathBuf, Option<Fingerprint>>) -> Result<()> {
    for (path, expected) in dependencies {
        if fingerprint(path)? != *expected {
            return Err("认证、接管备份或配置已被外部更新，请重新预览".into());
        }
    }
    Ok(())
}
fn signature(body: &Body) -> BTreeMap<PathBuf, (Option<Fingerprint>, Option<Fingerprint>)> {
    body.files()
        .changes
        .iter()
        .map(|c| {
            (
                c.path.clone(),
                (bytes_fingerprint(&c.before), bytes_fingerprint(&c.after)),
            )
        })
        .collect()
}
fn save_backups(p: &PreparedAction) -> Result<Vec<PathBuf>> {
    if p.body.files().changes.is_empty() {
        return Ok(vec![]);
    }
    let root = p
        .backup_root
        .join(format!("{}-{}", p.id, uuid::Uuid::new_v4()));
    private_dir(&root)?;
    let mut backups = Vec::new();
    let mut manifest = Vec::new();
    for (i, c) in p.body.files().changes.iter().enumerate() {
        let backup = if let Some(bytes) = &c.before {
            let path = root.join(format!("{i:04}.bin"));
            atomic_private(&path, bytes)?;
            if fingerprint(&path)? != bytes_fingerprint(&c.before) {
                return Err("认证原件备份验证失败，未执行修复".into());
            }
            backups.push(path.clone());
            Some(path.file_name().unwrap().to_string_lossy().to_string())
        } else {
            None
        };
        manifest.push(serde_json::json!({"path":c.path,"backup":backup,"operation":if c.after.is_none(){"delete"}else if c.before.is_none(){"create"}else{"replace"}}));
    }
    atomic_private(
        &root.join("manifest.json"),
        &serde_json::to_vec_pretty(&serde_json::json!({"itemId":p.id,"files":manifest}))
            .map_err(err)?,
    )?;
    Ok(backups)
}
fn execute(ctx: &Context, p: &PreparedAction, mode: RouterMode) -> Result<Outcome> {
    let backups = save_backups(p)?;
    let c = context(ctx, mode);
    let applied = match p.body.as_ref() {
        Body::Integrity(plan) => auth::apply_auth_integrity(&c, plan).map(|check| check.level),
        Body::Unlock(plan) => unlock::apply_router_unlock_auth_residue(&c, plan),
        Body::Takeover(plan) => takeover::apply_takeover_backup_orphan(&c, plan),
    };
    let check = diagnostic(ctx, &p.id, mode);
    let effects_match =
        p.body.files().changes.iter().all(|c| {
            fingerprint(&c.path).is_ok_and(|actual| actual == bytes_fingerprint(&c.after))
        });
    let success = applied.as_ref().is_ok_and(|level| *level == Level::Ok)
        && check.status == "ok"
        && effects_match;
    let changed = p
        .body
        .files()
        .changes
        .iter()
        .filter(|change| {
            fingerprint(&change.path)
                .is_ok_and(|actual| actual != bytes_fingerprint(&change.before))
        })
        .count();
    Ok(Outcome {
        id: p.id.clone(),
        success,
        changed,
        detail: if success {
            "原生恢复及实际残留复检通过；私有原件备份已保留。".into()
        } else if applied.is_err() {
            "原生恢复返回错误；已保留原件备份并实际复检，未标为成功。".into()
        } else {
            "原生恢复后字节复检或残留检查未通过，未标为成功；请保留备份并重新检测。".into()
        },
        backups,
        diagnostic: check,
    })
}
pub(super) fn apply(ctx: &Context, p: &PreparedAction, mode: RouterMode) -> Result<Outcome> {
    validate(ctx, p, mode)?;
    let fresh = prepare_internal(ctx, &p.id, &p.backup_root, mode, Some(p))?;
    if !fresh
        .dependencies
        .keys()
        .all(|path| p.dependencies.contains_key(path))
        || signature(fresh.body.as_ref()) != signature(p.body.as_ref())
    {
        return Err("认证恢复的已批准效果已变化，请重新预览".into());
    }
    execute(ctx, &fresh, mode)
}
/// Native fix_all @1003a69d8 dispatches Integrity, then Unlock, then Takeover.
pub(super) fn prepare_sequence(
    ctx: &Context,
    ids: &[String],
    backup_root: &Path,
    mode: RouterMode,
) -> Result<Vec<PreparedAction>> {
    IDS.iter()
        .filter(|id| ids.iter().any(|i| i == **id))
        .map(|id| prepare(ctx, id, backup_root, mode))
        .collect()
}
/// Validate every original input before the first mutation. Each fresh native
/// plan is restricted to the reviewed union, and later dependencies reflect only
/// actual earlier approved writes. Unapproved newly-created targets are refused.
pub(super) fn apply_sequence(
    ctx: &Context,
    plans: &[PreparedAction],
    mode: RouterMode,
) -> Result<Vec<Outcome>> {
    let mut approved = BTreeSet::new();
    let mut expected = BTreeMap::new();
    for p in plans {
        validate(ctx, p, mode)?;
        approved.extend(p.scope.iter().cloned());
        for (path, stamp) in &p.dependencies {
            if expected.get(path).is_some_and(|prior| prior != stamp) {
                return Err("串联认证计划的初始状态不一致".into());
            }
            expected.insert(path.clone(), stamp.clone());
        }
    }
    let mut outcomes = Vec::new();
    for p in plans {
        validate_dependencies(&expected)?;
        let fresh = prepare_internal(ctx, &p.id, &p.backup_root, mode, Some(p))?;
        let approved_after = approved_effects(p).into_iter().collect::<BTreeMap<_, _>>();
        if !fresh
            .dependencies
            .keys()
            .all(|path| p.dependencies.contains_key(path))
            || !fresh.scope.is_subset(&approved)
            || approved_effects(&fresh)
                .iter()
                .any(|(path, after)| approved_after.get(path) != Some(after))
        {
            return Err(
                "串联恢复产生未经审核的新来源、目标或效果，请重新预览；此前备份与结果已保留".into(),
            );
        }
        let outcome = execute(ctx, &fresh, mode)?;
        for change in &fresh.body.files().changes {
            expected.insert(change.path.clone(), bytes_fingerprint(&change.after));
        }
        let stop = !outcome.success;
        outcomes.push(outcome);
        if stop {
            break;
        }
    }
    for out in &mut outcomes {
        out.diagnostic = diagnostic(ctx, &out.id, mode);
        out.success &= out.diagnostic.status == "ok";
    }
    Ok(outcomes)
}

/// Internal coordinator projection only; never serialize config/auth preimages into UI.
pub(super) fn planned_config(
    p: &PreparedAction,
) -> Option<(PathBuf, Option<Vec<u8>>, Option<Vec<u8>>)> {
    let config = p.home.join("config.toml");
    p.body
        .files()
        .changes
        .iter()
        .find(|c| c.path == config)
        .map(|c| (c.path.clone(), c.before.clone(), c.after.clone()))
}
pub(super) fn approved_paths(p: &PreparedAction) -> BTreeSet<PathBuf> {
    p.scope.clone()
}
/// Read-only path + length/hash projection; contains no credential bytes.
pub(super) fn approved_dependencies(p: &PreparedAction) -> Vec<(PathBuf, Option<(usize, String)>)> {
    p.dependencies
        .iter()
        .map(|(path, f)| (path.clone(), f.as_ref().map(|f| (f.len, f.sha256.clone()))))
        .collect()
}
/// The shared coordinator has already backed up and published its reviewed,
/// combined config bytes. Only that exact before/after difference is accepted.
/// Recompute native actions against the new config, consume their config effect
/// here, and retain all native backup/lock/fsync side effects and postchecks.
pub(super) fn apply_after_approved_config(
    ctx: &Context,
    p: &PreparedAction,
    mode: RouterMode,
    approved_before: Option<&[u8]>,
    approved_after: Option<&[u8]>,
) -> Result<Outcome> {
    apply_after_approved_effects(ctx, p, mode, approved_before, approved_after, &[])
        .map(|(outcome, _)| outcome)
}
pub(super) fn item_id(p: &PreparedAction) -> &str {
    &p.id
}
pub(super) type EffectStamp = Option<(usize, String)>;
fn stamp_fingerprint(stamp: &EffectStamp) -> Option<Fingerprint> {
    stamp.as_ref().map(|(len, sha256)| Fingerprint {
        len: *len,
        sha256: sha256.clone(),
    })
}
/// Prefix entries are original-before -> approved-after, never arbitrary observed
/// current bytes. Fresh native actions may shrink, but cannot add dependencies,
/// targets or different after bytes beyond this action's original approval.
pub(super) fn apply_after_approved_effects(
    ctx: &Context,
    p: &PreparedAction,
    mode: RouterMode,
    approved_before: Option<&[u8]>,
    approved_after: Option<&[u8]>,
    prefix: &[(PathBuf, EffectStamp, EffectStamp)],
) -> Result<(Outcome, Vec<(PathBuf, EffectStamp)>)> {
    let fresh =
        replan_after_approved_effects(ctx, p, mode, approved_before, approved_after, prefix)?;
    let effects = approved_effects(&fresh);
    let outcome = execute(ctx, &fresh, mode)?;
    Ok((outcome, effects))
}
/// Build the same pinned fresh action without executing it. Non-config sources,
/// targets and after bytes remain restricted to the original approved plan.
pub(super) fn replan_after_approved_effects(
    ctx: &Context,
    p: &PreparedAction,
    mode: RouterMode,
    approved_before: Option<&[u8]>,
    approved_after: Option<&[u8]>,
    prefix: &[(PathBuf, EffectStamp, EffectStamp)],
) -> Result<PreparedAction> {
    if ctx.home != p.home || ctx.app_root != p.app_root || mode != p.mode {
        return Err("认证维护上下文或路由模式已变化，请重新预览".into());
    }
    let config = ctx.config();
    let before = approved_before.map(|b| b.to_vec());
    let after = approved_after.map(|b| b.to_vec());
    if p.dependencies.get(&config) != Some(&bytes_fingerprint(&before))
        || fingerprint(&config)? != bytes_fingerprint(&after)
    {
        return Err("组合配置的预审前后字节与认证计划不符，未执行后续恢复".into());
    }
    let mut seen = BTreeSet::new();
    for (path, _, _) in prefix {
        if !seen.insert(path) {
            return Err("已批准前序效果路径重复".into());
        }
    }
    for (path, initial) in &p.dependencies {
        if path == &config {
            continue;
        }
        let expected = if let Some((_, before, after)) = prefix.iter().find(|(p, _, _)| p == path) {
            if stamp_fingerprint(before) != *initial {
                return Err("前序效果的初始依赖与认证预审不符".into());
            }
            stamp_fingerprint(after)
        } else {
            initial.clone()
        };
        if fingerprint(path)? != expected {
            return Err("已批准前序效果之外的认证依赖已变化".into());
        }
    }
    let mut fresh =
        prepare_internal_with_prefix(ctx, &p.id, &p.backup_root, mode, Some(p), prefix)?;
    if !fresh
        .dependencies
        .keys()
        .all(|path| p.dependencies.contains_key(path))
    {
        return Err("认证恢复出现未经审核的新来源或依赖，请重新预览".into());
    }
    let body = Arc::get_mut(&mut fresh.body).ok_or("无法建立独占认证恢复计划")?;
    let files = match body {
        Body::Integrity(p) | Body::Unlock(p) => p,
        Body::Takeover(p) => &mut p.files,
    };
    files.changes.retain(|c| c.path != config);
    fresh.scope.remove(&config);
    let approved = approved_effects(p).into_iter().collect::<BTreeMap<_, _>>();
    let effects = approved_effects(&fresh);
    if !fresh.scope.is_subset(&p.scope)
        || effects
            .iter()
            .any(|(path, after)| approved.get(path) != Some(after))
    {
        return Err("前序修复后的认证恢复效果超出原批准范围".into());
    }
    Ok(fresh)
}

/// Exact approved post-effect fingerprints for the shared coordinator ledger.
pub(super) fn approved_effects(p: &PreparedAction) -> Vec<(PathBuf, Option<(usize, String)>)> {
    p.body
        .files()
        .changes
        .iter()
        .map(|c| {
            (
                c.path.clone(),
                bytes_fingerprint(&c.after).map(|f| (f.len, f.sha256)),
            )
        })
        .collect()
}
