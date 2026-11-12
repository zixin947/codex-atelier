//! Independent, read-only history scanner recovered from 0x1008bd284.
//! Limits count decompressed bytes, including blank/invalid/oversized records.
//! No home discovery, repair, provider/model validity inference or session text output.
use super::{resolve_rollout, MainThread};
use std::{
    collections::BTreeSet,
    fs,
    io::{self, BufRead, BufReader},
    path::{Path, PathBuf},
};

#[derive(Debug, Clone, Copy)]
pub struct HistoryLimits {
    pub total_bytes: u64,
    pub min_file_bytes: u64,
    pub max_file_bytes: u64,
    /// Native read_until is wrapped in Take(0x1800001).
    pub line_bytes: usize,
    pub drain_bytes: u64,
    pub thread_offset: usize,
    pub thread_limit: usize,
}
impl Default for HistoryLimits {
    fn default() -> Self {
        Self {
            total_bytes: 512 * 1024 * 1024,
            min_file_bytes: 2 * 1024 * 1024,
            max_file_bytes: 64 * 1024 * 1024,
            line_bytes: 0x1800001,
            drain_bytes: 64 * 1024 * 1024,
            thread_offset: 0,
            thread_limit: usize::MAX,
        }
    }
}
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct HistoryInspection {
    pub total: usize,
    pub selected: usize,
    pub scanned: usize,
    pub complete: usize,
    pub partial: usize,
    pub missing_rollouts: usize,
    pub missing_paths: usize,
    pub structural_errors: usize,
    pub missing_base_refs: usize,
    pub oversized_lines: usize,
    pub bytes_read: u64,
    pub errors: Vec<String>,
    pub limits: Vec<(String, HistoryLimit)>,
    pub examples: Vec<String>,
}
impl HistoryInspection {
    pub fn coverage_complete(&self) -> bool {
        self.selected == self.total
            && self.complete == self.total
            && self.errors.is_empty()
            && self.missing_paths == 0
            && self.missing_rollouts == 0
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HistoryLimit {
    ByteBudget,
    SummaryBudget,
    RecordLimit,
}
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct FileHistoryInspection {
    pub complete: bool,
    pub structural_errors: usize,
    pub missing_base_refs: usize,
    pub oversized_lines: usize,
    pub bytes_read: u64,
    pub error: Option<String>,
    pub limit: Option<HistoryLimit>,
}
/// 0x1008b3598: recurse at depths 0..6; UUID is the final 36 bytes before
/// .jsonl/.jsonl.zst of a rollout-* name. Preserve literal case, do not read content.
/// Native directory/entry errors are ignored: this is a filename index, not identity proof.
pub fn collect_rollout_ids(roots: &[PathBuf]) -> BTreeSet<String> {
    fn visit(path: &Path, depth: usize, ids: &mut BTreeSet<String>) {
        if depth == 7 {
            return;
        }
        let Ok(entries) = fs::read_dir(path) else {
            return;
        };
        for entry in entries.flatten() {
            let Ok(kind) = entry.file_type() else {
                continue;
            };
            if kind.is_dir() {
                visit(&entry.path(), depth + 1, ids);
                continue;
            }
            let name = entry.file_name();
            let Some(name) = name.to_str() else {
                continue;
            };
            let Some(stem) = name.strip_prefix("rollout-").and_then(|s| {
                s.strip_suffix(".jsonl.zst")
                    .or_else(|| s.strip_suffix(".jsonl"))
            }) else {
                continue;
            };
            let Some(start) = stem.len().checked_sub(36) else {
                continue;
            };
            let Some(id) = stem.get(start..) else {
                continue;
            };
            let bytes = id.as_bytes();
            if bytes.iter().enumerate().all(|(i, c)| {
                if [8, 13, 18, 23].contains(&i) {
                    *c == b'-'
                } else {
                    c.is_ascii_hexdigit()
                }
            }) {
                ids.insert(id.to_owned());
            }
        }
    }
    let mut ids = BTreeSet::new();
    for root in roots {
        visit(root, 0, &mut ids);
    }
    ids
}
fn reader(path: &Path) -> io::Result<Box<dyn BufRead>> {
    let f = fs::File::open(path)?;
    if path.to_string_lossy().ends_with(".jsonl.zst") {
        Ok(Box::new(BufReader::new(zstd::stream::read::Decoder::new(
            f,
        )?)))
    } else {
        Ok(Box::new(BufReader::new(f)))
    }
}
/// Bounded read_until: never allocates more than limit for a physical line.
pub(super) fn chunk(
    reader: &mut dyn BufRead,
    limit: usize,
    output: &mut Vec<u8>,
) -> io::Result<usize> {
    output.clear();
    while output.len() < limit {
        let buf = reader.fill_buf()?;
        if buf.is_empty() {
            break;
        }
        let available = buf.len().min(limit - output.len());
        let n = buf[..available]
            .iter()
            .position(|c| *c == b'\n')
            .map_or(available, |n| n + 1);
        let done = buf[n - 1] == b'\n';
        output.extend_from_slice(&buf[..n]);
        reader.consume(n);
        if done {
            break;
        }
    }
    Ok(output.len())
}
fn record(
    bytes: &[u8],
    ids: &BTreeSet<String>,
    previous: &mut Option<u64>,
    first_meta: &mut bool,
    out: &mut FileHistoryInspection,
) {
    let bytes = bytes.strip_suffix(b"\n").unwrap_or(bytes);
    let bytes = bytes.strip_suffix(b"\r").unwrap_or(bytes);
    if bytes.is_empty() {
        return;
    }
    let v = std::str::from_utf8(bytes)
        .ok()
        .and_then(|s| serde_json::from_str::<serde_json::Value>(s).ok());
    record_value(v.as_ref(), ids, previous, first_meta, out);
}
pub(super) fn record_value(
    value: Option<&serde_json::Value>,
    ids: &BTreeSet<String>,
    previous: &mut Option<u64>,
    first_meta: &mut bool,
    out: &mut FileHistoryInspection,
) {
    let Some(v) = value else {
        out.structural_errors += 1;
        return;
    };
    if let Some(n) = v.get("ordinal").and_then(|v| v.as_u64()) {
        if previous.is_some_and(|p| n <= p) {
            out.structural_errors += 1;
        }
        *previous = Some(n);
    }
    if !*first_meta && v.get("type").and_then(|v| v.as_str()) == Some("session_meta") {
        *first_meta = true;
        if let Some(base) = v.pointer("/payload/history_base").filter(|b| !b.is_null()) {
            // Verified offline VA 0x1013b5973, length 21: end_ordinal_exclusive.
            let end = base.get("end_ordinal_exclusive").and_then(|v| v.as_u64());
            if end.is_none() || end != v.get("ordinal").and_then(|v| v.as_u64()) {
                out.structural_errors += 1;
            }
            if base
                .get("thread_id")
                .and_then(|v| v.as_str())
                .is_some_and(|id| !id.is_empty() && !ids.contains(id))
            {
                out.missing_base_refs += 1;
            }
        }
    }
}
pub fn scan_history_file(
    path: &Path,
    ids: &BTreeSet<String>,
    budget: u64,
    limits: HistoryLimits,
) -> FileHistoryInspection {
    if let Some(observed) = super::observation::read(path) {
        return match observed {
            Ok(observed) => observed.history(ids, budget, limits),
            Err(error) => FileHistoryInspection {
                error: Some(error),
                ..Default::default()
            },
        };
    }
    let mut out = FileHistoryInspection::default();
    if budget == 0 || limits.line_bytes == 0 {
        return out;
    }
    let mut reader = match reader(path) {
        Ok(r) => r,
        Err(e) => {
            out.error = Some(e.to_string());
            return out;
        }
    };
    let mut line = Vec::new();
    let mut previous = None;
    let mut first_meta = false;
    let mut impaired = false;
    loop {
        let left = budget.saturating_sub(out.bytes_read);
        if left == 0 {
            break;
        }
        let cap = limits.line_bytes.min(left.min(usize::MAX as u64) as usize);
        let n = match chunk(&mut *reader, cap, &mut line) {
            Ok(n) => n,
            Err(e) => {
                // A decoder may fail after earlier fill_buf chunks were consumed.
                // Charge those bytes as well; repeated failed streams cannot bypass the budget.
                out.bytes_read += line.len() as u64;
                out.error = Some(e.to_string());
                break;
            }
        };
        out.bytes_read += n as u64;
        if n == 0 {
            out.complete = !impaired;
            break;
        }
        if line.last() == Some(&b'\n') {
            record(&line, ids, &mut previous, &mut first_meta, &mut out);
            continue;
        }
        // A non-newline tail shorter than native Take limit is partial, not bad JSON.
        if n < limits.line_bytes {
            break;
        }
        out.oversized_lines += 1;
        impaired = true;
        let mut drained = 0;
        loop {
            let remaining = budget
                .saturating_sub(out.bytes_read)
                .min(limits.drain_bytes.saturating_sub(drained));
            if remaining == 0 {
                return out;
            }
            let buf = match reader.fill_buf() {
                Ok(buf) => buf,
                Err(e) => {
                    out.error = Some(e.to_string());
                    return out;
                }
            };
            if buf.is_empty() {
                return out;
            }
            let cap = buf.len().min(remaining.min(usize::MAX as u64) as usize);
            let n = buf[..cap]
                .iter()
                .position(|c| *c == b'\n')
                .map_or(cap, |n| n + 1);
            let done = buf[n - 1] == b'\n';
            reader.consume(n);
            drained += n as u64;
            out.bytes_read += n as u64;
            if done {
                break;
            }
        }
    }
    out
}
/// Rows must come from collect_diagnostic_main_threads against caller-owned DB snapshots.
/// Unlike provider convergence, all active main-thread providers are observed.
pub fn inspect_thread_history(
    rows: &[MainThread],
    roots: &[PathBuf],
    limits: HistoryLimits,
) -> HistoryInspection {
    let ids = collect_rollout_ids(roots);
    inspect_thread_history_with_ids(rows, &ids, limits)
}
pub fn inspect_thread_history_with_ids(
    rows: &[MainThread],
    ids: &BTreeSet<String>,
    limits: HistoryLimits,
) -> HistoryInspection {
    let mut out = HistoryInspection {
        total: rows.len(),
        ..Default::default()
    };
    let selected = rows
        .iter()
        .skip(limits.thread_offset)
        .take(limits.thread_limit)
        .collect::<Vec<_>>();
    out.selected = selected.len();
    let mut existing = vec![];
    for row in selected {
        let Some(path) = row.rollout_path.as_deref() else {
            out.missing_paths += 1;
            continue;
        };
        match resolve_rollout(Path::new(path)) {
            Ok(Some(p)) => existing.push((row, p)),
            Ok(None) => out.missing_rollouts += 1,
            Err(e) => out.errors.push(format!("{}: {e}", row.id)),
        }
    }
    let mut remaining = limits.total_bytes;
    for (n, (row, path)) in existing.iter().enumerate() {
        if remaining == 0 {
            out.partial += 1;
            continue;
        }
        let share = (remaining / (existing.len() - n) as u64)
            .max(limits.min_file_bytes)
            .min(limits.max_file_bytes)
            .min(remaining);
        let file = scan_history_file(path, ids, share, limits);
        remaining = remaining.saturating_sub(file.bytes_read);
        out.scanned += 1;
        out.bytes_read += file.bytes_read;
        if file.complete {
            out.complete += 1;
        } else {
            out.partial += 1;
        }
        out.structural_errors += file.structural_errors;
        out.missing_base_refs += file.missing_base_refs;
        out.oversized_lines += file.oversized_lines;
        if let Some(e) = file.error {
            out.errors.push(format!("{}: {e}", row.id));
        }
        if file.structural_errors > 0 && out.examples.len() < 3 {
            out.examples.push(format!(
                "{}: {} 条记录结构异常",
                row.id, file.structural_errors
            ));
        }
        if file.missing_base_refs > 0 && out.examples.len() < 3 {
            out.examples
                .push(format!("{}: 继承的历史引用已不存在", row.id));
        }
    }
    out
}

/// Full coverage within the explicit ceiling supplied to observation::with_full_scope.
/// No fair-share truncation: consumes the aggregate produced by the shared stream.
pub fn inspect_thread_history_full(rows: &[MainThread]) -> Result<HistoryInspection, String> {
    let mut out = HistoryInspection {
        total: rows.len(),
        selected: rows.len(),
        ..Default::default()
    };
    for row in rows {
        let Some(path) = row.rollout_path.as_deref() else {
            out.missing_paths += 1;
            continue;
        };
        let path = match resolve_rollout(Path::new(path)) {
            Ok(Some(p)) => p,
            Ok(None) => {
                out.missing_rollouts += 1;
                continue;
            }
            Err(e) => {
                out.errors.push(format!("{}: {e}", row.id));
                continue;
            }
        };
        let result = super::observation::read(&path)
            .ok_or("full history requires full observation scope")?;
        out.scanned += 1;
        match result {
            Ok(observed) => {
                let h = observed
                    .full_history()
                    .ok_or("full history requires full observation scope")?;
                out.bytes_read += h.bytes_read;
                out.structural_errors += h.structural_errors;
                out.missing_base_refs += h.missing_base_refs;
                out.oversized_lines += h.oversized_lines;
                if h.complete {
                    out.complete += 1;
                } else {
                    out.partial += 1;
                }
                if let Some(limit) = h.limit {
                    out.limits.push((row.id.clone(), limit));
                }
                if let Some(error) = h.error {
                    out.errors.push(format!("{}: {error}", row.id));
                }
            }
            Err(e) => {
                out.partial += 1;
                if e == "operation history summary budget exhausted" {
                    out.limits
                        .push((row.id.clone(), HistoryLimit::SummaryBudget));
                } else if e == "operation history budget exhausted" {
                    out.limits.push((row.id.clone(), HistoryLimit::ByteBudget));
                } else {
                    out.errors.push(format!("{}: {e}", row.id));
                }
            }
        }
    }
    Ok(out)
}
