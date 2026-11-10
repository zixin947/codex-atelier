//! Explicit adapter for the recovered configuration checks. No path discovery.
use super::*;
use atelier_maintenance_core::config::{self, ConfigFs, Status};
use std::collections::HashSet;
#[cfg(test)]
#[path = "native_config_tests.rs"]
mod tests;

/// Native diagnostics builds this set from RelayState.providers, independently of
/// config.toml headers. Atelier's corresponding owner inventory is its real profiles.
pub(super) fn known_provider_ids_with_dependencies(
    ctx: &Context,
) -> Result<(HashSet<String>, Vec<(PathBuf, Option<(usize, String)>)>)> {
    use sha2::{Digest, Sha256};
    fn read(path: &Path) -> Result<Option<Vec<u8>>> {
        match std::fs::symlink_metadata(path) {
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(None),
            Err(e) => Err(err(e)),
            Ok(_) => read_bounded(path, 8 * 1024 * 1024).map(Some),
        }
    }
    fn dependency(path: PathBuf, bytes: Option<&[u8]>) -> (PathBuf, Option<(usize, String)>) {
        (
            path,
            bytes.map(|b| (b.len(), format!("{:x}", Sha256::digest(b)))),
        )
    }
    let native_path = ctx.home.join("codexmate/relay/state.json");
    let native_bytes = read(&native_path)?;
    let mut dependencies = vec![dependency(native_path, native_bytes.as_deref())];
    let config = super::inspect::config(ctx)?;
    let native_inputs = if let Some(bytes) = &native_bytes {
        let value: serde_json::Value = serde_json::from_slice(bytes).map_err(err)?;
        if value.get("profiles").is_some() || value.get("hostId").is_some() {
            return Err("Atelier目录不能作为原生Provider来源".into());
        }
        Some(
            atelier_maintenance_core::catalog::state::RelayStateDocument::from_json(bytes)
                .map_err(err)?
                .catalog_inputs()
                .map_err(err)?,
        )
    } else {
        None
    };
    if native_inputs
        .as_ref()
        .is_some_and(|input| input.router_enabled)
    {
        return Ok((
            native_inputs
                .expect("observed native source")
                .providers
                .into_iter()
                .map(|p| p.id)
                .collect(),
            dependencies,
        ));
    }
    let intent_path = super::router_intent::path(ctx);
    let intent_bytes = read(&intent_path)?;
    let intent = intent_bytes
        .as_deref()
        .map(super::router_intent::decode)
        .transpose()?;
    dependencies.push(dependency(intent_path, intent_bytes.as_deref()));
    let gateway_enabled =
        super::route_mode::atelier_from_record(&config, intent.as_ref()).enabled();
    if let Some(inputs) = native_inputs {
        if !gateway_enabled {
            return Ok((
                inputs.providers.into_iter().map(|p| p.id).collect(),
                dependencies,
            ));
        }
    }
    let gateway_path = ctx.app_root.join("gateway/catalog.json");
    let gateway_bytes = read(&gateway_path)?;
    dependencies.push(dependency(gateway_path, gateway_bytes.as_deref()));
    let Some(bytes) = gateway_bytes else {
        if native_bytes.is_some() {
            return Err("当前路由要求Atelier Provider来源，但实际目录不存在".into());
        }
        // The two absent inputs are dependencies, not an invented healthy catalog.
        return Ok((HashSet::new(), dependencies));
    };
    let catalog: crate::gateway::types::Catalog = serde_json::from_slice(&bytes).map_err(err)?;
    if catalog.host_id.is_empty() || catalog.profiles.len() > 64 {
        return Err("Atelier Provider来源超出实际支持范围".into());
    }
    Ok((
        catalog.profiles.into_iter().map(|p| p.id).collect(),
        dependencies,
    ))
}
pub(super) fn known_provider_ids(ctx: &Context) -> Result<HashSet<String>> {
    known_provider_ids_with_dependencies(ctx).map(|(ids, _)| ids)
}
pub(super) fn third_party(
    ctx: &Context,
    text: Option<&str>,
    writable: bool,
) -> Result<DiagnosticItem> {
    // The native unreadable-config branch returns before consulting owner IDs.
    let known = if text.is_some() {
        known_provider_ids(ctx)?
    } else {
        HashSet::new()
    };
    config::check_config_third_party(text, &known)
        .map(|item| adapt(item, writable))
        .map_err(err)
}

pub(super) struct Metadata;
impl ConfigFs for Metadata {
    fn metadata(&self, path: &Path) -> std::io::Result<()> {
        std::fs::metadata(path).map(|_| ())
    }
}

pub(super) fn profile(
    text: Option<&str>,
    route_enabled: bool,
    writable: bool,
) -> Result<DiagnosticItem> {
    config::check_config_profile_conflict(text, route_enabled)
        .map(|item| adapt(item, writable))
        .map_err(err)
}

pub(super) fn global(
    text: Option<&str>,
    managed_root: &Path,
    writable: bool,
) -> Result<DiagnosticItem> {
    config::check_config_global_validity(text, managed_root, &Metadata)
        .map(|item| adapt(item, writable))
        .map_err(err)
}

fn adapt(item: config::DiagnosticItem, writable: bool) -> DiagnosticItem {
    DiagnosticItem {
        id: item.id.into(),
        group: "config".into(),
        status: match item.status {
            Status::Ok => "ok",
            Status::Warning => "warning",
            Status::Error => "error",
        }
        .into(),
        label: item.label,
        detail: item.detail.unwrap_or_default(),
        fixable: item.fixable && writable,
    }
}
