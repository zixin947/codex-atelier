//! Native transition audit events, recovered from append_event @10078ebd8.
//! New source/reason strings become fixed categories; phase is copied verbatim.
//! Native read/publish errors are best effort; the receipt exposes a dropped event.
use crate::{
    catalog::publication::{write_atomic_report, AcknowledgedPublication, Publication},
    transition_journal::TransitionTarget,
};
use serde::{Deserialize, Serialize};
use std::{
    fs, io,
    path::{Path, PathBuf},
    time::{SystemTime, UNIX_EPOCH},
};
pub const FILE_NAME: &str = "router-events.jsonl";
pub const KEEP_LAST: usize = 200;
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EventSource {
    Compensation,
    Diagnostic,
    Bootstrap,
    AuthMode,
    ModelSlots,
    Toggle,
    Repair,
    Other,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EventReason {
    StorageCapacity,
    DbLock,
    ProcessControl,
    ThreadReconcile,
    ConfigTakeover,
    ModelCatalog,
    ProjectState,
    ConfigSync,
    Auth,
    StatePersist,
    Proxy,
    Completed,
    Unknown,
}
fn any(text: &str, needles: &[&str]) -> bool {
    needles.iter().any(|n| text.contains(n))
}
/// @10078ec8c..10078f824: ASCII-only lowercase, substring priority.
pub fn classify_source(source: &str) -> EventSource {
    let s = source.to_ascii_lowercase();
    if any(&s, &["rollback", "compensation"]) {
        EventSource::Compensation
    } else if s.contains("diagnostic") {
        EventSource::Diagnostic
    } else if s.contains("bootstrap") {
        EventSource::Bootstrap
    } else if s.contains("auth-mode") {
        EventSource::AuthMode
    } else if s.contains("model slots") {
        EventSource::ModelSlots
    } else if s.contains("router toggle") {
        EventSource::Toggle
    } else if any(&s, &["repair", "reconcile"]) {
        EventSource::Repair
    } else {
        EventSource::Other
    }
}
/// @10078ece4..10078fa1c. The returned fixed category is the entire new reason;
/// no caller text is copied or truncated into the event.
pub fn classify_reason(reason: &str) -> EventReason {
    let s = reason.to_ascii_lowercase();
    if any(
        &s,
        &[
            "no space",
            "disk full",
            "enospc",
            "insufficient space",
            "write capacity",
            "storage capacity",
            "空间不足",
            "磁盘已满",
        ],
    ) {
        EventReason::StorageCapacity
    } else if any(
        &s,
        &[
            "database is locked",
            "busy",
            "db holder",
            "held by",
            "数据库被锁",
            "数据库锁",
            "数据库占用",
        ],
    ) {
        EventReason::DbLock
    } else if any(
        &s,
        &[
            "taskkill",
            "force close",
            "force-close",
            "quiet window",
            "transition fence",
            "re-entry",
            "reentry",
            "failed to stop",
            "still running",
            "无法关闭进程",
            "进程仍在运行",
            "反复启动",
        ],
    ) {
        EventReason::ProcessControl
    } else if any(&s, &["thread", "rollout", "线程"]) {
        EventReason::ThreadReconcile
    } else if any(&s, &["takeover", "backup", "接管", "备份"]) {
        EventReason::ConfigTakeover
    } else if any(
        &s,
        &[
            "model catalog",
            "model_catalog",
            "catalog",
            "模型目录",
            "模型列表",
        ],
    ) {
        EventReason::ModelCatalog
    } else if any(&s, &["project", "workspace root", "项目", "工作区"]) {
        EventReason::ProjectState
    } else if any(&s, &["config", "toml", "managed block", "配置"]) {
        EventReason::ConfigSync
    } else if any(&s, &["auth", "login", "token", "认证", "登录", "令牌"]) {
        EventReason::Auth
    } else if any(&s, &["persist", "relay state", "持久化", "路由状态"]) {
        EventReason::StatePersist
    } else if any(&s, &["proxy", "port", "代理", "端口"]) {
        EventReason::Proxy
    } else if any(&s, &["completed", "already stable"]) {
        EventReason::Completed
    } else {
        EventReason::Unknown
    }
}
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransitionEvent {
    pub schema_version: u8,
    pub target: TransitionTarget,
    pub phase: String,
    pub occurred_at_ms: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<EventSource>,
    pub reason: Option<EventReason>,
}
impl TransitionEvent {
    pub fn new(
        target: TransitionTarget,
        phase: &str,
        source: Option<&str>,
        reason: Option<&str>,
        occurred_at_ms: i64,
    ) -> Self {
        Self {
            schema_version: 1,
            target,
            phase: phase.to_owned(),
            occurred_at_ms,
            source: source.map(classify_source),
            reason: reason.map(classify_reason),
        }
    }
}
/// Native previous log is str.lines().map(str::to_owned), not JSON parsing or
/// trimming/filtering. Blank and invalid-JSON lines count toward the 200 entries.
pub fn render_append(
    previous: &str,
    event: &TransitionEvent,
) -> Result<Vec<u8>, serde_json::Error> {
    let encoded = serde_json::to_string(event)?;
    Ok(append_encoded(previous, encoded))
}
fn append_encoded(previous: &str, encoded: String) -> Vec<u8> {
    let mut lines = previous.lines().map(str::to_owned).collect::<Vec<_>>();
    lines.push(encoded);
    if lines.len() > KEEP_LAST {
        lines.drain(..lines.len() - KEEP_LAST);
    }
    let mut text = lines.join("\n");
    text.push('\n');
    text.into_bytes()
}
pub trait EventIo {
    fn read_text(&self, path: &Path) -> io::Result<String>;
    fn publish(&self, path: &Path, bytes: &[u8]) -> io::Result<Publication>;
}
#[derive(Clone, Copy, Debug, Default)]
pub struct LocalEventIo;
impl EventIo for LocalEventIo {
    fn read_text(&self, path: &Path) -> io::Result<String> {
        fs::read_to_string(path)
    }
    fn publish(&self, path: &Path, bytes: &[u8]) -> io::Result<Publication> {
        write_atomic_report(path, bytes)
    }
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum AppendOutcome {
    Written(AcknowledgedPublication),
    Dropped {
        stage: DropStage,
        error: io::ErrorKind,
    },
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DropStage {
    Serialization,
    Publication,
}
/// Explicit timestamp entrypoint for the Manager's clock and deterministic fixtures.
/// Any previous read error, including invalid UTF-8, rebuilds from an empty log.
/// Publication failure is returned as a dropped audit event, never raised as a
/// transition error. Directory-sync uncertainty uses native acknowledge semantics.
pub fn append_event_with_io(
    io: &impl EventIo,
    relay_root: &Path,
    target: TransitionTarget,
    phase: &str,
    source: Option<&str>,
    reason: Option<&str>,
    occurred_at_ms: i64,
) -> AppendOutcome {
    let event = TransitionEvent::new(target, phase, source, reason, occurred_at_ms);
    let encoded = match serde_json::to_string(&event) {
        Ok(encoded) => encoded,
        Err(_) => {
            return AppendOutcome::Dropped {
                stage: DropStage::Serialization,
                error: io::ErrorKind::InvalidData,
            }
        }
    };
    let path = relay_root.join(FILE_NAME);
    let previous = io.read_text(&path).unwrap_or_default();
    let bytes = append_encoded(&previous, encoded);
    match io.publish(&path, &bytes) {
        Ok(publication) => AppendOutcome::Written(publication.acknowledge()),
        Err(e) => AppendOutcome::Dropped {
            stage: DropStage::Publication,
            error: e.kind(),
        },
    }
}
pub fn append_event_at(
    relay_root: &Path,
    target: TransitionTarget,
    phase: &str,
    source: Option<&str>,
    reason: Option<&str>,
    occurred_at_ms: i64,
) -> AppendOutcome {
    append_event_with_io(
        &LocalEventIo,
        relay_root,
        target,
        phase,
        source,
        reason,
        occurred_at_ms,
    )
}
/// Actual local filesystem and current UTC milliseconds; never discovers a home.
pub fn append_event(
    relay_root: &Path,
    target: TransitionTarget,
    phase: &str,
    source: Option<&str>,
    reason: Option<&str>,
) -> AppendOutcome {
    append_event_at(
        relay_root,
        target,
        phase,
        source,
        reason,
        timestamp_millis(SystemTime::now()),
    )
}
pub fn event_path(relay_root: &Path) -> PathBuf {
    relay_root.join(FILE_NAME)
}
fn timestamp_millis(time: SystemTime) -> i64 {
    match time.duration_since(UNIX_EPOCH) {
        Ok(d) => (d.as_secs() as i128 * 1000 + d.subsec_millis() as i128) as i64,
        Err(e) => {
            let d = e.duration();
            (-(d.as_secs() as i128 * 1000 + (d.subsec_nanos() as i128 + 999_999) / 1_000_000))
                as i64
        }
    }
}
#[cfg(test)]
mod clock_tests {
    use super::*;
    #[test]
    fn epoch_floor_matches_chrono_milliseconds() {
        assert_eq!(timestamp_millis(UNIX_EPOCH), 0);
        assert_eq!(
            timestamp_millis(UNIX_EPOCH - std::time::Duration::from_nanos(1)),
            -1
        );
        assert_eq!(
            timestamp_millis(UNIX_EPOCH + std::time::Duration::from_nanos(999_999)),
            0
        );
    }
}
