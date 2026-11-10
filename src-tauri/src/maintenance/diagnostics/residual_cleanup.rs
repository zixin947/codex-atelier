//! Residual paths and route gate supplied by DSH's 12-residual清理.md.
//! Planning reads only; backup, unlink, verification and rollback live in repair.rs.
use super::*;

#[derive(Clone, Copy)]
pub(super) enum Kind {
    Manifest,
    Cache,
}
impl Kind {
    pub const ALL: [Self; 2] = [Self::Manifest, Self::Cache];
    pub fn id(self) -> &'static str {
        match self {
            Self::Manifest => "residual_manifest",
            Self::Cache => "residual_cache",
        }
    }
    pub fn label(self) -> &'static str {
        match self {
            Self::Manifest => "无残留迁移 manifest",
            Self::Cache => "无残留 relay-state-cache",
        }
    }
    pub fn paths(self, ctx: &Context) -> [PathBuf; 2] {
        let upstream = ctx.home.join("codexmate/relay");
        let atelier = ctx.app_root.join("gateway");
        match self {
            Self::Manifest => [
                upstream.join("thread-router-migrations/current.json"),
                atelier.join("migration-manifest.json"),
            ],
            Self::Cache => [
                upstream.join("relay-state-cache.json"),
                atelier.join("relay-state-cache.json"),
            ],
        }
    }
    pub fn existing(self, ctx: &Context) -> Result<Vec<(PathBuf, Vec<u8>)>> {
        let mut files = vec![];
        for path in self.paths(ctx) {
            match std::fs::symlink_metadata(&path) {
                Ok(_) => files.push((path.clone(), read_bounded(&path, inspect::LIMIT)?)),
                Err(e) if e.kind() == std::io::ErrorKind::NotFound => {}
                Err(_) => return Err("无法检查路由残留文件".into()),
            }
        }
        Ok(files)
    }
    pub fn can_delete(self, ctx: &Context, config: &toml::Value, path: &Path) -> Result<bool> {
        let [native, atelier] = self.paths(ctx);
        if path == native {
            return Ok(!super::route_mode::native(ctx)?.is_some_and(|m| m.enabled()));
        }
        if path == atelier {
            return Ok(!super::route_mode::atelier(ctx, config)?.enabled());
        }
        Err("该路径不属于已声明的路由残留文件".into())
    }
    pub fn repairable(
        self,
        ctx: &Context,
        config: &toml::Value,
    ) -> Result<Vec<(PathBuf, Vec<u8>)>> {
        self.existing(ctx)?
            .into_iter()
            .filter_map(|entry| match self.can_delete(ctx, config, &entry.0) {
                Ok(true) => Some(Ok(entry)),
                Ok(false) => None,
                Err(e) => Some(Err(e)),
            })
            .collect()
    }
}

// Persisted native state and Atelier intent are authoritative. Runtime traffic
// remains a separate write-window check in the confirmed repair entry.
pub(super) fn route_enabled(ctx: &Context, c: &toml::Value) -> Result<bool> {
    Ok(super::route_mode::resolve(ctx, c)?.enabled())
}
