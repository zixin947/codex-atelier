//! Direct static-evidence port. No default paths or access to a running client.
use rusqlite::{Connection, OpenFlags};
use serde::{Deserialize, Serialize};
use std::{collections::BTreeSet, path::PathBuf, time::Duration};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DiagnosticItem {
    pub id: String,
    pub status: String,
    pub title: String,
    pub detail: Option<String>,
    pub fixable: bool,
}
fn item(
    id: &str,
    status: &str,
    title: impl Into<String>,
    detail: Option<String>,
    fixable: bool,
) -> DiagnosticItem {
    DiagnosticItem {
        id: id.into(),
        status: status.into(),
        title: title.into(),
        detail,
        fixable,
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MainThread {
    pub db: PathBuf,
    pub id: String,
    pub rollout_path: Option<String>,
    pub provider: Option<String>,
    pub model: Option<String>,
    pub reasoning_effort: Option<String>,
    pub has_model_column: bool,
    pub has_effort_column: bool,
}
/// Binary 0x100393ea4: read-only/no-mutex, 3-second timeout, fail the entire collection on any DB failure.
pub fn collect_diagnostic_main_threads(dbs: &[PathBuf]) -> Result<Vec<MainThread>, String> {
    let mut result = vec![];
    for db in dbs {
        if let Some(rows) = observation::cached_rows(db) {
            result.extend(rows?);
            continue;
        }
        let rows = collect_diagnostic_main_threads_uncached(std::slice::from_ref(db));
        observation::store_rows(db, rows.clone());
        result.extend(rows?);
    }
    Ok(result)
}
fn collect_diagnostic_main_threads_uncached(dbs: &[PathBuf]) -> Result<Vec<MainThread>, String> {
    let mut result = vec![];
    for db in dbs {
        let conn = Connection::open_with_flags(
            db,
            OpenFlags::SQLITE_OPEN_READ_ONLY | OpenFlags::SQLITE_OPEN_NO_MUTEX,
        )
        .map_err(|e| format!("open thread database {} for diagnostics: {e}", db.display()))?;
        let _ = conn.busy_timeout(Duration::from_secs(3));
        let mut stmt = conn
            .prepare("PRAGMA table_info(threads)")
            .map_err(|e| format!("prepare thread column query for {}: {e}", db.display()))?;
        let cols = stmt
            .query_map([], |r| r.get::<_, String>(1))
            .map_err(|e| format!("query thread columns from {}: {e}", db.display()))?
            .collect::<Result<BTreeSet<_>, _>>()
            .map_err(|e| format!("read thread columns from {}: {e}", db.display()))?;
        let hm = cols.contains("model");
        let he = cols.contains("reasoning_effort");
        let sql = format!("SELECT id, rollout_path, model_provider, COALESCE(source, ''), {}, {} FROM threads WHERE COALESCE(archived, 0) = 0 ORDER BY id", if hm {"model"} else {"NULL"}, if he {"reasoning_effort"} else {"NULL"});
        let mut stmt = conn
            .prepare(&sql)
            .map_err(|e| format!("prepare thread diagnostics query for {}: {e}", db.display()))?;
        let mut rows = stmt
            .query([])
            .map_err(|e| format!("query thread diagnostics rows from {}: {e}", db.display()))?;
        while let Some(row) = rows
            .next()
            .map_err(|e| format!("read thread diagnostics row from {}: {e}", db.display()))?
        {
            let parsed = (|| -> rusqlite::Result<_> {
                Ok((
                    row.get::<_, String>(0)?,
                    row.get::<_, Option<String>>(1)?,
                    row.get::<_, Option<String>>(2)?,
                    row.get::<_, String>(3)?,
                    row.get::<_, Option<String>>(4)?,
                    row.get::<_, Option<String>>(5)?,
                ))
            })()
            .map_err(|e| format!("read thread diagnostics row from {}: {e}", db.display()))?;
            let (id, rollout_path, provider, source, model, reasoning_effort) = parsed;
            if source.contains("\"subagent\"") {
                continue;
            }
            // Native trim only decides whether a value is absent; it does not replace nonblank text.
            let provider = provider.filter(|p| !p.trim().is_empty());
            result.push(MainThread {
                db: db.clone(),
                id,
                rollout_path,
                provider,
                model,
                reasoning_effort,
                has_model_column: hm,
                has_effort_column: he,
            });
        }
    }
    Ok(result)
}
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct MissingRolloutsInspection {
    pub errors: Vec<String>,
    pub examples: Vec<String>,
    pub total: usize,
    pub missing: usize,
    pub recoverable: usize,
    pub unrecoverable: usize,
}
/// Exact diagnostic wrapper (0x1003921a4); inspection errors deliberately produce an error item even if missing=0.
pub fn check_thread_missing_rollouts(i: &MissingRolloutsInspection) -> DiagnosticItem {
    if i.missing == 0 && i.errors.is_empty() {
        return item(
            "thread_missing_rollouts",
            "ok",
            "线程 rollout 文件完整",
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
        format!(
            "可从备份恢复 {} 条；无可信备份需隔离 {} 条。示例：{}",
            i.recoverable, i.unrecoverable, examples
        )
    } else {
        format!(
            "读取状态库失败 {} 项；可恢复 {} 条，不可恢复 {} 条。示例：{}",
            i.errors.len(),
            i.recoverable,
            i.unrecoverable,
            examples
        )
    };
    item(
        "thread_missing_rollouts",
        "error",
        format!("{} 条活跃线程缺少 rollout 文件", i.missing),
        Some(detail),
        true,
    )
}
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ProjectStateInspection {
    pub read_error: Option<String>,
    pub db_errors: Vec<String>,
    pub issue: Option<String>,
    pub needs_repair: bool,
    pub roots: usize,
    pub active_roots: usize,
    pub hints: usize,
    pub db_roots: usize,
    pub db_hints: usize,
    pub active_threads: usize,
    pub cwd_backfill: usize,
}
/// Wrapper 0x100391c44. Producer `codex_project_state::inspect` is a separately recovered function.
pub fn check_project_state_integrity(i: &ProjectStateInspection) -> DiagnosticItem {
    if let Some(e) = &i.read_error {
        return item(
            "project_state_integrity",
            "warning",
            "Codex 项目索引无法读取",
            Some(format!(
                ".codex-global-state.json 读取失败：{e}。项目入口/线程归属可能异常。"
            )),
            false,
        );
    }
    if !i.db_errors.is_empty() {
        return item(
            "project_state_integrity",
            "warning",
            "Codex 项目索引无法体检线程库",
            Some(format!("读取线程库失败：{}", i.db_errors.join(" | "))),
            false,
        );
    }
    if !i.needs_repair {
        return item(
            "project_state_integrity",
            "ok",
            "Codex 项目索引正常",
            None,
            false,
        );
    }
    item("project_state_integrity","error","Codex 项目索引异常",Some(format!("{}；roots={} active_roots={} hints={}，线程库可重建 roots={} hints={} active_threads={} cwd_backfill={}",i.issue.as_deref().unwrap_or("项目索引需要修复"),i.roots,i.active_roots,i.hints,i.db_roots,i.db_hints,i.active_threads,i.cwd_backfill)),true)
}
/// Closed ownership predicate from 0x1003d4efc, not inferred from missing config definitions.
pub fn is_aimami_provider(id: &str) -> bool {
    id == "aimami_relay"
        || id.starts_with("aimami_relay_")
        || id == "aimami_codex_router"
        || id == "aimai1"
}
/// `valid_provider_ids` is an explicit typed snapshot; callers must not replace it with all config keys.
pub fn check_db_orphan_providers(
    dbs: &[PathBuf],
    valid_provider_ids: &BTreeSet<String>,
) -> DiagnosticItem {
    if dbs.is_empty() {
        return item(
            "db_orphan_providers",
            "ok",
            "线程数据（数据库不存在，跳过）",
            None,
            false,
        );
    }
    let rows = match collect_diagnostic_main_threads(dbs) {
        Ok(r) => r,
        Err(e) => {
            return item(
                "db_orphan_providers",
                "warning",
                "无法完整读取线程数据库",
                Some(format!("Operation failed: {e}")),
                false,
            )
        }
    };
    let ids: BTreeSet<_> = rows
        .iter()
        .filter(|r| {
            r.provider
                .as_deref()
                .is_some_and(|p| is_aimami_provider(p) && !valid_provider_ids.contains(p))
        })
        .map(|r| r.id.as_str())
        .collect();
    if ids.is_empty() {
        return item("db_orphan_providers","ok","未发现失效的旧版工具线程 provider",Some("仅核对旧版工具管理的引用；其他原生、企业或自定义 provider 保持原样，不由本地定义缺失推断无效。".into()),false);
    }
    let examples = ids.iter().take(5).copied().collect::<Vec<_>>().join("、");
    item("db_orphan_providers","error",format!("{} 条主线程引用了失效的旧版工具 provider",ids.len()),Some(format!("这些主线程在 Codex 中可能无法正常对话。修复会同步接管 DB 与 Rollout，保留仍有效的官方模型，否则回落当前官方默认模型。仅修复旧版工具残留；混合外部归属时停止，未确认的第三方任务不接管。示例：{examples}")),true)
}
/// Resolve only representation and metadata. The native diagnostic does not decompress/read rollout content here.
/// 0x1008b01e8..0x1008b06c0: prefer plain JSONL even when given a .jsonl.zst path;
/// try .jsonl.zst only on NotFound, and stop on any other IO error/non-file.
pub fn resolve_rollout(path: &std::path::Path) -> std::io::Result<Option<PathBuf>> {
    let plain = if path.to_string_lossy().ends_with(".jsonl.zst") {
        path.with_extension("")
    } else {
        path.to_path_buf()
    };
    let mut candidates = vec![plain.clone()];
    // Codex 压缩历史后会把磁盘文件改成 .jsonl.zst，但 threads.rollout_path 仍可能是
    // 原始的 .json 或 .jsonl。这里按真实文件系统补齐后缀，而不是把记录当作缺失。
    if plain.extension().is_some_and(|e| e == "jsonl") {
        candidates.push(plain.with_extension("jsonl.zst"));
    }
    if plain.extension().is_some_and(|e| e == "json") {
        candidates.push(plain.with_extension("jsonl"));
        candidates.push(plain.with_extension("jsonl.zst"));
        candidates.push(plain.with_extension("json.zst"));
    }
    for p in candidates {
        match std::fs::metadata(&p) {
            Ok(m) if m.is_file() => return Ok(Some(p)),
            Ok(_) => {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidInput,
                    "rollout is not a file",
                ))
            }
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => (),
            Err(e) => return Err(e),
        }
    }
    Ok(None)
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProviderConvergenceTarget {
    pub id: String,
    pub provider: String,
    pub rollout_path: Option<String>,
    pub model: Option<String>,
}
/// 0x1008b799c: only the native `openai` spelling is ASCII-case-insensitive.
pub fn is_router_or_openai_provider(p: &str) -> bool {
    is_aimami_provider(p) || p.eq_ignore_ascii_case("openai")
}
/// 0x1008c8424 plus 0x10050e2f4: SQL → source/provider filter → blank model to None.
pub fn collect_provider_convergence_targets(
    conn: &Connection,
) -> Result<Vec<ProviderConvergenceTarget>, String> {
    let mut st = conn
        .prepare("PRAGMA table_info(threads)")
        .map_err(|e| format!("sqlite pragma threads: {e}"))?;
    let cols = st
        .query_map([], |r| r.get::<_, String>(1))
        .map_err(|e| format!("sqlite read columns: {e}"))?
        .collect::<Result<BTreeSet<_>, _>>()
        .map_err(|e| format!("sqlite collect columns: {e}"))?;
    let sql=format!("SELECT id, rollout_path, model_provider, COALESCE(source, ''), {} FROM threads WHERE COALESCE(archived, 0) = 0 ORDER BY id",if cols.contains("model") {"model"} else {"NULL"});
    let mut st = conn
        .prepare(&sql)
        .map_err(|e| format!("prepare thread convergence query: {e}"))?;
    let rows = st
        .query_map([], |r| {
            Ok((
                r.get::<_, String>(0)?,
                r.get::<_, Option<String>>(1)?,
                r.get::<_, String>(2)?,
                r.get::<_, String>(3)?,
                r.get::<_, Option<String>>(4)?,
            ))
        })
        .map_err(|e| format!("query thread convergence rows: {e}"))?;
    let mut result = vec![];
    for r in rows {
        let (id, rollout_path, provider, source, model) =
            r.map_err(|e| format!("read thread convergence row: {e}"))?;
        if source.contains("\"subagent\"") || !is_router_or_openai_provider(&provider) {
            continue;
        }
        result.push(ProviderConvergenceTarget {
            id,
            provider,
            rollout_path,
            model: model.filter(|m| !m.trim().is_empty()),
        });
    }
    Ok(result)
}
/// 0x1008b7540 + 0x1008ba190: find first valid session_meta; only that record may prove identity.
pub fn rollout_file_belongs_to_thread(path: &std::path::Path, id: &str) -> bool {
    first_session_meta(path)
        .is_some_and(|v| v.pointer("/payload/id").and_then(|v| v.as_str()) == Some(id))
}
/// 0x1008b62dc: depth-first directory stack, ignore unreadable entries and symlinks,
/// match the exact recorded file name then prove thread identity from session_meta.
pub fn find_trusted_rollout_backup(
    root: &std::path::Path,
    id: &str,
    recorded_path: &std::path::Path,
) -> Option<PathBuf> {
    let name = recorded_path.file_name()?;
    let mut stack = vec![root.to_path_buf()];
    while let Some(dir) = stack.pop() {
        let Ok(entries) = std::fs::read_dir(dir) else {
            continue;
        };
        for entry in entries.flatten() {
            let Ok(kind) = entry.file_type() else {
                continue;
            };
            let p = entry.path();
            if kind.is_dir() {
                stack.push(p);
            } else if kind.is_file()
                && entry.file_name() == name
                && rollout_file_belongs_to_thread(&p, id)
            {
                return Some(p);
            }
        }
    }
    None
}
/// 0x1008d6c44: metadata failures for DB paths are skipped; opened DB/row/rollout errors are accumulated.
pub fn inspect_missing_rollouts_for_convergence(
    dbs: &[PathBuf],
    backup_root: &std::path::Path,
) -> MissingRolloutsInspection {
    let mut i = MissingRolloutsInspection::default();
    for db in dbs {
        if std::fs::metadata(db).is_err() {
            continue;
        }
        let rows = (|| -> Result<Vec<ProviderConvergenceTarget>, String> {
            let conn = Connection::open_with_flags(
                db,
                OpenFlags::SQLITE_OPEN_READ_ONLY | OpenFlags::SQLITE_OPEN_NO_MUTEX,
            )
            .map_err(|e| format!("sqlite open readonly: {e}"))?;
            conn.busy_timeout(Duration::from_secs(5))
                .map_err(|e| format!("sqlite open readonly: {e}"))?;
            collect_provider_convergence_targets(&conn)
        })();
        let rows = match rows {
            Ok(rows) => rows,
            Err(e) => {
                i.errors
                    .push(format!("{}: Operation failed: {e}", db.display()));
                continue;
            }
        };
        i.total += rows.len();
        for row in rows {
            let example = match row.rollout_path {
                None => {
                    i.missing += 1;
                    i.unrecoverable += 1;
                    Some(format!("{}: rollout path missing", row.id))
                }
                Some(p) => match resolve_rollout(std::path::Path::new(&p)) {
                    Ok(Some(_)) => None,
                    Ok(None) => {
                        i.missing += 1;
                        if find_trusted_rollout_backup(
                            backup_root,
                            &row.id,
                            std::path::Path::new(&p),
                        )
                        .is_some()
                        {
                            i.recoverable += 1;
                            Some(format!("{}: missing but backup exists", row.id))
                        } else {
                            i.unrecoverable += 1;
                            Some(format!("{}: missing and no trusted backup", row.id))
                        }
                    }
                    Err(e) => {
                        i.errors.push(format!("{}: {e}", row.id));
                        None
                    }
                },
            };
            if let Some(e) = example {
                if i.examples.len() < 3 {
                    i.examples.push(e);
                }
            }
        }
    }
    i
}
/// 0x10039264c: valid-set composition. Config section extraction is supplied by the
/// separately recovered structural scanner; quarantine/config read errors do not invalidate the base set.
pub fn diagnostic_valid_provider_ids(
    base: &BTreeSet<String>,
    router_enabled: bool,
    quarantined: Result<&BTreeSet<String>, &str>,
    config_sections: Result<&BTreeSet<String>, &str>,
) -> BTreeSet<String> {
    let mut ids = base.clone();
    ids.insert("openai".into());
    ids.insert("OpenAI".into());
    if router_enabled {
        ids.insert("aimai1".into());
        if let Ok(q) = quarantined {
            ids.extend(q.iter().cloned());
        }
    }
    if let Ok(c) = config_sections {
        ids.extend(c.iter().cloned());
    }
    ids
}
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ProjectDbSnapshot {
    pub errors: Vec<String>,
    pub roots: usize,
    pub hints: usize,
    pub active_threads: usize,
    pub cwd_backfill: usize,
}
/// 0x1003b0db0: pure string normalization, deliberately no filesystem canonicalization.
pub fn normalize_codex_path(path: &str) -> String {
    path.trim()
        .strip_prefix("\\\\?\\")
        .unwrap_or(path.trim())
        .trim_end_matches(['\\', '/'])
        .to_string()
}
fn normalized_array_count(v: &serde_json::Value, key: &str) -> usize {
    v.get(key)
        .and_then(|x| x.as_array())
        .map(|a| {
            a.iter()
                .filter_map(|x| x.as_str())
                .map(normalize_codex_path)
                .filter(|x| !x.is_empty())
                .count()
        })
        .unwrap_or(0)
}
/// Project JSON producer and decision tree at 0x1003b5b34. DB snapshot is explicit because
/// active-DB discovery and rollout CWD backfill have distinct evidence/recovery boundaries.
pub fn inspect_project_state_with_db(
    state_path: &std::path::Path,
    db: ProjectDbSnapshot,
) -> ProjectStateInspection {
    let mut i = ProjectStateInspection {
        db_errors: db.errors.clone(),
        db_roots: db.roots,
        db_hints: db.hints,
        active_threads: db.active_threads,
        cwd_backfill: db.cwd_backfill,
        ..Default::default()
    };
    let exists = std::fs::metadata(state_path).is_ok();
    let mut object = None;
    let mut parse_failed = false;
    if exists {
        match std::fs::read_to_string(state_path) {
            Err(e) => i.read_error = Some(e.to_string()),
            Ok(s) => match serde_json::from_str::<serde_json::Value>(&s) {
                Err(_) => parse_failed = true,
                Ok(v) if !v.is_object() => parse_failed = true,
                Ok(v) => object = Some(v),
            },
        }
    }
    if let Some(v) = object.as_ref() {
        i.roots = normalized_array_count(v, "electron-saved-workspace-roots");
        i.active_roots = normalized_array_count(v, "active-workspace-roots");
        i.hints = v
            .get("thread-workspace-root-hints")
            .and_then(|v| v.as_object())
            .map(|v| v.len())
            .unwrap_or(0);
        // project-order and projectless-thread-ids are also inspected by native code but
        // neither contributes to the exposed roots/active_roots/hints summary.
        let _order = normalized_array_count(v, "project-order");
    }
    // Original read_error priority is applied by the diagnostic wrapper. Native producer
    // also suppresses repairs if DB collection is incomplete.
    if i.read_error.is_none() && db.errors.is_empty() {
        i.issue = if exists && parse_failed {
            Some(".codex-global-state.json 无法解析".into())
        } else if i.roots == 0 && db.roots > 0 {
            Some("Codex 项目入口为空，但线程库存在项目 cwd".into())
        } else if db.cwd_backfill > 0 {
            Some("线程库缺失项目 cwd，但 rollout 可回填真实项目路径".into())
        } else if i.hints == 0 && db.hints > 0 && (i.roots > 0 || db.roots > 0) {
            Some("Codex 线程到项目的映射为空".into())
        } else {
            None
        };
        i.needs_repair = i.issue.is_some();
    }
    i
}
/// End-to-end config half of diagnostic_valid_provider_ids: unreadable UTF8/config is ignored,
/// while syntactically incomplete TOML can still contribute structural provider headers.
pub fn diagnostic_valid_provider_ids_from_sources(
    base: &BTreeSet<String>,
    router_enabled: bool,
    quarantined: Result<&BTreeSet<String>, &str>,
    config_path: &std::path::Path,
) -> BTreeSet<String> {
    let sections = std::fs::read_to_string(config_path).map(|text| {
        crate::config::diagnostic_config_provider_ids(&text)
            .into_iter()
            .collect::<BTreeSet<_>>()
    });
    diagnostic_valid_provider_ids(
        base,
        router_enabled,
        quarantined,
        sections.as_ref().map_err(|_| "config read failed"),
    )
}
fn first_session_meta(path: &std::path::Path) -> Option<serde_json::Value> {
    if let Some(observed) = observation::read(path) {
        return observed.ok()?.first_meta.clone();
    }
    use std::io::{BufRead, BufReader};
    let p = resolve_rollout(path).ok()??;
    let f = std::fs::File::open(&p).ok()?;
    let mut reader: Box<dyn BufRead> = if p.to_string_lossy().ends_with(".jsonl.zst") {
        Box::new(BufReader::new(zstd::stream::read::Decoder::new(f).ok()?))
    } else {
        Box::new(BufReader::new(f))
    };
    let mut buf = vec![];
    loop {
        buf.clear();
        if reader.read_until(b'\n', &mut buf).ok()? == 0 {
            return None;
        };
        if buf.last() == Some(&b'\n') {
            buf.pop();
        }
        if buf.last() == Some(&b'\r') {
            buf.pop();
        }
        let Ok(s) = std::str::from_utf8(&buf) else {
            continue;
        };
        let Ok(v) = serde_json::from_str::<serde_json::Value>(s) else {
            continue;
        };
        if v.get("type").and_then(|x| x.as_str()) == Some("session_meta") {
            return Some(v);
        }
    }
}
/// 0x1003b2284: recognize generated Documents/Codex/YYYY-MM-DD path components.
pub fn is_codex_generated_projectless_dir(path: &str) -> bool {
    let parts: Vec<_> = path.split(['\\', '/']).filter(|p| !p.is_empty()).collect();
    parts.windows(3).any(|w| {
        w[0].eq_ignore_ascii_case("documents") && w[1].eq_ignore_ascii_case("codex") && {
            let b = w[2].as_bytes();
            b.len() == 10
                && b[4] == b'-'
                && b[7] == b'-'
                && b.iter()
                    .enumerate()
                    .all(|(i, c)| i == 4 || i == 7 || c.is_ascii_digit())
        }
    })
}
/// Native 0x1003b1444. Requires id/cwd/rollout_path; row errors fail the query;
/// per-rollout IO/JSON/identity failures skip only that candidate.
pub fn collect_cwd_backfill_candidates(
    conn: &Connection,
    columns: &BTreeSet<String>,
    projectless_ids: &BTreeSet<String>,
) -> rusqlite::Result<Vec<(String, String)>> {
    if !["id", "cwd", "rollout_path"]
        .iter()
        .all(|c| columns.contains(*c))
    {
        return Ok(vec![]);
    }
    let active = if columns.contains("archived") {
        "COALESCE(archived, 0) = 0"
    } else {
        "1 = 1"
    };
    let source = if columns.contains("source") {
        "COALESCE(source, '')"
    } else {
        "''"
    };
    let sql=format!("SELECT id, rollout_path, {source} FROM threads WHERE {active} AND COALESCE(cwd, '') = '' AND COALESCE(rollout_path, '') != ''");
    let mut st = conn.prepare(&sql)?;
    let rows = st.query_map([], |r| {
        Ok((
            r.get::<_, String>(0)?,
            r.get::<_, String>(1)?,
            r.get::<_, String>(2)?,
        ))
    })?;
    let mut out = vec![];
    for r in rows {
        let (id, p, source) = r?;
        if projectless_ids.contains(&id) || source.contains("\"subagent\"") {
            continue;
        }
        let Some(v) = first_session_meta(std::path::Path::new(&p)) else {
            continue;
        };
        if v.pointer("/payload/id").and_then(|v| v.as_str()) != Some(&id) {
            continue;
        }
        let Some(cwd) = v.pointer("/payload/cwd").and_then(|v| v.as_str()) else {
            continue;
        };
        let cwd = normalize_codex_path(cwd);
        if cwd.is_empty() || is_codex_generated_projectless_dir(&cwd) {
            continue;
        }
        out.push((id, cwd));
    }
    Ok(out)
}
/// Native project DB collector 0x1003b25a4. Selected DB is typed input from path discovery;
/// a missing/unstatable selected path is the native no-active-DB branch.
pub fn collect_project_state_from_db(
    active_db: Option<&std::path::Path>,
    projectless_ids: &BTreeSet<String>,
) -> ProjectDbSnapshot {
    use rusqlite::types::ValueRef;
    let Some(db) = active_db.filter(|p| std::fs::metadata(p).is_ok()) else {
        return ProjectDbSnapshot::default();
    };
    let conn = match Connection::open_with_flags(
        db,
        OpenFlags::SQLITE_OPEN_READ_ONLY | OpenFlags::SQLITE_OPEN_NO_MUTEX,
    ) {
        Ok(c) => c,
        Err(e) => {
            return ProjectDbSnapshot {
                errors: vec![format!("{}: sqlite open readonly: {e}", db.display())],
                ..Default::default()
            }
        }
    };
    let _ = conn.busy_timeout(Duration::from_millis(500));
    let result = (|| -> rusqlite::Result<ProjectDbSnapshot> {
        let mut st = conn.prepare("PRAGMA table_info(threads)")?;
        let columns = st
            .query_map([], |r| r.get::<_, String>(1))?
            .collect::<rusqlite::Result<BTreeSet<_>>>()?;
        if !columns.contains("id") || !columns.contains("cwd") {
            return Ok(ProjectDbSnapshot::default());
        }
        let active = if columns.contains("archived") {
            "COALESCE(archived, 0) = 0"
        } else {
            "1 = 1"
        };
        let order = if columns.contains("updated_at_ms") {
            "COALESCE(updated_at_ms, 0)"
        } else if columns.contains("updated_at") {
            "COALESCE(updated_at, 0)"
        } else {
            "0"
        };
        let count = conn
            .query_row(
                &format!("SELECT COUNT(*) FROM threads WHERE {active}"),
                [],
                |r| r.get::<_, i64>(0),
            )
            .unwrap_or(0)
            .max(0) as usize;
        let mut st=conn.prepare(&format!("SELECT id, cwd FROM threads WHERE {active} AND COALESCE(cwd, '') != '' ORDER BY {order} DESC"))?;
        let mut rows = st.query([])?;
        let mut roots = BTreeSet::new();
        let mut hints = std::collections::BTreeMap::new();
        while let Some(row) = rows.next()? {
            let id = row.get::<_, String>(0)?;
            let cwd = match row.get_ref(1)? {
                ValueRef::Text(b) | ValueRef::Blob(b) => match std::str::from_utf8(b) {
                    Ok(s) => normalize_codex_path(s),
                    Err(_) => continue,
                },
                _ => continue,
            };
            if cwd.is_empty() {
                continue;
            }
            hints.insert(id, cwd.clone());
            if !is_codex_generated_projectless_dir(&cwd) {
                roots.insert(cwd);
            }
        }
        let backfill = collect_cwd_backfill_candidates(&conn, &columns, projectless_ids)
            .map(|v| v.len())
            .unwrap_or(0);
        Ok(ProjectDbSnapshot {
            roots: roots.len(),
            hints: hints.len(),
            active_threads: count,
            cwd_backfill: backfill,
            errors: vec![],
        })
    })();
    result.unwrap_or_else(|e| ProjectDbSnapshot {
        errors: vec![format!("{}: {e}", db.display())],
        ..Default::default()
    })
}
/// End-to-end project diagnostic using an explicitly selected active DB and state file.
pub fn inspect_project_state(
    state_path: &std::path::Path,
    active_db: Option<&std::path::Path>,
) -> ProjectStateInspection {
    let projectless = std::fs::read_to_string(state_path)
        .ok()
        .and_then(|s| serde_json::from_str::<serde_json::Value>(&s).ok())
        .and_then(|v| {
            v.get("projectless-thread-ids")
                .and_then(|v| v.as_array())
                .map(|a| {
                    a.iter()
                        .filter_map(|x| x.as_str().map(str::to_owned))
                        .collect::<BTreeSet<_>>()
                })
        })
        .unwrap_or_default();
    inspect_project_state_with_db(
        state_path,
        collect_project_state_from_db(active_db, &projectless),
    )
}

pub mod consistency;
pub mod history;
#[cfg(test)]
mod history_tests;
pub mod observation;
#[cfg(test)]
mod observation_tests;
pub mod project;
pub mod repair;
