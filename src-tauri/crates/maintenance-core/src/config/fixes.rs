//! 静态恢复的纯文本修复。只生成可审阅的FileChange，不发布文件，不触碰Manager。
use super::*;
use std::path::{Path, PathBuf};
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RemainingFixStep {
    ManagerReconcileThenRereadAndCheck,
    PublishThenRereadAndCheck,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FileChange {
    pub path: PathBuf,
    pub before: String,
    pub after: String,
    pub removed_lines: usize,
    pub remaining_step: RemainingFixStep,
}
impl FileChange {
    pub fn changed(&self) -> bool {
        self.before != self.after
    }
}
/// fix_config_global_validity @100387c84；只移除明确失败的顶层行，逐字保留其余内容和行尾。
/// 此函数接受真实metadata观察接口，删除判断由恢复的invalid_global_config_reference执行。
pub fn fix_config_global_validity_text(
    config: &str,
    codex_dir: &Path,
    fs: &impl ConfigFs,
) -> Result<FileChange, RecoveryError> {
    fix_config_global_validity_at(config, &codex_dir.join("config.toml"), codex_dir, fs)
}
/// Actual output and the managed-reference exclusion directory are separate paths.
pub fn fix_config_global_validity_at(
    config: &str,
    config_path: &Path,
    managed_root: &Path,
    fs: &impl ConfigFs,
) -> Result<FileChange, RecoveryError> {
    let mut after = String::with_capacity(config.len());
    let mut removed_lines = 0;
    let mut lex = Lexical(2);
    let mut managed = false;
    let mut section = false;
    for raw in config.split_inclusive('\n') {
        let body = raw.trim_end_matches(['\r', '\n']);
        let line = body.trim();
        let structural = lex.line_is_structural(body);
        let mut remove = false;
        if structural {
            if BEGIN.contains(&line) {
                managed = true;
            } else if END.contains(&line) || managed {
                managed = !END.contains(&line) && managed;
            } else if line.starts_with('[') || section {
                section = true;
                managed = false;
            } else {
                remove = invalid_global_config_reference(line, managed_root, fs)?.is_some();
                section = false;
                managed = false;
            }
        }
        if remove {
            removed_lines += 1;
        } else {
            after.push_str(raw);
        }
    }
    Ok(FileChange {
        path: config_path.to_owned(),
        before: config.into(),
        after,
        removed_lines,
        remaining_step: RemainingFixStep::PublishThenRereadAndCheck,
    })
}
#[cfg(test)]
mod tests {
    use super::*;
    struct Missing;
    impl ConfigFs for Missing {
        fn metadata(&self, _: &Path) -> std::io::Result<()> {
            Err(std::io::ErrorKind::NotFound.into())
        }
    }
    #[test]
    fn global_fix_deletes_real_invalid_lines_preserving_other_bytes() {
        let input="# before\r\nmodel_reasoning_effort=''\r\nmodel_catalog_json='/missing.json'\nmodel='gpt-5'";
        let c = fix_config_global_validity_text(input, Path::new("/synthetic"), &Missing).unwrap();
        assert_eq!(c.after, "# before\r\nmodel='gpt-5'");
        assert_eq!(c.removed_lines, 2);
        assert!(c.changed());
        assert_eq!(
            check_config_global_validity(Some(&c.after), Path::new("/synthetic"), &Missing)
                .unwrap()
                .status,
            Status::Ok
        );
    }
    #[test]
    fn global_fix_keeps_managed_multiline_section_and_unknown_semantics() {
        let input=format!("{}\nmodel_reasoning_effort=''\n{}\nmodel_reasoning_effort='mystery'\ntext='''\nmodel_reasoning_effort=''\n'''\n[profile]\nmodel_catalog_json='/missing.json'\n",BEGIN[0],END[0]);
        let c = fix_config_global_validity_text(&input, Path::new("/synthetic"), &Missing).unwrap();
        assert_eq!(c.after, input);
        assert!(!c.changed());
    }
    #[test]
    fn global_fix_permission_and_relative_catalog_are_preserved() {
        struct Denied;
        impl ConfigFs for Denied {
            fn metadata(&self, _: &Path) -> std::io::Result<()> {
                Err(std::io::ErrorKind::PermissionDenied.into())
            }
        }
        let s = "model_catalog_json='/missing.json'\nmodel_reasoning_effort=' '";
        assert!(
            !fix_config_global_validity_text(s, Path::new("/synthetic"), &Denied)
                .unwrap()
                .changed()
        );
        assert!(!fix_config_global_validity_text(
            "model_catalog_json='relative.json'",
            Path::new("/synthetic"),
            &Missing
        )
        .unwrap()
        .changed());
    }
}
/// is_profile_assignment @100376d60：只有prefix+字符边界，没有要求实际等号（原生宽松行为）。
pub fn is_profile_assignment(line: &str) -> bool {
    line.strip_prefix("profile").is_some_and(|rest| {
        rest.is_empty()
            || rest
                .chars()
                .next()
                .is_some_and(|c| c == '=' || c.is_whitespace())
    })
}
/// fix_config_profile_conflict_text.c @100397734。保留managed块内容，删块外顶层profile；输出逐行LF。
pub fn fix_config_profile_conflict_text(config: &str, codex_dir: &Path) -> FileChange {
    let mut lex = Lexical(2);
    let mut managed = false;
    let mut section = false;
    let mut removed_lines = 0;
    let mut after = String::new();
    for raw in config.lines() {
        let line = raw.trim();
        let structural = lex.line_is_structural(raw);
        let mut remove = false;
        if structural {
            if BEGIN.contains(&line) {
                managed = true;
            } else if END.contains(&line) {
                managed = false;
            } else if !managed {
                if line.starts_with('[') || section {
                    section = true;
                } else {
                    remove = is_profile_assignment(line);
                    section = false;
                }
            }
        }
        if remove {
            removed_lines += 1;
        } else {
            after.push_str(raw);
            after.push('\n');
        }
    }
    FileChange {
        path: codex_dir.join("config.toml"),
        before: config.into(),
        after,
        removed_lines,
        remaining_step: RemainingFixStep::ManagerReconcileThenRereadAndCheck,
    }
}
/// split_toml_section_header.c @10078b924；双引号与quoted内escape，单引号不特殊；保留空组件。
pub fn split_toml_section_header(header: &str) -> Vec<String> {
    let mut result = vec![];
    let mut part = String::new();
    let mut quoted = false;
    let mut escaped = false;
    for c in header.chars() {
        if escaped {
            part.push(c);
            escaped = false;
            continue;
        }
        match c {
            '"' => quoted = !quoted,
            '\\' if quoted => escaped = true,
            '.' if !quoted => {
                result.push(part.trim().to_owned());
                part.clear();
            }
            _ => part.push(c),
        }
    }
    result.push(part.trim().to_owned());
    result
}
fn owned_section_name(inner: &str) -> bool {
    inner
        .strip_prefix("model_providers.")
        .or_else(|| inner.strip_prefix("profiles."))
        .is_some_and(|id| {
            id == "aimami_relay"
                || id.starts_with("aimami_relay_")
                || id == "aimai1"
                || id == "aimami_codex_router"
        })
}
fn stale_generated_comment(line: &str) -> bool {
    (line.starts_with("# CLI:") && line.contains("aimami_relay"))
        || [
            "# Generated by AiMaMi",
            "# 修改请通过 AiMaMi 应用页面",
            "# Codex 智能路由",
            "# 保持 openai 为 active provider",
            "# 在 AiMaMi 里关闭智能路由",
            "# 中转模型功能",
            "# Codex 桌面 GUI 智能路由已启用",
        ]
        .iter()
        .any(|s| line.starts_with(s))
}
/// strip_all_managed_blocks.c @100788e70；按原生三遍执行，保留managed块中不与用户section重叠的非owned section。
pub fn strip_all_managed_blocks(config: &str) -> String {
    let mut outside = vec![];
    let mut lex = Lexical(2);
    let mut managed = false;
    for raw in config.lines() {
        if !lex.line_is_structural(raw) {
            continue;
        }
        let line = raw.trim();
        if BEGIN.contains(&line) {
            managed = true;
            continue;
        }
        if END.contains(&line) {
            managed = false;
            continue;
        }
        if !managed {
            if let Some(inner) = line.strip_prefix('[').and_then(|s| s.strip_suffix(']')) {
                outside.push(split_toml_section_header(inner));
            }
        }
    }
    let mut middle = String::new();
    let mut lex = Lexical(2);
    let mut managed = false;
    let mut skip = true;
    for raw in config.lines() {
        let line = raw.trim();
        let structural = lex.line_is_structural(raw);
        if structural && BEGIN.contains(&line) {
            managed = true;
            skip = true;
            continue;
        }
        if structural && END.contains(&line) {
            managed = false;
            skip = false;
            continue;
        }
        if managed {
            if structural {
                if let Some(inner) = line.strip_prefix('[').and_then(|s| s.strip_suffix(']')) {
                    let parts = split_toml_section_header(inner);
                    let overlap = outside
                        .iter()
                        .any(|user| parts.starts_with(user) || user.starts_with(&parts));
                    skip = owned_section_name(inner) || overlap;
                }
            }
            if skip {
                continue;
            }
        }
        middle.push_str(raw);
        middle.push('\n');
    }
    let mut after = String::new();
    let mut lex = Lexical(2);
    let mut skip = false;
    for raw in middle.lines() {
        let line = raw.trim();
        let structural = lex.line_is_structural(raw);
        if !structural {
            if skip {
                continue;
            }
        } else if let Some(inner) = line.strip_prefix('[').and_then(|s| s.strip_suffix(']')) {
            if owned_section_name(inner) {
                skip = true;
                continue;
            }
        } else if skip {
            skip = !line.is_empty();
            continue;
        }
        if structural && stale_generated_comment(line) {
            skip = false;
            continue;
        }
        after.push_str(raw);
        after.push('\n');
        skip = false;
    }
    while after.ends_with("\n\n") {
        after.pop();
    }
    if after == "\n" {
        after.clear();
    }
    after
}
/// fix_config_stale_text.c @10037659c：只剥离，不自行重建路由；后续Manager重协调是原生必经步骤。
pub fn fix_config_stale_text(config: &str, codex_dir: &Path) -> FileChange {
    let after = strip_all_managed_blocks(config);
    let removed_lines = config.lines().count().saturating_sub(after.lines().count());
    FileChange {
        path: codex_dir.join("config.toml"),
        before: config.into(),
        after,
        removed_lines,
        remaining_step: RemainingFixStep::ManagerReconcileThenRereadAndCheck,
    }
}
#[cfg(test)]
mod profile_stale_tests {
    use super::*;
    #[test]
    fn profile_fix_removes_all_top_level_including_native_malformed_prefix_and_normalizes_lf() {
        let c = fix_config_profile_conflict_text(
            "profile='one'\r\nprofile\nprofile invalid\nprofiles='keep'",
            Path::new("/s"),
        );
        assert_eq!(c.after, "profiles='keep'\n");
        assert_eq!(c.removed_lines, 3);
        assert_eq!(
            c.remaining_step,
            RemainingFixStep::ManagerReconcileThenRereadAndCheck
        );
    }
    #[test]
    fn profile_fix_preserves_managed_and_nested_and_multiline_content() {
        let input=format!("{}\nprofile='managed'\n{}\ntext='''\nprofile='fake'\n'''\nprofile='remove'\n[profiles.user]\nprofile='nested'",BEGIN[0],END[0]);
        let c = fix_config_profile_conflict_text(&input, Path::new("/s"));
        assert_eq!(c.removed_lines, 1);
        assert!(c.after.contains("profile='managed'"));
        assert!(c.after.contains("profile='fake'"));
        assert!(c.after.contains("profile='nested'"));
    }
    #[test]
    fn split_section_drops_escaped_slash_but_keeps_literal_dot() {
        assert_eq!(
            split_toml_section_header(r#"profiles."with.dot"."escaped\"quote""#),
            vec!["profiles", "with.dot", "escaped\"quote"]
        );
        assert_eq!(split_toml_section_header(" a..b. "), vec!["a", "", "b", ""]);
    }
    #[test]
    fn stale_strip_removes_owned_blocks_but_preserves_unrelated_managed_sections() {
        let text=format!("keep=1\n{}\nmodel='injected'\n[model_providers.aimai1]\nbase_url='internal'\n[custom.extra]\nkept=2\n{}\n[user]\nkept=3\n",BEGIN[0],END[0]);
        assert_eq!(
            strip_all_managed_blocks(&text),
            "keep=1\n[custom.extra]\nkept=2\n[user]\nkept=3\n"
        );
    }
    #[test]
    fn stale_strip_protects_outside_section_from_ancestor_descendant_duplicate_inside_managed_block(
    ) {
        let text=format!("[custom.user]\nreal=1\n{}\n[custom]\ninjected=1\n[custom.user.child]\ninjected=2\n[unrelated]\nkept=4\n{}\n",BEGIN[0],END[0]);
        assert_eq!(
            strip_all_managed_blocks(&text),
            "[custom.user]\nreal=1\n[unrelated]\nkept=4\n"
        );
    }
    #[test]
    fn stale_strip_preserves_literal_markers_and_native_quoted_legacy_quirk() {
        let input = format!(
            "text='''\n{}\nunchanged\n{}\n'''\n[model_providers.\"aimai1\"]\nkept=1\n",
            BEGIN[0], END[0]
        );
        assert_eq!(strip_all_managed_blocks(&input), input);
    }
    #[test]
    fn stale_strip_generated_comments_and_final_newlines() {
        let input="# Generated by AiMaMi custom\r\n# CLI: aimami_relay\n# 中转模型功能\n# human comment\nkeep=1\n\n\n";
        assert_eq!(strip_all_managed_blocks(input), "# human comment\nkeep=1\n");
        assert_eq!(strip_all_managed_blocks("\n\n"), "");
        assert_eq!(strip_all_managed_blocks("keep=1"), "keep=1\n");
    }
    #[test]
    fn stale_strip_legacy_section_stops_at_blank_line_as_native_does() {
        assert_eq!(
            strip_all_managed_blocks("[model_providers.aimai1]\nremoved=1\n\nkept=2\n"),
            "kept=2\n"
        );
    }
}
/// foreign_provider_owners @10037a08c + owner_vec_fold.asm @10050ca18：对诊断section字符串再次first_section_part。
pub fn foreign_provider_owners(
    config: &str,
    known: &std::collections::HashSet<String>,
) -> std::collections::HashSet<String> {
    find_foreign_providers(config, known)
        .into_iter()
        .filter_map(|s| {
            let inner = s.trim().trim_start_matches('[').trim_end_matches(']');
            inner
                .strip_prefix("model_providers.")
                .or_else(|| inner.strip_prefix("profiles."))
                .map(first_section_part)
        })
        .collect()
}
fn assignment_boundary(line: &str, key: &str) -> bool {
    line.strip_prefix(key).is_some_and(|r| {
        r.is_empty()
            || r.chars()
                .next()
                .is_some_and(|c| c == '=' || c.is_whitespace())
    })
}
fn native_assignment_value(line: &str) -> &str {
    line.split('=')
        .nth(1)
        .unwrap_or("")
        .trim()
        .trim_matches('"')
}
/// fix_config_third_party_text.c @10038a534。两遍：删除明确foreign owner section，再清顶层关联残留；保留managed块。
pub fn fix_config_third_party_text(
    config: &str,
    codex_dir: &Path,
    known: &std::collections::HashSet<String>,
) -> Result<FileChange, RecoveryError> {
    let original_owners = foreign_provider_owners(config, known);
    let original_found = find_foreign_providers(config, known);
    if original_found.is_empty()
        && !is_cc_switch_fingerprint(config)
        && find_third_party_base_url(config)?.is_none()
    {
        return Ok(FileChange {
            path: codex_dir.join("config.toml"),
            before: config.into(),
            after: config.into(),
            removed_lines: 0,
            remaining_step: RemainingFixStep::ManagerReconcileThenRereadAndCheck,
        });
    }
    let mut middle = String::new();
    let mut lex = Lexical(2);
    let mut skip = false;
    let mut removed_lines = 0;
    for raw in config.lines() {
        let line = raw.trim();
        let structural = lex.line_is_structural(raw);
        if structural {
            if BEGIN.contains(&line) || END.contains(&line) {
                skip = false;
            } else if let Some(inner) = line.strip_prefix('[').and_then(|s| s.strip_suffix(']')) {
                skip = false;
                if let Some(rest) = inner
                    .strip_prefix("model_providers.")
                    .or_else(|| inner.strip_prefix("profiles."))
                {
                    let id = first_section_part(rest);
                    skip = !is_known_or_builtin(&id, known) && original_owners.contains(&id);
                }
            }
        }
        if skip {
            removed_lines += 1;
        } else {
            middle.push_str(raw);
            middle.push('\n');
        }
    }
    let owners = foreign_provider_owners(&middle, known);
    let has_third_party = is_cc_switch_fingerprint(&middle)
        || !find_foreign_providers(&middle, known).is_empty()
        || find_third_party_base_url(&middle)?.is_some();
    let mut after = String::new();
    let mut lex = Lexical(2);
    let mut managed = false;
    let mut section = false;
    for raw in middle.lines() {
        let line = raw.trim();
        let structural = lex.line_is_structural(raw);
        let mut remove = false;
        if structural {
            if BEGIN.contains(&line) {
                managed = true;
            } else if END.contains(&line) {
                managed = false;
            } else if !managed {
                if line.starts_with('[') {
                    section = true;
                } else if !section {
                    if is_profile_assignment(line) || assignment_boundary(line, "model_provider") {
                        let value = native_assignment_value(line);
                        if !value.is_empty()
                            && !is_known_or_builtin(value, known)
                            && (owners.contains(value) || is_explicit_config_owner_residue(value))
                        {
                            remove = true;
                        }
                    }
                    if !remove && line.starts_with("openai_base_url") {
                        let raw = native_assignment_value(line);
                        let value = unescape_toml_basic_string(raw);
                        remove = !raw.is_empty()
                            && !value.contains("/codex/router/v1")
                            && has_third_party;
                    }
                    if !remove && line.starts_with("model_catalog_json") {
                        let raw = native_assignment_value(line);
                        let value = unescape_toml_basic_string(raw);
                        remove = !raw.is_empty()
                            && !path_is_under_dir(Path::new(&value), codex_dir)?
                            && is_explicit_config_owner_residue(&value);
                    }
                    if !remove
                        && (line.starts_with("circuit_breaker")
                            || line.starts_with("provider_router"))
                    {
                        remove = is_explicit_third_party_marker(line);
                    }
                }
            }
        }
        if remove {
            removed_lines += 1;
        } else {
            after.push_str(raw);
            after.push('\n');
        }
    }
    Ok(FileChange {
        path: codex_dir.join("config.toml"),
        before: config.into(),
        after,
        removed_lines,
        remaining_step: RemainingFixStep::ManagerReconcileThenRereadAndCheck,
    })
}
#[cfg(test)]
mod third_party_fix_tests {
    use super::*;
    use std::collections::HashSet;
    fn fix(s: &str) -> FileChange {
        fix_config_third_party_text(s, Path::new("/synthetic"), &HashSet::new()).unwrap()
    }
    #[test]
    fn third_party_fix_noop_does_not_add_newline_when_no_issue() {
        assert_eq!(fix("model='normal'").after, "model='normal'");
    }
    #[test]
    fn third_party_fix_removes_foreign_sections_and_top_owner_assignments() {
        let s="model_provider='cc-switch'\nprofile='deecodex'\n[model_providers.cc-switch]\napi_key='synthetic'\n[profiles.deecodex]\nmodel='x'\n[user]\nkept=1";
        let c = fix(s);
        assert_eq!(c.after, "[user]\nkept=1\n");
        assert_eq!(c.removed_lines, 6);
        assert_eq!(
            check_config_third_party(Some(&c.after), &HashSet::new())
                .unwrap()
                .status,
            Status::Ok
        );
    }
    #[test]
    fn third_party_fix_keeps_known_section_even_if_marker_and_rechecks_without_fake_success() {
        let known = HashSet::from(["cc-switch".into()]);
        let s = "model_provider=\"cc-switch\"\n[model_providers.cc-switch]\nname='cc-switch'";
        let c = fix_config_third_party_text(s, Path::new("/synthetic"), &known).unwrap();
        assert_eq!(c.after, s.to_owned() + "\n");
        assert_eq!(
            check_config_third_party(Some(&c.after), &known)
                .unwrap()
                .status,
            Status::Error
        );
    }
    #[test]
    fn third_party_fix_removes_redirect_and_external_owner_catalog_preserves_internal_catalog() {
        let c=fix("openai_base_url=\"https://third.test/v1\"\nmodel_catalog_json='/external/ccswitch/catalog.json'\nprovider_router=true\nmodel='kept'");
        assert_eq!(c.after, "model='kept'\n");
        let c=fix("openai_base_url=\"https://third.test/v1\"\nmodel_catalog_json=\"/synthetic/ccswitch/catalog.json\"");
        assert_eq!(
            c.after,
            "model_catalog_json=\"/synthetic/ccswitch/catalog.json\"\n"
        );
    }
    #[test]
    fn third_party_fix_keeps_managed_top_level_lines_and_multiline_fake_markers() {
        let s=format!("openai_base_url=\"https://third.test\"\n{}\nprovider_router=true\n{}\ntext='''\n[model_providers.ccswitch]\n'''\n",BEGIN[0],END[0]);
        let c = fix(&s);
        assert!(c.after.contains("provider_router=true"));
        assert!(c.after.contains("[model_providers.ccswitch]"));
        assert!(!c.after.contains("openai_base_url"));
    }
    #[test]
    fn foreign_owner_fold_reparses_dotted_name_as_native_does() {
        let s = "[model_providers.\"custom.dot\"]\nname='provider_router'";
        assert_eq!(
            foreign_provider_owners(s, &HashSet::new()),
            HashSet::from(["custom".into()])
        );
    }
}
