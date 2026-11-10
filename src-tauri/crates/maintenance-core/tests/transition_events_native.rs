use atelier_maintenance_core::catalog::publication::Publication;
use atelier_maintenance_core::transition_events as events;
use atelier_maintenance_core::transition_journal::TransitionTarget;
use events::*;
use std::{
    cell::RefCell,
    fs, io,
    path::{Path, PathBuf},
};
#[test]
fn source_substring_priority_and_ascii_only_folding() {
    let cases = [
        ("", EventSource::Other),
        ("REPAIR nightly", EventSource::Repair),
        ("reconcile-fixture", EventSource::Repair),
        (
            "rollback diagnostic bootstrap auth-mode model slots router toggle repair",
            EventSource::Compensation,
        ),
        ("compensation", EventSource::Compensation),
        ("diagnostic bootstrap", EventSource::Diagnostic),
        ("bootstrap auth-mode", EventSource::Bootstrap),
        ("auth-mode model slots", EventSource::AuthMode),
        ("model slots router toggle", EventSource::ModelSlots),
        ("router toggle repair", EventSource::Toggle),
        ("auth_mode", EventSource::Other),
        ("model_slots", EventSource::Other),
        ("reconciling", EventSource::Other),
        ("ＲＥＰＡＩＲ", EventSource::Other),
    ];
    for (input, want) in cases {
        assert_eq!(classify_source(input), want, "{input}")
    }
}
#[test]
fn every_native_reason_keyword_is_classified_without_retaining_input() {
    let groups: &[(&[&str], EventReason)] = &[
        (
            &[
                "no space",
                "DISK FULL",
                "enospc",
                "insufficient space",
                "write capacity",
                "storage capacity",
                "空间不足",
                "磁盘已满",
            ],
            EventReason::StorageCapacity,
        ),
        (
            &[
                "database is locked",
                "busy",
                "db holder",
                "held by",
                "数据库被锁",
                "数据库锁",
                "数据库占用",
            ],
            EventReason::DbLock,
        ),
        (
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
            EventReason::ProcessControl,
        ),
        (&["thread", "rollout", "线程"], EventReason::ThreadReconcile),
        (
            &["takeover", "backup", "接管", "备份"],
            EventReason::ConfigTakeover,
        ),
        (
            &[
                "model catalog",
                "model_catalog",
                "catalog",
                "模型目录",
                "模型列表",
            ],
            EventReason::ModelCatalog,
        ),
        (
            &["project", "workspace root", "项目", "工作区"],
            EventReason::ProjectState,
        ),
        (
            &["config", "toml", "managed block", "配置"],
            EventReason::ConfigSync,
        ),
        (
            &["auth", "login", "token", "认证", "登录", "令牌"],
            EventReason::Auth,
        ),
        (
            &["persist", "relay state", "持久化", "路由状态"],
            EventReason::StatePersist,
        ),
        (&["proxy", "port", "代理", "端口"], EventReason::Proxy),
        (&["completed", "already stable"], EventReason::Completed),
    ];
    for (words, want) in groups {
        for word in *words {
            let input = format!("prefix {word} fixture-sensitive-value suffix");
            assert_eq!(classify_reason(&input), *want, "{word}");
            assert!(!serde_json::to_string(&classify_reason(&input))
                .unwrap()
                .contains("sensitive"));
        }
    }
}
#[test]
fn reason_priority_matches_native_conflicting_inputs() {
    let cases = [
        ("no space database is locked", EventReason::StorageCapacity),
        ("busy taskkill", EventReason::DbLock),
        ("still running thread", EventReason::ProcessControl),
        ("thread backup", EventReason::ThreadReconcile),
        ("backup model_catalog", EventReason::ConfigTakeover),
        ("catalog project", EventReason::ModelCatalog),
        ("project config", EventReason::ProjectState),
        ("config auth", EventReason::ConfigSync),
        ("auth persist", EventReason::Auth),
        ("persist proxy", EventReason::StatePersist),
        ("proxy completed", EventReason::Proxy),
        ("", EventReason::Unknown),
        ("mystery fixture secret", EventReason::Unknown),
        ("ＡＵＴＨ", EventReason::Unknown),
        ("unsupported", EventReason::Proxy),
    ];
    for (input, want) in cases {
        assert_eq!(classify_reason(input), want, "{input}")
    }
}
#[test]
fn event_fields_are_exact_source_not_status_and_none_reason_is_null() {
    let e = TransitionEvent::new(TransitionTarget::RouterOn, "committing", None, None, 123);
    assert_eq!(String::from_utf8(render_append("",&e).unwrap()).unwrap(),"{\"schemaVersion\":1,\"target\":\"routerOn\",\"phase\":\"committing\",\"occurredAtMs\":123,\"reason\":null}\n");
    let e = TransitionEvent::new(
        TransitionTarget::NativeOff,
        "failed",
        Some(""),
        Some(""),
        -1,
    );
    let v = serde_json::to_value(e).unwrap();
    assert_eq!(v["source"], "other");
    assert_eq!(v["reason"], "unknown");
    assert!(v.get("status").is_none());
}
#[test]
fn original_source_and_reason_are_never_copied_into_new_event() {
    let sensitive = "fixture-sensitive-auth-token-private";
    let e = TransitionEvent::new(
        TransitionTarget::NativeOff,
        "failed",
        Some(sensitive),
        Some(sensitive),
        0,
    );
    let line = render_append("", &e).unwrap();
    let s = String::from_utf8(line).unwrap();
    assert!(!s.contains(sensitive));
    assert!(s.contains("\"source\":\"other\""));
    assert!(s.contains("\"reason\":\"auth\""));
}
#[test]
fn phase_is_verbatim_including_unicode_control_characters_and_long_text() {
    let phase = "开始\n\t\"fixture\"".repeat(1000);
    let e = TransitionEvent::new(TransitionTarget::RouterOn, &phase, None, None, 0);
    let bytes = render_append("", &e).unwrap();
    assert_eq!(bytes.iter().filter(|b| **b == b'\n').count(), 1);
    let v: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(v["phase"], phase);
}
#[test]
fn retention_counts_blank_lines_and_preserves_malformed_existing_content() {
    let previous = (0..201)
        .map(|i| {
            if i == 200 {
                String::new()
            } else {
                format!("old-{i}")
            }
        })
        .collect::<Vec<_>>()
        .join("\n")
        + "\n";
    let bytes = render_append(
        &previous,
        &TransitionEvent::new(TransitionTarget::NativeOff, "cleared", None, None, 0),
    )
    .unwrap();
    let text = String::from_utf8(bytes).unwrap();
    let lines = text.lines().collect::<Vec<_>>();
    assert_eq!(lines.len(), 200);
    assert_eq!(lines[0], "old-2");
    assert_eq!(lines[198], "");
    assert!(lines[199].contains("cleared"));
}
#[test]
fn lines_semantics_preserve_whitespace_and_standalone_cr_but_normalize_crlf() {
    let e = TransitionEvent::new(TransitionTarget::RouterOn, "prepared", None, None, 0);
    let bytes = render_append(" malformed-json \r\n\n  \nstandalone\r", &e).unwrap();
    let text = String::from_utf8(bytes).unwrap();
    assert!(text.starts_with(" malformed-json \n\n  \nstandalone\r\n{"));
    assert_eq!(text.lines().count(), 5);
}
struct FakeIo {
    prior: io::Result<String>,
    publish_error: Option<io::ErrorKind>,
    sync_error: bool,
    written: RefCell<Vec<(PathBuf, Vec<u8>)>>,
}
impl EventIo for FakeIo {
    fn read_text(&self, _: &Path) -> io::Result<String> {
        self.prior
            .as_ref()
            .map(Clone::clone)
            .map_err(|e| io::Error::from(e.kind()))
    }
    fn publish(&self, path: &Path, bytes: &[u8]) -> io::Result<Publication> {
        if let Some(e) = self.publish_error {
            return Err(e.into());
        }
        self.written.borrow_mut().push((path.into(), bytes.into()));
        Ok(Publication {
            destination: path.into(),
            directory_sync_error: self
                .sync_error
                .then(|| io::Error::from(io::ErrorKind::PermissionDenied)),
        })
    }
}
fn fake(prior: io::Result<String>) -> FakeIo {
    FakeIo {
        prior,
        publish_error: None,
        sync_error: false,
        written: RefCell::new(vec![]),
    }
}
#[test]
fn any_prior_read_error_rebuilds_empty_instead_of_failing_transition() {
    for kind in [
        io::ErrorKind::NotFound,
        io::ErrorKind::PermissionDenied,
        io::ErrorKind::InvalidData,
    ] {
        let io = fake(Err(kind.into()));
        assert!(matches!(
            append_event_with_io(
                &io,
                Path::new("/synthetic/relay"),
                TransitionTarget::RouterOn,
                "committing",
                Some("repair"),
                Some("fixture unknown"),
                7
            ),
            AppendOutcome::Written(_)
        ));
        let writes = io.written.borrow();
        assert_eq!(
            writes[0].0,
            Path::new("/synthetic/relay/router-events.jsonl")
        );
        assert_eq!(String::from_utf8_lossy(&writes[0].1).lines().count(), 1);
    }
}
#[test]
fn publication_failure_is_explicit_drop_and_directory_sync_is_acknowledged() {
    let mut io = fake(Ok(String::new()));
    io.publish_error = Some(io::ErrorKind::PermissionDenied);
    assert_eq!(
        append_event_with_io(
            &io,
            Path::new("/synthetic"),
            TransitionTarget::RouterOn,
            "prepared",
            None,
            None,
            1
        ),
        AppendOutcome::Dropped {
            stage: DropStage::Publication,
            error: io::ErrorKind::PermissionDenied
        }
    );
    assert!(io.written.borrow().is_empty());
    io.publish_error = None;
    io.sync_error = true;
    assert!(
        matches!(append_event_with_io(&io,Path::new("/synthetic"),TransitionTarget::RouterOn,"prepared",None,None,1),AppendOutcome::Written(receipt)if receipt.directory_sync_error==Some(io::ErrorKind::PermissionDenied))
    );
}
#[test]
fn local_append_creates_directory_atomically_and_replaces_invalid_utf8_log() {
    let d = tempfile::tempdir().unwrap();
    let relay = d.path().join("relay");
    assert!(matches!(
        append_event_at(
            &relay,
            TransitionTarget::RouterOn,
            "prepared",
            Some("router toggle"),
            Some("done"),
            10
        ),
        AppendOutcome::Written(_)
    ));
    let file = event_path(&relay);
    fs::write(&file, [0xff, 0xfe]).unwrap();
    assert!(matches!(
        append_event_at(
            &relay,
            TransitionTarget::NativeOff,
            "cleared",
            None,
            Some("already stable"),
            11
        ),
        AppendOutcome::Written(_)
    ));
    let text = fs::read_to_string(&file).unwrap();
    assert_eq!(text.lines().count(), 1);
    let v: serde_json::Value = serde_json::from_str(text.trim_end()).unwrap();
    assert_eq!(v["target"], "nativeOff");
    assert_eq!(v["reason"], "completed");
    assert!(fs::read_dir(&relay).unwrap().all(|e| !e
        .unwrap()
        .file_name()
        .to_string_lossy()
        .contains(".tmp.")));
}
#[test]
fn actual_clock_append_uses_real_epoch_milliseconds_without_external_services() {
    let d = tempfile::tempdir().unwrap();
    let before = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_millis() as i64;
    assert!(matches!(
        append_event(d.path(), TransitionTarget::RouterOn, "prepared", None, None),
        AppendOutcome::Written(_)
    ));
    let after = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_millis() as i64;
    let event: serde_json::Value =
        serde_json::from_slice(&fs::read(event_path(d.path())).unwrap()).unwrap();
    let time = event["occurredAtMs"].as_i64().unwrap();
    assert!(before <= time && time <= after);
}
#[test]
fn public_local_append_retains_prior_events_and_their_field_order() {
    let d = tempfile::tempdir().unwrap();
    for (target, phase, time) in [
        (TransitionTarget::RouterOn, "prepared", 1),
        (TransitionTarget::NativeOff, "cleared", 2),
    ] {
        assert!(matches!(
            append_event_at(
                d.path(),
                target,
                phase,
                Some("diagnostic"),
                Some("completed"),
                time
            ),
            AppendOutcome::Written(_)
        ))
    }
    let text = fs::read_to_string(event_path(d.path())).unwrap();
    let events = text
        .lines()
        .map(|s| serde_json::from_str::<serde_json::Value>(s).unwrap())
        .collect::<Vec<_>>();
    assert_eq!(events.len(), 2);
    assert_eq!(events[0]["phase"], "prepared");
    assert_eq!(events[1]["phase"], "cleared");
    assert_eq!(events[0]["source"], "diagnostic");
    assert_eq!(events[1]["occurredAtMs"], 2);
}
