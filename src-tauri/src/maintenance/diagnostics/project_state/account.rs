//! Read-only Desktop project compatibility. Evidence: app.asar 60656 (owner/storage),
//! 78005 (authenticated principal), and initializeProjects/migrate (host checkpoints).
//! Projects are app-server-owned, not an account-wide union of legacy cwd hints.
use super::*;
use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};
use serde_json::Value;

fn item(status: &str, detail: &str) -> DiagnosticItem {
    DiagnosticItem {
        id: "project_state_integrity".into(),
        group: "runtime".into(),
        status: status.into(),
        label: if status == "ok" {
            "Codex 项目索引正常"
        } else {
            "Codex 项目索引无法读取"
        }
        .into(),
        detail: detail.into(),
        fixable: false,
    }
}
fn owner(auth: &[u8]) -> Option<String> {
    let auth: Value = serde_json::from_slice(auth).ok()?;
    let token = auth.get("tokens")?.get("access_token")?.as_str()?;
    let payload = URL_SAFE_NO_PAD
        .decode(token.split('.').nth(1)?.trim_end_matches('='))
        .ok()?;
    let payload: Value = serde_json::from_slice(&payload).ok()?;
    let claims = payload.get("https://api.openai.com/auth")?;
    // Desktop's authenticated principal uses user_id first, not chatgpt_user_id.
    let user = claims
        .get("user_id")
        .filter(|v| !v.is_null())
        .or_else(|| claims.get("chatgpt_user_id"))?
        .as_str()?;
    let account = claims
        .get("chatgpt_account_id")
        .filter(|v| !v.is_null())
        .or_else(|| claims.get("account_id"))?
        .as_str()?;
    if user.is_empty() || account.is_empty() {
        return None;
    }
    Some(format!(
        "{:x}",
        Sha256::digest(serde_json::to_vec(&[user, account]).ok()?)
    ))
}
fn strings(v: &Value) -> bool {
    v.as_array().is_some_and(|a| a.iter().all(Value::is_string))
}
fn validate_state(state: &Value, owner: &str, home: &Path) -> Result<bool> {
    let get = |name: &str| state.get(format!("electron-owner-value:{name}:{owner}"));
    for name in ["electron-saved-workspace-roots", "active-workspace-roots"] {
        if get(name).is_some_and(|v| !strings(v)) {
            return Err("当前账号项目路径列表格式损坏".into());
        }
    }
    if get("thread-workspace-root-hints").is_some_and(|v| {
        !v.as_object()
            .is_some_and(|m| m.values().all(Value::is_string))
    }) {
        return Err("当前账号线程项目提示格式损坏".into());
    }
    let Some(migrations) = get("app-server-projects-migration-by-host") else {
        return Ok(false);
    };
    let migrations = migrations
        .as_object()
        .ok_or("当前账号项目迁移记录格式损坏")?;
    let Some(local) = migrations.get(&format!("local:{}", home.display())) else {
        return Ok(false);
    };
    let local = local.as_object().ok_or("本地项目迁移记录格式损坏")?;
    if local.get("version").and_then(Value::as_u64) != Some(1) {
        return Err("本地项目迁移版本未知".into());
    }
    for name in [
        "projectsMigrated",
        "threadAssignmentsMigrated",
        "threadAssignmentsReadMigrated",
    ] {
        if local.get(name).is_some_and(|v| !v.is_boolean()) {
            return Err("本地项目迁移标记格式损坏".into());
        }
    }
    if local
        .get("pendingThreadAssignmentIds")
        .is_some_and(|v| !strings(v))
    {
        return Err("待迁移线程列表格式损坏".into());
    }
    // False/pending is an ordinary resumable Desktop migration, not corruption.
    Ok(local.get("projectsMigrated").and_then(Value::as_bool) == Some(true))
}
fn validate_database(copy: &Path) -> Result<()> {
    let c = Connection::open_with_flags(
        copy,
        OpenFlags::SQLITE_OPEN_READ_ONLY | OpenFlags::SQLITE_OPEN_NO_MUTEX,
    )
    .map_err(|_| "新版项目库无法读取")?;
    let checked = (|| -> rusqlite::Result<bool> {
        let integrity: String = c.query_row("PRAGMA quick_check", [], |r| r.get(0))?;
        if integrity != "ok" {
            return Ok(false);
        }
        let invalid: i64 = c.query_row("SELECT (SELECT COUNT(*) FROM projects WHERE id = '' OR name IS NULL OR NOT json_valid(metadata)) + (SELECT COUNT(*) FROM project_roots r LEFT JOIN projects p ON p.id=r.project_id WHERE p.id IS NULL OR r.path='' OR r.path IS NULL) + (SELECT COUNT(*) FROM threads t LEFT JOIN projects p ON p.id=t.project_id WHERE t.project_id IS NOT NULL AND t.project_id!='' AND p.id IS NULL)", [], |r| r.get(0))?;
        Ok(invalid == 0)
    })();
    match checked {
        Ok(true) => Ok(()),
        Ok(false) => Err("新版项目库存在损坏或悬空项目引用".into()),
        Err(_) => Err("新版项目库结构不可验证，未按旧版重建".into()),
    }
}
pub(super) fn inspect(ctx: &Context, bytes: &[u8]) -> DiagnosticItem {
    let check = (|| -> Result<()> {
        let state: Value = serde_json::from_slice(bytes).map_err(|_| "项目状态JSON损坏")?;
        let auth =
            optional(&ctx.home.join("auth.json"), 1024 * 1024)?.ok_or("无法只读确认当前账号")?;
        let current = owner(&auth).ok_or("当前认证无法解析为Desktop账号；未猜测owner")?;
        if state
            .get("electron-renderer-content-owner")
            .and_then(Value::as_str)
            != Some(current.as_str())
        {
            return Err("Desktop持久化owner与当前认证不一致或缺失，等待客户端确认账号".into());
        }
        let migrated = validate_state(&state, &current, &ctx.home)?;
        if migrated {
            let db = active_database(ctx)?.ok_or("项目已迁移但项目数据库缺失")?;
            with_private(&db, validate_database)?;
        }
        // Verify the identity and state did not change while checking the private DB.
        if optional(&ctx.home.join("auth.json"), 1024 * 1024)?.as_deref() != Some(auth.as_slice())
            || optional(&ctx.home.join(".codex-global-state.json"), 8 * 1024 * 1024)?.as_deref()
                != Some(bytes)
        {
            return Err("项目检查期间账号或状态发生变化，请重试".into());
        }
        Ok(())
    })();
    match check {
        Ok(()) => item("ok", "当前账号隔离项目字段及已完成迁移的本地项目库检查通过；待迁移线程由Desktop继续处理，不使用旧版全局cwd重建。"),
        Err(message) => item("warning", &message),
    }
}
