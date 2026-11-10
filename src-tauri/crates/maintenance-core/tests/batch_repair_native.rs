use atelier_maintenance_core::{
    batch_repair::{self as batch, *},
    manager::{DiagnosticItem, DiagnosticReport, FixResult},
    transition_journal::TransitionTarget,
};
use std::{collections::BTreeSet, fs, io, path::Path};
use tempfile::TempDir;
const PREPS: [ConfigPreparation; 4] = [
    ConfigPreparation::Syntax,
    ConfigPreparation::ThirdParty,
    ConfigPreparation::Profile,
    ConfigPreparation::Stale,
];
fn all_ids() -> Vec<&'static str> {
    let mut ids = vec![
        "auth_integrity",
        "router_unlock_auth_residue",
        "takeover_backup_orphan",
        "db_orphan_providers",
        "rollout_orphan_providers",
        "router_http_contract",
        "catalog_integrity",
        "catalog_path_validity",
        "router_postcondition",
    ];
    ids.extend(PREPS.map(ConfigPreparation::id));
    ids.extend(ORDINARY_ORDER.map(BatchItem::id));
    ids
}
struct Pool {
    auth: Option<Vec<u8>>,
}
struct Host {
    temp: TempDir,
    ids: Vec<&'static str>,
    target: TransitionTarget,
    calls: Vec<String>,
    auth: Option<atelier_maintenance_core::auth::AuthContext>,
}
impl Host {
    fn new(needs: &[&str]) -> Self {
        let temp = tempfile::tempdir().unwrap();
        let ids = all_ids();
        for id in &ids {
            fs::write(
                temp.path().join(id),
                if needs.contains(id) { "error" } else { "ok" },
            )
            .unwrap();
        }
        fs::write(temp.path().join("quiet"), "writers=0;guard=held").unwrap();
        Self {
            temp,
            ids,
            target: TransitionTarget::NativeOff,
            calls: vec![],
            auth: None,
        }
    }
    fn root(&self) -> &Path {
        self.temp.path()
    }
    fn set(&self, id: &str, status: &str) {
        fs::write(self.root().join(id), status).unwrap();
    }
    fn deny(&self, operation: &str) {
        fs::write(
            self.root().join(format!("deny-{operation}")),
            b"synthetic I/O failure",
        )
        .unwrap();
    }
    fn affected(&self, id: &str) -> bool {
        fs::read_to_string(self.root().join(id)).unwrap() == "ok"
    }
    fn error() -> io::Error {
        io::Error::new(io::ErrorKind::PermissionDenied, "synthetic refusal")
    }
}
impl BatchHost for Host {
    type Error = io::Error;
    type OfficialCatalog = Pool;
    fn diagnose(&mut self) -> io::Result<DiagnosticReport> {
        self.calls.push("diagnose".into());
        let mut items = vec![];
        for id in &self.ids {
            let status = if let Some(ctx) = &self.auth {
                match *id {
                    "auth_integrity" => {
                        if atelier_maintenance_core::auth::check_auth_integrity(ctx).level
                            == atelier_maintenance_core::auth::Level::Ok
                        {
                            "ok"
                        } else {
                            "error"
                        }
                        .to_string()
                    }
                    "router_unlock_auth_residue" => {
                        if atelier_maintenance_core::auth::unlock::check_router_unlock_auth_residue(
                            ctx,
                        ) == atelier_maintenance_core::auth::Level::Ok
                        {
                            "ok"
                        } else {
                            "warning"
                        }
                        .to_string()
                    }
                    _ => fs::read_to_string(self.root().join(id))?,
                }
            } else {
                fs::read_to_string(self.root().join(id))?
            };
            let fixable = !self.root().join(format!("readonly-{id}")).exists();
            items.push(DiagnosticItem {
                id: (*id).into(),
                status,
                label: (*id).into(),
                detail: String::new(),
                fixable,
            });
        }
        let has_issues = items.iter().any(|i| i.status != "ok");
        Ok(DiagnosticReport {
            items,
            has_issues,
            transition_in_flight: false,
        })
    }
    fn router_target(&self) -> TransitionTarget {
        self.target
    }
    fn ensure_quiet_window(&mut self) -> io::Result<()> {
        self.calls.push("quiet".into());
        if fs::read_to_string(self.root().join("quiet"))? != "writers=0;guard=held" {
            return Err(Self::error());
        }
        Ok(())
    }
    fn prepare_config(&mut self, prep: ConfigPreparation) -> io::Result<String> {
        self.calls.push(format!("prepare:{}", prep.id()));
        if self.root().join(format!("deny-{}", prep.id())).exists() {
            return Err(Self::error());
        }
        fs::write(self.root().join(prep.id()), "ok")?;
        Ok(format!("prepared {}", prep.id()))
    }
    fn resync_config(&mut self) -> io::Result<()> {
        self.calls.push("resync".into());
        if self.root().join("deny-resync").exists() {
            return Err(Self::error());
        }
        for id in [
            "router_http_contract",
            "catalog_integrity",
            "catalog_path_validity",
        ] {
            fs::write(self.root().join(id), "ok")?;
        }
        for id in &self.ids {
            if self.root().join(format!("keep-{id}")).exists() {
                fs::write(self.root().join(id), "warning")?;
            }
        }
        for id in &self.ids {
            if self.root().join(format!("readonly-after-{id}")).exists() {
                fs::write(self.root().join(format!("readonly-{id}")), b"")?;
            }
        }
        if self.root().join("new-provider-issue").exists() {
            fs::write(self.root().join("thread_provider_consistency"), "error")?;
        }
        if self.root().join("remove-report-file").exists() {
            fs::remove_file(self.root().join("residual_cache"))?;
        }
        Ok(())
    }
    fn dispatch(
        &mut self,
        item: BatchItem,
        pool: Option<&Pool>,
    ) -> Result<FixResult, ItemFailure<io::Error>> {
        self.calls.push(format!("fix:{}", item.id()));
        if let Some(ctx) = &self.auth {
            use atelier_maintenance_core::auth;
            if item == BatchItem::AuthIntegrity {
                let p = auth::plan_auth_integrity(ctx)
                    .map_err(|e| ItemFailure::new(io::Error::other(e)))?;
                let c = auth::apply_auth_integrity(ctx, &p)
                    .map_err(|e| ItemFailure::new(io::Error::other(e)))?;
                if c.level != auth::Level::Ok {
                    return Err(ItemFailure::new(Self::error()));
                }
                return Ok(FixResult {
                    fixed_ids: vec![item.id().into()],
                    failed_ids: vec![],
                    details: vec!["actual auth restored and checked".into()],
                });
            }
            if item == BatchItem::UnlockAuthResidue {
                let p = auth::unlock::plan_router_unlock_auth_residue(ctx)
                    .map_err(|e| ItemFailure::new(io::Error::other(e)))?;
                auth::unlock::apply_router_unlock_auth_residue(ctx, &p)
                    .map_err(|e| ItemFailure::new(io::Error::other(e)))?;
                return Ok(FixResult {
                    fixed_ids: vec![item.id().into()],
                    failed_ids: vec![],
                    details: vec!["actual fresh unlock cleanup".into()],
                });
            }
            if item == BatchItem::OrphanProviders {
                let now = fs::read(ctx.auth_path()).ok();
                if pool.is_some_and(|p| p.auth != now) {
                    return Err(ItemFailure::new(io::Error::other(
                        "prepared official auth source changed",
                    )));
                }
            }
        }
        if self.root().join(format!("deny-{}", item.id())).exists() {
            if self.root().join("partial-effect").exists() {
                fs::write(
                    self.root().join("partial-published"),
                    b"actual partial effect",
                )
                .unwrap();
                return Err(ItemFailure::from_partial(
                    Self::error(),
                    FixResult {
                        fixed_ids: vec!["earlier-reviewed-effect".into()],
                        failed_ids: vec![],
                        details: vec!["partial effect backup retained".into()],
                    },
                ));
            }
            return Err(ItemFailure::new(Self::error()));
        }
        let ids = if item == BatchItem::OrphanProviders {
            vec!["db_orphan_providers", "rollout_orphan_providers"]
        } else {
            vec![item.id()]
        };
        for id in &ids {
            fs::write(self.root().join(id), "ok").map_err(ItemFailure::new)?;
        }
        Ok(FixResult {
            fixed_ids: ids.into_iter().map(str::to_owned).collect(),
            failed_ids: vec![],
            details: vec![format!("applied {}", item.id())],
        })
    }
}
#[test]
fn exact_native_order_uses_one_initial_and_one_resync_report_not_input_order() {
    let mut ids = all_ids();
    ids.retain(|id| *id != "router_postcondition");
    ids.reverse();
    let mut host = Host::new(&ids);
    host.target = TransitionTarget::RouterOn;
    let out = batch::run(&mut host, None).unwrap();
    let mut expected = vec![
        "diagnose".into(),
        "quiet".into(),
        "fix:auth_integrity".into(),
        "fix:router_unlock_auth_residue".into(),
        "fix:takeover_backup_orphan".into(),
    ];
    expected.extend(PREPS.map(|p| format!("prepare:{}", p.id())));
    expected.extend([
        "resync".into(),
        "diagnose".into(),
        "fix:db_orphan_providers".into(),
    ]);
    expected.extend(ORDINARY_ORDER.map(|i| format!("fix:{}", i.id())));
    assert_eq!(host.calls, expected);
    assert!(out.fix.failed_ids.is_empty());
    for id in ids {
        assert!(host.affected(id));
        assert!(out.fix.fixed_ids.contains(&id.into()), "missing {id}");
    }
}
#[test]
fn no_fixable_issue_performs_no_writer_check_or_mutation() {
    let mut host = Host::new(&["auth_integrity"]);
    fs::write(host.root().join("readonly-auth_integrity"), b"").unwrap();
    fs::remove_file(host.root().join("quiet")).unwrap();
    let out = batch::run(&mut host, None).unwrap();
    assert_eq!(host.calls, ["diagnose"]);
    assert_eq!(out.fix.details, ["所有项目正常，无需修复"]);
    assert!(out.fix.fixed_ids.is_empty());
}
#[test]
fn actual_quiet_gate_failure_is_fatal_before_any_repair() {
    let mut host = Host::new(&["auth_integrity", "residual_cache"]);
    fs::remove_file(host.root().join("quiet")).unwrap();
    let failure = batch::run(&mut host, None).unwrap_err();
    assert_eq!(failure.stage, Stage::QuietWindow);
    assert!(failure.progress.fix.fixed_ids.is_empty());
    assert_eq!(host.calls, ["diagnose", "quiet"]);
    assert!(!host.affected("auth_integrity"));
}
#[test]
fn residual_only_batch_does_not_require_writer_quiet() {
    let mut host = Host::new(&["residual_manifest", "residual_cache"]);
    fs::remove_file(host.root().join("quiet")).unwrap();
    let out = batch::run(&mut host, None).unwrap();
    assert_eq!(
        host.calls,
        ["diagnose", "fix:residual_manifest", "fix:residual_cache"]
    );
    assert!(out.fix.failed_ids.is_empty());
}
#[test]
fn stale_text_off_does_not_resync_on_does() {
    for target in [TransitionTarget::NativeOff, TransitionTarget::RouterOn] {
        let mut host = Host::new(&["config_stale"]);
        host.target = target;
        let out = batch::run(&mut host, None).unwrap();
        assert_eq!(
            host.calls.contains(&"resync".into()),
            target == TransitionTarget::RouterOn
        );
        assert!(out.fix.fixed_ids.contains(&"config_stale".into()));
    }
}
#[test]
fn failed_preparation_continues_and_successful_other_preparation_still_resyncs() {
    let mut host = Host::new(&[
        "config_toml_syntax",
        "config_profile_conflict",
        "thread_response_item_ids",
    ]);
    host.deny("config_toml_syntax");
    let out = batch::run(&mut host, None).unwrap();
    assert!(out.fix.failed_ids.contains(&"config_toml_syntax".into()));
    assert!(out
        .fix
        .fixed_ids
        .contains(&"config_profile_conflict".into()));
    assert!(out
        .fix
        .fixed_ids
        .contains(&"thread_response_item_ids".into()));
    assert_eq!(out.deferred, ["config_profile_conflict"]);
}
#[test]
fn failed_resync_invalidates_only_deferred_successes_and_continues_orphan_and_threads() {
    let mut host = Host::new(&[
        "auth_integrity",
        "config_profile_conflict",
        "catalog_integrity",
        "db_orphan_providers",
        "thread_response_item_ids",
    ]);
    host.deny("resync");
    let out = batch::run(&mut host, None).unwrap();
    for id in ["config_profile_conflict", "catalog_integrity"] {
        assert!(out.fix.failed_ids.contains(&id.into()));
        assert!(!out.fix.fixed_ids.contains(&id.into()));
    }
    for id in [
        "auth_integrity",
        "db_orphan_providers",
        "thread_response_item_ids",
    ] {
        assert!(out.fix.fixed_ids.contains(&id.into()));
    }
    assert_eq!(
        host.calls
            .iter()
            .filter(|s| s.as_str() == "diagnose")
            .count(),
        1
    );
    assert!(out
        .fix
        .details
        .iter()
        .any(|s| s.starts_with("resync config 失败:")));
}
#[test]
fn post_resync_non_ok_is_failed_even_when_no_longer_fixable() {
    let mut host = Host::new(&["catalog_path_validity"]);
    fs::write(host.root().join("keep-catalog_path_validity"), b"").unwrap();
    fs::write(
        host.root().join("readonly-after-catalog_path_validity"),
        b"",
    )
    .unwrap();
    let out = batch::run(&mut host, None).unwrap();
    assert_eq!(out.fix.failed_ids, ["catalog_path_validity"]);
    assert!(!out.fix.fixed_ids.contains(&"catalog_path_validity".into()));
    assert!(out
        .fix
        .details
        .iter()
        .any(|s| s.ends_with("resync 后复查仍异常")));
}
#[test]
fn resync_does_not_create_an_extra_batch_for_newly_discovered_issues() {
    let mut host = Host::new(&["catalog_integrity"]);
    fs::write(host.root().join("new-provider-issue"), b"").unwrap();
    let out = batch::run(&mut host, None).unwrap();
    assert!(!out
        .initial_needs_fix
        .contains("thread_provider_consistency"));
    assert!(!host.affected("thread_provider_consistency"));
    assert!(!host
        .calls
        .contains(&"fix:thread_provider_consistency".into()));
    assert_eq!(
        host.calls
            .iter()
            .filter(|s| s.as_str() == "diagnose")
            .count(),
        2
    );
}
#[test]
fn ordinary_failure_keeps_partial_result_and_attempts_later_items_without_compensation() {
    let mut host = Host::new(&["thread_provider_consistency", "residual_cache"]);
    host.deny("thread_provider_consistency");
    fs::write(host.root().join("partial-effect"), b"").unwrap();
    let out = batch::run(&mut host, None).unwrap();
    assert!(host.root().join("partial-published").exists());
    assert!(out
        .fix
        .fixed_ids
        .contains(&"earlier-reviewed-effect".into()));
    assert!(out
        .fix
        .failed_ids
        .contains(&"thread_provider_consistency".into()));
    assert!(out.fix.fixed_ids.contains(&"residual_cache".into()));
    assert!(out
        .fix
        .details
        .contains(&"partial effect backup retained".into()));
}
#[test]
fn orphan_call_is_once_and_error_marks_only_initial_orphan_ids() {
    for ids in [
        vec!["rollout_orphan_providers"],
        vec!["db_orphan_providers", "rollout_orphan_providers"],
    ] {
        let mut host = Host::new(&ids);
        host.deny("db_orphan_providers");
        let out = batch::run(&mut host, None).unwrap();
        assert_eq!(
            host.calls
                .iter()
                .filter(|c| c.as_str() == "fix:db_orphan_providers")
                .count(),
            1
        );
        assert_eq!(
            out.fix
                .failed_ids
                .iter()
                .map(String::as_str)
                .collect::<BTreeSet<_>>(),
            ids.into_iter().collect()
        );
    }
}
#[test]
fn postcondition_is_owned_by_outer_manager_not_dispatch_loop() {
    let mut host = Host::new(&["router_postcondition"]);
    let out = batch::run(&mut host, None).unwrap();
    assert_eq!(host.calls, ["diagnose", "quiet"]);
    assert!(out.fix.fixed_ids.is_empty());
    assert!(out.initial_needs_fix.contains("router_postcondition"));
}
#[test]
fn fatal_second_diagnosis_preserves_actual_prior_publication() {
    let mut host = Host::new(&["auth_integrity", "config_profile_conflict"]);
    fs::write(host.root().join("remove-report-file"), b"").unwrap();
    let failure = batch::run(&mut host, None).unwrap_err();
    assert_eq!(failure.stage, Stage::ResyncDiagnosis);
    assert!(host.affected("auth_integrity"));
    assert!(failure
        .progress
        .fix
        .fixed_ids
        .contains(&"auth_integrity".into()));
}
#[test]
fn actual_auth_recovery_then_fresh_unlock_keeps_a_and_invalidates_prepared_catalog_source() {
    use atelier_maintenance_core::auth::{self, AuthContext, RouterMode};
    let mut host = Host::new(&[
        "auth_integrity",
        "router_unlock_auth_residue",
        "db_orphan_providers",
    ]);
    let context = AuthContext::new(host.root().join("synthetic-codex"), RouterMode::Off);
    fs::create_dir_all(context.auth_backup_dir()).unwrap();
    fs::create_dir_all(context.relay_dir()).unwrap();
    let a = b"{\"OPENAI_API_KEY\":\"sk-synthetic-A\"}";
    let b = b"{\"OPENAI_API_KEY\":\"sk-synthetic-B\"}";
    fs::write(context.auth_backup_dir().join("a.json"), a).unwrap();
    fs::write(auth::unlock::backup_path(&context), b).unwrap();
    fs::write(
        auth::unlock::marker_path(&context),
        "{\"version\":1,\"backedUpPreviousAuth\":true}",
    )
    .unwrap();
    let pool = Pool { auth: None };
    host.auth = Some(context.clone());
    let out = batch::run(&mut host, Some(&pool)).unwrap();
    assert_eq!(fs::read(context.auth_path()).unwrap(), a);
    assert!(!auth::unlock::backup_path(&context).exists());
    assert!(!auth::unlock::marker_path(&context).exists());
    assert!(out.fix.fixed_ids.contains(&"auth_integrity".into()));
    assert!(out
        .fix
        .fixed_ids
        .contains(&"router_unlock_auth_residue".into()));
    assert!(out.fix.failed_ids.contains(&"db_orphan_providers".into()));
    assert!(out
        .fix
        .details
        .iter()
        .any(|s| s.contains("prepared official auth source changed")));
}

#[test]
fn native_selection_uses_fixable_and_non_ok_including_unknown_and_skipped() {
    let mut host = Host::new(&[]);
    host.set("residual_cache", "unknown");
    host.set("residual_manifest", "skipped");
    let out = batch::run(&mut host, None).unwrap();
    assert_eq!(
        host.calls,
        ["diagnose", "fix:residual_manifest", "fix:residual_cache"]
    );
    assert_eq!(out.fix.fixed_ids, ["residual_manifest", "residual_cache"]);
}
#[test]
fn failed_preparation_alone_does_not_trigger_resync_or_stop_later_items() {
    let mut host = Host::new(&["config_third_party", "config_stale", "residual_cache"]);
    host.deny("config_third_party");
    let out = batch::run(&mut host, None).unwrap();
    assert!(!host.calls.contains(&"resync".into()));
    assert!(out.fix.failed_ids.contains(&"config_third_party".into()));
    assert!(out.fix.fixed_ids.contains(&"config_stale".into()));
    assert!(host.affected("residual_cache"));
}
