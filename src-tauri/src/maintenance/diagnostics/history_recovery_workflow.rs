//! Backup-backed recovery planning only; repair::Ticket owns all writes.
use super::*;
use std::{collections::BTreeSet, io::Read};
pub(super) const ID: &str = "thread_history_integrity";
#[derive(Clone)]
pub(super) struct Candidate {
    pub path: PathBuf,
    pub before: Vec<u8>,
    pub after: Vec<u8>,
    pub backup: PathBuf,
    pub evidence: Vec<u8>,
    pub repaired_records: usize,
    pub restored_separators: usize,
}
fn logical(path: &Path, bytes: &[u8]) -> Result<Vec<u8>> {
    if path.extension().is_some_and(|e| e == "zst") {
        let reader = zstd::stream::read::Decoder::new(bytes).map_err(err)?;
        let mut out = Vec::new();
        reader.take(inspect::LIMIT as u64 + 1).read_to_end(&mut out).map_err(err)?;
        if out.len() as u64 > inspect::LIMIT { return Err("历史解压超过恢复预算".into()); }
        Ok(out)
    } else { Ok(bytes.to_vec()) }
}
fn regular_under(root: &Path, path: &Path) -> Result<()> {
    if std::fs::symlink_metadata(root).map_err(err)?.file_type().is_symlink() { return Err("历史根目录是符号链接".into()); }
    let relative = path.strip_prefix(root).map_err(|_| "历史路径不属于当前目录")?;
    let mut checked = root.to_path_buf();
    for part in relative.components() {
        if !matches!(part, std::path::Component::Normal(_)) { return Err("历史路径包含越界分量".into()); }
        checked.push(part);
        if std::fs::symlink_metadata(&checked).map_err(err)?.file_type().is_symlink() { return Err("历史或备份路径包含符号链接".into()); }
    }
    Ok(())
}
/// Only inspect explicitly supplied rollout paths and their same-relative-path backups.
pub(super) fn candidates(home: &Path, paths: &[PathBuf]) -> Result<Vec<Candidate>> {
    let root = home.join("backups");
    let entries = match std::fs::read_dir(&root) {
        Ok(v) => v,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(vec![]),
        Err(e) => return Err(err(e)),
    };
    let mut backups = vec![];
    for entry in entries {
        let entry = entry.map_err(err)?;
        if entry.file_name().to_string_lossy().starts_with("provider-fix-") && entry.file_type().map_err(err)?.is_dir() { backups.push(entry.path()); }
    }
    backups.sort();
    let mut result = vec![];
    for path in paths.iter().collect::<BTreeSet<_>>() {
        let Some(physical) = atelier_maintenance_core::threads::resolve_rollout(path).map_err(err)? else { continue };
        regular_under(home, &physical)?;
        let relative = physical.strip_prefix(home).map_err(err)?;
        if !relative.starts_with("sessions") && !relative.starts_with("archived_sessions") { continue; }
        let plain = if relative.extension().is_some_and(|e| e == "zst") { relative.with_extension("") } else { relative.to_path_buf() };
        // Reuse this operation's strict scan: healthy files need no recovery read.
        if let Some(Ok(observed)) = atelier_maintenance_core::threads::observation::read(&physical) {
            if observed.full_history().is_some_and(|h| h.structural_errors == 0) { continue; }
        }
        if !backups.iter().any(|dir| dir.join(&plain).exists() || dir.join(format!("{}.zst", plain.display())).exists()) { continue; }
        let before = read_bounded(&physical, inspect::LIMIT)?;
        let current = logical(&physical, &before)?;
        let mut proven: Option<Candidate> = None;
        let mut ambiguous = false;
        for directory in &backups {
            for source in [directory.join(&plain), directory.join(format!("{}.zst", plain.display()))] {
                if !source.try_exists().map_err(err)? { continue; }
                regular_under(home, &source)?;
                let evidence = read_bounded(&source, inspect::LIMIT)?;
                let backup = logical(&source, &evidence)?;
                let Ok(recovery) = super::history_recovery::recover_history(&current, &backup) else { continue };
                if recovery.repaired_records == 0 || recovery.bytes == current { continue; }
                let after = if physical.extension().is_some_and(|e| e == "zst") { zstd::stream::encode_all(recovery.bytes.as_slice(), 3).map_err(err)? } else { recovery.bytes };
                if proven.as_ref().is_some_and(|p| p.after != after) { ambiguous = true; break; }
                proven = Some(Candidate { path: physical.clone(), before: before.clone(), after, backup: source, evidence, repaired_records: recovery.repaired_records, restored_separators: recovery.restored_separators });
            }
            if ambiguous { break; }
        }
        if !ambiguous { if let Some(candidate) = proven { result.push(candidate); } }
    }
    Ok(result)
}
pub(super) fn prepare(ctx: &Context) -> Result<Vec<Candidate>> {
    let c = inspect::config(ctx)?;
    let configured = atelier_maintenance_core::state_db::configured_sqlite_home(Some(&toml::to_string(&c).map_err(err)?),ctx.sqlite_home_env.as_deref(),ctx.working_directory.as_deref());
    let dirs = configured.map(|p| vec![p]).unwrap_or_else(|| vec![ctx.home.join("sqlite"), ctx.home.clone()]);
    let mut paths = vec![];
    for dir in dirs {
        if let Some(db) = atelier_maintenance_core::state_db::highest_state_db(&dir) {
            let rows = snapshot::with_observed_path(&db, |copy, _| atelier_maintenance_core::threads::collect_diagnostic_main_threads(&[copy.to_path_buf()]))?;
            paths.extend(rows.into_iter().filter_map(|r| r.rollout_path).map(|p| { let p = PathBuf::from(p); if p.is_absolute() { p } else { ctx.home.join(p) } }));
        }
    }
    candidates(&ctx.home, &paths)
}
/// Called by the diagnostic observer only after it has determined the real status.
pub(super) fn annotate(item: &mut DiagnosticItem, home: &Path, paths: &[PathBuf]) {
    if item.id != ID || item.status != "error" { return; }
    match candidates(home, paths) {
        Ok(plans) if !plans.is_empty() => {
            item.fixable = true;
            item.detail.push_str(&format!(" 可从同路径备份确证恢复 {} 个文件；仅恢复有佐证的损坏记录，其他错误不猜修。", plans.len()));
        }
        Ok(_) => { item.fixable = false; item.detail.push_str(" 无可确证备份，不猜测恢复历史。"); }
        Err(e) => { item.fixable = false; item.detail.push_str(&format!(" 备份恢复检查未完成：{e}")); }
    }
}
pub(super) fn validate_evidence(evidence: &[(PathBuf, Vec<u8>)]) -> Result<()> {
    for (path, bytes) in evidence {
        if read_bounded(path, inspect::LIMIT)? != *bytes {
            return Err("历史恢复备份在预览后变化，请重新预览".into());
        }
    }
    Ok(())
}
#[cfg(test)]
#[path = "history_recovery_workflow_tests.rs"]
mod tests;
