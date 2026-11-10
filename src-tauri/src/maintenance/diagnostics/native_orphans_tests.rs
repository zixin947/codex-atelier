use super::*;
use serde_json::{json, Value};
use std::collections::VecDeque;
struct Fixture {
    base: PathBuf,
    ctx: Context,
    db: PathBuf,
}
impl Fixture {
    fn new() -> Self {
        let base =
            std::env::temp_dir().join(format!("atelier-native-orphans-{}", uuid::Uuid::new_v4()));
        let ctx = Context {
            home: base.join("codex"),
            app_root: base.join("atelier"),
            desktop_running: false,
            runtime: None,
            official_catalog: None,
            proxy_handle: None,
            working_directory: None,
            sqlite_home_env: None,
        };
        fs::create_dir_all(ctx.home.join("sessions")).unwrap();
        fs::create_dir_all(&ctx.app_root).unwrap();
        fs::write(ctx.config(),"model_provider='openai'\n[model_providers.atelier_live]\nbase_url='http://fixture.invalid'\n").unwrap();
        let db = ctx.home.join("state_5.sqlite");
        let c = rusqlite::Connection::open(&db).unwrap();
        c.execute_batch("CREATE TABLE threads(id TEXT PRIMARY KEY,model_provider TEXT,model TEXT,reasoning_effort TEXT,rollout_path TEXT,archived INTEGER DEFAULT 0,source TEXT DEFAULT 'cli')").unwrap();
        drop(c);
        Self { base, ctx, db }
    }
    fn row(&self, id: &str, provider: &str, rollout_provider: &str, model: &str) -> PathBuf {
        let path = self.ctx.home.join("sessions").join(format!("{id}.jsonl"));
        fs::write(&path,format!("{}\n{}\n",json!({"type":"session_meta","payload":{"id":id,"model_provider":rollout_provider,"model":model,"model_reasoning_effort":"medium"}}),json!({"type":"event_msg","payload":{"type":"user_message","message":"preserve exact body"}}))).unwrap();
        let c = rusqlite::Connection::open(&self.db).unwrap();
        c.execute("INSERT INTO threads(id,model_provider,model,reasoning_effort,rollout_path) VALUES(?1,?2,?3,'medium',?4)",rusqlite::params![id,provider,model,path.to_str().unwrap()]).unwrap();
        path
    }
    fn state(&self, id: &str) -> (String, String) {
        rusqlite::Connection::open(&self.db)
            .unwrap()
            .query_row(
                "SELECT model_provider,model FROM threads WHERE id=?1",
                [id],
                |r| Ok((r.get(0)?, r.get(1)?)),
            )
            .unwrap()
    }
}
impl Drop for Fixture {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.base);
    }
}
fn official() -> catalog::OfficialCatalog {
    catalog::OfficialCatalog {
        dependencies: BTreeMap::new(),
        models: ["gpt-known", "gpt-default"]
            .into_iter()
            .map(str::to_owned)
            .collect(),
        default: Some("gpt-default".into()),
        raw: vec![
            json!({"model":"gpt-known","supportedReasoningEfforts":[{"reasoningEffort":"medium"}],"defaultReasoningEffort":"medium"}),
            json!({"model":"gpt-default","isDefault":true,"supportedReasoningEfforts":[{"reasoningEffort":"medium"}],"defaultReasoningEffort":"medium"}),
        ],
    }
}
#[test]
fn native_and_explicit_atelier_orphans_do_not_select_healthy_foreign_openai_or_defined_atelier() {
    let f = Fixture::new();
    for (id, p) in [
        ("foreign", "enterprise-private"),
        ("official", "openai"),
        ("atelier", "atelier_live"),
        ("orphan", "aimami_relay_deleted"),
        ("old-atelier", "atelier_deleted"),
    ] {
        f.row(id, p, p, "gpt-known");
    }
    let p = select(&f.ctx).unwrap();
    let ids = p
        .rows
        .iter()
        .map(|r| r.id.as_str())
        .collect::<BTreeSet<_>>();
    assert_eq!(ids, ["orphan", "old-atelier"].into_iter().collect());
    assert_eq!(p.db_ids.len(), 2);
    assert_eq!(p.rollout_ids.len(), 2);
}
#[test]
fn db_and_rollout_checks_form_independent_union() {
    let f = Fixture::new();
    f.row("db-only", "aimami_relay_deleted", "openai", "gpt-known");
    f.row(
        "rollout-only",
        "openai",
        "aimami_relay_deleted",
        "gpt-known",
    );
    let p = select(&f.ctx).unwrap();
    assert_eq!(p.rows.len(), 2);
    assert_eq!(p.db_ids, ["db-only".into()].into());
    assert_eq!(p.rollout_ids, ["rollout-only".into()].into());
}
#[test]
fn mixed_external_ownership_of_a_selected_history_stops_adoption() {
    let f = Fixture::new();
    f.row(
        "mixed",
        "aimami_relay_deleted",
        "enterprise-private",
        "gpt-known",
    );
    assert!(select(&f.ctx).err().unwrap().contains("external provider"));
    assert_eq!(f.state("mixed").0, "aimami_relay_deleted");
}
#[test]
fn unselected_external_history_is_preserved_even_when_invalid_or_missing() {
    let f = Fixture::new();
    let p = f.row(
        "foreign",
        "enterprise-private",
        "enterprise-private",
        "other",
    );
    fs::write(&p, b"not JSON").unwrap();
    f.row(
        "orphan",
        "aimami_relay_deleted",
        "aimami_relay_deleted",
        "gpt-known",
    );
    let selected = select(&f.ctx).unwrap();
    assert_eq!(selected.rows.len(), 1);
    assert_eq!(fs::read(p).unwrap(), b"not JSON");
}
#[test]
fn selected_shared_path_with_different_ids_is_rejected() {
    let f = Fixture::new();
    let path = f.row(
        "a",
        "aimami_relay_deleted",
        "aimami_relay_deleted",
        "gpt-known",
    );
    f.row("b", "openai", "openai", "gpt-known");
    rusqlite::Connection::open(&f.db)
        .unwrap()
        .execute(
            "UPDATE threads SET rollout_path=?1 WHERE id='b'",
            [path.to_str().unwrap()],
        )
        .unwrap();
    assert!(select(&f.ctx)
        .err()
        .unwrap()
        .contains("multiple thread ids"));
}
#[test]
fn source_column_is_required_and_subagents_do_not_enter_orphan_scope() {
    let f = Fixture::new();
    f.row(
        "orphan",
        "aimami_relay_deleted",
        "aimami_relay_deleted",
        "gpt-known",
    );
    rusqlite::Connection::open(&f.db)
        .unwrap()
        .execute("UPDATE threads SET source=?1", ["{\"subagent\":{}}"])
        .unwrap();
    assert!(select(&f.ctx).unwrap().rows.is_empty());
    rusqlite::Connection::open(&f.db)
        .unwrap()
        .execute_batch("ALTER TABLE threads DROP COLUMN source")
        .unwrap();
    assert!(select(&f.ctx).is_err());
}
#[test]
fn database_and_rollout_changes_after_review_fail_before_any_mutation() {
    let f = Fixture::new();
    let path = f.row(
        "orphan",
        "aimami_relay_deleted",
        "aimami_relay_deleted",
        "gpt-known",
    );
    let plan = build(&f.ctx, select(&f.ctx).unwrap(), &official()).unwrap();
    fs::write(&path, b"external change").unwrap();
    let guard = super::super::repair::acquire(&f.ctx).unwrap();
    assert!(apply(&f.ctx, &plan, &f.base.join("backups"), &guard)
        .unwrap_err()
        .contains("预览后变化"));
    assert!(!f.base.join("backups").exists());
    assert_eq!(f.state("orphan").0, "aimami_relay_deleted");
}
#[test]
fn database_membership_change_is_not_approved_by_unchanged_rollout_bytes() {
    let f = Fixture::new();
    f.row(
        "orphan",
        "aimami_relay_deleted",
        "aimami_relay_deleted",
        "gpt-known",
    );
    let p = select(&f.ctx).unwrap();
    fs::write(f.ctx.home.join("state_6.sqlite"), b"new database").unwrap();
    assert!(validate(&f.ctx, &p).unwrap_err().contains("数据库集合"));
}
#[test]
fn confirmed_apply_preserves_healthy_mixed_providers_and_preserves_or_falls_back_models() {
    let f = Fixture::new();
    let mut untouched = vec![];
    for (id, p) in [
        ("foreign", "enterprise-private"),
        ("official", "openai"),
        ("atelier", "atelier_live"),
    ] {
        let path = f.row(id, p, p, "gpt-known");
        untouched.push((id, p, path.clone(), fs::read(path).unwrap()));
    }
    f.row(
        "known",
        "aimami_relay_deleted",
        "aimami_relay_deleted",
        "gpt-known",
    );
    f.row("warning", "atelier_deleted", "atelier_deleted", "bad-model");
    let p = build(&f.ctx, select(&f.ctx).unwrap(), &official()).unwrap();
    let guard = super::super::repair::acquire(&f.ctx).unwrap();
    let result = apply(&f.ctx, &p, &f.base.join("backups"), &guard).unwrap();
    assert!(!result.is_empty());
    assert_eq!(f.state("known"), ("openai".into(), "gpt-known".into()));
    assert_eq!(f.state("warning"), ("openai".into(), "gpt-default".into()));
    for (id, provider, path, bytes) in untouched {
        assert_eq!(f.state(id).0, provider);
        assert_eq!(fs::read(path).unwrap(), bytes);
    }
    assert!(select(&f.ctx).unwrap().rows.is_empty());
    let recovery: Value =
        serde_json::from_slice(&fs::read(f.base.join("backups/adoption-recovery.json")).unwrap())
            .unwrap();
    assert_eq!(recovery["phase"], "completed");
}
struct FakeRpc {
    expected: VecDeque<(&'static str, Value, Value)>,
}
impl catalog::Rpc for FakeRpc {
    fn request(
        &mut self,
        method: &str,
        params: Value,
        timeout: std::time::Duration,
    ) -> Result<Value> {
        assert!(!timeout.is_zero());
        assert!(timeout <= std::time::Duration::from_secs(10));
        let (m, p, r) = self.expected.pop_front().expect("unexpected RPC");
        assert_eq!(method, m);
        assert_eq!(params, p);
        Ok(r)
    }
}
#[test]
fn official_reader_uses_real_config_guards_and_cursor_protocol_without_starting_processes() {
    let f = Fixture::new();
    let config = json!({"config":{"model_provider":"openai"}});
    let mut rpc = FakeRpc {
        expected: VecDeque::from([
            ("config/read", json!({"includeLayers":true}), config.clone()),
            (
                "model/list",
                json!({"includeHidden":true,"limit":100}),
                json!({"data":[{"id":"id-one","model":"gpt-known"}],"nextCursor":"next"}),
            ),
            (
                "model/list",
                json!({"includeHidden":true,"limit":100,"cursor":"next"}),
                json!({"data":[{"id":"id-two","model":"gpt-known"},{"model":"gpt-default","isDefault":true}],"nextCursor":null}),
            ),
            ("config/read", json!({"includeLayers":true}), config),
        ]),
    };
    let models = catalog::prepare_with(&f.ctx, &mut rpc).unwrap();
    assert_eq!(models.models.len(), 2);
    assert_eq!(models.default.as_deref(), Some("gpt-default"));
    assert!(rpc.expected.is_empty());
}
#[test]
fn gateway_inventory_cannot_be_used_as_official_pool() {
    let f = Fixture::new();
    let mut rpc = FakeRpc {
        expected: VecDeque::from([(
            "config/read",
            json!({"includeLayers":true}),
            json!({"config":{"model_provider":"atelier_live"}}),
        )]),
    };
    assert!(catalog::prepare_with(&f.ctx, &mut rpc)
        .err()
        .unwrap()
        .contains("第三方/Gateway"));
    assert!(rpc.expected.is_empty());
}

#[test]
fn default_test_transport_refuses_real_discovery_and_spawn() {
    let f = Fixture::new();
    assert!(catalog::prepare(&f.ctx)
        .err()
        .unwrap()
        .contains("测试默认禁止"));
}
#[test]
fn official_reader_filters_hidden_and_managed_qualified_models() {
    let f = Fixture::new();
    let config = json!({"config":{}});
    let mut rpc = FakeRpc {
        expected: VecDeque::from([
            ("config/read", json!({"includeLayers":true}), config.clone()),
            (
                "model/list",
                json!({"includeHidden":true,"limit":100}),
                json!({"data":[{"model":"hidden","hidden":true,"isDefault":true},{"model":"aimami_relay_old"},{"model":"provider::model"},{"model":"good"}]}),
            ),
            ("config/read", json!({"includeLayers":true}), config),
        ]),
    };
    let c = catalog::prepare_with(&f.ctx, &mut rpc).unwrap();
    assert_eq!(c.models, ["good".into()].into());
    assert_eq!(c.default.as_deref(), Some("good"));
}
#[test]
fn effort_must_be_supported_and_uses_session_meta_model_reasoning_effort() {
    let f = Fixture::new();
    let path = f.row(
        "orphan",
        "aimami_relay_deleted",
        "aimami_relay_deleted",
        "gpt-known",
    );
    let mut c = official();
    c.raw[0]["supportedReasoningEfforts"] = json!([]);
    c.raw[0]["defaultReasoningEffort"] = Value::Null;
    assert!(build(&f.ctx, select(&f.ctx).unwrap(), &c)
        .err()
        .unwrap()
        .contains("推理档位"));
    let p = build(&f.ctx, select(&f.ctx).unwrap(), &official()).unwrap();
    let guard = super::super::repair::acquire(&f.ctx).unwrap();
    apply(&f.ctx, &p, &f.base.join("backup"), &guard).unwrap();
    let bytes = fs::read(path).unwrap();
    let rs = records(&bytes);
    assert_eq!(rs[0].value["payload"]["model_reasoning_effort"], "medium");
}
#[test]
fn auth_snapshot_change_invalidates_prepared_official_catalog_source() {
    let f = Fixture::new();
    f.row(
        "orphan",
        "aimami_relay_deleted",
        "aimami_relay_deleted",
        "gpt-known",
    );
    let p = select(&f.ctx).unwrap();
    fs::write(
        f.ctx.home.join("auth.json"),
        b"synthetic external auth change",
    )
    .unwrap();
    assert!(validate(&f.ctx, &p).unwrap_err().contains("预览后变化"));
}
#[test]
fn plain_compressed_aliases_share_the_same_ownership_group() {
    let f = Fixture::new();
    let p = f.row(
        "orphan",
        "aimami_relay_deleted",
        "aimami_relay_deleted",
        "gpt-known",
    );
    f.row(
        "foreign",
        "enterprise-private",
        "enterprise-private",
        "other",
    );
    let alias = format!("{}.zst", p.display());
    rusqlite::Connection::open(&f.db)
        .unwrap()
        .execute(
            "UPDATE threads SET rollout_path=?1 WHERE id='foreign'",
            [alias],
        )
        .unwrap();
    assert!(select(&f.ctx).err().unwrap().contains("external provider"));
}
#[test]
fn twentieth_page_without_cursor_completes_but_with_cursor_exceeds_native_limit() {
    for more in [false, true] {
        let f = Fixture::new();
        let config = json!({"config":{}});
        let mut expected =
            VecDeque::from([("config/read", json!({"includeLayers":true}), config.clone())]);
        for n in 0..20 {
            let mut params = json!({"includeHidden":true,"limit":100});
            if n > 0 {
                params["cursor"] = format!("c{n}").into();
            }
            let next = if n < 19 || more {
                json!(format!("c{}", n + 1))
            } else {
                Value::Null
            };
            expected.push_back((
                "model/list",
                params,
                json!({"data":[{"model":format!("model{n}")}],"nextCursor":next}),
            ));
        }
        if !more {
            expected.push_back(("config/read", json!({"includeLayers":true}), config));
        }
        let mut rpc = FakeRpc { expected };
        let result = catalog::prepare_with(&f.ctx, &mut rpc);
        if more {
            assert!(result.err().unwrap().contains("page limit"));
        } else {
            assert_eq!(result.unwrap().models.len(), 20);
        }
        assert!(rpc.expected.is_empty());
    }
}
#[test]
fn repeated_cursor_and_effective_config_change_are_not_successful_catalogs() {
    let f = Fixture::new();
    let config = json!({"config":{}});
    let mut rpc = FakeRpc {
        expected: VecDeque::from([
            ("config/read", json!({"includeLayers":true}), config.clone()),
            (
                "model/list",
                json!({"includeHidden":true,"limit":100}),
                json!({"data":[{"model":"a"}],"nextCursor":"same"}),
            ),
            (
                "model/list",
                json!({"includeHidden":true,"limit":100,"cursor":"same"}),
                json!({"data":[{"model":"b"}],"nextCursor":"same"}),
            ),
        ]),
    };
    assert!(catalog::prepare_with(&f.ctx, &mut rpc)
        .err()
        .unwrap()
        .contains("cursor repeated"));
    assert!(rpc.expected.is_empty());
    let mut rpc = FakeRpc {
        expected: VecDeque::from([
            ("config/read", json!({"includeLayers":true}), config),
            (
                "model/list",
                json!({"includeHidden":true,"limit":100}),
                json!({"data":[{"model":"a"}]}),
            ),
            (
                "config/read",
                json!({"includeLayers":true}),
                json!({"config":{"model":"changed"}}),
            ),
        ]),
    };
    assert!(catalog::prepare_with(&f.ctx, &mut rpc)
        .err()
        .unwrap()
        .contains("effective config"));
}
#[test]
fn actual_database_trigger_failure_rolls_back_and_returns_verified_prefix_without_claiming_success()
{
    let f = Fixture::new();
    let path = f.row(
        "orphan",
        "aimami_relay_deleted",
        "aimami_relay_deleted",
        "gpt-known",
    );
    let bytes = fs::read(&path).unwrap();
    let c = rusqlite::Connection::open(&f.db).unwrap();
    c.execute_batch("CREATE TRIGGER reject_changes BEFORE UPDATE ON threads BEGIN SELECT RAISE(ABORT,'synthetic refusal'); END;").unwrap();
    drop(c);
    let p = build(&f.ctx, select(&f.ctx).unwrap(), &official()).unwrap();
    let guard = super::super::repair::acquire(&f.ctx).unwrap();
    let outcome = apply_verified(&f.ctx, &p, &f.base.join("backups"), &guard).unwrap();
    assert!(outcome.failure.is_some());
    assert_eq!(f.state("orphan").0, "aimami_relay_deleted");
    assert_eq!(fs::read(path).unwrap(), bytes);
    assert!(outcome.effects.effect(&f.db).is_some());
    let recovery: Value =
        serde_json::from_slice(&fs::read(f.base.join("backups/adoption-recovery.json")).unwrap())
            .unwrap();
    assert_eq!(recovery["phase"], "rolledBack");
}
