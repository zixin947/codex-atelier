use super::*;
use crate::accounts::{
    storage::{hash, hints, id_ok, normalize},
    types::{Account, AccountMetadata, Catalog},
};
use crate::gateway::storage::{atomic_private, open_private, private_dir, read_private};
use fs2::FileExt;
use serde_json::Value;
use std::{collections::BTreeSet, fs, path::Path, time::UNIX_EPOCH};
#[derive(Clone)]
struct Inspection {
    raw: Option<Vec<u8>>,
    catalog: Option<Catalog>,
    catalog_error: Option<String>,
    profiles: Vec<String>,
    stale: Vec<String>,
    backups: Vec<PathBuf>,
    fingerprint: String,
}
fn present_dir(path: &Path) -> Result<bool> {
    match fs::symlink_metadata(path) {
        Ok(m) if m.is_dir() && !m.file_type().is_symlink() => Ok(true),
        Ok(_) => Err(format!("数据目录类型不安全：{}", path.display())),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(false),
        Err(_) => Err(format!("无法读取目录：{}", path.display())),
    }
}
fn stamp(path: &Path) -> Result<String> {
    match fs::symlink_metadata(path) {
        Ok(m) => {
            if !m.is_file() || m.file_type().is_symlink() {
                return Err("维护目标包含非普通文件，未继续".into());
            }
            let time = m
                .modified()
                .map_err(|_| "无法读取文件时间")?
                .duration_since(UNIX_EPOCH)
                .map_err(|_| "文件时间无效")?
                .as_nanos();
            Ok(format!("{}:{time}", m.len()))
        }
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok("absent".into()),
        Err(_) => Err("无法检查档案文件；未当作缺失清理".into()),
    }
}
fn inspect(root: &Path) -> Result<Inspection> {
    present_dir(root)?;
    let catalog_path = root.join("catalog.json");
    let exists = stamp(&catalog_path)? != "absent";
    let raw = if exists {
        Some(read_private(&catalog_path, 4 * 1024 * 1024)?)
    } else {
        None
    };
    let (catalog, catalog_error) = match raw.as_ref() {
        None => (Some(Catalog::default()), None),
        Some(bytes) => match serde_json::from_slice::<Catalog>(bytes) {
            Ok(c) if c.accounts.len() <= 64 => (Some(c), None),
            _ => (
                None,
                Some("账号目录损坏或超过 64 个账号；重建前将备份原文件".into()),
            ),
        },
    };
    let mut fingerprints = vec![raw
        .as_ref()
        .map(|v| hash(v))
        .unwrap_or_else(|| "absent".into())];
    let mut profiles = vec![];
    let mut stale = vec![];
    let profiles_dir = root.join("profiles");
    if present_dir(&profiles_dir)? {
        for entry in fs::read_dir(&profiles_dir).map_err(|_| "无法枚举账号档案")? {
            let entry = entry.map_err(|_| "无法读取档案条目")?;
            let name = entry.file_name().to_string_lossy().into_owned();
            if id_ok(&name).is_err() {
                continue;
            }
            if !present_dir(&entry.path())? {
                continue;
            }
            profiles.push(name);
            if profiles.len() > 128 {
                return Err("档案数量超过维护上限 128，请先检查数据目录".into());
            }
        }
    }
    profiles.sort();
    let mut ids = profiles.iter().cloned().collect::<BTreeSet<_>>();
    if let Some(c) = &catalog {
        for a in &c.accounts {
            id_ok(&a.id)?;
            ids.insert(a.id.clone());
        }
    }
    for id in ids {
        let home = profiles_dir.join(&id);
        present_dir(&home)?;
        let auth = stamp(&home.join("auth.json"))?;
        if auth == "absent"
            && catalog
                .as_ref()
                .is_some_and(|c| c.accounts.iter().any(|a| a.id == id))
        {
            stale.push(id.clone())
        }
        fingerprints.push(format!("{id}:{auth}"));
    }
    let mut backups = vec![];
    let maintenance = root.join("maintenance");
    present_dir(&maintenance)?;
    let backup_dir = maintenance.join("backups");
    if present_dir(&backup_dir)? {
        for entry in fs::read_dir(&backup_dir).map_err(|_| "无法枚举维护备份")? {
            let entry = entry.map_err(|_| "无法读取备份条目")?;
            let n = entry.file_name().to_string_lossy().into_owned();
            if n.strip_suffix(".json")
                .is_some_and(|id| uuid::Uuid::parse_str(id).is_ok())
            {
                let p = entry.path();
                fingerprints.push(format!("{n}:{}", stamp(&p)?));
                backups.push(p);
                if backups.len() > 256 {
                    return Err("维护备份超过 256 份，请分批整理后重试".into());
                }
            }
        }
    }
    fingerprints.sort();
    backups.sort();
    Ok(Inspection {
        raw,
        catalog,
        catalog_error,
        profiles,
        stale,
        backups,
        fingerprint: hash(fingerprints.join("\n").as_bytes()),
    })
}
pub(super) fn snapshot(root: &Path) -> Result<Snapshot> {
    let desktop = process::discover();
    let mut checks = vec![
        Check::new(
            "系统",
            "pass",
            format!(
                "{} / {} · Atelier {}",
                std::env::consts::OS,
                std::env::consts::ARCH,
                env!("CARGO_PKG_VERSION")
            ),
        ),
        Check::new(
            "Codex Desktop",
            if desktop.is_some() { "pass" } else { "unknown" },
            desktop
                .as_ref()
                .map(|p| p.display().to_string())
                .unwrap_or_else(|| "没有在受支持位置发现 Desktop；未尝试启动".into()),
        ),
    ];
    let mut count = 0;
    match inspect(root) {
        Ok(i) => {
            count = i.catalog.as_ref().map(|c| c.accounts.len()).unwrap_or(0);
            checks.push(Check::new(
                "账号目录",
                if i.catalog_error.is_some() {
                    "error"
                } else if i.raw.is_none() {
                    "unknown"
                } else {
                    "pass"
                },
                i.catalog_error.unwrap_or_else(|| {
                    if i.raw.is_none() {
                        "尚未创建账号目录；检查未创建任何文件".into()
                    } else {
                        format!("{count} 个账号索引，{} 个档案目录", i.profiles.len())
                    }
                }),
            ));
            checks.push(Check::new(
                "档案一致性",
                if i.stale.is_empty() {
                    "pass"
                } else {
                    "warning"
                },
                format!("{} 条索引缺少档案凭据副本；未读取凭据内容", i.stale.len()),
            ));
            checks.push(Check::new(
                "维护备份",
                "pass",
                format!("{} 份历史注册表备份；当前切号回退文件保留", i.backups.len()),
            ));
        }
        Err(e) => checks.push(Check::new("账号目录", "error", e)),
    }
    checks.push(Check::new(
        "在线状态",
        "unknown",
        "未执行凭据验证、额度请求或模型请求",
    ));
    Ok(Snapshot {
        mode: "native".into(),
        platform: std::env::consts::OS.into(),
        arch: std::env::consts::ARCH.into(),
        version: env!("CARGO_PKG_VERSION").into(),
        account_count: count,
        managed_root: root.display().to_string(),
        desktop_path: desktop.map(|p| p.display().to_string()),
        checks,
    })
}
pub(super) fn preview(root: &Path, action: &str) -> Result<Ticket> {
    if !matches!(action, "clean" | "rebuild" | "restart" | "forceKill") {
        return Err("请选择清理、重建、重启或强制结束操作".into());
    }
    let mut changes = vec![];
    let mut warnings = vec![];
    let count;
    let fingerprint;
    let desktop;
    if matches!(action, "restart" | "forceKill") {
        desktop = process::discover();
        let target = desktop
            .as_ref()
            .ok_or("未发现支持的 Codex/ChatGPT Desktop 安装；此平台或路径暂不支持进程管理")?;
        if action == "forceKill" {
            changes.push(format!(
                "强制结束 {} 内的客户端与辅助进程",
                target.display()
            ));
            changes.push("不会自动重新启动客户端；只匹配已识别应用包中的可执行路径".into());
            warnings.push(
                "正在生成的任务会中断，未保存内容可能丢失。只有客户端无法正常退出时才使用此操作。"
                    .into(),
            );
        } else {
            changes.push(format!("退出并重新启动 {}", target.display()));
            warnings.push(
                "请保存工作并结束生成；这会中断当前客户端任务。退出超时将停止，不自动强杀。".into(),
            );
        }
        count = 1;
        fingerprint = String::new();
    } else {
        desktop = None;
        let i = inspect(root)?;
        fingerprint = i.fingerprint.clone();
        if action == "clean" {
            if let Some(e) = i.catalog_error {
                return Err(e);
            }
            count = i.stale.len() + i.backups.len();
            changes.push(format!(
                "移除 {} 条缺少档案凭据副本的账号索引",
                i.stale.len()
            ));
            changes.push(format!("删除 {} 份 Atelier 历史维护备份", i.backups.len()));
            warnings.push("只清理 Atelier 账号目录；保留凭据档案、当前切号回退和线程备份。删除的历史维护备份不可从本栏目恢复。".into());
        } else {
            count = i
                .profiles
                .len()
                .max(i.catalog.as_ref().map(|c| c.accounts.len()).unwrap_or(0))
                .max(usize::from(i.catalog_error.is_some()));
            changes.push(format!(
                "重新核对 {} 个本应用账号档案，重建账号目录",
                i.profiles.len()
            ));
            changes.push("按已有身份去重，保留可匹配账号的名称、分组、备注、额度及订阅资料".into());
            if !i.stale.is_empty() {
                changes.push(format!(
                    "移除 {} 条已缺少档案凭据副本的旧索引",
                    i.stale.len()
                ));
            }
            warnings.push("确认后仅解析 Atelier 保存的登录副本；不读取本机 Codex 登录、不联网、不更改副本。备份现有目录后原子替换。".into());
            if let Some(e) = i.catalog_error {
                warnings.push(e)
            }
        }
    }
    Ok(Ticket {
        view: Plan {
            ticket: uuid::Uuid::new_v4().to_string(),
            action: action.into(),
            title: match action {
                "clean" => "清理 Atelier 数据",
                "rebuild" => "重建账号注册表",
                "forceKill" => "强制结束 Codex Desktop",
                _ => "重启 Codex Desktop",
            }
            .into(),
            changes,
            warnings,
            count,
        },
        created_at: now_ms(),
        root: root.to_path_buf(),
        fingerprint,
        desktop,
    })
}
fn lock(root: &Path, id: &str) -> Result<fs::File> {
    if id != "catalog" {
        id_ok(id)?
    }
    let f = open_private(&root.join("locks").join(format!("{id}.lock")))?;
    f.try_lock_exclusive()
        .map_err(|_| "账号正在刷新或修改，请稍后重试")?;
    Ok(f)
}
fn rebuild(root: &Path, i: &Inspection) -> Result<(Catalog, Vec<String>)> {
    let mut old = i.catalog.clone().unwrap_or_default();
    let mut accounts = vec![];
    let mut identities = BTreeSet::new();
    let mut warnings = vec![];
    let mut ids = i.profiles.clone();
    ids.sort_by_key(|id| (!old.accounts.iter().any(|a| &a.id == id), id.clone()));
    for id in ids {
        let path = root.join("profiles").join(&id).join("auth.json");
        let existing = old.accounts.iter().find(|a| a.id == id).cloned();
        if stamp(&path)? == "absent" {
            continue;
        }
        let parsed = read_private(&path, 256 * 1024)
            .and_then(|bytes| {
                serde_json::from_slice::<Value>(&bytes).map_err(|_| "档案 JSON 无法解析".into())
            })
            .and_then(|v| normalize(&v));
        match parsed {
            Ok(auth) => {
                let (identity, email, _) = hints(&auth);
                if !identities.insert(identity.clone()) {
                    warnings.push(format!("跳过重复身份档案 {}；原副本保留", &id[..8]));
                    continue;
                }
                let account = if let Some(mut a) = existing {
                    if !a.identity.is_empty() && a.identity != identity {
                        return Err("已有档案身份发生变化，未覆盖账号目录".into());
                    }
                    a.identity = identity;
                    if a.email.is_none() {
                        a.email = email
                    }
                    a
                } else {
                    Account {
                        id: id.clone(),
                        name: email
                            .clone()
                            .unwrap_or_else(|| format!("恢复的账号 {}", &id[..8])),
                        email,
                        plan: None,
                        group: String::new(),
                        note: String::new(),
                        status: "imported".into(),
                        quota: None,
                        error: None,
                        created_at: now_ms(),
                        identity,
                        metadata: AccountMetadata::default(),
                    }
                };
                accounts.push(account)
            }
            Err(_) => {
                warnings.push(format!("档案 {} 无法解析，未删除其副本", &id[..8]));
                if let Some(a) = existing {
                    if a.identity.is_empty() || identities.insert(a.identity.clone()) {
                        accounts.push(a)
                    }
                }
            }
        }
    }
    if accounts.len() > 64 {
        return Err("重建结果超过 64 个账号上限，未覆盖".into());
    }
    accounts.sort_by(|a, b| a.name.cmp(&b.name));
    old.accounts = accounts;
    if old
        .last_selected_id
        .as_ref()
        .is_some_and(|id| !old.accounts.iter().any(|a| &a.id == id))
    {
        old.last_selected_id = None
    }
    if old
        .current_observation
        .as_ref()
        .is_some_and(|c| !old.accounts.iter().any(|a| a.identity == c.identity))
    {
        old.current_observation = None
    }
    Ok((old, warnings))
}
pub(super) fn apply(t: &Ticket) -> Result<ActionResult> {
    if !matches!(t.view.action.as_str(), "clean" | "rebuild") {
        return Err("维护操作无效".into());
    }
    let root = &t.root;
    let initial = inspect(root)?;
    if initial.fingerprint != t.fingerprint {
        return Err("预览后账号或备份发生变化，请重新预览".into());
    }
    if t.view.count == 0 {
        return Ok(result(
            &t.view.action,
            "没有需要处理的项目。".into(),
            0,
            vec![],
            None,
        ));
    }
    private_dir(root)?;
    private_dir(&root.join("locks"))?;
    // Match account mutations: all profile locks first, then catalog. No network while held.
    let mut ids = initial.profiles.iter().cloned().collect::<BTreeSet<_>>();
    if let Some(c) = &initial.catalog {
        for a in &c.accounts {
            ids.insert(a.id.clone());
        }
    }
    let _profiles = ids
        .iter()
        .map(|id| lock(root, id))
        .collect::<Result<Vec<_>>>()?;
    let _catalog = lock(root, "catalog")?;
    let i = inspect(root)?;
    if i.fingerprint != t.fingerprint {
        return Err("锁定前账号发生变化，请重新预览".into());
    }
    let (mut catalog, mut warnings) = if t.view.action == "rebuild" {
        rebuild(root, &i)?
    } else {
        let mut c = i.catalog.clone().ok_or("账号目录损坏，未执行清理")?;
        c.accounts.retain(|a| !i.stale.contains(&a.id));
        if c.last_selected_id
            .as_ref()
            .is_some_and(|id| i.stale.contains(id))
        {
            c.last_selected_id = None
        }
        if c.current_observation
            .as_ref()
            .is_some_and(|o| !c.accounts.iter().any(|a| a.identity == o.identity))
        {
            c.current_observation = None
        }
        (c, vec![])
    };
    let backup_dir = root.join("maintenance").join("backups");
    private_dir(&root.join("maintenance"))?;
    private_dir(&backup_dir)?;
    let backup = if let Some(raw) = &i.raw {
        let p = backup_dir.join(format!("{}.json", t.view.ticket));
        atomic_private(&p, raw)?;
        Some(p.display().to_string())
    } else {
        None
    };
    catalog.revision = uuid::Uuid::new_v4().to_string();
    atomic_private(
        &root.join("catalog.json"),
        &serde_json::to_vec_pretty(&catalog).map_err(|_| "无法编码账号目录")?,
    )?;
    let mut changed = if t.view.action == "clean" {
        i.stale.len()
    } else {
        catalog.accounts.len()
    };
    if t.view.action == "clean" {
        for path in &i.backups {
            match fs::remove_file(path) {
                Ok(()) => changed += 1,
                Err(_) => warnings.push(format!(
                    "备份 {} 删除失败，未计入已清理数量",
                    path.file_name().unwrap_or_default().to_string_lossy()
                )),
            }
        }
    }
    let summary = if t.view.action == "clean" {
        format!(
            "清理完成：实际处理 {changed} 项，保留 {} 个账号。",
            catalog.accounts.len()
        )
    } else {
        format!("账号目录已重建：{} 个账号。", catalog.accounts.len())
    };
    Ok(result(&t.view.action, summary, changed, warnings, backup))
}
