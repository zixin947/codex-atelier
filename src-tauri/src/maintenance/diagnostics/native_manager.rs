//! Production maintenance host. The shared native Manager runs against real
//! Atelier Catalog/RouterIntent or a real native document, never an invented one.
use super::{inspect, native_catalog, native_router, repair, router_intent, *};
use atelier_maintenance_core::{
    catalog::{publication::Publication, state::RouterOperationId},
    manager::{self as core, HostRepairOperations, HostState, RepairSnapshot},
    process::{self, ProcessContext, Runner},
    router_transition,
    router_verification::{
        self as verification, Depth, InvariantDim, VerificationIssue, VerificationReport,
    },
    transition_journal::{JournalStore, TransitionJournal, TransitionTarget},
};
use std::{cell::RefCell, collections::BTreeSet, fmt, io, sync::Arc};

#[derive(Debug)]
pub(super) struct Error(String);
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}
impl std::error::Error for Error {}
impl From<String> for Error {
    fn from(s: String) -> Self {
        Self(s)
    }
}
type HostResult<T> = std::result::Result<T, Error>;

#[derive(Clone)]
struct Snapshot {
    router: Option<native_router::Snapshot>,
    intent: Option<router_intent::Prepared>,
}
impl RepairSnapshot for Snapshot {
    fn target(&self) -> TransitionTarget {
        self.router.as_ref().map_or_else(
            || {
                self.intent
                    .as_ref()
                    .expect("actual observed intent")
                    .observation()
                    .record
                    .target
            },
            native_router::Snapshot::target,
        )
    }
}
fn optional(path: &Path) -> Result<Option<Vec<u8>>> {
    match std::fs::symlink_metadata(path) {
        Ok(_) => read_bounded(path, 8 * 1024 * 1024).map(Some),
        Err(e) if e.kind() == io::ErrorKind::NotFound => Ok(None),
        Err(e) => Err(err(e)),
    }
}
/// Source selection uses existing typed files. An absent gateway remains absent.
/// A running gateway alone does not make Desktop a routed client.
fn snapshot(ctx: &Context) -> Result<Snapshot> {
    let native = native_catalog::Source::Native {
        state_path: ctx.home.join("codexmate/relay/state.json"),
        catalog_root: ctx.home.join("codexmate/relay"),
        bundle: None,
    };
    let native = if optional(&ctx.home.join("codexmate/relay/state.json"))?.is_some() {
        Some(native_router::snapshot(ctx, &native)?)
    } else {
        None
    };
    let gateway_exists = optional(&ctx.app_root.join("gateway/catalog.json"))?.is_some();
    if native
        .as_ref()
        .is_some_and(|s| s.target() == TransitionTarget::RouterOn)
    {
        return Ok(Snapshot {
            router: native,
            intent: None,
        });
    }
    let intent = router_intent::prepare(ctx)?;
    if !gateway_exists
        && native.is_some()
        && intent.observation().record.target == TransitionTarget::NativeOff
    {
        return Ok(Snapshot {
            router: native,
            intent: None,
        });
    }
    let router = if gateway_exists {
        Some(native_router::snapshot(
            ctx,
            &native_catalog::Source::Gateway {
                catalog_path: ctx.app_root.join("gateway/catalog.json"),
                catalog_root: ctx.home.clone(),
                bundle: None,
                route_enabled: intent.observation().record.target == TransitionTarget::RouterOn,
            },
        )?)
    } else {
        None
    };
    if router.is_none() && intent.observation().record.target == TransitionTarget::RouterOn {
        return Err("配置要求 Atelier 路由，但真实网关目录不存在；未构造空目录替代".into());
    }
    Ok(Snapshot {
        router,
        intent: Some(intent),
    })
}
fn paths(ctx: &Context, snapshot: &Snapshot) -> core::RepairPaths {
    let configured = atelier_maintenance_core::state_db::configured_sqlite_home(
        optional(&ctx.config())
            .ok()
            .flatten()
            .as_deref()
            .and_then(|s| std::str::from_utf8(s).ok()),
        ctx.sqlite_home_env.as_deref(),
        ctx.working_directory.as_deref(),
    );
    let directories = configured
        .map(|p| vec![p])
        .unwrap_or_else(|| vec![ctx.home.join("sqlite"), ctx.home.clone()]);
    let mut databases = vec![];
    for directory in directories {
        if let Some(database) = atelier_maintenance_core::state_db::highest_state_db(&directory) {
            if !databases.contains(&database) {
                databases.push(database);
            }
        }
    }
    let native = snapshot
        .router
        .as_ref()
        .is_some_and(|s| s.native_document().is_some());
    let root = if native {
        ctx.home.join("codexmate/relay")
    } else {
        ctx.app_root.join("gateway")
    };
    core::RepairPaths {
        state_path: if native {
            root.join("state.json")
        } else {
            router_intent::path(ctx)
        },
        verification: verification::VerificationPaths {
            codex_home: ctx.home.clone(),
            config_path: ctx.config(),
            relay_root: root,
            databases,
        },
    }
}
fn failed_verification(error: impl Into<String>) -> VerificationReport {
    VerificationReport {
        issues: vec![VerificationIssue {
            dim: InvariantDim::Config,
            message: error.into(),
        }],
    }
}
fn verify(
    ctx: &Context,
    snapshot: &Snapshot,
    paths: &core::RepairPaths,
    target: TransitionTarget,
    depth: Depth,
) -> VerificationReport {
    if let Some(router) = &snapshot.router {
        return native_router::verify_target(ctx, router, target, depth)
            .unwrap_or_else(failed_verification);
    }
    if target != TransitionTarget::NativeOff {
        return failed_verification("缺少真实路由来源，不能验证 RouterOn");
    }
    if depth != Depth::Full {
        return verify_observed_off(ctx, &paths.verification, target, depth);
    }
    super::snapshot::with_verification_paths(&paths.verification, |projected| {
        let mut report = verify_observed_off(ctx, projected, target, depth);
        for issue in &mut report.issues {
            for (copy, source) in projected
                .databases
                .iter()
                .zip(&paths.verification.databases)
            {
                issue.message = issue.message.replace(
                    copy.to_string_lossy().as_ref(),
                    source.to_string_lossy().as_ref(),
                );
            }
        }
        Ok(report)
    })
    .unwrap_or_else(failed_verification)
}
fn verify_observed_off(
    ctx: &Context,
    paths: &verification::VerificationPaths,
    target: TransitionTarget,
    depth: Depth,
) -> VerificationReport {
    let models = BTreeSet::new();
    let proxy = Mutex::new(None);
    let mut report = verification::verify_with_contract(
        &verification::ContractContext {
            paths,
            proxy: &proxy,
        },
        &verification::VerificationContract::AtelierResponses(verification::AtelierContract {
            provider_id: "atelier",
            expected_base_url: "",
            catalog_path: &ctx
                .home
                .join(atelier_maintenance_core::catalog::CATALOG_FILE),
            expected_raw_models: &models,
            selected_model: "",
        }),
        target,
        depth,
    );
    // No persisted state is a real absence, not a default RelayState. NativeOff
    // still has native auth/takeover/config invariants at their actual home.
    let native_paths = verification::VerificationPaths {
        relay_root: ctx.home.join("codexmate/relay"),
        ..paths.clone()
    };
    report
        .issues
        .extend(verification::verify_native_off_with_depth(&native_paths, depth).issues);
    report
}
fn manager_report(report: DiagnosticReport) -> core::DiagnosticReport {
    core::DiagnosticReport {
        items: report
            .items
            .into_iter()
            .map(|i| core::DiagnosticItem {
                id: i.id,
                status: i.status,
                label: i.label,
                detail: i.detail,
                fixable: i.fixable,
            })
            .collect(),
        has_issues: report.has_issues,
        transition_in_flight: report.transition_in_flight,
    }
}
fn proxy_diagnostic(
    ctx: &Context,
    snapshot: std::result::Result<&Snapshot, &String>,
) -> DiagnosticItem {
    let (status, detail) = match snapshot {
        Err(error) => ("warning", format!("路由来源无法核对：{error}")),
        Ok(source) if source.target() == TransitionTarget::NativeOff => (
            "ok",
            "路由目标已关闭；监听器可能仍服务其他请求，不据此视为 Desktop 已接管。".into(),
        ),
        Ok(source)
            if source
                .router
                .as_ref()
                .is_some_and(|s| s.native_document().is_some()) =>
        {
            (
                "warning",
                "当前宿主没有该原生代理的运行句柄，不能从持久化 running 字段确认存活。".into(),
            )
        }
        Ok(source) => {
            let expected = source
                .router
                .as_ref()
                .and_then(native_router::Snapshot::gateway_catalog)
                .map(|c| c.settings.port);
            match &ctx.proxy_handle {
                Some(handle)
                    if expected == Some(handle.port)
                        && verification::proxy_handle_port_accepts(
                            handle,
                            &verification::LocalPortTransport,
                        ) =>
                {
                    (
                        "ok",
                        format!(
                            "已持有当前监听实例并确认 127.0.0.1:{} 可连接；未验证上游生成。",
                            handle.port
                        ),
                    )
                }
                Some(_) => (
                    "warning",
                    "所属代理实例已停止、端口不匹配或本地端口不可连接。".into(),
                ),
                None => (
                    "warning",
                    "没有当前 Atelier 监听实例的运行句柄，不能以状态文本代替存活验证。".into(),
                ),
            }
        }
    };
    DiagnosticItem {
        id: "proxy_running".into(),
        group: "runtime".into(),
        status: status.into(),
        label: "本地代理运行状态".into(),
        detail,
        fixable: false,
    }
}
fn official_diagnostic(
    ctx: &Context,
    snapshot: std::result::Result<&Snapshot, &String>,
) -> DiagnosticItem {
    use super::official_catalog::{self, OfficialCatalogRuntime, Scope};
    let scope = match snapshot {
        Err(error) => Err(error.clone()),
        Ok(snapshot) => match snapshot.router.as_ref() {
            Some(router) if router.native_document().is_some() => {
                Scope::native(router.native_document().expect("native source"))
            }
            Some(router) if router.gateway_catalog().is_some() => {
                let catalog = router.gateway_catalog().expect("gateway source");
                if snapshot.target() == TransitionTarget::NativeOff {
                    Ok(Scope::off())
                } else {
                    catalog
                        .settings
                        .preferred_profile_id
                        .as_deref()
                        .ok_or_else(|| "未选择实际账号来源".to_owned())
                        .and_then(|id| Scope::gateway_at_target(catalog, id, snapshot.target()))
                }
            }
            _ => Ok(Scope::off()),
        },
    };
    let read = scope.and_then(|scope| {
        let empty = OfficialCatalogRuntime::default();
        let runtime = ctx.official_catalog.as_deref().unwrap_or(&empty);
        official_catalog::diagnose(&scope, runtime).map(|diagnostic| diagnostic.item)
    });
    read.unwrap_or_else(|error| DiagnosticItem {
        id: "official_catalog_freshness".into(),
        group: "config".into(),
        status: "warning".into(),
        label: "官方目录来源无法核对".into(),
        detail: error,
        fixable: false,
    })
}
pub(super) fn report(ctx: &Context) -> Result<DiagnosticReport> {
    super::snapshot::with_observation(|| augment_report(ctx, inspect::run(ctx)?, snapshot(ctx)))
}
fn augment_report(
    ctx: &Context,
    mut report: DiagnosticReport,
    snap: Result<Snapshot>,
) -> Result<DiagnosticReport> {
    let proxy = proxy_diagnostic(ctx, snap.as_ref());
    if let Some(item) = report.items.iter_mut().find(|i| i.id == proxy.id) {
        *item = proxy;
    }
    let official = official_diagnostic(ctx, snap.as_ref());
    if let Some(item) = report.items.iter_mut().find(|i| i.id == official.id) {
        *item = official;
    }
    let item = match snap {
        Ok(s) => {
            let result = verify(ctx, &s, &paths(ctx, &s), s.target(), Depth::Full);
            DiagnosticItem {
                id: "router_postcondition".into(),
                group: "config".into(),
                label: "路由完整目标状态".into(),
                status: if result.issues.is_empty() {
                    "ok"
                } else {
                    "error"
                }
                .into(),
                detail: if result.issues.is_empty() {
                    "实际配置与目标状态检查通过；未验证上游生成。".into()
                } else {
                    result
                        .issues
                        .iter()
                        .map(|i| format!("{:?}: {}", i.dim, i.message))
                        .collect::<Vec<_>>()
                        .join("；")
                },
                fixable: !result.issues.is_empty() && s.router.is_some(),
            }
        }
        Err(e) => DiagnosticItem {
            id: "router_postcondition".into(),
            group: "config".into(),
            label: "路由完整目标状态".into(),
            status: "error".into(),
            detail: e,
            fixable: false,
        },
    };
    if let Some(existing) = report.items.iter_mut().find(|i| i.id == item.id) {
        *existing = item;
    } else {
        report.items.push(item);
    }
    let journal_root = ctx.app_root.join("gateway");
    match JournalStore::new(&journal_root).read_checked() {
        Ok(Some(journal)) => {
            report.transition_in_flight |= journal.is_active();
            if let Some(item) = report
                .items
                .iter_mut()
                .find(|i| i.id == "router_recovery_record")
            {
                item.status = if journal.is_active() { "warning" } else { "ok" }.into();
                item.detail = format!("Atelier 路由记录：{:?} / {}", journal.target, journal.phase);
                item.fixable = false;
            }
        }
        Err(e) => {
            if let Some(item) = report
                .items
                .iter_mut()
                .find(|i| i.id == "router_recovery_record")
            {
                item.status = "error".into();
                item.detail = format!("Atelier 路由恢复记录无法读取，未作为空记录处理：{e}");
                item.fixable = false;
            }
        }
        Ok(None) => (),
    }
    report.has_issues = report
        .items
        .iter()
        .any(|i| matches!(i.status.as_str(), "error" | "warning"));
    Ok(report)
}

pub(super) struct Reviewed {
    pub view: DiagnosticRepairPlan,
    pub created_at: u64,
    selected: String,
    home: PathBuf,
    app_root: PathBuf,
    original: Snapshot,
    primary: Option<repair::Ticket>,
    batch: Option<super::native_batch::ReviewedBatch>,
    reconcile: Option<native_router::ReviewedPlan>,
    native_primary: Option<native_router::NativePrimaryComposition>,
    expected_final_config: Option<Vec<u8>>,
    initial_config: Option<Vec<u8>>,
    context: Context,
}
pub(super) fn preview(ctx: &Context, selected: &str) -> Result<Reviewed> {
    let original = snapshot(ctx)?;
    let (ordinary_report, report) = super::snapshot::with_observation(|| {
        let ordinary = inspect::run(ctx)?;
        let report = augment_report(ctx, ordinary.clone(), Ok(original.clone()))?;
        Ok((ordinary, report))
    })?;
    let gates = manager_report(report.clone()).repair_gates(selected)?;
    let has_primary = ordinary_report.items.iter().any(|i| {
        i.fixable && i.id != "router_postcondition" && (selected == "all" || selected == i.id)
    });
    if selected == "all" && has_primary {
        return preview_batch(ctx, original, report, ordinary_report, gates);
    }
    let primary = if has_primary {
        Some(repair::preview(ctx, selected)?)
    } else {
        None
    };
    let needs_postcondition = report.items.iter().any(|i| {
        i.id == "router_postcondition" && i.fixable && (selected == "all" || selected == i.id)
    });
    if primary.is_none() && !needs_postcondition {
        return Err("当前没有可自动修复的项目，请重新检测".into());
    }
    let ticket_id = primary
        .as_ref()
        .map(|p| p.view.ticket.clone())
        .unwrap_or_else(|| format!("router-repair-{}", uuid::Uuid::new_v4()));
    let backup_root = ctx
        .app_root
        .join("maintenance/diagnostic-backups")
        .join(&ticket_id)
        .join("router");
    let reconcile = if gates.use_quiet_transition || gates.direct_postcondition {
        original
            .router
            .as_ref()
            .map(|s| {
                native_router::prepare_reconcile_for_target(
                    ctx,
                    s.source(),
                    original.target(),
                    &backup_root,
                )
            })
            .transpose()?
    } else {
        None
    };
    if original.target() == TransitionTarget::NativeOff
        && (gates.use_quiet_transition || gates.direct_postcondition)
        && inspect::config(ctx).is_ok()
    {
        let changes_provider = primary
            .as_ref()
            .is_some_and(repair::Ticket::changes_provider_target);
        super::native_threads::review_provider_target(ctx, "openai", changes_provider)?;
    }
    let primary_effect = primary.as_ref().and_then(repair::Ticket::config_after);
    let native_primary = if let (Some(plan), Some(after)) = (&reconcile, &primary_effect) {
        if original
            .router
            .as_ref()
            .is_some_and(|s| s.native_document().is_some())
            && plan.config_before() != after.as_deref()
        {
            let ids = primary
                .as_ref()
                .expect("primary effect")
                .selected_ids()
                .iter()
                .filter(|id| {
                    matches!(
                        id.as_str(),
                        "config_global_validity" | "config_profile_conflict"
                    )
                })
                .cloned()
                .collect::<Vec<_>>();
            Some(native_router::prepare_native_primary_composition(
                ctx,
                plan,
                &ids,
                after.as_deref(),
            )?)
        } else {
            None
        }
    } else {
        None
    };
    let expected_final_config = match (&reconcile, &primary_effect) {
        (_, _) if native_primary.is_some() => native_primary
            .as_ref()
            .and_then(|t| t.config_after().map(ToOwned::to_owned)),
        (Some(plan), None) => plan.config_after().map(ToOwned::to_owned),
        (Some(_), Some(after)) => {
            let snap = original.router.as_ref().expect("reviewed source");
            native_router::compose_config_after(ctx, snap, original.target(), after.as_deref())?
        }
        (None, Some(after)) => after.clone(),
        (None, None) => optional(&ctx.config())?,
    };
    let mut view =
        primary
            .as_ref()
            .map(|p| p.view.clone())
            .unwrap_or_else(|| DiagnosticRepairPlan {
                ticket: ticket_id,
                item_ids: vec!["router_postcondition".into()],
                title: "预览路由目标状态修复".into(),
                changes: vec![],
                warnings: vec![],
                requires_desktop_closed: true,
            });
    if let Some(plan) = &reconcile {
        view.changes.extend(
            native_router::preview_lines(plan)
                .into_iter()
                .filter(|line| !line.starts_with("配置：")),
        );
        view.changes.push(format!(
            "配置：{}（原始 {} → 最终批准 {} 字节）",
            ctx.config().display(),
            optional(&ctx.config())?.as_ref().map_or(0, Vec::len),
            expected_final_config.as_ref().map_or(0, Vec::len)
        ));
    }
    if needs_postcondition && !view.item_ids.iter().any(|id| id == "router_postcondition") {
        view.item_ids.push("router_postcondition".into());
    }
    if gates.use_quiet_transition || gates.direct_postcondition {
        view.warnings
            .retain(|w| !w.starts_with("请先退出 Codex / ChatGPT Desktop"));
        view.warnings.push("确认后将请求 Desktop 正常退出并建立写入静默窗口；必要时结束识别到的 Codex 写入进程。若操作前 Desktop 已运行，验证完成后自动重开。".into());
        view.changes.push(format!(
            "验证修复目标后提交路由回执：{}",
            paths(ctx, &original).state_path.display()
        ));
        // The Manager owns the stop/verify/relaunch lifecycle; users need not stop it manually.
        view.requires_desktop_closed = false;
    }
    Ok(Reviewed {
        view,
        created_at: now_ms(),
        selected: selected.into(),
        home: ctx.home.clone(),
        app_root: ctx.app_root.clone(),
        original,
        primary,
        batch: None,
        reconcile,
        native_primary,
        expected_final_config,
        initial_config: optional(&ctx.config())?,
        context: ctx.clone(),
    })
}

/// The all-items entry reviews the original native batch rather than preparing
/// and then replaying the old combined single-item ticket.
fn preview_batch(
    ctx: &Context,
    original: Snapshot,
    report: DiagnosticReport,
    ordinary_report: DiagnosticReport,
    gates: atelier_maintenance_core::policy::RepairGates,
) -> Result<Reviewed> {
    let batch = super::native_batch::prepare_from_report(
        ctx,
        original.router.as_ref().map(|router| router.source()),
        &ordinary_report,
    )?;
    if batch.target() != original.target() {
        return Err("批次与 Manager 的真实路由目标不一致，请重新检测".into());
    }
    let mut view = batch.view().clone();
    if report
        .items
        .iter()
        .any(|i| i.id == "router_postcondition" && i.fixable)
    {
        view.item_ids.push("router_postcondition".into());
    }
    if gates.use_quiet_transition {
        view.requires_desktop_closed = false;
        view.warnings.push("确认后由维护流程建立写入静默窗口；仅在最终目标验证通过后提交回执，并按操作前状态恢复 Desktop。".into());
    }
    let initial_config = optional(&ctx.config())?;
    Ok(Reviewed {
        view,
        created_at: now_ms(),
        selected: "all".into(),
        home: ctx.home.clone(),
        app_root: ctx.app_root.clone(),
        original,
        primary: None,
        batch: Some(batch),
        reconcile: None,
        native_primary: None,
        expected_final_config: initial_config.clone(),
        initial_config,
        context: ctx.clone(),
    })
}

struct Host<'a, R> {
    context: &'a Context,
    current: RefCell<Snapshot>,
    paths: &'a core::RepairPaths,
    processes: &'a ProcessContext,
    runner: Arc<R>,
    read_runtime: &'a dyn Fn() -> (
        Option<crate::gateway::types::RuntimeView>,
        Option<verification::ProxyHandle>,
    ),
}
impl<R: Runner> Host<'_, R> {
    fn context(&self) -> HostResult<Context> {
        let mut context = self.context.clone();
        let rows = process::list_codex_processes_matching(self.runner.as_ref(), self.processes)
            .map_err(|e| Error(e.to_string()))?;
        context.desktop_running =
            !rows.is_empty() || process::is_codex_app_running(self.runner.as_ref(), self.processes);
        let (runtime, proxy) = (self.read_runtime)();
        context.runtime = runtime;
        context.proxy_handle = proxy;
        Ok(context)
    }
    fn publish(
        &self,
        operation: &RouterOperationId,
        config_after: Option<&[u8]>,
    ) -> HostResult<Publication> {
        let ctx = self.context()?;
        let current = self.current.borrow().clone();
        let publication = if let Some(intent) = &current.intent {
            router_intent::commit_after_approved_config(
                &ctx,
                intent,
                config_after,
                operation,
                now_ms() as i64,
            )
            .map_err(Error)?
            .publication
        } else {
            let router = current
                .router
                .as_ref()
                .ok_or_else(|| Error("真实原生状态缺失".into()))?;
            let document = router
                .native_document()
                .ok_or_else(|| Error("非原生来源不能提交原生状态".into()))?
                .clone();
            let (source_path, expected) = router.source_dependency();
            if stamp(optional(&source_path).map_err(Error)?.as_deref()) != expected {
                return Err(Error("原生状态已被外部更新，未覆盖".into()));
            }
            let state = core::StateHandle::new(document.clone());
            let proxy = Mutex::new(None);
            let catalog = Mutex::new(core::CatalogRuntime::<()>::default());
            core::persist_with_commit(
                core::PersistenceContext {
                    paths: self.paths,
                    state: &state,
                    proxy: &proxy,
                    catalog: &catalog,
                    initialization_error: None,
                },
                document,
                Some(operation.clone()),
            )
            .map_err(|e| Error(e.to_string()))?
            .publication
        };
        *self.current.borrow_mut() = snapshot(&ctx).map_err(Error)?;
        Ok(publication)
    }
}

struct Operations<'a> {
    reviewed: &'a Reviewed,
    guard: &'a repair::MutationGuard,
    primary_result: RefCell<Option<core::FixResult>>,
    batch_receipt: RefCell<Option<super::native_batch::BatchReceipt>>,
    backup: RefCell<Option<String>>,
    reconciled: RefCell<bool>,
    active_reconcile: RefCell<Option<native_router::ReviewedPlan>>,
}
impl Operations<'_> {
    fn batch_config(&self, ctx: &Context) -> HostResult<Option<Option<Vec<u8>>>> {
        let Some(batch) = &self.reviewed.batch else {
            return Ok(None);
        };
        let receipt = self.batch_receipt.borrow();
        let receipt = receipt
            .as_ref()
            .ok_or_else(|| Error("批次尚无最终已批准效果凭据，未提交路由回执".into()))?;
        receipt.validate_current(ctx, batch).map_err(Error)?;
        Ok(Some(receipt.config_after().map(ToOwned::to_owned)))
    }
    fn restore<R: Runner + Send + Sync + 'static>(
        &self,
        source: &Snapshot,
        reason: &str,
        host: &Host<'_, R>,
    ) -> HostResult<Vec<String>> {
        let ctx = host.context()?;
        let current = optional(&ctx.config()).map_err(Error)?;
        let primary = self
            .reviewed
            .primary
            .as_ref()
            .and_then(repair::Ticket::config_after);
        let batch_config = if let Some(batch) = &self.reviewed.batch {
            match self.batch_config(&ctx) {
                Ok(config) => config,
                Err(_) => {
                    host.checked_recovery(host.paths)
                        .map_err(|e| Error(e.to_string()))?;
                    // A conflicting DB/WAL is not authority to replay a route successor.
                    // Independently prove an approved config and the source target instead.
                    if !batch.approves_config(current.as_deref()) {
                        return Err(Error("批次效果未确认且路由配置不属于原审批；未覆盖配置、未回滚会话库，备份与恢复记录保留".into()));
                    }
                    let checked = host.verify_target(host.paths, source.target(), Depth::Full);
                    if checked.issues.is_empty() {
                        return Ok(vec![]);
                    }
                    return Err(Error(format!(
                        "批次效果未确认；源路由独立复检未通过，未重放批次或覆盖会话库；需重新检测。路由问题：{}",
                        checked.issues.iter().map(|i| i.message.as_str()).collect::<Vec<_>>().join("；")
                    )));
                }
            }
        } else {
            None
        };
        if current != self.reviewed.initial_config
            && current != self.reviewed.expected_final_config
            && primary.as_ref() != Some(&current)
            && batch_config.as_ref() != Some(&current)
        {
            return Err(Error(
                "补偿遇到预览外配置变化，未覆盖外部写入；恢复记录与备份保留".into(),
            ));
        }
        // A pending commit may already have published. The host's actual receipt
        // guard decides whether recovery can proceed; it is never cleared blindly.
        host.checked_recovery(host.paths)
            .map_err(|e| Error(e.to_string()))?;
        let mut action = |_| {
            let checked = host.verify_target(host.paths, source.target(), Depth::Full);
            let (details, warnings) = if checked.issues.is_empty() {
                (vec![format!("源目标状态已回读确认：{reason}")], vec![])
            } else {
                let outcome = self
                    .reconcile(host, source.target())
                    .map_err(core::ManagerError::Primitive)?;
                (outcome.details, outcome.warnings)
            };
            Ok(core::CompletedRepair::from_parts(
                core::FixResult {
                    fixed_ids: vec![],
                    failed_ids: vec![],
                    details,
                },
                Some(()),
                warnings,
            ))
        };
        let request = core::QuietRepairRequest {
            paths: host.paths,
            source,
            original: source,
            selected: "router_postcondition",
            prepared_catalog: None::<&()>,
            include_missing_rollouts: false,
            reason: "diagnostic repair source recovery",
            policy: core::RepairTransitionPolicy::Strict,
            writer_policy: process::ProcessPolicy::ForceCloseAll,
            graceful_timeout: std::time::Duration::from_secs(5),
        };
        let mut commit = |_: &core::CompletedRepair<()>, operation: &RouterOperationId| {
            let ctx = host.context().map_err(core::ManagerError::Primitive)?;
            let after =
                optional(&ctx.config()).map_err(|e| core::ManagerError::Primitive(Error(e)))?;
            host.publish(operation, after.as_deref())
                .map_err(core::ManagerError::Primitive)
        };
        match router_transition::run_host_repair(
            request,
            core::Processes {
                runner: host.runner.clone(),
                context: host.processes,
            },
            host,
            &mut action,
            &mut commit,
        ) {
            core::TransitionOutcome::Verified { warnings, .. } => Ok(warnings),
            core::TransitionOutcome::Unverified {
                recovery_reasons, ..
            } => Err(Error(recovery_reasons.join("；"))),
            core::TransitionOutcome::Failed(failure) => Err(Error(failure.error.to_string())),
        }
    }
    fn apply_primary<R: Runner>(&self, host: &Host<'_, R>) -> HostResult<core::FixResult> {
        let ctx = host.context()?;
        self.guard.validate(&ctx).map_err(Error)?;
        if let Some(prior) = self.primary_result.borrow().as_ref() {
            self.batch_config(&ctx)?;
            // A fence retry must inspect the real current effects, never replay a stale
            // ticket or manufacture success from the first attempt's UI status.
            let report = report(&ctx).map_err(Error)?;
            for id in &prior.fixed_ids {
                if !report.items.iter().any(|i| &i.id == id && i.status == "ok") {
                    return Err(Error(format!(
                        "写入进程重现后 {id} 再次异常；原预览已失效，已保留备份"
                    )));
                }
            }
            return Ok(prior.clone());
        }
        let result = if let Some(batch) = &self.reviewed.batch {
            let execution = super::native_batch::apply_under_manager(
                &ctx,
                batch,
                self.guard,
                host.runner.as_ref(),
                host.processes,
            )
            .map_err(|failure| {
                Error(format!(
                    "原生批次在 {:?} 中断：{}；已完成：{}；失败：{}；备份：{}",
                    failure.stage,
                    failure.error,
                    failure.progress.fix.fixed_ids.join("、"),
                    failure.progress.fix.failed_ids.join("、"),
                    batch.backup_root().display(),
                ))
            })?;
            *self.backup.borrow_mut() = Some(batch.backup_root().display().to_string());
            if let Some(error) = execution.receipt_failure {
                return Err(Error(format!(
                    "批次最终效果未确认：{error}；已完成：{}；失败：{}；执行详情：{}；备份：{}",
                    execution.outcome.fix.fixed_ids.join("、"),
                    execution.outcome.fix.failed_ids.join("、"),
                    execution.outcome.fix.details.join("；"),
                    batch.backup_root().display(),
                )));
            }
            let receipt = execution
                .receipt
                .ok_or_else(|| Error("批次未返回最终效果凭据".into()))?;
            receipt.validate_current(&ctx, batch).map_err(Error)?;
            *self.batch_receipt.borrow_mut() = Some(receipt);
            execution.outcome.fix
        } else if let Some(ticket) = &self.reviewed.primary {
            let result =
                repair::apply_under_manager(&ctx, ticket.clone(), self.guard).map_err(Error)?;
            *self.backup.borrow_mut() = result.backup_path;
            core::FixResult {
                fixed_ids: result.fixed_ids,
                failed_ids: result.failed_ids,
                details: result.details,
            }
        } else {
            core::FixResult::default()
        };
        *self.primary_result.borrow_mut() = Some(result.clone());
        Ok(result)
    }
    fn reconcile<R: Runner>(
        &self,
        host: &Host<'_, R>,
        target: TransitionTarget,
    ) -> HostResult<core::ReconcileResult<(), Snapshot>> {
        let ctx = host.context()?;
        self.guard.validate(&ctx).map_err(Error)?;
        if ctx.desktop_running || ctx.runtime.as_ref().is_some_and(|r| r.active_requests != 0) {
            return Err(Error("路由修复尚未获得真实写入静默窗口".into()));
        }
        if let Some(batch) = &self.reviewed.batch {
            if target != batch.target() {
                return Err(Error("批次后继目标不是原审核目标".into()));
            }
            self.batch_config(&ctx)?;
            let actual = host.verify_target(host.paths, target, Depth::Full);
            if !actual.issues.is_empty() {
                // Take the receipt before entering a mutating primitive. Any error,
                // including partial publication, leaves commit/restore/fence without
                // authority to replay or bless its unconfirmed current bytes.
                let receipt = self
                    .batch_receipt
                    .borrow_mut()
                    .take()
                    .ok_or_else(|| Error("缺少批次累计凭据".into()))?;
                let (_, next) =
                    super::native_batch::reconcile_after_batch(&ctx, batch, &receipt, self.guard)
                        .map_err(Error)?;
                *self.batch_receipt.borrow_mut() = Some(next);
                let actual = host.verify_target(host.paths, target, Depth::Full);
                if !actual.issues.is_empty() {
                    self.batch_receipt.borrow_mut().take();
                    return Err(Error(format!(
                        "批次后继实际Full验证失败：{}；已完成效果和备份保留",
                        actual
                            .issues
                            .iter()
                            .map(|i| i.message.as_str())
                            .collect::<Vec<_>>()
                            .join("；")
                    )));
                }
            }
            *self.reconciled.borrow_mut() = true;
            return Ok(core::ReconcileResult {
                value: (),
                details: vec!["已按原生批次顺序修复，并回读确认实际路由目标".into()],
                warnings: vec![],
                state_after: host.snapshot(),
            });
        }
        if *self.reconciled.borrow() {
            let actual = host.verify_target(host.paths, target, Depth::Full);
            if actual.issues.is_empty()
                && optional(&ctx.config()).map_err(Error)? == self.reviewed.expected_final_config
            {
                return Ok(core::ReconcileResult {
                    value: (),
                    details: vec!["已回读确认前次收敛效果，无需重写旧预览".into()],
                    warnings: vec![],
                    state_after: host.snapshot(),
                });
            }
            // A respawn may have restored an earlier approved prefix. The resume
            // primitive below proves the whole ordered effect set before retrying.
        }
        let initial = self.reviewed.reconcile.as_ref().ok_or_else(|| {
            let actual = host.verify_target(host.paths, target, Depth::Full);
            Error(format!(
                "该修复没有获批的路由重写计划，请重新检测并预览；实际未满足项：{}",
                actual
                    .issues
                    .iter()
                    .map(|i| i.message.clone())
                    .collect::<Vec<_>>()
                    .join("；")
            ))
        })?;
        let primary_effect = self
            .reviewed
            .primary
            .as_ref()
            .and_then(repair::Ticket::config_after);
        let after_primary = primary_effect.as_ref().map(|v| v.as_deref());
        let rebase;
        let resumed;
        let previous = self.active_reconcile.borrow().clone();
        let plan = if let Some(previous) = &previous {
            resumed = native_router::resume_after_approved_effects(
                &ctx,
                previous,
                &ctx.app_root
                    .join("maintenance/diagnostic-backups")
                    .join(&self.reviewed.view.ticket)
                    .join("router-resume"),
            )
            .map_err(Error)?;
            &resumed
        } else if let Some(token) = &self.reviewed.native_primary {
            rebase = native_router::rebase_native_primary(
                &ctx,
                initial,
                token,
                &ctx.app_root
                    .join("maintenance/diagnostic-backups")
                    .join(&self.reviewed.view.ticket)
                    .join("router"),
            )
            .map_err(Error)?;
            &rebase
        } else if let Some(after) = after_primary {
            rebase = native_router::rebase_after_approved_config(
                &ctx,
                initial,
                after,
                &ctx.app_root
                    .join("maintenance/diagnostic-backups")
                    .join(&self.reviewed.view.ticket)
                    .join("router"),
            )
            .map_err(Error)?;
            &rebase
        } else {
            initial
        };
        if plan.config_after() != self.reviewed.expected_final_config.as_deref() {
            return Err(Error("后继路由效果不等于已审核差异，未执行重写".into()));
        }
        *self.active_reconcile.borrow_mut() = Some(plan.clone());
        let outcome = native_router::apply_under_manager(&ctx, plan, self.guard).map_err(Error)?;
        if let Some(error) = outcome.failure {
            return Err(Error(format!("路由修复部分完成：{error}；备份保留")));
        }
        if !outcome.verification.issues.is_empty() {
            return Err(Error(
                outcome
                    .verification
                    .issues
                    .iter()
                    .map(|i| i.message.clone())
                    .collect::<Vec<_>>()
                    .join("；"),
            ));
        }
        *self.reconciled.borrow_mut() = true;
        Ok(core::ReconcileResult {
            value: (),
            details: outcome
                .completed
                .into_iter()
                .map(|s| format!("路由阶段已完成：{s}"))
                .collect(),
            warnings: outcome.warnings,
            state_after: host.snapshot(),
        })
    }
    fn run_postcondition<R: Runner + Send + Sync + 'static>(
        &self,
        host: &Host<'_, R>,
    ) -> HostResult<core::FixResult> {
        let source = host.snapshot();
        let mut repair = |_| {
            let reconciled = self
                .reconcile(host, source.target())
                .map_err(core::ManagerError::Primitive)?;
            Ok(core::CompletedRepair::from_parts(
                core::FixResult {
                    fixed_ids: vec!["router_postcondition".into()],
                    failed_ids: vec![],
                    details: reconciled.details,
                },
                Some(()),
                reconciled.warnings,
            ))
        };
        let request = core::QuietRepairRequest {
            paths: host.paths,
            source: &source,
            original: &source,
            selected: "router_postcondition",
            prepared_catalog: None::<&super::native_orphans::OfficialCatalog>,
            include_missing_rollouts: false,
            reason: core::REPAIR_REASON,
            policy: core::RepairTransitionPolicy::Strict,
            writer_policy: process::ProcessPolicy::ForceCloseAll,
            graceful_timeout: std::time::Duration::from_secs(5),
        };
        match self.run_quiet_transition(
            request,
            core::Processes {
                runner: host.runner.clone(),
                context: host.processes,
            },
            host,
            &mut repair,
        ) {
            core::TransitionOutcome::Verified {
                value, warnings, ..
            } => {
                let (mut fix, _, _) = value.into_parts();
                fix.details.extend(warnings);
                Ok(fix)
            }
            core::TransitionOutcome::Unverified {
                recovery_reasons, ..
            } => Err(Error(recovery_reasons.join("；"))),
            core::TransitionOutcome::Failed(failure) => {
                let needs_compensation = failure.needs_compensation();
                let primary = failure.error.to_string();
                if needs_compensation {
                    match self.restore(&source, &primary, host) {
                        Ok(warnings) if warnings.is_empty() => Err(Error(primary)),
                        Ok(warnings) => Err(Error(format!(
                            "{primary}；源目标恢复仍需处理：{}",
                            warnings.join("；")
                        ))),
                        Err(restoration) => {
                            Err(Error(format!("{primary}；源目标恢复失败：{restoration}")))
                        }
                    }
                } else {
                    Err(Error(primary))
                }
            }
        }
    }
}
fn report_context(reviewed: &Reviewed) -> HostResult<Context> {
    Ok(reviewed.context.clone())
}

/// The only production system-runner entry. Tests call apply_with_runner using
/// synthetic process tables and explicitly supplied temporary storage roots.
pub(super) fn apply_native(
    app: &tauri::AppHandle,
    ctx: &Context,
    reviewed: Reviewed,
) -> Result<DiagnosticFixResult> {
    let runner = Arc::new(process::SystemRunner::default());
    let process_context = ProcessContext {
        user_home: std::env::var_os("HOME").map(PathBuf::from),
        codex_home: ctx.home.clone(),
        sqlite_home_env: ctx.sqlite_home_env.clone(),
        cwd: ctx.working_directory.clone(),
    };
    apply_with_runner(ctx, reviewed, runner, &process_context, &|| {
        (
            crate::gateway::diagnostic_runtime(app),
            crate::gateway::diagnostic_proxy_handle(app),
        )
    })
}
fn apply_with_runner<R: Runner + Send + Sync + 'static>(
    ctx: &Context,
    mut reviewed: Reviewed,
    runner: Arc<R>,
    process_context: &ProcessContext,
    runtime: &dyn Fn() -> (
        Option<crate::gateway::types::RuntimeView>,
        Option<verification::ProxyHandle>,
    ),
) -> Result<DiagnosticFixResult> {
    if ctx.home != reviewed.home
        || ctx.app_root != reviewed.app_root
        || process_context.codex_home != ctx.home
    {
        return Err("修复目标目录已变化".into());
    }
    if now_ms().saturating_sub(reviewed.created_at) > 300_000 {
        return Err("修复预览已过期，请重新预览".into());
    }
    let guard = repair::acquire(ctx)?;
    guard.validate(ctx)?;
    if optional(&ctx.config())? != reviewed.initial_config {
        return Err("配置已被外部更新，请重新预览；未修改文件".into());
    }
    let current = snapshot(ctx)?;
    if current.target() != reviewed.original.target() {
        return Err("路由目标在预览后变化，请重新预览".into());
    }
    if current
        .router
        .as_ref()
        .map(native_router::Snapshot::source_dependency)
        != reviewed
            .original
            .router
            .as_ref()
            .map(native_router::Snapshot::source_dependency)
    {
        return Err("路由来源或存在状态在预览后变化，请重新预览".into());
    }
    if let Some(router) = &reviewed.original.router {
        let (path, expected) = router.source_dependency();
        if stamp(optional(&path)?.as_deref()) != expected {
            return Err("真实路由来源已变化，请重新预览".into());
        }
    }
    if let Some(intent) = &reviewed.original.intent {
        router_intent::validate(ctx, intent)?;
    }
    if let Some(reconcile) = &reviewed.reconcile {
        native_router::validate(ctx, reconcile)?;
    }
    if let Some(batch) = &reviewed.batch {
        batch.validate_initial(ctx)?;
    } else if reviewed.original.target() == TransitionTarget::NativeOff
        && reviewed.reconcile.is_some()
        && inspect::config(ctx).is_ok()
    {
        super::native_threads::review_provider_target(
            ctx,
            "openai",
            reviewed
                .primary
                .as_ref()
                .is_some_and(repair::Ticket::changes_provider_target),
        )?;
    }
    reviewed.context = ctx.clone();
    let paths = paths(ctx, &reviewed.original);
    let host = Host {
        context: ctx,
        current: RefCell::new(reviewed.original.clone()),
        paths: &paths,
        processes: process_context,
        runner: runner.clone(),
        read_runtime: runtime,
    };
    let operations = Operations {
        reviewed: &reviewed,
        guard: &guard,
        primary_result: RefCell::new(None),
        batch_receipt: RefCell::new(None),
        backup: RefCell::new(None),
        reconciled: RefCell::new(false),
        active_reconcile: RefCell::new(None),
    };
    let execution = core::HostManager::new(&host, &operations, runner, process_context, &paths)
        .repair(&reviewed.selected);
    let fixed = execution
        .result
        .map_err(|e| format!("{e}；修复备份与恢复记录保留"))?;
    let ctx = host.context().map_err(err)?;
    let backup_path = operations.backup.borrow().clone();
    Ok(DiagnosticFixResult {
        fixed_ids: fixed.fixed_ids,
        failed_ids: fixed.failed_ids,
        details: fixed.details,
        backup_path,
        report: report(&ctx)?,
    })
}

#[cfg(test)]
#[path = "native_manager_tests.rs"]
mod tests;
impl<R: Runner + Send + Sync + 'static> HostRepairOperations<Host<'_, R>> for Operations<'_> {
    type PreparedCatalog = super::native_orphans::OfficialCatalog;
    type Reconciled = ();
    fn resolve_runtime_paths(
        &self,
        base: &core::RepairPaths,
        _: &Snapshot,
    ) -> HostResult<core::RepairPaths> {
        Ok(base.clone())
    }
    fn diagnose(
        &self,
        _: &core::RepairPaths,
        _: &Snapshot,
        _: core::DiagnosticMode,
    ) -> HostResult<core::DiagnosticReport> {
        // A refreshed context is supplied by Host before dispatch; no scan starts a service.
        report_context(self.reviewed)
            .and_then(|ctx| report(&ctx).map(manager_report).map_err(Error))
    }
    fn prepare_catalog(
        &self,
        _: &core::RepairPaths,
        _state: &Snapshot,
    ) -> HostResult<Self::PreparedCatalog> {
        // Preview already fetched this genuine official model list and pinned its
        // config/source inputs. Do not spawn a second app-server or reinterpret
        // Gateway's raw inventory as the native official adoption catalog.
        self.reviewed
            .batch
            .as_ref()
            .and_then(|p| p.official_catalog())
            .or_else(|| {
                self.reviewed
                    .primary
                    .as_ref()
                    .and_then(repair::Ticket::orphan_catalog)
            })
            .cloned()
            .ok_or_else(|| Error("孤儿 Provider 修复缺少已审核的官方模型目录".into()))
    }
    fn ensure_router_on_ready(&self, state: &Snapshot) -> HostResult<()> {
        let ctx = report_context(self.reviewed)?;
        let p = paths(&ctx, state);
        let report = verify(
            &ctx,
            state,
            &p,
            TransitionTarget::RouterOn,
            Depth::SkipProviderConvergence,
        );
        let errors = report
            .issues
            .iter()
            .filter(|i| matches!(i.dim, InvariantDim::Auth | InvariantDim::Proxy))
            .map(|i| i.message.clone())
            .collect::<Vec<_>>();
        if errors.is_empty() {
            Ok(())
        } else {
            Err(Error(errors.join("；")))
        }
    }
    fn fix(
        &self,
        _: &core::RepairPaths,
        _: &Snapshot,
        _: &str,
        prepared: Option<&Self::PreparedCatalog>,
        host: &Host<'_, R>,
    ) -> HostResult<core::FixResult> {
        let _ = prepared; // Ticket validation pins the concrete catalog's source inputs.
        self.apply_primary(host)
    }
    fn fix_postcondition(
        &self,
        _: &core::RepairPaths,
        host: &Host<'_, R>,
    ) -> HostResult<core::FixResult> {
        self.run_postcondition(host)
    }
    fn config_stale_reason(
        &self,
        _: &core::RepairPaths,
        state: &Snapshot,
    ) -> HostResult<Option<String>> {
        let ctx = report_context(self.reviewed)?;
        if let Some(router) = &state.router {
            native_router::stale_reason_for_target(&ctx, router, state.target()).map_err(Error)
        } else {
            let config = inspect::config(&ctx).map_err(Error)?;
            Ok(router_intent::owns_provider(&inspect::provider(&config))
                .then(|| "当前配置仍由 Atelier 接管".into()))
        }
    }
    fn reconcile_target<T: Runner + Send + Sync + 'static>(
        &self,
        _: &core::RepairPaths,
        source: &Snapshot,
        _: core::RepairAttempt,
        _: core::Processes<'_, T>,
        host: &Host<'_, R>,
    ) -> HostResult<core::ReconcileResult<(), Snapshot>> {
        self.reconcile(host, source.target())
    }
    fn run_quiet_transition<T: Runner + Send + Sync + 'static>(
        &self,
        request: core::QuietRepairRequest<'_, Self::PreparedCatalog, Snapshot>,
        processes: core::Processes<'_, T>,
        host: &Host<'_, R>,
        repair: &mut dyn FnMut(
            core::RepairAttempt,
        ) -> std::result::Result<
            core::CompletedRepair<()>,
            core::ManagerError<Error>,
        >,
    ) -> core::TransitionOutcome<(), core::ManagerError<Error>> {
        let mut commit = |_: &core::CompletedRepair<()>, operation: &RouterOperationId| {
            let ctx = host.context().map_err(core::ManagerError::Primitive)?;
            let actual =
                optional(&ctx.config()).map_err(|e| core::ManagerError::Primitive(Error(e)))?;
            let expected = if let Some(config) = self
                .batch_config(&ctx)
                .map_err(core::ManagerError::Primitive)?
            {
                config
            } else if *self.reconciled.borrow() {
                self.reviewed.expected_final_config.clone()
            } else {
                self.reviewed
                    .primary
                    .as_ref()
                    .and_then(repair::Ticket::config_after)
                    .unwrap_or_else(|| self.reviewed.initial_config.clone())
            };
            if actual != expected {
                return Err(core::ManagerError::Operation(
                    "最终配置不等于已审核执行效果，保留路由恢复记录".into(),
                ));
            }
            host.publish(operation, expected.as_deref())
                .map_err(core::ManagerError::Primitive)
        };
        router_transition::run_host_repair(request, processes, host, repair, &mut commit)
    }
    fn restore_router_on(
        &self,
        _: &core::RepairPaths,
        source: &Snapshot,
        reason: &str,
        host: &Host<'_, R>,
    ) -> HostResult<()> {
        self.restore(source, reason, host).map(|_| ())
    }
    fn rollback_native_off(
        &self,
        _: &core::RepairPaths,
        source: &Snapshot,
        reason: &str,
        host: &Host<'_, R>,
    ) -> HostResult<Option<Vec<String>>> {
        self.restore(source, reason, host)
            .map(|warnings| (!warnings.is_empty()).then_some(warnings))
    }
}
fn stamp(bytes: Option<&[u8]>) -> Option<(usize, String)> {
    use sha2::{Digest, Sha256};
    bytes.map(|b| (b.len(), format!("{:x}", Sha256::digest(b))))
}
impl<R: Runner> HostState for Host<'_, R> {
    type Error = Error;
    type Snapshot = Snapshot;
    fn snapshot(&self) -> Snapshot {
        self.current.borrow().clone()
    }
    fn initialization_error(&self) -> Option<&str> {
        None
    }
    fn check_write_capacity(
        &self,
        paths: &core::RepairPaths,
        include_missing_rollouts: bool,
    ) -> std::result::Result<(), core::ManagerError<Error>> {
        super::snapshot::with_verification_paths(&paths.verification, |projected| {
            atelier_maintenance_core::router_capacity::ensure_transition_rollout_write_capacity(
                projected,
                include_missing_rollouts,
            )
            .map_err(err)
        })
        .map_err(|e| core::ManagerError::Primitive(Error(e)))
    }
    fn checked_recovery(
        &self,
        paths: &core::RepairPaths,
    ) -> std::result::Result<(), core::ManagerError<Error>> {
        core::ensure_recovery_records_allowed(&self.context.home.join("codexmate/relay"))
            .map_err(core::ManagerError::Io)?;
        let journal = JournalStore::new(&paths.verification.relay_root)
            .read_checked()
            .map_err(core::ManagerError::Io)?;
        if let Some(journal) = journal.filter(|j| j.is_commit_pending()) {
            let snapshot = self.current.borrow();
            if let Some(intent) = &snapshot.intent {
                if intent.observation().record.operation_id.is_none() {
                    return Err(core::ManagerError::Operation(
                        "路由提交未获回执，保留恢复记录".into(),
                    ));
                }
                router_intent::verify_receipt(self.context, &intent.observation().record)
                    .map_err(|e| core::ManagerError::Primitive(Error(e)))?;
                if journal.target != snapshot.target() {
                    return Err(core::ManagerError::Operation(
                        "待提交记录与持久路由目标不一致，保留恢复记录".into(),
                    ));
                }
                let verified = self.verify_target(paths, journal.target, Depth::Full);
                if !verified.issues.is_empty() {
                    return Err(core::ManagerError::Operation(format!(
                        "路由回执存在，但目标复检未通过：{}",
                        verified
                            .issues
                            .iter()
                            .map(|i| i.message.clone())
                            .collect::<Vec<_>>()
                            .join("；")
                    )));
                }
                // This is an explicit verified-commit recovery, not an ordinary clear.
                // An unreceipted or unverified record never reaches this branch.
                JournalStore::new(&paths.verification.relay_root)
                    .clear(Some(journal.target))
                    .map_err(core::ManagerError::Io)?;
            } else {
                core::verify_router_commit(
                    paths,
                    snapshot
                        .router
                        .as_ref()
                        .and_then(native_router::Snapshot::native_document)
                        .ok_or_else(|| core::ManagerError::Operation("原生回执来源缺失".into()))?,
                )
                .map_err(core::ManagerError::Io)?;
            }
        }
        Ok(())
    }
    fn preflight(
        &self,
        paths: &core::RepairPaths,
        source: &Snapshot,
    ) -> std::result::Result<(), core::ManagerError<Error>> {
        if let Some(intent) = &source.intent {
            router_intent::preflight(self.context, intent)
                .map_err(|e| core::ManagerError::Primitive(Error(e)))
        } else {
            core::preflight_state_write(
                paths,
                source
                    .router
                    .as_ref()
                    .and_then(native_router::Snapshot::native_document)
                    .expect("native source validated"),
            )
            .map_err(core::ManagerError::Io)
        }
    }
    fn read_journal(
        &self,
        paths: &core::RepairPaths,
    ) -> std::result::Result<Option<TransitionJournal>, core::ManagerError<Error>> {
        JournalStore::new(&paths.verification.relay_root)
            .read_checked()
            .map_err(core::ManagerError::Io)
    }
    fn verify_target(
        &self,
        _paths: &core::RepairPaths,
        target: TransitionTarget,
        depth: Depth,
    ) -> VerificationReport {
        match self.context() {
            Ok(ctx) => {
                let current = self.current.borrow();
                // Syntax rescue/profile cleanup may expose a different sqlite_home.
                // A final verdict must inspect the actual repaired runtime paths.
                verify(&ctx, &current, &paths(&ctx, &current), target, depth)
            }
            Err(e) => failed_verification(e.to_string()),
        }
    }
}
