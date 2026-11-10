//! Native old-layout cleanup, staged through Atelier's reviewed backup transaction.
use super::*;
use atelier_maintenance_core::state_db::{self as native, Assessment, LocalFs, RelocationContext};
use native::RelocationFs;
use sha2::{Digest, Sha256};
use std::{collections::BTreeMap, io::Read};

fn context(ctx: &Context, config: &toml::Value) -> Result<RelocationContext> {
    let text = toml::to_string(config).map_err(err)?;
    let configured = native::configured_sqlite_home(
        Some(&text),
        ctx.sqlite_home_env.as_deref(),
        ctx.working_directory.as_deref(),
    );
    let fallback = ctx.home.join("sqlite");
    let directory = configured.as_deref().unwrap_or(&fallback);
    let active = native::highest_state_db(directory)
        .or_else(|| {
            configured
                .is_none()
                .then(|| native::highest_state_db(&ctx.home))
                .flatten()
        })
        .unwrap_or_else(|| directory.join("state_5.sqlite"));
    Ok(RelocationContext {
        codex_home: ctx.home.clone(),
        configured_sqlite_home: configured,
        active_database: active,
        relay_root: ctx.app_root.join("maintenance"),
    })
}

pub(super) fn inspect(ctx: &Context, config: &toml::Value) -> Result<DiagnosticItem> {
    let assessment = native::assess(&LocalFs, &context(ctx, config)?);
    let (status, detail, fixable) = match assessment {
        Assessment::ActiveOutsideSqlite => ("ok", "活库不在 sqlite/ 子目录，无需清理。".into(), false),
        Assessment::NoRootDatabases => ("ok", "无旧库残留".into(), false),
        Assessment::UnconfirmedConfiguration => {
            let roots = native::list_state_dbs_in(&LocalFs, &ctx.home);
            if roots.is_empty() {
                ("ok", "没有根目录旧库，无需清理。".into(), false)
            } else {
                ("warning", "Codex 未明确配置 sqlite_home 为本目录的 sqlite/ 子目录，无法证明根目录库是孤儿库；保留全部数据库。".into(), false)
            }
        }
        Assessment::OldRoot { databases } => ("warning", format!(
            "已明确配置 sqlite_home，发现 {} 个根目录旧库；修复会备份并清理旧库及其 WAL/SHM，不合并或改写活库。", databases.len()), true),
    };
    Ok(DiagnosticItem {
        id: "state_db_relocation".into(),
        group: "runtime".into(),
        label: "线程数据库位置正常".into(),
        status: status.into(),
        detail,
        fixable,
    })
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub(super) struct Deletion {
    pub path: PathBuf,
    pub sha256: String,
    pub length: u64,
}

/// Database cleanup streams content; it must not inherit rollout text limits.
fn fingerprint(path: &Path) -> Result<Deletion> {
    let metadata = std::fs::metadata(path).map_err(err)?;
    if !metadata.is_file() {
        return Err("旧库相关文件不是普通文件，未删除".into());
    }
    let mut input = std::fs::File::open(path).map_err(err)?;
    let mut hash = Sha256::new();
    let mut buffer = [0u8; 64 * 1024];
    let mut length = 0;
    loop {
        let n = input.read(&mut buffer).map_err(err)?;
        if n == 0 {
            break;
        }
        hash.update(&buffer[..n]);
        length += n as u64;
    }
    let after = std::fs::metadata(path).map_err(err)?;
    if length != metadata.len()
        || length != after.len()
        || metadata.modified().ok() != after.modified().ok()
    {
        return Err("旧库相关文件在读取期间变化，请重新预览".into());
    }
    Ok(Deletion {
        path: path.to_owned(),
        sha256: format!("{:x}", hash.finalize()),
        length,
    })
}

pub(super) fn plan(ctx: &Context, config: &toml::Value) -> Result<Vec<Deletion>> {
    let context = context(ctx, config)?;
    if !matches!(
        native::assess(&LocalFs, &context),
        Assessment::OldRoot { .. }
    ) {
        return Err("数据库布局已变化，不能清理根目录旧库，请重新检测".into());
    }
    native::planned_old_files(&LocalFs, &context)
        .into_iter()
        .map(|path| fingerprint(&path))
        .collect()
}

pub(super) fn validate(ctx: &Context, config: &toml::Value, expected: &[Deletion]) -> Result<()> {
    let current = plan(ctx, config)?;
    let paths = |files: &[Deletion]| {
        files
            .iter()
            .map(|p| (p.path.clone(), p.clone()))
            .collect::<BTreeMap<_, _>>()
    };
    if paths(&current) != paths(expected) {
        return Err("旧库及 WAL/SHM 清理范围已被外部更新，请重新预览".into());
    }
    Ok(())
}

struct GuardedFs<'a>(&'a [Deletion]);
impl GuardedFs<'_> {
    fn verify(&self, path: &Path) -> std::io::Result<&Deletion> {
        let expected = self
            .0
            .iter()
            .find(|p| p.path == path)
            .ok_or_else(|| std::io::Error::other("unplanned database file"))?;
        if fingerprint(path).map_err(std::io::Error::other)? != *expected {
            return Err(std::io::Error::other("database file changed after preview"));
        }
        Ok(expected)
    }
}
impl RelocationFs for GuardedFs<'_> {
    fn read_dir(&self, p: &Path) -> std::io::Result<Vec<PathBuf>> {
        LocalFs.read_dir(p)
    }
    fn is_file(&self, p: &Path) -> bool {
        LocalFs.is_file(p)
    }
    fn metadata(&self, p: &Path) -> std::io::Result<()> {
        LocalFs.metadata(p)
    }
    fn create_dir_all(&self, p: &Path) -> std::io::Result<()> {
        crate::gateway::storage::private_dir(p).map_err(std::io::Error::other)
    }
    fn copy(&self, source: &Path, destination: &Path) -> std::io::Result<()> {
        let expected = self.verify(source)?;
        std::fs::copy(source, destination)?;
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            std::fs::set_permissions(destination, std::fs::Permissions::from_mode(0o600))?;
        }
        std::fs::File::open(destination)?.sync_all()?;
        let copied = fingerprint(destination).map_err(std::io::Error::other)?;
        if copied.length != expected.length || copied.sha256 != expected.sha256 {
            return Err(std::io::Error::other("database backup verification failed"));
        }
        Ok(())
    }
    fn remove_file(&self, path: &Path) -> std::io::Result<()> {
        self.verify(path)?;
        std::fs::remove_file(path)
    }
}

pub(super) fn apply(
    ctx: &Context,
    config: &toml::Value,
    expected: &[Deletion],
    backup: &Path,
    seconds: u64,
) -> Result<native::RelocationResult> {
    validate(ctx, config, expected)?;
    let mut paths = context(ctx, config)?;
    paths.relay_root = backup.to_owned();
    Ok(native::fix_state_db_relocation(
        &GuardedFs(expected),
        &paths,
        seconds,
    ))
}
