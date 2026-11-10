//! Independently recovered static catalog checks. Never discovers a real home directory.
pub mod builder;
pub mod credentials;
pub mod publication;
mod reader;
pub mod state;
pub use reader::read_top_level_string_value;
use serde_json::Value;
use std::{collections::BTreeSet, io, path::Path};

pub const CATALOG_FILE: &str = "codex_router_catalog.json";
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Severity {
    Ok,
    Warning,
    Error,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Diagnostic {
    pub id: &'static str,
    pub severity: Severity,
    pub title: String,
    pub detail: Option<String>,
    pub fixable: bool,
}
impl Diagnostic {
    fn new(
        id: &'static str,
        severity: Severity,
        title: impl Into<String>,
        detail: impl Into<String>,
    ) -> Self {
        Self {
            id,
            severity,
            title: title.into(),
            detail: Some(detail.into()),
            fixable: severity != Severity::Ok,
        }
    }
    fn ok(id: &'static str, title: &str) -> Self {
        Self {
            id,
            severity: Severity::Ok,
            title: title.into(),
            detail: None,
            fixable: false,
        }
    }
}

/// Only the metadata success/error matters: directories and symlinks are not rejected.
pub trait CatalogFs {
    fn metadata(&self, path: &Path) -> io::Result<()>;
    fn read_to_string(&self, path: &Path) -> io::Result<String>;
}
pub struct LocalFs;
impl CatalogFs for LocalFs {
    fn metadata(&self, path: &Path) -> io::Result<()> {
        std::fs::metadata(path).map(|_| ())
    }
    fn read_to_string(&self, path: &Path) -> io::Result<String> {
        std::fs::read_to_string(path)
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AccountMode {
    Native,
    NoAccount,
}

/// Byte-for-byte slug identity: invalid/missing slug fields are ignored, empty strings remain.
/// Binary 1008079ec..100807a18; dedup is by HashSet in the original.
pub fn catalog_slugs(models: &[Value]) -> BTreeSet<String> {
    models
        .iter()
        .filter_map(|m| m.get("slug").and_then(Value::as_str))
        .map(str::to_owned)
        .collect()
}

/// Core check with a typed model inventory resolver. The resolver returns actual model rows,
/// not a caller-computed validity flag. Provider resolution is defined in models.rs.
pub fn check_catalog_integrity(
    fs: &impl CatalogFs,
    root: &Path,
    state: &RouterState,
) -> Diagnostic {
    const ID: &str = "catalog_integrity";
    if !state.enabled {
        return Diagnostic::ok(ID, "Catalog 完整性（路由未启用，跳过）");
    }
    let path = root.join(CATALOG_FILE);
    if fs.metadata(&path).is_err() {
        return Diagnostic::new(
            ID,
            Severity::Error,
            "Catalog 文件缺失",
            "路由已启用但 catalog 不存在，Codex 菜单无法显示中转模型",
        );
    }
    let text = match fs.read_to_string(&path) {
        Ok(s) => s,
        Err(e) => {
            return Diagnostic::new(ID, Severity::Error, "Catalog 文件无法读取", e.to_string())
        }
    };
    if text.trim().is_empty() {
        return Diagnostic::new(ID, Severity::Error, "Catalog 文件为空", "需要重新生成");
    }
    let value: Value = match serde_json::from_str(&text) {
        Ok(v) => v,
        Err(_) => {
            return Diagnostic::new(
                ID,
                Severity::Error,
                "Catalog 文件 JSON 格式损坏",
                "需要重新生成",
            )
        }
    };
    let models = match value.get("models").and_then(Value::as_array) {
        Some(m) => m,
        None => {
            return Diagnostic::new(
                ID,
                Severity::Error,
                "Catalog 缺少 models 数组",
                "需要重新生成",
            )
        }
    };
    let actual = catalog_slugs(models);
    let expected = expected_slugs(state);
    let missing: Vec<_> = expected.difference(&actual).cloned().collect();
    let extra: Vec<_> = if state.mode == AccountMode::NoAccount {
        actual.difference(&expected).cloned().collect()
    } else {
        vec![]
    };
    if missing.is_empty() && extra.is_empty() {
        return Diagnostic::ok(ID, "Catalog 完整");
    }
    let mut details = vec![];
    if !missing.is_empty() {
        details.push(format!("缺少：{}", missing.join("、")));
    }
    if !extra.is_empty() {
        details.push(format!("无账号目录包含非当前模型：{}", extra.join("、")));
    }
    Diagnostic::new(
        ID,
        Severity::Warning,
        "Catalog 与卡片模型及历史兼容映射不一致",
        format!("{}。重新生成后恢复一致。", details.join("；")),
    )
}

/// Native comparison only normalizes separators; it does not resolve symlinks or `..`.
pub fn normalize_path_for_compare(path: &str) -> String {
    let mut s = path.replace('\\', "/");
    while s.contains("//") {
        s = s.replace("//", "/");
    }
    s.trim_end_matches('/').to_owned()
}
pub fn path_is_under_dir(path: &str, directory: &Path) -> bool {
    let path = normalize_path_for_compare(path);
    let directory = normalize_path_for_compare(&directory.to_string_lossy());
    path == directory || path.starts_with(&format!("{directory}/"))
}

/// `config` is the previously read file text; None is a failed read (not failed TOML parse).
pub fn check_catalog_path_validity(
    fs: &impl CatalogFs,
    config: Option<&str>,
    root: &Path,
    enabled: bool,
    mode: AccountMode,
) -> Diagnostic {
    const ID: &str = "catalog_path_validity";
    if !enabled {
        return Diagnostic::ok(ID, "Catalog 路径（路由未启用，跳过）");
    }
    let Some(config) = config else {
        return Diagnostic::ok(ID, "Catalog 路径（config 不可读，跳过）");
    };
    let path = read_top_level_string_value(config, "model_catalog_json");
    if mode == AccountMode::Native {
        return if path.is_none() {
            Diagnostic::ok(ID, "官方目录使用原生发现，无静态目录覆盖")
        } else {
            Diagnostic::new(
                ID,
                Severity::Error,
                "真实账号仍被静态目录覆盖",
                "请通过正常路由重收敛迁移至原生目录。",
            )
        };
    }
    let Some(path) = path else {
        return Diagnostic::new(
            ID,
            Severity::Warning,
            "config.toml 缺少 model_catalog_json 引用",
            "路由已启用但配置中没有 catalog 路径，Codex 菜单无法显示中转模型",
        );
    };
    if path_is_under_dir(&path, root) {
        return if fs.metadata(Path::new(&path)).is_ok() {
            Diagnostic::ok(ID, "Catalog 路径有效")
        } else {
            Diagnostic::new(
                ID,
                Severity::Error,
                "model_catalog_json 指向的文件不存在",
                format!("路径：{path}"),
            )
        };
    }
    let foreign = ["cc-switch", "cc_switch", "ccswitch", "codex-switch"]
        .iter()
        .any(|p| path.contains(p));
    let title = if foreign {
        "model_catalog_json 指向第三方工具残留路径"
    } else {
        "model_catalog_json 指向非受管路径"
    };
    Diagnostic::new(ID, Severity::Error, title, format!("当前路径：{path}"))
}

/// Relevant native RelayProvider fields (native stride 0x100). Models are raw identifiers.
#[derive(Debug, Clone)]
pub struct Provider {
    pub id: String,
    pub model: String,
    pub models: Vec<String>,
}
/// Relevant LegacyNoAccountAlias field at native offset 0x30; other fields are not inspected.
#[derive(Debug, Clone)]
pub struct LegacyAlias {
    pub slot_slug: Option<String>,
}
#[derive(Debug, Clone)]
pub struct RouterState {
    pub enabled: bool,
    pub mode: AccountMode,
    pub providers: Vec<Provider>,
    pub active_provider_ids: Vec<String>,
    pub legacy_aliases: Vec<LegacyAlias>,
}
/// Native canonical_no_account_slot_slug @100370184 is an exact, case-sensitive allowlist.
pub fn canonical_no_account_slot_slug(slug: &str) -> Option<&'static str> {
    match slug {
        "gpt-5.6-sol" => Some("gpt-5.6-sol"),
        "gpt-5.6-terra" => Some("gpt-5.6-terra"),
        "gpt-5.6-luna" => Some("gpt-5.6-luna"),
        "gpt-5.5" => Some("gpt-5.5"),
        "gpt-5.4" => Some("gpt-5.4"),
        _ => None,
    }
}
pub fn expected_slugs(state: &RouterState) -> BTreeSet<String> {
    let mut slugs = BTreeSet::new();
    for id in &state.active_provider_ids {
        // Native find selects the first matching provider, without any enabled-field filter.
        let Some(provider) = state.providers.iter().find(|p| &p.id == id) else {
            continue;
        };
        if provider.models.is_empty() {
            slugs.insert(format!("{}::{}", provider.id, provider.model));
        } else {
            for model in &provider.models {
                slugs.insert(format!("{}::{model}", provider.id));
            }
        }
    }
    if state.mode == AccountMode::NoAccount {
        slugs.extend(
            state
                .legacy_aliases
                .iter()
                .filter_map(|a| a.slot_slug.as_deref())
                .filter_map(canonical_no_account_slot_slug)
                .map(str::to_owned),
        );
    }
    slugs
}

#[cfg(test)]
mod tests {
    use super::*;
    fn state(mode: AccountMode) -> RouterState {
        RouterState {
            enabled: true,
            mode,
            providers: vec![Provider {
                id: "p".into(),
                model: "fallback".into(),
                models: vec!["z".into(), "a".into(), "a".into()],
            }],
            active_provider_ids: vec!["missing".into(), "p".into()],
            legacy_aliases: vec![],
        }
    }
    struct Fs {
        metadata_error: Option<io::ErrorKind>,
        text: Result<String, io::ErrorKind>,
    }
    impl CatalogFs for Fs {
        fn metadata(&self, _: &Path) -> io::Result<()> {
            self.metadata_error.map_or(Ok(()), |e| Err(e.into()))
        }
        fn read_to_string(&self, _: &Path) -> io::Result<String> {
            self.text.clone().map_err(Into::into)
        }
    }
    fn check(text: &str, state: &RouterState) -> Diagnostic {
        check_catalog_integrity(
            &Fs {
                metadata_error: None,
                text: Ok(text.into()),
            },
            Path::new("/synthetic"),
            state,
        )
    }
    #[test]
    fn disabled_does_not_access_fs() {
        struct Never;
        impl CatalogFs for Never {
            fn metadata(&self, _: &Path) -> io::Result<()> {
                panic!()
            }
            fn read_to_string(&self, _: &Path) -> io::Result<String> {
                panic!()
            }
        }
        let mut s = state(AccountMode::Native);
        s.enabled = false;
        assert_eq!(
            check_catalog_integrity(&Never, Path::new("/synthetic"), &s).severity,
            Severity::Ok
        );
        assert_eq!(
            check_catalog_path_validity(&Never, None, Path::new("/synthetic"), false, s.mode)
                .severity,
            Severity::Ok
        );
    }
    #[test]
    fn integrity_io_and_parse_branches() {
        let s = state(AccountMode::Native);
        for kind in [io::ErrorKind::NotFound, io::ErrorKind::PermissionDenied] {
            let r = check_catalog_integrity(
                &Fs {
                    metadata_error: Some(kind),
                    text: Ok("".into()),
                },
                Path::new("/synthetic"),
                &s,
            );
            assert_eq!(r.title, "Catalog 文件缺失");
            assert_eq!(r.severity, Severity::Error);
        }
        assert_eq!(
            check_catalog_integrity(
                &Fs {
                    metadata_error: None,
                    text: Err(io::ErrorKind::InvalidData)
                },
                Path::new("/synthetic"),
                &s
            )
            .title,
            "Catalog 文件无法读取"
        );
        for (text, title) in [
            (" \n\u{2003}", "Catalog 文件为空"),
            ("x", "Catalog 文件 JSON 格式损坏"),
            ("null", "Catalog 缺少 models 数组"),
            ("{\"models\":{}}", "Catalog 缺少 models 数组"),
        ] {
            assert_eq!(check(text, &s).title, title);
        }
    }
    #[test]
    fn malformed_entries_empty_slug_and_duplicates() {
        let v: Value = serde_json::from_str(
            r#"[{"slug":"a"},{"slug":"a"},{"slug":""},{"slug":1},{},false,"x",null]"#,
        )
        .unwrap();
        assert_eq!(
            catalog_slugs(v.as_array().unwrap()),
            BTreeSet::from(["".into(), "a".into()])
        );
    }
    #[test]
    fn native_allows_extras_no_account_reports_sorted_differences() {
        let mut s = state(AccountMode::Native);
        let text = r#"{"models":[{"slug":"p::z"},{"slug":"p::a"},{"slug":"extra"}]}"#;
        assert_eq!(check(text, &s).severity, Severity::Ok);
        s.mode = AccountMode::NoAccount;
        assert_eq!(
            check(text, &s).detail.as_deref().unwrap(),
            "无账号目录包含非当前模型：extra。重新生成后恢复一致。"
        );
        assert_eq!(
            check(r#"{"models":[{"slug":"z"},{"slug":"b"}]}"#, &s)
                .detail
                .as_deref()
                .unwrap(),
            "缺少：p::a、p::z；无账号目录包含非当前模型：b、z。重新生成后恢复一致。"
        );
    }
    #[test]
    fn provider_first_match_fallback_and_legacy_allowlist() {
        let mut s = state(AccountMode::NoAccount);
        s.providers[0].models.clear();
        s.providers.push(Provider {
            id: "p".into(),
            model: "ignored".into(),
            models: vec![],
        });
        s.legacy_aliases = vec![
            LegacyAlias {
                slot_slug: Some("gpt-5.5".into()),
            },
            LegacyAlias {
                slot_slug: Some("GPT-5.5".into()),
            },
            LegacyAlias { slot_slug: None },
        ];
        assert_eq!(
            expected_slugs(&s),
            BTreeSet::from(["gpt-5.5".into(), "p::fallback".into()])
        );
        for slug in [
            "gpt-5.6-sol",
            "gpt-5.6-terra",
            "gpt-5.6-luna",
            "gpt-5.5",
            "gpt-5.4",
        ] {
            assert_eq!(canonical_no_account_slot_slug(slug), Some(slug));
        }
        assert_eq!(canonical_no_account_slot_slug("gpt-5.6"), None);
    }
    #[test]
    fn normalization_is_lexical_and_case_sensitive() {
        assert_eq!(normalize_path_for_compare(r"C:\\foo\\bar///"), "C:/foo/bar");
        assert!(path_is_under_dir("/base/../outside", Path::new("/base")));
        assert!(!path_is_under_dir("/baseball/x", Path::new("/base")));
        assert!(!path_is_under_dir("/BASE/x", Path::new("/base")));
        assert!(path_is_under_dir("/x", Path::new("/")));
    }
    #[test]
    fn synthetic_native_fs_checks_utf8_and_metadata() {
        let dir = tempfile::tempdir().unwrap();
        let s = state(AccountMode::Native);
        let p = dir.path().join(CATALOG_FILE);
        std::fs::write(&p, [0xff]).unwrap();
        assert_eq!(
            check_catalog_integrity(&LocalFs, dir.path(), &s).title,
            "Catalog 文件无法读取"
        );
        std::fs::remove_file(&p).unwrap();
        std::fs::create_dir(&p).unwrap();
        assert_eq!(
            check_catalog_integrity(&LocalFs, dir.path(), &s).title,
            "Catalog 文件无法读取"
        );
    }
}
#[cfg(test)]
mod path_tests {
    use super::*;
    fn check(config: Option<&str>, mode: AccountMode, valid: bool) -> Diagnostic {
        struct Metadata(bool);
        impl CatalogFs for Metadata {
            fn metadata(&self, _: &Path) -> io::Result<()> {
                if self.0 {
                    Ok(())
                } else {
                    Err(io::ErrorKind::PermissionDenied.into())
                }
            }
            fn read_to_string(&self, _: &Path) -> io::Result<String> {
                panic!()
            }
        }
        check_catalog_path_validity(&Metadata(valid), config, Path::new("/managed"), true, mode)
    }
    #[test]
    fn all_native_and_absent_path_branches() {
        assert_eq!(
            check(None, AccountMode::Native, false).title,
            "Catalog 路径（config 不可读，跳过）"
        );
        assert_eq!(
            check(Some(""), AccountMode::Native, false).severity,
            Severity::Ok
        );
        assert_eq!(
            check(Some("model_catalog_json=''"), AccountMode::Native, false).title,
            "真实账号仍被静态目录覆盖"
        );
        assert_eq!(
            check(Some("model_catalog_json=1"), AccountMode::NoAccount, false).severity,
            Severity::Warning
        );
    }
    #[test]
    fn managed_path_metadata_and_outside_fingerprints() {
        assert_eq!(
            check(
                Some("model_catalog_json='/managed/x'"),
                AccountMode::NoAccount,
                true
            )
            .severity,
            Severity::Ok
        );
        assert_eq!(
            check(
                Some("model_catalog_json='/managed/x'"),
                AccountMode::NoAccount,
                false
            )
            .detail
            .as_deref(),
            Some("路径：/managed/x")
        );
        for fingerprint in ["cc-switch", "cc_switch", "ccswitch", "codex-switch"] {
            let text = format!("model_catalog_json='/other/{fingerprint}/x'");
            assert_eq!(
                check(Some(&text), AccountMode::NoAccount, true).title,
                "model_catalog_json 指向第三方工具残留路径"
            );
        }
        assert_eq!(
            check(
                Some("model_catalog_json='/other/CC-SWITCH/x'"),
                AccountMode::NoAccount,
                true
            )
            .title,
            "model_catalog_json 指向非受管路径"
        );
        // Metadata receives the original path; normalization only affects containment comparison.
        assert_eq!(
            check(
                Some("model_catalog_json='/managed/../cc-switch/x'"),
                AccountMode::NoAccount,
                true
            )
            .severity,
            Severity::Ok
        );
    }
}
