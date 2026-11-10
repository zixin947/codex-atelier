//! Explicit native-state and Atelier Responses-gateway catalog adapters.
//! No environment lookup, keychain access or application/process discovery.
use super::{err, DiagnosticItem, Result};
use crate::gateway::types::Catalog as GatewayCatalog;
use atelier_maintenance_core::catalog::{
    self as core,
    builder::{self, Brand, CatalogProvider, ModelEntry, WireApi},
    credentials::{self, CredentialStore, LegacyKeychain, StateMigrationPublication},
    publication::{self, AcknowledgedPublication, PreparedCatalog},
    state::{CatalogInputs, RelayStateDocument},
};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    fs, io,
    path::{Path, PathBuf},
};

/// CodexPaths::from_home @1008efb84 joins these three literal components.
pub(super) fn native_state_path(home: &Path) -> PathBuf {
    home.join("codexmate/relay/state.json")
}
#[derive(Clone, Debug)]
pub(super) enum Source {
    Native {
        state_path: PathBuf,
        catalog_root: PathBuf,
        bundle: Option<PathBuf>,
    },
    Gateway {
        catalog_path: PathBuf,
        catalog_root: PathBuf,
        bundle: Option<PathBuf>,
        route_enabled: bool,
    },
}
impl Source {
    fn input_path(&self) -> &Path {
        match self {
            Self::Native { state_path, .. } => state_path,
            Self::Gateway { catalog_path, .. } => catalog_path,
        }
    }
    fn root(&self) -> &Path {
        match self {
            Self::Native { catalog_root, .. } | Self::Gateway { catalog_root, .. } => catalog_root,
        }
    }
    fn bundle(&self) -> Option<&Path> {
        match self {
            Self::Native { bundle, .. } | Self::Gateway { bundle, .. } => bundle.as_deref(),
        }
    }
}
fn read_optional(path: &Path) -> Result<Option<Vec<u8>>> {
    match fs::symlink_metadata(path) {
        Err(e) if e.kind() == io::ErrorKind::NotFound => Ok(None),
        Err(e) => Err(err(e)),
        Ok(m) => {
            if !m.is_file() || m.file_type().is_symlink() {
                return Err("目录维护来源不是普通文件".into());
            }
            if m.len() > 8 * 1024 * 1024 {
                return Err("目录维护来源超过8MiB上限".into());
            }
            let data = fs::read(path).map_err(err)?;
            if data.len() > 8 * 1024 * 1024 {
                return Err("目录维护来源读取超过8MiB上限".into());
            }
            Ok(Some(data))
        }
    }
}
struct Loaded {
    original: Vec<u8>,
    inputs: CatalogInputs,
    native: Option<RelayStateDocument>,
    gateway_provider: Option<String>,
    revision: String,
}
fn load(source: &Source) -> Result<Loaded> {
    let original =
        read_optional(source.input_path())?.ok_or("目录维护来源不存在；未构造空状态替代")?;
    let revision = format!("{:x}", Sha256::digest(&original));
    match source {
        Source::Native { .. } => {
            let value: Value = serde_json::from_slice(&original).map_err(err)?;
            if value.get("profiles").is_some() || value.get("hostId").is_some() {
                return Err("该文件是 Atelier 网关目录，不能作为原生 RelayState 读取".into());
            }
            let native = RelayStateDocument::from_json(&original).map_err(err)?;
            let inputs = native.catalog_inputs().map_err(err)?;
            Ok(Loaded {
                original,
                inputs,
                native: Some(native),
                gateway_provider: None,
                revision,
            })
        }
        Source::Gateway { route_enabled, .. } => {
            let catalog: GatewayCatalog = serde_json::from_slice(&original)
                .map_err(|_| "Atelier 网关目录格式无效；未套用原生缺字段默认值")?;
            if catalog.profiles.len() > 64 || catalog.host_id.is_empty() {
                return Err("Atelier 网关目录超出实际支持范围".into());
            }
            let selected = catalog
                .profiles
                .iter()
                .find(|p| Some(&p.id) == catalog.settings.preferred_profile_id.as_ref());
            // /v1/models exposes exactly the selected profile's inventory (server.rs:284-296).
            // It does not merge the auto-switch candidates or filter transient auth/cooldown state.
            let selected = selected.filter(|p| !p.models.is_empty());
            let providers = selected
                .map(|p| {
                    vec![CatalogProvider {
                        id: p.id.clone(),
                        name: p.name.clone(),
                        base_url: p.base_url.clone(),
                        model: String::new(),
                        models: p
                            .models
                            .iter()
                            .map(|m| ModelEntry {
                                model: m.id.clone(),
                                display_name: Some(m.name.clone()),
                                display_suffix: String::new(),
                                context_window: None,
                                wire_api: WireApi::Responses,
                            })
                            .collect(),
                        wire_api: WireApi::Responses,
                        brand: Brand::Auto,
                    }]
                })
                .unwrap_or_default();
            let gateway_provider = selected.map(|p| p.id.clone());
            let active_provider_ids = gateway_provider.iter().cloned().collect();
            let inputs = CatalogInputs {
                providers,
                active_provider_ids,
                router_enabled: *route_enabled,
                no_account_mode: false,
                aliases: vec![],
            };
            Ok(Loaded {
                original,
                inputs,
                native: None,
                gateway_provider,
                revision,
            })
        }
    }
}
fn router_state(loaded: &Loaded) -> Result<core::RouterState> {
    if let Some(state) = &loaded.native {
        return state.diagnostic_state().map_err(err);
    }
    Ok(core::RouterState {
        enabled: loaded.inputs.router_enabled,
        mode: core::AccountMode::Native,
        providers: loaded
            .inputs
            .providers
            .iter()
            .map(|p| core::Provider {
                id: p.id.clone(),
                model: p.model.clone(),
                models: p.models.iter().map(|m| m.model.clone()).collect(),
            })
            .collect(),
        active_provider_ids: loaded.inputs.active_provider_ids.clone(),
        legacy_aliases: vec![],
    })
}
/// The gateway accepts raw configured model IDs, while native catalogs use provider::model.
/// Translate only the observed JSON slug inventory for the native integrity algorithm.
struct GatewayInventoryFs<'a> {
    provider: Option<&'a str>,
}
impl core::CatalogFs for GatewayInventoryFs<'_> {
    fn metadata(&self, path: &Path) -> io::Result<()> {
        fs::metadata(path).map(|_| ())
    }
    fn read_to_string(&self, path: &Path) -> io::Result<String> {
        let text = fs::read_to_string(path)?;
        let Some(provider) = self.provider else {
            return Ok(text);
        };
        let Ok(mut v) = serde_json::from_str::<Value>(&text) else {
            return Ok(text);
        };
        if let Some(models) = v.get_mut("models").and_then(Value::as_array_mut) {
            for model in models {
                if let Some(slug) = model.get("slug").and_then(Value::as_str) {
                    model["slug"] = json!(format!("{provider}::{slug}"));
                }
            }
        }
        serde_json::to_string(&v).map_err(io::Error::other)
    }
}
fn adapt(item: core::Diagnostic, writable: bool) -> DiagnosticItem {
    DiagnosticItem {
        id: item.id.into(),
        group: "config".into(),
        label: item.title,
        status: match item.severity {
            core::Severity::Ok => "ok",
            core::Severity::Warning => "warning",
            core::Severity::Error => "error",
        }
        .into(),
        detail: item.detail.unwrap_or_default(),
        fixable: item.fixable && writable,
    }
}
pub(super) fn inspect(
    source: &Source,
    config: Option<&str>,
    writable: bool,
) -> Result<Vec<DiagnosticItem>> {
    let loaded = load(source)?;
    let state = router_state(&loaded)?;
    let path = core::check_catalog_path_validity(
        &core::LocalFs,
        config,
        source.root(),
        state.enabled,
        if matches!(source, Source::Gateway { .. }) {
            // Atelier's current Responses consumer needs the published static inventory.
            core::AccountMode::NoAccount
        } else {
            state.mode
        },
    );
    let integrity = if matches!(source, Source::Gateway { .. }) {
        core::check_catalog_integrity(
            &GatewayInventoryFs {
                provider: loaded.gateway_provider.as_deref(),
            },
            source.root(),
            &state,
        )
    } else {
        core::check_catalog_integrity(&core::LocalFs, source.root(), &state)
    };
    let mut items = vec![adapt(path, writable), adapt(integrity, writable)];
    if matches!(source, Source::Gateway { .. }) {
        if !items[1].detail.is_empty() {
            items[1].detail.push(' ');
        }
        items[1]
            .detail
            .push_str("仅检查目录与配置inventory一致性，不代表整个路由已可生成。");
        if loaded.gateway_provider.is_none() {
            items[1].detail.push_str(" 当前没有可用preferred profile inventory；网关生成可用性由router_postcondition/Manager另行判断。");
        }
    }
    Ok(items)
}
#[derive(Clone)]
struct ConfigChange {
    path: PathBuf,
    before: Option<Vec<u8>>,
    after: Option<Vec<u8>>,
}
#[derive(Clone)]
pub(super) struct PreparedAction {
    source: Source,
    source_bytes: Vec<u8>,
    catalog_before: Option<Vec<u8>>,
    prepared: PreparedCatalog,
    pub revision: String,
    item_id: String,
    config: Option<ConfigChange>,
}
impl PreparedAction {
    pub fn item_id(&self) -> &str {
        &self.item_id
    }
    pub fn config_path(&self) -> Option<&Path> {
        self.config.as_ref().map(|c| c.path.as_path())
    }
    pub fn config_after(&self) -> Option<&[u8]> {
        self.config.as_ref().and_then(|c| c.after.as_deref())
    }
    pub fn document(&self) -> &Value {
        &self.prepared.document
    }
    pub fn destination(&self) -> PathBuf {
        self.source.root().join(core::CATALOG_FILE)
    }
}
pub(super) fn prepare(source: &Source) -> Result<PreparedAction> {
    let loaded = load(source)?;
    let mut prepared = if let Some(state) = &loaded.native {
        credentials::validate_credentials(state).map_err(err)?;
        publication::prepare_catalog(state, source.bundle()).map_err(err)?
    } else {
        let mut document = builder::prepare_router_catalog(
            &loaded.inputs.providers,
            &loaded.inputs.active_provider_ids,
            true,
            &[],
        )
        .map_err(err)?;
        // Preserve recovered descriptors, but publish only model IDs the actual gateway accepts.
        // Stable first occurrence matches the selected profile's order and removes duplicates.
        let mut seen = std::collections::HashSet::new();
        let models = document["models"]
            .as_array_mut()
            .ok_or("构造器没有返回models数组")?;
        for model in models.iter_mut() {
            let slug = model["slug"].as_str().ok_or("构造器没有返回slug")?;
            let prefix = format!("{}::", loaded.gateway_provider.as_deref().unwrap_or(""));
            let raw = slug
                .strip_prefix(&prefix)
                .ok_or("构造器slug与网关Provider不一致")?
                .to_owned();
            model["slug"] = json!(raw);
        }
        models.retain(|m| seen.insert(m["slug"].as_str().unwrap().to_owned()));
        PreparedCatalog {
            bytes: builder::encode_prepared_catalog(&document).map_err(err)?,
            document,
            cli_signature: publication::current_bundled_cli_signature(source.bundle()),
            native_account: true,
        }
    };
    // Keep bytes coupled to the final document even for gateway slug translation.
    prepared.bytes = builder::encode_prepared_catalog(&prepared.document).map_err(err)?;
    Ok(PreparedAction {
        source: source.clone(),
        source_bytes: loaded.original,
        catalog_before: read_optional(&source.root().join(core::CATALOG_FILE))?,
        prepared,
        revision: loaded.revision,
        item_id: "catalog_integrity".into(),
        config: None,
    })
}
pub(super) fn handles(item_id: &str) -> bool {
    matches!(item_id, "catalog_path_validity" | "catalog_integrity")
}
/// Validate and materialize both outputs before asking the upper repair driver to commit.
/// Unknown config text is parsed with toml_edit; all unrelated table/comments stay intact.
pub(super) fn prepare_item(
    source: &Source,
    config_path: &Path,
    item_id: &str,
) -> Result<PreparedAction> {
    if !handles(item_id) {
        return Err("该目录adapter只支持catalog路径和完整性修复".into());
    }
    let loaded = load(source)?;
    if !loaded.inputs.router_enabled {
        return Err("路由未启用，无需重写目录或配置".into());
    }
    let mut plan = prepare(source)?;
    if plan.source_bytes != loaded.original {
        return Err("目录来源在预览期间变化，请重新预览".into());
    }
    if config_path == source.input_path() || config_path == plan.destination() {
        return Err("配置与目录来源/输出不能是同一路径".into());
    }
    let before = read_optional(config_path)?;
    let text = before
        .as_deref()
        .map(std::str::from_utf8)
        .transpose()
        .map_err(|_| "配置不是UTF-8，未重写目录或配置")?
        .unwrap_or("");
    let mut doc = text
        .parse::<toml_edit::DocumentMut>()
        .map_err(|_| "TOML语法无效，未重写目录或配置")?;
    if loaded.native.is_some() && !loaded.inputs.no_account_mode {
        doc.remove("model_catalog_json");
    } else {
        let path = plan.destination();
        let path = path.to_str().ok_or("模型目录路径不是UTF-8")?;
        if !Path::new(path).is_absolute() {
            return Err("模型目录需要显式绝对路径".into());
        }
        // Native NoAccount and Atelier Responses consume the actual published file.
        doc["model_catalog_json"] = toml_edit::value(path);
    }
    let after = doc.to_string().into_bytes();
    let after = if before.is_none() && after.is_empty() {
        None
    } else {
        Some(after)
    };
    plan.config = Some(ConfigChange {
        path: config_path.to_owned(),
        before,
        after,
    });
    plan.item_id = item_id.into();
    validate(&plan)?;
    Ok(plan)
}
pub(super) fn validate(plan: &PreparedAction) -> Result<()> {
    if read_optional(plan.source.input_path())?.as_deref() != Some(plan.source_bytes.as_slice()) {
        return Err("目录来源revision已变化，请重新预览".into());
    }
    if read_optional(&plan.destination())? != plan.catalog_before {
        return Err("模型目录已变化，请重新预览".into());
    }
    if let Some(config) = &plan.config {
        if read_optional(&config.path)? != config.before {
            return Err("config.toml已变化，请重新预览".into());
        }
    }
    if publication::current_bundled_cli_signature(plan.source.bundle())
        != plan.prepared.cli_signature
    {
        return Err("捆绑CLI签名已变化，请重新预览".into());
    }
    Ok(())
}
#[derive(Clone, Debug)]
pub(super) struct Outcome {
    pub destination: PathBuf,
    pub directory_sync_error: Option<io::ErrorKind>,
    pub backup_path: PathBuf,
    pub config_publication: Option<AcknowledgedPublication>,
}
fn backup_before_apply(plan: &PreparedAction) -> Result<PathBuf> {
    let root = plan.source.root().join("maintenance-backups");
    crate::gateway::storage::private_dir(&root)?;
    let directory = root.join(format!("catalog-{}", uuid::Uuid::new_v4()));
    fs::create_dir(&directory).map_err(err)?;
    crate::gateway::storage::private_dir(&directory)?;
    let mut records = vec![
        (
            "source.json",
            plan.source.input_path().to_owned(),
            Some(plan.source_bytes.clone()),
        ),
        (
            "catalog.json.before",
            plan.destination(),
            plan.catalog_before.clone(),
        ),
    ];
    if let Some(config) = &plan.config {
        records.push((
            "config.toml.before",
            config.path.clone(),
            config.before.clone(),
        ));
    }
    let mut manifest = vec![];
    for (name, path, bytes) in records {
        if let Some(bytes) = &bytes {
            let backup = directory.join(name);
            publication::write_atomic_private_report(&backup, bytes)
                .map_err(err)?
                .require_sync()
                .map_err(err)?;
            if fs::read(&backup).map_err(err)? != *bytes {
                return Err("原始字节备份读回不一致，未提交修复".into());
            }
        }
        manifest.push(
            json!({"path":path,"backup":bytes.as_ref().map(|_|name),"present":bytes.is_some(),
            "sha256":bytes.as_ref().map(|b|format!("{:x}",Sha256::digest(b)))}),
        );
    }
    publication::write_atomic_private_report(
        &directory.join("manifest.json"),
        &serde_json::to_vec_pretty(&manifest).map_err(err)?,
    )
    .map_err(err)?
    .require_sync()
    .map_err(err)?;
    Ok(directory)
}
fn compensate_catalog(plan: &PreparedAction) -> Result<()> {
    if read_optional(&plan.destination())?.as_deref() != Some(plan.prepared.bytes.as_slice()) {
        return Err("已发布目录随后变化，保留外部修改与备份".into());
    }
    match &plan.catalog_before {
        Some(bytes) => {
            publication::write_atomic_report(&plan.destination(), bytes)
                .map_err(err)?
                .acknowledge();
        }
        None => fs::remove_file(plan.destination()).map_err(err)?,
    }
    if read_optional(&plan.destination())? != plan.catalog_before {
        return Err("目录回退读回不一致".into());
    }
    Ok(())
}
pub(super) fn apply(plan: PreparedAction) -> Result<Outcome> {
    // Use the exact same cross-process lock as gateway::Store::read/update.
    // Native Manager must hold its own state gate during validation/publication.
    let _gateway_lock = if let Source::Gateway { catalog_path, .. } = &plan.source {
        use fs2::FileExt;
        let parent = catalog_path.parent().ok_or("网关目录没有父路径")?;
        let lock = crate::gateway::storage::open_private(&parent.join("catalog.lock"))?;
        lock.try_lock_exclusive()
            .map_err(|_| "另一Atelier正在修改网关目录，请重新预览")?;
        Some(lock)
    } else {
        None
    };
    validate(&plan)?;
    let backup_path = backup_before_apply(&plan)?;
    // Backups precede every mutation; recheck all dependencies after making them.
    validate(&plan)?;
    let catalog =
        publication::write_prepared_catalog(plan.source.root(), &plan.prepared).map_err(err)?;
    if read_optional(&plan.destination())?.as_deref() != Some(plan.prepared.bytes.as_slice()) {
        return Err(format!(
            "模型目录发布后读回不一致；原始备份：{}",
            backup_path.display()
        ));
    }
    let config_publication = if let Some(config) = &plan.config {
        let commit = (|| -> Result<Option<AcknowledgedPublication>> {
            if read_optional(&config.path)? != config.before {
                return Err("配置在目录发布后变化".into());
            }
            if read_optional(plan.source.input_path())?.as_deref()
                != Some(plan.source_bytes.as_slice())
            {
                return Err("目录来源在发布后变化".into());
            }
            if config.before == config.after {
                return Ok(None);
            }
            let receipt = match &config.after {
                Some(after) => Some(
                    publication::write_atomic_report(&config.path, after)
                        .map_err(err)?
                        .acknowledge(),
                ),
                None => {
                    fs::remove_file(&config.path).map_err(err)?;
                    None
                }
            };
            if read_optional(&config.path)? != config.after {
                return Err("配置发布后读回不一致".into());
            }
            Ok(receipt)
        })();
        match commit {
            Ok(receipt) => receipt,
            Err(error) => {
                let rollback = match compensate_catalog(&plan) {
                    Ok(()) => "已恢复原模型目录".into(),
                    Err(e) => format!("目录恢复未完成：{e}"),
                };
                return Err(format!(
                    "{error}；{rollback}；原始备份：{}",
                    backup_path.display()
                ));
            }
        }
    } else {
        None
    };
    Ok(Outcome {
        destination: catalog.destination,
        directory_sync_error: catalog.directory_sync_error,
        backup_path,
        config_publication,
    })
}
/// Resolve only explicitly supplied Context roots; never discover an environment or bundle.
/// Context::native may pass its previously discovered bundle to the second entry point.
pub(super) fn context_source(ctx: &super::Context) -> Result<Source> {
    context_source_with_bundle(ctx, None)
}
pub(super) fn context_source_with_bundle(
    ctx: &super::Context,
    bundle: Option<&Path>,
) -> Result<Source> {
    let config = super::inspect::config(ctx)?;
    let native = Source::Native {
        state_path: native_state_path(&ctx.home),
        catalog_root: ctx.home.join("codexmate/relay"),
        bundle: bundle.map(Path::to_owned),
    };
    let native_present = match fs::symlink_metadata(native.input_path()) {
        Ok(_) => {
            let loaded = load(&native)?;
            if loaded.inputs.router_enabled {
                return Ok(native);
            }
            true
        }
        Err(e) if e.kind() == io::ErrorKind::NotFound => false,
        Err(e) => return Err(err(e)),
    };
    let gateway_enabled = super::route_mode::atelier(ctx, &config)?.enabled();
    if native_present && !gateway_enabled {
        return Ok(native);
    }
    let gateway = Source::Gateway {
        catalog_path: ctx.app_root.join("gateway/catalog.json"),
        catalog_root: ctx.home.clone(),
        bundle: bundle.map(Path::to_owned),
        route_enabled: gateway_enabled,
    };
    load(&gateway)?;
    Ok(gateway)
}
/// Separate explicit migration stage: legacy secret writes must precede state commit.
/// Gateway credentials use their own existing store and are never reinterpreted as native keys.
pub(super) fn migrate_native_source<K: LegacyKeychain>(
    source: &Source,
    store: &mut CredentialStore<K>,
) -> Result<StateMigrationPublication> {
    let Source::Native { state_path, .. } = source else {
        return Err("Atelier gateway凭据不属于原生key-store迁移".into());
    };
    let loaded = load(source)?;
    let mut state = loaded.native.ok_or("不是原生RelayState")?;
    credentials::migrate_and_publish_state(&mut state, &loaded.original, state_path, store)
        .map_err(err)
}

#[cfg(test)]
#[path = "native_catalog_tests.rs"]
mod tests;
