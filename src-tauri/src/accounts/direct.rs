//! Pure official-account routing plan and explicit source observations.
use super::storage::read_external;
use crate::maintenance::diagnostics;
use atelier_maintenance_core::{
    catalog::state::RelayStateDocument,
    transition_journal::{TransitionJournal, TransitionTarget},
};
use std::path::{Path, PathBuf};

#[derive(Clone)]
pub(super) struct Plan {
    pub config: Option<Vec<u8>>,
    pub intent: Vec<u8>,
    pub changes: Vec<String>,
    pub dependencies: Vec<(PathBuf, Option<Vec<u8>>)>,
}
pub(super) fn app_root(store: &super::storage::Store) -> Result<PathBuf, String> {
    store
        .root
        .parent()
        .map(Path::to_path_buf)
        .ok_or_else(|| "账号目录缺少应用根目录".into())
}
pub(super) fn validate_parents(home: &Path, app: &Path) -> Result<(), String> {
    for path in [
        app.join("gateway"),
        home.join("codexmate"),
        home.join("codexmate/relay"),
    ] {
        match std::fs::symlink_metadata(&path) {
            Ok(meta) if meta.is_dir() && !meta.file_type().is_symlink() => (),
            Ok(_) => return Err("路由资料父目录不是普通目录，未跟随链接或覆盖".into()),
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => (),
            Err(_) => return Err("无法核对路由资料父目录".into()),
        }
    }
    Ok(())
}
pub(super) fn dependencies(
    home: &Path,
    app: &Path,
) -> Result<Vec<(PathBuf, Option<Vec<u8>>)>, String> {
    // Do not follow substituted gateway/relay parents, including on read-only preview.
    validate_parents(home, app)?;
    let native = home.join("codexmate/relay");
    let mut result = vec![];
    for (path, limit) in [
        (native.join("state.json"), 8 * 1024 * 1024),
        (native.join("router_model_restore.json"), 1024 * 1024),
        (
            native.join("router_no_account_model_restore.json"),
            1024 * 1024,
        ),
        (app.join("gateway/router_model_restore.json"), 1024 * 1024),
        (
            app.join("gateway/router_no_account_model_restore.json"),
            1024 * 1024,
        ),
        (native.join("router-transition-journal.json"), 1024 * 1024),
        (
            app.join("gateway/router-transition-journal.json"),
            1024 * 1024,
        ),
        (app.join("gateway/transition.json"), 1024 * 1024),
        (app.join("gateway/catalog.json"), 8 * 1024 * 1024),
        (native.join("router-takeover-backup.json"), 1024 * 1024),
        (
            native.join("codex_router_top_level_backup.json"),
            1024 * 1024,
        ),
        (native.join("router-unlock-auth-marker.json"), 1024 * 1024),
        (native.join("router-unlock-auth-backup.json"), 1024 * 1024),
        (native.join("virtual-auth-marker.json"), 1024 * 1024),
        (native.join("api-login-auth-backup.json"), 1024 * 1024),
        (app.join("gateway/auth-takeover.json"), 1024 * 1024),
        (app.join("gateway/takeover-backup.json"), 1024 * 1024),
    ] {
        let bytes = read_external(&path, limit)?;
        if let Some(raw) = &bytes {
            if path == native.join("state.json") {
                let state = RelayStateDocument::from_json(raw)
                    .map_err(|_| "原生路由状态无法核对，不能自动切换直连")?;
                if state
                    .catalog_inputs()
                    .map_err(|_| "原生路由模式无效")?
                    .router_enabled
                {
                    return Err("外部原生路由仍启用，请先在其所属应用退出接管后重新预览".into());
                }
            } else if path
                .file_name()
                .is_some_and(|n| n == "router-transition-journal.json")
            {
                let journal: TransitionJournal = serde_json::from_slice(raw)
                    .map_err(|_| "路由恢复记录损坏，请先处理恢复记录")?;
                if journal.schema_version != 1 || journal.is_active() || journal.phase != "done" {
                    return Err("存在未完成路由恢复记录，请先恢复后重新预览".into());
                }
            } else if path != app.join("gateway/catalog.json") && !is_model_restore(&path) {
                return Err(format!(
                    "存在待处理的接管/转换资料：{}；未自动覆盖",
                    path.display()
                ));
            }
        }
        result.push((path, bytes));
    }
    Ok(result)
}
fn is_model_restore(path: &Path) -> bool {
    path.file_name().is_some_and(|name| {
        name == "router_model_restore.json" || name == "router_no_account_model_restore.json"
    })
}
pub(super) fn validate_dependencies(expected: &[(PathBuf, Option<Vec<u8>>)]) -> Result<(), String> {
    for (path, bytes) in expected {
        if read_external(path, 8 * 1024 * 1024)? != *bytes {
            return Err("路由来源或恢复记录在预览后变化，请重新预览".into());
        }
    }
    Ok(())
}
/// Preserve provider definitions and unrelated TOML. Remove only active overrides,
/// never infer an official model name or alter conversation metadata.
pub(super) fn config(
    before: Option<&[u8]>,
    home: &Path,
    app: &Path,
) -> Result<(Option<Vec<u8>>, Vec<String>), String> {
    let text = std::str::from_utf8(before.unwrap_or_default()).map_err(|_| "Codex配置不是UTF8")?;
    let mut doc = text
        .parse::<toml_edit::DocumentMut>()
        .map_err(|_| "Codex配置无法解析；不会修改")?;
    if doc
        .get("cli_auth_credentials_store")
        .is_some_and(|v| v.as_str() != Some("file"))
    {
        return Err("当前不是file认证模式，不自动接管钥匙串、auto或临时认证".into());
    }
    if doc.get("profile").is_some() {
        return Err("当前包含命名profile选择，无法确认其覆盖；请先切回默认配置".into());
    }
    if doc
        .get("forced_login_method")
        .is_some_and(|v| v.as_str() != Some("chatgpt"))
    {
        return Err("配置强制了其他认证方式，不能切换官方订阅账号".into());
    }
    if doc
        .get("model_providers")
        .and_then(|v| v.get("openai"))
        .is_some()
    {
        return Err(
            "存在[model_providers.openai]覆盖，不能仅凭openai名称保证官方直连；请先核对该覆盖"
                .into(),
        );
    }
    let mut changes = vec![];
    let mut comments = String::new();
    let selected = match doc.get("model_provider") {
        None => "openai".to_owned(),
        Some(v) => v.as_str().ok_or("model_provider必须为字符串")?.to_owned(),
    };
    let custom = selected != "openai";
    if custom {
        remove_setting(&mut doc, "model_provider", &mut comments);
        changes.push(
            "撤销当前自定义Provider选择，使用Codex内置OpenAI连接；保留Provider定义供旧会话使用"
                .into(),
        );
    }
    let redirected = remove_setting(&mut doc, "openai_base_url", &mut comments);
    if redirected {
        changes.push("移除顶层openai_base_url地址覆盖，恢复内置官方端点".into());
    }
    let mut detached_catalog = false;
    if doc.get("model_catalog_json").is_some() {
        let raw = doc["model_catalog_json"]
            .as_str()
            .ok_or("模型目录引用必须为字符串")?;
        let path = PathBuf::from(raw);
        let path = if path.is_absolute() {
            path
        } else {
            home.join(path)
        };
        let managed = !path
            .components()
            .any(|c| matches!(c, std::path::Component::ParentDir))
            && (path.starts_with(app.join("gateway"))
                || path.starts_with(home.join("codexmate/relay")));
        if !managed {
            return Err(
                "存在非受管model_catalog_json覆盖，无法保证官方原生模型发现；请先核对该目录".into(),
            );
        }
        remove_setting(&mut doc, "model_catalog_json", &mut comments);
        detached_catalog = true;
        changes.push("移除本地受管模型目录引用，交回官方模型发现".into());
    }
    let managed_model = doc.get("model").and_then(|v| v.as_str()).is_some_and(|m| {
        m.starts_with("atelier_") || m.starts_with("aimami_relay_") || m.contains("::")
    });
    if custom || redirected || detached_catalog || managed_model {
        if remove_setting(&mut doc, "model", &mut comments) {
            changes.push("清除中转连接的模型选择，使用官方默认模型（不猜测官方可用模型ID）".into());
        }
        if remove_setting(&mut doc, "model_reasoning_effort", &mut comments) {
            changes.push("清除中转模型的推理档位，交回官方默认值".into());
        }
    }
    let rendered = format!("{comments}{doc}").into_bytes();
    let after = if before.is_none() && rendered.is_empty() {
        None
    } else {
        Some(rendered)
    };
    if changes.is_empty() {
        changes.push("连接配置无需修改：保留Codex内置官方连接".into());
    }
    Ok((after, changes))
}
fn remove_setting(doc: &mut toml_edit::DocumentMut, key: &str, comments: &mut String) -> bool {
    if let Some(prefix) = doc
        .as_table()
        .key(key)
        .and_then(|k| k.leaf_decor().prefix())
        .and_then(|s| s.as_str())
    {
        comments.push_str(prefix);
    }
    if let Some(suffix) = doc
        .get(key)
        .and_then(|i| i.as_value())
        .and_then(|v| v.decor().suffix())
        .and_then(|s| s.as_str())
    {
        if suffix.contains('#') {
            comments.push_str(suffix);
            comments.push('\n');
        }
    }
    doc.remove(key).is_some()
}
pub(super) fn prepare(
    home: &Path,
    app: &Path,
    before: Option<&[u8]>,
    intent: Option<&[u8]>,
) -> Result<Plan, String> {
    let deps = dependencies(home, app)?;
    let (config, mut changes) = config(before, home, app)?;
    if deps
        .iter()
        .any(|(path, bytes)| is_model_restore(path) && bytes.is_some())
    {
        changes.push("保留已有模型恢复记录，不因记录存在阻止切换；不删除或覆盖其内容".into());
    }
    let catalog = deps
        .iter()
        .find(|(p, _)| p == &app.join("gateway/catalog.json"))
        .and_then(|(_, b)| b.as_deref());
    let intent = diagnostics::official_account_intent(intent, catalog)?;
    changes.push("将Atelier路由意图设为官方直连，避免后续维护重新接入网关；不停止网关服务".into());
    // Ensure our serialized intent is accepted by the actual Gateway decision consumer.
    let value: serde_json::Value =
        serde_json::from_slice(&intent).map_err(|_| "路由意图编码失败")?;
    if value["target"] != serde_json::json!(TransitionTarget::NativeOff) {
        return Err("直连意图不一致".into());
    }
    Ok(Plan {
        config,
        intent,
        changes,
        dependencies: deps,
    })
}
