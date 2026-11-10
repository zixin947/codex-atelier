# 维护工具生产覆盖审计

## 2026-11-08 WAL与性能修正（优先于下方历史检查点）

用户真实操作暴露的WAL最终校验/恢复错误已用合成WAL缺失历史批次复现；最终路由验证和容量预检曾直接打开源SQLite、改变边车状态。应用宿主已统一私有DB/WAL验证和容量观察，最终效果保护保留；冲突回执不再遮蔽独立源路由观察，未批准配置/外部DB变化仍不能提交或覆盖。预览普通诊断3次→1次，六项线程检查快照6份→1份，结果等价断言通过。维护457、core414、前端92项通过；账号25通过1忽略；原生check通过但78条警告。仅合成回归与debug单样本性能，不代表真实Desktop修复已验收或达到原版速度；本轮未替换运行应用。详细证据、原版调用行号与限制见 `docs/maintenance-verification.md` 最新章节。

## 2026-11-08 本轮当前状态（优先于下方历史审计）

Manager生产入口现已调用真实报告、预览和修复；本轮 `all` 也已改用 `native_batch` 的实际原生顺序和最终效果凭据，不再调用旧的 combined `repair::apply(all)`。路由模式已改读真实Native状态/Atelier意图；残留按各自归属判断；Project已接实际计划；Gateway官方目录已接真实请求观测。下方早期表格中“Manager未接”“依赖runtime代替模式”“freshness固定unknown”等描述是历史审计，不再代表这些入口的当前实现。

本轮已补批次终态后的 sealed 有限后继计划及累计凭据：Manager 在最终 Full 失败时只消费同次预审批准的后继，复检成功后提交；外部变化或部分失败不能以旧凭据续签。Global-only 的真实终态失败→后继→实际 intent receipt 链已有隔离测试。Native 批次不得借用 Gateway proxy handle。

已替换 history 近似检查，接入有界 JSON/UTF8/ordinal/history_base/plain+zstd 观察；认证 expiry/revision 分支、清理 SHA-256/对象身份保护和独立确认目录写探针也已接通。原版 config readiness 没有实际配置替换探针，早期表格该项已纠正。最终维护套件452项、core全套414项通过，前端84项通过，原生构建通过但有75条警告。

**仍不能宣称全量迁移完成**：NativeOff 模型筛选 closure 未恢复，已移除空白字符近似并明确未确认；Native代理/官方目录真实宿主仍缺；未授权legacy Keychain、源DB RW/sidecar能力、跨重启授权恢复和全部35项逐分支一致性仍未完成。没有操作真实Codex或替换运行应用。准确命令和范围见 `docs/maintenance-verification.md` 最新收口记录。


审计日期：2026-11-08（Asia/Shanghai）。这是并行开发期间的静态源码快照，不是发布验收结论。基准为 `docs/maintenance-parity-plan.md:5` 的全部 35 项检查、全部修复与维护工具目标，以及安装版 AiMaMi 1.2.7 的 ARM64/C 证据。

**当前不能宣称完成全量迁移。** 具体恢复代码已有大量接入，但报告仍包含旧近似检查、固定未确认项和缺失修复。Manager 的生产命令接线已在复核中落盘；整体仍在验收，不能把模块注册或单元测试通过计作全部功能完成。

## 同日复核更新（优先于后文首轮快照）

并行修复使首轮行号和结论发生变化。以下是再次读取生产源码后的事实，不表示全量验收完成；后文 35 项表保留首轮问题定位，涉及下列项目时以本节为准。

| 项目 | 复核后的实际事实 | 仍未关闭的边界 |
| --- | --- | --- |
| Manager 生产命令 | `diagnostics/mod.rs:172`、`:186`、`:219` 已分别调用 `native_manager::report/preview/apply_native`，票据改存 Reviewed | 主任务正在验收 typed Host、quiet、pending recovery、receipt、补偿和重开；不能继续写“仅模块存在”，也不能因此宣称全量完成。 |
| 第三方配置 | `inspect.rs:507` 及 `native_config.rs:74` 已接真实 core 检查；`native_coordinator.rs:222` 实际组合 `fix_config_third_party_text` | 应保留合法 foreign provider 与原生归属/模式分支，整体 Manager/全部组合继续核。 |
| Catalog 来源失败 | `inspect.rs:1124` 改 match；`:1153` 来源选择失败显式 unknown/禁修复 | 原首轮 `if let Ok` 静默回旧启发式缺口已关闭；不等于官方目录新鲜度已实现。 |
| 孤儿 Provider P1 | `repair.rs:257` 已删除旧遍历全部 t.rows 的迁移分支，改 `native_orphans::prepare/validate/apply`；`inspect.rs:1095` 已覆盖两项检查 | 真实官方模型/推理档位选择已沿新增 C helper 核对；实际受管失效集合、foreign 混合归属拒绝、临时 DB 写入与 CAS 已接。all 的临时同 DB 拒绝已移除，改由全初始依赖核验→真实孤儿效果→opaque prefix→线程受限 fresh plan；四个旧 all 回归保留行为断言并已通过。安装版跨重启 adoption recovery schema/消费者与大历史分页策略仍未完全恢复。 |

首轮下述“未接入口”“第三方无修复”“catalog 来源失败回旧检查”“旧 orphan 宽迁移可达”均已被本节的具体改动替代。固定 freshness unknown、探针缺失、project/history/native-off 近似、完整 all 各功能族组合与真实 Desktop 验收仍保留在剩余清单；不能继续把已关闭的孤儿→六项线程同 DB 组合列为当前阻塞。具体实现和验证见 [孤儿接管证据](aimami-native-orphan-adoption.md)。

## 范围和判定方法

- 只读仓库源码和已提取二进制证据；本次仅新增本文档。没有执行诊断/修复命令，没有读取真实配置、认证、数据库、会话或钥匙串，没有操作真实客户端。
- 查的是命令注册 → 生产调用者 → 实际检查 → 可预览修复 → 确认执行/复检。函数存在、模块注册、35 个 ID 齐全、测试数量均不代表该链路成立。
- “已接”只表示存在具体业务调用链；仍须完成安装版分支核对及隔离/用户验收。“部分”表示生产有实现但缺原生分支或仍使用近似；“未接”表示只有候选实现或完全缺失。
- `unknown` 和 `skipped` 是有效的不确定/适用性状态，不能一概视为伪造；下文只将固定占位、适用条件错误、弱检查返回 `ok` 和原生能力被永久省略列为缺口。
- 原生证据根目录为 `/Users/Zhuanz/Documents/New project 5/relay-recovered/maintenance-full`，下文 `A/` 指该目录。安装版副本 `AiMaMi.arm64` SHA256 为 `b2a4b1b2b4babd2518e02c0f0d6203ad19e779909a47b3b643844ef403fe95de`。C 中含编译优化和未命名字段，不将旧 DSH 稿当作完整恢复源码。

## 首轮生产入口快照（历史定位，接线更新见上）

| 层级 | 当前实际入口 | 事实及边界 |
| --- | --- | --- |
| 命令注册 | `src-tauri/src/main.rs:29` | 注册诊断、预览、确认、工具与图片兼容命令。 |
| 诊断 | `src-tauri/src/maintenance/diagnostics/mod.rs:168` | `Context::native` → `inspect::run` → 脱敏摘要。不是 `native_manager::report`。 |
| 预览 | 同文件 `:179`、`:185` | `repair::preview`；私有票据、5 分钟有效期。不是 `native_manager::preview`。 |
| 确认修复 | 同文件 `:200`、`:218` | 明确确认、执行互斥、消费票据后 `repair::apply`。不是 `native_manager::apply_native`。 |
| 报告覆盖 | `diagnostics/inspect.rs:1129` | 最后覆盖原生 threads 六项、auth 三项、catalog 两项。没有调用 `native_router::diagnose`；其余保留前面的旧分支。 |
| 新 Manager | `diagnostics/native_manager.rs:117`、`:152`、`:343` | 候选生产适配正在实现，已具 report/preview/apply 外形与 typed Host；**未由上述生产命令调用，尚未完成验收**。 |
| 新 Router | `diagnostics/native_router.rs:480`、`:921`、`:1101` | 已有目标态诊断、具体协调计划与执行；当前由候选 Manager 调用，不能倒推旧 35 项已替换。 |
| 已生效局部协调 | `diagnostics/native_coordinator.rs:80`、`:200`、`:592` | 配置/认证/catalog 共享配置只发布一次，Integrity 先于 Unlock；这是局部组合，不等于原生完整 Manager。 |

以下位置简写：`I`=`src-tauri/src/maintenance/diagnostics/inspect.rs`，`R`=同目录 `repair.rs`，其余未带目录的 `.rs` 也在该目录。检查 ID 全集见 `src-tauri/src/maintenance/utilities.rs:110`。

## 首轮 35 项检查与修复事实表（变化项以上述复核为准）

| # | 检查 ID | 生产检查及状态 | 实际修复入口 | 剩余缺口 / 原生基准 |
| --- | --- | --- | --- | --- |
| 1 | `config_access` | 部分；`I:465` 读取已有文件，缺失返回 skipped | 无，`I:482` 固定不可修复 | 没有完成配置访问/缺失/受限路径的原生分支与恢复策略核对；不可把缺失统一解释为已通过。原生集中在 `run_diagnostics_with_mode @10037e464`。 |
| 2 | `config_replace_readiness` | 原版为 metadata/普通文件/权限观察；本轮已分离观察错误与缺失状态 | 无自动权限修复 | **纠正早期误判**：原版没有配置替换试写；`0x100376210` 是 `fix_config_preflight`，仅按需修 TOML。元数据通过不等于 ACL、沙盒或实际替换已验证，不应为追求等价添加真实配置试写。 |
| 3 | `config_toml_syntax` | 已接；`I:499` 真 TOML 解析、缺失/空白文案 | `R:161` → `config_syntax.rs` 抢救、备份、写入与复检 | 具体链路存在；Manager 门控仍待统一。原生 check `@10037b7a8` / fix `@100377794`。 |
| 4 | `config_third_party` | 部分；`I:250`、`:532` 递归已知键 marker | 无；`I:552` false，coordinator handles 不含此项 | core `config/mod.rs:738`、`config/fixes.rs:439` 已有真实检查与文本修复但未调用；base URL、归属、provider 清理等不由旧键扫描覆盖。原生 check `@10037aa04` / fix `@100377458`。 |
| 5 | `config_profile_conflict` | 已接 core 检查；`I:558` → `native_config.rs:12` | `native_coordinator.rs:200` 组合 profile 清理 | 启用态参数仍是 `owned_provider`（`I:560` 只认 Atelier），不是原生 RelayState 或新 RouterIntent；Native RouterOn 分支适用性需统一。check `@100390ce8` / fix `@10038a1f8`。 |
| 6 | `config_stale` | 部分；`I:617` 只核 Atelier 选中 provider 与有限 HTTP 字段 | coordinator 的 Gateway 配置 patch | core `config/stale.rs:209` 真配置 stale 检查未作为该报告项调用；Native/Off/目录与遗留状态完整链未接。check `@100372660` / fix `@100371e0c`。 |
| 7 | `config_global_validity` | 已接；`I:563` → `native_config.rs:22`，managed_root 明确 relay 目录 | coordinator → core `fix_config_global_validity_at` | 已分离 config 输出路径与 managed_root；不能再按整个 Codex home 误删引用。完整 Manager/pending 恢复仍待接。check `@10038f3ec` / fix `@100387c84`。 |
| 8 | `router_recovery_record` | 未接真实记录；`I:659` 的 app gateway 固定路径 presence | 无，循环固定不可修复 | 存在仅 warning，不存在 skipped；不是实际 checked journal/phase/receipt 恢复。core journal/Manager guard 已实现但生产入口未使用。原生 run `@10037e464`、Manager 与 commit guard 证据见下。 |
| 9 | `proxy_running` | 部分；`I:700` 根据 managed 与 runtime.running | 无 | runtime.running 可直接 ok，没有实际 owned handle 存活与 TCP 连接证据。新 verifier 有真实 handle+TCP，但该项未替换。 |
| 10 | `router_http_contract` | 部分；`I:600`、`:640` Atelier URI/字段启发式 | coordinator Gateway patch | 原生 check `@100387550` / fix `@10037bc04`；新显式 Native/Atelier verifier 已存在，但生产报告仍未调用。原生此 helper 是配置契约，不应误称 HTTP 在线请求。 |
| 11 | `router_postcondition` | 未接；`I:659` 用 transition.json presence 填项 | 无 | `native_router.rs:480` 已有实际 Full 维度验证，尚未进 inspect；直接修复特殊分支、普通/all 差异与确认回执需 Manager 接通。 |
| 12 | `takeover_backup_orphan` | 已接；`I:1138` → native_auth 三项覆盖 | `native_auth` specialized takeover plan/apply，经 coordinator | 原生真实路径、现代/legacy decode、恢复/删除副作用已实现；Gateway 模式解析仍需新 intent 统一。check `@10038fafc` / fix `@1003885e8`。 |
| 13 | `catalog_path_validity` | 条件已接；`I:1161` → `native_catalog::inspect` | `native_catalog::prepare_item` + coordinator 实际发布及 TOML 修正 | **context_source Err 时静默保留旧 I:727 项**；可能把未知来源降为旧启发式 ok。check `@1003898a0` / fix `@10037d440`。 |
| 14 | `catalog_integrity` | 条件已接；同上，真实 typed source/builder | 同上；完整构造、签名/revision 与原子 publication | 同来源错误回退缺口；目录获取/缓存新鲜度不是本项构造成功可以证明。check `@10037795c` / fix `@1003764e4`。 |
| 15 | `official_catalog_freshness` | 未实现；`I:770` 固定 unknown | 无；`:777` false | 没有实际最近请求记录、缓存时效或官方目录获取链。该显式 unknown 诚实但属于未完成，不是通过/不适用。需继续静态恢复原生对应内联分支与 native_catalog runtime。 |
| 16 | `auth_integrity` | 已接；native_auth file check，显式 keyring/ephemeral 排除 | core `plan_auth_integrity`，从真实 accounts/backups 候选恢复并复检 | Integrity→Unlock 顺序及 A/B 重叠已具体处理；不以“复制成功”当登录合格。非文件排除是诚实边界；尚不能代表原生 Keychain 所有认证能力。check `@100375360` / fix `@100373af8`。 |
| 17 | `router_unlock_auth_residue` | 已接；native_auth 原生 marker/backup/live auth 检查 | specialized unlock apply；前序批准效果后 fresh plan 可缩范围 | 保留 NoAccount 拒绝；Gateway 模式目前 fallback 到 runtime/config，需 intent 统一。check `@100397070` / fix `@100393114`。 |
| 18 | `api_key_integrity` | 部分；`I:821` 只读启用 Gateway apiKey 档案 UUID 与非空字段 | 无 | 有真实 Atelier 凭据文件检查；不涵盖安装版 provider credential/revision/Keychain 分支。原生 run C `:3461` 确实调用 `keychain::get_api_key`，不能凭本地字段 ok 宣称完整等价。 |
| 19 | `auth_token_expiry` | 部分；`I:844` file JWT payload exp 比当前时间 | 无；`:874` false | 原生 run C `:4057` token_remaining_seconds、`:4147` refresh_token 分支尚未逐支接入；当前无 refresh 可用性/存储策略完整判断。签名/在线认证并非本地 expiry 检查必须做，缺口不应以额外在线要求表述。 |
| 20 | `codex_process_state` | 部分；`I:876` desktop_running → ok/跳过 | 该项无修复；独立 forceKill/restart 已接原生 lifecycle | Context 中旧进程发现不等于 native classifier 的所有相关 CLI/writer/protected/external；新进程库存在不能代表该报告项已换用。 |
| 21 | `db_orphan_providers` | 部分；`I:358`、`:918` 只认 Atelier provider 前缀 | `R:252` 旧迁移路径 | 未调用 core `threads/mod.rs:204` 的原生归属、valid ids/quarantine 规则；修复遍历全部 t.rows，可能将仍健康的第三方线程也改到当前 provider。check `@10037c310` / fix with catalog `@10039e0c0`。 |
| 22 | `rollout_orphan_providers` | 部分；`I:399`、`:925` 同旧前缀/扫描 | 同 `R:252` | 同上；缺真实 orphan ID 筛选、原生缓存/ownership、prepared official repair catalog 与模型保留/回退链。check with cache `@1003a4ba8`。 |
| 23 | `thread_missing_rollouts` | 已接；native_threads 六项覆盖，实际源库私有快照 | native_threads plan/apply，按原生恢复/归档并复检 | 真实数据 E2E 未执行；静默窗口由 Manager 生产接线补齐。check `@1003921a4` / fix `@10038ec3c`。 |
| 24 | `thread_history_integrity` | 部分；`I:937` 只用旧 t.bad 首元数据/解析计数 | 无 | 原生 run C `:1586` 调用 `inspect_thread_history_integrity`；当前六项 adapter 不含此 ID，完整历史/结构分支未证明恢复。不能把旧 t.bad==0 当完整历史健康。 |
| 25 | `thread_provider_consistency` | 已接；native_threads 私有快照、真实 cache 检查 | 具体 provider 修复；依赖历史修复时延后重计划，非假成功 | Native/Off 适用模式仍依赖输入 gate；Manager 未完成。check `@1003a5d7c` / fix `@10039573c`。 |
| 26 | `thread_model_catalog_consistency` | 已接；native_threads 真实目录/线程模型检查 | 具体 model catalog 收敛、源依赖核验与复检 | 目录来源与运行模式必须与 RouterIntent/Native source 一致；不能由无效来源的空目录代替。check `@10039d6e4` / fix `@10039c9f0`。 |
| 27 | `native_off_invalid_openai_thread_models` | 未对齐；`I:448` 只统计 model 是否含空白，`:958` 填结果 | 无，`:961` false | 没有真实 NativeOff/官方 provider/有效模型语义；无空白的失效模型可 ok，RouterOn 也参与同一检查。需恢复并接原生内联规则/恢复策略。 |
| 28 | `thread_poisoned_instructions` | 已接；native_threads 实际主线程 plan | 原生指令修复实际执行并复检，非只指导 | 原生 check `@10039b888` / fix `@100397f7c`；Manager 生命周期尚待接。 |
| 29 | `thread_response_item_ids` | 已接；native_threads 实际可修复路径/计数 | 具体去重/补 ID 及后验，读错不报健康 | 原生 fix `@10038fd30`；保护路径/不可修复须保留具体结果。 |
| 30 | `legacy_migrated_threads` | 已接；native_threads 真 manifest/线程检查 | Off 时具体 cleanup；On 拒绝不是功能缺失 | check `@100391470` / fix `@10038d874`；应保留启用态门控，不能为了 all 强行清理。 |
| 31 | `residual_manifest` | 部分已接；`I:1030`、`residual_cleanup.rs:24` 真 Native+Atelier 文件 | `R:236` 明确文件备份、删除、复检 | gate 以 effective provider 或 Gateway runtime 为 On；runtime 运行不等于 Desktop 已接管，可能将 Off 残留报正常。fix `@100376750`。 |
| 32 | `residual_cache` | 部分已接；同上真实 cache 文件 | 同上 | 同模式缺口；不能把所有存在 cache 一律删，也不能因 Gateway running 就正常。fix `@1003746ac`。 |
| 33 | `codex_home_writable` | 部分；`I:1060` metadata readonly，部分 metadata 读错被忽略 | 无 | 原生 run C `:4354` 非只读模式逐目录实际 write/remove 探针；当前 ACL/配额/空间未测仍可 ok。上游只读模式可不写，但不能永久删掉有写入探针的正常模式。 |
| 34 | `project_state_integrity` | 部分；`I:1084` 缺文件 skipped，其余旧 inspect_existing | `R:297` 旧 project JSON/cwd 修复 | core `threads/mod.rs:164`、`:482`、`:748` 原生检查未调用；旧 `project_state.rs:134` 缺 source 列仍继续推断主线程，违反已恢复规则。缺文件重建、完整 DB/roots/hints/backfill 分支需接。check `@100391c44` / fix `@10038e4b8`。 |
| 35 | `state_db_relocation` | 已接；`I:1116` → 实际 sqlite_home 归属和旧根评估 | `R:219`、`:830`，三文件 DB/WAL/SHM 备份后删除，流式摘要和部分失败复检 | 具体实现存在；unknown 根不强删是合法门控。完整 Manager pending 恢复/静默窗口仍待生产入口接通。 |

## 维护工具功能表

| 功能 | 实际连接与副作用 | 覆盖结论 / 仍须完成 |
| --- | --- | --- |
| 系统诊断弹窗、单项/全部修复 | `DiagnosticDialog.vue:44`、`:65`；后端上述三个命令；35 项结构化报告、票据、日志与复检 | UI/命令存在；不能由 ID 数量替代上表业务缺口。旧 all 仅组合当前 fixable 项，未具原生完整 fix_all/Manager 语义。 |
| 强制结束 | `maintenance/mod.rs:179` → `native_process.rs:240` → core `force_close_all` | 已接具体 classifier、ForceCloseAll、protected/external 规则、目标预览和退出后复检；不得误缩成只关主 GUI。真实客户端验收按用户边界未执行。 |
| 重启 Desktop | 同工具入口 → `native_process.rs:256`，5 秒 ForceCloseAll prepare 后 launch/激活/确认主 PID | 已接固定平台 argv 和 bundle 路径校验；退出失败不启动、启动失败明确部分结果。与修复后 Manager 条件重启是两个入口，后者仍 WIP。 |
| 清理 | `maintenance/store.rs:46`、`:103`、`:238`、`:411` | 实际检查 file OAuth 结构，删 stale 账号索引与维护备份，保留 profile 凭据目录；有预览、锁、CAS、目录备份和删除失败计数。安装版 `run_daemon_once_blocking @1007b4dd8` 尚须逐闭包核清理对象/错误/调度；当前不能宣称 daemon 全语义等价。 |
| 导出诊断包/保存 | `utilities.rs:147` allowlist → `:175` ZIP → `:222` / `:455` | 真实 ZIP 与用户选择保存；导出上次摘要而非隐式重跑诊断。原生 debug bundle 更广字段未宣称齐全；不导凭据/原始会话/任意日志是 parity-plan 已明确的 Atelier 产品边界，不应倒补敏感材料。 |
| 发送诊断信息 | `utilities.rs:254`、`:340` | 明确确认、自选接收地址、实际 multipart/超时/重定向禁止/响应校验；无 AiMaMi 默认收件服务符合计划。只返回 HTTP 接受信息不能冒充远端工单已处理，真实外部服务验收未执行。 |
| 图片兼容 | `image_config.rs:60`、`:85`、`:138`；`gateway/server.rs:426`、`:437` | 实际配置变更、备份、原子发布/读回，且网关真实请求调用 core image tool 声明过滤，已超过仅保存偏好。Desktop 实际重新读取配置与真实请求验收未执行。 |
| Claude 联网兼容 | `Maintenance.vue:118` | 与上游一样尚未开放，计划明确保留此状态；不是应偷偷补成另一功能的缺口。 |
| 重建账号注册表（Atelier 次级功能） | `store.rs:326`、`:411`，`Maintenance.vue:138` | 实际重建 Atelier 索引、保留有效字段/账号与备份；这是已授权保留的附加入口，不替代原生维护清理。 |

## 首轮优先级清单（已关闭项见同日复核）

1. **完整 Manager 生产接线（进行中，不能判完成）**：将真实 typed GatewayCatalog/RouterIntent 或 Native RelayState 接入同一已恢复控制流；确认前完成可审阅 plan，确认后锁、checked recovery、写入 preflight、quiet/fence、实际修复/重协调、target verify、receipt/sync、补偿和条件重开。更新命令所存票据类型与诊断来源，不能仅新建模块。保留 pending 可恢复分支，不继续 blanket 拒绝所有 journal。原生 `decomp-fix_diagnostic_issue_audited_main.c` 及 core `evidence/manager/README.md:1` 已有逐分支证据；stage/attempt 不可退回 mutated bool。
2. **收紧旧孤儿 Provider 修复真实对象集合**：`R:264` 当前按全部 t.rows 转成当前 provider；必须以实际 orphan IDs、归属与模型保留规则计划和预览。原生 core `threads/mod.rs:196` 明确只认封闭管理归属，`:229` 只选择相应失效主线程。此问题不是“检查文案不准”，会影响未失效线程的 provider。
3. **消除报告中的候选未调用与旧替身**：接第三方配置 check/fix、真实 stale/HTTP/Router postcondition/recovery、官方目录新鲜度、project/history/native-off-invalid 检查。不能把当前固定 unknown、presence-only skipped、仅空白字符检查转述为原生完整覆盖。
4. **来源失败必须保留失败**：`I:1161` 的 `if let Ok(source)` 缺 else，来源错误会留下旧 catalog 状态；应返回具体 unknown/error 并禁修复，不能继续旧“存在 JSON/models 数组就 ok”。
5. **模式必须来自真实意图/状态**：`residual_cleanup.rs:53` 将 runtime running/starting/stopping/requests 视为已接管；`native_auth.rs:108` 与 `native_threads` 复用该 gate，`native_catalog.rs:580` 也有 gateway runtime 影响来源选择。新 RouterIntent 具体实现已存在但尚未统一旧入口；不能用 Gateway start 代替 Desktop RouterOn，也不能创建假的 Native 账号/RelayState 满足类型。
6. **恢复目录探针和原生 credential/expiry 分支**：原版配置替换 readiness 仅元数据，不存在所谓配置替换探针。目录写探针需明确确认、异常与清理失败完整传播，普通诊断/修复预览不执行。API key/expiry 按实际账号/凭据源适配原生语义；未授权 Keychain 不暗查，不能把 unknown 写成等价完成。
7. **完成工具逐分支审计与最后验收**：清理 daemon 闭包、官方 catalog runtime/获取、尚未解原生内联分支均需留下证据；再验证固定生产入口走新实现及有效反例。编译/合成测试通过、35 项显示齐全均不足以关闭此清单。真实 Desktop 操作和窗口验收仍由用户执行。

## 状态误报的具体边界

- `I:1190` 的 `has_issues` 只数 warning/error，unknown/skipped 不计入。因此 `has_issues=false` 不能解释为“35 项都已验证健康”。前端 `DiagnosticDialog.vue:37`、`:47` 已分别显示正常/留意/异常/未确认/不适用和计数，不能指控 UI 将 unknown 直接渲染成正常。
- 真正可能弱校验 ok 的例子：替换就绪只看 readonly；目录 metadata 错误被 filter_map 丢弃；proxy 只看 runtime.running；native-off 模型只看空白；catalog 来源错误回到旧 JSON 检查；project 缺 source 列继续选行。
- 合法拒绝/不适用必须保留：启用态不清 legacy/residual、NoAccount 不执行某些认证恢复、未知数据库归属不删、keyring/ephemeral 不用 file auth 覆盖。这些不应被 all 绕过；缺少真正的另一存储适配则仍登记为未完成。

## 本次验证与更新条件

执行的是 `rg`/带行号源码阅读、静态生产调用者核对与本文档 whitespace 检查；未运行任何程序功能或测试，未把既有测试总数当覆盖证据。并行实现继续变化，尤其 `native_manager.rs` 及共享 `mod.rs`；接线后应重新核对入口和上表受影响行，保留本文审计日期，不把 WIP 自动改成完成。新增的实际用例应围绕前述反例和副作用边界，而不是重复实现本身。
