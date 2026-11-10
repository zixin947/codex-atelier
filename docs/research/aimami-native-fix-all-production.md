# 原生 fix_all 应用接线与审批证据

日期：2026-11-08。目标：已安装 AiMaMi 1.2.7 arm64 的 `fix_all @1003a69d8`。本文只描述该批次迁移，不等于35项维护功能全部验收完成。未读取真实配置/认证/数据库，未执行真实客户端/GUI/Appserver。

## 原生控制流

原始证据：分析根 `relay-recovered/maintenance-full/codex-direct/decompiled/fix_all.c`；core留存副本 `src-tauri/crates/maintenance-core/evidence/batch_repair/fix_all.c`。`ordinary-order.json` 与 `fix-all-call-edges.asm` 记录实际 rodata/BL，而非按UI顺序推断。

| 阶段 | 原始证据 | 应用实现 |
| --- | --- | --- |
| 一次初诊，选择 fixable 且非 ok | C132及后续HashSet插入 | `native_batch.rs` 的 `selected` / `Host::diagnose`；新增问题在首写前拒绝，复诊不扩审批范围 |
| 实际静默窗口 | C quiet gate；core policy真实项目集合 | `Host::ensure_quiet_window` 实际 `MutationGuard.validate` + core `preview_stop`，检查进程、外部CLI、受保护DB持有者；不quit/kill/launch |
| Integrity→Unlock→Takeover | C260/412/568；BL1003a6d54/720c/758c | `native_coordinator_batch.rs::batch_auth`，实际core auth计划、固定备份来源、精确fresh effect子集 |
| Syntax→ThirdParty→Profile→Stale | C723/1051/1211/1344；BL1003a7908/7c24/7f50/826c | `batch_prepare_config`，使用原生文本函数；每一步只有预审候选的成功/保持原候选两分支 |
| 必要时一次重同步、成功才复诊 | C816/880；deferred移除C875及后段retain | `batch_repair::run` + `native_router` bound token；不加循环，不在每个dispatch运行剩余全部项目 |
| orphan联合一次收养 | C1985；BL1003a92ac | `native_orphans::apply_verified`，保留健康foreign，不把任意foreign归孤儿；官方模型池opaque原源校验仍有效 |
| 固定11普通项目 | rodata VA10181b9d0，C2204–2392 | core `ORDINARY_ORDER`，每项调用具体原始计划；普通错误继续聚合failedIds |

普通11项顺序为：missing rollouts、provider consistency、model catalog consistency、poisoned instructions、response item IDs、legacy migrated、residual manifest、residual cache、global config validity、project integrity、state DB relocation。`router_postcondition` 属外层Manager，不塞入该表。

## 审批与实际执行

`native_batch::ReviewedBatch` 缓存原始认证、线程、孤儿、普通尾项计划，以及配置有限候选和重同步token。`validate_initial` 对整个原始源全集校验，才允许首个备份/业务写入；不会运行时重新preview当前任意内容当作批准。

`PreparedBatch` 由原始配置与Takeover恢复候选起步，计算四个有序文本准备所有成功/失败子集的有限图。预审失败的候选保留确定错误；语法抢救原版确实可能仍产生无效TOML，不能把“写入成功”冒充语法修复成功。core resync失败会删除deferred fixed IDs并记failed IDs，保留其它已完成项目。

认证先前成功的 `auth.json=A` 可以使原Unlock的 `auth.json=B` 效果自然缩减为仅清marker/backup；新的来源文件、恢复目标、不同after均不接受。普通认证失败若只留下精确before/已审核fresh after，允许带此已知partial继续；未知部分效果阻止后继写入。

配置准备产生 `ApprovedBatchPrefix`；重同步产生 `ApprovedResyncEffects`（累计原始→最终，包含未变字节及mtime）；孤儿、逐项线程、普通残留、Global、Project依次提供私有构造凭据。Session精确合并批准链，逐次验证完整依赖与实际SQL/文件输出，不能读取当前hash后无条件续签。`PreparedOrdinary` 额外保存原始目标全hash/mtime；大旧库按64KiB流式检查，不套用配置文本大小限制。

Project单项现使用真正 `project_state::Prepared/validate/apply_under_manager`，不再在repair.rs用兼容wrapper手拼JSON/cwd。批次中Project实际SQL/JSON效果先经 `ApprovedProjectEffects` 证明，再进入最后旧库清理。

## 无Source场景

不伪造Native RelayState或Gateway目录。真实输入是三个状态源（Native state、Gateway catalog、RouterIntent）加原始config witness；不存在隐藏第四个状态文件。缺失必须为实际NotFound，权限/I/O错误继续错误。

原版默认Off字段证据：`relay_state_serialize_2.c:51–54` 将 `+0x16c` 映射 `codexRouterEnabled`，`:63–66` 将 `+0x17e` 映射 `codexRouterNoAccountMode`；`relay_storage_load.c:160–208` metadata失败分支直接返回默认state，`:203` 的 `u16(+0x16c)=0` 和 `:204` 的 `u16(+0x17e)=0x100` 两字段首字节均false。不是误命名 `relay_storage_default_state.c` 的错误closure。

有效config仍用真实provider ownership/持久intent判断模式，不能把SourceNone的managed Atelier provider当Off。仅syntax且原TOML无法解析、三个状态源实际缺失时才采用已恢复的默认Off语义。`native_router::prepare_bound_observed_off/apply_bound_observed_off` 只预审/执行Off字段清理，不创建state/catalog/accounts。

## Manager最终凭据

`apply_under_manager` 返回 `BatchExecution { outcome, receipt, receipt_failure }`。普通失败不丢已有fixed/failed IDs；未知partial不能生成receipt。

`BatchReceipt` 绑定同一 `ReviewedBatch` 的私有Arc identity，验证最终全effects的hash/mtime及目录。`config_after` 只能从原始有限配置候选、预审重同步final和Global after中按已批准hash选取；执行时当前bytes只用于核验，不用于签署新内容。其它预览即使输出相同也不能复用receipt。该凭据应在Manager publication前验证；publication自身合法写出的intent/state应由其独立回执核验。

外层Manager持有锁、真实fence、snapshot、Journal、Full/Depth1验证、补偿与relaunch；批次不自创Manager stage/attempt，不做自身transition补偿。当前生产Manager all分支已经改用该批次而非旧repair::apply(all)顺序，根worker正做最终入口回归。

## 实际验证及仍有边界

已执行：app `cargo test --offline --manifest-path src-tauri/Cargo.toml production_native_batch --bin codex-atelier` 最终10/10通过（执行63194，日志 `/tmp/atelier-production-native-batch.log`）；coordinator `reviewed_batch` 7项通过；原有Project筛选19项通过；自有文件 `git diff --check` 通过。最后一项实际验证内部resync→Global后最终receipt、不同preview及外部更改拒绝。编译存在全工程既有warnings，未声称零warning。

专用用例均使用显式temp目录、FakeRunner/FakeRpc，包括：SourceNone认证A优先于Unlock B；坏托管块Syntax→一次Off resync；不可抢救语法明确failed；新认证候选首写前CAS拒绝；实际进程分类出现外部CLI拒绝静默；普通Unlock失败而Residual仍执行；plain/zstd孤儿→响应ID线程阶段且健康foreign原样；Project真实SQL后旧库删除；Global位于内部resync之后的最终receipt及不同preview/外部改动拒绝。

仍需根worker完成并验证：生产Manager all全入口/整套回归；外层Full验收失败时如确实需要再次reconcile，必须使用已预审有限后继路由计划，目前不能以当前任意配置新签批准来绕过。线程跨重启恢复schema/官方catalog freshness等原审计剩余项目不由本批次测试数覆盖。
