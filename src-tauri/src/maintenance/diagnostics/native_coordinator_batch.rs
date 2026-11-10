//! The original fix_all auth prefix and four ordered, reviewed text preparations.
//! This deliberately does not reuse the legacy composed-config execution order.
use super::*;
use atelier_maintenance_core::batch_repair::ConfigPreparation;

#[derive(Clone)]
struct Candidate {
    bytes: Option<Vec<u8>>,
    next: BTreeMap<u8, std::result::Result<usize, String>>,
}
struct BatchData {
    home: PathBuf,
    app_root: PathBuf,
    backup: PathBuf,
    mode: RouterMode,
    ids: Vec<String>,
    auth: Vec<native_auth::PreparedAction>,
    dependencies: BTreeMap<PathBuf, Option<Stamp>>,
    modified: BTreeMap<PathBuf, Option<std::time::SystemTime>>,
    candidates: Vec<Candidate>,
    takeover_candidate: Option<usize>,
}
/// All source observations and each possible ordered preparation result are bound
/// at review time. No execution entry accepts replacement configuration bytes.
#[derive(Clone)]
pub(in crate::maintenance::diagnostics) struct PreparedBatch(Arc<BatchData>);
impl PreparedBatch {
    pub fn validate_initial(&self, ctx: &Context) -> Result<()> {
        validate_batch_initial(ctx, self)
    }
    pub fn auth_actions(&self) -> &[native_auth::PreparedAction] {
        &self.0.auth
    }
    pub fn mode(&self) -> RouterMode {
        self.0.mode
    }
    pub fn original_effect_stamp(
        &self,
        path: &Path,
    ) -> Option<Option<super::super::native_orphans::EffectStamp>> {
        self.0.dependencies.get(path).map(|s| {
            s.as_ref().map(|s| {
                (
                    s.length as u64,
                    s.sha256.clone(),
                    self.0.modified.get(path).copied().flatten(),
                )
            })
        })
    }
    pub fn original_dependencies(&self) -> Vec<(PathBuf, Option<(usize, String)>)> {
        self.0
            .dependencies
            .iter()
            .map(|(p, s)| (p.clone(), stamp_tuple(s)))
            .collect()
    }
    pub fn config_before(&self) -> Option<&[u8]> {
        self.0.candidates[0].bytes.as_deref()
    }
    pub fn config_candidates(&self) -> Vec<(usize, Option<Vec<u8>>)> {
        self.0
            .candidates
            .iter()
            .enumerate()
            .map(|(i, c)| (i, c.bytes.clone()))
            .collect()
    }
    pub fn same_plan(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.0, &other.0)
    }
    pub fn item_ids(&self) -> &[String] {
        &self.0.ids
    }
}
/// A certificate produced only after applying a previously reviewed effect and
/// checking its actual bytes. Keeping the original plan here prevents a prefix
/// from another preview from authorizing this preview's sources.
#[derive(Clone)]
pub(in crate::maintenance::diagnostics) struct ApprovedBatchPrefix {
    plan: PreparedBatch,
    expected: BTreeMap<PathBuf, Option<Stamp>>,
    candidate: usize,
    poisoned: bool,
    modified: BTreeMap<PathBuf, Option<std::time::SystemTime>>,
}
impl ApprovedBatchPrefix {
    pub fn plan(&self) -> &PreparedBatch {
        &self.plan
    }
    pub fn candidate_key(&self) -> usize {
        self.candidate
    }
    pub fn config_after(&self) -> Option<&[u8]> {
        self.plan.0.candidates[self.candidate].bytes.as_deref()
    }
    pub fn effect(
        &self,
        path: &Path,
    ) -> Option<(Option<(usize, String)>, Option<(usize, String)>)> {
        let before = self.plan.0.dependencies.get(path)?;
        let after = self.expected.get(path)?;
        (before != after).then(|| (stamp_tuple(before), stamp_tuple(after)))
    }
    pub fn effect_stamp(
        &self,
        path: &Path,
    ) -> Option<(
        Option<super::super::native_orphans::EffectStamp>,
        Option<super::super::native_orphans::EffectStamp>,
    )> {
        let (before, after) = self.effect(path)?;
        Some((
            before.map(|(n, h)| {
                (
                    n as u64,
                    h,
                    self.plan.0.modified.get(path).copied().flatten(),
                )
            }),
            after.map(|(n, h)| (n as u64, h, self.modified.get(path).copied().flatten())),
        ))
    }
    pub fn paths(&self) -> Vec<PathBuf> {
        self.effect_paths()
    }
    pub fn effect_paths(&self) -> Vec<PathBuf> {
        self.effects().into_iter().map(|(p, _, _)| p).collect()
    }
    pub fn effects(&self) -> Vec<(PathBuf, Option<(usize, String)>, Option<(usize, String)>)> {
        self.expected
            .keys()
            .filter_map(|p| self.effect(p).map(|(a, b)| (p.clone(), a, b)))
            .collect()
    }
    pub fn validate_current(&self, ctx: &Context) -> Result<()> {
        if self.poisoned {
            return Err("批次前序存在未确认的部分效果，保留现状并停止后继写入".into());
        }
        if ctx.home != self.plan.0.home || ctx.app_root != self.plan.0.app_root {
            return Err("批次审批目录不匹配".into());
        }
        if observed_batch_mode(ctx, &self.plan.0.ids)? != self.plan.0.mode {
            return Err("批次路由模式已变化".into());
        }
        validate_ledger(&self.expected, &BTreeMap::new())?;
        for (path, expected) in &self.modified {
            if modified(path)? != *expected {
                return Err("批次依赖时间戳已被外部更新".into());
            }
        }
        Ok(())
    }
}
pub(in crate::maintenance::diagnostics) struct BatchSession {
    prefix: ApprovedBatchPrefix,
    called_auth: BTreeSet<String>,
    called_config: BTreeSet<u8>,
}
impl BatchSession {
    pub fn prefix(&self) -> &ApprovedBatchPrefix {
        &self.prefix
    }
}
pub(in crate::maintenance::diagnostics) fn observed_batch_mode(
    ctx: &Context,
    ids: &[String],
) -> Result<RouterMode> {
    match native_auth::resolve_mode(ctx) {
        Ok(mode) => Ok(mode),
        Err(error)
            if ids.iter().any(|id| id == "config_toml_syntax")
                && !ids.iter().any(|id| native_auth::handles(id)) =>
        {
            // Invalid TOML cannot establish effective provider ownership. Only
            // actual persisted routing records may establish this syntax-only mode.
            if let Some(native) = super::super::route_mode::native(ctx)? {
                if native.enabled() {
                    return Ok(native);
                }
                if super::super::router_intent::read(ctx)?.is_none() {
                    return Ok(native);
                }
            }
            if let Some(intent) = super::super::router_intent::read(ctx)? {
                return Ok(
                    if intent.target
                        == atelier_maintenance_core::transition_journal::TransitionTarget::RouterOn
                    {
                        RouterMode::Native
                    } else {
                        RouterMode::Off
                    },
                );
            }
            // relay_storage_load.c:160-208 returns default state on absent
            // storage; serialize_2.c:51-66 maps default bytes +16c/+17e to
            // enabled=false/noAccount=false. This is an observed absence state,
            // not a fabricated RelayStateDocument. Permission/I/O errors propagate.
            let absent = |path: &Path| -> Result<bool> {
                match fs::symlink_metadata(path) {
                    Err(e) if e.kind() == io::ErrorKind::NotFound => Ok(true),
                    Err(e) => Err(err(e)),
                    Ok(_) => Ok(false),
                }
            };
            if absent(&ctx.home.join("codexmate/relay/state.json"))?
                && absent(&ctx.app_root.join("gateway/catalog.json"))?
                && absent(&super::super::router_intent::path(ctx))?
            {
                return Ok(RouterMode::Off);
            }
            Err(error)
        }
        Err(error) => Err(error),
    }
}
fn modified(path: &Path) -> Result<Option<std::time::SystemTime>> {
    match fs::symlink_metadata(path) {
        Ok(m) => Ok(m.modified().ok()),
        Err(e) if e.kind() == io::ErrorKind::NotFound => Ok(None),
        Err(e) => Err(err(e)),
    }
}
fn prep_index(kind: ConfigPreparation) -> u8 {
    match kind {
        ConfigPreparation::Syntax => 0,
        ConfigPreparation::ThirdParty => 1,
        ConfigPreparation::Profile => 2,
        ConfigPreparation::Stale => 3,
    }
}
fn prep_id(index: u8) -> &'static str {
    [
        "config_toml_syntax",
        "config_third_party",
        "config_profile_conflict",
        "config_stale",
    ][index as usize]
}
fn apply_text(
    ctx: &Context,
    index: u8,
    before: Option<&[u8]>,
    known: &std::collections::HashSet<String>,
) -> Result<Option<Vec<u8>>> {
    let bytes = before.unwrap_or_default();
    let text = std::str::from_utf8(bytes).map_err(|_| "批次配置不是UTF8")?;
    let after = match index {
        0 => match super::super::config_syntax::plan_config_toml_syntax_rescue(bytes) {
            super::super::config_syntax::RescuePlan::None => return Ok(before.map(<[u8]>::to_vec)),
            super::super::config_syntax::RescuePlan::Plan { after, .. } => return Ok(Some(after)),
        },
        1 => {
            config::fix_config_third_party_text(text, &ctx.home, known)
                .map_err(err)?
                .after
        }
        2 => config::fix_config_profile_conflict_text(text, &ctx.home).after,
        3 => config::fix_config_stale_text(text, &ctx.home).after,
        _ => unreachable!(),
    };
    Ok(if before.is_none() && after.is_empty() {
        None
    } else {
        Some(after.into_bytes())
    })
}
fn add_candidate(candidates: &mut Vec<Candidate>, bytes: Option<Vec<u8>>) -> usize {
    if let Some(index) = candidates.iter().position(|c| c.bytes == bytes) {
        return index;
    }
    let index = candidates.len();
    candidates.push(Candidate {
        bytes,
        next: BTreeMap::new(),
    });
    index
}
pub(in crate::maintenance::diagnostics) fn prepare_batch(
    ctx: &Context,
    ids: &[String],
    backup: &Path,
) -> Result<PreparedBatch> {
    let mode = observed_batch_mode(ctx, ids)?;
    let before = read(&ctx.config())?;
    let auth = native_auth::prepare_sequence(ctx, ids, backup, mode)?;
    let mut dependencies = BTreeMap::new();
    for path in [
        ctx.home.join("codexmate/relay/state.json"),
        ctx.app_root.join("gateway/router-intent.json"),
        ctx.app_root.join("gateway/catalog.json"),
    ] {
        insert_dependency(&mut dependencies, path.clone(), stamp(&path)?)?;
    }

    insert_dependency(
        &mut dependencies,
        ctx.config(),
        before.as_deref().map(stamp_bytes),
    )?;
    for p in &auth {
        for (path, value) in native_auth::approved_dependencies(p) {
            insert_dependency(&mut dependencies, path, tuple_stamp(value))?;
        }
    }
    let (known, known_deps) = if ids.iter().any(|id| id == "config_third_party") {
        super::super::native_config::known_provider_ids_with_dependencies(ctx)?
    } else {
        (std::collections::HashSet::new(), Vec::new())
    };
    for (path, value) in known_deps {
        insert_dependency(&mut dependencies, path, tuple_stamp(value))?;
    }
    let mut candidates = vec![Candidate {
        bytes: before.clone(),
        next: BTreeMap::new(),
    }];
    let mut takeover_candidate = None;
    for p in &auth {
        if let Some((path, original, after)) = native_auth::planned_config(p) {
            if path != ctx.config() || original != before {
                return Err("认证恢复配置不属于批次原始源".into());
            }
            takeover_candidate = Some(add_candidate(&mut candidates, after));
        }
    }
    // Retain both failure (unchanged candidate) and success branches. Four native
    // preparations give a finite, reviewable graph; no runtime source is signed.
    for index in 0..4 {
        if !ids.iter().any(|id| id == prep_id(index)) {
            continue;
        }
        let count = candidates.len();
        for from in 0..count {
            let outcome = apply_text(ctx, index, candidates[from].bytes.as_deref(), &known)
                .map(|after| add_candidate(&mut candidates, after));
            candidates[from].next.insert(index, outcome);
        }
    }
    let modified = dependencies
        .keys()
        .map(|p| Ok((p.clone(), modified(p)?)))
        .collect::<Result<BTreeMap<_, _>>>()?;
    let plan = PreparedBatch(Arc::new(BatchData {
        home: ctx.home.clone(),
        app_root: ctx.app_root.clone(),
        backup: backup.to_owned(),
        mode,
        ids: ids.to_vec(),
        auth,
        dependencies,
        modified,
        candidates,
        takeover_candidate,
    }));
    validate_batch_initial(ctx, &plan)?;
    Ok(plan)
}
pub(in crate::maintenance::diagnostics) fn validate_batch_initial(
    ctx: &Context,
    plan: &PreparedBatch,
) -> Result<()> {
    if ctx.home != plan.0.home || ctx.app_root != plan.0.app_root {
        return Err("批次审批目录不匹配".into());
    }
    if observed_batch_mode(ctx, &plan.0.ids)? != plan.0.mode {
        return Err("批次路由模式已变化".into());
    }
    validate_ledger(&plan.0.dependencies, &BTreeMap::new())?;
    for (path, expected) in &plan.0.modified {
        if modified(path)? != *expected {
            return Err("批次原始依赖时间戳已变化".into());
        }
    }
    for auth in &plan.0.auth {
        native_auth::validate(ctx, auth, plan.0.mode)?;
    }
    Ok(())
}
pub(in crate::maintenance::diagnostics) fn begin_batch(
    ctx: &Context,
    plan: &PreparedBatch,
) -> Result<BatchSession> {
    validate_batch_initial(ctx, plan)?;
    private_dir(&plan.0.backup)?;
    if let Some(bytes) = plan.config_before() {
        atomic_private(&plan.0.backup.join("batch-original-config.bin"), bytes)?;
    }
    validate_batch_initial(ctx, plan)?;
    Ok(BatchSession {
        prefix: ApprovedBatchPrefix {
            plan: plan.clone(),
            expected: plan.0.dependencies.clone(),
            candidate: 0,
            poisoned: false,
            modified: plan.0.modified.clone(),
        },
        called_auth: BTreeSet::new(),
        called_config: BTreeSet::new(),
    })
}
fn publish_candidate(ctx: &Context, session: &mut BatchSession, key: usize) -> Result<()> {
    session.prefix.validate_current(ctx)?;
    let bytes = session.prefix.plan.0.candidates[key].bytes.clone();
    if read(&ctx.config())? != bytes {
        match &bytes {
            Some(bytes) => atomic_private(&ctx.config(), bytes)?,
            None => fs::remove_file(ctx.config()).map_err(err)?,
        }
    }
    if read(&ctx.config())? != bytes {
        session.prefix.poisoned = true;
        return Err("批次配置发布读回不匹配".into());
    }
    session
        .prefix
        .expected
        .insert(ctx.config(), bytes.as_deref().map(stamp_bytes));
    session
        .prefix
        .modified
        .insert(ctx.config(), modified(&ctx.config())?);
    session.prefix.candidate = key;
    Ok(())
}
pub(in crate::maintenance::diagnostics) fn batch_auth(
    ctx: &Context,
    session: &mut BatchSession,
    id: &str,
) -> Result<native_auth::Outcome> {
    session.prefix.validate_current(ctx)?;
    if !session.called_auth.insert(id.to_owned()) {
        return Err("认证批次项目不得重复执行".into());
    }
    let action = session
        .prefix
        .plan
        .0
        .auth
        .iter()
        .find(|p| native_auth::item_id(p) == id)
        .cloned()
        .ok_or("项目未在批次审批中")?;
    if native_auth::planned_config(&action).is_some() {
        let key = session
            .prefix
            .plan
            .0
            .takeover_candidate
            .ok_or("缺少已审核认证配置候选")?;
        publish_candidate(ctx, session, key)?;
    }
    let prefix = session.prefix.effects();
    let fresh = native_auth::replan_after_approved_effects(
        ctx,
        &action,
        session.prefix.plan.mode(),
        session.prefix.plan.config_before(),
        session.prefix.config_after(),
        &prefix,
    )?;
    let oracle = native_auth::approved_effects(&fresh);
    let result = native_auth::apply_after_approved_effects(
        ctx,
        &action,
        session.prefix.plan.mode(),
        session.prefix.plan.config_before(),
        session.prefix.config_after(),
        &prefix,
    );
    let mut all_after = true;
    // An ordinary failed primitive can have a precisely known partial prefix.
    // Certify only original-before or this fresh action's precomputed after.
    for (path, after) in oracle {
        if path == ctx.config() {
            continue;
        }
        let actual = stamp(&path)?;
        let approved = tuple_stamp(after);
        if actual == approved {
            session.prefix.expected.insert(path.clone(), approved);
            session
                .prefix
                .modified
                .insert(path.clone(), modified(&path)?);
        } else if session.prefix.expected.get(&path) == Some(&actual) {
            all_after = false;
        } else {
            session.prefix.poisoned = true;
            return Err("认证部分效果既非原before也非已审核fresh after，禁止后继写入".into());
        }
    }
    match result {
        Err(error) => Err(error),
        Ok((mut outcome, _)) => {
            if outcome.success && !all_after {
                outcome.success = false;
                outcome.detail.push_str("；部分已审核效果未完成");
            }
            Ok(outcome)
        }
    }
}

pub(in crate::maintenance::diagnostics) fn batch_prepare_config(
    ctx: &Context,
    session: &mut BatchSession,
    kind: ConfigPreparation,
) -> Result<String> {
    session.prefix.validate_current(ctx)?;
    let index = prep_index(kind);
    if !session.called_config.insert(index) {
        return Err("配置批次项目不得重复执行".into());
    }
    let candidate = &session.prefix.plan.0.candidates[session.prefix.candidate];
    let next = candidate
        .next
        .get(&index)
        .ok_or("配置阶段未在原批次候选中批准")?
        .clone()?;
    publish_candidate(ctx, session, next)?;
    Ok(format!("{}：已按原始审批候选完成文本准备", prep_id(index)))
}

#[cfg(test)]
mod tests {
    use super::*;
    struct Fixture {
        root: PathBuf,
        ctx: Context,
    }
    impl Fixture {
        fn new(config: &[u8]) -> Self {
            let root = std::env::temp_dir().join(format!(
                "atelier-reviewed-native-batch-{}",
                uuid::Uuid::new_v4()
            ));
            let ctx = Context {
                home: root.join("codex"),
                app_root: root.join("app"),
                desktop_running: false,
                runtime: None,
                official_catalog: None,
                proxy_handle: None,
                working_directory: None,
                sqlite_home_env: None,
            };
            private_dir(&ctx.home.join("codexmate/relay")).unwrap();
            private_dir(&ctx.app_root).unwrap();
            atomic_private(&ctx.config(), config).unwrap();
            atomic_private(&ctx.home.join("codexmate/relay/state.json"),br#"{"schemaVersion":16,"codexRouterEnabled":false,"providers":[],"activeByIde":{}}"#).unwrap();
            Self { root, ctx }
        }
        fn plan(&self, ids: &[&str]) -> PreparedBatch {
            prepare_batch(
                &self.ctx,
                &ids.iter().map(|s| s.to_string()).collect::<Vec<_>>(),
                &self.root.join("backups"),
            )
            .unwrap()
        }
    }
    impl Drop for Fixture {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.root);
        }
    }
    #[test]
    fn reviewed_batch_syntax_then_profile_writes_only_precomputed_candidates() {
        let f=Fixture::new(b"model='x'\nprofile='legacy'\n# >>> aimami-relay managed start (DO NOT EDIT MANUALLY)\nmanaged = [broken\n# <<< aimami-relay managed end\n");
        let plan = f.plan(&["config_toml_syntax", "config_profile_conflict"]);
        let mut session = begin_batch(&f.ctx, &plan).unwrap();
        batch_prepare_config(&f.ctx, &mut session, ConfigPreparation::Syntax).unwrap();
        let syntax = fs::read(f.ctx.config()).unwrap();
        assert!(std::str::from_utf8(&syntax)
            .unwrap()
            .parse::<toml_edit::DocumentMut>()
            .is_ok());
        assert_eq!(session.prefix().config_after(), Some(syntax.as_slice()));
        batch_prepare_config(&f.ctx, &mut session, ConfigPreparation::Profile).unwrap();
        let after = fs::read(f.ctx.config()).unwrap();
        assert_eq!(session.prefix().config_after(), Some(after.as_slice()));
        assert!(plan
            .config_candidates()
            .iter()
            .any(|(_, v)| v.as_deref() == Some(after.as_slice())));
        assert_eq!(
            fs::read(f.root.join("backups/batch-original-config.bin")).unwrap(),
            plan.config_before().unwrap()
        );
    }
    #[test]
    fn reviewed_batch_failed_preparation_keeps_approved_original_branch() {
        let f = Fixture::new(b"model_provider='openai'\nprofile='old'\n");
        let plan = f.plan(&["config_profile_conflict", "config_stale"]);
        let mut session = begin_batch(&f.ctx, &plan).unwrap();
        // A failed/skipped profile stage leaves its original approved branch.
        batch_prepare_config(&f.ctx, &mut session, ConfigPreparation::Stale).unwrap();
        session.prefix().validate_current(&f.ctx).unwrap();
        assert!(batch_prepare_config(&f.ctx, &mut session, ConfigPreparation::Stale).is_err());
    }
    #[test]
    fn reviewed_batch_current_config_is_never_resigned_as_a_candidate() {
        let f = Fixture::new(b"profile='old'\n");
        let plan = f.plan(&["config_profile_conflict"]);
        let mut session = begin_batch(&f.ctx, &plan).unwrap();
        atomic_private(&f.ctx.config(), b"profile='unreviewed'\n").unwrap();
        assert!(batch_prepare_config(&f.ctx, &mut session, ConfigPreparation::Profile).is_err());
        assert_eq!(fs::read(f.ctx.config()).unwrap(), b"profile='unreviewed'\n");
        assert!(!session
            .prefix()
            .plan()
            .same_plan(&f.plan(&["config_profile_conflict"])));
    }
    #[test]
    fn reviewed_batch_integrity_a_precedes_unlock_and_preserves_login_a() {
        let f = Fixture::new(b"model_provider='openai'\n");
        private_dir(&f.ctx.home.join("accounts/backups")).unwrap();
        let a = br#"{"OPENAI_API_KEY":"sk-synthetic-account-A","fixture":"A"}"#;
        let b = br#"{"OPENAI_API_KEY":"sk-synthetic-unlock-B","fixture":"B"}"#;
        atomic_private(&f.ctx.home.join("accounts/backups/A.json"), a).unwrap();
        atomic_private(
            &f.ctx
                .home
                .join("codexmate/relay/router-unlock-auth-marker.json"),
            br#"{"backedUpPreviousAuth":true}"#,
        )
        .unwrap();
        atomic_private(
            &f.ctx
                .home
                .join("codexmate/relay/router-unlock-auth-backup.json"),
            b,
        )
        .unwrap();
        let plan = f.plan(&["router_unlock_auth_residue", "auth_integrity"]);
        let mut session = begin_batch(&f.ctx, &plan).unwrap();
        assert!(
            batch_auth(&f.ctx, &mut session, "auth_integrity")
                .unwrap()
                .success
        );
        assert!(
            batch_auth(&f.ctx, &mut session, "router_unlock_auth_residue")
                .unwrap()
                .success
        );
        assert_eq!(fs::read(f.ctx.home.join("auth.json")).unwrap(), a);
        assert!(!f
            .ctx
            .home
            .join("codexmate/relay/router-unlock-auth-marker.json")
            .exists());
        assert!(session
            .prefix()
            .effect_stamp(&f.ctx.home.join("auth.json"))
            .is_some());
        session.prefix().validate_current(&f.ctx).unwrap();
    }
    #[test]
    fn reviewed_batch_new_auth_candidate_rejects_before_any_backup_or_write() {
        let f = Fixture::new(b"model_provider='openai'\n");
        private_dir(&f.ctx.home.join("accounts/backups")).unwrap();
        atomic_private(
            &f.ctx.home.join("accounts/backups/A.json"),
            br#"{"OPENAI_API_KEY":"sk-synthetic-A"}"#,
        )
        .unwrap();
        let plan = f.plan(&["auth_integrity"]);
        atomic_private(
            &f.ctx.home.join("accounts/backups/B.json"),
            br#"{"OPENAI_API_KEY":"sk-synthetic-B"}"#,
        )
        .unwrap();
        assert!(begin_batch(&f.ctx, &plan).is_err());
        assert!(!f.root.join("backups").exists());
        assert!(!f.ctx.home.join("auth.json").exists());
    }
    #[test]
    fn reviewed_batch_global_is_ninth_ordinary_stage_over_approved_pure_prefix() {
        let f =
            Fixture::new(b"profile='old'\nmodel_reasoning_effort=''\nmodel_provider='openai'\n");
        let plan = f.plan(&["config_profile_conflict", "config_global_validity"]);
        let global = prepare_batch_global(&f.ctx, &plan, None).unwrap();
        let guard = super::super::super::repair::acquire(&f.ctx).unwrap();
        let mut session = begin_batch(&f.ctx, &plan).unwrap();
        batch_prepare_config(&f.ctx, &mut session, ConfigPreparation::Profile).unwrap();
        let threads = super::super::super::native_threads::begin_reviewed_after_batch_prefix(
            &f.ctx,
            &[],
            &Default::default(),
            session.prefix(),
            None,
        )
        .unwrap();
        let (_, proof) = apply_batch_global(&f.ctx, &global, threads.effects(), &guard).unwrap();
        proof.validate_current(&f.ctx).unwrap();
        assert!(String::from_utf8(fs::read(f.ctx.config()).unwrap())
            .unwrap()
            .find("model_reasoning_effort")
            .is_none());
        let (before, after) = proof.effect_stamp(&f.ctx.config()).unwrap();
        assert_ne!(before, after);
    }
    #[test]
    fn reviewed_batch_global_rejects_unapproved_config_after_thread_certificate() {
        let f = Fixture::new(b"model_reasoning_effort=''\nmodel_provider='openai'\n");
        let plan = f.plan(&["config_global_validity"]);
        let global = prepare_batch_global(&f.ctx, &plan, None).unwrap();
        let guard = super::super::super::repair::acquire(&f.ctx).unwrap();
        let session = begin_batch(&f.ctx, &plan).unwrap();
        let threads = super::super::super::native_threads::begin_reviewed_after_batch_prefix(
            &f.ctx,
            &[],
            &Default::default(),
            session.prefix(),
            None,
        )
        .unwrap();
        atomic_private(&f.ctx.config(), b"unreviewed='current'\n").unwrap();
        assert!(apply_batch_global(&f.ctx, &global, threads.effects(), &guard).is_err());
        assert_eq!(fs::read(f.ctx.config()).unwrap(), b"unreviewed='current'\n");
    }
}

struct GlobalCandidate {
    before: Option<Vec<u8>>,
    after: Result<Option<Vec<u8>>>,
    metadata: BTreeMap<PathBuf, std::result::Result<(), io::ErrorKind>>,
}
pub(in crate::maintenance::diagnostics) struct PreparedGlobal {
    batch: PreparedBatch,
    candidates: Vec<GlobalCandidate>,
    original_metadata: BTreeMap<PathBuf, std::result::Result<(), io::ErrorKind>>,
}
struct GlobalFs<'a> {
    observed: ObservedMetadata,
    catalog: Option<&'a Path>,
}
impl ConfigFs for GlobalFs<'_> {
    fn metadata(&self, path: &Path) -> io::Result<()> {
        if self.catalog == Some(path) {
            self.observed.0.borrow_mut().insert(path.to_owned(), Ok(()));
            Ok(())
        } else {
            self.observed.metadata(path)
        }
    }
}
impl PreparedGlobal {
    /// Exact branch identity is retained even when two branches have equal bytes.
    pub fn successor_inputs(&self) -> Vec<(usize, Option<Vec<u8>>)> {
        self.candidates
            .iter()
            .enumerate()
            .filter_map(|(key, c)| c.after.clone().ok().map(|after| (key, after)))
            .collect()
    }
    pub fn validate_branch(&self, ctx: &Context, key: usize, bytes: Option<&[u8]>) -> Result<()> {
        self.validate_initial_context(ctx)?;
        let candidate = self.candidates.get(key).ok_or("Global后继分支未预审")?;
        if candidate.after.clone()?.as_deref() != bytes {
            return Err("Global后继字节不属于精确原分支".into());
        }
        for (path, expected) in &candidate.metadata {
            if fs::metadata(path).map(|_| ()).map_err(|e| e.kind()) != *expected {
                return Err("Global后继原分支metadata变化".into());
            }
        }
        Ok(())
    }
    fn validate_initial_context(&self, ctx: &Context) -> Result<()> {
        if self.batch.0.home != ctx.home || self.batch.0.app_root != ctx.app_root {
            return Err("全局配置审批目录变化".into());
        }
        Ok(())
    }
    pub fn approved_outputs(&self) -> Vec<Option<Vec<u8>>> {
        self.candidates
            .iter()
            .filter_map(|c| c.after.clone().ok())
            .collect()
    }
    pub fn validate_initial(&self, ctx: &Context) -> Result<()> {
        if self.batch.0.home != ctx.home || self.batch.0.app_root != ctx.app_root {
            return Err("全局配置审批目录变化".into());
        }
        for (path, before) in &self.original_metadata {
            if fs::metadata(path).map(|_| ()).map_err(|e| e.kind()) != *before {
                return Err("全局配置引用的原始metadata依赖已变化".into());
            }
        }
        Ok(())
    }
    pub fn preview_lines(&self) -> Vec<String> {
        vec![format!(
            "config_global_validity：在原生第9普通阶段清理；{} 个已审核前序配置候选",
            self.candidates.len()
        )]
    }
}
pub(in crate::maintenance::diagnostics) fn prepare_batch_global(
    ctx: &Context,
    batch: &PreparedBatch,
    router: Option<&super::super::native_router::BoundBatchResyncToken>,
) -> Result<PreparedGlobal> {
    let mut inputs = batch
        .config_candidates()
        .into_iter()
        .map(|(_, b)| (b, false))
        .collect::<Vec<_>>();
    let catalog = router
        .and_then(|r| r.catalog_effect())
        .and_then(|(p, b)| b.map(|_| p));
    if let Some(router) = router {
        for (_, outcome) in router.config_candidates() {
            if let Ok(bytes) = outcome {
                inputs.push((bytes, true));
            }
        }
    }
    let mut candidates = vec![];
    let mut original_metadata = BTreeMap::new();
    for (before, after_resync) in inputs {
        let observed = GlobalFs {
            observed: ObservedMetadata::default(),
            catalog: if after_resync {
                catalog.as_deref()
            } else {
                None
            },
        };
        let after = (|| {
            let text = std::str::from_utf8(before.as_deref().unwrap_or_default())
                .map_err(|_| "全局配置候选不是UTF8")?;
            text.parse::<toml_edit::DocumentMut>()
                .map_err(|_| "全局配置候选TOML无效")?;
            let fixed = config::fix_config_global_validity_at(
                text,
                &ctx.config(),
                &ctx.home.join("codexmate/relay"),
                &observed,
            )
            .map_err(err)?
            .after;
            Ok(if before.is_none() && fixed.is_empty() {
                None
            } else {
                Some(fixed.into_bytes())
            })
        })();
        let metadata = observed.observed.0.into_inner();
        for path in metadata.keys() {
            original_metadata.insert(
                path.clone(),
                fs::metadata(path).map(|_| ()).map_err(|e| e.kind()),
            );
        }
        candidates.push(GlobalCandidate {
            before,
            after,
            metadata,
        });
    }
    let plan = PreparedGlobal {
        batch: batch.clone(),
        candidates,
        original_metadata,
    };
    plan.validate_initial(ctx)?;
    Ok(plan)
}
#[derive(Clone)]
pub(in crate::maintenance::diagnostics) struct ApprovedGlobalEffects {
    branch: usize,
    home: PathBuf,
    app_root: PathBuf,
    effects: BTreeMap<
        PathBuf,
        (
            Option<super::super::native_orphans::EffectStamp>,
            Option<super::super::native_orphans::EffectStamp>,
        ),
    >,
}
fn full_stamp(path: &Path) -> Result<Option<super::super::native_orphans::EffectStamp>> {
    Ok(stamp(path)?.map(|s| (s.length as u64, s.sha256, modified(path).ok().flatten())))
}
impl ApprovedGlobalEffects {
    pub fn branch(&self) -> usize {
        self.branch
    }
    pub fn paths(&self) -> Vec<PathBuf> {
        self.effects.keys().cloned().collect()
    }
    pub fn effect_stamp(
        &self,
        path: &Path,
    ) -> Option<(
        Option<super::super::native_orphans::EffectStamp>,
        Option<super::super::native_orphans::EffectStamp>,
    )> {
        self.effects.get(path).cloned()
    }
    pub fn validate_current(&self, ctx: &Context) -> Result<()> {
        if self.home != ctx.home || self.app_root != ctx.app_root {
            return Err("全局配置效果证书目录不匹配".into());
        }
        for (path, (_, after)) in &self.effects {
            if full_stamp(path)? != *after {
                return Err("全局配置已批准效果被外部更改".into());
            }
        }
        Ok(())
    }
}
pub(in crate::maintenance::diagnostics) fn apply_batch_global(
    ctx: &Context,
    plan: &PreparedGlobal,
    prefix: &super::super::native_threads::ApprovedThreadEffects,
    guard: &super::super::repair::MutationGuard,
) -> Result<(String, ApprovedGlobalEffects)> {
    guard.validate(ctx)?;
    prefix.validate_current(ctx)?;
    // Only the precise final edge produced by the preceding actual stages can
    // explain a changed original config. An arbitrary current config is rejected.
    let current = read(&ctx.config())?;
    let original = plan.batch.config_before().map(<[u8]>::to_vec);
    let original_stamp = original.as_deref().map(stamp_bytes);
    if let Some((before, after)) = prefix.effect_stamp(&ctx.config()) {
        if before.map(|(n, h, _)| Stamp {
            length: n as usize,
            sha256: h,
        }) != original_stamp
            || after != full_stamp(&ctx.config())?
        {
            return Err("全局配置前序效果不属于原始审批".into());
        }
    } else if current != original {
        return Err("全局配置存在未批准前序变化".into());
    }
    let (branch, candidate) = plan
        .candidates
        .iter()
        .enumerate()
        .find(|(_, c)| {
            c.before == current
                && c.metadata
                    .iter()
                    .all(|(p, s)| fs::metadata(p).map(|_| ()).map_err(|e| e.kind()) == *s)
        })
        .ok_or("当前配置或引用metadata不属于任何已审核原生候选")?;
    let after = candidate.after.clone()?;
    let before_stamp = full_stamp(&ctx.config())?;
    if current != after {
        match &after {
            Some(bytes) => atomic_private(&ctx.config(), bytes)?,
            None => fs::remove_file(ctx.config()).map_err(err)?,
        }
    }
    if read(&ctx.config())? != after {
        return Err("全局配置发布读回不匹配".into());
    }
    let proof = ApprovedGlobalEffects {
        branch,
        home: ctx.home.clone(),
        app_root: ctx.app_root.clone(),
        effects: BTreeMap::from([(ctx.config(), (before_stamp, full_stamp(&ctx.config())?))]),
    };
    proof.validate_current(ctx)?;
    Ok(("按原生第9普通阶段完成全局配置清理".into(), proof))
}
