# 原生 Router Transition 状态机

静态输入：只读 `maintenance-full/AiMaMi.arm64`，SHA256 `b2a4b1b2b4babd2518e02c0f0d6203ad19e779909a47b3b643844ef403fe95de`。主反编译为 `maintenance-full/codex-direct/decompiled/router_transition_run_repair.c`，实际符号 `run_with_environment @0x100326b7c`。实现 `src/router_transition.rs`；测试只使用临时目录、显式 synthetic ProcessContext 和 FakeRunner，绝不执行 SystemRunner。

## 已核分支

- C:383 调用 checked recovery 记录校验；383–405 在 Strict 调用实际 rollout 容量 preflight；BestEffort 跳过。原生策略 discriminant 0=Strict、1=NativeOffBestEffort，C:1889 的后者允许带未满足项提交；C:2618 回调错误在后者不重试。
- C:426–438 attempt 从 1 开始，最大 3；每次先 `stopping_codex` journal，再 acquire 实际 CodexTransitionFence。C:625 后 `reconciling` 才执行 repair/reconcile callback。callback 是 Manager 的实际诊断修复结果，不接 caller-valid bool。
- C:1253 无论 callback 成败都 finish fence。callback 成功且 finish 失败 => stage 3；callback 已失败且 finish 再失败 => 合并错误，保留 stage 2（C:2510–2593）。stop acquire 失败 stage 1；preflight 失败 stage 0 / attempt 0。
- C:1399–1403 仅 respawns=0 且 !tainted 时使用 Depth=1；否则 Full。所有深度仍实际检查配置、认证、目录、线程模型、proxy TCP，仅 Depth1 跳过 provider convergence。
- C:2359–2436 / 2674–2754，只有 fence respawn 或 taint 才可重试；第一次可重试结果建立 now+12sec 截止，后续复用，最多 3 次。安静情况下的 repair 错误或目标验证失败直接失败。Strict 目标验证失败 stage 4。
- C:1451–1470 / 1936–1958，目标验证完成后先 begin_commit（强制 journal directory sync），再实际 persist_with_commit / commit_reconciled_target。提交 helper 接真实 RouterOperationId v4，不能拿预生成成功状态替代。
- C:1471–1545，state 已 rename 但目录 sync 失败时保留 `committing`，返回 Unverified；其原生 recovery-reasons payload 与 warnings 分离。不得直接清除 journal 或当未发布进行补偿。
- C:1547–1622，已有 warnings 时 warn_after_commit 写 `degraded`；无 warnings 才 clear。warn_after_commit @0x100791a40 的短 ASM 证实先 read_checked、校验 target，再 allow_pending write；发布仅 acknowledge。clear 失败发生在状态已落盘以后，C:1660–1740 返回 Verified + cleanup warning，不触发 stage5 补偿。
- begin_commit / persist 失败 stage 5。原 write_journal @0x100325f24 的 ASM 证实普通阶段写入 acknowledge，失败只记录日志。commit_failed 的普通阶段写不能覆盖 pending committing；JournalStore 本身保留该约束。

## API 与边界

`run_native_repair` 直接使用 Manager 的 `persist_with_commit` / `commit_reconciled_target`，真实写入状态回执并更新 shared state；无 reconciled target 时保存实时 snapshot，有 target 时先实际 stale_reason 检查提案。`run_repair` 固定 Strict；`run_with_environment` 显式策略。两个入口都使用实际 core process fence、core verifier 和 JournalStore。required repair callback 只承接 app/Manager 的具体修复及 reconciliation；required commit callback 返回实际 Publication，包括 directory fsync 结果。无默认成功实现。

原生 verifier 契约仍为 aimai1 + `/codex/router/v1` + native model slugs。Atelier 的 Responses `/v1` gateway 契约必须由其明确 host 适配，不能伪造 native snapshot 或以 Gateway running 布尔值替代完整目标验证。

验证：`cargo test --manifest-path src-tauri/crates/maintenance-core/Cargo.toml --test router_transition_native`，13 项通过，零编译 warning。测试从实际公共 crate API 导入，包含真实状态回执发布、shared state 更新和 `verify_router_commit` 回读；故障分支以实际 temp 文件/合成进程快照/Publication 同步错误执行。

当前静态恢复边界：本模块直接接入已恢复的 Manager persistence 与 capacity，不宣称完整 app Host 或 gateway generation 已通过。错误消息的展示格式采用 Rust 当前核心错误与 typed issue，而非逐字复刻全部原版格式化文本。

## 显式 Host 扩展

`run_host_repair` 复用相同具体状态机，只把 checked recovery / actual target verification 交给 `manager::HostState` 的具体存储实现；snapshot 必须实现 RepairSnapshot。Native 原 API 仍调用原生 guard/文件验证，不修改默认 namespace。app 的合成 Gateway Host 测试走真实 Catalog/RouterIntent、真实 reviewed config/catalog 发布与 intent 回执，而不构造 RelayState 或 accounts 目录。
