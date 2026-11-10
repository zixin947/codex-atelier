use super::*;
use crate::catalog::read_top_level_string_value as read;
pub use crate::catalog::{AccountMode, LegacyAlias, Provider, RouterState};
use std::{collections::HashSet, path::Path};
fn owned(id: &str) -> bool {
    id == "aimami_relay"
        || id.starts_with("aimami_relay_")
        || id == "aimai1"
        || id == "aimami_codex_router"
}
/// normalize_catalog.asm @1001f9bc8: trim whitespace, replace separators, trim trailing '/'；不折叠//。
pub fn normalize_catalog_path(s: &str) -> String {
    s.trim().replace('\\', "/").trim_end_matches('/').to_owned()
}
fn legacy_router_url(s: &str) -> bool {
    let Ok(u) = url::Url::parse(s.trim()) else {
        return false;
    };
    u.scheme() == "http"
        && u.host_str() == Some("127.0.0.1")
        && u.username().is_empty()
        && u.password().is_none()
        && u.path().trim_end_matches('/') == "/codex/router/v1"
        && u.query().is_none()
        && u.fragment().is_none()
        && u.port().is_some()
}
fn fallback_model(config: &str) -> Option<String> {
    let mut lex = Lexical(2);
    let mut section = false;
    for raw in config.lines() {
        if !lex.line_is_structural(raw) {
            continue;
        }
        let line = raw.trim();
        if line.is_empty() {
            continue;
        }
        if line.starts_with('[') && line.ends_with(']') {
            section = true;
            continue;
        }
        if section || line.starts_with('#') {
            continue;
        }
        if line.starts_with("model ") || line.starts_with("model=") {
            if let Some(v) = line.split('=').nth(1) {
                let v = v.trim().trim_matches('"').trim_matches('\'').trim();
                if !v.is_empty() {
                    return Some(v.to_owned());
                }
            }
        }
    }
    None
}
#[derive(Debug, Clone)]
pub struct RouterConfigSurface {
    pub provider: Option<String>,
    pub profile: Option<String>,
    pub model: Option<String>,
    pub catalog: Option<String>,
    pub own_catalog: bool,
    pub legacy_url: bool,
    pub provider_section: bool,
    pub endpoint: bool,
    pub requires_openai_auth: bool,
    pub owned_sections: Vec<String>,
}
impl RouterConfigSurface {
    /// RouterConfigSurface_parse.c @100362404。输入root由调用方明确提供，不发现真实home。
    pub fn parse(config: &str, root: &Path) -> Self {
        let provider = read(config, "model_provider");
        let profile = read(config, "profile");
        let model = read(config, "model").or_else(|| fallback_model(config));
        let catalog = read(config, "model_catalog_json");
        let own_catalog = catalog.as_deref().is_some_and(|s| {
            normalize_catalog_path(s)
                == normalize_catalog_path(&root.join("codex_router_catalog.json").to_string_lossy())
        });
        let legacy_url = read(config, "openai_base_url").is_some_and(|s| legacy_router_url(&s));
        let requires_openai_auth = toml::from_str::<toml::Value>(config)
            .ok()
            .and_then(|v| {
                v.get("model_providers")?
                    .get("aimai1")?
                    .get("requires_openai_auth")?
                    .as_bool()
            })
            .unwrap_or(false);
        let mut lex = Lexical(2);
        let mut provider_section = false;
        let mut current_provider = false;
        let mut endpoint = false;
        let mut owned_sections = vec![];
        for raw in config.lines() {
            if !lex.line_is_structural(raw) {
                continue;
            }
            let line = raw.trim();
            if let Some(inner) = line.strip_prefix('[').and_then(|s| s.strip_suffix(']')) {
                current_provider = inner == "model_providers.aimai1";
                provider_section |= current_provider;
                if let Some(rest) = inner
                    .strip_prefix("model_providers.")
                    .or_else(|| inner.strip_prefix("profiles."))
                {
                    let rest = rest.trim_matches('"');
                    if owned(rest) {
                        owned_sections.push(rest.to_owned());
                    }
                }
            } else if current_provider {
                endpoint |= raw.contains("/codex/router/v1");
            }
        }
        Self {
            provider,
            profile,
            model,
            catalog,
            own_catalog,
            legacy_url,
            provider_section,
            endpoint,
            requires_openai_auth,
            owned_sections,
        }
    }
    /// config_surface_violations_for_surface.c @100364750。顺序保留原生。
    pub fn violations(&self, route_enabled: bool, root: &Path) -> Vec<String> {
        let mut v = vec![];
        if !route_enabled {
            if let Some(p) = &self.provider {
                if owned(p) {
                    v.push(format!(
                        "NativeOff must not keep legacy tool top-level model_provider: {p}"
                    ));
                }
            }
            if let Some(p) = &self.profile {
                if owned(p) {
                    v.push(format!(
                        "NativeOff must not keep legacy tool top-level profile: {p}"
                    ));
                }
            }
            if self.legacy_url {
                v.push(
                    "NativeOff must not keep the legacy top-level legacy tool openai_base_url".into(),
                );
            }
            if self.own_catalog {
                v.push("NativeOff must not keep legacy tool top-level model_catalog_json".into());
            }
            if let Some(m) = &self.model {
                if m == "aimami_relay" || m.starts_with("aimami_relay_") {
                    v.push(format!(
                        "NativeOff must not keep relay model slug at top-level model: {m}"
                    ));
                }
            }
            for s in &self.owned_sections {
                v.push(format!(
                    "NativeOff must not keep legacy tool provider/profile section: {s}"
                ));
            }
        } else {
            if self.profile.is_some() {
                v.push("RouterOn must not keep a legacy top-level profile".into());
            }
            if self.legacy_url {
                v.push("RouterOn must not keep the legacy top-level legacy tool openai_base_url".into());
            }
            if self.provider.as_deref() != Some("aimai1") {
                v.push(format!(
                    "RouterOn requires top-level model_provider = \"aimai1\" (current: {:?})",
                    self.provider
                ));
            }
            if !(self.provider_section && self.endpoint) {
                v.push(format!("RouterOn requires complete aimai1 router entry (provider_section={} endpoint={})",self.provider_section,self.endpoint));
            }
            if self.requires_openai_auth {
                if self.catalog.is_some() {
                    v.push("RouterOn real-account mode requires native discovery, not model_catalog_json".into());
                }
            } else if self.catalog.is_none() {
                v.push("RouterOn requires top-level model_catalog_json".into());
            } else if !self.own_catalog {
                v.push(format!(
                    "RouterOn model_catalog_json should point to {} (current: {:?})",
                    root.join("codex_router_catalog.json").display(),
                    self.catalog
                ));
            }
            for s in &self.owned_sections {
                if s != "aimai1" {
                    v.push(format!(
                        "RouterOn must not keep legacy legacy tool provider/profile section: {s}"
                    ));
                }
            }
        }
        v
    }
}
/// check_config_stale.c @100372660；预期集合由实际Provider/active IDs/alias数据计算。
pub fn check_config_stale(
    config: Option<&str>,
    root: &Path,
    state: &RouterState,
) -> DiagnosticItem {
    const ID: &str = "config_stale";
    let Some(config) = config else {
        return ok(ID, "Codex 配置一致性（文件不可读，跳过）");
    };
    let surface = RouterConfigSurface::parse(config, root);
    let mut problems = surface.violations(state.enabled, root);
    let active: HashSet<String> = crate::catalog::expected_slugs(state).into_iter().collect();
    if let Some(model) = &surface.model {
        if top_level_relay_model_requires_cleanup(model, state.enabled, &active) {
            let p = if state.enabled {
                format!("RouterOn top-level relay model must exist in the active catalog: {model}")
            } else {
                format!("NativeOff must not keep relay model slug at top-level model: {model}")
            };
            if !problems.contains(&p) {
                problems.push(p);
            }
        }
    }
    if problems.is_empty() {
        ok(ID, "Codex 配置一致性正常")
    } else {
        item(
            ID,
            if state.enabled {
                Status::Error
            } else {
                Status::Warning
            },
            if state.enabled {
                "路由已启用但 config.toml 不符合 RouterOn 契约"
            } else {
                "路由已关闭但 config.toml 仍有生效的旧版工具路由配置"
            },
            Some(problems.join("；")),
            true,
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    fn state(enabled: bool, mode: AccountMode) -> RouterState {
        RouterState {
            enabled,
            mode,
            providers: vec![],
            active_provider_ids: vec![],
            legacy_aliases: vec![],
        }
    }
    fn check(text: &str, enabled: bool) -> DiagnosticItem {
        check_config_stale(
            Some(text),
            Path::new("/synthetic"),
            &state(enabled, AccountMode::Native),
        )
    }
    #[test]
    fn stale_absent_clean_off_and_missing_on() {
        assert_eq!(
            check_config_stale(
                None,
                Path::new("/synthetic"),
                &state(true, AccountMode::Native)
            )
            .status,
            Status::Ok
        );
        assert_eq!(
            check("model='gpt-5'\nmodel_provider='custom'", false).status,
            Status::Ok
        );
        let r = check("", true);
        assert_eq!(r.status, Status::Error);
        let d = r.detail.unwrap();
        assert!(d.contains("model_provider = \"aimai1\" (current: None)"));
        assert!(d.contains("provider_section=false endpoint=false"));
        assert!(d.contains("requires top-level model_catalog_json"));
    }
    #[test]
    fn native_real_account_contract_has_no_catalog_requirement() {
        let s="model_provider='aimai1'\n[model_providers.aimai1]\nbase_url='http://127.0.0.1:4321/codex/router/v1'\nrequires_openai_auth=true";
        assert_eq!(check(s, true).status, Status::Ok);
        let s = format!("model_catalog_json='/synthetic/codex_router_catalog.json'\n{s}");
        assert!(check(&s, true)
            .detail
            .unwrap()
            .contains("real-account mode requires native discovery"));
    }
    #[test]
    fn no_account_style_contract_requires_owned_catalog_path() {
        let tail="\nmodel_provider='aimai1'\n[model_providers.aimai1]\nbase_url='http://127.0.0.1:4321/codex/router/v1'\nrequires_openai_auth=false";
        assert_eq!(
            check(
                &format!("model_catalog_json='/synthetic/codex_router_catalog.json'{tail}"),
                true
            )
            .status,
            Status::Ok
        );
        assert!(check(
            &format!("model_catalog_json='/other/catalog.json'{tail}"),
            true
        )
        .detail
        .unwrap()
        .contains("should point to /synthetic/codex_router_catalog.json"));
    }
    #[test]
    fn off_reports_owned_keys_sections_and_deduplicates_model_cleanup() {
        let text="model_provider='aimami_relay_old'\nprofile='aimami_relay'\nmodel='aimami_relay_dead'\nmodel_catalog_json='/synthetic/codex_router_catalog.json'\nopenai_base_url='http://127.0.0.1:4321/codex/router/v1'\n[profiles.aimai1]\n[model_providers.aimami_relay_old]";
        let r = check(text, false);
        assert_eq!(r.status, Status::Warning);
        assert!(r.fixable);
        let d = r.detail.unwrap();
        assert_eq!(d.matches("relay model slug at top-level model").count(), 1);
        assert_eq!(d.split('；').count(), 7);
    }
    #[test]
    fn router_entry_presence_is_raw_structural_substring_not_url_validation() {
        let s="model_provider='aimai1'\n[model_providers.aimai1]\n# /codex/router/v1\nrequires_openai_auth=true";
        assert_eq!(check(s, true).status, Status::Ok);
        let s="model_provider='aimai1'\n[model_providers.\"aimai1\"]\nbase_url='http://127.0.0.1:4321/codex/router/v1'\nrequires_openai_auth=true";
        assert!(check(s, true)
            .detail
            .unwrap()
            .contains("provider_section=false endpoint=false"));
    }
    #[test]
    fn legacy_url_only_exact_local_http_contract() {
        assert!(legacy_router_url(
            " http://127.0.0.1:4321/codex/router/v1/// "
        ));
        for s in [
            "https://127.0.0.1:4321/codex/router/v1",
            "http://localhost:4321/codex/router/v1",
            "http://u@127.0.0.1:4321/codex/router/v1",
            "http://127.0.0.1:4321/codex/router/v1?x=1",
            "http://127.0.0.1/codex/router/v1",
        ] {
            assert!(!legacy_router_url(s), "{s}");
        }
    }
    #[test]
    fn fallback_invalid_model_line_is_used_and_legacy_sections_are_detected() {
        let s=RouterConfigSurface::parse("model=aimami_relay_old\n[profiles.\"aimami_relay_old\"]\n[model_providers.aimami_relay_old.child]",Path::new("/synthetic"));
        assert_eq!(s.model.as_deref(), Some("aimami_relay_old"));
        assert_eq!(
            s.owned_sections,
            vec!["aimami_relay_old", "aimami_relay_old.child"]
        );
    }
    #[test]
    fn catalog_normalization_is_distinct_and_provider_inventory_is_real_data() {
        assert_eq!(normalize_catalog_path(" /x\\y//z/// "), "/x/y//z");
        let mut st = state(true, AccountMode::NoAccount);
        st.providers = vec![Provider {
            id: "provider".into(),
            model: "fallback".into(),
            models: vec!["model".into()],
        }];
        st.active_provider_ids = vec!["provider".into()];
        st.legacy_aliases = vec![LegacyAlias {
            slot_slug: Some("gpt-5.6-sol".into()),
        }];
        let active = crate::catalog::expected_slugs(&st);
        assert!(active.contains("provider::model"));
        assert!(active.contains("gpt-5.6-sol"));
        assert!(!active.contains("provider::fallback"));
    }
}
