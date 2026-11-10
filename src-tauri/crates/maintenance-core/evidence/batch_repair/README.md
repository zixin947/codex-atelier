# `fix_all` 原生阶段驱动

2026-11-08，AiMaMi 1.2.7 arm64。原始副本 SHA256 `b2a4b1b2b4babd2518e02c0f0d6203ad19e779909a47b3b643844ef403fe95de`。`fix_all.c` 是 root batch49 对 `0x1003a69d8` 的直接 Ghidra 导出，只清理行尾空白；没有用旧 DSH Markdown 推导顺序。

实现 `src/batch_repair.rs`；公开生产模块测试 `tests/batch_repair_native.rs`。本次不修改应用 `native_manager/repair/inspect`，由 root 用实际审核票据、源/效果 ledger 接线。core 模块存在和测试通过不能当作 App 调用链已迁移。

## 一次批次的真实顺序

初诊 `run_diagnostics_with_mode(...,0)`（C:132）形成 **fixable && status != ok** 的 ID 集（C:150–158）。不是按 UI、输入数组或 severity 顺序执行。即使 status 是 skipped/unknown，只要原生 fixable=true，仍在该集合；没有可自动修复项时返回 `所有项目正常，无需修复` 的原生文字（C:163–208）。该文字不能证明所有非 fixable 检查都已验证健康，App 仍需保留实际报告状态。

有需 quiet 的项时，原版先验证 `param_6` 的“已经进入静默窗口”上下文，缺失则整体失败（C:213、:919–974，call edge `0x1003a6d04`）。本移植不让调用者传 valid bool：`BatchHost::ensure_quiet_window` 必须核实际 MutationGuard 和真实 Runner 当前 writer 观察。外层实际 fence 仍由 Manager 持有；批次本身不 stop/kill/launch。

| 顺序 | 动作 | 直接证据 |
| --- | --- | --- |
| 1 | 若初始集合含 `auth_integrity`，执行完整修复 | C:260；BL `0x1003a6d54 → 0x10049f188 → fix_auth_integrity @0x100373af8`。 |
| 2 | `router_unlock_auth_residue` | C:412；BL `0x1003a720c → 0x10049fee4 → fix_router_unlock_auth_residue @0x100393114`。 |
| 3 | `takeover_backup_orphan` | C:568；BL `0x1003a758c → 0x1004a0224`。 |
| 4 | prepare_config：syntax → third_party → profile → stale，各只在初始集合含 ID 时调用 | C:723、:1051、:1211、:1344；四 wrapper 分别 BL text helper `0x10038c94c`、`0x10038a534`、`0x100397734`、`0x10037659c`。 |
| 5 | HTTP contract、catalog_integrity、catalog_path_validity 加入 deferred，必要时 resync_config 一次并复诊 | C:1471–1528、:1622–1798、:816–914。不是分别立即调用三个完整 fix 后再无条件 resync。 |
| 6 | DB/rollout 任一 orphan 在初始集合时，仅一次 union adoption | C:1845–1985；BL `0x1003a92ac → 0x10049fd40 → fix_orphan_thread_providers @0x10038e1d8`。错误只把初始集合实际含的 orphan IDs 列入失败。 |
| 7 | 按 rodata 固定11项依次执行初始集合命中的项 | C:2204–2392，table `0x10181b9d0`，见 `ordinary-order.json` 原始 pointer/length 解码。 |

固定11项严格为：

1. `thread_missing_rollouts`
2. `thread_provider_consistency`
3. `thread_model_catalog_consistency`
4. `thread_poisoned_instructions`
5. `thread_response_item_ids`
6. `legacy_migrated_threads`
7. `residual_manifest`
8. `residual_cache`
9. `config_global_validity`
10. `project_state_integrity`
11. `state_db_relocation`

原版没有“多轮直到无问题”的循环。只有一次初诊、在 resync 成功时的一次复诊；后者裁决 deferred 项，**不把新出现的项目加入当前批次**。`router_postcondition` 不在该11项内，由外层 Manager 原生特殊/协调分支处理，不应普通 dispatch 一次再 Manager 重做一次。

## resync 的条件和失败处理

- syntax/third_party/profile **准备成功**才触发；失败本身不触发。
- stale 准备成功时，只有原始 snapshot RouterOn 才触发（C:1385）；Off 可完成纯文本清理而不 resync。
- 初始 HTTP contract 或任一 catalog ID 无论 On/Off 都要求 resync。原生固定文案为 `将重新写入 aimai1 HTTP-only 路由契约` / `将随 config 一起重新生成 Catalog`（C:1507–1520、:1727–1742 的内联 UTF-8 字节）。Gateway 的显式 contract 适配不能把这句 native 文案当实际写了 aimai1 的证据。
- 成功 prepare/deferred 会暂加入 fixed_ids 和 deferred；resync 失败时将 deferred 各项加 failed_ids，**只从 fixed_ids 删除 deferred**（retain `0x10055d460`），保留无关认证/先前真实进展。继续 orphan 与11项，而不是整体抛异常、伪造全部回滚。
- resync 成功后遍历实际复诊结果，对 deferred 中匹配到 status != ok 的项改失败、移除 fixed（retain `0x10055dc8c`），并追加 `{id}: resync 后复查仍异常`（C:899–914、:1798–1830）。不看该项此时是否仍 fixable。
- 原生完整报告恒含固定检查。C 搜索的是“匹配且非 ok”；没有额外的“缺 ID 默认为异常”分支。本核心保留这个流程，但 Host 不能用缺项/假快照替代真正完整 diagnose。

## 官方目录准备时点与真实认证依赖

`fix_all` 自身没有 model/list 获取调用。`param_7` 是此前 Manager 准备好的 optional catalog（C:131 保存；:1982 传 orphan wrapper）。Manager 原生 `prepare_catalog` 位于进入修复 callback 之前；`PreparedRepairCatalog::catalog @0x100793058` 在真正 adoption 时重核 config/auth 的原 bytes。

因此“Integrity → Unlock → 使用原 catalog”并不保证 orphan 成功：如果认证恢复改变了先前准备目录的认证来源，原生会明确报源失效。不能为了 all 全绿忽略 pin、把新 auth 盲当已批准来源，或用 Gateway inventory 替官方池。专用测试调用实际 core auth 恢复 A、fresh unlock 保 A，再让真实源码依赖比较拒绝旧 `auth=None` 的 pool；保留两项认证成功和 orphan 失败。

## 可接入 API 与错误边界

- `BatchHost` 全部方法必须实现，无默认成功：实际 `diagnose`、snapshot `router_target`、`ensure_quiet_window`、`prepare_config(ConfigPreparation)`、`resync_config`、`dispatch(BatchItem,Option<&OfficialCatalog>)`。
- `run` 自己决定集合/顺序/条件复诊和结果聚合，不接 should_fix/known_status/valid bool。
- `dispatch` 的 `ItemFailure<E>{error,partial:FixResult}` 能保留执行器已经确证的部分结果；纯错误可用 `ItemFailure::new(error)`，不擅自声称源文件没变。原 native 正常返回的完整 FixResult 也原样合并，包括其中已有 failed_ids。
- 普通项目、prepare、resync 错误记录为 failed_ids/details/trace 后继续。`BatchOutcome` 是执行和部分结果，不能把某个 `StageRecord::Completed` 当所有检查健康；仍须看真实 `fix.failed_ids` 和外层目标态验证。
- 初诊/实际 quiet gate/必要复诊不能取得有效结果时，返回 `BatchFailure<E>{stage,error,progress}`，保留已完成真实进展与 typed 原错误。
- `Stage` 是批次内部位置，**不是**外层 `manager::FailureStage`。不得把内部 QuietWindow 等级重新解释为外层 preflight/stop，或据此伪造 mutated bool 来改变补偿。真正转换失败的 stage+attempt、补偿、receipt/sync、重开均仍由既有 Manager/router_transition 决定；普通 failed_ids 本身不等于发生了转换失败。

## 验证与未完成

执行 `cargo test --offline --manifest-path src-tauri/crates/maintenance-core/Cargo.toml --test batch_repair_native`：16项通过、零 warning。包括完整固定顺序、只一/二次实际报告、stale Off 例外、准备失败继续、resync 失败/deferred 非 ok、初始集合不扩充、orphan一次/失败子集、ordinary partial、quiet gate、outer-only postcondition，以及实际 auth A/B 文件恢复/清理/旧目录来源拒绝。

测试仅 TempDir、合成数据、真实 core 认证算法和可注入阶段 I/O；没有查询/停止/启动真实客户端，没有读取真实配置、认证、数据库、会话或钥匙串。新源码和 tests 的 no-index whitespace 检查无输出。

核心驱动已实现并注册；此证据提交时 App 实际阶段/审核效果 ledger 适配仍由 root 接入，原 `R orphan→threads→coordinated` 不能再作为完整 native fix_all 顺序的证明。全部35项检查、尚缺维护功能与真实 Desktop 验收仍按总清单继续，不由本模块测试数量关闭。
