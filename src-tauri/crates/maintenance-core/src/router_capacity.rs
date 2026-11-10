//! Native transition capacity preflight @1008d54e8. Explicit paths; does not publish repair outputs.
use crate::{router_verification::VerificationPaths, threads};
use rusqlite::{Connection, OpenFlags};
use std::{
    collections::BTreeSet,
    fs, io,
    path::{Path, PathBuf},
    time::Duration,
};

pub const RESERVE_BYTES: u64 = 32 * 1024 * 1024;
const MIB: u64 = 1024 * 1024;
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct CapacityReport {
    pub available_bytes: Option<u64>,
    pub required_bytes: u64,
    pub write_bytes: u64,
    pub logical_lengths: Vec<u64>,
    pub restore_files: usize,
    pub backup_databases: usize,
    pub warnings: Vec<String>,
}
pub trait FreeSpace {
    fn available_bytes(&self, path: &Path) -> io::Result<u64>;
}
pub struct LocalFreeSpace;
impl FreeSpace for LocalFreeSpace {
    fn available_bytes(&self, path: &Path) -> io::Result<u64> {
        fs2::available_space(path)
    }
}
/// Actual plain file metadata or streamed decoded zstd length; never a frame-size guess.
pub fn rollout_logical_len(path: &Path) -> io::Result<u64> {
    let resolved = threads::resolve_rollout(path)?
        .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "rollout is missing"))?;
    if resolved.to_string_lossy().ends_with(".jsonl.zst") {
        let mut decoder = zstd::stream::read::Decoder::new(fs::File::open(resolved)?)?;
        io::copy(&mut decoder, &mut io::sink())
    } else {
        fs::metadata(resolved).map(|m| m.len())
    }
}
pub fn required_bytes(write_bytes: u64, logical_lengths: &[u64]) -> u64 {
    let mut largest = logical_lengths.to_vec();
    largest.sort_unstable_by(|a, b| b.cmp(a));
    largest.into_iter().take(4).fold(
        write_bytes.saturating_add(RESERVE_BYTES),
        u64::saturating_add,
    )
}
fn row_error(message: String) -> io::Error {
    io::Error::other(message)
}
fn missing(id: &str) -> io::Error {
    row_error(format!(
        "thread {id}: rollout is missing; recover missing histories before the strict transition"
    ))
}
fn path_identity(path: &Path) -> PathBuf {
    let resolved = threads::resolve_rollout(path)
        .ok()
        .flatten()
        .unwrap_or_else(|| path.to_owned());
    fs::canonicalize(&resolved).unwrap_or(resolved)
}
fn warn(report: &mut CapacityReport, label: &str, path: &Path, error: impl std::fmt::Display) {
    report
        .warnings
        .push(format!("{label} {}: {error}", path.display()));
}
fn gather(paths: &VerificationPaths, include_missing: bool) -> io::Result<CapacityReport> {
    let mut report = CapacityReport::default();
    for path in threads::repair::collect_active_instruction_rollout_paths(&paths.databases) {
        match rollout_logical_len(&path) {
            Ok(length) => report.logical_lengths.push(length),
            Err(e) if e.kind() == io::ErrorKind::NotFound => {}
            Err(e) => warn(&mut report, "cannot measure active rollout", &path, e),
        }
    }
    let trusted = paths.codex_home.join("codexmate");
    let mut seen = BTreeSet::new();
    for database in &paths.databases {
        if fs::metadata(database).is_err() {
            continue;
        }
        let connection = match Connection::open_with_flags(
            database,
            OpenFlags::SQLITE_OPEN_READ_ONLY | OpenFlags::SQLITE_OPEN_NO_MUTEX,
        ) {
            Ok(c) => c,
            Err(e) if include_missing => {
                warn(&mut report, "cannot open database", database, e);
                continue;
            }
            Err(e) => {
                return Err(row_error(format!(
                    "open thread database {}: {e}",
                    database.display()
                )))
            }
        };
        let _ = connection.busy_timeout(Duration::from_secs(3));
        let targets = match threads::collect_provider_convergence_targets(&connection) {
            Ok(targets) => targets,
            Err(e) if include_missing => {
                warn(
                    &mut report,
                    "cannot collect convergence targets",
                    database,
                    e,
                );
                continue;
            }
            Err(e) => return Err(row_error(e)),
        };
        let mut needs_database_backup = false;
        for target in targets {
            let Some(raw_path) = target.rollout_path else {
                if !include_missing {
                    return Err(missing(&target.id));
                }
                needs_database_backup = true;
                continue;
            };
            let path = PathBuf::from(raw_path);
            match threads::resolve_rollout(&path) {
                Ok(Some(_)) => continue,
                Err(e) => {
                    return Err(row_error(format!(
                        "thread {}: cannot resolve rollout: {e}",
                        target.id
                    )))
                }
                Ok(None) if !include_missing => return Err(missing(&target.id)),
                Ok(None) => {}
            }
            let Some(backup) = threads::find_trusted_rollout_backup(&trusted, &target.id, &path)
            else {
                needs_database_backup = true;
                continue;
            };
            if !seen.insert(path_identity(&path)) {
                continue;
            }
            match fs::metadata(&backup) {
                Ok(m) if m.is_file() => {
                    report.write_bytes = report.write_bytes.saturating_add(m.len());
                    // This is the copy's physical length, including compressed backups.
                    report.logical_lengths.push(m.len());
                    report.restore_files += 1;
                }
                Ok(_) => {}
                Err(e) => warn(&mut report, "cannot measure trusted backup", &backup, e),
            }
        }
        if needs_database_backup {
            match fs::metadata(database) {
                Ok(m) if m.is_file() => {
                    report.write_bytes = report.write_bytes.saturating_add(m.len());
                    report.backup_databases += 1;
                }
                Ok(_) => {}
                Err(e) => warn(&mut report, "cannot measure database backup", database, e),
            }
        }
    }
    report.required_bytes = required_bytes(report.write_bytes, &report.logical_lengths);
    Ok(report)
}
/// Probe failure is natively best effort; SQL/rollout errors in strict mode are not.
pub fn inspect_transition_rollout_write_capacity_with_space(
    paths: &VerificationPaths,
    include_missing: bool,
    space: &impl FreeSpace,
) -> io::Result<CapacityReport> {
    let mut report = gather(paths, include_missing)?;
    match space.available_bytes(&paths.codex_home) {
        Ok(available) => {
            report.available_bytes = Some(available);
            if available < report.required_bytes {
                let ceil_mib = |bytes: u64| bytes.saturating_add(MIB - 1) / MIB;
                return Err(io::Error::other(format!(
                    "磁盘剩余空间不足：可用 {} MB，路由转换至少需要 {} MB；请清理空间后重试。",
                    ceil_mib(available),
                    ceil_mib(report.required_bytes)
                )));
            }
        }
        Err(e) => warn(
            &mut report,
            "free-space probe unavailable",
            &paths.codex_home,
            e,
        ),
    }
    Ok(report)
}
pub fn inspect_transition_rollout_write_capacity(
    paths: &VerificationPaths,
    include_missing: bool,
) -> io::Result<CapacityReport> {
    inspect_transition_rollout_write_capacity_with_space(paths, include_missing, &LocalFreeSpace)
}
pub fn ensure_transition_rollout_write_capacity(
    paths: &VerificationPaths,
    include_missing: bool,
) -> io::Result<()> {
    let report = inspect_transition_rollout_write_capacity(paths, include_missing)?;
    for warning in report.warnings {
        eprintln!("router capacity preflight: {warning}");
    }
    Ok(())
}
