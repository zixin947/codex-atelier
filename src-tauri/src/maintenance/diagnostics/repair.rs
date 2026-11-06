use super::*;
use crate::gateway::storage::{atomic_private, open_private, private_dir};
use fs2::FileExt;
use rusqlite::params;
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::collections::BTreeSet;
#[derive(Clone)]
struct FileChange {
    path: PathBuf,
    before: Vec<u8>,
    after: Vec<u8>,
}
#[derive(Clone)]
struct RowChange {
    id: String,
    before: String,
    after: String,
    path: String,
    after_path: String,
}
#[derive(Clone)]
pub(super) struct Ticket {
    pub view: DiagnosticRepairPlan,
    pub created_at: u64,
    home: PathBuf,
    app_root: PathBuf,
    config_hash: String,
    gateway_hash: Option<String>,
    files: Vec<FileChange>,
    db: Option<PathBuf>,
    rows: Vec<RowChange>,
    cwds: Vec<super::project_state::CwdUpdate>,
}
fn digest(b: &[u8]) -> String {
    format!("{:x}", Sha256::digest(b))
}
fn config_hash(ctx: &Context) -> Result<String> {
    if ctx.config().exists() {
        Ok(digest(&read_bounded(&ctx.config(), 1024 * 1024)?))
    } else {
        Ok("absent".into())
    }
}
fn change_rollout(bytes: &[u8], id: &str, target: &str) -> Result<Vec<u8>> {
    if super::record_compat::has_positions(bytes)? {
        return Err("会话含原生分页位置锚点，不能重写元数据字节布局".into());
    }
    let mut out = vec![];
    let mut seen = false;
    for line in bytes.split_inclusive(|b| *b == b'\n') {
        let body = if line.ends_with(b"\n") {
            &line[..line.len() - 1]
        } else {
            line
        };
        if body.is_empty() {
            out.extend_from_slice(line);
            continue;
        }
        let mut v: Value = serde_json::from_slice(body).map_err(|_| "损坏的记录不能自动迁移")?;
        if v["type"] == "session_meta" {
            if seen || v["payload"]["id"].as_str() != Some(id) {
                return Err("会话身份不一致，不能迁移".into());
            }
            seen = true;
            v["payload"]["model_provider"] = target.into();
            out.extend(serde_json::to_vec(&v).map_err(err)?);
            if line.ends_with(b"\n") {
                out.push(b'\n')
            }
        } else {
            out.extend_from_slice(line)
        }
    }
    if !seen {
        return Err("缺少会话元数据，不能迁移".into());
    }
    Ok(out)
}
fn remove_setting(doc: &mut toml_edit::DocumentMut, key: &str, comments: &mut String) {
    if let Some(prefix) = doc
        .as_table()
        .key(key)
        .and_then(|k| k.leaf_decor().prefix())
        .and_then(|r| r.as_str())
    {
        comments.push_str(prefix)
    }
    if let Some(suffix) = doc
        .get(key)
        .and_then(|v| v.as_value())
        .and_then(|v| v.decor().suffix())
        .and_then(|r| r.as_str())
    {
        if suffix.contains('#') {
            comments.push_str(suffix.trim_start());
            comments.push('\n')
        }
    }
    doc.remove(key);
}
pub(super) fn preview(ctx: &Context, item_id: &str) -> Result<Ticket> {
    let report = inspect::run(ctx)?;
    let selected = report
        .items
        .iter()
        .filter(|i| i.fixable && (item_id == "all" || item_id == i.id))
        .collect::<Vec<_>>();
    if selected.is_empty() {
        return Err("当前没有可自动修复的项目；请重新检测并展开问题详情".into());
    }
    let item_ids = selected.iter().map(|i| i.id.clone()).collect::<Vec<_>>();
    let mut changes = vec![];
    let mut warnings = vec!["会先保存备份，校验外部变更；确认后才修改上述文件。".into()];
    let mut files = vec![];
    let c = inspect::config(ctx)?;
    let original = if ctx.config().exists() {
        read_bounded(&ctx.config(), 1024 * 1024)?
    } else {
        vec![]
    };
    let mut doc = std::str::from_utf8(&original)
        .map_err(|_| "配置不是 UTF-8")?
        .parse::<toml_edit::DocumentMut>()
        .map_err(|_| "配置解析失败")?;
    let mut preserved_comments = String::new();
    for id in &item_ids {
        match id.as_str() {
            "config_profile_conflict" => {
                remove_setting(&mut doc, "profile", &mut preserved_comments);
                changes.push("移除不存在的顶层 profile 选择；保留所有 profiles 和注释".into())
            }
            "config_stale" | "router_http_contract" => {
                let gateway: Value = serde_json::from_slice(&read_bounded(
                    &ctx.app_root.join("gateway/catalog.json"),
                    8 * 1024 * 1024,
                )?)
                .map_err(|_| "网关设置无效")?;
                let port = gateway["settings"]["port"]
                    .as_u64()
                    .filter(|p| *p > 0 && *p <= 65535)
                    .ok_or("网关端口无效")?;
                let provider = inspect::provider(&c);
                let section = &mut doc["model_providers"][&provider];
                section["base_url"] = toml_edit::value(format!("http://127.0.0.1:{port}/v1"));
                section["wire_api"] = toml_edit::value("responses");
                section["env_key"] = toml_edit::value("ATELIER_GATEWAY_TOKEN");
                section["requires_openai_auth"] = toml_edit::value(false);
                section["supports_websockets"] = toml_edit::value(false);
                if !changes.iter().any(|s: &String| s.starts_with("修正受管")) {
                    changes.push(format!("修正受管 {provider} 连接到本机端口 {port} /v1，使用 Responses、环境变量认证并关闭 WebSocket；不写入密钥"))
                }
            }
            "config_global_validity" => {
                if inspect::profile_provider(&c).is_some() {
                    let name = c
                        .get("profile")
                        .and_then(|v| v.as_str())
                        .ok_or("工作配置已变化")?;
                    doc["profiles"][name]
                        .as_table_like_mut()
                        .ok_or("工作配置不是有效表")?
                        .remove("model_provider");
                    changes.push(format!("移除工作配置 {name} 中未定义的 model_provider 选择，回退到顶层配置；保留其他设置"));
                } else {
                    remove_setting(&mut doc, "model_provider", &mut preserved_comments);
                    changes.push("移除未定义的顶层 model_provider 选择；后续默认使用 openai，保留其他 Provider 定义".into());
                }
            }
            _ => {}
        }
    }
    let config_after = format!("{preserved_comments}{doc}").into_bytes();
    if original != config_after {
        files.push(FileChange {
            path: ctx.config(),
            before: original,
            after: config_after,
        })
    }
    let mut db = None;
    let mut rows = vec![];
    let mut cwds = vec![];
    if item_ids.iter().any(|id| {
        matches!(
            id.as_str(),
            "thread_provider_consistency" | "db_orphan_providers" | "rollout_orphan_providers"
        )
    }) {
        let t = inspect::scan_threads(ctx, &c);
        if t.incomplete || t.error.is_some() || t.bad > 0 || t.missing > 0 {
            return Err("会话扫描不完整，不能自动迁移；请关闭客户端后重新检测".into());
        }
        db = t.db;
        let target = inspect::provider(&c);
        for row in t.rows {
            let path = inspect::resolve(ctx, &row.path)?;
            let decoded = inspect::rollout(&path)?;
            let meta = inspect::meta(&decoded)?;
            let rp = meta["model_provider"].as_str().unwrap_or("openai");
            if row.provider == target && rp == target {
                continue;
            }
            if meta["id"].as_str() != Some(row.id.as_str()) {
                return Err("会话身份不匹配".into());
            }
            let after = change_rollout(&decoded, &row.id, &target)?;
            let before = read_bounded(&path, inspect::LIMIT)?;
            let after = if path.extension().is_some_and(|v| v == "zst") {
                zstd::stream::encode_all(after.as_slice(), 3).map_err(err)?
            } else {
                after
            };
            files.push(FileChange {
                path,
                before,
                after,
            });
            rows.push(RowChange {
                id: row.id,
                before: row.provider,
                after: target.clone(),
                after_path: row.path.clone(),
                path: row.path,
            });
        }
        changes.push(format!(
            "将 {} 条会话索引及对应记录元数据的 Provider 一起对齐到 {target}",
            rows.len()
        ));
        warnings.push("只改 session_meta 的 Provider；正文原字节保留，加密内容不转换，跨账号续聊仍可能被上游拒绝。".into());
    }
    if item_ids.iter().any(|id| id == "thread_missing_rollouts") {
        let t = inspect::scan_threads(ctx, &c);
        if t.incomplete || t.error.is_some() {
            return Err("扫描不完整，不能修复记录路径".into());
        }
        db = t.db;
        for row in t.rows {
            if inspect::resolve(ctx, &row.path).is_ok() {
                continue;
            }
            let recovered =
                inspect::recover_path(ctx, &row.id).ok_or("记录路径无唯一匹配，请重新检测")?;
            rows.push(RowChange {
                id: row.id,
                before: row.provider.clone(),
                after: row.provider,
                path: row.path,
                after_path: recovered.display().to_string(),
            });
        }
        changes.push(format!(
            "将 {} 条失效索引重新指向本地唯一且 session_meta.id 匹配的既有记录；不新建历史",
            rows.len()
        ));
    }
    if item_ids.iter().any(|id| id == "thread_response_item_ids") {
        let t = inspect::scan_threads(ctx, &c);
        if t.incomplete
            || t.error.is_some()
            || t.bad > 0
            || t.missing > 0
            || t.protected_ids > 0
            || t.unsupported_ids > 0
        {
            return Err("会话ID检查不完整或包含受保护布局，不能自动修复".into());
        }
        let mut count = 0;
        for row in t.rows {
            let path = inspect::resolve(ctx, &row.path)?;
            let prior = files.iter().position(|f| f.path == path);
            let raw = if let Some(i) = prior {
                files[i].after.clone()
            } else {
                read_bounded(&path, inspect::LIMIT)?
            };
            let decoded = inspect::decode(&path, raw)?;
            let n = super::record_compat::inspect(&decoded)?;
            if n == 0 {
                continue;
            }
            let repaired = super::record_compat::repair(&decoded)?;
            let after = if path.extension().is_some_and(|v| v == "zst") {
                zstd::stream::encode_all(repaired.as_slice(), 3).map_err(err)?
            } else {
                repaired
            };
            if let Some(i) = prior {
                files[i].after = after
            } else {
                files.push(FileChange {
                    before: read_bounded(&path, inspect::LIMIT)?,
                    path,
                    after,
                })
            }
            count += n;
        }
        changes.push(format!(
            "兼容修复 {count} 个历史工具调用项 ID；不改 call_id 或会话正文，拒绝分页位置锚点布局"
        ));
    }
    if item_ids.iter().any(|id| id == "project_state_integrity") {
        let current_db = inspect::databases(&inspect::db_dir(ctx, &c))
            .pop()
            .ok_or("没有可用于项目修复的会话索引")?;
        let plan = super::project_state::inspect_existing(ctx, &current_db)?;
        cwds = super::project_state::cwd_repairs(ctx, &current_db)?;
        if !cwds.is_empty() {
            db = Some(current_db);
            changes.push(format!(
                "从身份匹配的现有会话元数据回填 {} 条索引缺失的 cwd；不覆盖已有工作目录",
                cwds.len()
            ))
        }
        if let Some(after) = plan.after {
            files.push(FileChange {
                path: ctx.home.join(".codex-global-state.json"),
                before: plan.before,
                after,
            });
            changes.push(plan.detail)
        }
    }
    if files.is_empty() && rows.is_empty() && cwds.is_empty() {
        return Err("没有实际文件变更，请重新检测".into());
    }
    if files
        .iter()
        .map(|f| f.before.len() + f.after.len())
        .sum::<usize>()
        > 256 * 1024 * 1024
    {
        return Err("本次修复文件超过256MiB，请在会话管理中分批处理".into());
    }
    changes.extend(
        files
            .iter()
            .take(8)
            .map(|f| format!("备份并更新 {}", f.path.display())),
    );
    if files.len() > 8 {
        changes.push(format!(
            "另有 {} 个会话文件，全部保存在备份清单",
            files.len() - 8
        ))
    }
    warnings.push(
        "请先退出 Codex / ChatGPT Desktop 并结束 codex CLI。Atelier 不会替你强制结束客户端。"
            .into(),
    );
    Ok(Ticket {
        view: DiagnosticRepairPlan {
            ticket: uuid::Uuid::new_v4().to_string(),
            item_ids: item_ids.clone(),
            title: if item_id == "all" {
                "预览全部可修复项目".into()
            } else {
                "预览修复".into()
            },
            changes,
            warnings,
            requires_desktop_closed: true,
        },
        created_at: now_ms(),
        home: ctx.home.clone(),
        app_root: ctx.app_root.clone(),
        config_hash: config_hash(ctx)?,
        gateway_hash: if item_ids
            .iter()
            .any(|id| matches!(id.as_str(), "config_stale" | "router_http_contract"))
        {
            Some(digest(&read_bounded(
                &ctx.app_root.join("gateway/catalog.json"),
                8 * 1024 * 1024,
            )?))
        } else {
            None
        },
        files,
        db,
        rows,
        cwds,
    })
}
fn restore(files: &[&FileChange]) -> Vec<String> {
    files
        .iter()
        .rev()
        .filter_map(|f| match read_bounded(&f.path, inspect::LIMIT) {
            Ok(current) if current == f.after => atomic_private(&f.path, &f.before)
                .err()
                .map(|_| format!("恢复 {} 失败；请使用备份", f.path.display())),
            Ok(current) if current == f.before => None,
            _ => Some(format!(
                "{} 已被外部更新，未覆盖；请对照备份手动恢复",
                f.path.display()
            )),
        })
        .collect()
}
pub(super) fn apply(ctx: &Context, t: Ticket) -> Result<DiagnosticFixResult> {
    if now_ms().saturating_sub(t.created_at) > 300_000 {
        return Err("修复预览已过期，请重新检测".into());
    }
    if ctx.home != t.home || ctx.app_root != t.app_root {
        return Err("修复目标目录已变化".into());
    }
    if ctx.desktop_running {
        return Err("Desktop 仍在运行；请退出客户端后重新预览。未修改任何文件。".into());
    }
    if ctx.runtime.as_ref().is_some_and(|r| r.active_requests > 0) {
        return Err("本地网关仍有在途请求，请等待完成后重新预览".into());
    }
    // Exact same lock as other configuration writers in Maintenance.
    let lock_dir = ctx.app_root.join("maintenance");
    private_dir(&lock_dir)?;
    let lock = open_private(&lock_dir.join("codex-write.lock"))?;
    lock.try_lock_exclusive()
        .map_err(|_| "另一维护操作正在修改 Codex")?;
    let thread_dir = ctx.app_root.join("thread-backups");
    private_dir(&thread_dir)?;
    let tl = open_private(&thread_dir.join("mutation.lock"))?;
    tl.try_lock_exclusive()
        .map_err(|_| "线程管理正在执行修改")?;
    if config_hash(ctx)? != t.config_hash {
        return Err("配置已被外部更新，请重新预览；未修改文件".into());
    }
    if let Some(expected) = &t.gateway_hash {
        if digest(&read_bounded(
            &ctx.app_root.join("gateway/catalog.json"),
            8 * 1024 * 1024,
        )?) != *expected
        {
            return Err("网关配置已变化，请重新预览".into());
        }
    }
    for f in &t.files {
        if read_bounded(&f.path, inspect::LIMIT)? != f.before {
            return Err("待修复文件已被外部更新，请重新预览；未修改文件".into());
        }
    }
    let c = inspect::config(ctx)?;
    if let Some(db) = &t.db {
        if inspect::databases(&inspect::db_dir(ctx, &c)).pop().as_ref() != Some(db) {
            return Err("会话索引已变化".into());
        }
    }
    if t.view
        .item_ids
        .iter()
        .any(|id| id == "project_state_integrity")
    {
        let database = inspect::databases(&inspect::db_dir(ctx, &c))
            .pop()
            .ok_or("项目索引已不存在")?;
        let current = super::project_state::inspect_existing(ctx, &database)?;
        let expected = t
            .files
            .iter()
            .find(|f| f.path == ctx.home.join(".codex-global-state.json"))
            .map(|f| f.after.as_slice());
        if current.after.as_deref() != expected {
            return Err("项目索引已被外部更新，请重新预览".into());
        }
        let updates = super::project_state::cwd_repairs(ctx, &database)?;
        if updates.len() != t.cwds.len()
            || updates.iter().any(|u| {
                !t.cwds.iter().any(|v| {
                    u.id == v.id && u.before == v.before && u.after == v.after && u.path == v.path
                })
            })
        {
            return Err("工作目录修复来源已变化，请重新预览".into());
        }
    }
    let backup = lock_dir.join("diagnostic-backups").join(&t.view.ticket);
    private_dir(&backup)?;
    let manifest = json!({"version":1,"createdAt":now_ms(),"itemIds":t.view.item_ids,"database":t.db,"cwdUpdates":t.cwds,"files":t.files.iter().enumerate().map(|(i,f)|json!({"path":f.path,"backup":format!("{i:04}.bin"),"sha256":digest(&f.before),"afterSha256":digest(&f.after)})).collect::<Vec<_>>(),"rows":t.rows.iter().map(|r|json!({"id":r.id,"before":r.before,"after":r.after,"rolloutPath":r.path,"afterRolloutPath":r.after_path})).collect::<Vec<_>>()});
    for (i, f) in t.files.iter().enumerate() {
        atomic_private(&backup.join(format!("{i:04}.bin")), &f.before)?
    }
    atomic_private(
        &backup.join("manifest.json"),
        &serde_json::to_vec_pretty(&manifest).map_err(err)?,
    )?;
    let mut connection =
        t.db.as_ref()
            .map(|db| inspect::open_db(db, true))
            .transpose()?;
    let mut tx = connection
        .as_mut()
        .map(|db| {
            db.transaction_with_behavior(rusqlite::TransactionBehavior::Immediate)
                .map_err(err)
        })
        .transpose()?;
    if let Some(tx) = &tx {
        for row in &t.rows {
            let existing: (String, String) = tx
                .query_row(
                    "SELECT COALESCE(model_provider,''),rollout_path FROM threads WHERE id=?1",
                    [&row.id],
                    |r| Ok((r.get(0)?, r.get(1)?)),
                )
                .map_err(|_| "会话索引已变化")?;
            if row.after_path != row.path {
                let resolved = inspect::resolve(ctx, &row.after_path)?;
                if inspect::meta(&inspect::rollout(&resolved)?)?["id"].as_str()
                    != Some(row.id.as_str())
                {
                    return Err("恢复目标的会话身份已变化".into());
                }
            }
            if existing != (row.before.clone(), row.path.clone()) {
                return Err("会话已被外部修改，请重新预览；未修改文件".into());
            }
        }
    }
    if let Some(tx) = &tx {
        for change in &t.cwds {
            let existing: (String, String) = tx
                .query_row(
                    "SELECT COALESCE(cwd,''),rollout_path FROM threads WHERE id=?1",
                    [&change.id],
                    |r| Ok((r.get(0)?, r.get(1)?)),
                )
                .map_err(|_| "项目索引已变化")?;
            if existing != (change.before.clone(), change.path.clone()) {
                return Err("线程工作目录已被外部修改，未执行修复".into());
            }
            let path = inspect::resolve(ctx, &change.path)?;
            let meta = inspect::meta(&inspect::rollout(&path)?)?;
            if meta["id"].as_str() != Some(change.id.as_str())
                || meta["cwd"]
                    .as_str()
                    .map(|s| s.trim().trim_end_matches(['/', '\\']))
                    != Some(change.after.as_str())
            {
                return Err("会话工作目录已被外部修改，未执行修复".into());
            }
        }
    }
    let mut written = vec![];
    let outcome = (|| -> Result<()> {
        for f in &t.files {
            if read_bounded(&f.path, inspect::LIMIT)? != f.before {
                return Err("应用期间文件发生变化".into());
            }
            atomic_private(&f.path, &f.after)?;
            written.push(f);
            if read_bounded(&f.path, inspect::LIMIT)? != f.after {
                return Err("写入读回校验失败".into());
            }
        }
        if let Some(tx) = &tx {
            for row in &t.rows {
                let n=tx.execute("UPDATE threads SET model_provider=?1,rollout_path=?5 WHERE id=?2 AND COALESCE(model_provider,'')=?3 AND rollout_path=?4",params![row.after,row.id,row.before,row.path,row.after_path]).map_err(err)?;
                if n != 1 {
                    return Err("会话更新数量不一致".into());
                }
            }
        }
        if let Some(tx) = &tx {
            for change in &t.cwds {
                let n = tx
                    .execute(
                        "UPDATE threads SET cwd=?1 WHERE id=?2 AND COALESCE(cwd,'')=?3",
                        params![change.after, change.id, change.before],
                    )
                    .map_err(err)?;
                if n != 1 {
                    return Err("工作目录更新数量不一致".into());
                }
            }
        }
        Ok(())
    })();
    if let Err(e) = outcome {
        drop(tx);
        let errors = restore(&written);
        return Err(format!(
            "修复失败：{e}。{} 备份：{}",
            if errors.is_empty() {
                "已恢复写入的文件，索引事务已回滚".into()
            } else {
                errors.join("；")
            },
            backup.display()
        ));
    }
    if let Some(tx) = tx.take() {
        if let Err(e) = tx.commit() {
            let errors = restore(&written);
            return Err(format!(
                "索引提交失败：{e}；文件恢复{}。备份：{}",
                if errors.is_empty() {
                    "成功"
                } else {
                    "未全部成功"
                },
                backup.display()
            ));
        }
    }
    // Close the write connection before re-inspection; SQLite may checkpoint its own WAL here.
    drop(tx);
    drop(connection);
    let report = inspect::run(ctx)?;
    let requested = t.view.item_ids.iter().cloned().collect::<BTreeSet<_>>();
    let mut fixed_ids = vec![];
    let mut failed_ids = vec![];
    for id in requested {
        if report.items.iter().any(|i| i.id == id && i.status == "ok") {
            fixed_ids.push(id)
        } else {
            failed_ids.push(id)
        }
    }
    let mut details = vec![format!(
        "已更新 {} 个文件、{} 条索引；原始内容已备份",
        t.files.len(),
        t.rows.len()
    )];
    if !failed_ids.is_empty() {
        details.push("部分项目复检未通过或覆盖不完整；请查看新报告，不标记为已修复".into())
    }
    atomic_private(
        &backup.join("result.json"),
        &serde_json::to_vec(&json!({"fixedIds":fixed_ids,"failedIds":failed_ids,"at":now_ms()}))
            .map_err(err)?,
    )?;
    Ok(DiagnosticFixResult {
        fixed_ids,
        failed_ids,
        details,
        backup_path: Some(backup.display().to_string()),
        report,
    })
}
#[cfg(test)]
mod rollback_tests {
    use super::*;
    #[test]
    fn rollback_does_not_overwrite_external_change() {
        let dir =
            std::env::temp_dir().join(format!("atelier-rollback-test-{}", uuid::Uuid::new_v4()));
        private_dir(&dir).unwrap();
        let path = dir.join("file");
        atomic_private(&path, b"external").unwrap();
        let change = FileChange {
            path: path.clone(),
            before: b"original".to_vec(),
            after: b"ours".to_vec(),
        };
        let errors = restore(&[&change]);
        assert_eq!(errors.len(), 1);
        assert_eq!(std::fs::read(path).unwrap(), b"external");
        std::fs::remove_dir_all(dir).unwrap();
    }
}
