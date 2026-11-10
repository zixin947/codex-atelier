//! Explicit, single-use authorization for bounded directory access probes.
use super::{access_checks, now_ms, Context, DiagnosticFixResult, DiagnosticRepairPlan, Result};
use std::path::PathBuf;

#[derive(Debug)]
pub(super) struct ReviewedProbe {
    pub created_at: u64,
    home: PathBuf,
    app_root: PathBuf,
    prepared: access_checks::PreparedDirectoryProbe,
    pub view: DiagnosticRepairPlan,
}

pub(super) fn preview(ctx: &Context) -> Result<ReviewedProbe> {
    let prepared =
        access_checks::prepare_directory_probe(&access_checks::native_directories(&ctx.home))
            .map_err(|e| format!("无法准备目录检查：{e}"))?;
    let view = DiagnosticRepairPlan {
        ticket: uuid::Uuid::new_v4().to_string(),
        item_ids: vec!["codex_home_writable".into()],
        title: "目录写入检查预览".into(),
        changes: prepared
            .directories()
            .iter()
            .map(|d| {
                if d.exists {
                    format!(
                        "{}：独占创建小型临时文件，写入并同步，随后核对身份并删除",
                        d.path.display()
                    )
                } else {
                    format!("{}：目录不存在，仅跳过，不创建目录", d.path.display())
                }
            })
            .collect(),
        warnings: vec![
            "这是实际写入检查，不是普通只读诊断；仅操作随机独占的临时检查文件。".into(),
            "不修改配置、凭据或会话，不打开源数据库读写，不关闭或启动 Desktop。".into(),
            "写入、同步或清理失败会单独报告；若无法安全清理，将列出剩余检查文件路径。".into(),
        ],
        requires_desktop_closed: false,
    };
    Ok(ReviewedProbe {
        created_at: now_ms(),
        home: ctx.home.clone(),
        app_root: ctx.app_root.clone(),
        prepared,
        view,
    })
}

pub(super) fn apply(ctx: &Context, reviewed: ReviewedProbe) -> Result<DiagnosticFixResult> {
    if now_ms().saturating_sub(reviewed.created_at) >= 300_000 {
        return Err("目录检查预览已过期，请重新预览".into());
    }
    if ctx.home != reviewed.home || ctx.app_root != reviewed.app_root {
        return Err("目录检查上下文已变化，请重新预览".into());
    }
    // Acquire the read-only report first, so a later scan failure cannot hide probe cleanup errors.
    let mut report = super::native_manager::report(ctx)?;
    let observation = access_checks::execute_directory_probe(&reviewed.prepared)
        .map_err(|e| format!("目录检查未执行：{e}"))?;
    let mut details = vec!["仅检查目录临时文件写入与清理；源数据库读写能力未验证。".into()];
    let mut errors = 0;
    let mut checked = 0;
    for result in &observation.results {
        if result.status == "error" || result.cleanup_error.is_some() {
            errors += 1;
        }
        if result.status == "ok" {
            checked += 1;
        }
        details.push(format!(
            "{}：{} — {}",
            result.path.display(),
            result.status,
            result.detail
        ));
        if let Some(error) = &result.cleanup_error {
            details.push(format!(
                "清理未完成：{error}；检查文件：{}",
                result
                    .probe_path
                    .as_ref()
                    .map(|p| p.display().to_string())
                    .unwrap_or_else(|| "路径未确认".into())
            ));
        }
    }
    report.checked_at = now_ms();
    if let Some(item) = report
        .items
        .iter_mut()
        .find(|i| i.id == "codex_home_writable")
    {
        if errors > 0 {
            item.status = "error".into();
        }
        // Preserve metadata/space failures. A successful probe does not establish source DB RW.
        if errors == 0 && checked == 0 {
            item.status = "warning".into();
        }
        item.detail = format!("{}；显式临时文件检查：{checked} 个目录写入与清理完成，{errors} 项失败；源数据库读写仍未验证。{}", item.detail, details.iter().skip(1).cloned().collect::<Vec<_>>().join("；"));
    }
    report.has_issues = report
        .items
        .iter()
        .any(|i| matches!(i.status.as_str(), "warning" | "error" | "warning"));
    Ok(DiagnosticFixResult {
        fixed_ids: vec![],
        failed_ids: if errors > 0 {
            vec!["codex_home_writable".into()]
        } else {
            vec![]
        },
        details,
        backup_path: None,
        report,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    fn context(root: &std::path::Path) -> Context {
        Context {
            home: root.join("home"),
            app_root: root.join("app"),
            desktop_running: false,
            runtime: None,
            proxy_handle: None,
            official_catalog: None,
            working_directory: None,
            sqlite_home_env: None,
        }
    }
    #[test]
    fn confirmed_probe_preserves_configuration_and_does_not_create_missing_directories() {
        let temp =
            std::env::temp_dir().join(format!("atelier-probe-execute-{}", uuid::Uuid::new_v4()));
        std::fs::create_dir(&temp).unwrap();
        let root = temp.canonicalize().unwrap();
        let ctx = context(&root);
        std::fs::create_dir(&ctx.home).unwrap();
        let original = b"# synthetic fixture\nmodel='fixture'\n";
        std::fs::write(ctx.config(), original).unwrap();
        let ticket = preview(&ctx).unwrap();
        let result = apply(&ctx, ticket).unwrap();
        assert!(result.failed_ids.is_empty());
        assert!(result.fixed_ids.is_empty());
        assert!(result
            .details
            .iter()
            .any(|d| d.contains("源数据库读写能力未验证")));
        assert_eq!(std::fs::read(ctx.config()).unwrap(), original);
        assert!(!ctx.home.join("sessions").exists());
        assert!(!ctx.home.join("codexmate").exists());
        assert_eq!(std::fs::read_dir(&ctx.home).unwrap().count(), 1);
        std::fs::remove_dir_all(&root).unwrap();
    }
    #[test]
    fn preview_is_read_only_and_expiry_or_context_change_cannot_write() {
        let temp =
            std::env::temp_dir().join(format!("atelier-probe-ticket-{}", uuid::Uuid::new_v4()));
        std::fs::create_dir(&temp).unwrap();
        let root = temp.canonicalize().unwrap();
        let ctx = context(&root);
        std::fs::create_dir(&ctx.home).unwrap();
        let mut ticket = preview(&ctx).unwrap();
        assert_eq!(std::fs::read_dir(&ctx.home).unwrap().count(), 0);
        assert!(!ctx.home.join("sessions").exists());
        ticket.created_at = now_ms().saturating_sub(300_001);
        assert!(apply(&ctx, ticket).unwrap_err().contains("过期"));
        let ticket = preview(&ctx).unwrap();
        let mut changed = ctx.clone();
        changed.app_root = root.join("other");
        assert!(apply(&changed, ticket).unwrap_err().contains("上下文"));
        assert_eq!(std::fs::read_dir(&ctx.home).unwrap().count(), 0);
        std::fs::remove_dir_all(&root).unwrap();
    }
}
