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
/// Intentional replacement for the old guessed `::`/blank/allowlist predicate.
/// The supplied 0x1005365ec export only shows a Vec filter calling unresolved
/// &mut_F::call_mut. Neither it nor the model-set fold proves that closure.
/// Expose collection coverage without classifying any model as invalid or valid.
fn native_off_observation(rows: &[core::MainThread], router_enabled: bool) -> DiagnosticItem {
    if router_enabled {
        return item(
            "native_off_invalid_openai_thread_models",
            "ok",
            "官方直连线程模型（路由启用，跳过）",
            "路由启用时不执行NativeOff模型检查。".into(),
        );
    }
    item("native_off_invalid_openai_thread_models", "warning", "官方直连线程模型规则尚未恢复",
        format!("unknown：已从私有索引快照读取 {} 条活跃主线程；现有离线反编译未恢复原版筛选closure，无法确认官方模型有效集合及空白规则。未按模型前缀、双冒号或空白近似判错，未标为健康，不提供自动修复。", rows.len()))
}
pub(super) fn inspect(ctx: &Context) -> Vec<DiagnosticItem> {
    inspect_with_limits(ctx, HistoryLimits::default())
}
/// Injectable thread window and decompressed-byte/line/drain limits for synthetic tests.
pub(super) fn inspect_with_limits(ctx: &Context, limits: HistoryLimits) -> Vec<DiagnosticItem> {
    let rows = match rows(ctx) {
        Ok(rows) => rows,
        Err(_) => {
            return vec![
                item(
                    "thread_history_integrity",
                    "warning",
                    "线程历史结构自洽",
                    "无法建立完整的私有线程索引快照，历史覆盖度未知；未标为健康。".into(),
                ),
                item(
                    "native_off_invalid_openai_thread_models",
                    "warning",
                    "官方直连线程模型规则尚未恢复",
                    "unknown：线程快照不可用，原版模型筛选closure亦未恢复；未猜测规则。".into(),
                ),
            ]
        }
    };
    let h = history::inspect_thread_history(
        &rows,
        &[
            ctx.home.join("sessions"),
            ctx.home.join("archived_sessions"),
        ],
        limits,
    );
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
    } else {
        "线程历史结构自洽"
    };
    let history = item("thread_history_integrity", status, history_label,
        format!("活跃主线程 {}；窗口 {}；已扫描 {}；完整 {}；部分 {}；无路径 {}；历史缺失 {}；JSON/UTF8/ordinal/base结构异常 {}；继承引用缺失 {}；超长行 {}；读取错误 {}；已读解压字节 {}。仅检查以换行结束的记录；继承引用按sessions/archived_sessions深度小于7的rollout文件名UUID索引核对，非内容身份验证；目录不可读会被原版索引忽略。仅观察，不自动修复。",
            h.total, h.selected, h.scanned, h.complete, h.partial, h.missing_paths, h.missing_rollouts,
            h.structural_errors, h.missing_base_refs, h.oversized_lines, h.errors.len(), h.bytes_read));
    let model = match inspect::config(ctx).and_then(|c| residual_cleanup::route_enabled(ctx, &c)) {
        Ok(enabled) => native_off_observation(&rows, enabled),
        Err(_) => item(
            "native_off_invalid_openai_thread_models",
            "warning",
            "官方直连线程模型规则尚未恢复",
            "unknown：无法确认路由状态，且原版模型筛选closure未恢复；未标为健康。".into(),
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
    fn private_snapshot_relative_paths_and_honest_unknown_preserve_source() {
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
        assert_eq!(items[1].status, "warning");
        assert!(items[1].detail.contains("unknown："));
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
    fn no_guessed_model_predicate_and_router_skip() {
        assert_eq!(native_off_observation(&[], false).status, "warning");
        assert_eq!(native_off_observation(&[], true).status, "ok");
    }
}
