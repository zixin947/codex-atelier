use super::{
    consistency,
    history::{self, HistoryLimits},
    observation::*,
    repair,
};
use std::{collections::BTreeSet, fs};

#[test]
fn history_replay_matches_independent_limits_plain_and_zstd() {
    let t = tempfile::tempdir().unwrap();
    let samples: Vec<Vec<u8>> = vec![
        vec![], b"{}\n".to_vec(), b"{}".to_vec(), b"\n\r\n \n{bad}\n\xff\n".to_vec(),
        b"{\"ordinal\":4}\n{\"ordinal\":4}\n{\"ordinal\":2}\n{broken".to_vec(),
        b"{\"ordinal\":4,\"type\":\"session_meta\",\"payload\":{\"history_base\":{\"thread_id\":\"missing\",\"end_ordinal_exclusive\":4}}}\n{\"ordinal\":5,\"type\":\"session_meta\",\"payload\":{\"history_base\":{}}}\n".to_vec(),
        [vec![b'x'; 600], b"\n{\"ordinal\":1}\n{\"ordinal\":1}\n".to_vec()].concat(),
    ];
    for (i, bytes) in samples.iter().enumerate() {
        for compressed in [false, true] {
            let p = t
                .path()
                .join(format!("{i}.jsonl{}", if compressed { ".zst" } else { "" }));
            fs::write(
                &p,
                if compressed {
                    zstd::stream::encode_all(bytes.as_slice(), 1).unwrap()
                } else {
                    bytes.clone()
                },
            )
            .unwrap();
            for budget in [
                0,
                1,
                2,
                3,
                10,
                bytes.len() as u64,
                bytes.len() as u64 + 1,
                4096,
            ] {
                for line_bytes in [1, 8, 512, 0x1800001] {
                    for drain_bytes in [0, 10, 4096] {
                        let limits = HistoryLimits {
                            line_bytes,
                            drain_bytes,
                            ..Default::default()
                        };
                        let expected =
                            history::scan_history_file(&p, &BTreeSet::new(), budget, limits);
                        let actual = with_scope(|| {
                            history::scan_history_file(&p, &BTreeSet::new(), budget, limits)
                        });
                        assert_eq!(actual, expected, "sample={i} compressed={compressed} budget={budget} line={line_bytes} drain={drain_bytes}");
                    }
                }
            }
        }
    }
}
#[test]
fn consumers_share_one_open_and_decode_with_first_last_settings_rules() {
    let t = tempfile::tempdir().unwrap();
    for compressed in [false, true] {
        let p = t.path().join(format!(
            "shared.jsonl{}",
            if compressed { ".zst" } else { "" }
        ));
        let bytes = b"{\"type\":\"session_meta\",\"payload\":{\"id\":\"a\",\"model_provider\":\"external\",\"base_instructions\":{\"text\":\"healthy\"}}}\r\n{\"type\":\"event_msg\",\"payload\":{\"type\":\"thread_settings_applied\",\"thread_settings\":{\"model_provider_id\":\"old\"}}}\n{\"type\":\"session_meta\",\"payload\":{\"id\":\"a\",\"model_provider\":\"openai\"}}\n{\"type\":\"event_msg\",\"payload\":{\"type\":\"thread_settings_applied\",\"thread_settings\":{\"model_provider_id\":\"latest\"}}}\n{\"type\":\"response_item\",\"payload\":{\"type\":\"custom_tool_call\",\"id\":\"ct_abc\"}}";
        fs::write(
            &p,
            if compressed {
                zstd::stream::encode_all(&bytes[..], 1).unwrap()
            } else {
                bytes.to_vec()
            },
        )
        .unwrap();
        let expected = consistency::read_rollout_provider_anchors(&p).unwrap();
        let expected_ids = repair::count_invalid_response_item_ids(&p).unwrap();
        with_scope(|| {
            assert_eq!(
                consistency::read_rollout_provider_anchors(&p).unwrap(),
                expected
            );
            assert_eq!(
                repair::count_invalid_response_item_ids(&p).unwrap(),
                expected_ids
            );
            history::scan_history_file(&p, &BTreeSet::new(), 4096, Default::default());
            assert_eq!(metrics(), (1, bytes.len() as u64));
            validate().unwrap();
        });
        assert!(!active());
    }
}
#[test]
fn cache_rejects_replacement_and_new_round_reads_changes() {
    let t = tempfile::tempdir().unwrap();
    let p = t.path().join("a.jsonl");
    fs::write(&p, b"{}\n").unwrap();
    with_scope(|| {
        read(&p).unwrap().unwrap();
        let replacement = t.path().join("new");
        fs::write(&replacement, b"[]\n").unwrap();
        fs::rename(replacement, &p).unwrap();
        assert!(validate().is_err());
        assert!(read(&p).unwrap().is_err());
    });
    with_scope(|| {
        assert_eq!(read(&p).unwrap().unwrap().bytes_read, 3);
        assert_eq!(metrics().0, 1);
        validate().unwrap();
    });
}
#[test]
fn plain_preference_change_invalidates_and_hardlinks_deduplicate() {
    let t = tempfile::tempdir().unwrap();
    let p = t.path().join("a.jsonl");
    let z = p.with_extension("jsonl.zst");
    fs::write(&z, zstd::stream::encode_all(&b"{}\n"[..], 1).unwrap()).unwrap();
    with_scope(|| {
        read(&z).unwrap().unwrap();
        fs::write(&p, b"{}\n").unwrap();
        assert!(validate().is_err());
    });
    let alias = t.path().join("alias.jsonl");
    fs::hard_link(&p, &alias).unwrap();
    with_scope(|| {
        read(&p).unwrap().unwrap();
        read(&alias).unwrap().unwrap();
        assert_eq!(metrics().0, 1);
        validate().unwrap();
    });
}
#[test]
fn mid_stream_changes_and_oversized_records_never_prove_healthy() {
    let t = tempfile::tempdir().unwrap();
    let p = t.path().join("a.jsonl");
    fs::write(&p, b"{}\n{}\n").unwrap();
    let changed = p.clone();
    super::observation::after_record(move || fs::write(changed, b"[]\n[]\n").unwrap());
    with_scope(|| {
        assert!(read(&p).unwrap().is_err());
    });
    let mut data = vec![b'x'; 0x1800002];
    data.extend_from_slice(b"\n{bad}\n");
    fs::write(&p, &data).unwrap();
    let expected =
        history::scan_history_file(&p, &BTreeSet::new(), 64 * 1024 * 1024, Default::default());
    with_scope(|| {
        let actual =
            history::scan_history_file(&p, &BTreeSet::new(), 64 * 1024 * 1024, Default::default());
        assert_eq!(actual, expected);
        assert!(read(&p).unwrap().unwrap().require_complete().is_err());
    });
}

#[test]
fn history_summary_exhaustion_does_not_discard_complete_provider_evidence() {
    let t = tempfile::tempdir().unwrap();
    let p = t.path().join("dense.jsonl");
    let mut bytes = b"{\"type\":\"session_meta\",\"payload\":{\"id\":\"dense\",\"model_provider\":\"openai\"}}\n".to_vec();
    bytes.extend(b"{}\n".repeat(30_000));
    bytes.extend_from_slice(b"{\"type\":\"event_msg\",\"payload\":{\"type\":\"thread_settings_applied\",\"thread_settings\":{\"model_provider_id\":\"aimai1\"}}}\n");
    fs::write(&p, bytes).unwrap();
    let observed = super::observation::scan(&p, 4 * 1024 * 1024, 2 * 1024 * 1024).unwrap();
    assert!(
        observed.require_complete().is_ok(),
        "history summary limit must not truncate provider stream"
    );
    assert_eq!(
        observed
            .anchors
            .as_ref()
            .unwrap()
            .as_ref()
            .unwrap()
            .native
            .as_deref(),
        Some("aimai1")
    );
    let h = observed.history(&BTreeSet::new(), 4 * 1024 * 1024, Default::default());
    assert!(!h.complete);
    assert!(h.error.as_deref().is_some_and(|e| e.contains("summary")));
}

#[test]
fn large_plain_history_uses_native_reverse_tail_and_preserves_latest_settings() {
    use std::io::{Seek, SeekFrom, Write};
    let t = tempfile::tempdir().unwrap();
    let p = t.path().join("large.jsonl");
    let first = b"{\"type\":\"session_meta\",\"payload\":{\"id\":\"large\",\"model_provider\":\"openai\"}}\n";
    let mut file = fs::File::create(&p).unwrap();
    file.write_all(first).unwrap();
    file.seek(SeekFrom::Start(65 * 1024 * 1024)).unwrap();
    file.write_all(b"\n").unwrap();
    file.write_all(first).unwrap();
    file.write_all(b"{\"type\":\"event_msg\",\"payload\":{\"type\":\"thread_settings_applied\",\"thread_settings\":{\"model_provider_id\":\"aimai1\"}}}\n").unwrap();
    drop(file);
    let expected = consistency::read_rollout_provider_anchors(&p).unwrap();
    with_scope(|| {
        let observed = read(&p).unwrap().unwrap();
        assert!(observed.require_complete().is_err());
        assert_eq!(
            consistency::read_rollout_provider_anchors(&p).unwrap(),
            expected
        );
        assert_eq!(
            consistency::read_rollout_provider_anchors(&p)
                .unwrap()
                .unwrap()
                .native
                .as_deref(),
            Some("aimai1")
        );
        validate().unwrap();
    });
}

#[test]
fn native_fallback_keeps_invalid_settings_and_source_change_errors() {
    let t = tempfile::tempdir().unwrap();
    for compressed in [false, true] {
        let p = t.path().join(format!(
            "fallback-{compressed}.jsonl{}",
            if compressed { ".zst" } else { "" }
        ));
        let mut bytes = b"{\"type\":\"session_meta\",\"payload\":{\"id\":\"a\",\"model_provider\":\"openai\"}}\n".to_vec();
        bytes.extend_from_slice(b"{\"type\":\"event_msg\",\"payload\":{\"type\":\"thread_settings_applied\",\"thread_settings\":{}}}\n");
        fs::write(
            &p,
            if compressed {
                zstd::stream::encode_all(bytes.as_slice(), 1).unwrap()
            } else {
                bytes.clone()
            },
        )
        .unwrap();
        with_scope(|| {
            // Force the general observation to be incomplete without changing the file.
            super::observation::force_partial(&p);
            let result = consistency::read_rollout_provider_anchors(&p);
            assert!(format!("{:?}", result.unwrap_err()).contains("settings has no provider"));
            fs::write(&p, b"changed\n").unwrap();
            assert!(consistency::read_rollout_provider_anchors(&p).is_err());
            assert!(validate().is_err());
        });
    }
}

#[test]
fn provider_budget_exhaustion_falls_back_to_unbounded_native_read() {
    use std::{cell::RefCell, rc::Rc};
    let t = tempfile::tempdir().unwrap();
    let p = t.path().join("big.jsonl");
    fs::write(&p, b"{}\n").unwrap();
    with_scope(|| {
        let calls: Rc<RefCell<Vec<Option<u64>>>> = Rc::new(RefCell::new(vec![]));
        let observed = {
            let calls = calls.clone();
            super::observation::native_provider(&p, move |budget| {
                calls.borrow_mut().push(budget);
                match budget {
                    Some(_) => Err(consistency::RolloutReadFailure::Other(
                        "native provider reverse-tail budget exhausted; not verified".into(),
                    )),
                    None => Ok(Some(consistency::RolloutProviderAnchors {
                        first: Some("openai".into()),
                        last: None,
                        native: None,
                    })),
                }
            })
            .unwrap()
            .unwrap()
        };
        assert_eq!(observed.first.as_deref(), Some("openai"));
        assert_eq!(calls.borrow().len(), 2);
        assert!(calls.borrow()[0].is_some());
        assert!(calls.borrow()[1].is_none());
        // The unbounded answer is cached; a later call must not re-read.
        super::observation::native_provider(&p, |_| {
            Err(consistency::RolloutReadFailure::Other(
                "must not re-read".into(),
            ))
        })
        .unwrap()
        .unwrap();
        assert_eq!(calls.borrow().len(), 2);
        validate().unwrap();
    });
}

#[test]
fn incomplete_and_summary_budget_never_prove_healthy() {
    let t = tempfile::tempdir().unwrap();
    let p = t.path().join("a.jsonl");
    fs::write(&p, b"{}\n{}\n").unwrap();
    let observed = super::observation::scan(&p, 4, 1024 * 1024 * 4).unwrap();
    assert!(observed.require_complete().is_err());
    assert!(super::observation::scan(&p, 64, 0).is_err());
}
