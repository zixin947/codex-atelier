//! Atelier-owned routing intent. Gateway data is never decoded as native RelayState.
use super::*;
use crate::gateway::types::Catalog;
use atelier_maintenance_core::{
    catalog::{
        publication::{write_atomic_private_report, Publication},
        state::RouterOperationId,
    },
    transition_journal::TransitionTarget,
};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub(super) struct IntentRecord {
    pub schema_version: u8,
    pub target: TransitionTarget,
    pub provider: Option<String>,
    pub operation_id: Option<RouterOperationId>,
    pub catalog_revision: Option<String>,
    pub updated_at_ms: i64,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum Origin {
    Persisted,
    ObservedConfiguration,
}
#[derive(Clone)]
pub(super) struct Observation {
    pub record: IntentRecord,
    pub origin: Origin,
    pub catalog: Option<Catalog>,
}
#[derive(Clone)]
pub(super) struct Prepared {
    home: PathBuf,
    app_root: PathBuf,
    path: PathBuf,
    before: Option<Vec<u8>>,
    config_hash: Option<String>,
    catalog_hash: Option<String>,
    observation: Observation,
}
pub(super) struct IntentPublication {
    pub publication: Publication,
    pub record: IntentRecord,
}
pub(super) fn path(ctx: &Context) -> PathBuf {
    ctx.app_root.join("gateway/router-intent.json")
}
pub(super) fn owns_provider(provider: &str) -> bool {
    provider == "atelier" || provider == "codex_atelier" || provider.starts_with("atelier_")
}
fn hash(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}
fn optional(path: &Path, limit: u64) -> Result<Option<Vec<u8>>> {
    match std::fs::symlink_metadata(path) {
        Ok(_) => read_bounded(path, limit).map(Some),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(None),
        Err(_) => Err("无法核对路由意图依赖文件".into()),
    }
}
fn revision(bytes: &Option<Vec<u8>>) -> Option<String> {
    bytes.as_deref().map(hash)
}
pub(super) fn decode(bytes: &[u8]) -> Result<IntentRecord> {
    let record: IntentRecord =
        serde_json::from_slice(bytes).map_err(|_| "Atelier 路由意图格式损坏，未替换原文件")?;
    if record.schema_version != 1 {
        return Err("不支持的 Atelier 路由意图版本，原文件保留".into());
    }
    if record.catalog_revision.as_ref().is_some_and(|h| {
        h.len() != 64
            || !h
                .bytes()
                .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
    }) {
        return Err("Atelier 路由意图来源摘要格式无效".into());
    }
    match (record.target, record.provider.as_deref()) {
        (TransitionTarget::RouterOn, Some(provider)) if owns_provider(provider) => {}
        (TransitionTarget::NativeOff, None) => {}
        _ => return Err("Atelier 路由意图与 Provider 归属不一致".into()),
    }
    Ok(record)
}
pub(super) fn read(ctx: &Context) -> Result<Option<IntentRecord>> {
    optional(&path(ctx), 1024 * 1024)?
        .as_deref()
        .map(decode)
        .transpose()
}
fn config_provider(bytes: Option<&[u8]>, allow_rescue: bool) -> Result<String> {
    let text = std::str::from_utf8(bytes.unwrap_or_default())
        .map_err(|_| "配置不是 UTF-8，无法判断路由归属")?;
    match toml::from_str::<toml::Value>(text) {
        Ok(value) => Ok(super::inspect::provider(&value)),
        Err(_) if allow_rescue => Ok(
            atelier_maintenance_core::catalog::read_top_level_string_value(text, "model_provider")
                .unwrap_or_else(|| "openai".into()),
        ),
        Err(_) => Err("最终配置不是有效 TOML，未提交路由意图".into()),
    }
}
/// Initial target comes from effective Desktop configuration, never from a running
/// gateway alone. A persisted intent stays authoritative until a reviewed repair.
pub(super) fn prepare(ctx: &Context) -> Result<Prepared> {
    let path = path(ctx);
    let before = optional(&path, 1024 * 1024)?;
    let config = optional(&ctx.config(), 1024 * 1024)?;
    let raw_catalog = optional(&ctx.app_root.join("gateway/catalog.json"), 8 * 1024 * 1024)?;
    let catalog = raw_catalog
        .as_deref()
        .map(|bytes| {
            serde_json::from_slice::<Catalog>(bytes)
                .map_err(|_| "Atelier 网关目录损坏，未将它当作原生状态或空目录")
        })
        .transpose()?;
    let catalog_hash = revision(&raw_catalog);
    let (record, origin) = if let Some(bytes) = &before {
        (decode(bytes)?, Origin::Persisted)
    } else {
        let provider = config_provider(config.as_deref(), true)?;
        let owned = owns_provider(&provider);
        (
            IntentRecord {
                schema_version: 1,
                target: if owned {
                    TransitionTarget::RouterOn
                } else {
                    TransitionTarget::NativeOff
                },
                provider: owned.then_some(provider),
                operation_id: None,
                catalog_revision: catalog_hash.clone(),
                updated_at_ms: 0,
            },
            Origin::ObservedConfiguration,
        )
    };
    Ok(Prepared {
        home: ctx.home.clone(),
        app_root: ctx.app_root.clone(),
        path,
        before,
        config_hash: revision(&config),
        catalog_hash,
        observation: Observation {
            record,
            origin,
            catalog,
        },
    })
}
impl Prepared {
    pub(super) fn observation(&self) -> &Observation {
        &self.observation
    }
    pub(super) fn destination(&self) -> &Path {
        &self.path
    }
}
/// Only invoked after confirmation, under the maintenance mutation locks. Exercise
/// the actual private atomic publication and directory-sync boundary without
/// replacing the durable intent or touching Desktop's configuration.
pub(super) fn preflight(ctx: &Context, plan: &Prepared) -> Result<()> {
    validate(ctx, plan)?;
    let parent = plan.path.parent().ok_or("路由意图目录无效")?;
    crate::gateway::storage::private_dir(parent)?;
    if plan.before.is_some() {
        std::fs::OpenOptions::new()
            .write(true)
            .open(&plan.path)
            .map_err(|_| "路由意图文件不可写")?;
    }
    let probe = parent.join(format!(
        ".maintenance-preflight-{}.json",
        uuid::Uuid::new_v4()
    ));
    let bytes = serde_json::to_vec_pretty(&plan.observation.record).map_err(err)?;
    let result = write_atomic_private_report(&probe, &bytes)
        .and_then(Publication::require_sync)
        .map_err(err);
    let cleanup = match std::fs::remove_file(&probe) {
        Ok(()) => Ok(()),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(()),
        Err(e) => Err(format!("路由意图预检临时文件清理失败：{e}")),
    };
    result?;
    cleanup
}
fn validate_sources(ctx: &Context, plan: &Prepared) -> Result<()> {
    if ctx.home != plan.home || ctx.app_root != plan.app_root {
        return Err("路由意图目标目录已变化".into());
    }
    if optional(&plan.path, 1024 * 1024)? != plan.before {
        return Err("路由意图已被外部更新，请重新预览".into());
    }
    let raw = optional(&ctx.app_root.join("gateway/catalog.json"), 8 * 1024 * 1024)?;
    if revision(&raw) != plan.catalog_hash {
        return Err("网关来源目录已变化，请重新预览".into());
    }
    Ok(())
}
pub(super) fn validate(ctx: &Context, plan: &Prepared) -> Result<()> {
    validate_sources(ctx, plan)?;
    if revision(&optional(&ctx.config(), 1024 * 1024)?) != plan.config_hash {
        return Err("路由意图的配置来源已变化，请重新预览".into());
    }
    Ok(())
}
pub(super) fn commit(
    ctx: &Context,
    plan: &Prepared,
    operation: &RouterOperationId,
    now_ms: i64,
) -> Result<IntentPublication> {
    validate(ctx, plan)?;
    let config = optional(&ctx.config(), 1024 * 1024)?;
    commit_after_approved_config(ctx, plan, config.as_deref(), operation, now_ms)
}
/// The parent transaction has already approved these final config bytes. Verify
/// the actual bytes and target ownership; no external validity boolean is accepted.
pub(super) fn commit_after_approved_config(
    ctx: &Context,
    plan: &Prepared,
    config_after: Option<&[u8]>,
    operation: &RouterOperationId,
    now_ms: i64,
) -> Result<IntentPublication> {
    validate_sources(ctx, plan)?;
    if optional(&ctx.config(), 1024 * 1024)?.as_deref() != config_after {
        return Err("最终配置不等于已批准效果，未提交路由意图".into());
    }
    let provider = config_provider(config_after, false)?;
    match plan.observation.record.target {
        TransitionTarget::RouterOn
            if plan.observation.record.provider.as_deref() != Some(provider.as_str()) =>
        {
            return Err("最终配置未进入获批的 Atelier Provider，未提交路由意图".into())
        }
        TransitionTarget::NativeOff if owns_provider(&provider) => {
            return Err("最终配置仍由 Atelier 接管，不能标记为已退出".into())
        }
        _ => {}
    }
    let mut record = plan.observation.record.clone();
    record.operation_id = Some(operation.clone());
    record.catalog_revision = plan.catalog_hash.clone();
    record.updated_at_ms = now_ms;
    let bytes = serde_json::to_vec_pretty(&record).map_err(|_| "路由意图编码失败")?;
    let publication = write_atomic_private_report(&plan.path, &bytes).map_err(err)?;
    verify_receipt(ctx, &record)?;
    Ok(IntentPublication {
        publication,
        record,
    })
}
pub(super) fn verify_receipt(ctx: &Context, expected: &IntentRecord) -> Result<()> {
    let actual = read(ctx)?.ok_or("Atelier 路由提交回执不存在，保留恢复记录")?;
    if &actual != expected {
        return Err("Atelier 路由提交回执冲突，保留恢复记录".into());
    }
    Ok(())
}
