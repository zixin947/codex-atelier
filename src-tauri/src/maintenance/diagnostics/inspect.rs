use super::*;
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
pub(super) fn transition_in_flight(ctx: &Context) -> bool {
    atelier_maintenance_core::transition_journal::JournalStore::new(
        &ctx.home.join("codexmate/relay"),
    )
    .in_flight()
    .is_some()
        || presence(&ctx.app_root.join("gateway/transition.json"))
        || ctx
            .runtime
            .as_ref()
            .is_some_and(|r| matches!(r.state.as_str(), "starting" | "stopping"))
}
pub(super) fn db_dir(ctx: &Context, c: &toml::Value) -> PathBuf {
    let text = toml::to_string(c).ok();
    atelier_maintenance_core::state_db::configured_sqlite_home(
        text.as_deref(),
        ctx.sqlite_home_env.as_deref(),
        ctx.working_directory.as_deref(),
    )
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
        // AiMaMi 1.2.7 session_meta_instruction_issue @ 0x100393c6c:
        // inspect /payload/base_instructions/text; empty and approval-judge prefix are issues.
        let instructions = m
            .pointer("/base_instructions/text")
            .and_then(Value::as_str)
            .unwrap_or("");
        if instructions.trim().is_empty()
            || instructions
                .trim_start()
                .starts_with("You are judging one planned coding-agent action")
        {
            t.instruction_issues += 1;
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
#[cfg(test)]
thread_local! {
    static SCAN_COUNT: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
}
#[cfg(test)]
pub(super) fn take_scan_count() -> usize {
    SCAN_COUNT.with(|count| count.replace(0))
}
pub(super) fn run(ctx: &Context) -> Result<DiagnosticReport> {
    super::snapshot::with_full_observation(ctx, || run_observed(ctx))
}
fn run_observed(ctx: &Context) -> Result<DiagnosticReport> {
    #[cfg(test)]
    SCAN_COUNT.with(|count| count.set(count.get() + 1));
    let mut items = vec![];
    let config_result = config(ctx);
    let c = config_result
        .clone()
        .unwrap_or(toml::Value::Table(Default::default()));
    let config_ok = config_result.is_ok();
    let config_access = super::access_checks::observe_config(&ctx.config());
    let writable = config_access
        .as_ref()
        .is_ok_and(|o| o.exists && o.is_regular_file && !o.is_symlink && !o.readonly);
    let (access_status, access_label, access_detail) = if !ctx.config().exists() {
        (
            "ok",
            "config.toml 尚不存在（正常）",
            "配置文件尚不存在，Codex 可使用默认配置；不创建文件",
        )
    } else if read_bounded(&ctx.config(), 1024 * 1024).is_ok() {
        ("ok", "config.toml 可正常读取", "读取现有配置，不创建文件")
    } else {
        (
            "error",
            "config.toml 无法读取",
            "路由不探测或刷新 OpenAI Token，登录交给 ChatGPT",
        )
    };
    add(
        &mut items,
        "config_access",
        "config",
        access_label,
        access_status,
        access_detail,
        false,
    );
    let (replace_status, replace_detail) = match config_access {
        Ok(o) if !o.exists => ("ok", "配置文件不存在，未创建文件".into()),
        Ok(o) if !o.is_regular_file || o.is_symlink => {
            ("error", "配置不是普通文件或为符号链接，不能安全替换".into())
        }
        Ok(o) if o.readonly => ("warning", "配置权限位标记为只读，未修改权限".into()),
        Ok(_) => (
            "ok",
            "元数据与类型检查通过；与原版一致未执行配置试写，ACL、实际替换及沙盒权限仍需写入时确认"
                .into(),
        ),
        Err(e) => (
            "warning",
            format!("无法核对配置元数据：{e}；未执行写入或权限变更"),
        ),
    };
    add(
        &mut items,
        "config_replace_readiness",
        "config",
        "config.toml 权限未显示只读",
        replace_status,
        replace_detail,
        false,
    );
    // AiMaMi 1.2.7 decomp-check_config_toml_syntax.c：
    // label 承载状态文案（空文件/不存在/错误/正常），status = ok|error，error 可抢救修复。
    let syntax_present = ctx.config().exists();
    let blank_file = syntax_present
        && read_bounded(&ctx.config(), 1024 * 1024)
            .ok()
            .and_then(|b| std::str::from_utf8(&b).ok().map(|s| s.trim().is_empty()))
            .unwrap_or(false);
    let syntax_label = if !syntax_present {
        "TOML 语法（文件不存在，跳过）"
    } else if blank_file {
        "TOML 语法正常（空文件）"
    } else if config_ok {
        "TOML 语法正常"
    } else {
        "config.toml TOML 语法错误"
    };
    add(
        &mut items,
        "config_toml_syntax",
        "config",
        syntax_label,
        if config_ok { "ok" } else { "error" },
        // 原版 ok 分支 detail 为空（ok_item 只写 id/label）；错误分支带解析器原文。
        if config_ok {
            String::new()
        } else {
            super::config_syntax::check_config_toml_syntax(
                &read_bounded(&ctx.config(), 1024 * 1024)
                    .ok()
                    .and_then(|v| String::from_utf8(v).ok())
                    .unwrap_or_default(),
                ctx.config().exists(),
            )
            .2
        },
        !config_ok,
    );
    let p = provider(&c);
    let native_config_text = read_bounded(&ctx.config(), 1024 * 1024)
        .ok()
        .and_then(|v| String::from_utf8(v).ok());
    match super::native_config::third_party(ctx, native_config_text.as_deref(), writable) {
        Ok(item) => items.push(item),
        Err(_) => add(
            &mut items,
            "config_third_party",
            "config",
            "第三方注入残留",
            "warning",
            "实际配置或已保存的 Provider 来源无法完整核对；未使用空集合推断为健康",
            false,
        ),
    }
    if config_ok {
        items.push(super::native_config::profile(
            native_config_text.as_deref(),
            owned_provider(&p),
            writable,
        )?);
        items.push(super::native_config::global(
            native_config_text.as_deref(),
            &ctx.home.join("codexmate/relay"),
            writable,
        )?);
    } else {
        for (id, label) in [
            ("config_profile_conflict", "顶层 profile 配置"),
            ("config_global_validity", "本地配置引用"),
        ] {
            add(
                &mut items,
                id,
                "config",
                label,
                "warning",
                "先修复 TOML 语法，再检查配置引用",
                false,
            );
        }
    }
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
        "Codex 配置一致性正常",
        if !managed {
            "ok"
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
        "aimai1 HTTP-only 路由契约正常",
        if !managed {
            "ok"
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
            "路由恢复记录可正常读取",
            "gateway/recovery.json",
        ),
        (
            "router_postcondition",
            "目标态非配置维度稳定（配置偏差由 config_stale 报告）",
            "gateway/transition.json",
        ),
        (
            "takeover_backup_orphan",
            "无 takeover 备份残留",
            "gateway/takeover-backup.json",
        ),
        (
            "router_unlock_auth_residue",
            "无虚拟账号残留",
            "gateway/auth-takeover.json",
        ),
    ] {
        let exists = presence(&ctx.app_root.join(path));
        add(
            &mut items,
            id,
            // 原版分组表：router_unlock_auth_residue 属 auth；
            // router_recovery_record / router_postcondition 未在表内具名，回退 runtime。
            if id == "router_unlock_auth_residue" {
                "auth"
            } else if id == "takeover_backup_orphan" {
                "config"
            } else {
                "runtime"
            },
            label,
            if exists { "warning" } else { "ok" },
            if exists {
                "发现相关状态文件，需要通过创建它的路由版本恢复；诊断不会提交或删除恢复记录"
            } else {
                "Atelier 当前没有此类接管记录；不读取旧版工具的私有状态"
            },
            false,
        );
    }
    let proxy_status = if !managed {
        "ok"
    } else {
        match ctx.runtime.as_ref().map(|r| r.state.as_str()) {
            Some("running") => "ok",
            Some("error" | "stopped") => "warning",
            _ => "warning",
        }
    };
    let proxy_label = if !managed {
        "本地代理（智能路由未启用，跳过检测）"
    } else if proxy_status == "ok" {
        "本地代理运行中"
    } else {
        "本地代理未运行"
    };
    add(
        &mut items,
        "proxy_running",
        "runtime",
        proxy_label,
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
        if catalog.is_none() {
            "Catalog 路径（路由未启用，跳过）"
        } else if catalog.as_ref().is_some_and(|p| p.is_file()) {
            "Catalog 路径有效"
        } else {
            "model_catalog_json 指向的文件不存在"
        },
        if catalog.is_none() {
            "ok"
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
        if catalog.is_none() {
            "Catalog 完整性（路由未启用，跳过）"
        } else if catalog_valid {
            "Catalog 完整"
        } else {
            "Catalog 文件 JSON 格式损坏"
        },
        if catalog.is_none() {
            "ok"
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
        "尚未观测到目录请求，不能据此确认官方目录可用",
        "warning",
        "未联网获取官方目录，也没有可验证的最近请求记录",
        false,
    );
    let auth = ctx.home.join("auth.json");
    let non_file_auth = matches!(
        c.get("cli_auth_credentials_store").and_then(|v| v.as_str()),
        Some("keyring" | "ephemeral")
    );
    let a = if !non_file_auth && presence(&auth) {
        Some(json_file(&auth))
    } else {
        None
    };
    let av = a.as_ref().and_then(|v| v.as_ref().ok());
    if non_file_auth {
        add(
            &mut items,
            "auth_integrity",
            "auth",
            "文件认证完整性",
            "ok",
            "当前认证存储不是文件模式；不读取或恢复 auth.json，不访问钥匙串",
            false,
        );
    } else {
        let check = atelier_maintenance_core::auth::check_auth_integrity(
            &atelier_maintenance_core::auth::AuthContext::new(
                &ctx.home,
                atelier_maintenance_core::auth::RouterMode::Off,
            ),
        );
        use atelier_maintenance_core::auth::Level;
        let status = match check.level {
            Level::Ok => "ok",
            Level::Warning => "warning",
            Level::Error => "error",
        };
        add(&mut items,"auth_integrity","auth",check.label(),status,
            "本地文件检查，不验证在线登录状态、签名或额度；恢复候选来自 accounts/backups 并按修改时间选择，不显示凭据",
            config_ok && check.fixable);
    }
    items.extend(super::credential_checks::inspect(
        ctx,
        &c,
        av,
        (now_ms() / 1000) as i64,
    ));
    add(
        &mut items,
        "codex_process_state",
        "runtime",
        if ctx.desktop_running {
            "Codex 运行中"
        } else {
            "Codex 未运行"
        },
        // 原版 codex_process_state 的两个分支都走 ok_item：未运行不是故障。
        "ok",
        if ctx.desktop_running {
            "检测到 Desktop 进程；文件修复需要先退出客户端"
        } else {
            "未检测到已识别的 Desktop 主进程；不自动启动"
        },
        false,
    );
    // Native observers below replace these slots. Never run the discarded legacy scan.
    for (id, label) in [
        ("db_orphan_providers", "失效的受管线程 Provider"),
        ("rollout_orphan_providers", "会话文件的失效受管 Provider"),
        ("thread_missing_rollouts", "会话记录文件存在性"),
        ("thread_provider_consistency", "索引与会话 Provider 一致性"),
        ("thread_poisoned_instructions", "会话基础指令结构"),
        ("thread_response_item_ids", "历史响应项 ID"),
        ("thread_model_catalog_consistency", "线程模型与目录一致性"),
    ] {
        add(
            &mut items,
            id,
            "runtime",
            label,
            "warning",
            "需有效配置及稳定线程观察后确认",
            false,
        );
    }
    items.extend(super::thread_observation::inspect(ctx));
    for (id, label, name) in [(
        "legacy_migrated_threads",
        "旧版线程迁移记录",
        "legacy-thread-migration.json",
    )] {
        let file = ctx.app_root.join("gateway").join(name);
        let exists = presence(&file);
        add(
            &mut items,
            id,
            "runtime",
            label,
            if exists { "warning" } else { "ok" },
            if exists {
                "发现遗留文件；归属和恢复语义尚未确认，不自动删除"
            } else {
                "当前 Atelier 未生成此类旧版工具迁移产物"
            },
            false,
        );
    }
    for kind in super::residual_cleanup::Kind::ALL {
        let state = if config_ok {
            kind.existing(ctx)
                .and_then(|files| Ok((files, kind.repairable(ctx, &c)?)))
        } else {
            Err("配置无法解析，尚不能确认路由状态".into())
        };
        let (status, detail, fixable) = match state {
            Ok((files, _)) if files.is_empty() => ("ok", "无路由残留文件".into(), false),
            Ok((_, inactive)) if inactive.is_empty() => (
                "ok",
                "文件存在，所属路由启用中正常使用；不会删除".into(),
                false,
            ),
            Ok((_, files)) => (
                "warning",
                format!(
                    "路由已关闭，发现 {} 个残留文件；修复将先备份再删除",
                    files.len()
                ),
                true,
            ),
            Err(e) => ("warning", e, false),
        };
        add(
            &mut items,
            kind.id(),
            "runtime",
            kind.label(),
            status,
            detail,
            fixable,
        );
    }
    let dirs = super::access_checks::native_directories(&ctx.home);
    let (mut directory_status, mut directory_detail) =
        match super::access_checks::observe_directories(&dirs) {
            Ok(observations) => {
                let invalid = observations
                    .iter()
                    .filter(|d| d.exists && (!d.is_directory || d.is_symlink || d.readonly))
                    .count();
                let missing = observations.iter().filter(|d| !d.exists).count();
                let home_exists = observations
                    .first()
                    .is_some_and(|d| d.exists && d.is_directory && !d.is_symlink);
                (
                    // 原版只有元数据确实异常才报 error；「未验证写入」本身不构成问题。
                    if invalid > 0 { "error" } else { "ok" },
                    format!("检查原版四目录 home、sessions、codexmate、codexmate/relay：{invalid} 项元数据异常，{missing} 个缺失；home存在且为目录：{home_exists}。"),
                )
            }
            Err(e) => (
                "warning",
                format!("目录元数据检查失败：{e}；未将读取错误当作健康结果"),
            ),
        };
    let mut space_bytes = None;
    match super::access_checks::observe_available_space(&ctx.home) {
        Ok(space) => {
            space_bytes = Some(space.available_bytes);
            if space.level == "error" {
                directory_status = "error";
            } else if space.level == "warning" && directory_status != "error" {
                directory_status = "warning";
            }
            directory_detail.push_str(&format!(
                " 可用空间 {} MiB（{}）；源数据库读写能力未验证。",
                space.available_bytes / (1024 * 1024),
                space.level
            ));
        }
        Err(e) => {
            directory_detail.push_str(&format!(" 空间查询未完成：{e}；源数据库读写能力未验证。"))
        }
    }
    // label 与 status 必须同源：都从同一个磁盘读数推导，避免出现
    // 「文件系统正常」却标 warning 这种自相矛盾。
    let (directory_label, directory_status) = match space_bytes {
        Some(bytes) => (
            super::access_checks::space_label(bytes),
            super::access_checks::space_level(bytes).to_string(),
        ),
        None => (
            "文件系统正常（空间检测不可用）".to_string(),
            "ok".to_string(),
        ),
    };
    add(
        &mut items,
        "codex_home_writable",
        "env",
        directory_label.as_str(),
        directory_status.as_str(),
        directory_detail,
        false,
    );
    let project = super::project_state::inspect(ctx);
    items.push(project);
    if config_ok {
        items.push(super::state_db_relocation::inspect(ctx, &c)?);
    } else {
        add(
            &mut items,
            "state_db_relocation",
            "env",
            "线程数据库位置正常",
            "warning",
            "先修复配置语法，再确认数据库布局",
            false,
        );
    }
    // Replace the old approximations with the evidence-backed checks. The adapter
    // copies DB/WAL into private snapshots, and never opens a source DB for diagnosis.
    if config_ok {
        for native in super::native_threads::inspect(ctx) {
            if let Some(item) = items.iter_mut().find(|i| i.id == native.id) {
                *item = native;
            }
        }
        for native in super::native_orphans::inspect(ctx) {
            if let Some(item) = items.iter_mut().find(|i| i.id == native.id) {
                *item = native;
            }
        }
    }
    if config_ok {
        match super::native_auth::resolve_mode(ctx) {
            Ok(mode) => {
                for native in super::native_auth::inspect(ctx, mode) {
                    if let Some(item) = items.iter_mut().find(|i| i.id == native.id) {
                        *item = native;
                    }
                }
            }
            Err(_) => {
                for item in items
                    .iter_mut()
                    .filter(|i| super::native_auth::handles(&i.id))
                {
                    item.status = "warning".into();
                    item.detail =
                        "路由模式状态无法核对，未推断为关闭；请检查状态文件后重新检测".into();
                    item.fixable = false;
                }
            }
        }
    }
    if config_ok {
        match super::native_catalog::context_source(ctx) {
            Ok(source) => {
                let raw = fs::read_to_string(ctx.config()).ok();
                match super::native_catalog::inspect(&source, raw.as_deref(), true) {
                    Ok(native) => {
                        for mut native in native {
                            native.fixable &= super::native_catalog::prepare_item(
                                &source,
                                &ctx.config(),
                                &native.id,
                            )
                            .is_ok();
                            if let Some(item) = items.iter_mut().find(|i| i.id == native.id) {
                                *item = native;
                            }
                        }
                    }
                    Err(_) => {
                        for item in items
                            .iter_mut()
                            .filter(|i| super::native_catalog::handles(&i.id))
                        {
                            item.status = "warning".into();
                            item.fixable = false;
                            item.detail = "模型目录来源无法完整核对；未使用空目录替代".into();
                        }
                    }
                }
            }
            Err(_) => {
                for item in items
                    .iter_mut()
                    .filter(|i| super::native_catalog::handles(&i.id))
                {
                    item.status = "warning".into();
                    item.fixable = false;
                    item.detail =
                        "模型目录来源选择失败，无法完整核对；未沿用旧近似结果或使用空目录替代"
                            .into();
                }
            }
        }
    }
    debug_assert_eq!(items.len(), 34);
    Ok(DiagnosticReport {
        mode: "native".into(),
        has_issues: items
            .iter()
            .any(|i| matches!(i.status.as_str(), "warning" | "error")),
        transition_in_flight: transition_in_flight(ctx),
        items,
        checked_at: now_ms(),
        root: ctx.home.display().to_string(),
    })
}
