//! Repair gates recovered from arm64 0x100682818 and its two iterator predicates.
//! This module computes gates; it neither performs nor claims to perform a repair.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QuietWindow {
    NotRequired,
    Required,
    Unknown,
}

/// Native 0x10037d594..0x10037dc84: 0=no, 1=yes, 2=unknown.
/// The shared XOR/OR blocks return 1 on an exact match, 2 otherwise.
pub fn quiet_window(id: &str) -> QuietWindow {
    match id {
        "residual_cache" | "residual_manifest" => QuietWindow::NotRequired,
        "config_stale"
        | "auth_integrity"
        | "catalog_integrity"
        | "config_toml_syntax"
        | "config_third_party"
        | "db_orphan_providers"
        | "state_db_relocation"
        | "router_postcondition"
        | "router_http_contract"
        | "catalog_path_validity"
        | "takeover_backup_orphan"
        | "config_global_validity"
        | "config_profile_conflict"
        | "thread_missing_rollouts"
        | "legacy_migrated_threads"
        | "thread_response_item_ids"
        | "router_unlock_auth_residue"
        | "thread_poisoned_instructions"
        | "rollout_orphan_providers"
        | "project_state_integrity"
        | "thread_model_catalog_consistency"
        | "thread_provider_consistency" => QuietWindow::Required,
        _ => QuietWindow::Unknown,
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Item<'a> {
    pub id: &'a str,
    pub status: &'a str,
    pub fixable: bool,
}
impl Item<'_> {
    fn needs_fix(self) -> bool {
        self.fixable && self.status != "ok"
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RepairGates {
    pub prepare_catalog: bool,
    pub preflight_state_write: bool,
    pub use_quiet_transition: bool,
    pub reconcile_after_fix: bool,
    pub direct_postcondition: bool,
    pub include_missing_rollouts: bool,
}

/// Derives gates from the actual report, not caller-supplied validity decisions.
/// A prepared-catalog failure has separate all/single handling in the native driver;
/// this function intentionally does not turn that failure into success.
pub fn repair_gates(selected: &str, report: &[Item<'_>]) -> Result<RepairGates, String> {
    if selected != "all" && quiet_window(selected) == QuietWindow::Unknown {
        return Err(format!(
            "unknown or non-fixable diagnostic item: {selected}"
        ));
    }
    let all = selected == "all";
    let direct_postcondition = selected == "router_postcondition";
    let use_quiet_transition = !direct_postcondition
        && if all {
            // 0x10070b1cc: fixable, status != ok, requires_quiet_window == 1.
            report
                .iter()
                .any(|i| i.needs_fix() && quiet_window(i.id) == QuietWindow::Required)
        } else {
            quiet_window(selected) == QuietWindow::Required
        };
    Ok(RepairGates {
        // 0x100682b88..0x100682d18 -> 0x100683244. This provider gate is
        // independent of the postcondition predicate at 0x10070b0fc.
        prepare_catalog: report.iter().any(|i| {
            (all || i.id == selected)
                && i.needs_fix()
                && matches!(i.id, "db_orphan_providers" | "rollout_orphan_providers")
        }),
        preflight_state_write: use_quiet_transition,
        use_quiet_transition,
        reconcile_after_fix: direct_postcondition
            || (all
                && report
                    .iter()
                    .any(|i| i.needs_fix() && i.id == "router_postcondition")),
        direct_postcondition,
        include_missing_rollouts: all || selected == "thread_missing_rollouts",
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    fn issue(id: &str) -> Item<'_> {
        Item {
            id,
            status: "error",
            fixable: true,
        }
    }
    #[test]
    fn shared_native_switch_arms_are_required() {
        for id in [
            "rollout_orphan_providers",
            "project_state_integrity",
            "thread_model_catalog_consistency",
            "thread_provider_consistency",
        ] {
            assert_eq!(quiet_window(id), QuietWindow::Required, "{id}");
            for altered in [format!("{id}x"), id.to_uppercase()] {
                assert_eq!(quiet_window(&altered), QuietWindow::Unknown);
            }
        }
    }
    #[test]
    fn plain_repairs_do_not_always_prepare_or_reconcile() {
        let g = repair_gates("config_stale", &[issue("config_stale")]).unwrap();
        assert!(g.use_quiet_transition && g.preflight_state_write);
        assert!(!g.prepare_catalog && !g.reconcile_after_fix);
        let g = repair_gates("residual_cache", &[issue("residual_cache")]).unwrap();
        assert!(!g.use_quiet_transition && !g.preflight_state_write);
    }
    #[test]
    fn provider_preparation_and_postcondition_are_independent() {
        let g = repair_gates("all", &[issue("db_orphan_providers")]).unwrap();
        assert!(g.prepare_catalog);
        assert!(!g.reconcile_after_fix);
        let g = repair_gates("all", &[issue("router_postcondition")]).unwrap();
        assert!(!g.prepare_catalog);
        assert!(g.reconcile_after_fix && g.use_quiet_transition);
        let g = repair_gates("router_postcondition", &[]).unwrap();
        assert!(g.direct_postcondition && g.reconcile_after_fix);
        assert!(!g.use_quiet_transition && !g.preflight_state_write);
    }
    #[test]
    fn all_uses_only_fixable_non_ok_items_for_gates() {
        for row in [
            Item {
                status: "ok",
                ..issue("db_orphan_providers")
            },
            Item {
                fixable: false,
                ..issue("db_orphan_providers")
            },
            issue("unrecognized"),
        ] {
            let g = repair_gates("all", &[row]).unwrap();
            assert!(!g.prepare_catalog && !g.use_quiet_transition && !g.reconcile_after_fix);
        }
        assert!(!repair_gates("all", &[]).unwrap().use_quiet_transition);
        assert!(
            !repair_gates("config_stale", &[issue("db_orphan_providers")])
                .unwrap()
                .prepare_catalog
        );
    }
    #[test]
    fn unknown_or_observation_id_is_rejected() {
        assert!(repair_gates("thread_history_integrity", &[]).is_err());
        assert!(repair_gates("unknown", &[]).is_err());
    }
}
