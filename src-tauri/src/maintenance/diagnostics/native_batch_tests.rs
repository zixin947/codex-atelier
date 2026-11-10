use super::*;
use atelier_maintenance_core::process::{BundleField, CommandOutput, Operation};
use serde_json::json;
use std::{fs, sync::Mutex, time::Duration};
struct FakeRunner {
    root: PathBuf,
    writer: bool,
    calls: Mutex<Vec<Operation>>,
}
impl Runner for FakeRunner {
    fn output(&self, op: &Operation) -> io::Result<CommandOutput> {
        self.calls.lock().unwrap().push(op.clone());
        let output = match op {
            Operation::Processes if self.writer => {
                "711 /usr/local/bin/codex --home synthetic".to_owned()
            }
            Operation::IsAppRunning => "false".into(),
            Operation::BundleValue {
                field: BundleField::Identifier,
                ..
            } => "com.openai.codex".into(),
            Operation::BundleValue {
                field: BundleField::Executable,
                ..
            } => "ChatGPT".into(),
            Operation::Processes
            | Operation::FindBundles
            | Operation::DbHolders(_)
            | Operation::CommandForPid(_) => String::new(),
            _ => panic!("batch must only observe process state, got {op:?}"),
        };
        Ok(CommandOutput {
            success: true,
            stdout: output.into_bytes(),
            stderr: vec![],
        })
    }
    fn signal_kill(&self, _: u32) -> io::Result<()> {
        panic!("batch cannot signal")
    }
    fn exists(&self, path: &Path) -> bool {
        path.starts_with(&self.root) && path.exists()
    }
    fn is_file(&self, path: &Path) -> bool {
        path.starts_with(&self.root) && path.is_file()
    }
    fn read_text(&self, path: &Path) -> io::Result<String> {
        assert!(path.starts_with(&self.root));
        fs::read_to_string(path)
    }
    fn read_dir(&self, path: &Path) -> io::Result<Vec<PathBuf>> {
        assert!(path.starts_with(&self.root));
        fs::read_dir(path)?.map(|e| e.map(|e| e.path())).collect()
    }
    fn process_id(&self) -> u32 {
        999_999
    }
    fn now(&self) -> Duration {
        Duration::ZERO
    }
    fn sleep(&self, _: Duration) {
        panic!("batch does not own fence waits")
    }
}
struct Fixture {
    root: PathBuf,
    ctx: Context,
    process: ProcessContext,
    runner: FakeRunner,
}
impl Fixture {
    fn new(config: &[u8]) -> Self {
        let root = std::env::temp_dir().join(format!(
            "atelier-production-native-batch-{}",
            uuid::Uuid::new_v4()
        ));
        let ctx = Context {
            home: root.join("codex"),
            app_root: root.join("app"),
            desktop_running: false,
            runtime: None,
            official_catalog: None,
            proxy_handle: None,
            working_directory: Some(root.clone()),
            sqlite_home_env: None,
        };
        fs::create_dir_all(ctx.home.join("sessions")).unwrap();
        fs::create_dir_all(&ctx.app_root).unwrap();
        fs::write(ctx.config(), config).unwrap();
        let process = ProcessContext {
            user_home: None,
            codex_home: ctx.home.clone(),
            sqlite_home_env: None,
            cwd: Some(root.clone()),
        };
        let runner = FakeRunner {
            root: root.clone(),
            writer: false,
            calls: Mutex::new(vec![]),
        };
        Self {
            root,
            ctx,
            process,
            runner,
        }
    }
    fn auth_inputs(&self) -> Vec<u8> {
        let a = br#"{"OPENAI_API_KEY":"sk-synthetic-batch-A","fixture":"A"}"#.to_vec();
        let relay = self.ctx.home.join("codexmate/relay");
        fs::create_dir_all(&relay).unwrap();
        fs::create_dir_all(self.ctx.home.join("accounts/backups")).unwrap();
        fs::write(self.ctx.home.join("accounts/backups/A.json"), &a).unwrap();
        fs::write(
            relay.join("router-unlock-auth-marker.json"),
            br#"{"backedUpPreviousAuth":true}"#,
        )
        .unwrap();
        fs::write(
            relay.join("router-unlock-auth-backup.json"),
            br#"{"OPENAI_API_KEY":"sk-synthetic-batch-B","fixture":"B"}"#,
        )
        .unwrap();
        a
    }
    fn apply(&self, plan: &ReviewedBatch) -> batch_repair::BatchOutcome {
        let guard = repair::acquire(&self.ctx).unwrap();
        let execution =
            apply_under_manager(&self.ctx, plan, &guard, &self.runner, &self.process).unwrap();
        assert!(
            execution.receipt.is_some(),
            "{:?}",
            execution.receipt_failure
        );
        execution
            .receipt
            .as_ref()
            .unwrap()
            .validate_current(&self.ctx, plan)
            .unwrap();
        assert_eq!(
            execution.receipt.as_ref().unwrap().config_after(),
            fs::read(self.ctx.config()).ok().as_deref()
        );
        execution.outcome
    }
}
impl Drop for Fixture {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.root);
    }
}
#[test]
fn production_native_batch_source_none_auth_integrity_before_unlock_preserves_a() {
    let f = Fixture::new(b"model_provider='openai'\n");
    let a = f.auth_inputs();
    let plan = prepare(&f.ctx).unwrap();
    assert!(plan.ids.contains(&"auth_integrity".into()));
    assert!(plan.ids.contains(&"router_unlock_auth_residue".into()));
    let outcome = f.apply(&plan);
    assert_eq!(fs::read(f.ctx.home.join("auth.json")).unwrap(), a);
    assert!(outcome.fix.failed_ids.is_empty(), "{:?}", outcome.fix);
    let items = outcome
        .trace
        .iter()
        .filter_map(|r| {
            if let batch_repair::Stage::Item(id) = r.stage {
                Some(id)
            } else {
                None
            }
        })
        .collect::<Vec<_>>();
    assert!(
        items.iter().position(|id| *id == BatchItem::AuthIntegrity)
            < items
                .iter()
                .position(|id| *id == BatchItem::UnlockAuthResidue)
    );
    assert!(!f.ctx.home.join("codexmate/relay/state.json").exists());
    assert!(!f.ctx.app_root.join("gateway/catalog.json").exists());
}
#[test]
fn production_native_batch_source_none_bad_syntax_has_one_observed_off_resync() {
    let f=Fixture::new(b"model='fixture'\n# >>> aimami-relay managed start (DO NOT EDIT MANUALLY)\ninvalid=[broken\n# <<< aimami-relay managed end\n");
    let plan = prepare(&f.ctx).unwrap();
    assert_eq!(plan.target(), TransitionTarget::NativeOff);
    let outcome = f.apply(&plan);
    assert!(
        outcome.fix.fixed_ids.contains(&"config_toml_syntax".into()),
        "{:?}",
        outcome.fix
    );
    assert_eq!(
        outcome
            .trace
            .iter()
            .filter(|r| r.stage == batch_repair::Stage::ResyncConfig)
            .count(),
        1
    );
    assert!(toml::from_str::<toml::Value>(&fs::read_to_string(f.ctx.config()).unwrap()).is_ok());
    assert!(!f.ctx.home.join("codexmate/relay/state.json").exists());
    assert!(!f.ctx.app_root.join("gateway/catalog.json").exists());
    assert!(!f.ctx.home.join("accounts").exists());
}
#[test]
fn production_native_batch_original_candidate_membership_is_validated_before_first_write() {
    let f = Fixture::new(b"model_provider='openai'\n");
    f.auth_inputs();
    let plan = prepare(&f.ctx).unwrap();
    fs::write(
        f.ctx.home.join("accounts/backups/unreviewed.json"),
        br#"{"OPENAI_API_KEY":"sk-synthetic-new"}"#,
    )
    .unwrap();
    let guard = repair::acquire(&f.ctx).unwrap();
    let failure = apply_under_manager(&f.ctx, &plan, &guard, &f.runner, &f.process).unwrap_err();
    assert_eq!(failure.stage, batch_repair::Stage::InitialDiagnosis);
    assert!(!f.ctx.home.join("auth.json").exists());
    assert!(!plan.backup.exists());
}
#[test]
fn production_native_batch_uses_real_runner_observation_and_never_stops_client() {
    let mut f = Fixture::new(b"model_provider='openai'\n");
    f.auth_inputs();
    f.runner.writer = true;
    let plan = prepare(&f.ctx).unwrap();
    let guard = repair::acquire(&f.ctx).unwrap();
    let failure = apply_under_manager(&f.ctx, &plan, &guard, &f.runner, &f.process).unwrap_err();
    assert_eq!(failure.stage, batch_repair::Stage::QuietWindow);
    assert!(!f.ctx.home.join("auth.json").exists());
    assert!(!plan.backup.exists());
}
#[test]
fn production_native_batch_ordinary_residuals_are_real_separate_ordered_actions() {
    let f = Fixture::new(b"model_provider='openai'\n");
    let relay = f.ctx.home.join("codexmate/relay");
    fs::create_dir_all(relay.join("thread-router-migrations")).unwrap();
    fs::write(
        relay.join("thread-router-migrations/current.json"),
        serde_json::to_vec(&json!({"fixture":"manifest"})).unwrap(),
    )
    .unwrap();
    fs::write(relay.join("relay-state-cache.json"), b"fixture cache").unwrap();
    let plan = prepare(&f.ctx).unwrap();
    let outcome = f.apply(&plan);
    let stages = outcome
        .trace
        .iter()
        .filter_map(|r| {
            if let batch_repair::Stage::Item(id) = r.stage {
                Some(id)
            } else {
                None
            }
        })
        .collect::<Vec<_>>();
    assert_eq!(
        stages,
        vec![BatchItem::ResidualManifest, BatchItem::ResidualCache]
    );
    assert!(!relay.join("thread-router-migrations/current.json").exists());
    assert!(!relay.join("relay-state-cache.json").exists());
    assert!(outcome.fix.failed_ids.is_empty(), "{:?}", outcome.fix);
}
struct FakeRpc;
impl native_orphans::CatalogRpc for FakeRpc {
    fn request(
        &mut self,
        method: &str,
        _params: serde_json::Value,
        _timeout: Duration,
    ) -> Result<serde_json::Value> {
        Ok(match method {
            "config/read" => json!({"config":{"model_provider":"openai"}}),
            "model/list" => {
                json!({"data":[{"model":"gpt-known","isDefault":true,"supportedReasoningEfforts":[{"reasoningEffort":"medium"}],"defaultReasoningEffort":"medium"}],"nextCursor":null})
            }
            _ => panic!("unexpected fixture RPC"),
        })
    }
}
#[test]
fn production_native_batch_orphan_then_native_threads_preserves_healthy_foreign_plain_and_zstd() {
    for compressed in [false, true] {
        let f=Fixture::new(b"model_provider='openai'\n[model_providers.enterprise]\nbase_url='https://example.test/v1'\n");
        let db = rusqlite::Connection::open(f.ctx.home.join("state_5.sqlite")).unwrap();
        db.execute_batch("CREATE TABLE threads(id TEXT PRIMARY KEY,model_provider TEXT,rollout_path TEXT,model TEXT,archived INTEGER DEFAULT 0,source TEXT DEFAULT 'cli')").unwrap();
        let orphan = f.ctx.home.join(if compressed {
            "sessions/orphan.jsonl.zst"
        } else {
            "sessions/orphan.jsonl"
        });
        let source = format!(
            "{}\n{}\n",
            json!({"type":"session_meta","payload":{"id":"orphan","model_provider":"aimami_relay_deleted","model":"gpt-known","base_instructions":{"text":"Healthy fixture instructions."}}}),
            json!({"type":"response_item","payload":{"type":"custom_tool_call","id":"fc_fixture"}})
        );
        fs::write(
            &orphan,
            if compressed {
                zstd::stream::encode_all(source.as_bytes(), 3).unwrap()
            } else {
                source.as_bytes().to_vec()
            },
        )
        .unwrap();
        let foreign = f.ctx.home.join("sessions/foreign.jsonl");
        let foreign_bytes = format!(
            "{}\n",
            json!({"type":"session_meta","payload":{"id":"foreign","model_provider":"enterprise","model":"private-model","base_instructions":{"text":"Healthy foreign instructions."}}})
        );
        fs::write(&foreign, &foreign_bytes).unwrap();
        for (id, provider, path, model) in [
            ("orphan", "aimami_relay_deleted", &orphan, "gpt-known"),
            ("foreign", "enterprise", &foreign, "private-model"),
        ] {
            db.execute(
                "INSERT INTO threads(id,model_provider,rollout_path,model) VALUES(?1,?2,?3,?4)",
                rusqlite::params![id, provider, path.to_str().unwrap(), model],
            )
            .unwrap();
        }
        drop(db);
        let catalog = native_orphans::prepare_catalog_with_rpc(&f.ctx, &mut FakeRpc).unwrap();
        let plan = prepare_with_catalog(&f.ctx, &catalog).unwrap();
        let outcome = f.apply(&plan);
        assert!(outcome.fix.failed_ids.is_empty(), "{:?}", outcome.fix);
        let db = rusqlite::Connection::open(f.ctx.home.join("state_5.sqlite")).unwrap();
        assert_eq!(
            db.query_row(
                "SELECT model_provider FROM threads WHERE id='orphan'",
                [],
                |r| r.get::<_, String>(0)
            )
            .unwrap(),
            "openai"
        );
        assert_eq!(
            db.query_row(
                "SELECT model_provider FROM threads WHERE id='foreign'",
                [],
                |r| r.get::<_, String>(0)
            )
            .unwrap(),
            "enterprise"
        );
        assert_eq!(fs::read_to_string(&foreign).unwrap(), foreign_bytes);
        let bytes = fs::read(&orphan).unwrap();
        let bytes = if compressed {
            zstd::stream::decode_all(bytes.as_slice()).unwrap()
        } else {
            bytes
        };
        assert!(String::from_utf8(bytes).unwrap().contains("ctc_fixture"));
        let items = outcome
            .trace
            .iter()
            .filter_map(|r| {
                if let batch_repair::Stage::Item(id) = r.stage {
                    Some(id)
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();
        assert!(
            items
                .iter()
                .position(|id| *id == BatchItem::OrphanProviders)
                < items
                    .iter()
                    .position(|id| *id == BatchItem::ThreadResponseItemIds)
        );
    }
}
#[test]
fn production_native_batch_ordinary_auth_failure_keeps_partial_failed_ids_and_continues_residual() {
    let f = Fixture::new(b"model_provider='openai'\n");
    let a = f.auth_inputs();
    // A synthetic directory forces actual fixed-path lock open to fail immediately.
    fs::create_dir(f.ctx.home.join("accounts/.live-auth.lock")).unwrap();
    let cache = f.ctx.home.join("codexmate/relay/relay-state-cache.json");
    fs::write(&cache, b"synthetic obsolete cache").unwrap();
    let plan = prepare(&f.ctx).unwrap();
    let outcome = f.apply(&plan);
    assert!(
        outcome.fix.fixed_ids.contains(&"auth_integrity".into()),
        "{:?}",
        outcome.fix
    );
    assert!(outcome
        .fix
        .failed_ids
        .contains(&"router_unlock_auth_residue".into()));
    assert!(outcome.fix.fixed_ids.contains(&"residual_cache".into()));
    assert_eq!(fs::read(f.ctx.home.join("auth.json")).unwrap(), a);
    assert!(!cache.exists());
    assert!(outcome
        .trace
        .iter()
        .any(|r| r.stage == batch_repair::Stage::Item(BatchItem::ResidualCache)));
}
#[test]
fn production_native_batch_unrecoverable_syntax_marks_resync_failed_instead_of_healthy() {
    let f = Fixture::new(b"model='x'\n[broken\n");
    let plan = prepare(&f.ctx).unwrap();
    let outcome = f.apply(&plan);
    assert!(outcome
        .fix
        .failed_ids
        .contains(&"config_toml_syntax".into()));
    assert!(!outcome.fix.fixed_ids.contains(&"config_toml_syntax".into()));
    assert!(outcome
        .trace
        .iter()
        .any(|r| r.stage == batch_repair::Stage::ResyncConfig
            && matches!(r.result, batch_repair::StageResult::Failed(_))));
    assert!(outcome.resync_report.is_none());
    assert!(toml::from_str::<toml::Value>(&fs::read_to_string(f.ctx.config()).unwrap()).is_err());
}
#[test]
fn production_native_batch_project_actual_sql_effect_is_certified_before_old_database_cleanup() {
    let f = Fixture::new(b"model_provider='openai'\n");
    let sqlite = f.ctx.home.join("sqlite");
    fs::create_dir(&sqlite).unwrap();
    fs::write(
        f.ctx.config(),
        format!(
            "model_provider='openai'\nsqlite_home={:?}\n",
            sqlite.to_str().unwrap()
        ),
    )
    .unwrap();
    let history = f.ctx.home.join("sessions/project.jsonl");
    fs::write(&history,format!("{}\n",json!({"type":"session_meta","payload":{"id":"project","model_provider":"openai","cwd":"/fixture/workspace","base_instructions":{"text":"Healthy project instructions."}}}))).unwrap();
    let active = sqlite.join("state_6.sqlite");
    let db = rusqlite::Connection::open(&active).unwrap();
    db.execute_batch("CREATE TABLE threads(id TEXT PRIMARY KEY,model_provider TEXT,model TEXT,rollout_path TEXT,archived INTEGER DEFAULT 0,source TEXT DEFAULT 'cli',cwd TEXT DEFAULT '',updated_at_ms INTEGER DEFAULT 0)").unwrap();
    db.execute("INSERT INTO threads(id,model_provider,model,rollout_path) VALUES('project','openai','gpt-known',?1)",[history.to_str().unwrap()]).unwrap();
    drop(db);
    let old = f.ctx.home.join("state_5.sqlite");
    rusqlite::Connection::open(&old)
        .unwrap()
        .execute_batch("CREATE TABLE old_marker(id INTEGER)")
        .unwrap();
    fs::write(
        f.ctx.home.join(".codex-global-state.json"),
        b"{\"unrelated\":42}",
    )
    .unwrap();
    let plan = prepare(&f.ctx).unwrap();
    assert!(plan.ids.contains(&"project_state_integrity".into()));
    assert!(plan.ids.contains(&"state_db_relocation".into()));
    let outcome = f.apply(&plan);
    assert!(outcome.fix.failed_ids.is_empty(), "{:?}", outcome.fix);
    assert!(!old.exists());
    assert!(active.exists());
    assert_eq!(
        rusqlite::Connection::open(&active)
            .unwrap()
            .query_row("SELECT cwd FROM threads WHERE id='project'", [], |r| r
                .get::<_, String>(
                0
            ))
            .unwrap(),
        "/fixture/workspace"
    );
    let value: serde_json::Value =
        serde_json::from_slice(&fs::read(f.ctx.home.join(".codex-global-state.json")).unwrap())
            .unwrap();
    assert_eq!(value["unrelated"], 42);
    let stages = outcome
        .trace
        .iter()
        .filter_map(|r| {
            if let batch_repair::Stage::Item(id) = r.stage {
                Some(id)
            } else {
                None
            }
        })
        .collect::<Vec<_>>();
    assert!(
        stages
            .iter()
            .position(|id| *id == BatchItem::ProjectStateIntegrity)
            < stages
                .iter()
                .position(|id| *id == BatchItem::StateDbRelocation)
    );
}
#[test]
fn production_native_batch_final_receipt_uses_global_after_internal_resync_and_rejects_other_preview(
) {
    let f = Fixture::new(b"profile='legacy'\nmodel_provider='openai'\nmodel_reasoning_effort=''\n");
    let plan = prepare(&f.ctx).unwrap();
    let other = prepare(&f.ctx).unwrap();
    assert!(plan.ids.contains(&"config_profile_conflict".into()));
    assert!(plan.ids.contains(&"config_global_validity".into()));
    let guard = repair::acquire(&f.ctx).unwrap();
    let execution = apply_under_manager(&f.ctx, &plan, &guard, &f.runner, &f.process).unwrap();
    let receipt = execution
        .receipt
        .as_ref()
        .expect("known global final must have receipt");
    receipt.validate_current(&f.ctx, &plan).unwrap();
    assert!(receipt.validate_current(&f.ctx, &other).is_err());
    let after = fs::read(f.ctx.config()).unwrap();
    assert_eq!(receipt.config_after(), Some(after.as_slice()));
    assert!(plan.config_outputs().contains(&Some(after.clone())));
    let text = String::from_utf8(after).unwrap();
    assert!(!text.contains("profile="));
    assert!(!text.contains("model_reasoning_effort"));
    let stages = execution
        .outcome
        .trace
        .iter()
        .map(|r| r.stage)
        .collect::<Vec<_>>();
    assert!(
        stages
            .iter()
            .position(|s| *s == batch_repair::Stage::ResyncConfig)
            < stages
                .iter()
                .position(|s| *s == batch_repair::Stage::Item(BatchItem::ConfigGlobalValidity))
    );
    fs::write(f.ctx.config(), b"model='outside-preview'\n").unwrap();
    assert!(receipt.validate_current(&f.ctx, &plan).is_err());
}

#[test]
fn production_native_batch_end_successor_retains_global_branch_and_single_use_receipt() {
    let f = Fixture::new(b"model_provider='openai'\nmodel_reasoning_effort=''\n");
    let plan = prepare(&f.ctx).unwrap();
    let other = prepare(&f.ctx).unwrap();
    let guard = repair::acquire(&f.ctx).unwrap();
    let execution = apply_under_manager(&f.ctx, &plan, &guard, &f.runner, &f.process).unwrap();
    let receipt = execution
        .receipt
        .expect("Global effect must remain certified");
    assert!(receipt.global_branch().is_some());
    assert!(reconcile_after_batch(&f.ctx, &other, &receipt, &guard).is_err());
    let (outcome, next) = reconcile_after_batch(&f.ctx, &plan, &receipt, &guard).unwrap();
    assert!(outcome.failure.is_none());
    assert!(outcome.verification.issues.is_empty());
    assert_eq!(next.config_after(), receipt.config_after());
    next.validate_current(&f.ctx, &plan).unwrap();
    assert!(reconcile_after_batch(&f.ctx, &plan, &next, &guard).is_err());
    assert!(!f.ctx.home.join("codexmate/relay/state.json").exists());
    assert!(!f.ctx.app_root.join("gateway/catalog.json").exists());
}

#[test]
fn production_native_batch_end_successor_rejects_new_auth_member_source_intent_and_config() {
    for mutation in ["member", "source", "intent", "config"] {
        let f = Fixture::new(b"model_provider='openai'\nmodel_reasoning_effort=''\n");
        let plan = prepare(&f.ctx).unwrap();
        let guard = repair::acquire(&f.ctx).unwrap();
        let execution = apply_under_manager(&f.ctx, &plan, &guard, &f.runner, &f.process).unwrap();
        let receipt = execution.receipt.unwrap();
        let path = match mutation {
            "member" => f.ctx.home.join("accounts/backups/unreviewed.json"),
            "source" => f.ctx.home.join("codexmate/relay/state.json"),
            "intent" => super::super::router_intent::path(&f.ctx),
            _ => f.ctx.config(),
        };
        fs::create_dir_all(path.parent().unwrap()).unwrap();
        fs::write(&path, b"unreviewed fixture bytes").unwrap();
        let before = fs::read(&path).unwrap();
        assert!(
            reconcile_after_batch(&f.ctx, &plan, &receipt, &guard).is_err(),
            "{mutation}"
        );
        assert_eq!(fs::read(path).unwrap(), before);
        assert!(!plan.backup.join("final-router").exists());
    }
}

#[test]
fn production_native_batch_end_successor_rejects_wrong_global_identity_even_for_equal_bytes() {
    let f = Fixture::new(b"model_provider='openai'\nmodel_reasoning_effort=''\n");
    let plan = prepare(&f.ctx).unwrap();
    let guard = repair::acquire(&f.ctx).unwrap();
    let execution = apply_under_manager(&f.ctx, &plan, &guard, &f.runner, &f.process).unwrap();
    let mut receipt = execution.receipt.unwrap();
    receipt.global_branch = Some(usize::MAX);
    assert!(reconcile_after_batch(&f.ctx, &plan, &receipt, &guard).is_err());
    assert!(!plan.backup.join("final-router").exists());
}
