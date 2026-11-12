//! Operation-scoped streaming observations. No rollout bodies survive the scan.
use super::{
    consistency::{RolloutProviderAnchors, RolloutReadFailure},
    history, repair,
};
use serde_json::{json, Value};
use std::{
    cell::RefCell,
    collections::BTreeMap,
    fs,
    io::BufRead,
    path::{Path, PathBuf},
    rc::Rc,
    time::SystemTime,
};

const TOTAL_BYTES: u64 = 512 * 1024 * 1024;
const FILE_BYTES: u64 = 64 * 1024 * 1024;
const LINE_BYTES: usize = 0x1800001;
const MAX_LINES: usize = 262_144;
const MAX_FIELD_BYTES: usize = 256 * 1024;
#[derive(Clone, Debug, PartialEq, Eq)]
struct Stamp {
    len: u64,
    modified: Option<SystemTime>,
    identity: (u64, u64, i64, i64),
}
fn stamp(path: &Path) -> Result<Stamp, String> {
    let m = fs::symlink_metadata(path).map_err(|e| e.to_string())?;
    if !m.is_file() || m.file_type().is_symlink() {
        return Err("history is not a regular file".into());
    }
    #[cfg(unix)]
    let identity = {
        use std::os::unix::fs::MetadataExt;
        (m.dev(), m.ino(), m.ctime(), m.ctime_nsec())
    };
    #[cfg(not(unix))]
    let identity = (0, 0, 0, 0);
    Ok(Stamp {
        len: m.len(),
        modified: m.modified().ok(),
        identity,
    })
}
#[derive(Debug)]
struct HistoryLine {
    end: u64,
    newline: bool,
    invalid: bool,
    ordinal: Option<u64>,
    meta: bool,
    base: Option<String>,
    invalid_base: bool,
}
#[derive(Debug)]
pub struct Observation {
    pub first_meta: Option<Value>,
    pub anchors: Result<Option<RolloutProviderAnchors>, RolloutReadFailure>,
    pub response_ids: Result<usize, String>,
    pub instruction_issue: repair::SessionInstructionIssue,
    pub reference: Option<(usize, String)>,
    pub complete: bool,
    pub bytes_read: u64,
    history: Vec<HistoryLine>,
    history_error: Option<String>,
    full_history: Option<history::FileHistoryInspection>,
    error: Option<String>,
    eof: bool,
    summary_bytes: usize,
}
#[derive(Default)]
struct Cache {
    full: Option<(u64, std::collections::BTreeSet<String>)>,
    entries: BTreeMap<PathBuf, (Stamp, Result<Rc<Observation>, String>)>,
    requests: BTreeMap<PathBuf, PathBuf>,
    rows: BTreeMap<PathBuf, Result<Vec<super::MainThread>, String>>,
    providers: BTreeMap<
        PathBuf,
        (
            Stamp,
            Result<Option<RolloutProviderAnchors>, RolloutReadFailure>,
        ),
    >,
    provider_bytes: u64,
    opens: usize,
    bytes: u64,
    summary_bytes: usize,
}
thread_local! { static CACHE: RefCell<Option<Cache>> = const { RefCell::new(None) }; }
pub fn active() -> bool {
    CACHE.with(|cache| cache.borrow().is_some())
}
pub fn with_scope<T>(f: impl FnOnce() -> T) -> T {
    let owner = CACHE.with(|cache| {
        let mut cache = cache.borrow_mut();
        if cache.is_some() {
            false
        } else {
            *cache = Some(Cache::default());
            true
        }
    });
    struct Guard(bool);
    impl Drop for Guard {
        fn drop(&mut self) {
            if self.0 {
                CACHE.with(|cache| *cache.borrow_mut() = None);
            }
        }
    }
    let _guard = Guard(owner);
    f()
}
/// Run all diagnostic consumers in one bounded streaming operation. The caller supplies
/// an explicit decompressed-byte safety ceiling; exhausted coverage stays incomplete.
/// History is aggregated while streaming, not retained as per-record replay summaries.
/// Call this OUTSIDE the application's snapshot/observation scope.
pub fn with_full_scope<T>(
    roots: &[PathBuf],
    total_bytes: u64,
    f: impl FnOnce() -> T,
) -> Result<T, String> {
    if active() {
        return Err("full observation must own the outer scope".into());
    }
    let ids = history::collect_rollout_ids(roots);
    Ok(with_scope(|| {
        CACHE.with(|c| c.borrow_mut().as_mut().unwrap().full = Some((total_bytes, ids)));
        f()
    }))
}
pub fn full_active() -> bool {
    CACHE.with(|cache| cache.borrow().as_ref().is_some_and(|c| c.full.is_some()))
}
pub fn metrics() -> (usize, u64) {
    CACHE.with(|cache| {
        cache
            .borrow()
            .as_ref()
            .map_or((0, 0), |c| (c.opens, c.bytes))
    })
}
pub fn retained_summary_bytes() -> usize {
    CACHE.with(|cache| cache.borrow().as_ref().map_or(0, |c| c.summary_bytes))
}
#[cfg(test)]
thread_local! { static AFTER_RECORD: RefCell<Option<Box<dyn FnOnce()>>> = const { RefCell::new(None) }; }
#[cfg(test)]
pub(super) fn after_record(hook: impl FnOnce() + 'static) {
    AFTER_RECORD.with(|h| *h.borrow_mut() = Some(Box::new(hook)));
}
/// Validate every dependency again before publishing the operation's result.
pub fn validate() -> Result<(), String> {
    CACHE.with(|cache| {
        let cache = cache.borrow();
        let Some(cache) = cache.as_ref() else {
            return Ok(());
        };
        for (path, (expected, _)) in &cache.entries {
            if &stamp(path)? != expected {
                return Err("history changed during observation; retry required".into());
            }
        }
        for (path, (expected, _)) in &cache.providers {
            if &stamp(path)? != expected {
                return Err("provider history changed during observation; retry required".into());
            }
        }
        for (requested, expected) in &cache.requests {
            let physical = physical(requested)?;
            if &physical != expected {
                return Err(
                    "history representation changed during observation; retry required".into(),
                );
            }
        }
        Ok(())
    })
}
fn physical(path: &Path) -> Result<PathBuf, String> {
    let resolved = super::resolve_rollout(path)
        .map_err(|e| e.to_string())?
        .ok_or("rollout missing")?;
    stamp(&resolved)?;
    fs::canonicalize(resolved).map_err(|e| e.to_string())
}
/// None means no observation scope; callers retain their standalone behavior.
pub fn read(path: &Path) -> Option<Result<Rc<Observation>, String>> {
    if !active() {
        return None;
    }
    Some((|| {
        let physical = physical(path)?;
        let before = stamp(&physical)?;
        CACHE.with(|cache| -> Result<(), String> {
            let mut cache = cache.borrow_mut();
            let c = cache.as_mut().unwrap();
            if c.requests.get(path).is_some_and(|p| p != &physical) {
                return Err(
                    "history representation changed during observation; retry required".into(),
                );
            }
            c.requests.insert(path.into(), physical.clone());
            Ok(())
        })?;
        let cached = CACHE.with(|cache| {
            let cache = cache.borrow();
            let c = cache.as_ref().unwrap();
            c.entries.get(&physical).cloned().or_else(|| {
                #[cfg(unix)]
                {
                    c.entries
                        .values()
                        .find(|(s, _)| {
                            s.identity.0 == before.identity.0 && s.identity.1 == before.identity.1
                        })
                        .cloned()
                }
                #[cfg(not(unix))]
                {
                    None
                }
            })
        });
        if let Some((expected, result)) = cached {
            CACHE.with(|cache| {
                cache
                    .borrow_mut()
                    .as_mut()
                    .unwrap()
                    .entries
                    .insert(physical.clone(), (expected.clone(), result.clone()));
            });
            return if expected == before {
                result
            } else {
                Err("history changed during observation; retry required".into())
            };
        }
        let allowance = CACHE.with(|cache| {
            let cache = cache.borrow();
            let c = cache.as_ref().unwrap();
            match &c.full {
                Some((total, _)) => total.saturating_sub(c.bytes),
                None => TOTAL_BYTES.saturating_sub(c.bytes).min(FILE_BYTES),
            }
        });
        let summary_budget = CACHE.with(|cache| {
            (64usize * 1024 * 1024).saturating_sub(cache.borrow().as_ref().unwrap().summary_bytes)
        });
        let full_ids = CACHE.with(|c| {
            c.borrow()
                .as_ref()
                .unwrap()
                .full
                .as_ref()
                .map(|(_, ids)| ids.clone())
        });
        let mut result =
            scan_with_ids(&physical, allowance, summary_budget, full_ids.as_ref()).map(Rc::new);
        if stamp(&physical)? != before {
            result = Err("history changed while streaming; retry required".into());
        }
        CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            let c = cache.as_mut().unwrap();
            c.opens += usize::from(allowance != 0);
            c.bytes += result.as_ref().map_or(allowance, |v| v.bytes_read);
            c.summary_bytes += result.as_ref().map_or(0, |v| v.summary_bytes);
            c.entries.insert(physical, (before, result.clone()));
        });
        result
    })())
}
#[cfg(test)]
pub(super) fn force_partial(path: &Path) {
    let physical = physical(path).unwrap();
    let before = stamp(&physical).unwrap();
    let observed = scan(&physical, 1, 4 * 1024 * 1024).unwrap();
    CACHE.with(|cache| {
        let mut cache = cache.borrow_mut();
        let c = cache.as_mut().unwrap();
        c.requests.insert(path.into(), physical.clone());
        c.entries.insert(physical, (before, Ok(Rc::new(observed))));
    });
}
/// Native traversal when the general history scan cannot cover the file. The first
/// attempt stays inside the operation's shared budget; on budget exhaustion it
/// repeats the recovered native producer's unbounded read instead of refusing.
/// Never convert partial anchors into a healthy result.
pub(super) fn native_provider(
    path: &Path,
    mut read: impl FnMut(Option<u64>) -> Result<Option<RolloutProviderAnchors>, RolloutReadFailure>,
) -> Result<Option<RolloutProviderAnchors>, RolloutReadFailure> {
    let fail = RolloutReadFailure::Other;
    let physical = physical(path).map_err(fail)?;
    let before = stamp(&physical).map_err(fail)?;
    let cached = CACHE.with(|cache| {
        cache
            .borrow()
            .as_ref()
            .and_then(|c| c.providers.get(&physical).cloned())
    });
    if let Some((expected, result)) = cached {
        return if expected == before {
            result
        } else {
            Err(fail("provider history changed; retry required".into()))
        };
    }
    let allowance = CACHE
        .with(|cache| {
            cache
                .borrow()
                .as_ref()
                .map_or(0, |c| TOTAL_BYTES.saturating_sub(c.provider_bytes))
        })
        .min(FILE_BYTES);
    let mut result = read(Some(allowance));
    let exhausted = matches!(
        &result,
        Err(RolloutReadFailure::Other(message)) if message.contains("budget exhausted")
    );
    if exhausted {
        // Real histories can exceed the Atelier-side cap. The recovered native
        // producer has no such cap; match it before reporting "not verified".
        result = read(None);
    }
    if stamp(&physical).map_err(fail)? != before {
        return Err(fail(
            "provider history changed while reading; retry required".into(),
        ));
    }
    CACHE.with(|cache| {
        if let Some(c) = cache.borrow_mut().as_mut() {
            c.provider_bytes += allowance;
            c.providers.insert(physical, (before, result.clone()));
        }
    });
    result
}
pub(super) fn cached_rows(path: &Path) -> Option<Result<Vec<super::MainThread>, String>> {
    CACHE.with(|cache| {
        cache
            .borrow()
            .as_ref()
            .and_then(|c| c.rows.get(path).cloned())
    })
}
pub(super) fn store_rows(path: &Path, rows: Result<Vec<super::MainThread>, String>) {
    CACHE.with(|cache| {
        if let Some(c) = cache.borrow_mut().as_mut() {
            c.rows.insert(path.into(), rows);
        }
    });
}
fn provider(value: &Value, which: &str) -> Result<Option<String>, RolloutReadFailure> {
    match value.pointer("/payload/model_provider") {
        None | Some(Value::Null) => Ok(None),
        Some(Value::String(s)) if !s.trim().is_empty() => Ok(Some(s.clone())),
        _ => Err(RolloutReadFailure::Other(format!(
            "{which} session_meta provider invalid; history preserved"
        ))),
    }
}
pub(super) fn scan(path: &Path, budget: u64, summary_budget: usize) -> Result<Observation, String> {
    scan_with_ids(path, budget, summary_budget, None)
}
fn scan_with_ids(
    path: &Path,
    budget: u64,
    summary_budget: usize,
    full_ids: Option<&std::collections::BTreeSet<String>>,
) -> Result<Observation, String> {
    let mut out = Observation {
        first_meta: None,
        anchors: Ok(None),
        response_ids: Ok(0),
        instruction_issue: repair::SessionInstructionIssue::Healthy,
        reference: None,
        complete: false,
        bytes_read: 0,
        history: vec![],
        history_error: None,
        full_history: full_ids.map(|_| Default::default()),
        error: None,
        eof: false,
        summary_bytes: 0,
    };
    if budget == 0 {
        return Err("operation history budget exhausted".into());
    }
    if summary_budget < 4 * MAX_FIELD_BYTES {
        return Err("operation history summary budget exhausted".into());
    }
    let max_lines = MAX_LINES.min(
        (summary_budget.saturating_sub(4 * MAX_FIELD_BYTES)) / std::mem::size_of::<HistoryLine>(),
    );
    let file = fs::File::open(path).map_err(|e| e.to_string())?;
    let mut reader: Box<dyn BufRead> = if path.to_string_lossy().ends_with(".jsonl.zst") {
        Box::new(std::io::BufReader::new(
            zstd::stream::read::Decoder::new(file).map_err(|e| e.to_string())?,
        ))
    } else {
        Box::new(std::io::BufReader::new(file))
    };
    let mut record = vec![];
    let mut oversized = false;
    let mut last_provider = Ok(None);
    let mut settings = None;
    let mut line = 0;
    let mut base_bytes = 0usize;
    let mut previous = None;
    let mut first_meta = false;
    while out.bytes_read < budget {
        if full_ids.is_none()
            && (out.history.len() == max_lines
                || out.history.len() * 2 * std::mem::size_of::<HistoryLine>()
                    + base_bytes
                    + 4 * MAX_FIELD_BYTES
                    >= summary_budget)
        {
            out.history_error =
                Some("history summary limit exhausted; structure coverage partial".into());
        }
        let cap = (budget - out.bytes_read).min(LINE_BYTES as u64) as usize;
        let n = match history::chunk(&mut *reader, cap, &mut record) {
            Ok(n) => n,
            Err(e) => {
                out.bytes_read += record.len() as u64;
                out.error = Some(e.to_string());
                break;
            }
        };
        if n == 0 {
            out.complete = true;
            break;
        }
        line += 1;
        out.bytes_read += n as u64;
        #[cfg(test)]
        if let Some(hook) = AFTER_RECORD.with(|h| h.borrow_mut().take()) {
            hook();
        }
        let mut newline = record.last() == Some(&b'\n');
        if !newline && n == LINE_BYTES {
            // Drain without retaining oversized bodies. Other checks remain explicitly incomplete.
            oversized = true;
            if let Some(h) = out.full_history.as_mut() {
                h.oversized_lines += 1;
            }
            while out.bytes_read < budget {
                let available = match reader.fill_buf() {
                    Ok(b) => b,
                    Err(e) => {
                        out.error = Some(e.to_string());
                        break;
                    }
                };
                if available.is_empty() {
                    break;
                }
                let cap = available.len().min((budget - out.bytes_read) as usize);
                let n = available[..cap]
                    .iter()
                    .position(|b| *b == b'\n')
                    .map_or(cap, |i| i + 1);
                newline = available[n - 1] == b'\n';
                reader.consume(n);
                out.bytes_read += n as u64;
                if newline {
                    break;
                }
            }
            if full_ids.is_none() && out.history_error.is_none() {
                out.history.push(HistoryLine {
                    end: out.bytes_read,
                    newline,
                    invalid: false,
                    ordinal: None,
                    meta: false,
                    base: None,
                    invalid_base: false,
                });
            }
            if !newline {
                break;
            }
            continue;
        }
        let value = serde_json::from_slice::<Value>(&record).ok();
        let body = record.strip_suffix(b"\n").unwrap_or(&record);
        let body = body.strip_suffix(b"\r").unwrap_or(body);
        if newline && !body.is_empty() {
            if let (Some(ids), Some(h)) = (full_ids, out.full_history.as_mut()) {
                history::record_value(value.as_ref(), ids, &mut previous, &mut first_meta, h);
            }
        }
        let meta = value.as_ref().is_some_and(|v| v["type"] == "session_meta");
        let base_value = value
            .as_ref()
            .filter(|_| meta)
            .and_then(|v| v.pointer("/payload/history_base"))
            .filter(|v| !v.is_null());
        let base = base_value
            .and_then(|v| v.get("thread_id"))
            .and_then(Value::as_str)
            .filter(|s| !s.is_empty())
            .map(str::to_owned);
        let ordinal = value
            .as_ref()
            .and_then(|v| v.get("ordinal"))
            .and_then(Value::as_u64);
        let invalid_base = base_value.is_some_and(|v| {
            let end = v.get("end_ordinal_exclusive").and_then(Value::as_u64);
            end.is_none() || end != ordinal
        });
        if base.as_ref().is_some_and(|s| s.len() > MAX_FIELD_BYTES) {
            out.error = Some("history reference exceeds summary budget".into());
            break;
        }
        if full_ids.is_none() && out.history_error.is_none() {
            base_bytes += base.as_ref().map_or(0, String::len);
            out.history.push(HistoryLine {
                end: out.bytes_read,
                newline,
                invalid: !body.is_empty() && value.is_none(),
                ordinal,
                meta,
                base,
                invalid_base,
            });
        }
        if let Some(v) = &value {
            if v["type"] == "session_meta" {
                if record.len() > MAX_FIELD_BYTES {
                    out.error = Some("session metadata exceeds observation budget".into());
                    break;
                }
                if line <= 5 && out.reference.is_none() {
                    if let Some(text) = v
                        .pointer("/payload/base_instructions/text")
                        .and_then(Value::as_str)
                        .filter(|s| {
                            s.len() > 1000
                                && !s.trim_start().starts_with(repair::APPROVAL_JUDGE_PREFIX)
                        })
                    {
                        out.reference = Some((line, text.into()));
                    }
                }
                if out.first_meta.is_none() {
                    // Only retain fields actually consumed by identity/cwd/provider checks.
                    out.first_meta = Some(
                        json!({"type":"session_meta", "payload":{"id":v.pointer("/payload/id"), "cwd":v.pointer("/payload/cwd"), "model_provider":v.pointer("/payload/model_provider")}}),
                    );
                    out.instruction_issue = repair::session_meta_instruction_issue_value(v);
                    last_provider = provider(v, "last");
                    settings = None;
                } else if v.pointer("/payload/id").and_then(Value::as_str)
                    == out
                        .first_meta
                        .as_ref()
                        .and_then(|v| v.pointer("/payload/id"))
                        .and_then(Value::as_str)
                {
                    last_provider = provider(v, "last");
                    settings = None;
                }
            } else if out.first_meta.is_some()
                && v["type"] == "event_msg"
                && v.pointer("/payload/type").and_then(Value::as_str)
                    == Some("thread_settings_applied")
            {
                let provider = v
                    .pointer("/payload/thread_settings/model_provider_id")
                    .and_then(Value::as_str)
                    .filter(|s| !s.trim().is_empty());
                if provider.is_some_and(|s| s.len() > MAX_FIELD_BYTES) {
                    out.error = Some("settings provider exceeds observation budget".into());
                    break;
                }
                settings = Some(provider.map(str::to_owned));
            }
        }
        if repair::candidate_response_record(&record) {
            match value {
                None if out.response_ids.is_ok() => {
                    out.response_ids = Err(format!("invalid candidate response record line {line}"))
                }
                Some(v) if v["type"] == "response_item" => {
                    if let (Some(kind), Some(id)) = (
                        v.pointer("/payload/type").and_then(Value::as_str),
                        v.pointer("/payload/id").and_then(Value::as_str),
                    ) {
                        if repair::canonical_response_item_id(kind, id).is_some() {
                            if let Ok(count) = &mut out.response_ids {
                                *count += 1;
                            }
                        }
                    }
                }
                _ => (),
            }
        }
        if !newline {
            out.complete = reader.fill_buf().map_err(|e| e.to_string())?.is_empty();
            break;
        }
    }
    if out.bytes_read == budget && out.error.is_none() {
        out.complete = reader.fill_buf().map_err(|e| e.to_string())?.is_empty();
    }
    out.eof = out.complete;
    if oversized {
        out.complete = false;
        out.error = Some("oversized history record; other checks incomplete".into());
    }
    if let Some(h) = out.full_history.as_mut() {
        h.bytes_read = out.bytes_read;
        h.complete = out.complete && record.last().is_none_or(|b| *b == b'\n');
        h.error = out.error.clone();
        if oversized {
            h.limit = Some(history::HistoryLimit::RecordLimit);
            h.error = None;
        } else if out.error.as_deref().is_some_and(|e| {
            matches!(
                e,
                "history reference exceeds summary budget"
                    | "session metadata exceeds observation budget"
                    | "settings provider exceeds observation budget"
            )
        }) {
            h.limit = Some(history::HistoryLimit::SummaryBudget);
            h.error = None;
        }
        if !out.complete && h.error.is_none() && h.limit.is_none() {
            h.limit = Some(history::HistoryLimit::ByteBudget);
        }
    }
    out.summary_bytes = out.history.capacity() * std::mem::size_of::<HistoryLine>()
        + base_bytes
        + out.first_meta.as_ref().map_or(0, |v| v.to_string().len())
        + out.reference.as_ref().map_or(0, |(_, text)| text.len())
        + if full_ids.is_some() {
            last_provider
                .as_ref()
                .ok()
                .and_then(|p| p.as_ref())
                .map_or(0, String::len)
                + settings
                    .as_ref()
                    .and_then(|p| p.as_ref())
                    .map_or(0, String::len)
                + std::mem::size_of::<Observation>()
        } else {
            2 * MAX_FIELD_BYTES.min(out.bytes_read as usize)
        };
    out.anchors = match &out.first_meta {
        Some(first) => {
            if first
                .pointer("/payload/id")
                .and_then(Value::as_str)
                .is_none_or(str::is_empty)
            {
                Err(RolloutReadFailure::Corrupt(format!(
                    "Invalid data: first session_meta thread id missing in {}",
                    path.display()
                )))
            } else {
                (|| {
                    Ok(Some(RolloutProviderAnchors { native: match settings { Some(Some(p)) => Some(p), Some(None) => return Err(RolloutReadFailure::Other("Operation failed: native thread settings has no provider; history preserved".into())), None => None }, first: provider(first, "first")?, last: last_provider? }))
                })()
            }
        }
        None => Ok(None),
    };
    Ok(out)
}
impl Observation {
    pub fn full_history(&self) -> Option<history::FileHistoryInspection> {
        self.full_history.clone()
    }
    pub fn require_complete(&self) -> Result<(), String> {
        if self.complete {
            Ok(())
        } else {
            Err(self
                .error
                .clone()
                .unwrap_or_else(|| "history observation incomplete; not healthy".into()))
        }
    }
    pub fn history(
        &self,
        ids: &std::collections::BTreeSet<String>,
        budget: u64,
        limits: history::HistoryLimits,
    ) -> history::FileHistoryInspection {
        let mut out = history::FileHistoryInspection::default();
        if budget == 0 || limits.line_bytes == 0 {
            return out;
        }
        let mut start = 0;
        let mut impaired = false;
        let mut previous = None;
        let mut first_meta = false;
        for line in &self.history {
            if start >= budget {
                return out;
            }
            let length = line.end - start;
            let cap = (limits.line_bytes as u64).min(budget - start);
            let n = length.min(cap);
            out.bytes_read = start + n;
            if line.newline && n == length {
                if length > LINE_BYTES as u64 {
                    out.error =
                        Some("requested line limit exceeds retained observation coverage".into());
                    return out;
                }
                out.structural_errors += usize::from(line.invalid);
                if let Some(ordinal) = line.ordinal {
                    out.structural_errors += usize::from(previous.is_some_and(|p| ordinal <= p));
                    previous = Some(ordinal);
                }
                if !first_meta && line.meta {
                    first_meta = true;
                    out.structural_errors += usize::from(line.invalid_base);
                    out.missing_base_refs +=
                        usize::from(line.base.as_ref().is_some_and(|s| !ids.contains(s)));
                }
            } else if n < limits.line_bytes as u64 {
                return out;
            } else {
                out.oversized_lines += 1;
                impaired = true;
                let drain = (length - n)
                    .min(limits.drain_bytes)
                    .min(budget - out.bytes_read);
                out.bytes_read += drain;
                if drain < length - n || !line.newline {
                    return out;
                }
            }
            start = line.end;
        }
        if out.bytes_read >= budget {
            return out;
        }
        if let Some(error) = &self.history_error {
            out.error = Some(error.clone());
            return out;
        }
        if self.eof && self.history.last().is_none_or(|l| l.newline) {
            out.complete = !impaired;
        } else if !self.eof {
            out.bytes_read = self.bytes_read.min(budget);
            out.error = self
                .error
                .clone()
                .or_else(|| Some("history observation incomplete".into()));
        }
        out
    }
}
