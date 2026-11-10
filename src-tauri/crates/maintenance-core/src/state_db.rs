//! Native state DB relocation cleanup (0x1003789f4), with explicit path inputs.
//! This is cleanup of the old root layout, not a merge of two active databases.
use std::{
    fs, io,
    path::{Path, PathBuf},
};

#[derive(Debug, Clone)]
pub struct RelocationContext {
    pub codex_home: PathBuf,
    pub configured_sqlite_home: Option<PathBuf>,
    pub active_database: PathBuf,
    pub relay_root: PathBuf,
}

pub trait RelocationFs {
    fn read_dir(&self, path: &Path) -> io::Result<Vec<PathBuf>>;
    fn is_file(&self, path: &Path) -> bool;
    fn metadata(&self, path: &Path) -> io::Result<()>;
    fn create_dir_all(&self, path: &Path) -> io::Result<()>;
    fn copy(&self, source: &Path, target: &Path) -> io::Result<()>;
    fn remove_file(&self, path: &Path) -> io::Result<()>;
}

pub struct LocalFs;
impl RelocationFs for LocalFs {
    fn read_dir(&self, path: &Path) -> io::Result<Vec<PathBuf>> {
        Ok(fs::read_dir(path)?
            .filter_map(Result::ok)
            .map(|e| e.path())
            .collect())
    }
    fn is_file(&self, path: &Path) -> bool {
        path.is_file()
    }
    fn metadata(&self, path: &Path) -> io::Result<()> {
        fs::metadata(path).map(|_| ())
    }
    fn create_dir_all(&self, path: &Path) -> io::Result<()> {
        fs::create_dir_all(path)
    }
    fn copy(&self, source: &Path, target: &Path) -> io::Result<()> {
        fs::copy(source, target).map(|_| ())
    }
    fn remove_file(&self, path: &Path) -> io::Result<()> {
        fs::remove_file(path)
    }
}

/// Exact name filter: not numeric-version parsing, no sort or recursive walk.
pub fn list_state_dbs_in(fs: &impl RelocationFs, directory: &Path) -> Vec<PathBuf> {
    fs.read_dir(directory)
        .unwrap_or_default()
        .into_iter()
        .filter(|p| {
            fs.is_file(p)
                && p.file_name()
                    .and_then(|s| s.to_str())
                    .is_some_and(|n| n.starts_with("state_") && n.ends_with(".sqlite"))
        })
        .collect()
}

/// Native highest_state_db (0x1008edac4): parse an unsigned 32-bit version,
/// retain the first equal-version entry, and do not add an is_file predicate.
pub fn highest_state_db(directory: &Path) -> Option<PathBuf> {
    let entries = fs::read_dir(directory).ok()?;
    let mut highest: Option<(u32, PathBuf)> = None;
    for entry in entries.filter_map(Result::ok) {
        let filename = entry.file_name();
        let name = filename.to_string_lossy();
        let Some(version) = name
            .strip_prefix("state_")
            .and_then(|s| s.strip_suffix(".sqlite"))
            .and_then(|s| s.parse::<u32>().ok())
        else {
            continue;
        };
        if highest.as_ref().is_none_or(|(prior, _)| version > *prior) {
            highest = Some((version, entry.path()));
        }
    }
    highest.map(|(_, path)| path)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Assessment {
    ActiveOutsideSqlite,
    UnconfirmedConfiguration,
    NoRootDatabases,
    OldRoot { databases: Vec<PathBuf> },
}

pub fn assess(fs: &impl RelocationFs, ctx: &RelocationContext) -> Assessment {
    let sqlite = ctx.codex_home.join("sqlite");
    // Native Path::starts_with, followed by exact Path equality for configured home.
    // Merely having another directory named sqlite elsewhere does not qualify.
    if !ctx.active_database.starts_with(&sqlite) {
        return Assessment::ActiveOutsideSqlite;
    }
    if ctx.configured_sqlite_home.as_deref() != Some(sqlite.as_path()) {
        return Assessment::UnconfirmedConfiguration;
    }
    let databases = list_state_dbs_in(fs, &ctx.codex_home);
    if databases.is_empty() {
        Assessment::NoRootDatabases
    } else {
        Assessment::OldRoot { databases }
    }
}

pub fn related_files(database: &Path) -> [PathBuf; 3] {
    let append = |suffix: &str| {
        let mut p = database.as_os_str().to_os_string();
        p.push(suffix);
        PathBuf::from(p)
    };
    [database.to_path_buf(), append("-wal"), append("-shm")]
}

/// File inventory for Atelier's reviewed transaction. Reassess and compare the
/// inventory again under the mutation/quiet-window gate before applying it.
pub fn planned_old_files(fs: &impl RelocationFs, ctx: &RelocationContext) -> Vec<PathBuf> {
    match assess(fs, ctx) {
        Assessment::OldRoot { databases } => databases
            .iter()
            .flat_map(|p| related_files(p))
            .filter(|p| fs.metadata(p).is_ok())
            .collect(),
        _ => vec![],
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RelocationResult {
    pub fixed: bool,
    pub failed: bool,
    pub details: Vec<String>,
    pub removed_databases: usize,
    pub backups: Vec<PathBuf>,
    pub remaining: Vec<PathBuf>,
}
impl RelocationResult {
    fn settled(fixed: bool, detail: impl Into<String>) -> Self {
        Self {
            fixed,
            failed: !fixed,
            details: vec![detail.into()],
            removed_databases: 0,
            backups: vec![],
            remaining: vec![],
        }
    }
}

/// Actual native two-pass copy/remove + residual verification. The caller must
/// stop writers first. Copy or delete failures are retained as remaining paths;
/// a partial cleanup is never reported as success. Time is an explicit input.
pub fn fix_state_db_relocation(
    fs: &impl RelocationFs,
    ctx: &RelocationContext,
    unix_seconds: u64,
) -> RelocationResult {
    let databases =
        match assess(fs, ctx) {
            Assessment::ActiveOutsideSqlite => {
                return RelocationResult::settled(true, "活库不在 sqlite/ 子目录，无需清理。")
            }
            Assessment::UnconfirmedConfiguration => return RelocationResult::settled(
                false,
                "Codex 未明确配置 sqlite_home，无法证明根目录数据库是孤儿库；已保留全部数据库。",
            ),
            Assessment::NoRootDatabases => return RelocationResult::settled(true, "无旧库残留"),
            Assessment::OldRoot { databases } => databases,
        };
    let backup_root = ctx.relay_root.join("state-db-backups");
    if let Err(e) = fs.create_dir_all(&backup_root) {
        return RelocationResult::settled(false, format!("无法创建备份目录：{e}"));
    }
    let mut result = RelocationResult::settled(false, "");
    result.details.clear();
    for db in &databases {
        for (index, file) in related_files(db).into_iter().enumerate() {
            if fs.metadata(&file).is_err() {
                continue;
            }
            let name = file
                .file_name()
                .and_then(|v| v.to_str())
                .unwrap_or("state.sqlite");
            let target = backup_root.join(format!("{unix_seconds}.{name}"));
            if fs.copy(&file, &target).is_ok() {
                result.backups.push(target);
                if fs.remove_file(&file).is_ok() && index == 0 {
                    result.removed_databases += 1;
                }
            }
        }
    }
    result.remaining = databases
        .iter()
        .flat_map(|p| related_files(p))
        .filter(|p| fs.metadata(p).is_ok())
        .collect();
    result.fixed = result.remaining.is_empty();
    result.failed = !result.fixed;
    if result.fixed {
        result.details.push(format!(
            "已备份并移除 {} 个旧版线程数据库（备份在 relay/state-db-backups/）",
            result.removed_databases
        ));
    } else {
        result.details.push(format!("仍有 {} 个旧库相关文件被占用，无法删除：{}。请完全退出 Codex（含 computer-use / node_repl 等插件子进程）后重试，或手动删除该文件。",
            result.remaining.len(), result.remaining.iter().map(|p| p.display().to_string()).collect::<Vec<_>>().join("、")));
    }
    result
}

/// Native configured_sqlite_home reads valid TOML first, then falls back to the
/// nonempty OS environment value; relative paths are based on cwd, not Codex home.
/// No tilde expansion or filesystem canonicalization is performed.
pub fn configured_sqlite_home(
    config: Option<&str>,
    environment: Option<&std::ffi::OsStr>,
    cwd: Option<&Path>,
) -> Option<PathBuf> {
    let from_config = config
        .and_then(|s| toml::from_str::<toml::Value>(s).ok())
        .and_then(|v| {
            v.get("sqlite_home")
                .and_then(|v| v.as_str())
                .map(str::trim)
                .filter(|s| !s.is_empty())
                .map(PathBuf::from)
        });
    let path = from_config.or_else(|| environment.filter(|s| !s.is_empty()).map(PathBuf::from))?;
    Some(if path.is_relative() {
        cwd.map(|cwd| cwd.join(&path)).unwrap_or(path)
    } else {
        path
    })
}
