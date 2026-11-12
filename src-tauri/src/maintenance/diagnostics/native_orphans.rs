//! Native orphan adoption. An absent foreign definition is never proof of ownership.
use super::*;
use atelier_maintenance_core::{
    catalog::state::RelayStateDocument,
    state_db,
    threads::{self as core, consistency},
};
use sha2::{Digest, Sha256};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    io::Read,
};
#[cfg(test)]
#[path = "native_orphans_tests.rs"]
mod tests;

pub(super) const IDS: [&str; 2] = ["db_orphan_providers", "rollout_orphan_providers"];
pub(super) fn handles(id: &str) -> bool {
    IDS.contains(&id)
}
fn managed(id: &str) -> bool {
    core::is_aimami_provider(id) || super::router_intent::owns_provider(id)
}
fn adoptable(id: &str) -> bool {
    managed(id) || id.eq_ignore_ascii_case("openai")
}
#[derive(Clone, PartialEq, Eq)]
struct Stamp {
    length: u64,
    sha256: String,
    modified: Option<std::time::SystemTime>,
}
fn stamp(path: &Path) -> Result<Option<Stamp>> {
    let m = match fs::symlink_metadata(path) {
        Ok(m) => m,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(None),
        Err(e) => return Err(err(e)),
    };
    if !m.is_file() || m.file_type().is_symlink() {
        return Err(format!("孤儿修复依赖不是普通文件：{}", path.display()));
    }
    let mut f = fs::File::open(path).map_err(err)?;
    let mut digest = Sha256::new();
    let mut buffer = [0u8; 65536];
    let mut length = 0;
    loop {
        let n = f.read(&mut buffer).map_err(err)?;
        if n == 0 {
            break;
        }
        digest.update(&buffer[..n]);
        length += n as u64;
    }
    let after = fs::symlink_metadata(path).map_err(err)?;
    if m.len() != length || m.len() != after.len() || m.modified().ok() != after.modified().ok() {
        return Err("孤儿修复依赖读取期间变化".into());
    }
    Ok(Some(Stamp {
        length,
        sha256: format!("{:x}", digest.finalize()),
        modified: after.modified().ok(),
    }))
}
fn optional(path: &Path, limit: u64) -> Result<Option<Vec<u8>>> {
    match fs::symlink_metadata(path) {
        Ok(_) => read_bounded(path, limit).map(Some),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(None),
        Err(e) => Err(err(e)),
    }
}
#[derive(Clone)]
pub(super) struct Selection {
    pub rows: Vec<core::MainThread>,
    pub db_ids: BTreeSet<String>,
    pub rollout_ids: BTreeSet<String>,
    databases: Vec<PathBuf>,
    dependencies: BTreeMap<PathBuf, Option<Stamp>>,
}
fn databases(ctx: &Context) -> Result<Vec<PathBuf>> {
    let raw = optional(&ctx.config(), 1024 * 1024)?
        .map(|b| String::from_utf8(b).map_err(err))
        .transpose()?;
    let configured = state_db::configured_sqlite_home(
        raw.as_deref(),
        ctx.sqlite_home_env.as_deref(),
        ctx.working_directory.as_deref(),
    );
    let dirs = configured
        .map(|p| vec![p])
        .unwrap_or_else(|| vec![ctx.home.join("sqlite"), ctx.home.clone()]);
    let mut dbs = vec![];
    for dir in dirs {
        if let Some(db) = state_db::highest_state_db(&dir) {
            if !dbs.contains(&db) {
                dbs.push(db);
            }
        }
    }
    Ok(dbs)
}
fn absolute(ctx: &Context, p: &str) -> PathBuf {
    let p = PathBuf::from(p);
    if p.is_absolute() {
        p
    } else {
        ctx.home.join(p)
    }
}
fn valid_ids(
    ctx: &Context,
    deps: &mut BTreeMap<PathBuf, Option<Stamp>>,
) -> Result<BTreeSet<String>> {
    let state_path = ctx.home.join("codexmate/relay/state.json");
    deps.insert(state_path.clone(), stamp(&state_path)?);
    let mut base = BTreeSet::new();
    let mut enabled = false;
    if let Some(bytes) = optional(&state_path, 8 * 1024 * 1024)? {
        let d = RelayStateDocument::from_json(&bytes).map_err(err)?;
        let i = d.catalog_inputs().map_err(err)?;
        enabled = i.router_enabled;
        base.extend(i.providers.into_iter().map(|p| p.id));
    }
    let intent_path = super::router_intent::path(ctx);
    deps.insert(intent_path.clone(), stamp(&intent_path)?);
    if let Some(intent) = super::router_intent::read(ctx)? {
        if intent.target == atelier_maintenance_core::transition_journal::TransitionTarget::RouterOn
        {
            if let Some(provider) = intent.provider {
                base.insert(provider);
            }
        }
    }
    // Native quarantine-source composition is supplied by the actual takeover decoder.
    let quarantine_path = ctx.home.join("codexmate/relay/router-takeover-backup.json");
    deps.insert(quarantine_path.clone(), stamp(&quarantine_path)?);
    let quarantined = quarantine_ids(ctx);
    Ok(core::diagnostic_valid_provider_ids_from_sources(
        &base,
        enabled,
        quarantined.as_ref().map_err(|_| "quarantine unavailable"),
        &ctx.config(),
    ))
}
fn quarantine_ids(ctx: &Context) -> Result<BTreeSet<String>> {
    let path = ctx.home.join("codexmate/relay/router-takeover-backup.json");
    let Some(bytes) = optional(&path, 8 * 1024 * 1024)? else {
        return Ok(BTreeSet::new());
    };
    let backup: atelier_maintenance_core::auth::takeover::TakeoverBackup =
        serde_json::from_slice(&bytes).map_err(err)?;
    if backup.version != 1 {
        return Err("unsupported takeover backup".into());
    }
    let mut ids = BTreeSet::new();
    for block in backup.section_blocks {
        if let Some(id) = block.key.strip_prefix("model_providers.") {
            let id = atelier_maintenance_core::config::first_section_part(id);
            if !id.is_empty() && !core::is_aimami_provider(&id) {
                ids.insert(id);
            }
        }
    }
    Ok(ids)
}
/// Both checks independently select rows; adoption operates on their union, never every row.
/// Native selection @10039ff14..1003a03c8; shared-history ownership @1003a0e00.
pub(super) fn select(ctx: &Context) -> Result<Selection> {
    select_inner(ctx, true)
}
fn select_inner(ctx: &Context, reviewed: bool) -> Result<Selection> {
    let databases = databases(ctx)?;
    let mut dependencies = BTreeMap::new();
    if reviewed {
        dependencies.insert(ctx.config(), stamp(&ctx.config())?);
        dependencies.insert(
            ctx.home.join("auth.json"),
            stamp(&ctx.home.join("auth.json"))?,
        );
    }
    let valid = valid_ids(ctx, &mut dependencies)?;
    let official_alias = optional(&ctx.config(), 1024 * 1024)?
        .and_then(|b| String::from_utf8(b).ok())
        .and_then(|text| atelier_maintenance_core::router_verification::native_off_provider_target(&text).ok());
    let adoptable = |id: &str| adoptable(id) || official_alias.as_deref() == Some(id);

    let mut all = vec![];
    for db in &databases {
        if reviewed {
            for p in [db.clone(), PathBuf::from(format!("{}-wal", db.display()))] {
                dependencies.insert(p.clone(), stamp(&p)?);
            }
        }
        let mut rows = super::snapshot::with_observed_path(db, |copy, _| {
            core::collect_diagnostic_main_threads(&[copy.to_path_buf()])
        })?;
        for r in &mut rows {
            r.db = db.clone();
            if let Some(p) = r.rollout_path.as_mut() {
                *p = absolute(ctx, p).to_string_lossy().into_owned();
            }
        }
        all.extend(rows);
    }
    let orphan = |p: &str| managed(p) && !valid.contains(p);
    let mut db_ids = BTreeSet::new();
    let mut rollout_ids = BTreeSet::new();
    let mut groups: BTreeMap<PathBuf, Vec<usize>> = BTreeMap::new();
    for (i, row) in all.iter().enumerate() {
        if row.provider.as_deref().is_some_and(&orphan) {
            db_ids.insert(row.id.clone());
        }
        if let Some(p) = row.rollout_path.as_ref() {
            let requested = PathBuf::from(p);
            let physical = core::resolve_rollout(&requested)
                .ok()
                .flatten()
                .unwrap_or(requested);
            groups.entry(physical).or_default().push(i);
        } else if db_ids.contains(&row.id) {
            return Err(format!("orphan thread {} has no rollout path", row.id));
        }
    }
    let mut selected = BTreeSet::new();
    for (requested, indices) in &groups {
        let plain = if requested.to_string_lossy().ends_with(".jsonl.zst") {
            requested.with_extension("")
        } else {
            requested.clone()
        };
        if reviewed {
            for p in [plain.clone(), plain.with_extension("jsonl.zst")] {
                dependencies.insert(p.clone(), stamp(&p)?);
            }
        }
        let anchors = consistency::read_rollout_provider_anchors(requested);
        let db_orphan = indices.iter().any(|&i| db_ids.contains(&all[i].id));
        let anchor_orphan = anchors
                .as_ref()
                .ok()
                .and_then(|v| v.as_ref())
                .is_some_and(|a| {
                    [&a.first, &a.last, &a.native]
                        .iter()
                        .any(|p| p.as_deref().is_some_and(&orphan))
                });
        if !db_orphan && !anchor_orphan {
            continue;
        }
        let a = anchors
            .map_err(|e| format!("孤儿会话元数据读取失败：{e:?}"))?
            .ok_or_else(|| format!("orphan thread rollout is missing: {}", requested.display()))?;
        if indices
            .iter()
            .any(|&i| !all[i].provider.as_deref().is_some_and(adoptable))
            || [&a.first, &a.last, &a.native]
                .iter()
                .any(|p| p.as_deref().is_some_and(|p| !adoptable(p)))
        {
            return Err(format!(
                "orphan thread points to rollout owned by external provider: {}",
                requested.display()
            ));
        }
        let ids = indices
            .iter()
            .map(|&i| all[i].id.as_str())
            .collect::<BTreeSet<_>>();
        if ids.len() != 1 {
            return Err(format!(
                "multiple thread ids reference rollout: {}",
                requested.display()
            ));
        }
        for &i in indices {
            if anchor_orphan {
                rollout_ids.insert(all[i].id.clone());
            }
            selected.insert(i);
        }
    }
    let rows = selected.into_iter().map(|i| all[i].clone()).collect();
    let result = Selection {
        rows,
        db_ids,
        rollout_ids,
        databases,
        dependencies,
    };
    validate(ctx, &result)?;
    Ok(result)
}
pub(super) fn validate(ctx: &Context, p: &Selection) -> Result<()> {
    if databases(ctx)? != p.databases {
        return Err("孤儿修复数据库集合在预览后变化".into());
    }
    for (path, expected) in &p.dependencies {
        if &stamp(path)? == expected {
            continue;
        }
        // Codex 关闭时会把 WAL 合并进主库（checkpoint），因此主库、WAL、SHM
        // 在预览→apply 之间变化属预期。内容一致性由后续逐行校验与 digest 复检保证。
        let name = path.file_name().and_then(|n| n.to_str()).unwrap_or_default();
        if name.ends_with(".sqlite") || name.ends_with("-wal") || name.ends_with("-shm") {
            continue;
        }
        return Err(format!("孤儿修复依赖在预览后变化：{}", path.display()));
    }
    Ok(())
}
pub(super) fn inspect(ctx: &Context) -> Vec<DiagnosticItem> {
    match select_inner(ctx, false) {
        Ok(p) => IDS.into_iter().map(|id| {
            let count=if id==IDS[0] { p.db_ids.len() } else { p.rollout_ids.len() };
            DiagnosticItem { id:id.into(),group:"runtime".into(),label:(if count==0 {if id==IDS[0] {"未发现失效的旧版工具线程 provider".to_string()} else {format!("未发现失效的旧版工具 Rollout provider（已检查 {count} 个）")}} else {if id==IDS[0] {format!("{count} 条主线程引用了失效的旧版工具 provider")} else {format!("{count} 个 Rollout 引用了失效的旧版工具 provider")}}),status:if count==0 {"ok"} else {"error"}.into(),detail:if id==IDS[0] {"仅核对旧版工具管理的引用；其他原生、企业或自定义 provider 保持原样，不由本地定义缺失推断无效。".into()} else {format!("这些主线程在 Codex 中可能无法正常对话。修复会同步接管 DB 与 Rollout，保留原模型与推理档位；模型无法确证时拒绝接管。仅修复旧版工具残留；混合外部归属时停止，未确认的第三方任务不接管。示例：{count} 个")},fixable:count>0 }
        }).collect(),
        Err(e) => IDS.into_iter().map(|id|DiagnosticItem { id:id.into(),group:"runtime".into(),label:"无法完整读取线程数据库".into(),status:"warning".into(),detail:e.clone(),fixable:false }).collect(),
    }
}
#[path = "native_orphans/catalog.rs"]
mod catalog;
pub(super) use catalog::{OfficialCatalog, Rpc as CatalogRpc};
pub(super) fn prepare_catalog_with_rpc(
    ctx: &Context,
    rpc: &mut impl CatalogRpc,
) -> Result<OfficialCatalog> {
    catalog::prepare_with(ctx, rpc)
}
pub(super) fn prepare_with_catalog(ctx: &Context, catalog: &OfficialCatalog) -> Result<Prepared> {
    build(ctx, select(ctx)?, catalog)
}
pub(super) fn prepare_catalog(ctx: &Context) -> Result<OfficialCatalog> {
    catalog::prepare(ctx)
}
#[derive(Clone)]
struct RowEffect {
    before: core::MainThread,
    provider: String,
    model: String,
    effort: Option<String>,
}
#[derive(Clone)]
struct FileEffect {
    path: PathBuf,
    before: Vec<u8>,
    after: Vec<u8>,
}
#[derive(Clone)]
pub(super) struct Prepared {
    selection: Selection,
    rows: Vec<RowEffect>,
    files: Vec<FileEffect>,
    catalog: OfficialCatalog,
    expected_databases: BTreeMap<PathBuf, String>,
    original_databases: BTreeMap<PathBuf, String>,
}
impl Prepared {
    pub(super) fn official_catalog(&self) -> &OfficialCatalog {
        &self.catalog
    }
}
fn decode(path: &Path) -> Result<Vec<u8>> {
    let bytes = read_bounded(path, inspect::LIMIT)?;
    if path.to_string_lossy().ends_with(".jsonl.zst") {
        let decoder = zstd::stream::read::Decoder::new(bytes.as_slice()).map_err(err)?;
        let mut out = vec![];
        decoder
            .take(inspect::LIMIT + 1)
            .read_to_end(&mut out)
            .map_err(err)?;
        if out.len() as u64 > inspect::LIMIT {
            return Err("孤儿历史解压后超过维护上限".into());
        }
        Ok(out)
    } else {
        Ok(bytes)
    }
}
#[derive(Clone)]
struct Record {
    begin: usize,
    end: usize,
    value: serde_json::Value,
}
fn records(bytes: &[u8]) -> Vec<Record> {
    let mut result = vec![];
    let mut offset = 0;
    for line in bytes.split_inclusive(|b| *b == b'\n') {
        let content = line.strip_suffix(b"\n").unwrap_or(line);
        if let Ok(value) = serde_json::from_slice(content) {
            result.push(Record {
                begin: offset,
                end: offset + content.len(),
                value,
            });
        }
        offset += line.len();
    }
    result
}
fn selected_records(records: &[Record], id: &str) -> Result<(usize, usize, Option<usize>)> {
    let is_meta = |v: &serde_json::Value| v["type"] == "session_meta";
    let first = records
        .iter()
        .position(|r| is_meta(&r.value))
        .ok_or("missing session_meta")?;
    if records[first].value["payload"]["id"].as_str() != Some(id) {
        return Err("孤儿会话身份不匹配，未接管".into());
    }
    let mut last = first;
    let mut settings = None;
    for (i, r) in records.iter().enumerate().skip(first + 1) {
        if is_meta(&r.value) && r.value["payload"]["id"].as_str() == Some(id) {
            last = i;
            settings = None;
        } else if r.value["type"] == "event_msg" && r.value["payload"]["type"] == "thread_settings_applied"
        {
            settings = Some(i);
        }
    }
    if let Some(i) = settings {
        if records[i].value["payload"]["thread_id"]
            .as_str()
            .is_some_and(|s| s != id)
        {
            return Err("latest native settings belongs to another task; history preserved".into());
        }
    }
    Ok((first, last, settings))
}
pub(super) fn prepare(ctx: &Context) -> Result<Prepared> {
    let selection = select(ctx)?;
    if selection.rows.is_empty() {
        return Err("当前没有真实孤儿候选；未将健康 Provider 对齐到当前 Provider".into());
    }
    let official = catalog::prepare(ctx)?;
    build(ctx, selection, &official)
}
fn build(
    ctx: &Context,
    selection: Selection,
    official: &catalog::OfficialCatalog,
) -> Result<Prepared> {
    for (path, before) in &official.dependencies {
        if &stamp(path)? != before {
            return Err("官方目录准备来源在预览后变化".into());
        }
    }
    validate(ctx, &selection)?;
    let mut groups: BTreeMap<PathBuf, Vec<core::MainThread>> = BTreeMap::new();
    for row in &selection.rows {
        let requested = row
            .rollout_path
            .as_ref()
            .ok_or("orphan rollout path missing")?;
        let path = core::resolve_rollout(Path::new(requested))
            .map_err(err)?
            .ok_or("orphan rollout missing")?;
        groups.entry(path).or_default().push(row.clone());
    }
    let mut rows = vec![];
    let mut files = vec![];
    for (path, refs) in groups {
        let id = &refs[0].id;
        let logical = decode(&path)?;
        let rs = records(&logical);
        let (first, last, settings) = selected_records(&rs, id)?;
        let mut models = refs
            .iter()
            .filter_map(|r| r.model.as_deref())
            .collect::<Vec<_>>();
        models.extend(
            [first, last]
                .into_iter()
                .filter_map(|i| rs[i].value["payload"]["model"].as_str()),
        );
        if let Some(i) = settings {
            if let Some(s) = rs[i].value["payload"]["thread_settings"]["model"].as_str() {
                models.push(s);
            }
        }
        let model = models.first().copied().ok_or("原线程模型缺失，无法确证，未修改任何线程")?;
        if models.iter().any(|m| *m != model) || !official.models.contains(model) {
            return Err(format!("线程 {id} 原模型无法由官方目录确证或元数据不一致，拒绝替换模型"));
        }
        let model = model.to_owned();
        let definition = official
            .raw
            .iter()
            .find(|v| v["model"] == model)
            .ok_or("官方模型定义缺失")?;
        let efforts = definition["supportedReasoningEfforts"]
            .as_array()
            .map(|a| {
                a.iter()
                    .filter_map(|v| v.get("reasoningEffort").and_then(serde_json::Value::as_str))
                    .collect::<BTreeSet<_>>()
            })
            .unwrap_or_default();
        let mut preferences = refs
            .iter()
            .filter_map(|r| r.reasoning_effort.as_deref())
            .collect::<Vec<_>>();
        preferences.extend(
            [first, last]
                .into_iter()
                .filter_map(|i| rs[i].value["payload"]["model_reasoning_effort"].as_str()),
        );
        if let Some(i) = settings {
            if let Some(s) = rs[i].value["payload"]["thread_settings"]["reasoning_effort"].as_str()
            {
                preferences.push(s);
            }
        }
        let effort = preferences.first().copied();
        if preferences.iter().any(|s| Some(*s) != effort)
            || effort.is_some_and(|s| !efforts.contains(s))
        {
            return Err(format!("线程 {id} 原推理档位无法确证或元数据不一致，未修改任何线程"));
        }
        let effort = effort.map(str::to_owned);
        let mut patches = BTreeMap::new();
        for i in [Some(first), Some(last), settings].into_iter().flatten() {
            let mut v = rs[i].value.clone();
            if Some(i) == settings {
                v["payload"]["thread_settings"]["model_provider_id"] = official.provider.clone().into();
            } else {
                v["payload"]["model_provider"] = official.provider.clone().into();
            }
            let fitted = core::repair::fit_record(
                &logical[rs[i].begin..rs[i].end],
                serde_json::to_vec(&v).map_err(err)?,
            ).map_err(|e| format!("线程 {id} 历史 {} 无法安全修改：{}", path.display(), e.0))?;
            patches.insert(i, fitted);
        }
        let mut after = vec![];
        let mut offset = 0;
        for (i, patch) in patches {
            after.extend_from_slice(&logical[offset..rs[i].begin]);
            after.extend(patch);
            offset = rs[i].end;
        }
        after.extend_from_slice(&logical[offset..]);
        if path.to_string_lossy().ends_with(".jsonl.zst") {
            after = zstd::stream::encode_all(after.as_slice(), 3).map_err(err)?;
        }
        let before = read_bounded(&path, inspect::LIMIT)?;
        files.push(FileEffect { path, before, after });
        rows.extend(refs.into_iter().map(|before| RowEffect {
            provider: official.provider.clone(),
            before,
            model: model.clone(),
            effort: effort.clone(),
        }));
    }
    validate(ctx, &selection)?;
    let mut expected_databases = BTreeMap::new();
    let mut original_databases = BTreeMap::new();
    for db in rows
        .iter()
        .map(|r| r.before.db.clone())
        .collect::<BTreeSet<_>>()
    {
        let expected = super::snapshot::with_snapshot_path(&db, |_, conn| {
            conn.execute_batch("BEGIN IMMEDIATE").map_err(err)?;
            let original = logical_database_digest(conn)?;
            let triggers = {
                let mut q=conn.prepare("SELECT name,sql FROM sqlite_master WHERE type='trigger' AND tbl_name='threads'").map_err(err)?;
                let rows = q
                    .query_map([], |r| Ok((r.get::<_, String>(0)?, r.get::<_, String>(1)?)))
                    .map_err(err)?
                    .collect::<std::result::Result<Vec<_>, _>>()
                    .map_err(err)?;
                rows
            };
            // Projection describes reviewed SQL effects. Real triggers remain installed
            // on the source and can reject the actual operation with normal rollback.
            for (name, _) in &triggers {
                conn.execute_batch(&format!("DROP TRIGGER \"{}\"", name.replace('"', "\"\"")))
                    .map_err(err)?;
            }
            for row in rows.iter().filter(|r| r.before.db == db) {
                update(conn, row, true)?;
            }
            for (_, sql) in &triggers {
                conn.execute_batch(sql).map_err(err)?;
            }
            let digest = logical_database_digest(conn)?;
            conn.execute_batch("ROLLBACK").map_err(err)?;
            Ok((original, digest))
        })?;
        original_databases.insert(db.clone(), expected.0);
        expected_databases.insert(db, expected.1);
    }
    Ok(Prepared {
        selection,
        rows,
        files,
        catalog: official.clone(),
        expected_databases,
        original_databases,
    })
}
pub(super) fn preview_lines(p: &Prepared) -> Vec<String> {
    let mut lines=vec![format!("仅接管 {} 条已选失效受管主线程、{} 个历史文件；恢复已验证的当前官方 Provider，保留原模型与推理档位",p.rows.len(),p.files.len())];
    lines.extend(p.rows.iter().map(|r| {
        format!(
            "线程 {}：{} → {}；model {} → {}",
            r.before.id,
            r.before.provider.as_deref().unwrap_or("<missing>"),
            r.provider,
            r.before.model.as_deref().unwrap_or("<missing>"),
            r.model
        )
    }));
    lines
}
pub(super) fn validate_plan(ctx: &Context, p: &Prepared) -> Result<()> {
    validate(ctx, &p.selection)?;
    for (path, expected) in &p.catalog.dependencies {
        if &stamp(path)? != expected {
            return Err("官方目录准备来源在预览后变化".into());
        }
    }
    Ok(())
}
fn update(conn: &rusqlite::Connection, r: &RowEffect, forward: bool) -> Result<()> {
    use rusqlite::types::Value as Sql;
    let old = &r.before;
    let sets = vec!["model_provider=?1".to_string()];
    let mut values = vec![Sql::Text(if forward {
        r.provider.clone()
    } else {
        old.provider.clone().unwrap_or_default()
    })];
    let id = values.len() + 1;
    values.push(Sql::Text(old.id.clone()));
    let provider = values.len() + 1;
    values.push(if forward {
        old.provider.clone().map(Sql::Text).unwrap_or(Sql::Null)
    } else {
        Sql::Text(r.provider.clone())
    });
    let mut where_clauses = vec![
        format!("id=?{id}"),
        format!("model_provider IS ?{provider}"),
        "COALESCE(archived,0)=0".into(),
    ];
    if old.has_model_column {
        let n = values.len() + 1;
        values.push(old.model.clone().map(Sql::Text).unwrap_or(Sql::Null));
        where_clauses.push(format!("model IS ?{n}"));
    }
    if old.has_effort_column {
        let n = values.len() + 1;
        values.push(old.reasoning_effort.clone().map(Sql::Text).unwrap_or(Sql::Null));
        where_clauses.push(format!("reasoning_effort IS ?{n}"));
    }
    let n = conn
        .execute(
            &format!(
                "UPDATE threads SET {} WHERE {}",
                sets.join(","),
                where_clauses.join(" AND ")
            ),
            rusqlite::params_from_iter(values),
        )
        .map_err(err)?;
    if n != 1 {
        return Err(format!(
            "adopt thread changed rows={n}, expected 1: {}",
            old.id
        ));
    }
    Ok(())
}
/// Caller holds the existing MutationGuard and has established the native writer window.
pub(super) fn apply_verified(
    ctx: &Context,
    p: &Prepared,
    backup: &Path,
    guard: &super::repair::MutationGuard,
) -> Result<Outcome> {
    use crate::gateway::storage::{atomic_private, private_dir};
    eprintln!("[orphan-apply] enter rows={} files={}", p.rows.len(), p.files.len());
    guard.validate(ctx)?;
    validate_plan(ctx, p)?;
    eprintln!("[orphan-apply] validate ok");
    private_dir(backup)?;
    let manifest = backup.join("adoption-recovery.json");
    let recovery = |phase: &str| serde_json::json!({"version":1,"phase":phase,"rows":p.rows.iter().map(|r|serde_json::json!({"database":r.before.db,"id":r.before.id,"beforeProvider":r.before.provider,"afterProvider":r.provider,"beforeModel":r.before.model,"afterModel":r.model,"beforeEffort":r.before.reasoning_effort,"afterEffort":r.effort})).collect::<Vec<_>>(),"files":p.files.iter().enumerate().map(|(i,f)|serde_json::json!({"path":f.path,"backup":format!("rollout-{i}.bin")})).collect::<Vec<_>>()});
    for (i, f) in p.files.iter().enumerate() {
        atomic_private(&backup.join(format!("rollout-{i}.bin")), &f.before)?;
    }
    for (i, db) in p.selection.databases.iter().enumerate() {
        for (suffix, label) in [("", "db"), ("-wal", "wal")] {
            let path = PathBuf::from(format!("{}{suffix}", db.display()));
            if let Some(bytes) = optional(&path, 128 * 1024 * 1024)? {
                atomic_private(&backup.join(format!("database-{i}.{label}")), &bytes)?;
            }
        }
    }
    atomic_private(
        &manifest,
        &serde_json::to_vec_pretty(&recovery("prepared")).map_err(err)?,
    )?;
    validate_plan(ctx, p)?;
    let mut connections: Vec<(PathBuf, rusqlite::Connection)> = vec![];
    for db in p
        .rows
        .iter()
        .map(|r| r.before.db.clone())
        .collect::<BTreeSet<_>>()
    {
        let c = rusqlite::Connection::open_with_flags(
            &db,
            rusqlite::OpenFlags::SQLITE_OPEN_READ_WRITE | rusqlite::OpenFlags::SQLITE_OPEN_NO_MUTEX,
        )
        .map_err(err)?;
        c.busy_timeout(std::time::Duration::from_secs(5))
            .map_err(err)?;
        if let Err(e) = c.execute_batch("BEGIN IMMEDIATE") {
            for (_, c) in &connections {
                let _: rusqlite::Result<()> = c.execute_batch("ROLLBACK");
            }
            return Err(err(e));
        }
        connections.push((db, c));
    }
    let mut committed = BTreeSet::new();
    let mut written = vec![];
    let result = (|| -> Result<()> {
        // Validate every selected row while all write transactions are held, before first file effect.
        for (db, c) in &connections {
            for r in p.rows.iter().filter(|r| r.before.db == *db) {
                let actual:(Option<String>,Option<String>)=c.query_row("SELECT model_provider,rollout_path FROM threads WHERE id=?1 AND COALESCE(archived,0)=0",[&r.before.id],|row|Ok((row.get(0)?,row.get(1)?))).map_err(err)?;
                if actual.0 != r.before.provider
                    || actual.1.as_ref().map(|s| absolute(ctx, s))
                        != r.before.rollout_path.as_ref().map(PathBuf::from)
                {
                    return Err("孤儿线程 provider/path 在预览后变化".into());
                }
                update(c, r, true)?;
            }
        }
        for (i, f) in p.files.iter().enumerate() {
            if read_bounded(&f.path, inspect::LIMIT)? != f.before {
                return Err("孤儿历史文件已被外部更新".into());
            }
            atomic_private(&f.path, &f.after)?;
            written.push(i);
            if read_bounded(&f.path, inspect::LIMIT)? != f.after {
                return Err("孤儿历史写入后验失败".into());
            }
        }
        eprintln!("[orphan-apply] db updates done, committing");
        for (db, c) in &connections {
            c.execute_batch("COMMIT").map_err(err)?;
            committed.insert(db.clone());
        }
        eprintln!("[orphan-apply] committed {} dbs", committed.len());
        // Read actual selected database and rollout state; no status-only success.
        for (db, c) in &connections {
            for r in p.rows.iter().filter(|r| r.before.db == *db) {
                let provider: String = c
                    .query_row(
                        "SELECT model_provider FROM threads WHERE id=?1",
                        [&r.before.id],
                        |row| row.get(0),
                    )
                    .map_err(err)?;
                if provider != r.provider {
                    return Err("adopted database metadata verification failed".into());
                }
                if r.before.has_model_column {
                    let model: Option<String> = c
                        .query_row(
                            "SELECT model FROM threads WHERE id=?1",
                            [&r.before.id],
                            |row| row.get(0),
                        )
                        .map_err(err)?;
                    if model != r.before.model {
                        return Err("adopted model verification failed".into());
                    }
                }
                if r.before.has_effort_column {
                    let effort: Option<String> = c.query_row(
                        "SELECT reasoning_effort FROM threads WHERE id=?1", [&r.before.id], |row| row.get(0)
                    ).map_err(err)?;
                    if effort != r.before.reasoning_effort {
                        return Err("adopted effort verification failed".into());
                    }
                }
            }
        }
        // 验证完整历史字节；关闭连接后再核验数据库逻辑摘要。
        for f in &p.files {
            if read_bounded(&f.path, inspect::LIMIT)? != f.after {
                return Err("adopted rollout metadata verification failed".into());
            }
        }
        let remaining = select_inner(ctx, false)?.rows.len();
        eprintln!("[orphan-apply] post-check remaining={remaining}");
        if remaining > 0 {
            return Err(format!(
                "orphan provider diagnostic still reports {remaining} unresolved candidates"
            ));
        }
        atomic_private(
            &manifest,
            &serde_json::to_vec_pretty(&recovery("applied")).map_err(err)?,
        )?;
        Ok(())
    })();
    if let Err(e) = result {
        let mut errors = vec![];
        for (db, c) in &connections {
            if committed.contains(db) {
                if let Err(e) = c.execute_batch("BEGIN IMMEDIATE") {
                    errors.push(err(e));
                    continue;
                }
                let undo = (|| -> Result<()> {
                    for r in p.rows.iter().filter(|r| r.before.db == *db) {
                        update(c, r, false)?;
                    }
                    c.execute_batch("COMMIT").map_err(err)
                })();
                if let Err(e) = undo {
                    let _ = c.execute_batch("ROLLBACK");
                    errors.push(e);
                }
            } else {
                let _ = c.execute_batch("ROLLBACK");
            }
        }
        for i in written.into_iter().rev() {
            let f = &p.files[i];
            match read_bounded(&f.path, inspect::LIMIT) {
                Ok(actual) if actual == f.after => {
                    if let Err(e) = atomic_private(&f.path, &f.before) {
                        errors.push(e);
                    }
                }
                _ => errors.push("回滚时历史已外部变化，保留当前文件及备份".into()),
            }
        }
        let phase = if errors.is_empty() {
            "rolledBack"
        } else {
            "rollbackFailed"
        };
        let _ = atomic_private(
            &manifest,
            &serde_json::to_vec_pretty(&recovery(phase)).map_err(err)?,
        );
        let message = format!(
            "孤儿接管失败：{e}；回滚{}；恢复记录：{}",
            if errors.is_empty() {
                "完成".into()
            } else {
                errors.join("；")
            },
            manifest.display()
        );
        if errors.is_empty() {
            drop(connections);
            let effects = certify_effects(ctx, p, &p.original_databases, false)?;
            return Ok(Outcome {
                details: vec![message.clone()],
                effects,
                failure: Some(message),
            });
        }
        return Err(message);
    }
    drop(connections);
    let effects = certify_effects(ctx, p, &p.expected_databases, true)?;
    atomic_private(
        &manifest,
        &serde_json::to_vec_pretty(&recovery("completed")).map_err(err)?,
    )?;
    Ok(Outcome { details:vec![format!("已从真实候选集接管 {} 个线程数据库引用和 {} 个历史文件，健康 foreign/openai/Atelier 记录保持原样；备份 {}",p.rows.len(),p.files.len(),backup.display())],effects,failure:None })
}
/// Canonical full database observation, including non-thread tables and schema. This
/// proves the physical DB after stamp came from the reviewed SQL effects, not an
/// arbitrary file hash observed after another writer changed unrelated data.
pub(super) fn logical_database_digest(conn: &rusqlite::Connection) -> Result<String> {
    let mut hash = Sha256::new();
    let mut schema = conn
        .prepare("SELECT type,name,tbl_name,sql FROM sqlite_master ORDER BY type,name,tbl_name")
        .map_err(err)?;
    let definitions = schema
        .query_map([], |r| {
            Ok((
                r.get::<_, String>(0)?,
                r.get::<_, String>(1)?,
                r.get::<_, String>(2)?,
                r.get::<_, Option<String>>(3)?,
            ))
        })
        .map_err(err)?
        .collect::<std::result::Result<Vec<_>, _>>()
        .map_err(err)?;
    hash.update(serde_json::to_vec(&definitions).map_err(err)?);
    for pragma in ["user_version", "application_id"] {
        let value: i64 = conn
            .query_row(&format!("PRAGMA {pragma}"), [], |r| r.get(0))
            .map_err(err)?;
        hash.update(value.to_le_bytes());
    }
    for (_, name, _, _) in definitions.iter().filter(|v| v.0 == "table") {
        let quoted = name.replace('"', "\"\"");
        let mut statement = conn
            .prepare(&format!("SELECT * FROM \"{quoted}\""))
            .map_err(err)?;
        let columns = statement
            .column_names()
            .map(|s| s.to_string())
            .collect::<Vec<_>>();
        hash.update(serde_json::to_vec(&columns).map_err(err)?);
        let mut rows = statement.query([]).map_err(err)?;
        let mut encoded = vec![];
        while let Some(row) = rows.next().map_err(err)? {
            let mut fields = vec![];
            for n in 0..columns.len() {
                use rusqlite::types::ValueRef;
                fields.push(match row.get_ref(n).map_err(err)? {
                    ValueRef::Null => serde_json::json!(["null"]),
                    ValueRef::Integer(i) => serde_json::json!(["integer", i]),
                    ValueRef::Real(f) => serde_json::json!(["real", f.to_bits()]),
                    ValueRef::Text(s) => serde_json::json!(["text", s]),
                    ValueRef::Blob(b) => serde_json::json!(["blob", b]),
                });
            }
            encoded.push(serde_json::to_vec(&fields).map_err(err)?);
        }
        encoded.sort();
        for row in encoded {
            hash.update((row.len() as u64).to_le_bytes());
            hash.update(row);
        }
    }
    Ok(format!("{:x}", hash.finalize()))
}
pub(super) type EffectStamp = (u64, String, Option<std::time::SystemTime>);
#[derive(Clone, Default)]
pub(super) struct ApprovedEffects {
    effects: BTreeMap<PathBuf, (Option<Stamp>, Option<Stamp>)>,
}
impl ApprovedEffects {
    pub(super) fn paths(&self) -> Vec<PathBuf> {
        self.effects.keys().cloned().collect()
    }
    pub(super) fn effect_stamp(
        &self,
        path: &Path,
    ) -> Option<(Option<EffectStamp>, Option<EffectStamp>)> {
        self.effect(path)
    }
    pub(super) fn effect(&self, path: &Path) -> Option<(Option<EffectStamp>, Option<EffectStamp>)> {
        let tuple =
            |s: &Option<Stamp>| s.as_ref().map(|s| (s.length, s.sha256.clone(), s.modified));
        self.effects
            .get(path)
            .map(|(before, after)| (tuple(before), tuple(after)))
    }
}

pub(super) struct Outcome {
    pub details: Vec<String>,
    pub effects: ApprovedEffects,
    pub failure: Option<String>,
}
#[cfg(test)]
fn apply(
    ctx: &Context,
    p: &Prepared,
    backup: &Path,
    guard: &super::repair::MutationGuard,
) -> Result<Vec<String>> {
    apply_verified(ctx, p, backup, guard).and_then(|o| match o.failure {
        Some(e) => Err(e),
        None => Ok(o.details),
    })
}

fn certify_effects(
    ctx: &Context,
    p: &Prepared,
    expected_databases: &BTreeMap<PathBuf, String>,
    committed: bool,
) -> Result<ApprovedEffects> {
    let mut effects = ApprovedEffects::default();
    for db in expected_databases.keys() {
        let wal = PathBuf::from(format!("{}-wal", db.display()));
        let before = (stamp(db)?, stamp(&wal)?);
        let digest =
            super::snapshot::with_snapshot_path(db, |_, conn| logical_database_digest(conn))?;
        if digest != expected_databases[db] || before != (stamp(db)?, stamp(&wal)?) {
            return Err(
                "孤儿接管关闭写连接后的稳定后验失败；已保留备份与恢复记录，未批准后续线程修复"
                    .into(),
            );
        }
        effects.effects.insert(
            db.clone(),
            (
                p.selection.dependencies.get(db).cloned().flatten(),
                before.0,
            ),
        );
        effects.effects.insert(
            wal.clone(),
            (
                p.selection.dependencies.get(&wal).cloned().flatten(),
                before.1,
            ),
        );
    }
    for f in &p.files {
        let bytes = if committed { &f.after } else { &f.before };
        let after = stamp(&f.path)?;
        if !after.as_ref().is_some_and(|s| {
            s.length == bytes.len() as u64 && s.sha256 == format!("{:x}", Sha256::digest(bytes))
        }) {
            return Err("孤儿历史效果指纹核验失败，未批准后续线程修复".into());
        }
        effects.effects.insert(
            f.path.clone(),
            (
                p.selection.dependencies.get(&f.path).cloned().flatten(),
                after,
            ),
        );
    }
    let _ = ctx;
    Ok(effects)
}
