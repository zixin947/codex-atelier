//! Catalog construction recovered from make_relay_model_info and prepare_router_catalog.
use serde_json::{json, Value};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WireApi {
    ChatCompletions,
    Responses,
    Anthropic,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Brand {
    Auto,
    DeepSeek,
    Kimi,
    MiniMax,
    MiMo,
    Glm,
    OpenRouter,
}
#[derive(Debug, Clone)]
pub struct ModelEntry {
    pub model: String,
    pub display_suffix: String,
    pub display_name: Option<String>,
    pub context_window: Option<i64>,
    pub wire_api: WireApi,
}
#[derive(Debug, Clone)]
pub struct CatalogProvider {
    pub id: String,
    pub name: String,
    pub base_url: String,
    pub model: String,
    pub models: Vec<ModelEntry>,
    pub wire_api: WireApi,
    pub brand: Brand,
}
impl CatalogProvider {
    pub fn exposed_models(&self) -> Vec<ModelEntry> {
        if !self.models.is_empty() {
            return self.models.clone();
        }
        vec![ModelEntry {
            model: self.model.clone(),
            display_suffix: String::new(),
            display_name: None,
            context_window: None,
            wire_api: self.wire_api,
        }]
    }
}
#[derive(Debug, Clone)]
pub struct CatalogAlias {
    pub provider_id: String,
    pub target_model: String,
    pub slot_slug: Option<String>,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CatalogBuildError(pub String);
impl std::fmt::Display for CatalogBuildError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}
impl std::error::Error for CatalogBuildError {}

fn tail(model: &str) -> String {
    model
        .trim()
        .rsplit('/')
        .next()
        .unwrap_or(model)
        .to_ascii_lowercase()
}
fn positive_integer(value: Option<&Value>) -> Option<i64> {
    value.and_then(Value::as_i64).filter(|n| *n > 0)
}
fn family(name: &str, f: &str) -> bool {
    name == f || name.strip_prefix(f).is_some_and(|s| s.starts_with('-'))
}

pub fn brand_for_model(model: &str) -> Brand {
    let name = tail(model);
    if name.starts_with("deepseek") {
        Brand::DeepSeek
    } else if name.starts_with("kimi") || name.starts_with("moonshot") {
        Brand::Kimi
    } else if name.starts_with("minimax") || name.starts_with("abab") {
        Brand::MiniMax
    } else if name.starts_with("mimo") {
        Brand::MiMo
    } else if name.starts_with("glm") {
        Brand::Glm
    } else {
        Brand::Auto
    }
}
fn resolved_brand(provider: &CatalogProvider, entry: &ModelEntry) -> Brand {
    if provider.brand == Brand::Auto {
        brand_for_model(&entry.model)
    } else {
        provider.brand
    }
}
pub fn rejects_hosted_web_search(provider: &CatalogProvider, entry: &ModelEntry) -> bool {
    if entry.wire_api != WireApi::Responses {
        return false;
    }
    let brand = resolved_brand(provider, entry);
    if matches!(brand, Brand::MiniMax | Brand::MiMo) {
        return true;
    }
    let name = tail(&entry.model);
    ["mimo", "minimax", "longcat", "qwen3-coder"]
        .iter()
        .any(|p| name.starts_with(p))
        || [
            "xiaomimimo.com",
            "longcat.chat",
            "minimax.io",
            "minimaxi.com",
        ]
        .iter()
        .any(|p| provider.base_url.to_ascii_lowercase().contains(p))
}
const MIMO_INSTRUCTIONS:&str="You are MiMo, an AI assistant developed by Xiaomi. Today's date: {date} {week}. Your knowledge cutoff date is December 2024.";
const MINIMAX_INSTRUCTIONS:&str="You are Codex, a coding agent based on MiniMax-M3. You and the user share the same workspace and collaborate to achieve the user's goals.";
fn levels(rows: &[(&str, &str)]) -> Value {
    Value::Array(
        rows.iter()
            .map(|(effort, description)| json!({"effort":effort,"description":description}))
            .collect(),
    )
}

/// All flags below are calculated from raw provider/model inputs, never caller verdicts.
pub fn make_relay_model_info(
    provider: &CatalogProvider,
    entry: &ModelEntry,
    template: Option<&Value>,
) -> Result<Value, CatalogBuildError> {
    make_relay_model_info_sources(provider, entry, template, template)
}
fn make_relay_model_info_sources(
    provider: &CatalogProvider,
    entry: &ModelEntry,
    template: Option<&Value>,
    reasoning_template: Option<&Value>,
) -> Result<Value, CatalogBuildError> {
    let name = tail(&entry.model);
    let brand = resolved_brand(provider, entry);
    let deepseek_flash = entry.wire_api == WireApi::Responses && name == "deepseek-v4-flash";
    let native = !deepseek_flash && !rejects_hosted_web_search(provider, entry);
    let anthropic = entry.wire_api == WireApi::Anthropic;
    let profile = if anthropic {
        anthropic_profile(&entry.model)?
    } else {
        None
    };
    let mimo = !native && !deepseek_flash && (brand == Brand::MiMo || name.starts_with("mimo"));
    let minimax = !native
        && !deepseek_flash
        && !mimo
        && (brand == Brand::MiniMax || name.starts_with("minimax"));
    let minimax_m3 = minimax && name == "minimax-m3";
    let image = native || (mimo && name == "mimo-v2.5") || minimax_m3;
    let native_responses = native && entry.wire_api == WireApi::Responses;
    let verbosity = native || deepseek_flash;
    let (default_reasoning, reasoning) = catalog_reasoning(
        provider,
        entry,
        !native,
        reasoning_template,
        profile.as_ref(),
    )?;
    let display = entry
        .display_name
        .as_deref()
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(str::to_owned)
        .unwrap_or_else(|| format!("{} / {}", provider.name, entry.model));
    let display = if entry.display_suffix.trim().is_empty() {
        display
    } else {
        format!("{display} {}", entry.display_suffix.trim())
    };
    let base = if mimo {
        MIMO_INSTRUCTIONS
    } else if minimax_m3 {
        MINIMAX_INSTRUCTIONS
    } else {
        ""
    };
    let mut object = json!({
        "slug":format!("{}::{}",provider.id,entry.model),
        "display_name":display,"description":format!("{} → {}",provider.name,entry.model),
        "default_reasoning_level":if anthropic && profile.is_none() {Value::Null} else {json!(default_reasoning)},
        "supported_reasoning_levels":reasoning,"shell_type":"shell_command","visibility":"list",
        "supported_in_api":true,"priority":100,"additional_speed_tiers":[],"availability_nux":null,"upgrade":null,
        "base_instructions":base,"supports_reasoning_summaries":entry.wire_api==WireApi::Responses || profile.is_some(),
        "default_reasoning_summary":"none","support_verbosity":verbosity,
        "truncation_policy":{"mode":if minimax_m3 {"bytes"} else {"tokens"},"limit":10000},
        "supports_parallel_tool_calls":native || deepseek_flash || minimax_m3,
        "supports_image_detail_original":image,"supports_search_tool":deepseek_flash || native_responses || anthropic,
        "effective_context_window_percent":if native_responses {positive_integer(template.and_then(|v|v.get("effective_context_window_percent"))).unwrap_or(95)} else {95},
        "experimental_supported_tools":[],"use_responses_lite":false,"service_tiers":[],
        "input_modalities":if image {vec!["text","image"]} else {vec!["text"]}
    });
    let out = object.as_object_mut().unwrap();
    if verbosity {
        out.insert("default_verbosity".into(), json!("low"));
    }
    if native || deepseek_flash {
        out.insert("apply_patch_tool_type".into(), json!("freeform"));
    }
    if deepseek_flash {
        out.insert("web_search_tool_type".into(), json!("text"));
        out.insert("prefer_websockets".into(), json!(false));
        out.insert("minimal_client_version".into(), json!("0.144.0"));
    } else if native_responses || anthropic {
        out.insert("web_search_tool_type".into(), json!("text_and_image"));
    }
    let context = context_windows(
        provider,
        entry,
        template,
        native_responses,
        mimo,
        minimax,
        deepseek_flash,
        anthropic,
    )?;
    out.insert("context_window".into(), json!(context.0));
    out.insert("max_context_window".into(), json!(context.1));
    if native_responses || deepseek_flash {
        if let Some(t) =
            template.filter(|t| t.get("slug").and_then(Value::as_str) == Some(entry.model.as_str()))
        {
            if let Some(base) = t
                .get("base_instructions")
                .and_then(Value::as_str)
                .filter(|s| !s.trim().is_empty())
            {
                out.insert("base_instructions".into(), json!(base));
            }
            if let Some(messages) = t
                .get("model_messages")
                .filter(|v| v.as_object().is_some_and(|o| !o.is_empty()))
            {
                out.insert("model_messages".into(), messages.clone());
            }
        }
    }
    Ok(object)
}
fn context_windows(
    provider: &CatalogProvider,
    entry: &ModelEntry,
    template: Option<&Value>,
    native_responses: bool,
    mimo: bool,
    minimax: bool,
    deepseek_flash: bool,
    anthropic: bool,
) -> Result<(i64, i64), CatalogBuildError> {
    let name = tail(&entry.model);
    let claude_million = [
        "claude-opus-4-6",
        "claude-opus-4-7",
        "claude-opus-4-8",
        "claude-opus-5",
        "claude-fable-5",
    ]
    .iter()
    .any(|f| family(&name, f));
    let gpt56 = family(&name, "gpt-5.6");
    let sonnet55 = if anthropic {
        model_matches_family(&normalize_anthropic(&entry.model), "sonnet-5-5")?
    } else {
        false
    };
    let mut result = if claude_million {
        (1_000_000, 1_000_000)
    } else if gpt56 || sonnet55 {
        if sonnet55 {
            (1_000_000, 1_000_000)
        } else {
            (372_000, 372_000)
        }
    } else if mimo || minimax || deepseek_flash {
        let n = if minimax { 1_000_000 } else { 1_048_576 };
        (n, n)
    } else if native_responses
        && positive_integer(template.and_then(|t| t.get("context_window"))).is_some()
    {
        let c = positive_integer(template.and_then(|t| t.get("context_window"))).unwrap();
        (
            c,
            positive_integer(template.and_then(|t| t.get("max_context_window"))).unwrap_or(c),
        )
    } else {
        match resolved_brand(provider, entry) {
            Brand::DeepSeek => (1_000_000, 1_000_000),
            Brand::Kimi => (262_144, 262_144),
            Brand::Glm => (200_000, 200_000),
            _ if anthropic => (200_000, 200_000),
            _ => (272_000, 1_000_000),
        }
    };
    if let Some(c) = entry.context_window.filter(|c| *c > 0) {
        result = (c, c);
    }
    Ok(result)
}

fn normalize_anthropic(model: &str) -> String {
    model.trim().to_ascii_lowercase().replace(['.', '_'], "-")
}
/// Family tokens have - / : @ boundaries. A numeric '-' suffix is a version only when
/// it is exactly an eight-digit date; e.g. opus-4 must not match opus-4-5.
pub fn model_matches_family(model: &str, wanted: &str) -> Result<bool, CatalogBuildError> {
    fn delimiter(c: u8) -> bool {
        matches!(c, b'-' | b'/' | b':' | b'@')
    }
    let b = model.as_bytes();
    for (start, _) in model.match_indices(wanted) {
        let end = start + wanted.len();
        if start > 0 && !delimiter(b[start - 1]) {
            continue;
        }
        if end < b.len() && !delimiter(b[end]) {
            continue;
        }
        if b.get(end) == Some(&b'-') {
            let segment = model[end + 1..]
                .split(['-', '/', '@', ':'])
                .next()
                .unwrap_or("");
            if segment.as_bytes().first().is_some_and(u8::is_ascii_digit)
                && !(segment.len() == 8 && segment.bytes().all(|c| c.is_ascii_digit()))
            {
                continue;
            }
        }
        return Ok(true);
    }
    Ok(false)
}
struct AnthropicProfile {
    default: &'static str,
    efforts: &'static [&'static str],
}
fn anthropic_profile(model: &str) -> Result<Option<AnthropicProfile>, CatalogBuildError> {
    const ALL: &[&str] = &["low", "medium", "high", "xhigh", "max"];
    const MAX: &[&str] = &["low", "medium", "high", "max"];
    const HIGH: &[&str] = &["low", "medium", "high"];
    const XHIGH: &[&str] = &["low", "medium", "high", "xhigh"];
    let name = normalize_anthropic(model);
    let matches = |names: &[&str]| -> Result<bool, CatalogBuildError> {
        for n in names {
            if model_matches_family(&name, n)? {
                return Ok(true);
            }
        }
        Ok(false)
    };
    let p = if matches(&[
        "sonnet-5-5",
        "fable-5-1",
        "mythos-5-1",
        "opus-5-5",
        "fable-5",
        "mythos-5",
        "opus-5",
        "sonnet-5",
        "opus-4-8",
        "opus-4-7",
    ])? {
        AnthropicProfile {
            default: if model_matches_family(&name, "opus-5-5")? {
                "medium"
            } else {
                "high"
            },
            efforts: ALL,
        }
    } else if matches(&["mythos-preview", "opus-4-6", "sonnet-4-6"])? {
        AnthropicProfile {
            default: "high",
            efforts: MAX,
        }
    } else if model_matches_family(&name, "opus-4-5")? {
        AnthropicProfile {
            default: "high",
            efforts: HIGH,
        }
    } else if matches(&[
        "3-7-sonnet",
        "sonnet-4",
        "sonnet-4-5",
        "haiku-4-5",
        "opus-4",
        "opus-4-1",
    ])? {
        AnthropicProfile {
            default: "medium",
            efforts: XHIGH,
        }
    } else {
        return Ok(None);
    };
    Ok(Some(p))
}
fn anthropic_effort_description(effort: &str) -> &'static str {
    match effort {
        "low" => "Fast responses with lighter reasoning",
        "medium" => "Balances speed and reasoning depth for everyday tasks",
        "high" => "Greater reasoning depth for complex problems",
        "xhigh" => "Extended reasoning for long-running coding and agentic tasks",
        "max" => "Maximum reasoning depth for the hardest tasks",
        _ => "Anthropic reasoning effort",
    }
}
fn catalog_reasoning(
    provider: &CatalogProvider,
    entry: &ModelEntry,
    special: bool,
    template: Option<&Value>,
    profile: Option<&AnthropicProfile>,
) -> Result<(String, Value), CatalogBuildError> {
    if entry.wire_api == WireApi::Responses && tail(&entry.model) == "deepseek-v4-flash" {
        return Ok((
            "high".into(),
            levels(&[
                ("low", "Fast responses with lighter reasoning"),
                ("high", "Extra high reasoning depth for complex problems"),
                ("max", "Maximum reasoning depth for the hardest problems"),
            ]),
        ));
    }
    if special {
        return Ok((
            "high".into(),
            levels(&[("none", "No reasoning"), ("high", "High reasoning effort")]),
        ));
    }
    if entry.wire_api == WireApi::Anthropic {
        return Ok(if let Some(p) = profile {
            (
                p.default.into(),
                levels(
                    &p.efforts
                        .iter()
                        .map(|e| (*e, anthropic_effort_description(e)))
                        .collect::<Vec<_>>(),
                ),
            )
        } else {
            ("none".into(), json!([]))
        });
    }
    if entry.wire_api == WireApi::Responses {
        if let Some(t) = template {
            if let (Some(default), Some(levels)) = (
                t.get("default_reasoning_level")
                    .and_then(Value::as_str)
                    .map(str::trim)
                    .filter(|s| !s.is_empty()),
                t.get("supported_reasoning_levels")
                    .and_then(Value::as_array)
                    .filter(|a| !a.is_empty()),
            ) {
                return Ok((default.into(), json!(levels)));
            }
        }
        return Ok((
            "medium".into(),
            levels(&[
                ("low", "Fast responses with lighter reasoning"),
                (
                    "medium",
                    "Balances speed and reasoning depth for everyday tasks",
                ),
                ("high", "Greater reasoning depth for complex problems"),
                ("xhigh", "Extra high reasoning depth for complex problems"),
            ]),
        ));
    }
    Ok(match resolved_brand(provider, entry) {
        Brand::DeepSeek => (
            "high".into(),
            levels(&[
                ("high", "DeepSeek high reasoning effort"),
                ("xhigh", "DeepSeek max reasoning effort"),
            ]),
        ),
        Brand::Auto | Brand::OpenRouter => (
            "medium".into(),
            levels(&[("medium", "Relay default reasoning level")]),
        ),
        _ => ("high".into(), levels(&[("high", "Relay thinking mode")])),
    })
}

pub fn validate_relay_provider_id(id: &str) -> Result<(), CatalogBuildError> {
    let invalid = || {
        CatalogBuildError(
            "relay provider id is not a portable filename or contains reserved slug syntax".into(),
        )
    };
    if id.is_empty() {
        return Err(CatalogBuildError(
            "relay provider id must not be empty".into(),
        ));
    }
    if id.len() > 128 {
        return Err(CatalogBuildError(
            "relay provider id exceeds 128 UTF-8 bytes".into(),
        ));
    }
    if matches!(id, "." | "..")
        || id.ends_with([' ', '.'])
        || id.chars().any(|c| {
            c <= '\u{1f}' || ('\u{7f}'..='\u{9f}').contains(&c) || "\"*/:<>?\\|".contains(c)
        })
    {
        return Err(invalid());
    }
    let stem = id.split('.').next().unwrap().to_ascii_uppercase();
    if matches!(stem.as_str(), "CON" | "PRN" | "AUX" | "NUL")
        || (stem.len() == 4
            && (stem.starts_with("COM") || stem.starts_with("LPT"))
            && (b'1'..=b'9').contains(&stem.as_bytes()[3]))
    {
        return Err(CatalogBuildError(
            "relay provider id uses a reserved Windows filename".into(),
        ));
    }
    Ok(())
}
pub fn validate_legacy_no_account_aliases(
    aliases: &[CatalogAlias],
) -> Result<(), CatalogBuildError> {
    let mut seen = std::collections::HashSet::new();
    for alias in aliases {
        if alias.target_model.trim().is_empty()
            || validate_relay_provider_id(&alias.provider_id).is_err()
            || alias
                .slot_slug
                .as_deref()
                .and_then(crate::catalog::canonical_no_account_slot_slug)
                .is_none()
            || !seen.insert(alias.slot_slug.as_deref().unwrap())
        {
            return Err(CatalogBuildError(
                "旧版无账号模型映射无效，已保留原记录，未重新分配模型。".into(),
            ));
        }
    }
    Ok(())
}
/// Construct model rows in active-provider order and exposed-model order. No model sort or
/// dedup is performed in the native loop. This is the prepare path with no upstream templates.
pub fn build_relay_models(
    providers: &[CatalogProvider],
    active_provider_ids: &[String],
) -> Result<Vec<Value>, CatalogBuildError> {
    let mut models = vec![];
    for id in active_provider_ids {
        let Some(provider) = providers.iter().find(|p| &p.id == id) else {
            continue;
        };
        for model in provider.exposed_models() {
            models.push(make_relay_model_info(provider, &model, None)?);
        }
    }
    Ok(models)
}
/// The content stage of native prepare_router_catalog. `native_account` is the original
/// param6; it is separate from route-enabled state. CLI-signature observation/publication
/// belongs to the outer adapter and is not silently invented here.
pub fn prepare_router_catalog(
    providers: &[CatalogProvider],
    active_provider_ids: &[String],
    native_account: bool,
    aliases: &[CatalogAlias],
) -> Result<Value, CatalogBuildError> {
    validate_legacy_no_account_aliases(aliases)?;
    let mut models = build_relay_models(providers, active_provider_ids)?;
    if !native_account {
        for alias in aliases {
            let fallback = CatalogProvider {
                id: alias.provider_id.clone(),
                name: alias.provider_id.clone(),
                base_url: String::new(),
                model: alias.target_model.clone(),
                models: vec![],
                wire_api: WireApi::ChatCompletions,
                brand: Brand::Auto,
            };
            let provider = providers
                .iter()
                .find(|p| p.id == alias.provider_id)
                .unwrap_or(&fallback);
            let model = provider
                .exposed_models()
                .into_iter()
                .find(|m| m.model == alias.target_model)
                .unwrap_or_else(|| fallback.exposed_models().remove(0));
            let mut row = make_relay_model_info(provider, &model, None)?;
            let object = row.as_object_mut().unwrap();
            object.insert("slug".into(), json!(alias.slot_slug));
            object.insert("visibility".into(), json!("hide"));
            object.insert("priority".into(), json!(1000));
            models.push(row);
        }
    }
    Ok(json!({"models":models,"aimami_no_account":!native_account}))
}
pub fn encode_prepared_catalog(document: &Value) -> Result<Vec<u8>, CatalogBuildError> {
    serde_json::to_vec_pretty(document)
        .map_err(|e| CatalogBuildError(format!("encode codex router catalog failed: {e}")))
}

#[cfg(test)]
mod tests {
    use super::*;
    pub(super) fn provider(model: &str, wire_api: WireApi) -> CatalogProvider {
        CatalogProvider {
            id: "p".into(),
            name: "Provider".into(),
            base_url: "https://example.invalid/v1".into(),
            model: model.into(),
            models: vec![],
            wire_api,
            brand: Brand::Auto,
        }
    }
    fn row(model: &str, wire: WireApi) -> Value {
        let p = provider(model, wire);
        make_relay_model_info(&p, &p.exposed_models()[0], None).unwrap()
    }
    #[test]
    fn constructor_fixed_types_and_native_defaults() {
        let v = row("gpt-other", WireApi::Responses);
        assert_eq!(v["slug"], "p::gpt-other");
        assert_eq!(v["display_name"], "Provider / gpt-other");
        assert_eq!(v["description"], "Provider → gpt-other");
        assert_eq!(
            v["truncation_policy"],
            json!({"mode":"tokens","limit":10000})
        );
        assert_eq!(v["supported_reasoning_levels"].as_array().unwrap().len(), 4);
        assert_eq!(
            v["supported_reasoning_levels"][0],
            json!({"effort":"low","description":"Fast responses with lighter reasoning"})
        );
        for key in [
            "supported_in_api",
            "supports_reasoning_summaries",
            "support_verbosity",
            "supports_parallel_tool_calls",
            "supports_image_detail_original",
            "supports_search_tool",
        ] {
            assert_eq!(v[key], true, "{key}");
        }
        for key in ["availability_nux", "upgrade"] {
            assert!(v[key].is_null());
        }
        for key in [
            "additional_speed_tiers",
            "experimental_supported_tools",
            "service_tiers",
        ] {
            assert_eq!(v[key], json!([]));
        }
        assert_eq!(v["context_window"], 272000);
        assert_eq!(v["max_context_window"], 1000000);
        assert_eq!(v["effective_context_window_percent"], 95);
        assert_eq!(v["input_modalities"], json!(["text", "image"]));
        assert_eq!(v["use_responses_lite"], false);
        assert!(v.get("tier").is_none());
        assert!(v.get("prefer_websockets").is_none());
        assert!(v.get("minimal_client_version").is_none());
    }
    #[test]
    fn model_capability_special_branches() {
        let deep = row("deepseek-v4-flash", WireApi::Responses);
        assert_eq!(deep["minimal_client_version"], "0.144.0");
        assert_eq!(deep["prefer_websockets"], false);
        assert_eq!(deep["web_search_tool_type"], "text");
        assert_eq!(deep["input_modalities"], json!(["text"]));
        assert_eq!(deep["supported_reasoning_levels"][2]["effort"], "max");
        for (m, image, c) in [
            ("mimo-v2.5", true, 1048576),
            ("mimo-other", false, 1048576),
            ("minimax-m3", true, 1000000),
            ("minimax-other", false, 1000000),
        ] {
            let v = row(m, WireApi::Responses);
            assert_eq!(v["supports_image_detail_original"], image);
            assert_eq!(v["context_window"], c);
            assert_eq!(v["supports_search_tool"], false);
            assert_eq!(v["support_verbosity"], false);
            assert_eq!(v["default_reasoning_level"], "high");
            assert_eq!(v["supported_reasoning_levels"][0]["effort"], "none");
        }
        let m = row("minimax-m3", WireApi::Responses);
        assert_eq!(m["truncation_policy"]["mode"], "bytes");
        assert_eq!(m["base_instructions"], MINIMAX_INSTRUCTIONS);
        let c = row("deepseek-v4-flash", WireApi::ChatCompletions);
        assert_eq!(c["supported_reasoning_levels"][1]["effort"], "xhigh");
        assert!(c.get("minimal_client_version").is_none());
    }
    #[test]
    fn brand_and_url_selection() {
        for (model, brand) in [
            (" org/DeepSeek-x ", Brand::DeepSeek),
            ("moonshot-x", Brand::Kimi),
            ("kimi", Brand::Kimi),
            ("abab", Brand::MiniMax),
            ("minimax", Brand::MiniMax),
            ("mimo", Brand::MiMo),
            ("glm-x", Brand::Glm),
            ("gpt-x", Brand::Auto),
        ] {
            assert_eq!(brand_for_model(model), brand);
        }
        let mut p = provider("ordinary", WireApi::Responses);
        p.base_url = "https://MINIMAXI.COM/v1".into();
        assert!(rejects_hosted_web_search(&p, &p.exposed_models()[0]));
        p.wire_api = WireApi::ChatCompletions;
        assert!(!rejects_hosted_web_search(&p, &p.exposed_models()[0]));
    }
    #[test]
    fn display_context_and_template_precedence() {
        let p = provider("gpt-other", WireApi::Responses);
        let mut e = p.exposed_models().remove(0);
        e.display_name = Some(" custom ".into());
        e.display_suffix = " suffix ".into();
        let template = json!({"slug":"gpt-other","context_window":100,"max_context_window":200,"effective_context_window_percent":88,"base_instructions":" instructions ","model_messages":{"x":1},"default_reasoning_level":" low ","supported_reasoning_levels":[{"effort":"low"}]});
        let v = make_relay_model_info(&p, &e, Some(&template)).unwrap();
        assert_eq!(v["display_name"], "custom suffix");
        assert_eq!(v["context_window"], 100);
        assert_eq!(v["max_context_window"], 200);
        assert_eq!(v["effective_context_window_percent"], 88);
        assert_eq!(v["base_instructions"], " instructions ");
        assert_eq!(v["model_messages"], json!({"x":1}));
        assert_eq!(v["default_reasoning_level"], "low");
        e.context_window = Some(999);
        let v = make_relay_model_info(&p, &e, Some(&template)).unwrap();
        assert_eq!(v["context_window"], 999);
        assert_eq!(v["max_context_window"], 999);
        e.model = "gpt-5.6-sol".into();
        e.context_window = None;
        let v = make_relay_model_info(&p, &e, Some(&template)).unwrap();
        assert_eq!(v["context_window"], 372000);
        assert!(v.get("model_messages").is_none());
    }
    #[test]
    fn anthropic_profiles_and_family_boundaries() {
        for (name, default, count) in [
            ("claude-sonnet-5.5", "high", 5),
            ("claude-opus-5.5", "medium", 5),
            ("claude-mythos-preview", "high", 4),
            ("claude-opus-4.5", "high", 3),
            ("claude-sonnet-4.5", "medium", 4),
            ("unknown", "none", 0),
        ] {
            let v = row(name, WireApi::Anthropic);
            assert_eq!(
                v["supported_reasoning_levels"].as_array().unwrap().len(),
                count,
                "{name}"
            );
            if count == 0 {
                assert!(v["default_reasoning_level"].is_null());
                assert_eq!(v["supports_reasoning_summaries"], false);
            } else {
                assert_eq!(v["default_reasoning_level"], default);
            }
        }
        for (name, f, want) in [
            ("xopus-4", "opus-4", false),
            ("claude-opus-4-5", "opus-4", false),
            ("claude-opus-4-20250101", "opus-4", true),
            ("claude-opus-4-high", "opus-4", true),
            ("claude-opus-4-123", "opus-4", false),
        ] {
            assert_eq!(model_matches_family(name, f).unwrap(), want);
        }
    }
    #[test]
    fn prepare_order_duplicates_and_alias_fallback() {
        let p = provider("z", WireApi::Responses);
        let a = CatalogAlias {
            provider_id: "missing".into(),
            target_model: "fallback".into(),
            slot_slug: Some("gpt-5.5".into()),
        };
        let doc = prepare_router_catalog(&[p], &["p".into(), "p".into()], false, &[a]).unwrap();
        assert_eq!(doc["aimami_no_account"], true);
        let m = doc["models"].as_array().unwrap();
        assert_eq!(m.len(), 3);
        assert_eq!(m[0]["slug"], "p::z");
        assert_eq!(m[1]["slug"], "p::z");
        assert_eq!(m[2]["slug"], "gpt-5.5");
        assert_eq!(m[2]["visibility"], "hide");
        assert_eq!(m[2]["priority"], 1000);
        assert_eq!(m[2]["description"], "missing → fallback");
        assert_eq!(
            serde_json::from_slice::<Value>(&encode_prepared_catalog(&doc).unwrap()).unwrap(),
            doc
        );
    }
    #[test]
    fn aliases_are_validated_even_when_unused() {
        let a = CatalogAlias {
            provider_id: "p".into(),
            target_model: "x".into(),
            slot_slug: Some("gpt-5.5".into()),
        };
        assert!(validate_legacy_no_account_aliases(&[a.clone(), a.clone()]).is_err());
        for id in [
            "", "..", "a:", "a/", "COM1", "nul.txt", "LPT9.txt", "tail.", "x\u{7f}",
        ] {
            assert!(validate_relay_provider_id(id).is_err(), "{id}");
        }
        for id in ["中文", "a b", "com0", "normal.id"] {
            assert!(validate_relay_provider_id(id).is_ok(), "{id}");
        }
        let mut a = a;
        a.target_model = " ".into();
        assert!(prepare_router_catalog(&[], &[], true, &[a]).is_err());
    }
}

/// Native nonempty-template branch: exact raw slug first; GPT-5.6 variants otherwise
/// borrow only reasoning fields from the longest matching normalized slug prefix.
/// Equal-length prefixes select the last candidate (native max_by_key fold).
pub fn build_relay_models_with_templates(
    providers: &[CatalogProvider],
    active_provider_ids: &[String],
    templates: &[Value],
) -> Result<Vec<Value>, CatalogBuildError> {
    let mut out = vec![];
    for id in active_provider_ids {
        let Some(provider) = providers.iter().find(|p| &p.id == id) else {
            continue;
        };
        for entry in provider.exposed_models() {
            let exact = templates
                .iter()
                .find(|t| t.get("slug").and_then(Value::as_str) == Some(entry.model.as_str()));
            let name = tail(&entry.model);
            let reasoning = if exact.is_some() {
                exact
            } else if family(&name, "gpt-5.6") {
                templates
                    .iter()
                    .filter_map(|t| {
                        let slug = t.get("slug")?.as_str()?.trim().to_ascii_lowercase();
                        (family(&tail(&slug), "gpt-5.6") && family(&name, &slug))
                            .then_some((slug.len(), t))
                    })
                    .max_by_key(|(len, _)| *len)
                    .map(|(_, t)| t)
            } else {
                None
            };
            out.push(make_relay_model_info_sources(
                provider, &entry, exact, reasoning,
            )?);
        }
    }
    Ok(out)
}
#[cfg(test)]
mod template_selection_tests {
    use super::*;
    #[test]
    fn longest_prefix_is_reasoning_only_and_last_tie_wins() {
        let p = super::tests::provider("gpt-5.6-sol-new", WireApi::Responses);
        let ts = vec![
            json!({"slug":"gpt-5.6","default_reasoning_level":"low","supported_reasoning_levels":[1]}),
            json!({"slug":"gpt-5.6-sol","default_reasoning_level":"high","supported_reasoning_levels":[2],"base_instructions":"do not copy","model_messages":{"x":1}}),
            json!({"slug":" GPT-5.6-SOL ","default_reasoning_level":"xhigh","supported_reasoning_levels":[3]}),
        ];
        let rows = build_relay_models_with_templates(&[p], &["p".into()], &ts).unwrap();
        assert_eq!(rows[0]["default_reasoning_level"], "xhigh");
        assert_eq!(rows[0]["supported_reasoning_levels"], json!([3]));
        assert_eq!(rows[0]["base_instructions"], "");
        assert!(rows[0].get("model_messages").is_none());
    }
}
