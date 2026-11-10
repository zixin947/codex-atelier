//! 只验证合成 rollout 字节，不读取或修改任何真实会话。
use super::poisoned_instructions::{
    find_first_session_meta_line, replace_first_session_meta_line,
    rewrite_session_meta_base_instructions,
};
use super::{inspect, repair, Context, PathBuf};
use serde_json::{json, Value};
use std::fs;

fn assert_unrelated_record_bytes_unchanged(separator: &[u8]) {
    let meta = br#"{"type":"session_meta","payload":{"id":"synthetic","base_instructions":{"text":"You are judging one planned coding-agent action"}}}"#;
    let tail = b"{\"type\":\"response_item\",\"payload\":{\"text\":\"preserve exactly\"}}\r\n";
    let mut original = meta.to_vec();
    original.extend_from_slice(separator);
    original.extend_from_slice(tail);
    let (start, length) = find_first_session_meta_line(&original).unwrap();
    let rewritten = rewrite_session_meta_base_instructions(
        &original[start..start + length],
        "Healthy instructions from a synthetic fixture",
    )
    .unwrap();
    let after = replace_first_session_meta_line(&original, &rewritten).unwrap();
    let metadata_end = after.iter().position(|b| *b == b'\n').unwrap() + 1;
    assert_eq!(
        String::from_utf8_lossy(&after[metadata_end..]),
        String::from_utf8_lossy(tail),
        "replacing metadata must not add a blank record or alter subsequent bytes"
    );
}

#[test]
fn replacing_metadata_keeps_subsequent_lf_records_exact() {
    assert_unrelated_record_bytes_unchanged(b"\n");
}

#[test]
fn replacing_metadata_keeps_subsequent_crlf_records_exact() {
    assert_unrelated_record_bytes_unchanged(b"\r\n");
}

struct ThreadFixture {
    root: PathBuf,
    ctx: Context,
}
impl Drop for ThreadFixture {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.root);
    }
}

fn assert_repair_entry_restores_a_synthetic_thread(compressed: bool, archived_decoy: bool) {
    let root = std::env::temp_dir().join(format!(
        "atelier-poisoned-workflow-{}",
        uuid::Uuid::new_v4()
    ));
    let f = ThreadFixture {
        ctx: Context {
            home: root.join("codex"),
            app_root: root.join("atelier"),
            desktop_running: false,
            runtime: None,
            official_catalog: None,
            proxy_handle: None,
            working_directory: None,
            sqlite_home_env: None,
        },
        root,
    };
    fs::create_dir_all(f.ctx.home.join("sessions")).unwrap();
    fs::create_dir_all(&f.ctx.app_root).unwrap();
    fs::write(f.ctx.config(), "model = 'gpt-fixture'\n").unwrap();
    let target = f.ctx.home.join(if compressed {
        "sessions/target.jsonl.zst"
    } else {
        "sessions/target.jsonl"
    });
    let source = f.ctx.home.join("sessions/healthy.jsonl");
    let healthy = "Healthy instructions from the only synthetic source thread. ".repeat(30);
    let tail = b"{\"type\":\"event_msg\",\"payload\":{\"message\":\"preserve  raw  body\"}}\n";
    let make_record = |id: &str, text: &str| {
        let mut b = serde_json::to_vec(&json!({"type":"session_meta", "payload":{
            "id":id, "model_provider":"openai", "cwd":f.root.to_string_lossy(),
            "base_instructions":{"text":text}, "extra":{"preserve":true}
        }}))
        .unwrap();
        b.push(b'\n');
        b.extend_from_slice(tail);
        b
    };
    let decoded = make_record(
        "target",
        "You are judging one planned coding-agent action. Return a verdict.",
    );
    let before = if compressed {
        zstd::stream::encode_all(decoded.as_slice(), 3).unwrap()
    } else {
        decoded.clone()
    };
    let source_before = make_record("healthy", &healthy);
    fs::write(&target, &before).unwrap();
    fs::write(&source, &source_before).unwrap();
    let db = rusqlite::Connection::open(f.ctx.home.join("state_5.sqlite")).unwrap();
    db.execute_batch("CREATE TABLE threads(id TEXT PRIMARY KEY,model_provider TEXT,model TEXT,rollout_path TEXT,archived INTEGER DEFAULT 0,source TEXT DEFAULT 'cli')").unwrap();
    let decoy = if archived_decoy {
        let path = f.ctx.home.join("sessions/archived.jsonl");
        let bytes = make_record(
            "a-archived",
            &"Archived instructions must never be selected. ".repeat(40),
        );
        fs::write(&path, &bytes).unwrap();
        db.execute(
            "INSERT INTO threads(id,model_provider,model,rollout_path,archived) VALUES ('a-archived','openai','gpt-fixture',?1,1)",
            rusqlite::params![path.to_str().unwrap()],
        )
        .unwrap();
        Some((path, bytes))
    } else {
        None
    };
    for (id, path) in [("target", &target), ("healthy", &source)] {
        db.execute(
            "INSERT INTO threads(id,model_provider,model,rollout_path) VALUES (?1,'openai','gpt-fixture',?2)",
            rusqlite::params![id, path.to_str().unwrap()],
        )
        .unwrap();
    }
    drop(db);
    let report = inspect::run(&f.ctx).unwrap();
    assert!(
        report
            .items
            .iter()
            .find(|i| i.id == "thread_poisoned_instructions")
            .unwrap()
            .fixable,
        "the actual diagnostic repair entry must be available"
    );
    let ticket = repair::preview(&f.ctx, "thread_poisoned_instructions").unwrap();
    assert_eq!(
        fs::read(&target).unwrap(),
        before,
        "preview must not alter the target"
    );
    assert_eq!(
        fs::read(&source).unwrap(),
        source_before,
        "preview must not alter the source"
    );
    let result = repair::apply(&f.ctx, ticket).unwrap();
    let bytes = fs::read(&target).unwrap();
    let after = if compressed {
        zstd::stream::decode_all(bytes.as_slice()).unwrap()
    } else {
        bytes
    };
    let (s, n) = find_first_session_meta_line(&after).unwrap();
    let meta: Value = serde_json::from_slice(&after[s..s + n]).unwrap();
    assert_eq!(meta["payload"]["id"], "target");
    assert_eq!(meta["payload"]["extra"]["preserve"], true);
    assert_eq!(meta["payload"]["base_instructions"]["text"], healthy);
    let newline = after.iter().position(|b| *b == b'\n').unwrap() + 1;
    assert_eq!(&after[newline..], tail);
    assert_eq!(
        fs::read(&source).unwrap(),
        source_before,
        "healthy source must remain untouched"
    );
    assert!(result
        .fixed_ids
        .iter()
        .any(|id| id == "thread_poisoned_instructions"));
    if let Some((path, original)) = decoy {
        assert_eq!(
            fs::read(path).unwrap(),
            original,
            "archived source must remain untouched"
        );
    }
    let backup = PathBuf::from(result.backup_path.unwrap());
    let original_decoded = if compressed {
        zstd::stream::decode_all(before.as_slice()).unwrap()
    } else {
        before
    };
    let (start, len) = find_first_session_meta_line(&original_decoded).unwrap();
    assert_eq!(
        fs::read(backup.join("threads/guardian-prompt-fix-backup/target.jsonl.bak")).unwrap(),
        original_decoded[start..start + len]
    );
}

#[test]
fn actual_repair_entry_restores_plain_thread_from_the_only_healthy_source() {
    assert_repair_entry_restores_a_synthetic_thread(false, false);
}

#[test]
fn actual_repair_entry_preserves_compressed_rollout_format() {
    assert_repair_entry_restores_a_synthetic_thread(true, false);
}

#[test]
fn archived_thread_is_not_used_as_the_instruction_source() {
    assert_repair_entry_restores_a_synthetic_thread(false, true);
}
