//! Native wire-field mapping backed by the model/state serializers and shipped relay UI.
//! The document retains fields outside catalog maintenance verbatim. It never loads secrets.
use crate::catalog::{self, builder};
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use std::{fs, io, path::Path};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StateError(pub String);
impl std::fmt::Display for StateError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}
impl std::error::Error for StateError {}
impl From<serde_json::Error> for StateError {
    fn from(e: serde_json::Error) -> Self {
        Self(e.to_string())
    }
}
impl From<io::Error> for StateError {
    fn from(e: io::Error) -> Self {
        Self(e.to_string())
    }
}
/// Native RouterOperationId is a canonical lowercase, hyphenated UUID with version 4.
/// Its native TryFrom checks the version nibble but imposes no UUID variant restriction.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct RouterOperationId(String);
impl RouterOperationId {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}
impl TryFrom<String> for RouterOperationId {
    type Error = StateError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        let bytes = value.as_bytes();
        if bytes.len() == 36
            && bytes[14] == b'4'
            && bytes.iter().enumerate().all(|(i, b)| {
                if matches!(i, 8 | 13 | 18 | 23) {
                    *b == b'-'
                } else {
                    b.is_ascii_digit() || (b'a'..=b'f').contains(b)
                }
            })
        {
            Ok(Self(value))
        } else {
            Err(StateError(
                "router operation id must be a canonical UUID v4".into(),
            ))
        }
    }
}
impl From<RouterOperationId> for String {
    fn from(value: RouterOperationId) -> Self {
        value.0
    }
}
#[derive(Debug, Clone, Copy, Default, Deserialize, PartialEq, Eq)]
pub enum NativeWireApi {
    #[default]
    #[serde(rename = "openai-chat")]
    Chat,
    #[serde(rename = "openai-responses")]
    Responses,
    #[serde(rename = "anthropic")]
    Anthropic,
}
impl From<NativeWireApi> for builder::WireApi {
    fn from(v: NativeWireApi) -> Self {
        match v {
            NativeWireApi::Chat => Self::ChatCompletions,
            NativeWireApi::Responses => Self::Responses,
            NativeWireApi::Anthropic => Self::Anthropic,
        }
    }
}
#[derive(Debug, Clone, Copy, Default, Deserialize, PartialEq, Eq)]
pub enum NativeBrand {
    #[default]
    #[serde(rename = "generic")]
    Generic,
    #[serde(rename = "deepseek")]
    DeepSeek,
    #[serde(rename = "kimi")]
    Kimi,
    #[serde(rename = "minimax")]
    MiniMax,
    #[serde(rename = "xiaomi")]
    Xiaomi,
    #[serde(rename = "zhipu")]
    Zhipu,
    #[serde(rename = "openrouter")]
    OpenRouter,
}
impl From<NativeBrand> for builder::Brand {
    fn from(v: NativeBrand) -> Self {
        match v {
            NativeBrand::Generic => Self::Auto,
            NativeBrand::DeepSeek => Self::DeepSeek,
            NativeBrand::Kimi => Self::Kimi,
            NativeBrand::MiniMax => Self::MiniMax,
            NativeBrand::Xiaomi => Self::MiMo,
            NativeBrand::Zhipu => Self::Glm,
            NativeBrand::OpenRouter => Self::OpenRouter,
        }
    }
}
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NativeModelEntry {
    pub model: String,
    pub wire_api: NativeWireApi,
    pub context_window: Option<i64>,
    pub display_name: Option<String>,
    #[serde(default)]
    pub suffix: String,
}
impl From<NativeModelEntry> for builder::ModelEntry {
    fn from(v: NativeModelEntry) -> Self {
        Self {
            model: v.model,
            wire_api: v.wire_api.into(),
            context_window: v.context_window,
            display_name: v.display_name,
            display_suffix: v.suffix,
        }
    }
}
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct NativeCatalogProvider {
    pub id: String,
    pub name: String,
    pub base_url: String,
    pub model: String,
    pub wire_api: NativeWireApi,
    pub brand: NativeBrand,
    pub models: Vec<NativeModelEntry>,
}
impl From<NativeCatalogProvider> for builder::CatalogProvider {
    fn from(v: NativeCatalogProvider) -> Self {
        Self {
            id: v.id,
            name: v.name,
            base_url: v.base_url,
            model: v.model,
            wire_api: v.wire_api.into(),
            brand: v.brand.into(),
            models: v.models.into_iter().map(Into::into).collect(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CodexNoAccountSlot {
    pub provider_id: String,
    pub model: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_slug: Option<String>,
}
impl From<CodexNoAccountSlot> for builder::CatalogAlias {
    fn from(v: CodexNoAccountSlot) -> Self {
        Self {
            provider_id: v.provider_id,
            target_model: v.model,
            slot_slug: v.catalog_slug,
        }
    }
}
#[derive(Debug)]
pub struct CatalogInputs {
    pub providers: Vec<builder::CatalogProvider>,
    pub active_provider_ids: Vec<String>,
    pub router_enabled: bool,
    pub no_account_mode: bool,
    pub aliases: Vec<builder::CatalogAlias>,
}
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct RelayProxyStatus {
    pub running: bool,
    pub port: u16,
    pub base_url: String,
    pub codex_base_url: String,
    pub last_error: Option<String>,
}
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct ProtocolMigrationCounts {
    pub minimax_m3: usize,
    pub deepseek_v4_flash: usize,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProviderIdMigration {
    pub old_id: String,
    pub new_id: String,
    pub provider_index: usize,
}

fn is_official_protocol_endpoint(base_url: &str, minimax: bool) -> bool {
    let Ok(url) = url::Url::parse(base_url.trim()) else {
        return false;
    };
    url.scheme() == "https"
        && url.username().is_empty()
        && url.password().is_none()
        && url.query().is_none()
        && url.fragment().is_none()
        && if minimax {
            matches!(url.host_str(), Some("api.minimax.io" | "api.minimaxi.com"))
        } else {
            url.host_str() == Some("api.deepseek.com")
        }
        && matches!(url.path().trim_end_matches('/'), "" | "/v1")
}
fn is_protocol_migration_model(model: &str, minimax: bool) -> bool {
    model.trim().rsplit('/').next().is_some_and(|tail| {
        tail.eq_ignore_ascii_case(if minimax {
            "minimax-m3"
        } else {
            "deepseek-v4-flash"
        })
    })
}
/// Retains opaque credential/proxy/health fields without interpreting or rewriting them.
/// Omitted top-level catalog fields use the native map visitor's defaults.
/// Defaults are applied to the projection; the retained document stays lossless.
#[derive(Debug, Clone)]
pub struct RelayStateDocument {
    raw: Value,
}
impl RelayStateDocument {
    /// Exact serialized shape of native storage::load's metadata-error default state.
    /// This constructor performs no I/O and does not silently replace an unreadable file.
    pub fn native_default() -> Self {
        Self {
            raw: serde_json::json!({
                "schemaVersion":16,
                "providers":[],
                "activeByIde":{"codex":[]},
                "proxy":{"running":false,"port":0,"baseUrl":"","codexBaseUrl":"","lastError":null},
                "codexRouterEnabled":false,
                "codexRouterNoAccountMode":false,
                "codexNoAccountSlots":[],
                "claudeWebSearchCompat":true
            }),
        }
    }

    pub fn from_json(bytes: &[u8]) -> Result<Self, StateError> {
        let mut document = Self::from_value(serde_json::from_slice(bytes)?)?;
        document.validate_schema_version()?;
        document.validate_storage_router_commit()?;
        document.proxy_status()?;
        document.claude_web_search_compat()?;
        document.normalize_legacy_active_codex();
        super::credentials::validate_credentials(&document)?;
        Ok(document)
    }
    pub fn from_value(value: Value) -> Result<Self, StateError> {
        if !value.is_object() {
            return Err(StateError("relay state must be an object".into()));
        }
        Ok(Self { raw: value })
    }
    pub fn read(path: &Path) -> Result<Self, StateError> {
        Self::from_json(&fs::read(path)?)
    }
    pub fn as_value(&self) -> &Value {
        &self.raw
    }
    pub fn to_pretty_json(&self) -> Result<Vec<u8>, StateError> {
        Ok(serde_json::to_vec_pretty(&self.raw)?)
    }
    pub fn schema_version(&self) -> Result<i32, StateError> {
        // Native RelayState's map visitor defaults to 16 and calls deserialize_i32.
        match self.raw.get("schemaVersion") {
            None => Ok(16),
            Some(value) => value
                .as_i64()
                .and_then(|v| i32::try_from(v).ok())
                .ok_or_else(|| StateError("schemaVersion must be a signed 32-bit integer".into())),
        }
    }
    pub fn validate_schema_version(&self) -> Result<(), StateError> {
        let version = self.schema_version()?;
        if version > 16 {
            return Err(StateError(format!("relay state schema v{version} is newer than supported v16; original file was preserved")));
        }
        Ok(())
    }
    fn validate_storage_router_commit(&self) -> Result<(), StateError> {
        let version = if self.raw.get("schemaVersion").is_none() {
            0
        } else {
            self.schema_version()?
        };
        if self.raw.get("routerCommitId").is_some_and(|v| !v.is_null()) && version <= 14 {
            return Err(StateError(
                "relay state router commit receipt is invalid; original file was preserved".into(),
            ));
        }
        self.router_commit_id()?;
        Ok(())
    }
    /// Verified pre-deserialization shape migration in storage::load, at every schema version.
    /// The method mutates only activeByIde.codex string/null. It does not upgrade schemaVersion.
    pub fn normalize_legacy_active_codex(&mut self) -> bool {
        let Some(active) = self
            .raw
            .get_mut("activeByIde")
            .and_then(Value::as_object_mut)
        else {
            return false;
        };
        let Some(codex) = active.get_mut("codex") else {
            return false;
        };
        match codex {
            Value::String(_) => {
                *codex = Value::Array(vec![codex.take()]);
                true
            }
            Value::Null => {
                *codex = Value::Array(vec![]);
                true
            }
            _ => false,
        }
    }
    /// Pure native protocol migrations. Does not write, upgrade the schema, or migrate secrets.
    /// Native's returned counts include changed model entries; with a nonempty model list,
    /// updating the fallback provider wireApi does not add to the count.
    pub fn migrate_legacy_official_protocols(
        &mut self,
    ) -> Result<ProtocolMigrationCounts, StateError> {
        self.validate_schema_version()?;
        let version = if self.raw.get("schemaVersion").is_none() {
            0
        } else {
            self.schema_version()?
        };
        let providers: Vec<NativeCatalogProvider> = self
            .raw
            .get("providers")
            .map(|v| serde_json::from_value(v.clone()))
            .transpose()?
            .unwrap_or_default();
        let mut counts = ProtocolMigrationCounts::default();
        let Some(raw_providers) = self.raw.get_mut("providers").and_then(Value::as_array_mut)
        else {
            return Ok(counts);
        };
        for (raw, provider) in raw_providers.iter_mut().zip(providers) {
            for minimax in [true, false] {
                if (minimax && version >= 10)
                    || (!minimax && version >= 13)
                    || !is_official_protocol_endpoint(&provider.base_url, minimax)
                {
                    continue;
                }
                let mut count = 0;
                if let Some(models) = raw.get_mut("models").and_then(Value::as_array_mut) {
                    for (model, typed) in models.iter_mut().zip(&provider.models) {
                        if typed.wire_api == NativeWireApi::Chat
                            && is_protocol_migration_model(&typed.model, minimax)
                        {
                            model
                                .as_object_mut()
                                .expect("typed model is object")
                                .insert("wireApi".into(), Value::String("openai-responses".into()));
                            count += 1;
                        }
                    }
                }
                if provider.wire_api == NativeWireApi::Chat
                    && is_protocol_migration_model(&provider.model, minimax)
                {
                    raw.as_object_mut()
                        .expect("typed provider is object")
                        .insert("wireApi".into(), Value::String("openai-responses".into()));
                    if provider.models.is_empty() {
                        count += 1;
                    }
                }
                if minimax {
                    counts.minimax_m3 += count;
                } else {
                    counts.deepseek_v4_flash += count;
                }
            }
        }
        Ok(counts)
    }
    /// Native load runs this for source schema <14. It validates provider/model membership,
    /// deduplicates pairs, canonicalizes reserved aliases and assigns the first unused alias.
    /// If no valid old slot remains, no-account mode generates slots in active/model order.
    pub fn migrate_legacy_no_account_slots(&mut self) -> Result<bool, StateError> {
        self.validate_schema_version()?;
        let version = if self.raw.get("schemaVersion").is_none() {
            0
        } else {
            self.schema_version()?
        };
        if version >= 14 {
            return Ok(false);
        }
        let inputs = self.catalog_inputs()?;
        let original: Vec<CodexNoAccountSlot> = self
            .raw
            .get("codexNoAccountSlots")
            .map(|v| serde_json::from_value(v.clone()))
            .transpose()?
            .unwrap_or_default();
        const SLUGS: [&str; 5] = [
            "gpt-5.6-sol",
            "gpt-5.6-terra",
            "gpt-5.6-luna",
            "gpt-5.5",
            "gpt-5.4",
        ];
        let mut slots = Vec::new();
        let mut pairs = std::collections::HashSet::new();
        let mut used = std::collections::HashSet::new();
        for mut slot in original.iter().cloned() {
            if slots.len() == 5 {
                break;
            }
            let Some(provider) = inputs.providers.iter().find(|p| p.id == slot.provider_id) else {
                continue;
            };
            if !inputs.active_provider_ids.contains(&provider.id)
                || !provider
                    .exposed_models()
                    .iter()
                    .any(|m| m.model == slot.model)
                || !pairs.insert((slot.provider_id.clone(), slot.model.clone()))
            {
                continue;
            }
            slot.catalog_slug = slot
                .catalog_slug
                .as_deref()
                .and_then(catalog::canonical_no_account_slot_slug)
                .filter(|s| used.insert((*s).to_owned()))
                .map(str::to_owned);
            slots.push(slot);
        }
        for slot in &mut slots {
            if slot.catalog_slug.is_none() {
                let slug = SLUGS
                    .iter()
                    .find(|s| !used.contains(**s))
                    .expect("at most five slots");
                used.insert((*slug).to_owned());
                slot.catalog_slug = Some((*slug).into());
            }
        }
        if slots.is_empty() && inputs.no_account_mode {
            'providers: for id in &inputs.active_provider_ids {
                let Some(provider) = inputs.providers.iter().find(|p| p.id == *id) else {
                    continue;
                };
                for model in provider.exposed_models() {
                    if slots.len() == 5 {
                        break 'providers;
                    }
                    if pairs.insert((provider.id.clone(), model.model.clone())) {
                        slots.push(CodexNoAccountSlot {
                            provider_id: provider.id.clone(),
                            model: model.model,
                            catalog_slug: Some(SLUGS[slots.len()].into()),
                        });
                    }
                }
            }
        }
        if slots == original {
            return Ok(false);
        }
        self.raw
            .as_object_mut()
            .expect("document is object")
            .insert("codexNoAccountSlots".into(), serde_json::to_value(slots)?);
        Ok(true)
    }
    pub fn plan_provider_id_migrations(&self) -> Result<Vec<ProviderIdMigration>, StateError> {
        use sha2::{Digest, Sha256};
        self.validate_schema_version()?;
        let providers: Vec<NativeCatalogProvider> = self
            .raw
            .get("providers")
            .map(|v| serde_json::from_value(v.clone()))
            .transpose()?
            .unwrap_or_default();
        let mut seen = std::collections::HashSet::new();
        let mut reserved = std::collections::HashSet::new();
        for p in &providers {
            if !seen.insert(p.id.clone()) {
                return Err(StateError(format!(
                    "duplicate relay provider id: {:?}",
                    p.id
                )));
            }
            if builder::validate_relay_provider_id(&p.id).is_ok() {
                reserved.insert(p.id.clone());
            }
        }
        let mut plan = Vec::new();
        for (index, p) in providers.iter().enumerate() {
            if builder::validate_relay_provider_id(&p.id).is_ok() {
                continue;
            }
            let digest = Sha256::digest(p.id.as_bytes());
            let hex: String = digest[..8].iter().map(|b| format!("{b:02x}")).collect();
            let base = format!("aimami_relay_legacy_{hex}");
            let mut candidate = base.clone();
            let mut suffix = 1u32;
            while reserved.contains(&candidate) {
                candidate = format!("{base}_{suffix}");
                suffix += 1;
            }
            reserved.insert(candidate.clone());
            plan.push(ProviderIdMigration {
                old_id: p.id.clone(),
                new_id: candidate,
                provider_index: index,
            });
        }
        Ok(plan)
    }
    pub fn catalog_inputs(&self) -> Result<CatalogInputs, StateError> {
        self.validate_schema_version()?;
        self.router_commit_id()?;
        #[derive(Deserialize)]
        #[serde(rename_all = "camelCase")]
        struct Fields {
            #[serde(default)]
            providers: Vec<NativeCatalogProvider>,
            #[serde(default)]
            active_by_ide: Active,
            #[serde(default)]
            codex_router_enabled: bool,
            #[serde(default)]
            codex_router_no_account_mode: bool,
            #[serde(default)]
            codex_no_account_slots: Vec<CodexNoAccountSlot>,
        }
        #[derive(Default, Deserialize)]
        struct Active {
            #[serde(default)]
            codex: Vec<String>,
        }
        let fields: Fields = serde_json::from_value(self.raw.clone())?;
        Ok(CatalogInputs {
            providers: fields.providers.into_iter().map(Into::into).collect(),
            active_provider_ids: fields.active_by_ide.codex,
            router_enabled: fields.codex_router_enabled,
            no_account_mode: fields.codex_router_no_account_mode,
            aliases: fields
                .codex_no_account_slots
                .into_iter()
                .map(Into::into)
                .collect(),
        })
    }
    pub fn diagnostic_state(&self) -> Result<catalog::RouterState, StateError> {
        let inputs = self.catalog_inputs()?;
        Ok(catalog::RouterState {
            enabled: inputs.router_enabled,
            mode: if inputs.no_account_mode {
                catalog::AccountMode::NoAccount
            } else {
                catalog::AccountMode::Native
            },
            providers: inputs
                .providers
                .into_iter()
                .map(|p| catalog::Provider {
                    id: p.id,
                    model: p.model,
                    models: p.models.into_iter().map(|m| m.model).collect(),
                })
                .collect(),
            active_provider_ids: inputs.active_provider_ids,
            legacy_aliases: inputs
                .aliases
                .into_iter()
                .map(|a| catalog::LegacyAlias {
                    slot_slug: a.slot_slug,
                })
                .collect(),
        })
    }
    pub fn set_router_flags(&mut self, enabled: bool, no_account: bool) {
        let map = self
            .raw
            .as_object_mut()
            .expect("document is validated as object");
        map.insert("codexRouterEnabled".into(), Value::Bool(enabled));
        map.insert("codexRouterNoAccountMode".into(), Value::Bool(no_account));
    }
    pub fn set_active_codex_provider_ids(&mut self, ids: Vec<String>) -> Result<(), StateError> {
        let object = self
            .raw
            .get_mut("activeByIde")
            .and_then(Value::as_object_mut)
            .ok_or_else(|| StateError("activeByIde must be an object".into()))?;
        object.insert("codex".into(), serde_json::to_value(ids)?);
        Ok(())
    }
    pub fn router_commit_id(&self) -> Result<Option<RouterOperationId>, StateError> {
        match self.raw.get("routerCommitId") {
            None | Some(Value::Null) => Ok(None),
            Some(value) => Ok(Some(serde_json::from_value(value.clone())?)),
        }
    }
    pub fn set_router_commit_id(&mut self, commit: Option<RouterOperationId>) {
        let map = self
            .raw
            .as_object_mut()
            .expect("document is validated as object");
        match commit {
            Some(v) => {
                map.insert("routerCommitId".into(), Value::String(v.into()));
            }
            None => {
                map.remove("routerCommitId");
            }
        }
    }
    pub fn opaque_fields(&self) -> &Map<String, Value> {
        self.raw
            .as_object()
            .expect("document is validated as object")
    }
    pub fn proxy_status(&self) -> Result<RelayProxyStatus, StateError> {
        match self.raw.get("proxy") {
            None => Ok(RelayProxyStatus::default()),
            Some(value) => Ok(serde_json::from_value(value.clone())?),
        }
    }
    pub fn claude_web_search_compat(&self) -> Result<bool, StateError> {
        match self.raw.get("claudeWebSearchCompat") {
            None => Ok(true),
            Some(Value::Bool(value)) => Ok(*value),
            Some(_) => Err(StateError("claudeWebSearchCompat must be a boolean".into())),
        }
    }
    /// Updates only known native proxy fields and retains extension fields.
    pub fn set_proxy_status(&mut self, proxy: RelayProxyStatus) -> Result<(), StateError> {
        let map = self
            .raw
            .as_object_mut()
            .expect("document is validated as object");
        let raw_proxy = map
            .entry("proxy")
            .or_insert_with(|| Value::Object(Map::new()));
        let proxy_map = raw_proxy
            .as_object_mut()
            .ok_or_else(|| StateError("proxy must be an object".into()))?;
        proxy_map.extend(
            serde_json::to_value(proxy)?
                .as_object()
                .expect("proxy serializes as object")
                .clone(),
        );
        Ok(())
    }
}
