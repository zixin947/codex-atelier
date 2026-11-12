use super::history::*;
use super::*;
use std::{collections::BTreeSet, fs};
fn limits() -> HistoryLimits {
    HistoryLimits {
        total_bytes: 4096,
        min_file_bytes: 64,
        max_file_bytes: 4096,
        line_bytes: 512,
        drain_bytes: 1024,
        ..Default::default()
    }
}
fn scan(bytes: &[u8]) -> FileHistoryInspection {
    let t = tempfile::tempdir().unwrap();
    let p = t.path().join("fixture.jsonl");
    fs::write(&p, bytes).unwrap();
    scan_history_file(&p, &BTreeSet::new(), 4096, limits())
}
#[test]
fn history_json_utf8_and_nondecreasing_ordinals() {
    let s=scan(b"\n\r\n{\"ordinal\":4}\n{\"ordinal\":4}\n{\"ordinal\":2}\n{\"ordinal\":3}\n{bad}\n\xff\n{\"ordinal\":-1}\n{\"ordinal\":\"x\"}\n");
    assert!(s.complete);
    assert_eq!(s.structural_errors, 4); // duplicate, decrease, JSON, UTF8
    assert_eq!(s.missing_base_refs, 0);
}
#[test]
fn history_first_meta_only_exact_end_field_and_reference() {
    let s=scan(br#"{"ordinal":4,"type":"session_meta","payload":{"history_base":{"end_ordinal_exclusive":4,"thread_id":"missing"}}}
{"ordinal":5,"type":"session_meta","payload":{"history_base":{"end_ordinal_exclusive":0,"thread_id":"also-missing"}}}
"#);
    assert_eq!(s.structural_errors, 0);
    assert_eq!(s.missing_base_refs, 1);
    let s = scan(
        br#"{"ordinal":4,"type":"session_meta","payload":{"history_base":{"end_ordinal":4}}}
"#,
    );
    assert_eq!(s.structural_errors, 1);
    assert_eq!(
        scan(b"{\"type\":\"session_meta\",\"payload\":{\"history_base\":null}}\n")
            .structural_errors,
        0
    );
    assert_eq!(
        scan(b"{\"type\":\"session_meta\",\"payload\":{\"history_base\":{}}}\n").structural_errors,
        1
    );
}
#[test]
fn history_tails_are_partial_not_parsed() {
    let s = scan(b"{\"ordinal\":1}\n{broken");
    assert!(!s.complete);
    assert_eq!(s.structural_errors, 0);
    let s = scan(b"{\"ordinal\":1}");
    assert!(!s.complete);
    assert_eq!(s.structural_errors, 0);
    assert!(scan(b"").complete);
}
#[test]
fn history_long_line_is_bounded_drained_and_following_record_checked() {
    let t = tempfile::tempdir().unwrap();
    let p = t.path().join("fixture.jsonl");
    let mut data = vec![b'x'; 600];
    data.extend_from_slice(b"\n{broken}\n");
    fs::write(&p, &data).unwrap();
    let s = scan_history_file(&p, &BTreeSet::new(), 4096, limits());
    assert_eq!(s.oversized_lines, 1);
    assert_eq!(s.structural_errors, 1);
    assert!(!s.complete);
    assert_eq!(s.bytes_read, data.len() as u64);
    let s = scan_history_file(&p, &BTreeSet::new(), 530, limits());
    assert_eq!(s.bytes_read, 530);
    assert!(!s.complete);
    let l = HistoryLimits {
        drain_bytes: 10,
        ..limits()
    };
    let s = scan_history_file(&p, &BTreeSet::new(), 4096, l);
    assert_eq!(s.bytes_read, 522);
}
fn row(p: Option<PathBuf>) -> MainThread {
    MainThread {
        db: "private-copy.sqlite".into(),
        id: "fixture".into(),
        rollout_path: p.map(|p| p.to_string_lossy().into_owned()),
        provider: Some("external".into()),
        model: None,
        reasoning_effort: None,
        has_model_column: false,
        has_effort_column: false,
    }
}
#[test]
fn history_plain_preference_zstd_and_corrupt_zstd() {
    let t = tempfile::tempdir().unwrap();
    let p = t.path().join("fixture.jsonl");
    let z = p.with_extension("jsonl.zst");
    fs::write(
        &z,
        zstd::stream::encode_all(&b"{\"ordinal\":1}\n"[..], 1).unwrap(),
    )
    .unwrap();
    let s = inspect_thread_history_with_ids(&[row(Some(p.clone()))], &BTreeSet::new(), limits());
    assert!(s.coverage_complete());
    fs::write(&p, b"{bad}\n").unwrap();
    let s = inspect_thread_history_with_ids(&[row(Some(z.clone()))], &BTreeSet::new(), limits());
    assert_eq!(s.structural_errors, 1);
    fs::remove_file(p).unwrap();
    fs::write(&z, b"bad zstd").unwrap();
    let s = inspect_thread_history_with_ids(&[row(Some(z))], &BTreeSet::new(), limits());
    assert_eq!(s.errors.len(), 1);
    assert!(!s.coverage_complete());
}
#[test]
fn history_window_budget_missing_path_and_resolve_errors() {
    let t = tempfile::tempdir().unwrap();
    let p = t.path().join("fixture.jsonl");
    fs::write(&p, b"{}\n").unwrap();
    let rows = vec![row(Some(p.clone())), row(Some(p.clone()))];
    let s = inspect_thread_history_with_ids(
        &rows,
        &BTreeSet::new(),
        HistoryLimits {
            thread_offset: 1,
            thread_limit: 1,
            ..limits()
        },
    );
    assert_eq!(s.selected, 1);
    assert_eq!(s.complete, 1);
    assert!(!s.coverage_complete());
    let s = inspect_thread_history_with_ids(
        &rows,
        &BTreeSet::new(),
        HistoryLimits {
            total_bytes: 1,
            ..limits()
        },
    );
    assert_eq!(s.bytes_read, 1);
    assert_eq!(s.partial, 2);
    assert!(!s.coverage_complete());
    let rows = vec![
        row(None),
        row(Some(t.path().join("missing.jsonl"))),
        row(Some(t.path().to_path_buf())),
    ];
    let s = inspect_thread_history_with_ids(&rows, &BTreeSet::new(), limits());
    assert_eq!(s.missing_paths, 1);
    assert_eq!(s.missing_rollouts, 1);
    assert_eq!(s.errors.len(), 1);
}
#[test]
fn history_filename_uuid_index_depth_suffix_and_case() {
    let t = tempfile::tempdir().unwrap();
    let id = "ABCDEF01-2345-6789-abcd-ef0123456789";
    fs::write(
        t.path().join(format!("rollout-any-{id}.jsonl.zst")),
        b"not decoded",
    )
    .unwrap();
    fs::write(t.path().join(format!("other-{id}.jsonl")), b"").unwrap();
    let mut dir = t.path().to_path_buf();
    for _ in 0..7 {
        dir = dir.join("nested");
        fs::create_dir(&dir).unwrap();
    }
    fs::write(
        dir.join("rollout-00000000-0000-0000-0000-000000000000.jsonl"),
        b"",
    )
    .unwrap();
    let ids = collect_rollout_ids(&[t.path().to_path_buf()]);
    assert_eq!(ids.len(), 1);
    assert!(ids.contains(id));
    let p = t.path().join("first.jsonl");
    fs::write(&p,format!("{{\"ordinal\":8,\"type\":\"session_meta\",\"payload\":{{\"history_base\":{{\"end_ordinal_exclusive\":8,\"thread_id\":\"{id}\"}}}}}}\n")).unwrap();
    assert_eq!(
        scan_history_file(&p, &ids, 4096, limits()).missing_base_refs,
        0
    );
}
#[test]
fn history_budget_rebalances_per_file_and_counts_decompressed_bytes() {
    let t = tempfile::tempdir().unwrap();
    let p = t.path().join("small.jsonl");
    let z = t.path().join("large.jsonl.zst");
    fs::write(&p, b"{}\n").unwrap();
    let data = b"{}\n".repeat(1000);
    fs::write(&z, zstd::stream::encode_all(&data[..], 1).unwrap()).unwrap();
    let rows = vec![row(Some(p)), row(Some(z))];
    let s = inspect_thread_history_with_ids(
        &rows,
        &BTreeSet::new(),
        HistoryLimits {
            total_bytes: 100,
            min_file_bytes: 2,
            max_file_bytes: 80,
            ..limits()
        },
    );
    assert_eq!(s.bytes_read, 83); // small file releases unused share; second capped at 80
    assert_eq!(s.complete, 1);
    assert_eq!(s.partial, 1);
    let s = inspect_thread_history_with_ids(
        &rows,
        &BTreeSet::new(),
        HistoryLimits {
            total_bytes: 0,
            ..limits()
        },
    );
    assert_eq!(s.scanned, 0);
    assert_eq!(s.partial, 2);
    assert!(!s.coverage_complete());
}
#[test]
fn history_null_first_meta_suppresses_later_base_and_ordinal_uses_unsigned_integers() {
    let s = scan(b"{\"ordinal\":0,\"type\":\"session_meta\"}\n{\"ordinal\":1,\"type\":\"session_meta\",\"payload\":{\"history_base\":{}}}\n{\"ordinal\":1.5}\n{\"ordinal\":-1}\n{\"ordinal\":2}\n");
    assert!(s.complete);
    assert_eq!(s.structural_errors, 0);
    assert_eq!(s.missing_base_refs, 0);
}
#[test]
fn history_default_limits_are_native_values() {
    let l = HistoryLimits::default();
    assert_eq!(l.total_bytes, 0x20000000);
    assert_eq!(l.min_file_bytes, 0x200000);
    assert_eq!(l.max_file_bytes, 0x4000000);
    assert_eq!(l.line_bytes, 0x1800001);
    assert_eq!(l.drain_bytes, 0x4000000);
}

#[test]
fn full_stream_aggregates_without_fair_share_or_retained_line_summaries() {
    let t = tempfile::tempdir().unwrap();
    for compressed in [false, true] {
        let p = t.path().join(if compressed {
            "full.jsonl.zst"
        } else {
            "full.jsonl"
        });
        let bytes = b"{\"ordinal\":1}\n{\"ordinal\":1}\n{bad}\n";
        fs::write(
            &p,
            if compressed {
                zstd::stream::encode_all(&bytes[..], 1).unwrap()
            } else {
                bytes.to_vec()
            },
        )
        .unwrap();
        super::observation::with_full_scope(&[], 4096, || {
            let h = inspect_thread_history_full(&[row(Some(p.clone()))]).unwrap();
            assert_eq!(h.complete, 1);
            assert_eq!(h.structural_errors, 2);
            assert!(h.errors.is_empty());
            assert!(h.limits.is_empty());
            let again = inspect_thread_history_full(&[row(Some(p.clone()))]).unwrap();
            assert_eq!(h, again);
            assert_eq!(super::observation::metrics(), (1, bytes.len() as u64));
            super::observation::validate().unwrap();
        })
        .unwrap();
    }
}
#[test]
fn full_stream_budget_is_typed_and_unterminated_tail_stays_partial() {
    let t = tempfile::tempdir().unwrap();
    let a = t.path().join("a.jsonl");
    let b = t.path().join("b.jsonl");
    fs::write(&a, b"{}\n{}\n").unwrap();
    fs::write(&b, b"{}\n").unwrap();
    super::observation::with_full_scope(&[], 4, || {
        let h = inspect_thread_history_full(&[row(Some(a.clone())), row(Some(b.clone()))]).unwrap();
        assert_eq!(h.partial, 2);
        assert_eq!(h.limits.len(), 2);
        assert!(h.limits.iter().all(|(_, l)| *l == HistoryLimit::ByteBudget));
        assert!(h.errors.is_empty());
        assert_eq!(h.bytes_read, 4);
    })
    .unwrap();
    fs::write(&a, b"{}").unwrap();
    super::observation::with_full_scope(&[], 4096, || {
        let h = inspect_thread_history_full(&[row(Some(a.clone()))]).unwrap();
        assert_eq!(h.partial, 1);
        assert!(h.errors.is_empty());
        assert!(h.limits.is_empty());
    })
    .unwrap();
}
#[test]
fn full_stream_crosses_legacy_file_ceiling_with_bounded_summary() {
    let t = tempfile::tempdir().unwrap();
    let p = t.path().join("large.jsonl.zst");
    let line = format!("{{\"ignored\":\"{}\"}}\n", "x".repeat(1024 * 1024));
    let mut encoder = zstd::stream::write::Encoder::new(fs::File::create(&p).unwrap(), 1).unwrap();
    use std::io::Write;
    for _ in 0..65 {
        encoder.write_all(line.as_bytes()).unwrap();
    }
    encoder.finish().unwrap();
    super::observation::with_full_scope(&[], 70 * 1024 * 1024, || {
        let h = inspect_thread_history_full(&[row(Some(p.clone()))]).unwrap();
        assert_eq!(h.complete, 1);
        assert!(h.coverage_complete());
        assert_eq!(h.bytes_read, 65 * line.len() as u64);
        assert!(super::observation::retained_summary_bytes() < 1024 * 1024);
    })
    .unwrap();
}
