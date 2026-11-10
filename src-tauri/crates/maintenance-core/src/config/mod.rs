//! AiMaMi 1.2.7 的配置诊断恢复。纯输入，不探测用户目录，不启动任何程序。
//! 原始诊断项结构中 label 实际承载摘要，detail 为可选补充说明。
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Status {
    Ok,
    Warning,
    Error,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DiagnosticItem {
    pub id: &'static str,
    pub status: Status,
    pub label: String,
    pub detail: Option<String>,
    pub fixable: bool,
}
fn item(
    id: &'static str,
    status: Status,
    label: impl Into<String>,
    detail: Option<String>,
    fixable: bool,
) -> DiagnosticItem {
    DiagnosticItem {
        id,
        status,
        label: label.into(),
        detail,
        fixable,
    }
}
fn ok(id: &'static str, label: &str) -> DiagnosticItem {
    item(id, Status::Ok, label, None, false)
}
const BEGIN: [&str; 3] = [
    "# >>> aimami-relay managed start (DO NOT EDIT MANUALLY)",
    "# >>> aimami-relay managed start (top, DO NOT EDIT MANUALLY)",
    "# >>> aimami-relay codex-router top start (DO NOT EDIT MANUALLY)",
];
const END: [&str; 3] = [
    "# <<< aimami-relay managed end",
    "# <<< aimami-relay managed end (top)",
    "# <<< aimami-relay codex-router top end",
];

/// decomp-check_config_profile_conflict.c:19-326 @0x100390ce8。
/// 缺失配置保留原生 ok/跳过语义；本函数只检查 legacy 顶层 profile。
pub fn check_config_profile_conflict(
    config: Option<&str>,
    route_enabled: bool,
) -> Result<DiagnosticItem, RecoveryError> {
    let Some(config) = config else {
        return Ok(ok(
            "config_profile_conflict",
            "顶层 profile 配置（文件不可读，跳过）",
        ));
    };
    for line in top_level_lines(config)? {
        if let Some(value) = assignment(line, "profile") {
            let value = value.trim().trim_matches('"');
            if !value.is_empty() {
                return Ok(item("config_profile_conflict", if route_enabled { Status::Error } else { Status::Warning },
                    format!("检测到 legacy 顶层 profile = \"{value}\""),
                    Some("Codex 0.134+ 已不再支持 config.toml 顶层 profile；智能路由应改用 model_provider + model_catalog_json 的桌面端路由契约。".into()), true));
            }
        }
    }
    Ok(ok("config_profile_conflict", "无 legacy 顶层 profile"))
}
fn assignment<'a>(line: &'a str, key: &str) -> Option<&'a str> {
    let rest = line.strip_prefix(key)?;
    if let Some(c) = rest.chars().next() {
        if c != '=' && !c.is_whitespace() {
            return None;
        }
    }
    rest.trim_start().strip_prefix('=').map(str::trim)
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecoveryError(pub String);
impl std::fmt::Display for RecoveryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}
impl std::error::Error for RecoveryError {}

/// 状态码与 arm64 @0x10078da1c-0x10078dbe0 一致：2=普通、0=基本多行、1=字面多行。
#[derive(Debug, Clone)]
struct Lexical(u8);
impl Lexical {
    fn line_is_structural(&mut self, line: &str) -> bool {
        let started_normal = self.0 == 2;
        let b = line.as_bytes();
        let mut i = 0;
        let mut state = self.0;
        while i < b.len() {
            if state == 2 {
                match b[i] {
                    b'#' => break,
                    q @ (b'"' | b'\'') => {
                        if b.get(i..i + 3) == Some(&[q, q, q]) {
                            state = if q == b'"' { 0 } else { 1 };
                            self.0 = state;
                            i += 3;
                        } else {
                            i += 1;
                            while i < b.len() {
                                if q == b'"' && b[i] == b'\\' {
                                    i = (i + 2).min(b.len());
                                } else {
                                    let close = b[i] == q;
                                    i += 1;
                                    if close {
                                        break;
                                    }
                                }
                            }
                            state = 2;
                        }
                    }
                    _ => i += 1,
                }
            } else if state == 0 && b[i] == b'\\' {
                i = (i + 2).min(b.len());
            } else {
                let q = if state == 0 { b'"' } else { b'\'' };
                if b.get(i..i + 3) == Some(&[q, q, q]) {
                    state = 2;
                    self.0 = 2;
                    i += 3;
                } else {
                    i += 1;
                }
            }
        }
        started_normal
    }
}
fn top_level_lines(config: &str) -> Result<Vec<&str>, RecoveryError> {
    let mut result = vec![];
    let mut lex = Lexical(2);
    let mut managed = false;
    let mut section = false;
    for raw in config.lines() {
        let line = raw.trim();
        if !lex.line_is_structural(raw) {
            if managed || !section {
                section = managed && section;
            }
            continue;
        }
        if BEGIN.contains(&line) {
            managed = true;
            continue;
        }
        let ending = END.contains(&line);
        if ending || managed {
            managed = !ending && managed;
            continue;
        }
        if line.starts_with('[') {
            managed = false;
            section = true;
        } else if !section {
            result.push(line);
            managed = false;
            section = false;
        }
    }
    Ok(result)
}
#[cfg(test)]
mod tests {
    use super::*;
    fn profile(s: &str, on: bool) -> DiagnosticItem {
        check_config_profile_conflict(Some(s), on).unwrap()
    }
    #[test]
    fn profile_route_status_and_value() {
        for (on, status) in [(true, Status::Error), (false, Status::Warning)] {
            let r = profile(" profile = \"old\" ", on);
            assert_eq!(r.status, status);
            assert!(r.fixable);
            assert_eq!(r.label, "检测到 legacy 顶层 profile = \"old\"");
        }
    }
    /// 文案字节长度对齐 arm64 常量：ok label 0x19(25)、detail 0x8f(143)。
    /// 这是原文而非近似写法，长度变化说明有人改动了措辞。
    #[test]
    fn profile_wording_matches_arm64_constant_lengths() {
        let ok = check_config_profile_conflict(Some("model = 'gpt-5'"), true).unwrap();
        assert_eq!(ok.label.as_bytes().len(), 0x19);
        assert_eq!(ok.label, "无 legacy 顶层 profile");
        assert!(ok.detail.is_none(), "ok_item 不写 detail");

        let bad = profile("profile = \"old\"", true);
        let detail = bad.detail.unwrap();
        assert_eq!(detail.as_bytes().len(), 0x8f);
        assert_eq!(
            detail,
            "Codex 0.134+ 已不再支持 config.toml 顶层 profile；\
智能路由应改用 model_provider + model_catalog_json 的桌面端路由契约。"
        );

        // 不可读分支：原版走 ok_item，label 长 0x33(51) 字节。
        let unreadable = check_config_profile_conflict(None, true).unwrap();
        assert_eq!(unreadable.status, Status::Ok);
        assert_eq!(unreadable.label.as_bytes().len(), 0x33);
        assert_eq!(unreadable.label, "顶层 profile 配置（文件不可读，跳过）");
        assert!(!unreadable.fixable);
    }

    #[test]
    fn profile_absent_or_unreadable() {
        assert_eq!(
            check_config_profile_conflict(None, true).unwrap().status,
            Status::Ok
        );
        for text in [
            "",
            "profile = \"\"",
            "profiles = \"x\"",
            "# profile=\"x\"",
            "[profiles.x]\nprofile=\"x\"",
        ] {
            assert_eq!(profile(text, true).status, Status::Ok, "{text}");
        }
    }
    #[test]
    fn all_three_managed_marker_pairs_are_ignored() {
        for (start, end) in BEGIN.into_iter().zip(END) {
            assert_eq!(
                profile(&format!("{start}\nprofile=\"injected\"\n{end}\n"), true).status,
                Status::Ok
            );
            assert_eq!(
                profile(
                    &format!("{start}\nprofile=\"injected\"\n{end}\nprofile=\"old\""),
                    true
                )
                .status,
                Status::Error
            );
        }
    }
    #[test]
    fn multiline_fake_keys_and_markers_do_not_count() {
        for q in ["\"\"\"", "'''"] {
            let text = format!(
                "description={q}\n{}\nprofile=\"fake\"\n{q}\nprofile=\"old\"",
                BEGIN[0]
            );
            assert_eq!(
                profile(&text, true).label,
                "检测到 legacy 顶层 profile = \"old\""
            );
        }
    }
    #[test]
    fn profile_keeps_native_textual_assignment_quirks() {
        // 原生 trim_matches('"')，没有 TOML 解码或删除行内注释。
        assert_eq!(
            profile("profile='old'", true).label,
            "检测到 legacy 顶层 profile = \"'old'\""
        );
        assert_eq!(
            profile("profile=\"old\" # hi", true).label,
            "检测到 legacy 顶层 profile = \"old\" # hi\""
        );
        assert_eq!(
            profile("profile\u{2003}=\"old\"\r\n", true).status,
            Status::Error
        );
    }
    #[test]
    fn lexical_comments_escapes_and_multiline_closure() {
        let mut l = Lexical(2);
        assert!(l.line_is_structural("x=\"\\\"#'''\" # \"\"\""));
        assert_eq!(l.0, 2);
        assert!(l.line_is_structural("x=\"\"\"begin"));
        assert_eq!(l.0, 0);
        assert!(!l.line_is_structural("escaped \\\"\"\""));
        assert_eq!(l.0, 0);
        assert!(!l.line_is_structural("end\"\"\""));
        assert_eq!(l.0, 2);
        assert!(l.line_is_structural(""));
    }
}
/// 直接核对 model_cleanup.asm @0x10078d874-0x10078da18。
/// AiMaMi 保留名称或命名空间的模型才检查；路由开启时，活动模型集合中的名称保留。
pub fn top_level_relay_model_requires_cleanup(
    model: &str,
    route_enabled: bool,
    active_model_slugs: &std::collections::HashSet<String>,
) -> bool {
    let owned = model == "aimami_relay" || model.starts_with("aimami_relay_");
    owned && (!route_enabled || !active_model_slugs.contains(model))
}
/// 观察型文件系统接口；实现者只对合成快照/授权目录提供 metadata，检查自行判断状态。
pub trait ConfigFs {
    fn metadata(&self, path: &std::path::Path) -> std::io::Result<()>;
}
/// decomp-check_config_global_validity.c @0x10038f3ec，helper global_reference.asm @0x100395fb8。
pub fn check_config_global_validity(
    config: Option<&str>,
    codex_dir: &std::path::Path,
    fs: &impl ConfigFs,
) -> Result<DiagnosticItem, RecoveryError> {
    const ID: &str = "config_global_validity";
    let Some(config) = config else {
        return Ok(ok(ID, "全局配置（文件不存在，跳过）"));
    };
    let mut problems = vec![];
    for line in top_level_lines(config)? {
        if let Some(problem) = invalid_global_config_reference(line, codex_dir, fs)? {
            problems.push(problem);
        }
    }
    if problems.is_empty() {
        Ok(item(ID,Status::Ok,"本地配置引用检查通过",Some("此项仅检查可在本地确认的引用；provider、profile 及模型支持的推理档位由 ChatGPT 原生加载器判断，不根据本地白名单删除。".into()),false))
    } else {
        Ok(item(
            ID,
            Status::Error,
            format!("全局配置存在 {} 处无效引用", problems.len()),
            Some(problems.join("；")),
            true,
        ))
    }
}
/// normalize_path.asm @0x100388f50: 仅替换分隔符、折叠重复斜杠、移除尾斜杠，不解析 .. 或 symlink。
pub fn normalize_path_for_compare(path: &str) -> String {
    let mut s = path.replace('\\', "/");
    while s.contains("//") {
        s = s.replace("//", "/");
    }
    s.trim_end_matches('/').to_string()
}
fn path_is_under_dir(path: &std::path::Path, dir: &std::path::Path) -> Result<bool, RecoveryError> {
    let p = normalize_path_for_compare(&path.to_string_lossy());
    let d = normalize_path_for_compare(&dir.to_string_lossy());
    Ok(p == d || p.starts_with(&format!("{d}/")))
}
#[cfg(test)]
mod global_tests {
    use super::*;
    struct Fs(std::io::ErrorKind);
    impl ConfigFs for Fs {
        fn metadata(&self, _: &std::path::Path) -> std::io::Result<()> {
            Err(std::io::Error::from(self.0))
        }
    }
    fn global(s: &str, k: std::io::ErrorKind) -> DiagnosticItem {
        check_config_global_validity(Some(s), std::path::Path::new("/synthetic/codex"), &Fs(k))
            .unwrap()
    }
    #[test]
    fn global_empty_effort_only_not_unknown_effort_or_whitespace() {
        assert_eq!(
            global("model_reasoning_effort=''", std::io::ErrorKind::NotFound).status,
            Status::Error
        );
        for s in [
            "model_reasoning_effort=' '",
            "model_reasoning_effort='unknown'",
            "model_reasoning_effort=0",
            "profile='invalid'",
            "model_provider='unknown'",
            "model_reasoning_effort=oops",
        ] {
            assert_eq!(
                global(s, std::io::ErrorKind::NotFound).status,
                Status::Ok,
                "{s}"
            );
        }
    }
    #[test]
    fn global_absolute_external_not_found_only() {
        let s = "model_catalog_json='/synthetic/other/catalog.json'";
        assert_eq!(
            global(s, std::io::ErrorKind::NotFound).status,
            Status::Error
        );
        assert_eq!(
            global(s, std::io::ErrorKind::PermissionDenied).status,
            Status::Ok
        );
        for s in [
            "model_catalog_json='relative.json'",
            "model_catalog_json='/synthetic/codex/catalog.json'",
            "model_catalog_json='/synthetic/codex'",
            "[profile.foo]\nmodel_catalog_json='/missing.json'",
        ] {
            assert_eq!(
                global(s, std::io::ErrorKind::NotFound).status,
                Status::Ok,
                "{s}"
            );
        }
        assert_eq!(
            global(
                "model_catalog_json='/synthetic/codex-other/catalog.json'",
                std::io::ErrorKind::NotFound
            )
            .status,
            Status::Error
        );
    }
    #[test]
    fn global_collects_each_bad_line_in_order() {
        let r = global(
            "model_reasoning_effort=''\nmodel_catalog_json='/missing.json'",
            std::io::ErrorKind::NotFound,
        );
        assert_eq!(r.label, "全局配置存在 2 处无效引用");
        assert_eq!(
            r.detail.as_deref(),
            Some("model_reasoning_effort 不能为空；model_catalog_json 指向不存在的外部路径")
        );
    }
    #[test]
    fn global_skips_managed_and_multiline_fake_values() {
        let s = format!(
            "{}\nmodel_reasoning_effort=''\n{}\ndescription='''\nmodel_reasoning_effort=''\n'''\n",
            BEGIN[0], END[0]
        );
        assert_eq!(global(&s, std::io::ErrorKind::NotFound).status, Status::Ok);
    }
    #[test]
    fn native_path_normalization_does_not_resolve_parent_components() {
        assert_eq!(normalize_path_for_compare("/a\\b///c//"), "/a/b/c");
        assert_eq!(normalize_path_for_compare("/a/../b/"), "/a/../b");
        assert_eq!(normalize_path_for_compare("///"), "");
    }
    #[test]
    fn cleanup_owned_names_and_active_set() {
        let active = std::collections::HashSet::from(["aimami_relay_active".to_owned()]);
        for on in [false, true] {
            assert!(!top_level_relay_model_requires_cleanup(
                "gpt-5", on, &active
            ));
            assert!(!top_level_relay_model_requires_cleanup(
                "aimami_relayish",
                on,
                &active
            ));
            assert!(top_level_relay_model_requires_cleanup(
                "aimami_relay",
                on,
                &active
            ));
            assert!(top_level_relay_model_requires_cleanup(
                "aimami_relay_old",
                on,
                &active
            ));
        }
        assert!(!top_level_relay_model_requires_cleanup(
            "aimami_relay_active",
            true,
            &active
        ));
        assert!(top_level_relay_model_requires_cleanup(
            "aimami_relay_active",
            false,
            &active
        ));
    }
}
/// third_party_marker.asm @0x100393740，ASCII 小写后子串匹配；不是 TOML 键匹配。
pub fn is_explicit_third_party_marker(text: &str) -> bool {
    let text = text.to_ascii_lowercase();
    [
        "cc-switch",
        "cc_switch",
        "ccswitch",
        "codex-switch",
        "codex_switch",
        "circuit_breaker",
        "provider_router",
    ]
    .iter()
    .any(|marker| text.contains(marker))
}
/// fingerprint.asm @0x10037c1ac：跳过注释和多行字符串内容。
pub fn is_cc_switch_fingerprint(config: &str) -> bool {
    let mut lex = Lexical(2);
    config.lines().any(|raw| {
        lex.line_is_structural(raw)
            && !raw.trim().starts_with('#')
            && is_explicit_third_party_marker(raw.trim())
    })
}
/// check_config_third_party.c:267-438 @0x10037aa04。
/// URL parse 成功时移除 userinfo/query/fragment 与尾斜杠；失败时截断 ?/#，并把 @ 写为 [at]。
pub fn sanitize_third_party_base_url(raw: &str) -> String {
    let raw = raw.trim();
    match url::Url::parse(raw) {
        Ok(mut parsed) => {
            let _ = parsed.set_username("");
            let _ = parsed.set_password(None);
            parsed.set_query(None);
            parsed.set_fragment(None);
            parsed.as_str().trim_end_matches('/').to_owned()
        }
        Err(_) => raw
            .split(['?', '#'])
            .next()
            .unwrap_or("")
            .replace('@', "[at]"),
    }
}
#[cfg(test)]
mod third_party_component_tests {
    use super::*;
    #[test]
    fn fingerprint_skips_comments_and_multiline_body_but_keeps_native_substring_behavior() {
        assert!(!is_cc_switch_fingerprint(
            "# CC-SWITCH\nx='''\nprovider_router=true\n'''"
        ));
        for marker in [
            "cc-switch",
            "cc_switch",
            "ccswitch",
            "codex-switch",
            "codex_switch",
            "circuit_breaker",
            "provider_router",
        ] {
            assert!(is_cc_switch_fingerprint(&format!(
                "name='{}'",
                marker.to_uppercase()
            )));
        }
        // Native helper receives whole structural line, including same-line comment.
        assert!(is_cc_switch_fingerprint("x=1 # cc-switch"));
    }
    #[test]
    fn sanitizer_removes_userinfo_and_secret_query_fragment() {
        assert_eq!(
            sanitize_third_party_base_url(
                " https://user:secret@EXAMPLE.com/v1/?token=private#secret "
            ),
            "https://example.com/v1"
        );
        assert_eq!(
            sanitize_third_party_base_url("bad@host/path?secret#secret"),
            "bad[at]host/path"
        );
        assert_eq!(
            sanitize_third_party_base_url("bad@host/path/#secret"),
            "bad[at]host/path/"
        );
    }
}
/// first_section_part.asm @0x1003735e4-0x100373af8。
/// 保留原生轻量scanner：只识别双引号，quoted反斜杠原样保留，不做TOML unescape。
pub fn first_section_part(section: &str) -> String {
    let mut out = String::new();
    let mut quoted = false;
    let mut escaped = false;
    for c in section.chars() {
        if escaped {
            out.push(c);
            escaped = false;
            continue;
        }
        match c {
            '"' => quoted = !quoted,
            '.' if !quoted => break,
            '\\' if quoted => {
                out.push(c);
                escaped = true;
            }
            _ => out.push(c),
        }
    }
    out.trim().to_owned()
}
/// diagnostic_valid_provider_ids @0x10039264c 的配置集合提取段。
/// 原生并非完整TOML parse；只识别完整独占一行的 [model_providers.*] header。
pub fn diagnostic_config_provider_ids(text: &str) -> std::collections::HashSet<String> {
    let mut lex = Lexical(2);
    let mut ids = std::collections::HashSet::new();
    for raw in text.lines() {
        let structural = lex.line_is_structural(raw);
        let line = raw.trim();
        if structural {
            if let Some(section) = line.strip_prefix('[').and_then(|s| s.strip_suffix(']')) {
                if let Some(rest) = section.strip_prefix("model_providers.") {
                    ids.insert(first_section_part(rest));
                }
            }
        }
    }
    ids
}
#[cfg(test)]
mod section_tests {
    use super::*;
    #[test]
    fn first_section_parser_native_quoting_and_escaping() {
        assert_eq!(first_section_part(" foo.bar "), "foo");
        assert_eq!(first_section_part("\"foo.bar\".child"), "foo.bar");
        assert_eq!(first_section_part("'foo.bar'.child"), "'foo");
        assert_eq!(first_section_part("\"foo\\\".bar\".child"), "foo\\\".bar");
        assert_eq!(first_section_part("\"中文.服务\".child"), "中文.服务");
    }
    #[test]
    fn provider_ids_preserve_native_header_boundary() {
        let ids=diagnostic_config_provider_ids("[model_providers.alpha]\n[model_providers.\"dotted.id\".nested]\n[model_providers.beta] #comment\n[[model_providers.array]]\n[ model_providers.space]\nx='''\n[model_providers.fake]\n'''\n");
        assert_eq!(
            ids,
            std::collections::HashSet::from(["alpha".into(), "dotted.id".into()])
        );
    }
}
/// unescape.asm @0x100389264-0x1003898a0；保留未知转义（包括\\u），没有Unicode escape解码。
pub fn unescape_toml_basic_string(raw: &str) -> String {
    let mut chars = raw.chars();
    let mut out = String::new();
    while let Some(c) = chars.next() {
        if c != '\\' {
            out.push(c);
            continue;
        }
        match chars.next() {
            Some('"') => out.push('"'),
            Some('\\') => out.push('\\'),
            Some('n') => out.push('\n'),
            Some('r') => out.push('\r'),
            Some('t') => out.push('\t'),
            Some('b') => out.push('\u{8}'),
            Some('f') => out.push('\u{c}'),
            Some(c) => {
                out.push('\\');
                out.push(c);
            }
            None => out.push('\\'),
        }
    }
    out
}
/// asm/find_third_party_base_url.asm @0x10037ce64-0x10037d440。
/// 与原生一致：prefix匹配、split('=').nth(1)，排除 /codex/router/v1 子串；不检查域名。
pub fn find_third_party_base_url(config: &str) -> Result<Option<String>, RecoveryError> {
    for line in top_level_lines(config)? {
        if !line.starts_with("openai_base_url") {
            continue;
        }
        if let Some(value) = line.split('=').nth(1) {
            let value = value.trim().trim_matches('"');
            if !value.is_empty() {
                let decoded = unescape_toml_basic_string(value);
                if !decoded.contains("/codex/router/v1") {
                    return Ok(Some(decoded));
                }
            }
        }
    }
    Ok(None)
}
#[cfg(test)]
mod base_url_tests {
    use super::*;
    #[test]
    fn limited_unescape_preserves_unknown_and_trailing() {
        assert_eq!(
            unescape_toml_basic_string(r#"a\n\r\t\b\f\"\\\u0041\z\"#),
            "a\n\r\t\u{8}\u{c}\"\\\\u0041\\z\\"
        );
    }
    #[test]
    fn base_url_keeps_original_non_domain_and_textual_rules() {
        assert_eq!(
            find_third_party_base_url("openai_base_url=\"https://api.openai.com/v1\"").unwrap(),
            Some("https://api.openai.com/v1".into())
        );
        assert_eq!(
            find_third_party_base_url("openai_base_url=\"http://127.0.0.1:9/codex/router/v1\"")
                .unwrap(),
            None
        );
        assert_eq!(
            find_third_party_base_url("openai_base_url_other=\"abc=def\"").unwrap(),
            Some("abc".into())
        );
        assert_eq!(
            find_third_party_base_url("[x]\nopenai_base_url=\"https://third.test\"").unwrap(),
            None
        );
    }
}
/// known_builtin.asm @0x100374940：已知集合仍须排除旧 relay model 命名空间。
pub fn is_known_or_builtin(id: &str, known: &std::collections::HashSet<String>) -> bool {
    id.eq_ignore_ascii_case("openai")
        || ["aimai1", "openai-bundled", "openai-primary-runtime"].contains(&id)
        || (known.contains(id) && id != "aimami_relay" && !id.starts_with("aimami_relay_"))
}
/// owner_residue.asm @0x10039b494。注意原二进制字面量是 deecodex。
pub fn is_explicit_config_owner_residue(id: &str) -> bool {
    let id = id.to_ascii_lowercase();
    [
        "cc-switch",
        "cc_switch",
        "ccswitch",
        "codex-switch",
        "codex_switch",
        "deecodex",
    ]
    .iter()
    .any(|s| id.contains(s))
}
/// foreign_providers.asm @0x100376eb0：未知配置只有 owner名称或section内容明确标记第三方时才报告。
pub fn find_foreign_providers(
    config: &str,
    known: &std::collections::HashSet<String>,
) -> Vec<String> {
    let mut lex = Lexical(2);
    let mut found = vec![];
    let mut pending: Option<(String, bool, bool)> = None;
    for raw in config.lines() {
        if !lex.line_is_structural(raw) {
            continue;
        }
        let line = raw.trim();
        if let Some(inner) = line.strip_prefix('[').and_then(|s| s.strip_suffix(']')) {
            if let Some((name, owner, marker)) = pending.take() {
                if !owner && marker {
                    found.push(name);
                }
            }
            let candidate = inner
                .strip_prefix("model_providers.")
                .map(|r| ("model_providers", r))
                .or_else(|| inner.strip_prefix("profiles.").map(|r| ("profiles", r)));
            if let Some((prefix, rest)) = candidate {
                let id = first_section_part(rest);
                if !is_known_or_builtin(&id, known) {
                    let owner = is_explicit_config_owner_residue(&id);
                    let name = format!("[{prefix}.{id}]");
                    if owner {
                        found.push(name.clone());
                    }
                    pending = Some((name, owner, false));
                }
            }
        } else if let Some((_, owner, marker)) = &mut pending {
            if !*owner && !*marker {
                *marker = is_explicit_third_party_marker(line);
            }
        }
    }
    if let Some((name, owner, marker)) = pending {
        if !owner && marker {
            found.push(name);
        }
    }
    found
}
/// decomp-check_config_third_party.c @0x10037aa04，完整检查（known为实际已配置provider/profile标识）。
pub fn check_config_third_party(
    config: Option<&str>,
    known: &std::collections::HashSet<String>,
) -> Result<DiagnosticItem, RecoveryError> {
    const ID: &str = "config_third_party";
    let Some(config) = config else {
        return Ok(item(
            ID,
            Status::Warning,
            "无法读取 config.toml",
            Some("文件不存在或无读取权限".into()),
            false,
        ));
    };
    let foreign = find_foreign_providers(config, known);
    let fingerprint = is_cc_switch_fingerprint(config);
    let base = find_third_party_base_url(config)?;
    if foreign.is_empty() && !fingerprint && base.is_none() {
        return Ok(ok(ID, "无第三方注入残留"));
    }
    let mut parts = vec![];
    if !foreign.is_empty() {
        parts.push(format!(
            "{} 处 section（{}）",
            foreign.len(),
            foreign.join(", ")
        ));
    }
    if fingerprint {
        parts.push("cc-switch 指纹".into());
    }
    if let Some(base) = &base {
        parts.push(format!(
            "顶层 openai_base_url 重定向到第三方域名（{}）",
            sanitize_third_party_base_url(base)
        ));
    }
    // 原版三种 label：base_url 重定向优先，其次 cc-switch 指纹，其余为工具残留。
    // 「发现 N 处第三方注入残留」在原版是 detail 模板，不是 label。
    let label = if base.is_some() {
        "检测到第三方域名重定向残留"
    } else if fingerprint {
        "检测到第三方工具残留"
    } else {
        "检测到第三方工具残留"
    };
    // 原版 detail：cc-switch 指纹走长句，否则用「发现 N 处第三方注入残留」。
    let detail = if fingerprint {
        format!(
            "config.toml 含第三方工具残留，会与本地智能路由冲突或把官方请求打到第三方域名。\n残留: {}",
            parts.join("；")
        )
    } else {
        format!("发现 {} 处第三方注入残留: {}", parts.len(), parts.join("；"))
    };
    Ok(item(ID, Status::Error, label, Some(detail), true))
}
#[cfg(test)]
mod third_party_parity_tests {
    use super::*;
    use std::collections::HashSet;

    /// 原版三分支：未读到配置 → warning 且不可修复；无三种信号 → ok；有信号 → error 且可修复。
    #[test]
    fn three_original_branches_and_label_wording() {
        let known = HashSet::new();

        let missing = check_config_third_party(None, &known).unwrap();
        assert_eq!(missing.status, Status::Warning);
        assert_eq!(missing.label, "无法读取 config.toml");
        assert_eq!(missing.detail.as_deref(), Some("文件不存在或无读取权限"));
        assert!(!missing.fixable);

        let clean = check_config_third_party(Some("model = 'gpt-5'\n"), &known).unwrap();
        assert_eq!(clean.status, Status::Ok);
        assert_eq!(clean.label, "无第三方注入残留");

        let redirected = check_config_third_party(
            Some("openai_base_url = \"https://third.test/v1\"\n"),
            &known,
        )
        .unwrap();
        assert_eq!(redirected.status, Status::Error);
        assert_eq!(redirected.label, "检测到第三方域名重定向残留");
        assert!(redirected.fixable);
        assert!(redirected.detail.as_deref().unwrap().starts_with("发现 1 处第三方注入残留"));
    }

    /// cc-switch 指纹走长句 detail，label 仍是「检测到第三方工具残留」。
    #[test]
    fn fingerprint_uses_long_detail_and_tool_label() {
        let known = HashSet::new();
        let item = check_config_third_party(
            Some("[model_providers.x]\nname = 'cc-switch'\nbase_url = 'https://a.test'\n"),
            &known,
        )
        .unwrap();
        assert_eq!(item.status, Status::Error);
        assert_eq!(item.label, "检测到第三方工具残留");
        let detail = item.detail.unwrap();
        assert!(detail.contains("cc-switch 指纹"));
        assert!(detail.contains("会与本地智能路由冲突"));
    }
}

#[cfg(test)]
mod third_party_full_tests {
    use super::*;
    use std::collections::HashSet;
    #[test]
    fn third_party_absent_clean_and_url_status() {
        let known = HashSet::new();
        let missing = check_config_third_party(None, &known).unwrap();
        assert_eq!(missing.status, Status::Warning);
        assert!(!missing.fixable);
        assert_eq!(
            check_config_third_party(Some("model='gpt-5'"), &known)
                .unwrap()
                .status,
            Status::Ok
        );
        let r = check_config_third_party(
            Some("openai_base_url=\"https://u:pw@third.test/v1?key=secret\""),
            &known,
        )
        .unwrap();
        assert_eq!(r.status, Status::Error);
        assert!(r.fixable);
        assert!(!r.detail.as_ref().unwrap().contains("secret"));
        assert!(!r.detail.as_ref().unwrap().contains("pw"));
    }
    #[test]
    fn foreign_unknown_name_requires_evidence_not_a_blanket_whitelist() {
        let known = HashSet::new();
        assert!(find_foreign_providers(
            "[model_providers.custom]\nbase_url='https://custom.test'",
            &known
        )
        .is_empty());
        assert_eq!(
            find_foreign_providers("[model_providers.custom]\nname='provider_router'", &known),
            vec!["[model_providers.custom]"]
        );
        assert_eq!(
            find_foreign_providers("[profiles.custom]\n# circuit_breaker", &known),
            vec!["[profiles.custom]"]
        );
    }
    #[test]
    fn owner_residue_immediate_and_known_names_preserved() {
        let known = HashSet::from(["custom".into(), "aimami_relay_bad".into()]);
        assert!(is_known_or_builtin("OpEnAI", &known));
        assert!(is_known_or_builtin("aimai1", &known));
        assert!(!is_known_or_builtin("aimami1", &known));
        assert!(!is_known_or_builtin("aimami_relay_bad", &known));
        assert!(is_known_or_builtin("custom", &known));
        assert_eq!(find_foreign_providers("[model_providers.deecodex]\n[profiles.cc_switch]\n[model_providers.custom]\nname='cc-switch'",&known),vec!["[model_providers.deecodex]","[profiles.cc_switch]"]);
    }
    #[test]
    fn foreign_flushes_on_header_and_eof_without_duplicate_owner() {
        let s="[model_providers.ccswitch]\nx='cc-switch'\n[other]\n[profiles.\"custom.name\"]\nx='provider_router'\n[model_providers.tail]\nx='circuit_breaker'";
        assert_eq!(
            find_foreign_providers(s, &HashSet::new()),
            vec![
                "[model_providers.ccswitch]",
                "[profiles.custom.name]",
                "[model_providers.tail]"
            ]
        );
    }
    #[test]
    fn foreign_ignores_fake_multiline_sections() {
        let s="text='''\n[model_providers.ccswitch]\n'''\n[model_providers.custom]\nx='''\nprovider_router\n'''";
        assert!(find_foreign_providers(s, &HashSet::new()).is_empty());
    }
    #[test]
    fn check_label_precedence_and_problem_order() {
        // 原版 label 只有三种取值：「发现 N 处…」是 detail 模板而非 label。
        let s = "[model_providers.deecodex]";
        let r = check_config_third_party(Some(s), &HashSet::new()).unwrap();
        assert_eq!(r.label, "检测到第三方工具残留");
        assert!(r.detail.unwrap().starts_with("发现 1 处第三方注入残留"));
        let r =
            check_config_third_party(Some("[model_providers.ccswitch]"), &HashSet::new()).unwrap();
        assert_eq!(r.label, "检测到第三方工具残留");
        assert!(r.detail.unwrap().contains("cc-switch 指纹"));
    }
}
mod stale;
pub use stale::*;

fn invalid_global_config_reference(
    line: &str,
    codex_dir: &std::path::Path,
    fs: &impl ConfigFs,
) -> Result<Option<&'static str>, RecoveryError> {
    let Ok(value) = toml::from_str::<toml::Value>(line) else {
        return Ok(None);
    };
    if value
        .get("model_reasoning_effort")
        .and_then(toml::Value::as_str)
        == Some("")
    {
        return Ok(Some("model_reasoning_effort 不能为空"));
    }
    if let Some(path) = value
        .get("model_catalog_json")
        .and_then(toml::Value::as_str)
    {
        let path = std::path::Path::new(path);
        if path.is_absolute()
            && !path_is_under_dir(path, codex_dir)?
            && matches!(fs.metadata(path),Err(e) if e.kind()==std::io::ErrorKind::NotFound)
        {
            return Ok(Some("model_catalog_json 指向不存在的外部路径"));
        }
    }
    Ok(None)
}
mod fixes;
pub use fixes::*;
