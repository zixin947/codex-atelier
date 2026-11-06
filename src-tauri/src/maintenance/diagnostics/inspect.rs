use super::*;
use base64::Engine;
use rusqlite::{Connection, OpenFlags};
use serde_json::Value;
use std::{
    collections::{BTreeSet, HashSet},
    fs,
    io::Read,
};
pub(super) const LIMIT: u64 = 32 * 1024 * 1024;
#[derive(Clone, Debug)]
pub(super) struct Row {
    pub id: String,
    pub provider: String,
    pub model: String,
    pub path: String,
}
pub(super) struct Threads {
    pub db: Option<PathBuf>,
    pub rows: Vec<Row>,
    pub missing: usize,
    pub recoverable_missing: usize,
    pub bad: usize,
    pub mismatch: usize,
    pub orphan_db: usize,
    pub orphan_rollout: usize,
    pub bad_ids: usize,
    pub unsupported_ids: usize,
    pub protected_provider: usize,
    pub protected_ids: usize,
    pub instruction_issues: usize,
    pub invalid_models: usize,
    pub incomplete: bool,
    pub error: Option<String>,
}
pub(super) fn profile_provider(c: &toml::Value) -> Option<&str> {
    let name = c.get("profile")?.as_str()?;
    c.get("profiles")?
        .get(name)?
        .get("model_provider")?
        .as_str()
}
pub(super) fn provider(c: &toml::Value) -> String {
    profile_provider(c)
        .or_else(|| c.get("model_provider").and_then(|v| v.as_str()))
        .unwrap_or("openai")
        .into()
}
pub(super) fn config(ctx: &Context) -> Result<toml::Value> {
    if !ctx.config().exists() {
        return Ok(toml::Value::Table(Default::default()));
    }
    toml::from_str(
        std::str::from_utf8(&read_bounded(&ctx.config(), 1024 * 1024)?)
            .map_err(|_| "配置不是 UTF-8")?,
    )
    .map_err(|_| "TOML 语法无效；未自动重写".into())
}
fn add(
    items: &mut Vec<DiagnosticItem>,
    id: &str,
    group: &str,
    label: &str,
    status: &str,
    detail: impl Into<String>,
    fixable: bool,
) {
    items.push(DiagnosticItem {
        id: id.into(),
        group: group.into(),
        label: label.into(),
        status: status.into(),
        detail: detail.into(),
        fixable,
    });
}
fn json_file(p: &Path) -> Result<Value> {
    serde_json::from_slice(&read_bounded(p, 8 * 1024 * 1024)?).map_err(|_| "JSON 格式无效".into())
}
fn presence(p: &Path) -> bool {
    fs::symlink_metadata(p).is_ok()
}
pub(super) fn db_dir(ctx: &Context, c: &toml::Value) -> PathBuf {
    c.get("sqlite_home")
        .and_then(|v| v.as_str())
        .map(|v| {
            let p = PathBuf::from(v);
            if p.is_absolute() {
                p
            } else {
                ctx.home.join(p)
            }
        })
        .unwrap_or_else(|| ctx.home.clone())
}
pub(super) fn databases(dir: &Path) -> Vec<PathBuf> {
    let mut found = fs::read_dir(dir)
        .into_iter()
        .flatten()
        .filter_map(|e| e.ok())
        .filter_map(|e| {
            let n = e.file_name().to_string_lossy().into_owned();
            let v = n
                .strip_prefix("state_")?
                .strip_suffix(".sqlite")?
                .parse::<u32>()
                .ok()?;
            Some((v, e.path()))
        })
        .collect::<Vec<_>>();
    found.sort_by_key(|v| v.0);
    found.into_iter().map(|v| v.1).collect()
}
pub(super) fn open_db(p: &Path, write: bool) -> Result<Connection> {
    let m = fs::symlink_metadata(p).map_err(|_| "会话索引无法读取")?;
    if !m.is_file() || m.file_type().is_symlink() {
        return Err("会话索引不是普通文件".into());
    }
    let c = if write {
        Connection::open_with_flags(p, OpenFlags::SQLITE_OPEN_READ_WRITE)
    } else {
        // immutable keeps inspection from creating SQLite shared-memory sidecars. WAL presence is
        // surfaced as incomplete coverage; no result from this view authorizes a repair.
        let u = url::Url::from_file_path(p.canonicalize().map_err(err)?)
            .map_err(|_| "数据库路径无效")?;
        Connection::open_with_flags(
            format!("{}?immutable=1", u),
            OpenFlags::SQLITE_OPEN_READ_ONLY | OpenFlags::SQLITE_OPEN_URI,
        )
    }
    .map_err(|_| "无法打开会话索引")?;
    c.busy_timeout(std::time::Duration::from_secs(2))
        .map_err(err)?;
    Ok(c)
}
pub(super) fn rows(c: &Connection) -> Result<Vec<Row>> {
    let mut s = c.prepare("PRAGMA table_info(threads)").map_err(err)?;
    let columns = s
        .query_map([], |r| r.get::<_, String>(1))
        .map_err(err)?
        .collect::<std::result::Result<HashSet<_>, _>>()
        .map_err(err)?;
    if !columns.contains("id") || !columns.contains("rollout_path") {
        return Err("不支持此版本索引，缺少 id / rollout_path".into());
    }
    let col = |n: &str| {
        if columns.contains(n) {
            format!("COALESCE({n},'')")
        } else {
            "''".into()
        }
    };
    let sql = format!(
        "SELECT id,{},{},rollout_path FROM threads ORDER BY id LIMIT 1001",
        col("model_provider"),
        col("model")
    );
    let mut s = c.prepare(&sql).map_err(err)?;
    let r = s
        .query_map([], |r| {
            Ok(Row {
                id: r.get(0)?,
                provider: r.get(1)?,
                model: r.get(2)?,
                path: r.get(3)?,
            })
        })
        .map_err(err)?
        .collect::<std::result::Result<Vec<_>, _>>()
        .map_err(err)?;
    Ok(r)
}
pub(super) fn resolve(ctx: &Context, path: &str) -> Result<PathBuf> {
    let p = PathBuf::from(path);
    let p = if p.is_absolute() { p } else { ctx.home.join(p) };
    let p = if p.exists() {
        p
    } else {
        PathBuf::from(format!("{}.zst", p.display()))
    };
    let p = p.canonicalize().map_err(|_| "记录文件未找到")?;
    if ![
        ctx.home.join("sessions"),
        ctx.home.join("archived_sessions"),
    ]
    .iter()
    .filter_map(|p| p.canonicalize().ok())
    .any(|root| p.starts_with(root))
    {
        return Err("记录路径不在受支持的会话目录中".into());
    }
    Ok(p)
}
pub(super) fn rollout(p: &Path) -> Result<Vec<u8>> {
    decode(p, read_bounded(p, LIMIT)?)
}
pub(super) fn decode(p: &Path, raw: Vec<u8>) -> Result<Vec<u8>> {
    if p.extension().is_some_and(|v| v == "zst") {
        let mut decoder =
            zstd::stream::read::Decoder::new(raw.as_slice()).map_err(|_| "压缩记录无法解码")?;
        decoder.window_log_max(27).map_err(err)?;
        let mut v = vec![];
        decoder
            .take(LIMIT + 1)
            .read_to_end(&mut v)
            .map_err(|_| "压缩记录不完整")?;
        if v.len() as u64 > LIMIT {
            return Err("记录解压超过32MiB检查上限".into());
        }
        Ok(v)
    } else {
        Ok(raw)
    }
}
pub(super) fn meta(bytes: &[u8]) -> Result<Value> {
    for line in bytes.split(|b| *b == b'\n').take(32) {
        if line.is_empty() {
            continue;
        }
        let v: Value = serde_json::from_slice(line).map_err(|_| "记录包含损坏的 JSON")?;
        if v["type"] == "session_meta" {
            return Ok(v["payload"].clone());
        }
    }
    Err("记录中未找到 session_meta".into())
}
fn known_providers(c: &toml::Value) -> BTreeSet<String> {
    let mut ids = BTreeSet::from(["openai".into(), "ollama".into(), "lmstudio".into()]);
    if let Some(t) = c.get("model_providers").and_then(|v| v.as_table()) {
        ids.extend(
            t.iter()
                .filter(|(_, value)| value.is_table())
                .map(|(key, _)| key.clone()),
        )
    }
    ids
}
fn owned_provider(p: &str) -> bool {
    p == "atelier" || p == "codex_atelier" || p.starts_with("atelier_")
}
/// Identify known structural ownership markers, not arbitrary custom provider names or URLs.
fn third_party_markers(config: &toml::Value) -> BTreeSet<String> {
    let mut found = BTreeSet::new();
    let mut pending = vec![config];
    while let Some(value) = pending.pop() {
        match value {
            toml::Value::Table(table) => {
                for (key, value) in table {
                    let marker = key.to_ascii_lowercase();
                    if matches!(
                        marker.as_str(),
                        "cc-switch"
                            | "cc_switch"
                            | "ccswitch"
                            | "codex-switch"
                            | "codex_switch"
                            | "circuit_breaker"
                            | "provider_router"
                            | "deecodex"
                    ) {
                        found.insert(marker);
                    }
                    pending.push(value);
                }
            }
            toml::Value::Array(values) => pending.extend(values),
            _ => {}
        }
    }
    found
}
/// Locate an existing identity-matched rollout; never synthesize history or pick an ambiguous match.
pub(super) fn recover_path(ctx: &Context, id: &str) -> Option<PathBuf> {
    if id.is_empty() {
        return None;
    }
    let mut stack = vec![
        ctx.home.join("sessions"),
        ctx.home.join("archived_sessions"),
    ];
    let mut candidates = vec![];
    let mut examined = 0;
    while let Some(dir) = stack.pop() {
        for entry in fs::read_dir(dir).into_iter().flatten().flatten() {
            examined += 1;
            if examined > 5000 {
                return None;
            }
            let ty = entry.file_type().ok()?;
            if ty.is_symlink() {
                continue;
            }
            if ty.is_dir() {
                stack.push(entry.path());
                continue;
            }
            let name = entry.file_name().to_string_lossy().into_owned();
            if !name.contains(id) || !(name.ends_with(".jsonl") || name.ends_with(".jsonl.zst")) {
                continue;
            }
            let path = resolve(ctx, &entry.path().to_string_lossy()).ok()?;
            if rollout(&path)
                .ok()
                .and_then(|b| meta(&b).ok())
                .is_some_and(|m| m["id"].as_str() == Some(id))
            {
                candidates.push(path);
                if candidates.len() > 1 {
                    return None;
                }
            }
        }
    }
    candidates.pop()
}
pub(super) fn scan_threads(ctx: &Context, c: &toml::Value) -> Threads {
    let mut t = Threads {
        db: databases(&db_dir(ctx, c)).pop(),
        rows: vec![],
        missing: 0,
        recoverable_missing: 0,
        bad: 0,
        mismatch: 0,
        orphan_db: 0,
        orphan_rollout: 0,
        bad_ids: 0,
        unsupported_ids: 0,
        protected_provider: 0,
        protected_ids: 0,
        instruction_issues: 0,
        invalid_models: 0,
        incomplete: false,
        error: None,
    };
    let Some(db) = t.db.as_ref() else { return t };
    match super::snapshot::read_rows(db) {
        Ok(r) => t.rows = r,
        Err(e) => {
            t.error = Some(e);
            return t;
        }
    }
    if t.rows.len() > 1000 {
        t.incomplete = true;
        t.rows.truncate(1000)
    }
    let providers = known_providers(c);
    let mut total = 0usize;
    for row in &t.rows {
        if owned_provider(&row.provider) && !providers.contains(&row.provider) {
            t.orphan_db += 1
        }
        let Ok(path) = resolve(ctx, &row.path) else {
            t.missing += 1;
            if recover_path(ctx, &row.id).is_some() {
                t.recoverable_missing += 1
            }
            continue;
        };
        if total > 128 * 1024 * 1024 {
            t.incomplete = true;
            break;
        }
        let bytes = match rollout(&path) {
            Ok(b) => b,
            Err(_) => {
                t.bad += 1;
                t.incomplete = true;
                continue;
            }
        };
        total += bytes.len();
        let m = match meta(&bytes) {
            Ok(m) => m,
            Err(_) => {
                t.bad += 1;
                continue;
            }
        };
        if m["id"].as_str() != Some(row.id.as_str()) {
            t.bad += 1;
            continue;
        }
        let rp = m["model_provider"].as_str().unwrap_or("openai");
        if rp != row.provider || (!row.provider.is_empty() && row.provider != provider(c)) {
            t.mismatch += 1;
            if super::record_compat::has_positions(&bytes).unwrap_or(true) {
                t.protected_provider += 1
            }
        }
        if owned_provider(rp) && !providers.contains(rp) {
            t.orphan_rollout += 1
        }
        if m.get("base_instructions").is_some_and(|v| {
            !v.is_null() && !v.is_string() && !(v.is_object() && v["text"].is_string())
        }) {
            t.instruction_issues += 1
        }
        let compat_count = super::record_compat::inspect(&bytes);
        match compat_count {
            Ok(count) => {
                t.bad_ids += count;
                if count > 0 && super::record_compat::has_positions(&bytes).unwrap_or(true) {
                    t.protected_ids += 1
                }
            }
            Err(_) => t.bad += 1,
        };
        let mut ids = HashSet::new();
        let mut malformed = false;
        for line in bytes.split(|b| *b == b'\n').filter(|l| !l.is_empty()) {
            let Ok(v) = serde_json::from_slice::<Value>(line) else {
                malformed = true;
                continue;
            };
            if v["type"] == "response_item" {
                if let Some(id) = v["payload"]["id"].as_str() {
                    if id.is_empty()
                        || id.len() > 256
                        || id.chars().any(|c| c.is_control())
                        || !ids.insert(id.to_owned())
                    {
                        t.bad_ids += 1;
                        t.unsupported_ids += 1
                    }
                }
            }
        }
        if malformed {
            t.bad += 1
        }
        if row.model.chars().any(char::is_whitespace) {
            t.invalid_models += 1
        }
    }
    t
}
pub(super) fn run(ctx: &Context) -> Result<DiagnosticReport> {
    let mut items = vec![];
    let config_result = config(ctx);
    let c = config_result
        .clone()
        .unwrap_or(toml::Value::Table(Default::default()));
    let config_ok = config_result.is_ok();
    let cm = fs::symlink_metadata(ctx.config());
    let writable = cm
        .as_ref()
        .is_ok_and(|m| m.is_file() && !m.file_type().is_symlink() && !m.permissions().readonly());
    add(
        &mut items,
        "config_access",
        "config",
        "config.toml 读取",
        if !ctx.config().exists() {
            "skipped"
        } else if read_bounded(&ctx.config(), 1024 * 1024).is_ok() {
            "ok"
        } else {
            "error"
        },
        if ctx.config().exists() {
            "读取现有配置，不创建文件"
        } else {
            "配置文件尚不存在，Codex 可使用默认配置"
        },
        false,
    );
    add(
        &mut items,
        "config_replace_readiness",
        "config",
        "配置文件替换条件",
        if !ctx.config().exists() {
            "skipped"
        } else if writable {
            "ok"
        } else {
            "warning"
        },
        "根据文件元数据检查普通文件和只读标记；未执行写入探针，ACL / 沙盒权限需实际写入时确认",
        false,
    );
    add(
        &mut items,
        "config_toml_syntax",
        "config",
        "TOML 语法",
        if config_ok { "ok" } else { "error" },
        config_result
            .as_ref()
            .err()
            .cloned()
            .unwrap_or("TOML 可以解析".into()),
        false,
    );
    let p = provider(&c);
    let providers = known_providers(&c);
    let markers = third_party_markers(&c);
    add(
        &mut items,
        "config_third_party",
        "config",
        "第三方管理残留标记",
        if !config_ok {
            "unknown"
        } else if markers.is_empty() {
            "ok"
        } else {
            "warning"
        },
        if markers.is_empty() {
            "未发现已识别的第三方管理残留；合法自定义 Provider 不作为异常".into()
        } else {
            format!(
                "发现已识别的配置结构标记：{}。请核对归属，保留原配置，不自动删除自定义连接。",
                markers.into_iter().collect::<Vec<_>>().join("、")
            )
        },
        false,
    );
    let invalid_profile = c
        .get("profile")
        .and_then(|v| v.as_str())
        .is_some_and(|v| c.get("profiles").and_then(|t| t.get(v)).is_none());
    add(
        &mut items,
        "config_profile_conflict",
        "config",
        "顶层 profile 引用",
        if !config_ok {
            "unknown"
        } else if invalid_profile {
            "warning"
        } else {
            "ok"
        },
        if invalid_profile {
            "顶层 profile 指向不存在的配置档；可移除失效选择，保留 profiles 内容"
        } else {
            "没有失效的顶层 profile 引用；有效 profile 不作为残留删除"
        },
        invalid_profile && writable,
    );
    let invalid_provider = !providers.contains(&p);
    add(
        &mut items,
        "config_global_validity",
        "config",
        "配置引用一致性",
        if !config_ok {
            "unknown"
        } else if invalid_provider {
            "error"
        } else {
            "ok"
        },
        if invalid_provider {
            "当前 model_provider 没有对应定义；可移除失效选择，回到内置 openai（不删除其他 Provider）"
        } else {
            "当前 Provider 引用有本地定义或属于内置 Provider"
        },
        invalid_provider && writable,
    );
    let gateway = ctx.app_root.join("gateway/catalog.json");
    let g = json_file(&gateway).ok();
    let managed = owned_provider(&p);
    let base = c
        .get("model_providers")
        .and_then(|v| v.get(&p))
        .and_then(|v| v.get("base_url"))
        .and_then(|v| v.as_str());
    let expected_port = g
        .as_ref()
        .and_then(|v| v["settings"]["port"].as_u64())
        .filter(|p| *p > 0 && *p <= 65535);
    let route_section = c.get("model_providers").and_then(|v| v.get(&p));
    let route_valid = expected_port.is_some()
        && base.and_then(|s| url::Url::parse(s).ok()).is_some_and(|u| {
            u.scheme() == "http"
                && matches!(u.host_str(), Some("localhost" | "127.0.0.1" | "[::1]"))
                && u.port().map(u64::from) == expected_port
                && u.path().trim_end_matches('/') == "/v1"
                && u.username().is_empty()
                && u.password().is_none()
                && u.query().is_none()
                && u.fragment().is_none()
        })
        && route_section.is_some_and(|s| {
            s.get("wire_api").and_then(|v| v.as_str()) == Some("responses")
                && s.get("env_key").and_then(|v| v.as_str()) == Some("ATELIER_GATEWAY_TOKEN")
                && s.get("requires_openai_auth").and_then(|v| v.as_bool()) == Some(false)
                && s.get("supports_websockets").and_then(|v| v.as_bool()) == Some(false)
        });
    add(
        &mut items,
        "config_stale",
        "config",
        "受管连接配置一致性",
        if !managed {
            "skipped"
        } else if route_valid {
            "ok"
        } else {
            "warning"
        },
        if managed {
            "比对 Atelier 网关端口与当前受管 Provider；不自动接管自定义连接"
        } else {
            "当前配置没有使用 Atelier 受管 Provider"
        },
        managed
            && !route_valid
            && expected_port.is_some()
            && writable
            && route_section.is_some_and(|s| s.is_table()),
    );
    add(
        &mut items,
        "router_http_contract",
        "config",
        "本地路由 HTTP 契约",
        if !managed {
            "skipped"
        } else if route_valid {
            "ok"
        } else {
            "error"
        },
        "核对本机 HTTP /v1、网关端口、Responses 协议、环境变量认证及关闭 WebSocket",
        managed
            && !route_valid
            && expected_port.is_some()
            && writable
            && route_section.is_some_and(|s| s.is_table()),
    );
    for (id, label, path) in [
        (
            "router_recovery_record",
            "路由恢复记录",
            "gateway/recovery.json",
        ),
        (
            "router_postcondition",
            "路由转换完成状态",
            "gateway/transition.json",
        ),
        (
            "takeover_backup_orphan",
            "接管备份残留",
            "gateway/takeover-backup.json",
        ),
        (
            "router_unlock_auth_residue",
            "路由认证接管残留",
            "gateway/auth-takeover.json",
        ),
    ] {
        let exists = presence(&ctx.app_root.join(path));
        add(
            &mut items,
            id,
            if id == "router_unlock_auth_residue" {
                "auth"
            } else {
                "config"
            },
            label,
            if exists { "warning" } else { "skipped" },
            if exists {
                "发现相关状态文件，需要通过创建它的路由版本恢复；诊断不会提交或删除恢复记录"
            } else {
                "Atelier 当前没有此类接管记录；不读取 AiMaMi 的私有状态"
            },
            false,
        );
    }
    let proxy_status = if !managed {
        "skipped"
    } else {
        match ctx.runtime.as_ref().map(|r| r.state.as_str()) {
            Some("running") => "ok",
            Some("error" | "stopped") => "warning",
            _ => "unknown",
        }
    };
    add(
        &mut items,
        "proxy_running",
        "runtime",
        "本地代理运行状态",
        proxy_status,
        if !managed {
            "当前没有使用 Atelier 受管路由".into()
        } else if let Some(runtime) = &ctx.runtime {
            format!(
                "Atelier 网关状态：{}；在途请求 {}。没有发送网络探针",
                runtime.state, runtime.active_requests
            )
        } else {
            "本次 Atelier 进程尚未创建网关实例；不以端口猜测其他进程的归属".into()
        },
        false,
    );
    let catalog = c
        .get("model_catalog_json")
        .and_then(|v| v.as_str())
        .map(PathBuf::from)
        .map(|v| if v.is_absolute() { v } else { ctx.home.join(v) });
    let catalog_data = catalog.as_ref().map(|p| json_file(p));
    let catalog_valid = catalog_data
        .as_ref()
        .is_some_and(|r| r.as_ref().is_ok_and(|v| v["models"].is_array()));
    add(
        &mut items,
        "catalog_path_validity",
        "config",
        "模型目录路径",
        if catalog.is_none() {
            "skipped"
        } else if catalog.as_ref().is_some_and(|p| p.is_file()) {
            "ok"
        } else {
            "error"
        },
        if catalog.is_none() {
            "未覆盖默认模型目录"
        } else {
            "检查 model_catalog_json 指向的本地文件"
        },
        false,
    );
    add(
        &mut items,
        "catalog_integrity",
        "config",
        "模型目录完整性",
        if catalog.is_none() {
            "skipped"
        } else if catalog_valid {
            "ok"
        } else {
            "error"
        },
        "自定义目录需要可读取的 JSON 和 models 数组；不通过空文件掩盖损坏",
        false,
    );
    add(
        &mut items,
        "official_catalog_freshness",
        "config",
        "官方模型目录新鲜度",
        "unknown",
        "未联网获取官方目录，也没有可验证的最近请求记录",
        false,
    );
    let auth = ctx.home.join("auth.json");
    let a = if presence(&auth) {
        Some(json_file(&auth))
    } else {
        None
    };
    let av = a.as_ref().and_then(|v| v.as_ref().ok());
    let token = av.and_then(|v| v["tokens"]["access_token"].as_str());
    let api = av
        .and_then(|v| v["OPENAI_API_KEY"].as_str())
        .is_some_and(|v| !v.trim().is_empty());
    let auth_valid = api || token.is_some_and(|v| !v.trim().is_empty());
    add(
        &mut items,
        "auth_integrity",
        "auth",
        "文件认证完整性",
        if a.is_none() {
            "unknown"
        } else if auth_valid {
            "ok"
        } else {
            "error"
        },
        if a.is_none() {
            "没有文件型认证；不据此判断 Desktop 是否登录，也不读取系统钥匙串"
        } else {
            "仅检查本地文件结构和必要字段，不验证令牌有效性、不返回令牌内容"
        },
        false,
    );
    let profiles = g.as_ref().and_then(|v| v["profiles"].as_array());
    let mut api_total = 0;
    let mut api_bad = 0;
    if let Some(profiles) = profiles {
        for profile in profiles {
            if profile["kind"] == "apiKey" && profile["enabled"] == true {
                api_total += 1;
                let id = profile["id"].as_str().unwrap_or("");
                let valid = uuid::Uuid::parse_str(id).is_ok()
                    && json_file(
                        &ctx.app_root
                            .join("gateway/credentials")
                            .join(format!("{id}.json")),
                    )
                    .ok()
                    .is_some_and(|v| v["apiKey"].as_str().is_some_and(|s| !s.trim().is_empty()));
                if !valid {
                    api_bad += 1
                }
            }
        }
    }
    add(&mut items,"api_key_integrity","auth","中转密钥完整性",if api_total==0{"skipped"}else if api_bad==0{"ok"}else{"error"},format!("检查 {api_total} 个启用的 Atelier 中转档案，{api_bad} 个缺少本地密钥；未联网验证、未显示密钥"),false);
    let expiry = token
        .and_then(|s| s.split('.').nth(1))
        .and_then(|s| {
            base64::engine::general_purpose::URL_SAFE_NO_PAD
                .decode(s)
                .ok()
        })
        .and_then(|b| serde_json::from_slice::<Value>(&b).ok())
        .and_then(|v| v["exp"].as_u64());
    add(
        &mut items,
        "auth_token_expiry",
        "auth",
        "登录令牌到期",
        if api {
            "skipped"
        } else if expiry.is_some_and(|e| e < now_ms() / 1000) {
            "warning"
        } else if expiry.is_some() {
            "ok"
        } else {
            "unknown"
        },
        match expiry {
            Some(e) => {
                format!("本地令牌声明的到期时间戳 {e}；未校验签名或在线会话，刷新由账号流程处理")
            }
            None => "没有可读取的令牌到期声明".into(),
        },
        false,
    );
    add(
        &mut items,
        "codex_process_state",
        "runtime",
        "Desktop 进程状态",
        if ctx.desktop_running { "ok" } else { "skipped" },
        if ctx.desktop_running {
            "检测到 Desktop 进程；文件修复需要先退出客户端"
        } else {
            "未检测到已识别的 Desktop 主进程；不自动启动"
        },
        false,
    );
    let t = scan_threads(ctx, &c);
    let thread_status = |count: usize| {
        if !config_ok || t.error.is_some() || t.incomplete {
            "unknown"
        } else if t.db.is_none() {
            "skipped"
        } else if count > 0 {
            "warning"
        } else {
            "ok"
        }
    };
    let summary = |count: usize| {
        format!(
            "检查 {} 条稳定快照索引，发现 {count} 项。{}",
            t.rows.len(),
            t.error.clone().unwrap_or_else(|| if t.incomplete {
                "覆盖不完整（数量或文件大小限制），不能当作全量检查".into()
            } else {
                if t.protected_ids + t.protected_provider > 0 {
                    "部分记录有原生分页位置锚点；不允许改变其字节布局的自动修复".into()
                } else {
                    "未读取会话正文到界面；保留历史记录".into()
                }
            })
        )
    };
    let reparable =
        t.db.is_some() && t.error.is_none() && !t.incomplete && config_ok && !invalid_provider;
    for (id, label, count, fix) in [
        (
            "db_orphan_providers",
            "失效的受管线程 Provider",
            t.orphan_db,
            reparable && t.protected_provider == 0,
        ),
        (
            "rollout_orphan_providers",
            "会话文件的失效受管 Provider",
            t.orphan_rollout,
            reparable && t.protected_provider == 0,
        ),
        (
            "thread_missing_rollouts",
            "会话记录文件存在性",
            t.missing,
            reparable && t.recoverable_missing == t.missing,
        ),
        ("thread_history_integrity", "会话历史结构", t.bad, false),
        (
            "thread_provider_consistency",
            "索引与会话 Provider 一致性",
            t.mismatch,
            reparable && t.protected_provider == 0,
        ),
        (
            "thread_poisoned_instructions",
            "会话基础指令结构",
            t.instruction_issues,
            false,
        ),
        (
            "thread_response_item_ids",
            "历史响应项 ID",
            t.bad_ids,
            reparable && t.unsupported_ids == 0 && t.protected_ids == 0,
        ),
        (
            "native_off_invalid_openai_thread_models",
            "原生线程模型字段",
            t.invalid_models,
            false,
        ),
    ] {
        add(
            &mut items,
            id,
            "runtime",
            label,
            thread_status(count),
            summary(count),
            count > 0 && fix && t.bad == 0 && (t.missing == 0 || id == "thread_missing_rollouts"),
        );
    }
    let model_ids = catalog_data
        .as_ref()
        .and_then(|v| v.as_ref().ok())
        .and_then(|v| v["models"].as_array())
        .map(|a| {
            a.iter()
                .filter_map(|v| v["slug"].as_str().or_else(|| v["id"].as_str()))
                .collect::<BTreeSet<_>>()
        });
    let unmatched = model_ids
        .as_ref()
        .map(|ids| {
            t.rows
                .iter()
                .filter(|r| !r.model.is_empty() && !ids.contains(r.model.as_str()))
                .count()
        })
        .unwrap_or(0);
    add(
        &mut items,
        "thread_model_catalog_consistency",
        "runtime",
        "线程模型与目录一致性",
        if model_ids.is_none() {
            "unknown"
        } else {
            thread_status(unmatched)
        },
        if model_ids.is_none() {
            "未提供可验证的自定义模型目录；不猜测模型是否受账号支持".into()
        } else {
            summary(unmatched)
        },
        false,
    );
    for (id, label, name) in [
        (
            "legacy_migrated_threads",
            "旧版线程迁移记录",
            "legacy-thread-migration.json",
        ),
        (
            "residual_manifest",
            "迁移清单残留",
            "migration-manifest.json",
        ),
        ("residual_cache", "路由缓存引用", "relay-state-cache.json"),
    ] {
        let file = ctx.app_root.join("gateway").join(name);
        let exists = presence(&file);
        add(
            &mut items,
            id,
            "runtime",
            label,
            if exists { "warning" } else { "skipped" },
            if exists {
                "发现遗留文件；归属和恢复语义尚未确认，不自动删除"
            } else {
                "当前 Atelier 未生成此类 AiMaMi 迁移产物"
            },
            false,
        );
    }
    let dirs = [
        ctx.home.clone(),
        ctx.home.join("sessions"),
        ctx.app_root.clone(),
    ];
    let readonly = dirs
        .iter()
        .filter_map(|p| fs::metadata(p).ok())
        .any(|m| m.permissions().readonly());
    add(
        &mut items,
        "codex_home_writable",
        "env",
        "目录权限",
        if !ctx.home.is_dir() {
            "unknown"
        } else if readonly {
            "warning"
        } else {
            "ok"
        },
        "检查已有目录只读标记；不创建探针文件，ACL、磁盘配额和剩余空间未验证",
        false,
    );
    let project = ctx.home.join(".codex-global-state.json");
    let project_result = if !presence(&project) {
        None
    } else {
        Some(
            t.db.as_ref()
                .ok_or_else(|| "没有会话索引，无法验证项目入口完整性".to_string())
                .and_then(|db| super::project_state::inspect_existing(ctx, db)),
        )
    };
    let (status, detail, fixable) = match project_result {
        None => (
            "skipped",
            "项目状态文件尚不存在；诊断不会创建空文件".into(),
            false,
        ),
        Some(Err(e)) => ("unknown", e, false),
        Some(Ok(p)) => (
            if p.issues > 0 { "warning" } else { "ok" },
            p.detail,
            p.issues > 0,
        ),
    };
    add(
        &mut items,
        "project_state_integrity",
        "env",
        "项目状态完整性",
        status,
        detail,
        fixable,
    );
    let relocated = db_dir(ctx, &c) != ctx.home;
    let double = relocated && !databases(&ctx.home).is_empty();
    add(
        &mut items,
        "state_db_relocation",
        "env",
        "会话索引存储布局",
        if !config_ok {
            "unknown"
        } else if double {
            "warning"
        } else {
            "ok"
        },
        if double {
            "sqlite_home 与默认目录同时存在数据库；未自动合并或删除旧库"
        } else {
            "按 sqlite_home 查找当前索引，未发现双目录残留"
        },
        false,
    );
    debug_assert_eq!(items.len(), 35);
    Ok(DiagnosticReport {
        mode: "native".into(),
        has_issues: items
            .iter()
            .any(|i| matches!(i.status.as_str(), "warning" | "error")),
        transition_in_flight: presence(&ctx.app_root.join("gateway/transition.json"))
            || ctx
                .runtime
                .as_ref()
                .is_some_and(|r| matches!(r.state.as_str(), "starting" | "stopping")),
        items,
        checked_at: now_ms(),
        root: ctx.home.display().to_string(),
    })
}
