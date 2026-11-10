//! 审批提示词污染恢复（thread_poisoned_instructions 修复）
//!
//! 依据：decomp-fix_thread_poisoned_instructions.c @0x10039573c 区（1591 行）+ 字符串池：
//! · 检查：session_meta_instruction_issue(@0x100393c6c) 判 /base_instructions/text：
//!   审批裁决前缀（"You are judging one planned coding-agent action"）→ 污染；
//!   空/缺失 → 缺失。两者都是 issue。
//! · 修复（decomp 证实流程）：
//!   1. collect_diagnostic_main_threads 收集候选主线程
//!   2. 先备份原 rollout 文件（`<name>.jsonl.bak`，写入前创建目录）
//!   3. 模板来源 = 正常线程的 base_instructions.text（serde_json::Value 缓存，
//!      BTreeMap 按序选择健康线程）
//!   4. 找不到完整模板 → 使用最小安全提示（字符串池逐字）：
//!      "Follow the user's instructions carefully. Respond helpfully and use tools when appropriate."
//!   5. rewrite_session_meta_base_instructions（tag 0xb 成功）→
//!      replace_first_session_meta_line 写回 rollout 首部 session_meta 行
//!   6. 复检 session_meta_instruction_issue；部分线程失败时
//!      「该线程 rollout，原文件和数据库记录均未改动，其余线程已正常收敛」
//!
//! Atelier 迁移：检查已有（inspect.rs），修复此前缺失——本模块补齐。

use super::*;
use serde_json::{json, Value};

/// 最小安全提示（AiMaMi 字符串池逐字，两处出现：relay-state-cache / no-account restore）
pub const MINIMAL_SAFE_INSTRUCTIONS: &str =
    "Follow the user's instructions carefully. Respond helpfully and use tools when appropriate.";

/// 审批裁决污染前缀（与 inspect.rs 的检查一致）
pub const APPROVAL_JUDGE_PREFIX: &str = "You are judging one planned coding-agent action";

#[derive(Debug, Clone, PartialEq)]
pub enum InstructionIssue {
    Polluted,
    MissingOrEmpty,
    Healthy,
}

/// session_meta_instruction_issue 对齐：检查 base_instructions/text
pub fn instruction_issue(meta_payload: &Value) -> InstructionIssue {
    let text = meta_payload
        .pointer("/base_instructions/text")
        .and_then(Value::as_str)
        .unwrap_or("");
    let trimmed = text.trim();
    if trimmed.is_empty() {
        return InstructionIssue::MissingOrEmpty;
    }
    if trimmed.starts_with(APPROVAL_JUDGE_PREFIX) {
        return InstructionIssue::Polluted;
    }
    InstructionIssue::Healthy
}

/// 从 rollout 字节中定位首条 session_meta 行（decomp: find_first_session_meta_line）
pub fn find_first_session_meta_line(bytes: &[u8]) -> Option<(usize, usize)> {
    let mut offset = 0;
    for line in bytes.split_inclusive(|b| *b == b'\n') {
        let body = line.strip_suffix(b"\n").unwrap_or(line);
        let body = body.strip_suffix(b"\r").unwrap_or(body);
        if !body.is_empty() {
            if let Ok(v) = serde_json::from_slice::<Value>(body) {
                if v["type"] == "session_meta" {
                    return Some((offset, body.len()));
                }
            }
        }
        offset += line.len();
    }
    None
}

/// rewrite_session_meta_base_instructions（decomp：改 payload 内 text，保持其余结构）
/// 成功返回重写后的整行 JSON 字节（tag 0xb 对应 Ok）。
pub fn rewrite_session_meta_base_instructions(
    meta_line: &[u8],
    new_instructions: &str,
) -> Result<Vec<u8>> {
    let mut v: Value =
        serde_json::from_slice(meta_line).map_err(|e| format!("session_meta 解析失败：{e}"))?;
    // 结构路径：payload.base_instructions.text；缺失则补齐对象
    let payload = v.get_mut("payload").ok_or("session_meta 缺 payload")?;
    if payload.get("base_instructions").is_none() {
        payload["base_instructions"] = json!({});
    }
    let bi = payload.get_mut("base_instructions").unwrap();
    if bi.get("text").is_none() {
        bi["text"] = json!(new_instructions);
    } else {
        bi["text"] = json!(new_instructions);
    }
    serde_json::to_vec(&v).map_err(|e| format!("session_meta 序列化失败：{e}"))
}

/// replace_first_session_meta_line：把 rollout 字节的首条 session_meta 行替换为重写行
pub fn replace_first_session_meta_line(bytes: &[u8], rewritten: &[u8]) -> Result<Vec<u8>> {
    let (start, len) = find_first_session_meta_line(bytes).ok_or("记录中未找到 session_meta")?;
    let end = start + len;
    // 反编译：replace 保持原行的行尾（\n / \r\n / 无），不改后续字节
    let mut line_end = end;
    let mut term: &[u8] = b"";
    if bytes.get(line_end) == Some(&b'\r') {
        line_end += 1;
        term = b"\r";
    }
    if bytes.get(line_end) == Some(&b'\n') {
        line_end += 1;
        if term.is_empty() {
            term = b"\n";
        } else {
            term = b"\r\n";
        }
    }
    let mut rewritten = rewritten.to_vec();
    if let Some(pos) = rewritten.iter().rposition(|b| *b == b'\n') {
        // rewrite 输出带尾 \n 时，替换为原行的行尾
        rewritten.truncate(pos);
    }
    let mut out = Vec::with_capacity(bytes.len() + rewritten.len());
    out.extend_from_slice(&bytes[..start]);
    out.extend_from_slice(&rewritten);
    out.extend_from_slice(term);
    out.extend_from_slice(&bytes[line_end..]);
    Ok(out)
}

/// 模板来源证据（decomp-collect_diagnostic_main_threads.c @0x100393ea4 +
/// decomp-fix_thread_poisoned_instructions.c @0x10039573c 联合裁定）：
/// · 收集 SQL：`SELECT DISTINCT rollout_path FROM threads WHERE rollout_path IS NOT NULL
///   AND rollout_path <> '' AND COALESCE(archived, 0) = 0`（无 ORDER BY → SQLite 自然序；
///   排除归档——归档线程不作为模板来源）
/// · 对每个 rollout 读 `/payload/base_instructions/text`，trim 非空 → 候选
/// · fix 侧（decomp 行 296-348）：BTreeMap::entry(key) →
///   Occupied（已有缓存模板）→ 用缓存；Vacant → insert_entry(MINIMAL_SAFE)，
///   随后 healthy 候选覆盖。即「healthy 优先，minimal 兜底」，且模板按 key 缓存。
/// ★未解证据项（不标完成）：
///   1. 缓存 key 的确切含义（thread id? cwd? 项目分组?）——fix 侧 local_1178 构造
///      未跟踪到源头，BTreeMap 按 key 排序可能使「多健康来源时的最终选择」≠ 全局
///      first healthy。
///   2. 多健康来源的最终选择顺序：需要拿到 key 构造分支与迭代消费分支才能裁定。
///   3. 当前 Atelier 实现 = 单模板缓存 + scan 顺序首个健康来源，仅在「单一健康来源」
///      样本下与上游等价；多来源样本待补（见登记）。
/// · 最小安全提示 = 91 字节 "Follow the user's instructions carefully. …"
///   （decomp 行 296-306 三段 8 字节常量拼出 0x5b=91，与字符串池 0x10130cdac 一致）
/// · 与 Atelier scan_threads 的差异：上游专用 collector 排除归档、无 LIMIT 1001、
///   无 ORDER BY id。Atelier 侧已改用专用查询（见 repair.rs collect_template_candidates）。

/// 修复执行（注入 fs 以隔离测试）。
/// `threads`: 待修复的 (rollout_path, 原始字节)；
/// `template`: 健康线程的 instructions（None = 用最小安全提示）。
/// 返回逐文件结果：(path, 备份路径, 修复后字节)；失败项原样返回并带原因。
pub struct PoisonedRepair<'a, F: RolloutFs> {
    pub fs: &'a F,
    pub template: Option<String>,
}

pub trait RolloutFs {
    fn create_dir_all(&self, dir: &Path) -> std::result::Result<(), String>;
    fn write_backup(&self, backup_path: &Path, bytes: &[u8]) -> std::result::Result<(), String>;
    fn write_atomic(&self, path: &Path, bytes: &[u8]) -> std::result::Result<(), String>;
}

pub struct PoisonedRepairOutcome {
    pub repaired: Vec<(PathBuf, PathBuf)>,
    pub skipped: Vec<(PathBuf, String)>,
    pub used_minimal: bool,
}

impl<'a, F: RolloutFs> PoisonedRepair<'a, F> {
    pub fn run(&self, threads: &[(PathBuf, Vec<u8>)]) -> Result<PoisonedRepairOutcome> {
        let mut repaired = vec![];
        let mut skipped = vec![];
        let mut used_minimal = false;
        let template = match &self.template {
            Some(t) if !t.trim().is_empty() && !t.trim().starts_with(APPROVAL_JUDGE_PREFIX) => {
                t.clone()
            }
            _ => {
                used_minimal = true;
                MINIMAL_SAFE_INSTRUCTIONS.to_string()
            }
        };
        for (path, bytes) in threads {
            // 定位 session_meta
            let (start, len) = match find_first_session_meta_line(bytes) {
                Some(x) => x,
                None => {
                    skipped.push((path.clone(), "记录中未找到 session_meta".into()));
                    continue;
                }
            };
            let meta_line = &bytes[start..start + len];
            let payload: Value = match serde_json::from_slice::<Value>(meta_line) {
                Ok(v) => v["payload"].clone(),
                Err(e) => {
                    skipped.push((path.clone(), format!("session_meta 解析失败：{e}")));
                    continue;
                }
            };
            match instruction_issue(&payload) {
                InstructionIssue::Healthy => continue,
                issue => {
                    // 备份先行（decomp：先建目录 → 写 .jsonl.bak → 再重写）
                    let backup = path.with_extension("jsonl.bak");
                    if let Some(dir) = backup.parent() {
                        if let Err(e) = self.fs.create_dir_all(dir) {
                            skipped.push((path.clone(), format!("备份目录创建失败：{e}")));
                            continue;
                        }
                    }
                    if let Err(e) = self.fs.write_backup(&backup, bytes) {
                        skipped.push((path.clone(), format!("备份写入失败：{e}")));
                        continue;
                    }
                    // 压缩文件（.zst）先解码再重写再编码——由上层完成解码，这里按字节处理
                    let rewritten_line =
                        match rewrite_session_meta_base_instructions(meta_line, &template) {
                            Ok(b) => b,
                            Err(e) => {
                                skipped.push((path.clone(), e));
                                continue;
                            }
                        };
                    let after = match replace_first_session_meta_line(bytes, &rewritten_line) {
                        Ok(b) => b,
                        Err(e) => {
                            skipped.push((path.clone(), e));
                            continue;
                        }
                    };
                    // 复检
                    let (s2, l2) = find_first_session_meta_line(&after)
                        .ok_or_else(|| "重写后 session_meta 丢失".to_string())?;
                    let p2: Value = serde_json::from_slice(&after[s2..s2 + l2])
                        .map_err(|e| format!("重写后解析失败：{e}"))?;
                    if instruction_issue(&p2["payload"]) != InstructionIssue::Healthy {
                        skipped.push((
                            path.clone(),
                            "重写后复检仍异常；原文件和数据库记录均未改动".into(),
                        ));
                        continue;
                    }
                    if let Err(e) = self.fs.write_atomic(path, &after) {
                        skipped.push((path.clone(), format!("写入失败：{e}")));
                        continue;
                    }
                    repaired.push((path.clone(), backup));
                }
            }
        }
        Ok(PoisonedRepairOutcome {
            repaired,
            skipped,
            used_minimal,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::{json, Value};
    use std::cell::RefCell;
    use std::collections::BTreeMap;

    #[derive(Default)]
    struct MemRolloutFs {
        files: RefCell<BTreeMap<PathBuf, Vec<u8>>>,
        backups: RefCell<Vec<(PathBuf, Vec<u8>)>>,
        dirs: RefCell<Vec<PathBuf>>,
    }

    impl RolloutFs for MemRolloutFs {
        fn create_dir_all(&self, dir: &Path) -> std::result::Result<(), String> {
            self.dirs.borrow_mut().push(dir.to_path_buf());
            Ok(())
        }
        fn write_backup(&self, p: &Path, bytes: &[u8]) -> std::result::Result<(), String> {
            self.backups
                .borrow_mut()
                .push((p.to_path_buf(), bytes.to_vec()));
            Ok(())
        }
        fn write_atomic(&self, p: &Path, bytes: &[u8]) -> std::result::Result<(), String> {
            self.files
                .borrow_mut()
                .insert(p.to_path_buf(), bytes.to_vec());
            Ok(())
        }
    }

    fn meta_json(instructions: &str) -> String {
        format!(
            r#"{{"type":"session_meta","payload":{{"id":"t1","model_provider":"openai","base_instructions":{{"text":{}}}}}}}"#,
            serde_json::to_string(instructions).unwrap()
        )
    }

    fn rollout_bytes(instructions: &str) -> Vec<u8> {
        let mut b = meta_json(instructions).into_bytes();
        b.extend_from_slice(b"\n");
        b.extend_from_slice(br#"{"type":"response_item","payload":{}}"#);
        b
    }

    #[test]
    fn issue_classifier_matches_decompiled_check() {
        let judge = json!({"base_instructions":{"text":"You are judging one planned coding-agent action. Return verdict."}});
        assert_eq!(instruction_issue(&judge), InstructionIssue::Polluted);
        let empty = json!({"base_instructions":{"text":"   "}});
        assert_eq!(instruction_issue(&empty), InstructionIssue::MissingOrEmpty);
        let absent = json!({});
        assert_eq!(instruction_issue(&absent), InstructionIssue::MissingOrEmpty);
        let healthy = json!({"base_instructions":{"text":"You are a coding assistant."}});
        assert_eq!(instruction_issue(&healthy), InstructionIssue::Healthy);
    }

    #[test]
    fn find_first_session_meta_line_locates_first_line() {
        let bytes = rollout_bytes("x");
        let (s, l) = find_first_session_meta_line(&bytes).unwrap();
        assert_eq!(s, 0);
        assert_eq!(&bytes[s..s + l], meta_json("x").as_bytes());
    }

    #[test]
    fn repair_restores_from_healthy_template_with_backup() {
        let fs = MemRolloutFs::default();
        let path = PathBuf::from("/tmp-test/t1.jsonl");
        let bytes = rollout_bytes("You are judging one planned coding-agent action. X");
        let repair = PoisonedRepair {
            fs: &fs,
            template: Some("You are a coding assistant. Be helpful.".into()),
        };
        let out = repair.run(&[(path.clone(), bytes.clone())]).unwrap();
        assert_eq!(out.repaired.len(), 1);
        assert!(!out.used_minimal);
        // 备份逐字
        assert_eq!(fs.backups.borrow()[0].1, bytes);
        assert!(fs.backups.borrow()[0].0.ends_with("t1.jsonl.bak"));
        // 写回内容：污染行被替换，response_item 保留
        let after = &fs.files.borrow()[&path];
        let text = String::from_utf8_lossy(after);
        assert!(text.contains("You are a coding assistant."));
        assert!(!text.contains("You are judging"));
        assert!(text.contains("response_item"));
        // 复检健康
        let (s, l) = find_first_session_meta_line(after).unwrap();
        let p: Value = serde_json::from_slice(&after[s..s + l]).unwrap();
        assert_eq!(instruction_issue(&p["payload"]), InstructionIssue::Healthy);
    }

    #[test]
    fn repair_falls_back_to_minimal_safe_prompt_without_template() {
        let fs = MemRolloutFs::default();
        let path = PathBuf::from("/tmp-test/t2.jsonl");
        let bytes = rollout_bytes("");
        let repair = PoisonedRepair {
            fs: &fs,
            template: None,
        };
        let out = repair.run(&[(path.clone(), bytes)]).unwrap();
        assert_eq!(out.repaired.len(), 1);
        assert!(out.used_minimal);
        let after = &fs.files.borrow()[&path];
        let text = String::from_utf8_lossy(after);
        assert!(text.contains(MINIMAL_SAFE_INSTRUCTIONS));
    }

    #[test]
    fn repair_skips_healthy_threads_without_writes() {
        let fs = MemRolloutFs::default();
        let path = PathBuf::from("/tmp-test/t3.jsonl");
        let bytes = rollout_bytes("healthy prompt");
        let repair = PoisonedRepair {
            fs: &fs,
            template: None,
        };
        let out = repair.run(&[(path.clone(), bytes.clone())]).unwrap();
        assert_eq!(out.repaired.len(), 0);
        assert_eq!(out.skipped.len(), 0);
        assert!(fs.files.borrow().get(&path).is_none());
        assert!(fs.backups.borrow().is_empty());
    }

    #[test]
    fn repair_failure_leaves_file_untouched_and_reports() {
        // 缺 session_meta 的记录 → skip，原字节不动
        let fs = MemRolloutFs::default();
        let path = PathBuf::from("/tmp-test/t4.jsonl");
        let bytes = b"{\"type\":\"other\"}\n".to_vec();
        let repair = PoisonedRepair {
            fs: &fs,
            template: None,
        };
        let out = repair.run(&[(path.clone(), bytes.clone())]).unwrap();
        assert_eq!(out.skipped.len(), 1);
        assert!(out.skipped[0].1.contains("session_meta"));
        assert!(fs.files.borrow().get(&path).is_none());
    }

    #[test]
    fn replace_preserves_trailing_records() {
        let meta = meta_json("a");
        let mut bytes = meta.clone().into_bytes();
        bytes.push(b'\n');
        bytes.extend_from_slice(b"line2\nline3");
        let rewritten = rewrite_session_meta_base_instructions(meta.as_bytes(), "b").unwrap();
        let after = replace_first_session_meta_line(&bytes, &rewritten).unwrap();
        let text = String::from_utf8(after).unwrap();
        assert!(text.starts_with('{'));
        assert!(text.contains("line2\nline3"));
    }
}
