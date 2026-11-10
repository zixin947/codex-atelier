//! Observed persisted ownership. A listener being alive is not routing intent.
use super::*;
use atelier_maintenance_core::{
    auth::RouterMode, catalog::state::RelayStateDocument, transition_journal::TransitionTarget,
};

pub(super) fn native(ctx: &Context) -> Result<Option<RouterMode>> {
    let path = ctx.home.join("codexmate/relay/state.json");
    let bytes = match std::fs::symlink_metadata(&path) {
        Ok(_) => read_bounded(&path, inspect::LIMIT)?,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(None),
        Err(_) => return Err("无法读取原生路由状态，未推断为关闭".into()),
    };
    let document =
        RelayStateDocument::from_json(&bytes).map_err(|_| "原生路由状态无效，未推断为关闭")?;
    let inputs = document
        .catalog_inputs()
        .map_err(|_| "原生路由模式字段无效，未推断为关闭")?;
    Ok(Some(if !inputs.router_enabled {
        RouterMode::Off
    } else if inputs.no_account_mode {
        RouterMode::NoAccount
    } else {
        RouterMode::Native
    }))
}
pub(super) fn atelier(ctx: &Context, config: &toml::Value) -> Result<RouterMode> {
    Ok(atelier_from_record(
        config,
        super::router_intent::read(ctx)?.as_ref(),
    ))
}
pub(super) fn atelier_from_record(
    config: &toml::Value,
    intent: Option<&super::router_intent::IntentRecord>,
) -> RouterMode {
    if let Some(intent) = intent {
        return if intent.target == TransitionTarget::RouterOn {
            RouterMode::Native
        } else {
            RouterMode::Off
        };
    }
    if super::router_intent::owns_provider(&inspect::provider(config)) {
        RouterMode::Native
    } else {
        RouterMode::Off
    }
}
pub(super) fn resolve(ctx: &Context, config: &toml::Value) -> Result<RouterMode> {
    if let Some(native) = native(ctx)? {
        if native.enabled() {
            return Ok(native);
        }
    }
    atelier(ctx, config)
}

#[cfg(test)]
#[path = "route_mode_tests.rs"]
mod tests;
