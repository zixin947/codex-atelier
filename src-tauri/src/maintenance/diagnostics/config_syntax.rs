//! AiMaMi 1.2.7 config.toml 语法检查与抢救 —— 语义迁移
//!
//! 依据（Ghidra 反编译，见 relay-recovered/maintenance-full/pseudocode/）：
//! - `decomp-check_config_toml_syntax.c` @0x10037b7a8：
//!   空文本+不存在语境 → ok「TOML 语法（文件不存在，跳过）」；
//!   trim 空 → ok「TOML 语法正常（空文件）」；
//!   第 1 层 `toml_edit` parse（内含 parse_document）失败 或 第 2 层
//!   deserialize（结构化为 toml::Value 表，等价 deserialize_any 下游）失败
//!   → error「config.toml TOML 语法错误」（fixable=true）；
//!   双成功 → ok「TOML 语法正常」。
//! - `decomp-fix_config_toml_syntax_text.c` @0x10038c94c：
//!   read → parse OK ⇒ Noop；失败 ⇒ strip_all_managed_blocks → 再 parse
//!   OK ⇒ 原子写剥离文本（文案「清除 managed block 后 TOML 语法恢复正常」）；
//!   仍失败 ⇒ backup 先行 → 226B 双语头 + 全部非空行（各加 `\n`）→ trim_end →
//!   原子写（两处写入都无最终解析闸；写失败 ⇒ CoreError）。
//!
//! 全部解析使用真实 `toml_edit` / `toml`，不用近似解析器。

use super::*;
use std::fs;

pub const RESCUE_HEADER: &str =
    "# Rebuilt by AiMaMi diagnostics · original config.toml was corrupted\n\
# 由 AiMaMi 系统诊断抢救重建 · 原 config.toml 存在无法解析的语法错误\n\
# 原文件已备份到 ~/.codex/codexmate/relay/config-backups/\n";

pub const MSG_STRIP_OK: &str = "清除 managed block 后 TOML 语法恢复正常";
pub const MSG_SALVAGE_OK: &str = "已抢救重建 config.toml：保留可解析的 Codex 配置段";

/// 检查：返回 (label, status, detail, fixable)。
pub fn check_config_toml_syntax(text: &str, exists: bool) -> (String, &'static str, String, bool) {
    if !exists {
        return (
            "TOML 语法（文件不存在，跳过）".into(),
            "ok",
            // 原版 ok_item 只写 id/label，detail 为 None（见 asm/ok_item.asm 的 Option 哨兵）。
            String::new(),
            false,
        );
    }
    if text.trim().is_empty() {
        return (
            "TOML 语法正常（空文件）".into(),
            "ok",
            String::new(),
            false,
        );
    }
    // 第 1 层：toml_edit::parse_document（from_str 内部）
    // 第 2 层：deserialize_any —— 结构化为 toml::Value 表（serde 反序列化）
    match text.parse::<toml_edit::DocumentMut>() {
        Err(e) => (
            "config.toml TOML 语法错误".into(),
            "error",
            format!("{e}"),
            true,
        ),
        Ok(_) => match toml::from_str::<toml::Value>(text) {
            Ok(_) => ("TOML 语法正常".into(), "ok", String::new(), false),
            Err(e) => (
                "config.toml TOML 语法错误".into(),
                "error",
                e.to_string(),
                true,
            ),
        },
    }
}

/// 管理块边界（decomp-strip_all_managed_blocks.c：6 个字面量，长度 30/36/39/55/60/64）
pub fn is_managed_block_marker(trimmed: &str) -> Option<bool> {
    const STARTS: [&str; 3] = [
        "# >>> aimami-relay managed start (DO NOT EDIT MANUALLY)",
        "# >>> aimami-relay managed start (top, DO NOT EDIT MANUALLY)",
        "# >>> aimami-relay codex-router top start (DO NOT EDIT MANUALLY)",
    ];
    const ENDS: [&str; 3] = [
        "# <<< aimami-relay managed end",
        "# <<< aimami-relay managed end (top)",
        "# <<< aimami-relay codex-router top end",
    ];
    if STARTS.contains(&trimmed) {
        Some(true)
    } else if ENDS.contains(&trimmed) {
        Some(false)
    } else {
        None
    }
}

/// strip_all_managed_blocks：多行块内整行保留；标记行与块体行丢弃。
pub fn strip_managed_blocks(text: &str) -> String {
    let mut state = crate::maintenance::diagnostics::toml_lexical::TomlLexicalState::default();
    let mut in_block = false;
    let mut out = String::with_capacity(text.len());
    for raw in text.split_inclusive('\n') {
        let (line, nl) = match raw.strip_suffix('\n') {
            Some(l) => (l, "\n"),
            None => (raw, ""),
        };
        let line = line.strip_suffix('\r').unwrap_or(line);
        if state.line_is_structural(line) {
            out.push_str(line);
            out.push_str(nl);
            continue;
        }
        let t = line.trim();
        match is_managed_block_marker(t) {
            Some(start) => {
                in_block = start;
                continue;
            }
            None if in_block => continue,
            _ => {}
        }
        out.push_str(line);
        out.push_str(nl);
    }
    out
}

/// 修复结果（与反编译 tag 对齐：0xb = Rewritten，2 = CoreError，Noop）
#[derive(Debug)]
pub enum SyntaxFixOutcome {
    Noop,
    Rewritten { text: String, message: &'static str },
    Error(String),
}

/// 注入的文件系统抽象（测试用替身实现）
pub trait SyntaxFs {
    fn read(&self, path: &Path) -> std::result::Result<String, String>;
    fn write_atomic(&self, path: &Path, bytes: &[u8]) -> std::result::Result<(), String>;
    fn backup(&self, path: &Path, content: &[u8]) -> std::result::Result<String, String>;
}

/// fix_config_toml_syntax_text 的注入式实现。
pub fn fix_config_toml_syntax_text<F: SyntaxFs>(fs: &F, path: &Path) -> SyntaxFixOutcome {
    let text = match fs.read(path) {
        Ok(t) => t,
        Err(e) => return SyntaxFixOutcome::Error(e),
    };
    if text.trim().is_empty() {
        return SyntaxFixOutcome::Noop;
    }
    if check_config_toml_syntax(&text, true).1 == "ok" {
        return SyntaxFixOutcome::Noop;
    }
    // 第 1 步：剥离管理块
    let stripped = strip_managed_blocks(&text);
    if check_config_toml_syntax(&stripped, true).1 == "ok" {
        if let Err(e) = fs.backup(path, text.as_bytes()) {
            return SyntaxFixOutcome::Error(e);
        }
        if let Err(e) = fs.write_atomic(path, stripped.as_bytes()) {
            return SyntaxFixOutcome::Error(e);
        }
        return SyntaxFixOutcome::Rewritten {
            text: stripped,
            message: MSG_STRIP_OK,
        };
    }
    // 第 2 步：抢救重建（备份先行；保留全部非空行；两处写均无最终解析闸——反编译证实）
    if let Err(e) = fs.backup(path, text.as_bytes()) {
        return SyntaxFixOutcome::Error(e);
    }
    let mut rebuilt = String::from(RESCUE_HEADER);
    rebuilt.push('\n');
    for raw in text.split('\n') {
        let line = raw.strip_suffix('\r').unwrap_or(raw);
        if !line.trim().is_empty() {
            rebuilt.push_str(line);
            rebuilt.push('\n');
        }
    }
    let rebuilt = rebuilt.trim_end().to_string() + "\n";
    if let Err(e) = fs.write_atomic(path, rebuilt.as_bytes()) {
        return SyntaxFixOutcome::Error(e);
    }
    SyntaxFixOutcome::Rewritten {
        text: rebuilt,
        message: MSG_SALVAGE_OK,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::collections::BTreeMap;
    use std::fs;
    use std::rc::Rc;

    #[derive(Default, Clone)]
    struct MemFs {
        files: BTreeMap<PathBuf, String>,
        written: Rc<RefCell<BTreeMap<PathBuf, String>>>,
        backups: Rc<RefCell<Vec<Vec<u8>>>>,
        readonly: bool,
    }

    impl SyntaxFs for MemFs {
        fn read(&self, path: &Path) -> std::result::Result<String, String> {
            self.files
                .get(path)
                .cloned()
                .ok_or_else(|| "config.toml 无法读取".into())
        }
        fn write_atomic(&self, path: &Path, bytes: &[u8]) -> std::result::Result<(), String> {
            if self.readonly {
                return Err("operation_failed: EACCES".into());
            }
            self.written.borrow_mut().insert(
                path.to_path_buf(),
                String::from_utf8_lossy(bytes).into_owned(),
            );
            Ok(())
        }
        fn backup(&self, _path: &Path, content: &[u8]) -> std::result::Result<String, String> {
            self.backups.borrow_mut().push(content.to_vec());
            Ok("~/.codex/codexmate/relay/config-backups/config.toml.bak".into())
        }
    }

    const CFG: &str = "config.toml";

    #[test]
    fn check_labels_match_decompiled_strings() {
        let (l, s, _, f) = check_config_toml_syntax("", false);
        assert_eq!(
            (l.as_str(), s, f),
            ("TOML 语法（文件不存在，跳过）", "ok", false)
        );

        let (l, s, _, f) = check_config_toml_syntax("  \n\t ", true);
        assert_eq!((l.as_str(), s, f), ("TOML 语法正常（空文件）", "ok", false));

        // 复核者正例：缺值 → 真实 toml_edit 报错
        let (l, s, _, f) = check_config_toml_syntax("model =\n", true);
        assert_eq!(
            (l.as_str(), s, f),
            ("config.toml TOML 语法错误", "error", true)
        );

        // 复核者反例：合法多行数组 → toml_edit/toml 均接受
        let multi = "items = [\n  \"a\",\n  \"b\",\n]\n";
        let (l, s, _, f) = check_config_toml_syntax(multi, true);
        assert_eq!((l.as_str(), s, f), ("TOML 语法正常", "ok", false));

        // 原版 ok_item 不写 detail：三个 ok 分支的详情必须为空，否则 UI 会多出可展开行。
        for (text, exists) in [("", false), ("   ", true), ("model = 'gpt-5'", true)] {
            let (_, _, detail, _) = check_config_toml_syntax(text, exists);
            assert!(detail.is_empty(), "ok 分支不应带详情：{text:?}");
        }
        // 错误分支保留解析器原文，便于用户定位行号。
        let (_, _, detail, _) = check_config_toml_syntax("model =\n", true);
        assert!(!detail.is_empty());

        let (l, s, _, _) = check_config_toml_syntax("model = \"gpt\"\n", true);
        assert_eq!((l.as_str(), s), ("TOML 语法正常", "ok"));
    }

    #[test]
    fn header_is_exactly_226_utf8_bytes() {
        assert_eq!(RESCUE_HEADER.as_bytes().len(), 226);
    }

    #[test]
    fn fix_noop_on_valid() {
        let fs = MemFs {
            files: BTreeMap::from([(PathBuf::from(CFG), "model = \"gpt\"\n".into())]),
            ..Default::default()
        };
        assert!(matches!(
            fix_config_toml_syntax_text(&fs, Path::new(CFG)),
            SyntaxFixOutcome::Noop
        ));
        assert_eq!(fs.written.borrow().len(), 0);
    }

    #[test]
    fn fix_strips_broken_managed_block_with_backup_first() {
        let cfg = "model = \"gpt\"\n\
                   # >>> aimami-relay managed start (DO NOT EDIT MANUALLY)\n\
                   model_providers.aimai1 = [broken\n\
                   # <<< aimami-relay managed end\n";
        let fs = MemFs {
            files: BTreeMap::from([(PathBuf::from(CFG), cfg.into())]),
            ..Default::default()
        };
        match fix_config_toml_syntax_text(&fs, Path::new(CFG)) {
            SyntaxFixOutcome::Rewritten { text, message, .. } => {
                assert_eq!(message, MSG_STRIP_OK);
                assert!(text.contains("model = \"gpt\""));
                assert!(!text.contains("managed start"));
                assert_eq!(fs.backups.borrow().len(), 1);
                assert!(fs.backups.borrow()[0]
                    .windows(32)
                    .any(|w| w == b"model_providers.aimai1 = [broken"));
            }
            other => panic!("expected rewritten, got {other:?}"),
        }
    }

    #[test]
    fn fix_salvage_keeps_nonempty_lines_and_writes() {
        let cfg = "valid_line = 1\n]]] bad [[[\nother = \"ok\"\n";
        let fs = MemFs {
            files: BTreeMap::from([(PathBuf::from(CFG), cfg.into())]),
            ..Default::default()
        };
        match fix_config_toml_syntax_text(&fs, Path::new(CFG)) {
            SyntaxFixOutcome::Rewritten { text, message, .. } => {
                assert_eq!(message, MSG_SALVAGE_OK);
                assert!(text.starts_with(RESCUE_HEADER));
                assert!(text.contains("valid_line = 1"));
                assert!(text.contains("other = \"ok\""));
                assert_eq!(fs.backups.borrow().len(), 1);
                assert_eq!(fs.written.borrow().len(), 1);
            }
            other => panic!("expected rewritten, got {other:?}"),
        }
    }

    #[test]
    fn fix_write_failure_maps_to_error() {
        let fs = MemFs {
            files: BTreeMap::from([(PathBuf::from(CFG), "]]] bad [[[\n".into())]),
            readonly: true,
            ..Default::default()
        };
        match fix_config_toml_syntax_text(&fs, Path::new(CFG)) {
            SyntaxFixOutcome::Error(m) => assert!(m.contains("operation_failed")),
            other => panic!("expected error, got {other:?}"),
        }
    }

    #[test]
    fn salvage_writes_even_if_rebuilt_still_fails_parse() {
        // 反编译：两处写入均无最终解析闸
        let fs = MemFs {
            files: BTreeMap::from([(PathBuf::from(CFG), "open = \"never-closed".into())]),
            ..Default::default()
        };
        match fix_config_toml_syntax_text(&fs, Path::new(CFG)) {
            SyntaxFixOutcome::Rewritten { message, .. } => assert_eq!(message, MSG_SALVAGE_OK),
            other => panic!("expected rewritten, got {other:?}"),
        }
    }

    #[test]
    fn managed_marker_lengths_match_binary() {
        let lens: [usize; 6] = [
            "# <<< aimami-relay managed end".len(),
            "# <<< aimami-relay managed end (top)".len(),
            "# <<< aimami-relay codex-router top end".len(),
            "# >>> aimami-relay managed start (DO NOT EDIT MANUALLY)".len(),
            "# >>> aimami-relay managed start (top, DO NOT EDIT MANUALLY)".len(),
            "# >>> aimami-relay codex-router top start (DO NOT EDIT MANUALLY)".len(),
        ];
        assert_eq!(lens, [30, 36, 39, 55, 60, 64]);
        for s in [
            "# >>> aimami-relay managed start (DO NOT EDIT MANUALLY)",
            "# >>> aimami-relay managed start (top, DO NOT EDIT MANUALLY)",
            "# >>> aimami-relay codex-router top start (DO NOT EDIT MANUALLY)",
        ] {
            assert_eq!(is_managed_block_marker(s), Some(true));
        }
        for s in [
            "# <<< aimami-relay managed end",
            "# <<< aimami-relay managed end (top)",
            "# <<< aimami-relay codex-router top end",
        ] {
            assert_eq!(is_managed_block_marker(s), Some(false));
        }
        assert_eq!(is_managed_block_marker("# plain comment"), None);
    }
}

/// 抢救计划（预览纯函数）：返回实际将要写入的 after 文本与文案。
/// 不得有任何文件副作用。
pub enum RescuePlan {
    None,
    Plan {
        after: Vec<u8>,
        message: &'static str,
    },
}

pub fn plan_config_toml_syntax_rescue(original: &[u8]) -> RescuePlan {
    let text = String::from_utf8_lossy(original).into_owned();
    if text.trim().is_empty() {
        return RescuePlan::None;
    }
    if check_config_toml_syntax(&text, true).1 == "ok" {
        return RescuePlan::None;
    }
    // 第 1 步：剥离管理块
    let stripped = strip_managed_blocks(&text);
    if check_config_toml_syntax(&stripped, true).1 == "ok" {
        return RescuePlan::Plan {
            after: stripped.into_bytes(),
            message: MSG_STRIP_OK,
        };
    }
    // 第 2 步：抢救重建
    let mut rebuilt = String::from(RESCUE_HEADER);
    rebuilt.push('\n');
    for raw in text.split('\n') {
        let line = raw.strip_suffix('\r').unwrap_or(raw);
        if !line.trim().is_empty() {
            rebuilt.push_str(line);
            rebuilt.push('\n');
        }
    }
    let rebuilt = rebuilt.trim_end().to_string() + "\n";
    RescuePlan::Plan {
        after: rebuilt.into_bytes(),
        message: MSG_SALVAGE_OK,
    }
}

/// 确认（apply）阶段：按预览计划的 after 逐字备份 + 原子写 + 复检。
/// `expected_before` 用于外部变更校验；`backup_dir`/`atomic` 由调用方注入。
pub fn apply_config_toml_syntax_rescue<F: SyntaxFs>(
    fs: &F,
    path: &Path,
    expected_before: &[u8],
    planned_after: &[u8],
) -> std::result::Result<&'static str, String> {
    // 外部变更校验：当前文件必须与预览时的原文逐字相同
    let current = fs.read(path)?;
    if current.as_bytes() != expected_before {
        return Err("配置已被外部更新，请重新预览；未修改文件".into());
    }
    // 备份原件（逐字）
    fs.backup(path, expected_before)?;
    // 原子写计划文本
    fs.write_atomic(path, planned_after)?;
    // 复检：真实反映修复/仍异常
    let after_text = String::from_utf8_lossy(planned_after).into_owned();
    match check_config_toml_syntax(&after_text, true).1 {
        "ok" => Ok(MSG_SALVAGE_OK),
        _ => Ok("已写入，但复检显示 TOML 仍异常：请查看 config_toml_syntax 项"),
    }
}

/// 真实文件系统适配器（apply 用）：备份逐字 + 原子私有写。
pub struct RealSyntaxFs {
    pub backup_dir: PathBuf,
}

impl SyntaxFs for RealSyntaxFs {
    fn read(&self, path: &Path) -> std::result::Result<String, String> {
        fs::read(path)
            .map_err(|e| format!("config.toml 无法读取：{e}"))
            .and_then(|b| String::from_utf8(b).map_err(|e| format!("配置不是 UTF-8：{e}")))
    }
    fn write_atomic(&self, path: &Path, bytes: &[u8]) -> std::result::Result<(), String> {
        // 原子写：写临时文件 → fsync → rename（同目录保证原子性）
        let dir = path.parent().unwrap_or(Path::new("."));
        crate::gateway::storage::private_dir(dir).map_err(|e| format!("创建目录失败：{e}"))?;
        let tmp = dir.join(format!(".config.toml.{}.tmp", std::process::id()));
        {
            use std::io::Write;
            let mut f = fs::File::create(&tmp).map_err(|e| format!("operation_failed: {e}"))?;
            f.write_all(bytes)
                .map_err(|e| format!("operation_failed: {e}"))?;
            f.sync_all().map_err(|e| format!("operation_failed: {e}"))?;
        }
        fs::rename(&tmp, path).map_err(|e| {
            let _ = fs::remove_file(&tmp);
            format!("operation_failed: {e}")
        })
    }
    fn backup(&self, path: &Path, content: &[u8]) -> std::result::Result<String, String> {
        crate::gateway::storage::private_dir(&self.backup_dir)
            .map_err(|e| format!("创建备份目录失败：{e}"))?;
        let stamp = crate::gateway::types::now_ms();
        let dest = self.backup_dir.join(format!("config.toml.{stamp}.bak"));
        fs::write(&dest, content).map_err(|e| format!("备份写入失败：{e}"))?;
        Ok(dest.to_string_lossy().into_owned())
    }
}
