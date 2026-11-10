use super::*;
use crate::accounts::{
    storage::{hash, hints, id_ok, normalize},
    types::{Account, AccountMetadata, Catalog},
};
use crate::gateway::storage::{atomic_private, open_private, private_dir, read_private};
use fs2::FileExt;
use serde_json::Value;
use sha2::{Digest, Sha256};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    io::Read,
    path::Path,
    time::UNIX_EPOCH,
};
#[derive(Clone)]
struct Inspection {
    raw: Option<Vec<u8>>,
    catalog: Option<Catalog>,
    catalog_error: Option<String>,
    profiles: Vec<String>,
    stale: Vec<String>,
    backups: Vec<PathBuf>,
    backup_stamps: BTreeMap<PathBuf, String>,
    backup_directory_identity: Option<String>,
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
fn object_identity(m: &fs::Metadata) -> Result<String> {
    #[cfg(unix)]
    {
        use std::os::unix::fs::MetadataExt;
        Ok(format!("{}:{}", m.dev(), m.ino()))
    }
    #[cfg(not(unix))]
    {
        let _ = m;
        Err("当前平台无法确认清理对象身份，未执行".into())
    }
}
fn backup_stamp(path: &Path) -> Result<String> {
    let before = fs::symlink_metadata(path).map_err(|_| "备份对象已变化，未继续")?;
    let identity = object_identity(&before)?;
    let kind = if before.file_type().is_symlink() {
        "symlink"
    } else if before.is_dir() {
        "directory"
    } else if before.is_file() {
        "file"
    } else {
        "special"
    };
    let content = if before.is_file() {
        // Open without following a last-component symlink, then bind the bytes to that inode.
        let mut options = fs::OpenOptions::new();
        options.read(true);
        #[cfg(unix)]
        {
            use std::os::unix::fs::OpenOptionsExt;
            options.custom_flags(libc::O_NOFOLLOW | libc::O_NONBLOCK);
        }
        let mut file = options.open(path).map_err(|_| "无法读取备份内容，未继续")?;
        if object_identity(&file.metadata().map_err(|_| "无法核对备份对象")?)? != identity {
            return Err("备份对象在读取期间变化，未继续".into());
        }
        let mut hasher = Sha256::new();
        let mut buffer = [0u8; 64 * 1024];
        loop {
            let n = file
                .read(&mut buffer)
                .map_err(|_| "无法读取备份内容，未继续")?;
            if n == 0 {
                break;
            }
            hasher.update(&buffer[..n]);
        }
        format!("{:x}", hasher.finalize())
    } else if before.file_type().is_symlink() {
        let target = fs::read_link(path).map_err(|_| "无法核对备份链接，未继续")?;
        // Bind the link itself; never read its target.
        format!("{:?}", target.as_os_str())
    } else {
        String::new()
    };
    let after = fs::symlink_metadata(path).map_err(|_| "备份对象已变化，未继续")?;
    if object_identity(&after)? != identity
        || before.len() != after.len()
        || before.modified().ok() != after.modified().ok()
    {
        return Err("备份对象在读取期间变化，未继续".into());
    }
    Ok(format!(
        "{identity}:{kind}:{}:{:?}:{content}",
        before.len(),
        before.modified().ok()
    ))
}
fn backup_directory_identity(path: &Path) -> Result<Option<String>> {
    if !present_dir(path)? {
        return Ok(None);
    }
    Ok(Some(object_identity(
        &fs::symlink_metadata(path).map_err(|_| "无法核对备份目录")?,
    )?))
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
fn inspect(root: &Path, check_oauth_snapshots: bool) -> Result<Inspection> {
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
        let indexed = catalog
            .as_ref()
            .is_some_and(|c| c.accounts.iter().any(|a| a.id == id));
        let eligible = if check_oauth_snapshots && auth != "absent" {
            match read_private(&home.join("auth.json"), 256 * 1024) {
                Ok(bytes) => {
                    // Native snapshot eligibility is local structure, not online authentication.
                    // The source bytes never leave the plan/fingerprint boundary.
                    fingerprints.push(format!("{id}:auth-content:{}", hash(&bytes)));
                    serde_json::from_slice::<atelier_maintenance_core::auth::AuthFile>(&bytes)
                        .is_ok_and(|a| atelier_maintenance_core::auth::is_switchable_oauth_auth(&a))
                }
                Err(_) => false,
            }
        } else {
            auth != "absent"
        };
        if indexed && !eligible {
            stale.push(id.clone());
        }
        fingerprints.push(format!("{id}:{auth}"));
    }
    let mut backups = vec![];
    let maintenance = root.join("maintenance");
    present_dir(&maintenance)?;
    let backup_dir = maintenance.join("backups");
    let backup_directory_identity = backup_directory_identity(&backup_dir)?;
    let mut backup_stamps = BTreeMap::new();
    fingerprints.push(format!("backup-directory:{backup_directory_identity:?}"));
    if backup_directory_identity.is_some() {
        for entry in fs::read_dir(&backup_dir).map_err(|_| "无法枚举维护备份")? {
            let entry = entry.map_err(|_| "无法读取备份条目")?;
            let p = entry.path();
            let stamp = backup_stamp(&p)?;
            fingerprints.push(format!("backup:{:?}:{stamp}", entry.file_name()));
            backup_stamps.insert(p.clone(), stamp);
            // Only approved direct entries; directories remain failures, never recurse.
            backups.push(p);
        }
        if self::backup_directory_identity(&backup_dir)? != backup_directory_identity {
            return Err("备份目录在检查期间变化，未继续".into());
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
        backup_stamps,
        backup_directory_identity,
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
    match inspect(root, false) {
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
    let native_process;
    if matches!(action, "restart" | "forceKill") {
        let plan = process::prepare_native(action)?;
        changes.extend(plan.preview_lines());
        warnings.extend(plan.warnings());
        count = plan.preview().targets.len() + usize::from(action == "restart");
        fingerprint = String::new();
        native_process = Some(plan);
    } else {
        native_process = None;
        let i = inspect(root, action == "clean")?;
        fingerprint = i.fingerprint.clone();
        if action == "clean" {
            if let Some(e) = i.catalog_error {
                return Err(e);
            }
            count = i.stale.len() + i.backups.len();
            changes.push(format!(
                "移除 {} 条缺少可切换 OAuth 登录快照的账号索引",
                i.stale.len()
            ));
            changes.push(format!(
                "删除 {} 个 Atelier 历史维护备份直接条目（失败项保留并报告）",
                i.backups.len()
            ));
            warnings.push("仅按本地 OAuth 结构检查清理 Atelier 账号索引与历史备份，不联网验证；保留凭据档案、当前切号回退和线程备份。删除的历史维护备份不可从本栏目恢复。".into());
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
            process: native_process.as_ref().map(|p| p.preview().clone()),
            action: action.into(),
            title: match action {
                "clean" => "清理 Atelier 数据",
                "rebuild" => "重建账号注册表",
                "forceKill" => "强制结束全部 Codex 相关进程",
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
        native_process,
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
    apply_with_delete_hook(t, |_| {})
}
// Hook permits deterministic replacement-race tests without touching real user data.
pub(super) fn apply_with_delete_hook(
    t: &Ticket,
    mut before_delete: impl FnMut(usize),
) -> Result<ActionResult> {
    if !matches!(t.view.action.as_str(), "clean" | "rebuild") {
        return Err("维护操作无效".into());
    }
    let root = &t.root;
    let initial = inspect(root, t.view.action == "clean")?;
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
    let i = inspect(root, t.view.action == "clean")?;
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
        let approved_directory = i
            .backup_directory_identity
            .clone()
            .or(backup_directory_identity(&backup_dir)?);
        for (index, path) in i.backups.iter().enumerate() {
            before_delete(index);
            // Refuse replacement objects, including byte-identical inode swaps, immediately
            // before each unlink. Never enumerate a fresh, expanded deletion set after writes.
            let unchanged = backup_directory_identity(&backup_dir)
                .is_ok_and(|actual| actual == approved_directory)
                && backup_stamp(path)
                    .is_ok_and(|actual| i.backup_stamps.get(path) == Some(&actual));
            if !unchanged {
                warnings.push(
                    "执行中备份对象发生变化，已停止后续删除；已保存注册表备份，未删除替换对象"
                        .into(),
                );
                break;
            }
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
