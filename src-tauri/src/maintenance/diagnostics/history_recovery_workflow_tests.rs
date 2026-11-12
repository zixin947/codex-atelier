use super::*;
#[test]
fn compressed_rollout_uses_matching_plain_backup_and_preserves_sources() {
    let home = std::env::temp_dir().join(format!("history-workflow-{}",uuid::Uuid::new_v4()));
    let relative = Path::new("sessions/2026/rollout-fixture.jsonl");
    let source = home.join("backups/provider-fix-fixture").join(relative);
    let target = home.join(relative).with_extension("jsonl.zst");
    std::fs::create_dir_all(source.parent().unwrap()).unwrap();
    std::fs::create_dir_all(target.parent().unwrap()).unwrap();
    let backup = "{\"type\":\"event_msg\",\"text\":\"a\u{2028}b\"}\n".as_bytes();
    let current = "{\"type\":\"event_msg\",\"text\":\"a\nb\"}\n".as_bytes();
    let compressed = zstd::stream::encode_all(current,3).unwrap();
    std::fs::write(&source,backup).unwrap();
    std::fs::write(&target,&compressed).unwrap();
    let plans = candidates(&home,&[home.join(relative)]).unwrap();
    assert_eq!(plans.len(),1);
    assert_eq!(plans[0].repaired_records,1);
    assert_eq!(logical(&target,&plans[0].after).unwrap(),backup);
    assert_eq!(std::fs::read(&target).unwrap(),compressed);
    assert_eq!(std::fs::read(&source).unwrap(),backup);
    let evidence = vec![(source.clone(), backup.to_vec())];
    validate_evidence(&evidence).unwrap();
    std::fs::write(&source, b"changed").unwrap();
    assert!(validate_evidence(&evidence).is_err());
    std::fs::remove_file(&source).unwrap();
    assert!(candidates(&home,&[target]).unwrap().is_empty());
    std::fs::remove_dir_all(home).unwrap();
}

#[test]
fn non_error_is_never_marked_fixable() {
    let mut item = DiagnosticItem { id: ID.into(), group: "runtime".into(), label: "incomplete".into(), status: "warning".into(), detail: String::new(), fixable: false };
    annotate(&mut item, Path::new("/nonexistent-synthetic-home"), &[]);
    assert!(!item.fixable);
    assert!(item.detail.is_empty());
}
