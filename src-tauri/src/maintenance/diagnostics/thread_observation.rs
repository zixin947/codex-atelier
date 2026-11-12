//! Observation only: no planner/apply path and no source SQLite connection.
use super::*;
use atelier_maintenance_core::{
    state_db,
    threads::{
        self as core,
        history::{self, HistoryLimits},
    },
};

fn item(id: &str, status: &str, label: &str, detail: String) -> DiagnosticItem {
    DiagnosticItem {
        id: id.into(),
        group: "runtime".into(),
        status: status.into(),
        label: label.into(),
        detail,
        fixable: false,
    }
}
fn rows(ctx: &Context) -> Result<Vec<core::MainThread>> {
    let raw = match std::fs::symlink_metadata(ctx.config()) {
        Ok(_) => Some(
            String::from_utf8(read_bounded(&ctx.config(), 1024 * 1024)?)
                .map_err(|_| "配置不是UTF-8")?,
        ),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => None,
        Err(_) => return Err("无法读取线程数据库配置".into()),
    };
    let configured = state_db::configured_sqlite_home(
        raw.as_deref(),
        ctx.sqlite_home_env.as_deref(),
        ctx.working_directory.as_deref(),
    );
    let directories = configured
        .map(|p| vec![p])
        .unwrap_or_else(|| vec![ctx.home.join("sqlite"), ctx.home.clone()]);
    let mut rows = vec![];
    let mut seen = std::collections::BTreeSet::new();
    for dir in directories {
        let Some(db) = state_db::highest_state_db(&dir) else {
            continue;
        };
        if !seen.insert(db.clone()) {
            continue;
        }
        let mut found = snapshot::with_observed_path(&db, |copy, _| {
            core::collect_diagnostic_main_threads(&[copy.to_path_buf()])
        })?;
        for row in &mut found {
            row.db = db.clone();
            if let Some(path) = row.rollout_path.as_mut() {
                if !Path::new(path).is_absolute() {
                    *path = ctx.home.join(&*path).to_string_lossy().into_owned();
                }
            }
        }
        rows.extend(found);
    }
    Ok(rows)
}
/// AiMaMi arm64 SHA256 b2a4b1b2b4babd2518e02c0f0d6203ad19e779909a47b3b643844ef403fe95de:
/// 0x100536634 calls 0x1003d4c94 (previously unresolved FnMut).
/// That predicate requires ASCII-case-insensitive openai, Some(nonblank trimmed model),
/// then exact aimami_relay, prefix aimami_relay_, or membership in the relay model set.
/// 0x100803368 -> 0x10050d744 builds that set from exposed_models, trimming nonempty
/// model names and formatting provider.id::model (format bytes @0x10151cfd8: c0 02 3a 3a c0 00).
/// This is NOT an official-model allowlist: absence from model/list proves nothing.
fn relay_models(ctx: &Context) -> Result<std::collections::BTreeSet<String>> {
    use atelier_maintenance_core::catalog::state::RelayStateDocument;
    let path = ctx.home.join("codexmate/relay/state.json");
    let bytes = match std::fs::symlink_metadata(&path) {
        Ok(_) => read_bounded(&path, 8 * 1024 * 1024)?,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(Default::default()),
        Err(_) => return Err("无法读取原生路由模型来源".into()),
    };
    let inputs = RelayStateDocument::from_json(&bytes)
        .map_err(err)?
        .catalog_inputs()
        .map_err(err)?;
    Ok(inputs
        .providers
        .iter()
        .flat_map(|p| {
            p.exposed_models()
                .into_iter()
                .filter_map(|m| {
                    let model = m.model.trim();
                    (!model.is_empty()).then(|| format!("{}::{model}", p.id))
                })
                .collect::<Vec<_>>()
        })
        .collect())
}
fn native_off_observation(
    rows: &[core::MainThread],
    router_enabled: bool,
    models: Result<std::collections::BTreeSet<String>>,
) -> DiagnosticItem {
    const ID: &str = "native_off_invalid_openai_thread_models";
    if router_enabled {
        return item(
            ID,
            "ok",
            "NativeOff 官方线程模型检查（路由启用中，跳过）",
            "路由启用时不执行NativeOff模型检查。".into(),
        );
    }
    let models = match models {
        Ok(models) => models,
        Err(_) => {
            return item(
                ID,
                "warning",
                "无法完整读取 NativeOff 官方线程模型",
                "中转模型来源不可读，覆盖度未知；未标为健康，不提供自动修复。".into(),
            )
        }
    };
    if rows.iter().any(|row| {
        !row.has_model_column
            && row
                .provider
                .as_deref()
                .is_some_and(|p| p.eq_ignore_ascii_case("openai"))
    }) {
        return item(
            ID,
            "warning",
            "无法完整读取 NativeOff 官方线程模型",
            "线程数据库缺少model列，不能确认中转模型残留；未标为健康。".into(),
        );
    }
    let invalid = rows
        .iter()
        .filter(|row| {
            row.provider
                .as_deref()
                .is_some_and(|p| p.eq_ignore_ascii_case("openai"))
                && row.model.as_deref().is_some_and(|m| {
                    let m = m.trim();
                    !m.is_empty()
                        && (m == "aimami_relay"
                            || m.starts_with("aimami_relay_")
                            || models.contains(m))
                })
        })
        .count();
    item(ID, if invalid == 0 { "ok" } else { "warning" },
        if invalid == 0 { "NativeOff 官方线程模型正常" } else { "NativeOff 官方线程模型检查" },
        format!("已检查 {} 条活跃主线程；{} 条旧对话保留了中转模型选择。按原版规则检测中转残留，不将当前官方目录中未列出的历史模型判为损坏；空模型沿用原版不判异常。仅观察，不修改模型或provider。", rows.len(), invalid))
}
pub(super) fn inspect(ctx: &Context) -> Vec<DiagnosticItem> {
    inspect_mode(ctx, HistoryLimits::default(), core::observation::full_active())
}
/// Injectable thread window and decompressed-byte/line/drain limits for synthetic tests.
pub(super) fn inspect_with_limits(ctx: &Context, limits: HistoryLimits) -> Vec<DiagnosticItem> {
    inspect_mode(ctx, limits, false)
}
fn inspect_mode(ctx: &Context, limits: HistoryLimits, full: bool) -> Vec<DiagnosticItem> {
    let rows = match rows(ctx) {
        Ok(rows) => rows,
        Err(_) => {
            return vec![
                item(
                    "thread_history_integrity",
                    "warning",
                    "线程历史结构检查不完整",
                    "无法建立完整的私有线程索引快照，历史覆盖度未知；未标为健康。".into(),
                ),
                item(
                    "native_off_invalid_openai_thread_models",
                    "warning",
                    "无法完整读取 NativeOff 官方线程模型",
                    "线程快照不可用，模型检查覆盖度未知；未标为健康。".into(),
                ),
            ]
        }
    };
    let h = if full { match history::inspect_thread_history_full(&rows) {
        Ok(h) => h,
        Err(error) => return vec![item("thread_history_integrity", "warning", "线程历史结构检查不完整", error)],
    }} else { history::inspect_thread_history(
        &rows,
        &[
            ctx.home.join("sessions"),
            ctx.home.join("archived_sessions"),
        ],
        limits,
    ) };
    let status = if h.structural_errors > 0 || h.missing_base_refs > 0 {
        "error"
    } else if !h.coverage_complete() {
        "warning"
    } else {
        "ok"
    };
    // 原版两种 label：自洽 / 存在异常。此前写死「自洽」导致 error 与文案矛盾。
    let history_label = if h.structural_errors > 0 || h.missing_base_refs > 0 {
        "线程历史结构存在异常"
    } else if !h.coverage_complete() {
        "线程历史结构检查不完整"
    } else {
        "线程历史结构自洽"
    };
    let mut history = item("thread_history_integrity", status, history_label,
        format!("活跃主线程 {}；窗口 {}；已扫描 {}；完整 {}；部分 {}；无路径 {}；历史缺失 {}；JSON/UTF8/ordinal/base结构异常 {}；继承引用缺失 {}；超长行 {}；读取错误 {}；已读解压字节 {}。仅检查以换行结束的记录；继承引用按sessions/archived_sessions深度小于7的rollout文件名UUID索引核对，非内容身份验证；目录不可读会被原版索引忽略。只有备份能逐字节证明的历史损坏才提供恢复。",
            h.total, h.selected, h.scanned, h.complete, h.partial, h.missing_paths, h.missing_rollouts,
            h.structural_errors, h.missing_base_refs, h.oversized_lines, h.errors.len(), h.bytes_read));
    history.detail.push_str(&format!(" 扫描资源限制 {} 项。", h.limits.len()));
    let rollout_paths = rows.iter().filter_map(|row| row.rollout_path.as_ref())
        .map(|p| { let p = PathBuf::from(p); if p.is_absolute() { p } else { ctx.home.join(p) } }).collect::<Vec<_>>();
    super::history_recovery_workflow::annotate(&mut history, &ctx.home, &rollout_paths);
    let model = match inspect::config(ctx).and_then(|c| residual_cleanup::route_enabled(ctx, &c)) {
        Ok(enabled) => native_off_observation(
            &rows,
            enabled,
            if enabled {
                Ok(Default::default())
            } else {
                relay_models(ctx)
            },
        ),
        Err(_) => item(
            "native_off_invalid_openai_thread_models",
            "warning",
            "无法完整读取 NativeOff 官方线程模型",
            "无法确认路由状态；未标为健康。".into(),
        ),
    };
    vec![history, model]
}

#[cfg(test)]
mod tests {
    use super::*;
    struct Temp(PathBuf);
    impl Temp {
        fn new() -> Self {
            let p = std::env::temp_dir().join(format!(
                "atelier-thread-observation-{}",
                uuid::Uuid::new_v4()
            ));
            std::fs::create_dir_all(&p).unwrap();
            Self(p)
        }
        fn path(&self) -> &Path {
            &self.0
        }
    }
    impl Drop for Temp {
        fn drop(&mut self) {
            let _ = std::fs::remove_dir_all(&self.0);
        }
    }
    fn context(home: &Path) -> Context {
        Context {
            home: home.into(),
            app_root: home.join("app"),
            desktop_running: false,
            runtime: None,
            proxy_handle: None,
            official_catalog: None,
            working_directory: None,
            sqlite_home_env: None,
        }
    }
    fn fixture(home: &Path) -> rusqlite::Connection {
        std::fs::create_dir_all(home).unwrap();
        std::fs::write(home.join("config.toml"), "model_provider = 'openai'\n").unwrap();
        let c = rusqlite::Connection::open(home.join("state_5.sqlite")).unwrap();
        c.execute_batch("CREATE TABLE threads(id TEXT,rollout_path TEXT,model_provider TEXT,source TEXT,model TEXT,reasoning_effort TEXT,archived INTEGER);").unwrap();
        c
    }
    #[test]
    fn private_snapshot_relative_paths_and_unlisted_model_preserve_source() {
        let temp = Temp::new();
        let c = fixture(temp.path());
        std::fs::write(temp.path().join("fixture.jsonl"), b"{\"ordinal\":1}\n").unwrap();
        c.execute(
            "INSERT INTO threads VALUES('one','fixture.jsonl','openai','','model::literal',NULL,0)",
            [],
        )
        .unwrap();
        drop(c);
        let db = temp.path().join("state_5.sqlite");
        let before = std::fs::read(&db).unwrap();
        let items = inspect(&context(temp.path()));
        assert_eq!(items[0].status, "ok");
        assert_eq!(items[1].status, "ok");
        assert!(items[1].detail.contains("0 条旧对话"));
        assert!(items.iter().all(|i| !i.fixable));
        assert_eq!(std::fs::read(&db).unwrap(), before);
        assert!(!temp.path().join("state_5.sqlite-wal").exists());
        assert!(!temp.path().join("state_5.sqlite-shm").exists());
    }
    #[test]
    fn injected_window_and_budget_cannot_report_healthy() {
        let temp = Temp::new();
        let c = fixture(temp.path());
        for id in ["a", "b"] {
            c.execute(
                "INSERT INTO threads VALUES(?1,'fixture.jsonl','external','','x',NULL,0)",
                [id],
            )
            .unwrap();
        }
        std::fs::write(temp.path().join("fixture.jsonl"), b"{\"ordinal\":1}\n").unwrap();
        drop(c);
        let result = inspect_with_limits(
            &context(temp.path()),
            HistoryLimits {
                thread_limit: 1,
                total_bytes: 2,
                ..Default::default()
            },
        );
        assert_eq!(result[0].status, "warning");
        assert!(result[0].detail.contains("窗口 1"));
    }
    #[test]
    fn snapshot_error_is_unknown_not_zero_healthy() {
        let temp = Temp::new();
        std::fs::write(temp.path().join("state_5.sqlite"), b"not sqlite").unwrap();
        let result = inspect(&context(temp.path()));
        assert!(result.iter().all(|i| i.status == "warning" && !i.fixable));
    }
    #[test]
    fn committed_wal_is_included_without_touching_source_and_main_filter_is_shared() {
        let temp = Temp::new();
        let c = fixture(temp.path());
        c.execute_batch("PRAGMA journal_mode=WAL; PRAGMA wal_autocheckpoint=0; PRAGMA wal_checkpoint(TRUNCATE);").unwrap();
        std::fs::write(temp.path().join("fixture.jsonl"), b"{bad}\n").unwrap();
        c.execute_batch("INSERT INTO threads VALUES('external','fixture.jsonl','enterprise','','m',NULL,0); INSERT INTO threads VALUES('archived',NULL,'openai','','m',NULL,1); INSERT INTO threads VALUES('child',NULL,'openai','{\"subagent\":true}','m',NULL,0);").unwrap();
        let db = temp.path().join("state_5.sqlite");
        let wal = temp.path().join("state_5.sqlite-wal");
        let shm = temp.path().join("state_5.sqlite-shm");
        let before = [
            std::fs::read(&db).unwrap(),
            std::fs::read(&wal).unwrap(),
            std::fs::read(&shm).unwrap(),
        ];
        let items = inspect(&context(temp.path()));
        assert_eq!(items[0].status, "error");
        assert!(items[0].detail.contains("活跃主线程 1"));
        assert_eq!(std::fs::read(&db).unwrap(), before[0]);
        assert_eq!(std::fs::read(&wal).unwrap(), before[1]);
        assert_eq!(std::fs::read(&shm).unwrap(), before[2]);
    }
    #[test]
    fn missing_history_and_corrupt_structure_are_separate_without_sensitive_text() {
        let temp = Temp::new();
        let c = fixture(temp.path());
        c.execute_batch(
            "INSERT INTO threads VALUES('missing','missing.jsonl','openai','','m',NULL,0);",
        )
        .unwrap();
        let result = inspect(&context(temp.path()));
        assert_eq!(result[0].status, "warning");
        assert!(result[0].detail.contains("历史缺失 1"));
        std::fs::write(
            temp.path().join("missing.jsonl"),
            b"private secret invalid JSON\n",
        )
        .unwrap();
        let result = inspect(&context(temp.path()));
        assert_eq!(result[0].status, "error");
        assert!(!result[0].detail.contains("secret"));
    }
    #[test]
    fn recovered_native_predicate_does_not_guess_official_validity() {
        let temp = Temp::new();
        let c = fixture(temp.path());
        let cases = [
            ("openai", "aimami_relay", true),
            ("OPENAI", "  aimami_relay_custom  ", true),
            ("openai", "p::model", true),
            ("openai", "other::model", false),
            ("openai", "aimami_relayish", false),
            ("external", "aimami_relay", false),
            ("openai", "retired-official-model", false),
            ("openai", "   ", false),
            ("openai", "AIMAMI_RELAY", false),
        ];
        for (n, (provider, model, invalid)) in cases.iter().enumerate() {
            c.execute("DELETE FROM threads", []).unwrap();
            c.execute(
                "INSERT INTO threads VALUES(?1,NULL,?2,'',?3,NULL,0)",
                rusqlite::params![n.to_string(), provider, model],
            )
            .unwrap();
            let found = rows(&context(temp.path())).unwrap();
            let result = native_off_observation(&found, false, Ok(["p::model".into()].into()));
            assert_eq!(
                result.status,
                if *invalid { "warning" } else { "ok" },
                "{provider}/{model}"
            );
            assert!(!result.fixable);
        }
        c.execute("UPDATE threads SET model=NULL", []).unwrap();
        assert_eq!(
            native_off_observation(
                &rows(&context(temp.path())).unwrap(),
                false,
                Ok(Default::default())
            )
            .status,
            "ok"
        );
        assert_eq!(
            native_off_observation(&[], false, Err("bad state".into())).status,
            "warning"
        );
    }
    #[test]
    fn missing_model_column_is_incomplete_not_healthy() {
        let temp = Temp::new();
        let c = fixture(temp.path());
        c.execute(
            "INSERT INTO threads VALUES('one',NULL,'openai','',NULL,NULL,0)",
            [],
        )
        .unwrap();
        c.execute("ALTER TABLE threads DROP COLUMN model", [])
            .unwrap();
        let found = rows(&context(temp.path())).unwrap();
        assert_eq!(
            native_off_observation(&found, false, Ok(Default::default())).status,
            "warning"
        );
    }
    #[test]
    fn relay_source_is_read_once_and_invalid_state_is_unknown() {
        let temp = Temp::new();
        let ctx = context(temp.path());
        assert!(relay_models(&ctx).unwrap().is_empty());
        let p = temp.path().join("codexmate/relay/state.json");
        std::fs::create_dir_all(p.parent().unwrap()).unwrap();
        let mut state =
            atelier_maintenance_core::catalog::state::RelayStateDocument::native_default()
                .as_value()
                .clone();
        state["providers"] = serde_json::json!([{"id":"p", "name":"test", "baseUrl":"https://example.invalid", "model":" model "}]);
        std::fs::write(&p, serde_json::to_vec(&state).unwrap()).unwrap();
        assert_eq!(relay_models(&ctx).unwrap(), ["p::model".into()].into());
        std::fs::write(&p, b"invalid").unwrap();
        assert!(relay_models(&ctx).is_err());
    }
    #[test]
    fn no_guessed_model_predicate_and_router_skip() {
        assert_eq!(
            native_off_observation(&[], false, Ok(Default::default())).status,
            "ok"
        );
        assert_eq!(
            native_off_observation(&[], true, Err("unreadable".into())).status,
            "ok"
        );
    }
}
