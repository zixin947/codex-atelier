//! Native thread/provider/model consistency checks. Explicit synthetic/offline filesystem inputs only.
use crate::threads::{item, DiagnosticItem};
use std::{
    collections::{BTreeMap, BTreeSet},
    path::{Path, PathBuf},
};
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ThreadModelCatalogInspection {
    pub errors: Vec<String>,
    pub examples: Vec<String>,
    pub total: usize,
    pub inconsistent: usize,
    pub accepted: usize,
    pub missing_models: usize,
}
/// Diagnostic wrapper at 0x10039d6e4. Catalog/DB failures retain error+fixable semantics.
pub fn check_thread_model_catalog_consistency(
    router_enabled: bool,
    i: &ThreadModelCatalogInspection,
) -> DiagnosticItem {
    let id = "thread_model_catalog_consistency";
    if !router_enabled {
        return item(
            id,
            "ok",
            "线程模型与 Catalog 一致性（路由未启用，跳过）",
            None,
            false,
        );
    }
    if i.inconsistent == 0 && i.errors.is_empty() {
        return item(
            id,
            "ok",
            "线程模型均可由 Catalog 或官方直通识别",
            None,
            false,
        );
    }
    let examples = if i.examples.is_empty() {
        "无示例".into()
    } else {
        i.examples.join("；")
    };
    let detail = if i.errors.is_empty() {
        format!("{} 条 aimai1 活跃主线程的 model 既不在当前 Catalog 中，也不是可直通的官方裸模型，其中 {} 条缺少 model。示例：{}",i.inconsistent,i.missing_models,examples)
    } else {
        format!(
            "读取 Catalog 或线程库失败 {} 项；已发现 {} 条不一致。示例：{}",
            i.errors.len(),
            i.inconsistent,
            examples
        )
    };
    item(
        id,
        "error",
        if i.errors.is_empty() {
            format!("{} 条线程模型无法安全路由", i.inconsistent)
        } else {
            "线程模型与 Catalog 一致性检查失败".into()
        },
        Some(detail),
        true,
    )
}
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ThreadProviderConvergenceInspection {
    pub target_provider: String,
    pub errors: Vec<String>,
    pub examples: Vec<String>,
    pub total: usize,
    pub db_mismatch: usize,
    pub rollout_mismatch: usize,
    pub missing_rollouts: usize,
    pub corrupt_rollouts: usize,
}
fn provider_diagnostic(
    router_enabled: bool,
    i: &ThreadProviderConvergenceInspection,
) -> DiagnosticItem {
    let id = "thread_provider_consistency";
    if i.db_mismatch == 0
        && i.rollout_mismatch == 0
        && i.missing_rollouts == 0
        && i.corrupt_rollouts == 0
        && i.errors.is_empty()
    {
        return item(
            id,
            "ok",
            if router_enabled {
                "线程已收敛到路由 provider"
            } else {
                "线程已收敛到官方 provider"
            },
            None,
            false,
        );
    }
    let examples = if i.examples.is_empty() {
        "无示例".into()
    } else {
        i.examples.join("；")
    };
    let mismatches = i.db_mismatch + i.rollout_mismatch;
    if mismatches == 0 && i.missing_rollouts == 0 && i.errors.is_empty() {
        return item(
            id,
            "warning",
            format!(
                "{} 条线程 rollout 结构损坏，已与正常线程隔离",
                i.corrupt_rollouts
            ),
            Some(examples),
            false,
        );
    }
    let mut details = vec![format!(
        "{} {}",
        if router_enabled {
            "路由启用中，活跃主线程必须收敛到"
        } else {
            "路由关闭时，活跃主线程必须收敛到"
        },
        i.target_provider
    )];
    if mismatches > 0 && router_enabled {
        details.push("否则 Codex 会隐藏线程".into());
    }
    if i.missing_rollouts > 0 {
        details.push(format!(
            "另有 {} 条历史文件缺失，已交由「线程缺历史文件」单独报告",
            i.missing_rollouts
        ));
    }
    if i.corrupt_rollouts > 0 {
        details.push(format!(
            "{} 条 rollout 结构损坏，无法判断收敛状态",
            i.corrupt_rollouts
        ));
    }
    let title = if mismatches == 0 {
        "线程 provider 已收敛（仍有其它历史问题）".into()
    } else {
        format!("{} 项线程 provider 未收敛", mismatches)
    };
    item(
        id,
        "error",
        title,
        Some(format!("{}。示例：{}", details.join("；"), examples)),
        true,
    )
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RouterCatalogModelIndex {
    pub slugs: BTreeSet<String>,
    pub aliases: BTreeMap<String, String>,
    pub default_model: Option<String>,
    pub no_account: bool,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ModelDisposition {
    Catalog,
    OfficialPassthrough,
    Replace(String),
    NoReplacement,
}
fn is_slot(slug: &str) -> bool {
    matches!(
        slug,
        "gpt-5.6-sol" | "gpt-5.6-terra" | "gpt-5.6-luna" | "gpt-5.5" | "gpt-5.4"
    )
}
/// @0x10036cbd4: explicit flag wins; absent flag infers nonempty all-slot catalog.
pub fn catalog_no_account_mode(v: &serde_json::Value) -> Result<bool, String> {
    if let Some(flag) = v.get("aimami_no_account") {
        return flag.as_bool().ok_or_else(|| {
            "Invalid data: router catalog aimami_no_account must be boolean".into()
        });
    }
    let models = v
        .get("models")
        .and_then(|v| v.as_array())
        .ok_or("Invalid data: router catalog has no models array")?;
    Ok(!models.is_empty()
        && models
            .iter()
            .all(|v| v.get("slug").and_then(|v| v.as_str()).is_some_and(is_slot)))
}
impl RouterCatalogModelIndex {
    pub fn from_json(v: &serde_json::Value) -> Result<Self, String> {
        let mut slugs = BTreeSet::new();
        let mut aliases = BTreeMap::new();
        let mut first_managed = None;
        let mut first_slot = None;
        if let Some(models) = v.get("models").and_then(|v| v.as_array()) {
            for model in models {
                let Some(slug) = model.get("slug").and_then(|v| v.as_str()) else {
                    continue;
                };
                slugs.insert(slug.to_string());
                if is_slot(slug) && first_slot.is_none() {
                    first_slot = Some(slug.to_string())
                }
                if let Some((provider, suffix)) = slug.split_once("::") {
                    if provider.starts_with("aimami_relay_") && !suffix.trim().is_empty() {
                        if first_managed.is_none() {
                            first_managed = Some(slug.to_string())
                        }
                        aliases
                            .entry(suffix.to_string())
                            .or_insert_with(|| slug.to_string());
                    }
                }
            }
        }
        let no_account = catalog_no_account_mode(v)?;
        let default_model = first_managed.or_else(|| if no_account { first_slot } else { None });
        Ok(Self {
            slugs,
            aliases,
            default_model,
            no_account,
        })
    }
    /// @0x1008b3fdc: accept exact catalog slug first; ordinary bare names pass through
    /// only outside no-account mode. Legacy bare suffix aliases are considered afterwards.
    pub fn disposition(&self, model: Option<&str>) -> ModelDisposition {
        let m = model.unwrap_or("").trim();
        if !m.is_empty() && self.slugs.contains(m) {
            return ModelDisposition::Catalog;
        }
        if !self.no_account
            && !m.is_empty()
            && !m.contains("::")
            && !crate::threads::is_aimami_provider(m)
        {
            return ModelDisposition::OfficialPassthrough;
        }
        if let Some(mapped) = self.aliases.get(m) {
            if !mapped.trim().is_empty() {
                return ModelDisposition::Replace(mapped.clone());
            }
        }
        match self.default_model.as_ref().filter(|m| !m.trim().is_empty()) {
            Some(m) => ModelDisposition::Replace(m.clone()),
            None => ModelDisposition::NoReplacement,
        }
    }
}
pub fn load_router_catalog_model_index(
    catalog_dir: &Path,
) -> Result<RouterCatalogModelIndex, String> {
    let p = catalog_dir.join("codex_router_catalog.json");
    let s = std::fs::read_to_string(&p)
        .map_err(|e| format!("Operation failed: read router catalog {}: {e}", p.display()))?;
    let v = serde_json::from_str::<serde_json::Value>(&s).map_err(|e| {
        format!(
            "Operation failed: parse router catalog {}: {e}",
            p.display()
        )
    })?;
    RouterCatalogModelIndex::from_json(&v)
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RouterModelTarget {
    pub id: String,
    pub rollout_path: Option<String>,
    pub model: Option<String>,
}
pub fn collect_router_model_targets(
    conn: &rusqlite::Connection,
) -> Result<Vec<RouterModelTarget>, String> {
    let mut st = conn
        .prepare("PRAGMA table_info(threads)")
        .map_err(|e| format!("sqlite pragma threads: {e}"))?;
    let cols = st
        .query_map([], |r| r.get::<_, String>(1))
        .map_err(|e| format!("sqlite read columns: {e}"))?
        .collect::<rusqlite::Result<BTreeSet<_>>>()
        .map_err(|e| format!("sqlite collect columns: {e}"))?;
    if !cols.contains("model") {
        return Ok(vec![]);
    }
    let mut st=conn.prepare("SELECT id, rollout_path, model, COALESCE(source, '') FROM threads WHERE COALESCE(archived, 0) = 0 AND model_provider = ?1 ORDER BY id").map_err(|e|format!("prepare router model query: {e}"))?;
    let rows = st
        .query_map(["aimai1"], |r| {
            Ok((
                r.get::<_, String>(0)?,
                r.get::<_, Option<String>>(1)?,
                r.get::<_, Option<String>>(2)?,
                r.get::<_, String>(3)?,
            ))
        })
        .map_err(|e| format!("query router model rows: {e}"))?;
    let mut out = vec![];
    for r in rows {
        let (id, rollout_path, model, source) =
            r.map_err(|e| format!("read router model row: {e}"))?;
        if !source.contains("\"subagent\"") {
            out.push(RouterModelTarget {
                id,
                rollout_path,
                model,
            })
        }
    }
    Ok(out)
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RolloutProviderAnchors {
    pub first: Option<String>,
    pub last: Option<String>,
    pub native: Option<String>,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RolloutReadFailure {
    Corrupt(String),
    Other(String),
}
impl RolloutReadFailure {
    fn message(&self) -> &str {
        match self {
            Self::Corrupt(s) | Self::Other(s) => s,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProviderObservation {
    PathMissing,
    FileMissing(PathBuf),
    Anchors(RolloutProviderAnchors),
    Corrupt(String),
    Error(String),
}
fn example(i: &mut ThreadProviderConvergenceInspection, s: String) {
    if i.examples.len() < 3 {
        i.examples.push(s)
    }
}
/// Counter policy @0x1008c9d64. Do not count DB drift for missing/broken/external-owned histories.
pub fn apply_provider_convergence_observation(
    i: &mut ThreadProviderConvergenceInspection,
    row: &crate::threads::ProviderConvergenceTarget,
    observation: ProviderObservation,
) {
    i.total += 1;
    match observation {
        ProviderObservation::PathMissing => {
            i.missing_rollouts += 1;
            example(i, format!("{}: rollout path missing", row.id));
        }
        ProviderObservation::FileMissing(p) => {
            i.missing_rollouts += 1;
            example(i, format!("{}: rollout missing ({})", row.id, p.display()));
        }
        ProviderObservation::Corrupt(e) => {
            i.corrupt_rollouts += 1;
            example(i, format!("{}: {e}", row.id));
        }
        ProviderObservation::Error(e) => i.errors.push(format!("{}: {e}", row.id)),
        ProviderObservation::Anchors(a) => {
            if [&a.first, &a.last, &a.native].iter().any(|p| {
                p.as_deref()
                    .is_some_and(|p| !crate::threads::is_router_or_openai_provider(p))
            }) {
                return;
            }
            let db_mismatch = row.provider != i.target_provider;
            let rollout_mismatch = [&a.first, &a.last, &a.native]
                .iter()
                .any(|p| p.as_deref().is_some_and(|p| p != i.target_provider));
            if db_mismatch {
                i.db_mismatch += 1
            }
            if rollout_mismatch {
                i.rollout_mismatch += 1
            }
            if db_mismatch || rollout_mismatch {
                example(
                    i,
                    format!(
                        "{}: DB={} / rollout_first={:?} / rollout_last={:?} / target={}",
                        row.id, row.provider, a.first, a.last, i.target_provider
                    ),
                );
            }
        }
    }
}
#[derive(Debug, Clone)]
struct SessionAnchors {
    first: serde_json::Value,
    last: serde_json::Value,
    native: Option<serde_json::Value>,
}
fn rollout_reader(path: &Path) -> Result<Box<dyn std::io::BufRead>, RolloutReadFailure> {
    use std::io::BufReader;
    let p = crate::threads::resolve_rollout(path)
        .map_err(|e| RolloutReadFailure::Other(format!("IO error: {e}")))?
        .ok_or_else(|| RolloutReadFailure::Other("IO error: rollout not found".into()))?;
    let f =
        std::fs::File::open(&p).map_err(|e| RolloutReadFailure::Other(format!("IO error: {e}")))?;
    if p.to_string_lossy().ends_with(".jsonl.zst") {
        Ok(Box::new(BufReader::new(
            zstd::stream::read::Decoder::new(f)
                .map_err(|e| RolloutReadFailure::Other(format!("IO error: {e}")))?,
        )))
    } else {
        Ok(Box::new(BufReader::new(f)))
    }
}
fn parse_line(bytes: &[u8]) -> Option<serde_json::Value> {
    let bytes = bytes.strip_suffix(b"\r").unwrap_or(bytes);
    serde_json::from_slice(bytes).ok()
}
fn is_meta(v: &serde_json::Value) -> bool {
    v.get("type").and_then(|v| v.as_str()) == Some("session_meta")
}
fn is_settings(v: &serde_json::Value) -> bool {
    v.get("type").and_then(|v| v.as_str()) == Some("event_msg")
        && v.pointer("/payload/type").and_then(|v| v.as_str()) == Some("thread_settings_applied")
}
/// Preserve native traversal: locate first meta, then scan plain tail backwards in 1 MiB
/// blocks or reopen zstd and stream its decompressed tail forwards.
fn session_anchors(path: &Path) -> Result<Option<SessionAnchors>, RolloutReadFailure> {
    session_anchors_with_budget(path, None)
}
fn native_line(
    reader: &mut dyn std::io::BufRead,
    line: &mut Vec<u8>,
    remaining: &mut Option<u64>,
) -> Result<usize, RolloutReadFailure> {
    let Some(left) = remaining.as_mut() else {
        return reader
            .read_until(b'\n', line)
            .map_err(|e| RolloutReadFailure::Other(format!("IO error: {e}")));
    };
    if *left == 0 {
        return if reader
            .fill_buf()
            .map_err(|e| RolloutReadFailure::Other(e.to_string()))?
            .is_empty()
        {
            Ok(0)
        } else {
            Err(RolloutReadFailure::Other(
                "native provider traversal byte budget exhausted; not verified".into(),
            ))
        };
    }
    let cap = (*left).min(0x1800001) as usize;
    let n = super::history::chunk(reader, cap, line)
        .map_err(|e| RolloutReadFailure::Other(format!("IO error: {e}")))?;
    *left -= n as u64;
    if n == cap
        && line.last() != Some(&b'\n')
        && !reader
            .fill_buf()
            .map_err(|e| RolloutReadFailure::Other(e.to_string()))?
            .is_empty()
    {
        return Err(RolloutReadFailure::Other(
            "native provider traversal line/byte budget exhausted; not verified".into(),
        ));
    }
    Ok(n)
}
fn session_anchors_with_budget(
    path: &Path,
    mut remaining: Option<u64>,
) -> Result<Option<SessionAnchors>, RolloutReadFailure> {
    use std::io::{Read, Seek, SeekFrom};
    let p = crate::threads::resolve_rollout(path)
        .map_err(|e| RolloutReadFailure::Other(format!("IO error: {e}")))?
        .ok_or_else(|| RolloutReadFailure::Other("IO error: rollout not found".into()))?;
    let mut reader = rollout_reader(&p)?;
    let mut line = vec![];
    let mut offset = 0u64;
    let first = loop {
        line.clear();
        let n = native_line(&mut *reader, &mut line, &mut remaining)?;
        if n == 0 {
            return Ok(None);
        }
        offset += n as u64;
        if line.last() == Some(&b'\n') {
            line.pop();
        }
        if let Some(v) = parse_line(&line) {
            if is_meta(&v) {
                break v;
            }
        }
    };
    let id = first
        .pointer("/payload/id")
        .and_then(|v| v.as_str())
        .filter(|s| !s.is_empty())
        .ok_or_else(|| {
            RolloutReadFailure::Corrupt(format!(
                "Invalid data: first session_meta thread id missing in {}",
                p.display()
            ))
        })?
        .to_string();
    let mut a = SessionAnchors {
        first: first.clone(),
        last: first,
        native: None,
    };
    if p.to_string_lossy().ends_with(".jsonl.zst") {
        let mut reader = rollout_reader(&p)?;
        if let Some(left) = remaining.as_mut() {
            if offset > *left {
                return Err(RolloutReadFailure::Other("native provider traversal byte budget exhausted before compressed tail; not verified".into()));
            }
            *left -= offset;
        }
        let copied = std::io::copy(&mut reader.by_ref().take(offset), &mut std::io::sink())
            .map_err(|e| RolloutReadFailure::Other(format!("IO error: {e}")))?;
        if copied != offset {
            return Err(RolloutReadFailure::Other(format!(
                "Operation failed: rollout changed while streaming: expected {offset} bytes, got {copied}"
            )));
        }
        loop {
            line.clear();
            if native_line(&mut *reader, &mut line, &mut remaining)? == 0 {
                break;
            }
            if line.last() == Some(&b'\n') {
                line.pop();
            }
            let Some(v) = parse_line(&line) else { continue };
            if is_meta(&v) && v.pointer("/payload/id").and_then(|v| v.as_str()) == Some(&id) {
                a.last = v;
                a.native = None
            } else if is_settings(&v) {
                a.native = Some(v)
            }
        }
    } else {
        let mut f = std::fs::File::open(&p)
            .map_err(|e| RolloutReadFailure::Other(format!("IO error: {e}")))?;
        let mut end = f
            .metadata()
            .map_err(|e| RolloutReadFailure::Other(format!("IO error: {e}")))?
            .len();
        if end <= offset {
            return Ok(Some(a));
        }
        let mut pending = vec![];
        'blocks: while end > offset {
            let start = end.saturating_sub(1024 * 1024).max(offset);
            if let Some(left) = remaining.as_mut() {
                if end - start > *left {
                    return Err(RolloutReadFailure::Other(
                        "native provider reverse-tail budget exhausted; not verified".into(),
                    ));
                }
                *left -= end - start;
                if pending.len() > 0x1800001 {
                    return Err(RolloutReadFailure::Other(
                        "native provider reverse-tail line budget exhausted; not verified".into(),
                    ));
                }
            }
            let mut block = vec![0u8; (end - start) as usize];
            f.seek(SeekFrom::Start(start))
                .map_err(|e| RolloutReadFailure::Other(format!("IO error: {e}")))?;
            f.read_exact(&mut block)
                .map_err(|e| RolloutReadFailure::Other(format!("IO error: {e}")))?;
            block.extend_from_slice(&pending);
            let mut stop = block.len();
            for n in (0..block.len()).rev() {
                if block[n] == b'\n' {
                    let bytes = &block[n + 1..stop];
                    if let Some(v) = parse_line(bytes) {
                        if a.native.is_none() && is_settings(&v) {
                            a.native = Some(v.clone())
                        }
                        if is_meta(&v)
                            && v.pointer("/payload/id").and_then(|v| v.as_str()) == Some(&id)
                        {
                            a.last = v;
                            break 'blocks;
                        }
                    }
                    stop = n
                }
            }
            if start == offset {
                if let Some(v) = parse_line(&block[..stop]) {
                    if a.native.is_none() && is_settings(&v) {
                        a.native = Some(v.clone())
                    }
                    if is_meta(&v) && v.pointer("/payload/id").and_then(|v| v.as_str()) == Some(&id)
                    {
                        a.last = v
                    }
                }
                break;
            }
            pending = block[..stop].to_vec();
            end = start;
        }
    }
    Ok(Some(a))
}
fn meta_provider(
    v: &serde_json::Value,
    path: &Path,
    which: &str,
) -> Result<Option<String>, RolloutReadFailure> {
    match v.pointer("/payload/model_provider") {
        None | Some(serde_json::Value::Null) => Ok(None),
        Some(serde_json::Value::String(s)) if !s.trim().is_empty() => Ok(Some(s.clone())),
        _ => Err(RolloutReadFailure::Other(format!(
            "Operation failed: {which} session_meta provider invalid in {}; history preserved",
            path.display()
        ))),
    }
}
fn anchor_providers(
    a: &SessionAnchors,
    path: &Path,
) -> Result<RolloutProviderAnchors, RolloutReadFailure> {
    // Native settings provider is checked before either session_meta provider.
    let native = if let Some(v) = a.native.as_ref() {
        let s = v
            .pointer("/payload/thread_settings/model_provider_id")
            .and_then(|v| v.as_str())
            .filter(|s| !s.trim().is_empty())
            .ok_or_else(|| {
                RolloutReadFailure::Other(
                    "Operation failed: native thread settings has no provider; history preserved"
                        .into(),
                )
            })?;
        Some(s.to_string())
    } else {
        None
    };
    Ok(RolloutProviderAnchors {
        first: meta_provider(&a.first, path, "first")?,
        last: meta_provider(&a.last, path, "last")?,
        native,
    })
}
pub fn read_rollout_provider_anchors(
    path: &Path,
) -> Result<Option<RolloutProviderAnchors>, RolloutReadFailure> {
    if let Some(observed) = super::observation::read(path) {
        match observed {
            Ok(observed) if observed.require_complete().is_ok() => return observed.anchors.clone(),
            Ok(_) => (),
            Err(error)
                if error == "operation history budget exhausted"
                    || error == "operation history summary budget exhausted" =>
            {
                ()
            }
            Err(error) => return Err(RolloutReadFailure::Other(error)),
        }
        return super::observation::native_provider(path, |budget| {
            session_anchors_with_budget(path, budget)?
                .map(|a| anchor_providers(&a, path))
                .transpose()
        });
    }
    session_anchors(path)?
        .map(|a| anchor_providers(&a, path))
        .transpose()
}
#[derive(Debug, Clone, Default)]
pub struct DiagnosticRolloutProviderCache {
    entries: BTreeMap<PathBuf, Result<Option<RolloutProviderAnchors>, RolloutReadFailure>>,
}
impl DiagnosticRolloutProviderCache {
    pub fn len(&self) -> usize {
        self.entries.len()
    }
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
    pub fn read(
        &mut self,
        path: &Path,
    ) -> Result<Option<RolloutProviderAnchors>, RolloutReadFailure> {
        if super::observation::active() {
            return read_rollout_provider_anchors(path);
        }
        self.entries
            .entry(path.to_path_buf())
            .or_insert_with(|| read_rollout_provider_anchors(path))
            .clone()
    }
    /// Same native threshold (8) and worker count (4), using portable scoped workers.
    pub fn prefetch(&mut self, paths: &[PathBuf]) {
        // Operation summaries are thread-local; worker scans would bypass the shared observation.
        if super::observation::active() {
            for path in paths {
                let _ = self.read(path);
            }
            return;
        }
        let pending: Vec<_> = paths
            .iter()
            .filter(|p| !self.entries.contains_key(*p))
            .cloned()
            .collect();
        if pending.len() < 8 {
            return;
        }
        let results = std::thread::scope(|scope| {
            let mut workers = vec![];
            for offset in 0..4 {
                let pending = &pending;
                match std::thread::Builder::new().spawn_scoped(scope, move || {
                    pending
                        .iter()
                        .skip(offset)
                        .step_by(4)
                        .map(|p| (p.clone(), read_rollout_provider_anchors(p)))
                        .collect::<Vec<_>>()
                }) {
                    Ok(h) => workers.push(h),
                    Err(_) => return vec![],
                }
            }
            workers
                .into_iter()
                .flat_map(|h| h.join().unwrap_or_default())
                .collect::<Vec<_>>()
        });
        self.entries.extend(results);
    }
}
fn open_db(path: &Path) -> Result<rusqlite::Connection, String> {
    let c = rusqlite::Connection::open_with_flags(
        path,
        rusqlite::OpenFlags::SQLITE_OPEN_READ_ONLY | rusqlite::OpenFlags::SQLITE_OPEN_NO_MUTEX,
    )
    .map_err(|e| format!("sqlite open readonly: {e}"))?;
    c.busy_timeout(std::time::Duration::from_secs(5))
        .map_err(|e| format!("sqlite open readonly: {e}"))?;
    Ok(c)
}
pub fn collect_existing_convergence_rollout_paths(dbs: &[PathBuf]) -> Vec<PathBuf> {
    let mut paths = vec![];
    let mut seen = BTreeSet::new();
    for db in dbs {
        if std::fs::metadata(db).is_err() {
            continue;
        }
        let Ok(conn) = open_db(db) else { continue };
        let Ok(rows) = crate::threads::collect_provider_convergence_targets(&conn) else {
            continue;
        };
        for row in rows {
            if let Some(p) = row.rollout_path {
                let p = PathBuf::from(p);
                if matches!(crate::threads::resolve_rollout(&p), Ok(Some(_)))
                    && seen.insert(p.clone())
                {
                    paths.push(p)
                }
            }
        }
    }
    paths
}
pub fn inspect_thread_provider_convergence_with_reader(
    dbs: &[PathBuf],
    target: &str,
    cache: &mut DiagnosticRolloutProviderCache,
) -> ThreadProviderConvergenceInspection {
    let mut i = ThreadProviderConvergenceInspection {
        target_provider: target.into(),
        ..Default::default()
    };
    if target != "aimai1" && target != "openai" {
        i.errors.push(format!(
            "Operation failed: unsupported thread provider convergence target: {target}"
        ));
        return i;
    }
    for db in dbs {
        if std::fs::metadata(db).is_err() {
            continue;
        }
        let rows =
            open_db(db).and_then(|c| crate::threads::collect_provider_convergence_targets(&c));
        let rows = match rows {
            Ok(r) => r,
            Err(e) => {
                i.errors
                    .push(format!("{}: Operation failed: {e}", db.display()));
                continue;
            }
        };
        for row in rows {
            let observation = match row.rollout_path.as_ref() {
                None => ProviderObservation::PathMissing,
                Some(p) => match crate::threads::resolve_rollout(Path::new(p)) {
                    Ok(None) => ProviderObservation::FileMissing(p.into()),
                    Err(e) => ProviderObservation::Error(e.to_string()),
                    Ok(Some(_)) => match cache.read(Path::new(p)) {
                        Ok(Some(a)) => ProviderObservation::Anchors(a),
                        Ok(None) => ProviderObservation::Corrupt(format!(
                            "Invalid data: session_meta missing in {p}"
                        )),
                        Err(RolloutReadFailure::Corrupt(e)) => ProviderObservation::Corrupt(e),
                        Err(RolloutReadFailure::Other(e)) => ProviderObservation::Error(e),
                    },
                },
            };
            apply_provider_convergence_observation(&mut i, &row, observation);
        }
    }
    i
}
pub fn check_thread_provider_consistency_with_cache(
    dbs: &[PathBuf],
    router_enabled: bool,
    cache: &mut DiagnosticRolloutProviderCache,
) -> DiagnosticItem {
    cache.prefetch(&collect_existing_convergence_rollout_paths(dbs));
    let i = inspect_thread_provider_convergence_with_reader(
        dbs,
        if router_enabled { "aimai1" } else { "openai" },
        cache,
    );
    provider_diagnostic(router_enabled, &i)
}
fn has_unwritable_router_model_rollout(
    target: &RouterModelTarget,
) -> Result<bool, RolloutReadFailure> {
    let Some(path) = target.rollout_path.as_ref() else {
        return Ok(false);
    };
    let path = Path::new(path);
    if crate::threads::resolve_rollout(path)
        .map_err(|e| RolloutReadFailure::Other(format!("IO error: {e}")))?
        .is_none()
    {
        return Ok(false);
    }
    let a = match session_anchors(path) {
        Ok(Some(a)) => a,
        Ok(None) => return Ok(true),
        Err(RolloutReadFailure::Corrupt(_)) => return Ok(true),
        Err(e) => return Err(e),
    };
    if a.first.pointer("/payload/id").and_then(|v| v.as_str()) != Some(&target.id) {
        return Err(RolloutReadFailure::Other(format!(
            "Operation failed: rollout identity does not match thread {}; repair its reference before retrying",
            target.id
        )));
    }
    let providers = match anchor_providers(&a, path) {
        Ok(p) => p,
        Err(RolloutReadFailure::Corrupt(_)) => return Ok(true),
        Err(e) => return Err(e),
    };
    if [&providers.first, &providers.last, &providers.native]
        .iter()
        .any(|p| {
            p.as_deref()
                .is_some_and(|p| !crate::threads::is_router_or_openai_provider(p))
        })
    {
        return Ok(true);
    }
    if let Some(v) = a.native.as_ref() {
        match v.pointer("/payload/thread_id") {
            None | Some(serde_json::Value::Null) => (),
            Some(serde_json::Value::String(id)) if id == &target.id => (),
            _ => {
                return Err(RolloutReadFailure::Other(
                    "Operation failed: latest native settings belongs to another task; history preserved".into(),
                ))
            }
        }
    }
    Ok(false)
}
pub fn inspect_router_thread_model_catalog(
    dbs: &[PathBuf],
    catalog_dir: &Path,
) -> ThreadModelCatalogInspection {
    let mut i = ThreadModelCatalogInspection::default();
    let index = match load_router_catalog_model_index(catalog_dir) {
        Ok(i) => i,
        Err(e) => {
            i.errors.push(e);
            return i;
        }
    };
    for db in dbs {
        if std::fs::metadata(db).is_err() {
            continue;
        }
        let rows = open_db(db).and_then(|c| collect_router_model_targets(&c));
        let rows = match rows {
            Ok(r) => r,
            Err(e) => {
                i.errors
                    .push(format!("{}: Operation failed: {e}", db.display()));
                continue;
            }
        };
        i.total += rows.len();
        for row in rows {
            match index.disposition(row.model.as_deref()) {
                ModelDisposition::Catalog | ModelDisposition::OfficialPassthrough => {
                    i.accepted += 1;
                }
                _ => match has_unwritable_router_model_rollout(&row) {
                    Ok(true) => (),
                    Err(e) => i.errors.push(format!("{}: {}", row.id, e.message())),
                    Ok(false) => {
                        i.inconsistent += 1;
                        let model = row.model.as_deref().unwrap_or("").trim();
                        if model.is_empty() {
                            i.missing_models += 1
                        }
                        if i.examples.len() < 3 {
                            i.examples.push(format!("{}: model={} is neither catalog-valid nor an official passthrough model",row.id,if model.is_empty(){"<empty>"}else{model}));
                        }
                    }
                },
            }
        }
    }
    i
}
pub fn check_thread_model_catalog_consistency_from_sources(
    dbs: &[PathBuf],
    catalog_dir: &Path,
    router_enabled: bool,
) -> DiagnosticItem {
    if !router_enabled {
        return check_thread_model_catalog_consistency(
            false,
            &ThreadModelCatalogInspection::default(),
        );
    }
    check_thread_model_catalog_consistency(
        true,
        &inspect_router_thread_model_catalog(dbs, catalog_dir),
    )
}
