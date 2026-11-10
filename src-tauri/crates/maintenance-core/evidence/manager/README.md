# 原生诊断修复 Manager 控制流

静态目标：AiMaMi 1.2.7 arm64，SHA256 `b2a4b1b2b4babd2518e02c0f0d6203ad19e779909a47b3b643844ef403fe95de`。没有启动/停止/重启任何真实 Desktop、CLI 或代理；验证使用 TempDir 与 FakeRunner。

本模块恢复 `RelayManager::fix_diagnostic_issue_audited`，不是旧 DSH `manager.rs` 的改名。原版其他 RouterOn/NativeOff 转换、生产诊断分派、实际原子提交仍是必须接入的操作。没有默认成功、`known_status`、`should_fix` 或 `Unsupported` 替代分支。

## 已核实并实现

| 分支 | 原生证据（本目录文件保留原行） | 实现 |
|---|---|---|
| 首个 gate 是初始化加载失败 | `manager_new.c:99-114` storage::load 失败格式化字符串；`native-main.c:358-373` 克隆 Option<String> 返回 CoreError tag9 | `from_loaded` 保留实际 load Result，失败缓存并在 selector/锁之前拒绝所有 repair |
| 不是 transitionInFlight bool | native-main 首 guard 是 Manager field2 的 Option<String>，不是日志活动状态 | 不以 report.transition_in_flight 决定拒绝；日志改走 checked recovery guards |
| 未知项在加锁前拒绝 | `native-main.c:375-381`，requires_quiet_window=2 | 使用恢复的 `policy::quiet_window` |
| 两把锁及 poison 边界 | `native-main.c:382-456`，transition Mutex 后 shared CodexMutationGate；`mutation_gate_lock.c` | Arc<Mutex<()>>，默认 shared OnceLock；RAII 逆序释放；poison 不清除；两种错误不同 |
| Snapshot state lock | `snapshot.asm:10068cdd0-10068ce94`；二进制 `1013a1ff6`=20 字节 `relay state poisoned` | StateHandle 实际 clone 受锁 StateDocument，poison 原文 panic；不 Debug/Serialize 凭据快照 |
| 解析实际运行路径前的恢复 guard | `manager_transition_paths.c:77-111`；`ensure_recovery_records_allowed.c:27-75` | checked journal→两份 model restore journal→两份 takeover backup→新 snapshot→pending receipt→实际 runtime resolver |
| 提交期仅校验真实磁盘收据 | `ensure_pending_commit_matches.c:29-35`；`verify_router_commit.c:109-166,1538-1548,1621-1669` | 只有 committing 调用 receipt 校验；读取实际 state.json 四字段；不把所有 pending 状态当拒绝；所有冲突统一原生错误 |
| report derive gates | `native-main.c:490-550,680-856`；native iterator predicates 已落 `policy.rs` | prepare、quiet、reconcile 等由实际 items.fixable/status/id 决定，非调用者 bool |
| prepare 单项/全部错误不同 | `native-main.c:562-679,1253-1280` | 单项错误早退；all 保存错误字符串，成功 FixResult.detail 追加“孤儿任务目录准备未完成：…” |
| 实际写入预检 | `storage_preflight_save.c:65-178,222-304`；`ensure-directories.asm` | 按固定顺序建目录，已有 state 写打开不截断，pretty JSON 写私有 atomic 临时文件、ack 后删除；从不覆盖原 state |
| 原生临时文件名 | 二进制 fmt `10151d65e`: `.relay-state-preflight-{}-{}` | pid + UUIDv4 simple；不是点号分隔 |
| RouterOn readiness 在 running-before 之前 | `native-main.c:799-855` | quiet 才 preflight；源 On 才 ensure_ready；错误直接退出，不重新打开客户端 |
| direct postcondition | `native-main.c:856-864` | 单次 audited fix，不走 quiet wrapper，也不在其后第二次 fix |
| quiet 修复实际嵌入转换 | `native-main.c:968-1012`；`native-transition-callback.c.txt:620-754` | fixed strict / writer ForceCloseAll /5s；all 与 missing-rollouts 才 include_missing；修复 callback 内运行，外层不再次 dispatch |
| callback 自身条件协调 | `native-transition-callback.c.txt:755-826,832-952,1098-1201` | dispatch→实际 Full verify；report 要求或 issues 非空或 stale reason 存在则 reconcile；否则再取 snapshot 查 stale；协调后去重追加 postcondition fixedId、合并 warning，用实际 reconciled state_after 查 stale |
| native failure 的两个字节 | `native-main.c:1013-1016`；run_repair 原 C failure+0x60/+0x61；stage 写入原函数 `100326b7c` | stage 0 preflight /1 stop /2 reconcile /3 settle /4 verify /5 commit，attempt u8。stage>=2 或 attempt>=2 才补偿；其他失败仍是失败，不是假成功 |
| 错误载荷保留 | `native-main.c:1017-1111`；TransitionFailure Display `1003106a4` 尾跳 CoreError Display `1007fcd0c` | 泛型 E 原值保留在 ManagerError::Primitive；补偿/重启失败以结构包装同时保留 primary/secondary |
| compensation 目标由源 snapshot 决定 | `manager_compensate_repair.c:30-95` | On→restore；Off→rollback；Off 的 Some(warnings) 即使空数组也为恢复失败；主错误从不因恢复成功变成成功 |
| verified / warnings / unverified | `transition_into_verified_fix_result.c:41-151`；`unverified-join.asm:10033b8d4-10033b920` | warning 追加详情；unverified 转 Operation 错误；issues 用 ` | ` 连接；原生中文 recovery 前缀原样 |
| ordinary error 不自动 reopen | `native-main.c:1280-1372` | dispatch 错误 eligibility=false；postcondition/quiet 的结果 eligibility=true |
| 失败后重新打开的确认不是 bool 注入 | `manager_has_confirmed_target_state.c`；`native-main.c:865-874` | checked journal 无 committing + 实际 `verify_target_state_at_with_depth(Full)` issues.empty；错误/不确定均保留主错误且不启动 |
| 重新打开的成功/失败不同 | `native-main.c:1399-1533` | 成功返回仍保留原失败；修复成功+launch失败只加警告；双失败保留双错误 |
| 原文展示瑕疵 | native main ASM `100683afc-100683be4` | 已经运行时无 open/audit step，但原生仍追加“已自动重新打开…”。保留原 detail，另以 RelaunchEffect::AlreadyRunning 精确记录实际行为 |

`RepairOperations` 是跨 app/core 的必要原子操作边界：真实诊断/单项或 all 分派、目录准备、runtime paths、RouterOn readiness、stale reason、router reconcile、转换引擎、稳定恢复。全部为必需方法。`CompletedRepair<T>` 封装实际 FixResult、reconcile metadata T 与独立 warnings；不接受 caller verified bool。`ReconciledRouterTarget::into_details` ASM 已区分 details 与 warnings 两个 Vec，不能混写或重复追加。Root adapter 仍须以实际操作实现这些方法，单元测试通过不代表整个迁移已交付。

## 精确收据语义

原 Receipt derive 的字段为 `schemaVersion: i32`、`routerCommitId: Option<RouterOperationId>`、`codexRouterEnabled: bool`、`codexRouterNoAccountMode: bool`。前3项逐一相等，noAccount 仅 RouterOn 时比较。未知字段忽略，已知重复/错类型/非法 UUID/JSON 尾随数据拒绝。文件不存在仅当 source Off 且 commitId None 允许。其余 IO/JSON/不一致一律返回：

`ROUTER_COMMIT_STATE_CONFLICT: 无法核对磁盘路由回执与当前模式，已停止继续写入；请检查状态文件并重启 AiMaMi`

## 生产接入边界

此模块的系统进程动作复用 `process::Runner` 的固定 argv 和归属筛选；本轮测试从未构造 `SystemRunner`。正常成功结果不会额外强制 Full confirmation（原生只有 failure 路径才要求），也不会以 failed_ids 非空擅自转总错误。由调用方在既有预览/确认后调用具体 Manager；当前 app 的旧 blanket journal guard 不能声称等价，root 接线后需替换为真实 recovery 流程。


## 真实持久化与提交

新增 `PersistenceContext<C>` 持实际路径/StateHandle/proxy Mutex/CatalogRuntime<C>；`persist_with_commit` 不接 route_changed/known_status bool。证据为 `persist_with_commit.c @10067d67c`、`commit_reconciled_target.c @100680790`、`compose_proxy_status.c @10067dd5c` 与 `state_save_with_publication.c @100207780`。

- 初始化 load error 先返回；candidate aliases 真实校验；compose proxy 在 state lock 前执行。Proxy lock poison/None→默认 RelayProxyStatus；Some handle→running=true（不看 alive、不连 TCP），baseUrl=`http://127.0.0.1:{port}`，codexBaseUrl=`http://127.0.0.1:{port}/codex/v1`，lastError=None。后者和路由配置 `/codex/router/v1` 契约是两个不同字段。
- state Mutex 持锁期间，先核 existing snapshot 对 pending receipt，再分 Some commit/普通 settings。没有 commit 不能改变 active route（Off 时 noAccount 变更可允许）；candidate receipt 已过期时，转换提交一律拒绝，普通 settings 只有所有 route mode 字段仍一致才保留 current receipt。此细节保留原分支，而非笼统拒绝所有旧 snapshot。
- `save_state_with_publication` 真校验 credentials/schema，建固定目录、pretty JSON、atomic_write_report；schema>16 或 schema<=14 却带 commit receipt 返回原错误；没有写入失败的假成功。
- 已发布且目录 sync 未确认时，额外核真实磁盘 receipt；原版失败只日志警告，仍更新内存并返回已发布 Publication。`StatePublication.receipt_warning` 显示该不确定性，不执行回滚。转换引擎仍须 require_sync 并保留 committing recovery record。
- `native_catalog_revision.c @100678ae0`：仅 build_relay_models(active providers/exposed models) 的 compact JSON→SHA256 小写 hex。修改未激活 provider 不影响 revision。active mode/noAccount 或此 revision 变更才两个 generation counter+1、丢 cached preparation；失败发布不更新内存或 cache。
- `commit_reconciled_target` 从当前真实 snapshot 和 ReconciledTarget 元数据派生 candidate；NativeOffWriteWindow::Unavailable 直接拒绝；实际 stale reason 检查后再保存对应 operationId，不接 caller verified bool。错误前缀由原二进制 fmt `10152874a`（reconciled auth mode）、`101528786`（router residue）核实。

## 两种实际来源共用一个驱动

新增 HostState 关联 Snapshot（RepairSnapshot::target 是实际意图投影，不是 validity verdict），并要求真实 checked_recovery、preflight、read_journal、verify_target→VerificationReport。HostRepairOperations 的所有原子修复/协调方法都必须实现，没有成功默认值。HostManager 是唯一控制流实现；原 NativeManager API 原样保留，通过 NativeHostState 借用真实 RelayDoc/Proxy/初始化错误委托。Gateway Host 可以持真实 GatewayCatalog 与 RouterIntent，不构造假的 RelayStateDocument，也不调用 Native state.json 发布器。各自实际发布由对应 quiet-transition Store 执行。

`manager_native` 中两个 Gateway-shaped fixture 只证明共用 driver 不强迫 Native 数据模型：Gateway catalog enabled 但无 ownership/intent 时仍 Off；已有真实 intent On 才运行 readiness 和配置契约修复；两者都未创建 Native state.json/accounts。实际 Atelier GatewayCatalog/RouterIntent 生产适配和端到端入口仍由 root 接入，不能以这两 fixture 代替生产接线验收。

## 恢复日志 schema

`load_fallback_model_restore_journal.c @1008ab7c4` 与 `fallback-entry-predicate.asm @100914dd8`：固定 `router_model_restore.json`，symlink_metadata regular file，metadata/read 双 8MiB 上限；缺失允许，其余错误用原有限中文类别。version u32 必须1；thread_id.trim 非空、raw id 唯一、original_model 仅 starts_with `aimami_relay_`、fallback_model.trim 非空。`attempts` u32 不参与有效性 gate。

`fallback_entry_deserialize_struct.c @100269290:270-279,383-388` 最终证实 attempts 缺省0（map与seq），前三个 string 必需，unknown/duplicate 拒绝。Journal 本身 version/entries 必需、denyUnknown。测试覆盖负数/溢出、缺失、unknown、duplicate、序列格式。

验证命令（最后一次已完成）：`cargo test --offline --manifest-path src-tauri/crates/maintenance-core/Cargo.toml --test manager_native`，65 passed，0 failed，零 warning。全部只触及显式 TempDir、FakeRunner；没有调用真实客户端操作。


No-account 日志尾项已由 `no_account_model_restore_entry_deserialize.c @10027b0e0:282-293,382` 闭合：`original_model` 为 **required nullable String**，不能用普通 Option 的 missing→None 默认替代；Journal 和 Entry 都 denyUnknown。Manager 使用自己的严格单读 schema，保留既有 threads reader 的正确版本/borrowed-model/raw-id 语义；未更改 threads 的计划/执行 API。`validate_no_account_model_restore_journal` 覆盖显式 null/string、缺失拒绝、未来字段拒绝、版本1只 canonical slot/版本2可合格 relay slug。当前 Manager guard 已精确拒绝；threads reader 的同类宽松 schema 已通知其负责人同步，不能把该独立模块的旧读取行为称为已对齐。

接线要求：NativeHostState 必须接实际 native storage load 校验后的 RelayDoc。当前 Document 的 raw projection 要额外通过 `catalog_inputs()`，才能验证 wireApi/brand 等 provider/model 字段；不能把仅 JSON 外壳合法视作原生强类型 state load 成功。否则 Snapshot 的原生强类型前提（native snapshot 只对 poison panic）会被不完整输入破坏。Gateway Host 以其真实强类型 Catalog/Intent 构建快照，无此 RelayDoc 约束。

HostManager quiet 接线：

1. 调用方把实际 shared outer transition Mutex 和 mutation gate 交 `.with_locks`；NativeManager 原实例已持有并传入。两锁不能在 Ops 内再入锁。
2. `checked_recovery(base)` 在报告和任何修复前执行（即使 residual 的 no-quiet 路径），且 Native 默认会在恢复记录检查后再 snapshot 核 pending receipt。run-host transition 的 stage0 再检查一次恢复记录是原生控制流，不应省略。
3. 只有报告派生 quiet 才调用来源自己的真实 preflight；On 才调用 readiness；随后才测 running-before。all 的目录准备失败保警告，single 直接失败。
4. quiet primitive 调用传入 callback **内部** dispatch/reconcile；禁止在外层又 apply 一次。callback 用 original snapshot dispatch；source compensation 使用紧邻 transition 的新 snapshot；最终错误重开确认目标仍是 original target。
5. callback 后真实 fence.finish 与 verify 由 transition driver 执行；commit 使用真实来源 Store 与 operationId；已发布但 sync 未确认不能当作“未写入”补偿。Native/Atelier Store 各自执行真实 receipt readback。
6. Host.read_journal 返回实际 typed journal（Error 不能转 None）；Host.verify_target 返回实际 issue collection；driver 只从 checked 非 committing + Full issues.empty 推导错误后可重开，不接 known-valid bool。
7. success launch failure 仅 warning；primary failure+launch failure 保两错误；普通 no-quiet dispatch failure不重新打开。这里只定义实现，测试仍禁止真实 Desktop 生命周期执行。
