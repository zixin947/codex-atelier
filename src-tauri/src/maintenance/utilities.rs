//! Maintenance utilities operate on Atelier-owned preferences and allowlisted diagnostic summaries.
use crate::gateway::{
    storage::{atomic_private, open_private, private_dir, read_private},
    types::now_ms,
};
use fs2::FileExt;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::{
    path::{Path, PathBuf},
    sync::Mutex,
    time::Duration,
};
use tauri::Manager;
use uuid::Uuid;

type Result<T> = std::result::Result<T, String>;
static SETTINGS_GATE: Mutex<()> = Mutex::new(());
const MAX_BUNDLE: usize = 512 * 1024;
const MAX_RESPONSE: usize = 64 * 1024;

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UtilitySettings {
    #[serde(default)]
    pub image_compat: bool,
    #[serde(default)]
    pub diagnostic_endpoint: Option<String>,
}
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BundleExport {
    pub file_name: String,
    pub bytes: Vec<u8>,
    pub summary: String,
    pub included: Vec<String>,
    pub excluded: Vec<String>,
}
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SendReceipt {
    pub report_id: String,
    pub message: String,
}

fn root(app: &tauri::AppHandle) -> Result<PathBuf> {
    app.path()
        .app_data_dir()
        .map_err(|_| "无法定位 Atelier 数据目录".into())
}
fn settings_path(root: &Path) -> PathBuf {
    root.join("maintenance/utility-settings.json")
}
pub(crate) fn read_settings_at(root: &Path) -> Result<UtilitySettings> {
    let path = settings_path(root);
    if !path.try_exists().map_err(|_| "无法检查维护设置")? {
        return Ok(UtilitySettings::default());
    }
    serde_json::from_slice(&read_private(&path, 16 * 1024)?)
        .map_err(|_| "维护设置格式错误，未覆盖原文件".into())
}
fn update_settings(
    root: &Path,
    update: impl FnOnce(&mut UtilitySettings),
) -> Result<UtilitySettings> {
    let _gate = SETTINGS_GATE.lock().map_err(|_| "维护设置锁不可用")?;
    private_dir(&root.join("maintenance"))?;
    let lock = open_private(&root.join("maintenance/settings.lock"))?;
    lock.try_lock_exclusive()
        .map_err(|_| "另一窗口正在更新维护设置")?;
    let mut settings = read_settings_at(root)?;
    update(&mut settings);
    atomic_private(
        &settings_path(root),
        &serde_json::to_vec(&settings).map_err(|_| "设置编码失败")?,
    )?;
    Ok(settings)
}
pub(super) fn set_image_compat_at(root: &Path, enabled: bool) -> Result<UtilitySettings> {
    update_settings(root, |s| s.image_compat = enabled)
}
#[tauri::command]
pub fn maintenance_utility_settings(app: tauri::AppHandle) -> Result<UtilitySettings> {
    read_settings_at(&root(&app)?)
}

/// Mirrors the installed app's declaration filter: top-level tools and additional_tools input
/// records, including the image_gen namespace. Ordinary input images are left intact.
fn image_declaration(tool: &Value) -> bool {
    tool.get("type").and_then(Value::as_str) == Some("image_generation")
        || (tool.get("type").and_then(Value::as_str) == Some("namespace")
            && tool.get("name").and_then(Value::as_str) == Some("image_gen"))
}
fn filter_tools(tools: &mut Vec<Value>) -> usize {
    let before = tools.len();
    tools.retain(|tool| !image_declaration(tool));
    before - tools.len()
}
pub(crate) fn filter_image_declarations(body: &mut Value, enabled: bool) -> usize {
    if !enabled {
        return 0;
    }
    let mut removed = 0;
    if let Some(tools) = body.get_mut("tools").and_then(Value::as_array_mut) {
        removed += filter_tools(tools);
    }
    if let Some(input) = body.get_mut("input").and_then(Value::as_array_mut) {
        input.retain_mut(|item| {
            if item.get("type").and_then(Value::as_str) == Some("additional_tools") {
                if let Some(tools) = item.get_mut("tools").and_then(Value::as_array_mut) {
                    removed += filter_tools(tools);
                    return !tools.is_empty();
                }
            }
            true
        });
    }
    if body.get("tool_choice").is_some_and(image_declaration) {
        if let Some(obj) = body.as_object_mut() {
            obj.remove("tool_choice");
        }
    }
    removed
}

const DIAGNOSTIC_IDS: &[&str] = &[
    "config_access",
    "config_replace_readiness",
    "config_toml_syntax",
    "config_third_party",
    "config_profile_conflict",
    "config_stale",
    "config_global_validity",
    "router_recovery_record",
    "proxy_running",
    "router_http_contract",
    "router_postcondition",
    "takeover_backup_orphan",
    "catalog_path_validity",
    "catalog_integrity",
    "official_catalog_freshness",
    "auth_integrity",
    "router_unlock_auth_residue",
    "api_key_integrity",
    "auth_token_expiry",
    "codex_process_state",
    "db_orphan_providers",
    "rollout_orphan_providers",
    "thread_missing_rollouts",
    "thread_history_integrity",
    "thread_provider_consistency",
    "thread_model_catalog_consistency",
    "native_off_invalid_openai_thread_models",
    "thread_poisoned_instructions",
    "thread_response_item_ids",
    "legacy_migrated_threads",
    "residual_manifest",
    "residual_cache",
    "codex_home_writable",
    "project_state_integrity",
    "state_db_relocation",
];
fn safe_summary(report: &Value) -> Value {
    let items: Vec<_> = report
        .get("items")
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
        .filter_map(|item| {
            let id = item.get("id")?.as_str()?;
            let status = item.get("status")?.as_str()?;
            if !DIAGNOSTIC_IDS.contains(&id)
                || !["ok", "warning", "error", "unknown", "skipped"].contains(&status)
            {
                return None;
            }
            Some(json!({"id":id,"status":status}))
        })
        .take(DIAGNOSTIC_IDS.len())
        .collect();
    json!({"checkedAt": report.get("checkedAt").and_then(Value::as_u64), "items":items})
}
pub(super) fn record_diagnostic_summary(root: &Path, report: &impl Serialize) -> Result<()> {
    let value = serde_json::to_value(report).map_err(|_| "诊断摘要编码失败")?;
    private_dir(&root.join("maintenance"))?;
    atomic_private(
        &root.join("maintenance/last-diagnostic-summary.json"),
        &serde_json::to_vec(&safe_summary(&value)).map_err(|_| "诊断摘要编码失败")?,
    )
}
fn export_at(root: &Path) -> Result<BundleExport> {
    let settings = read_settings_at(root)?;
    let summary_path = root.join("maintenance/last-diagnostic-summary.json");
    let summary = if summary_path.try_exists().map_err(|_| "无法检查诊断摘要")? {
        safe_summary(
            &serde_json::from_slice::<Value>(&read_private(&summary_path, 64 * 1024)?)
                .map_err(|_| "诊断摘要格式错误")?,
        )
    } else {
        json!({"checkedAt": null,"items":[]})
    };
    let manifest = json!({"format":"atelier-diagnostics/v1","application":"Codex Atelier","version":env!("CARGO_PKG_VERSION"),
        "platform":std::env::consts::OS,"architecture":std::env::consts::ARCH,"createdAt":now_ms(),
        "settings":{"imageCompat":settings.image_compat,"diagnosticEndpointConfigured":settings.diagnostic_endpoint.is_some()},
        "privacy":"No credentials, account identifiers, filesystem paths, conversations, raw logs or environment variables."});
    let manifest = serde_json::to_vec_pretty(&manifest).map_err(|_| "诊断包编码失败")?;
    let summary = serde_json::to_vec_pretty(&summary).map_err(|_| "诊断摘要编码失败")?;
    let readme = "Codex Atelier diagnostic bundle\nOnly application/platform metadata and allowlisted check IDs/statuses.\nThis package does not run a new diagnostic scan.\nNo credentials, account identifiers, paths, conversation text or raw logs.\n";
    let bytes = zip_store(&[
        ("manifest.json", &manifest),
        ("diagnostics.json", &summary),
        ("README.txt", readme.as_bytes()),
    ])?;
    Ok(BundleExport {
        file_name: format!("atelier-diagnostics-{}.zip", now_ms()),
        bytes,
        summary: "已生成脱敏诊断包；包含最近一次检查的状态摘要，不会重新扫描。".into(),
        included: vec![
            "应用版本、平台与架构".into(),
            "Image 声明过滤状态".into(),
            "最近检查的项目编号和状态（如有）".into(),
        ],
        excluded: vec![
            "登录凭据、API Key 与钥匙串".into(),
            "账号、用户路径与环境变量".into(),
            "会话内容、原始日志与问题详情".into(),
        ],
    })
}
#[tauri::command]
pub async fn maintenance_export_bundle(app: tauri::AppHandle) -> Result<BundleExport> {
    let root = root(&app)?;
    tauri::async_runtime::spawn_blocking(move || export_at(&root))
        .await
        .map_err(|_| "诊断包生成任务失败")?
}

fn endpoint(raw: &str) -> Result<url::Url> {
    if raw.len() > 2048 {
        return Err("接收地址过长".into());
    }
    let url = url::Url::parse(raw.trim()).map_err(|_| "请输入完整接收地址")?;
    let loopback = matches!(url.host_str(), Some("localhost" | "127.0.0.1" | "[::1]"));
    if !(url.scheme() == "https" || (url.scheme() == "http" && loopback))
        || url.host_str().is_none()
        || !url.username().is_empty()
        || url.password().is_some()
        || url.query().is_some()
        || url.fragment().is_some()
    {
        return Err("接收端需要 HTTPS（本机回环可用 HTTP），地址不能包含凭据、查询或片段".into());
    }
    Ok(url)
}
fn multipart(bundle: &BundleExport, note: &str, request_id: &str, boundary: &str) -> Vec<u8> {
    let mut body = Vec::new();
    body.extend_from_slice(format!("--{boundary}\r\nContent-Disposition: form-data; name=\"requestId\"\r\n\r\n{request_id}\r\n--{boundary}\r\nContent-Disposition: form-data; name=\"userNote\"\r\n\r\n{note}\r\n--{boundary}\r\nContent-Disposition: form-data; name=\"bundle\"; filename=\"{}\"\r\nContent-Type: application/zip\r\n\r\n",bundle.file_name).as_bytes());
    body.extend_from_slice(&bundle.bytes);
    body.extend_from_slice(format!("\r\n--{boundary}--\r\n").as_bytes());
    body
}
async fn send_at(
    root: &Path,
    raw_endpoint: &str,
    note: &str,
    confirmed: bool,
) -> Result<SendReceipt> {
    if !confirmed {
        return Err("需要明确确认发送诊断包和问题说明".into());
    }
    if note.chars().count() > 2000
        || note
            .chars()
            .any(|c| c.is_control() && !matches!(c, '\n' | '\r' | '\t'))
    {
        return Err("问题说明最多 2000 字，不能包含控制字符".into());
    }
    let url = endpoint(raw_endpoint)?;
    let bundle = export_at(root)?;
    let request_id = Uuid::new_v4().to_string();
    let boundary = format!("atelier-{}", Uuid::new_v4());
    let client = reqwest::Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .connect_timeout(Duration::from_secs(10))
        .timeout(Duration::from_secs(30))
        .build()
        .map_err(|_| "无法创建诊断发送客户端")?;
    let mut response = client
        .post(url.clone())
        .header(
            "Content-Type",
            format!("multipart/form-data; boundary={boundary}"),
        )
        .body(multipart(&bundle, note, &request_id, &boundary))
        .send()
        .await
        .map_err(|_| "诊断包发送失败，未获得接收确认")?;
    let status = response.status();
    if !status.is_success() {
        return Err(format!(
            "接收端返回 HTTP {}，未确认接收；没有自动重试",
            status.as_u16()
        ));
    }
    let mut bytes = Vec::new();
    while let Some(chunk) = response
        .chunk()
        .await
        .map_err(|_| "接收端已响应，但回执读取失败；请核对后再发送")?
    {
        if bytes.len() + chunk.len() > MAX_RESPONSE {
            return Err("接收端已响应，但回执超过上限；请核对后再发送".into());
        }
        bytes.extend_from_slice(&chunk);
    }
    let receipt: Value = serde_json::from_slice(&bytes).unwrap_or(Value::Null);
    let id = receipt
        .get("reportId")
        .or_else(|| receipt.get("id"))
        .and_then(Value::as_str)
        .filter(|s| {
            !s.is_empty()
                && s.len() <= 128
                && s.chars()
                    .all(|c| c.is_ascii_alphanumeric() || matches!(c, '-' | '_' | '.'))
        });
    let message = if id.is_some() {
        format!("接收端已确认诊断包（HTTP {}）。", status.as_u16())
    } else {
        format!(
            "接收端已接受诊断包（HTTP {}）；编号为本次发送的本地追踪编号。",
            status.as_u16()
        )
    };
    // Saving the optional receiver is separate from delivery; never turn a successful send into an invitation to retry.
    let saved = update_settings(root, |s| s.diagnostic_endpoint = Some(url.into())).is_ok();
    Ok(SendReceipt {
        report_id: id.unwrap_or(&request_id).to_string(),
        message: if saved {
            message
        } else {
            format!("{message} 接收地址未能保存。")
        },
    })
}
#[tauri::command]
pub async fn maintenance_send_bundle(
    app: tauri::AppHandle,
    endpoint: String,
    user_note: String,
    confirmed: bool,
) -> Result<SendReceipt> {
    send_at(&root(&app)?, &endpoint, &user_note, confirmed).await
}

// A small, uncompressed ZIP with three fixed, allowlisted entries; no archive reading or arbitrary paths.
fn crc32(bytes: &[u8]) -> u32 {
    let mut crc = !0u32;
    for b in bytes {
        crc ^= u32::from(*b);
        for _ in 0..8 {
            crc = (crc >> 1) ^ ((0u32.wrapping_sub(crc & 1)) & 0xedb88320);
        }
    }
    !crc
}
fn zip_store(entries: &[(&str, &[u8])]) -> Result<Vec<u8>> {
    fn u16le(out: &mut Vec<u8>, value: u16) {
        out.extend(value.to_le_bytes());
    }
    fn u32le(out: &mut Vec<u8>, value: u32) {
        out.extend(value.to_le_bytes());
    }
    let mut out = Vec::new();
    let mut directory = Vec::new();
    for (name, data) in entries {
        if name.len() > u16::MAX as usize
            || data.len() > MAX_BUNDLE
            || out.len() + data.len() > MAX_BUNDLE
        {
            return Err("诊断包超过大小上限".into());
        }
        let offset = out.len() as u32;
        let crc = crc32(data);
        u32le(&mut out, 0x04034b50);
        for x in [20, 0, 0, 0, 0x21] {
            u16le(&mut out, x)
        }
        for x in [crc, data.len() as u32, data.len() as u32] {
            u32le(&mut out, x)
        }
        u16le(&mut out, name.len() as u16);
        u16le(&mut out, 0);
        out.extend(name.as_bytes());
        out.extend(*data);
        u32le(&mut directory, 0x02014b50);
        for x in [20, 20, 0, 0, 0, 0x21] {
            u16le(&mut directory, x)
        }
        for x in [crc, data.len() as u32, data.len() as u32] {
            u32le(&mut directory, x)
        }
        for x in [name.len() as u16, 0, 0, 0, 0] {
            u16le(&mut directory, x)
        }
        u32le(&mut directory, 0);
        u32le(&mut directory, offset);
        directory.extend(name.as_bytes());
    }
    let start = out.len() as u32;
    let len = directory.len() as u32;
    out.extend(directory);
    u32le(&mut out, 0x06054b50);
    for x in [0, 0, entries.len() as u16, entries.len() as u16] {
        u16le(&mut out, x)
    }
    u32le(&mut out, len);
    u32le(&mut out, start);
    u16le(&mut out, 0);
    if out.len() > MAX_BUNDLE {
        return Err("诊断包超过大小上限".into());
    }
    Ok(out)
}

#[cfg(test)]
#[path = "utility_tests.rs"]
mod tests;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SavedBundle {
    pub saved: bool,
    pub path: Option<String>,
    pub summary: String,
}
fn save_bundle_at(root: &Path, target: &Path) -> Result<SavedBundle> {
    if !target.is_absolute() || target.file_name().is_none() {
        return Err("请选择有效的诊断包保存位置".into());
    }
    let bundle = export_at(root)?;
    atomic_private(target, &bundle.bytes).map_err(|_| "诊断包保存失败；请检查目标目录权限")?;
    let actual =
        read_private(target, MAX_BUNDLE as u64).map_err(|_| "文件已写入，但无法校验保存结果")?;
    if actual != bundle.bytes {
        return Err("文件已写入，但保存结果与诊断包不一致".into());
    }
    Ok(SavedBundle {
        saved: true,
        path: Some(target.display().to_string()),
        summary: "脱敏诊断包已保存并校验。".into(),
    })
}
#[tauri::command]
pub async fn maintenance_save_bundle(app: tauri::AppHandle) -> Result<SavedBundle> {
    use tauri_plugin_dialog::DialogExt;
    let app_root = root(&app)?;
    tauri::async_runtime::spawn_blocking(move || {
        let selected = app
            .dialog()
            .file()
            .add_filter("ZIP", &["zip"])
            .set_file_name(format!("atelier-diagnostics-{}.zip", now_ms()))
            .blocking_save_file();
        let Some(selected) = selected else {
            return Ok(SavedBundle {
                saved: false,
                path: None,
                summary: "已取消保存。".into(),
            });
        };
        let target = selected
            .into_path()
            .map_err(|_| "所选位置不是可写入的本地文件")?;
        save_bundle_at(&app_root, &target)
    })
    .await
    .map_err(|_| "保存诊断包任务失败")?
}
