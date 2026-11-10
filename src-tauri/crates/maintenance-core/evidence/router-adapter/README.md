# Native / Atelier 路由 Host primitive

实现：app `src/maintenance/diagnostics/native_router.rs`；core `src/router_verification.rs` 的显式 `VerificationContract` 与 `src/router_transition.rs::run_host_repair`。该适配层不把 Gateway Catalog 转换成 RelayStateDocument，不从 runtime.running 推导已验证状态。

## 实际来源与验证

- Native：显式 Source::Native、真实 RelayStateDocument；原有默认验证函数保持原生 aimai1、`/codex/router/v1`、relay catalog、原生 unlock/认证残留规则。Context 的 Gateway ProxyHandle 不用于证明 Native proxy 存活；Native Manager 必须传自己的实际 handle。缺少 native 上游凭据存储确认时实报 Auth 未确认，绝不读取隐式真实钥匙串。
- Gateway：强类型 `gateway/types.rs::Catalog/Profile/Settings/Credential`；目标及 expected provider 来自 `router_intent::prepare` 读出的实际记录/配置观测。运行中的网关服务不覆盖 Off 意图。Explicit target 参数表示需要验证的目标，不是 caller-valid bool。
- Atelier contract 按实际 Responses server：owned provider 名称、`/v1`、`responses`、`env_key=ATELIER_GATEWAY_TOKEN`、false requires_openai_auth/websockets、静态原始模型 ID catalog。实际有效 profile/top-level provider 和 model 均参与检查。
- Catalog 比较 preferred profile 的原始 ID 集合，不向文件写 native provider::model 假名称。没有 preferred / 空 inventory 不能使整个路由标绿。
- Gateway 账号按 server 的 enabled、auth_status、cooldown、同种类、auto_switch、priority、max_attempts 选候选，并真实读取 private `gateway/credentials/<UUID>.json`。API key 缺失/格式错误为 Auth issue；ChatGPT 本地未过期 access token 或可供既有 AuthManager 刷新的 refresh_token/client_id 是本地材料。后者带单独 warning，不声称在线刷新或上游授权已成功。
- core 实际检查 SQLite 活跃主线程的 provider/raw model，检查真实 rollout 身份及首/末/native provider anchors。unmanaged foreign provider 与 subagent 行保持独立。
- proxy 检查 actual owned handle、lifecycle AtomicBool 和实际 loopback TCP；单一 true flag 不足以通过。该检查不证明 Codex Desktop 持有本次 Gateway 本地 key，也不证明上游生成可用。server Live.key 每次启动变化，gateway/mod.rs 的手动配置预览明确要求更新环境变量。
- app diagnose 使用实际 checked journal：Native relay root；Gateway appRoot/gateway。committing / malformed journal 不能给出健康状态。验证入口本身不排斥 committing，以供实际 commit-recovery 验证，Manager 另执行 journal gate。

## Reviewed 配置与发布

`prepare_reconcile_for_target` 生成具体 config/catalog/auth plan；`config_effect` 给出路径与实际前后字节，`approved_dependencies` 只给 len/SHA256 stamps。Source/intent/config/catalog 初始依赖在第一次备份前验证，catalog 还有自身 CLI metadata signature 校验。所有 preview 不含 credential bytes。

Gateway On 修复原有或真实意图中明确归属的 Atelier provider，保存无关字段与注释。Gateway Off 只撤自己的 active provider/profile 选择和 catalog 引用：不修改 Codex auth.json、不清 native unlock 残留、不删生成 catalog、不改 Gateway Catalog 或上游 credentials。

Native Off 将原生 takeover 的合法配置恢复效果与退出清理合成，再只发布一次 config。退出清理依据已恢复的 RouterConfigSurface 与 native owned IDs 清除裸写 aimai1 顶层键/owned sections，弥补 core stale 的“仅 strip managed blocks，后续由 Manager reconcile”边界；无关配置保留。原生 unlock 优先保留当前用户登录，再清原生标记/备份；不把另一个备份覆盖到当前用户。

`apply` 取得 repair::acquire 的两个真实文件锁；`apply_under_manager` 验证外层 MutationGuard 并走同一 body，不重入锁。配置原始 bytes（含不存在状态）以私有 base64 manifest 保存并回读；实际 config 发布 require_sync 后才执行 auth 残留删除。失败保留具体完成阶段、备份和真实复检结果；已 rename 但 sync 未确认单独记录，不假装未写入。

## 后继计划边界

`compose_config_after(ctx, snapshot, target, primaryAfter)` 对 Gateway 作纯配置合成，不写文件。`rebase_after_approved_config` 要求实际当前 config 精确等 primaryAfter；所有 source、intent、catalog 和其他依赖保留初始 bytes。复用旧 snapshot 的 target/provider，避免 profile 清理后重新观察成 Off 而丢失审核意图。上层必须把后继 `config_after` 与最初展示的 expectedFinal 精确比较。

兼容的 `rebase_after_approved_config` 仍保留 Native 限制；下述专用 NativePrimaryComposition 已闭合 global/profile 两项。Native auth-primary 交叉作用仍需独立确切 effects ledger，未闭合部分不冒称完整 fix_all。Source/intent 或 catalog 已被其他阶段修改时也必须按实际批准效果专门衔接，不能盲目重新 prepare 扩 scope。

## 验证

测试均从 app 实际模块或 core 公共 crate API 导入，只操作临时目录、合成凭据、真实 loopback listener、临时 SQLite/rollout 和 FakeRunner。包括实际 HostState → run_host_repair → reviewed config/catalog apply → RouterIntent commit/readback 的链路，并断言没有创建 native RelayState/accounts 作为替代。最终命令及计数由交付消息列出。

## 已批准 partial 的真实继续修复

`resume_after_approved_effects(ctx, reviewed, backupRoot)` 是应用层 reviewed transaction 扩展，不冒称一个独立原生符号。它把原始 union stamps 按真实执行顺序推进：catalog payload → 最终 config → 各原生 auth action 的有序 non-config after effects。整个实际 union 必须恰好等于其中一个完整前缀；不能逐文件任取 before/after。source、intent 和没有被批准写入的依赖始终保持原始 stamps；CLI 另外核原 metadata signature。

确认前缀后，native_auth 的 `replan_after_approved_effects` 只作真实重新规划，不执行；fresh 来源、目标必须是原批准 subset，留下的 after stamps 精确等原批准效果。配置候选再走实际 Native takeover plan 与 config patch，最终 bytes 必须等旧批准 final；catalog 重建后的 encode payload 也必须等旧 payload。Gateway 始终沿用旧 snapshot 的 target/provider，不因 config 已修改重新观察出另一目标。

原生 quarantine copy 若已完成而 source 删除失败，prefix 必须证明 copy 是原审核数据；纯 replan 固定旧 destination，并仅把这种已批准 copy 的 before 设为已证明的 after。后续再次 fresh replan 使用其真实 pinned before，不接受任意当前 bytes，也不分配一个新 quarantine 名称。普通初次 prepare 仍要求原目的地不存在。

resume 不删除旧备份：新 backupRoot 保存此次 config 输入，auth 原件归档仍在原批准 root 的唯一子目录下。config 已是 final 时，apply 会真实同步该文件与父目录、再回读后才消费 auth backups，从而能确认上一次 rename 后的 sync uncertainty，而非直接把旧错误当作成功。

新增真实 fixture 覆盖：实际 config 发布后以 `.live-auth.lock` 目录造成 EISDIR 认证失败，清障后按旧 plan 完成；catalog 已发布而 config 拒写再继续；config rename 后同步错误再真实 fsync 继续；外部 auth/source 改动拒绝；config/auth 或 auth 内部乱序 after 值拒绝；已落盘 quarantine copy 固定旧路径并完成实际 source 删除。测试只使用临时路径，故障不触碰真实客户端。该阶段路由测试 27 项通过，原有 auth 测试 28 项独立保留。

partial resume 不替代 primary 组合；Native global/profile 由下面的专用 token 单独审核与执行，旧 wrapper 的限制为兼容保留。


## Native global/profile 专用组合闭环

`prepare_native_primary_composition(ctx, reviewed, approved_item_ids, primary_after)` 仅接受 `config_global_validity` / `config_profile_conflict`，按固定 global→profile 顺序执行已恢复的实际纯算法。先从原始 config 重新计算并精确核对 primary_after；再取原 auth/takeover plan 的真实配置恢复候选，重跑同一两算法，最后按显式 Native target / owned native proxy 渲染路由字段。它不建立虚拟 FS，不对当前任意字节做三路合并。

返回的 `NativePrimaryComposition` 不实现 Debug/Serialize；只通过 getter 提供 final/primary 配置、IDs 与 metadata 观测。初始 config、native source、auth 原件/备份/隔离目标、catalog 和 plain final stamps 均绑定原 ReviewedPlan；两轮 global 算法查询的真实路径 metadata 状态也记录并在 rebase、apply、resume 时重核。NativeOn 使用原 ReviewedPlan 持有的独立 native ProxyHandle，不能复用 Context 的 Gateway handle。

`rebase_native_primary(ctx, reviewed, token, backup_root)` 要求实际 config 等 token.primary_after，所有非config原始依赖不变，再重建 token 并要求 final 与 metadata 范围完全相同。fresh auth 仍固定原批准 nonconfig 来源/目标/after 值；其 fresh config candidate 被消费掉，不能把已清理的 model_catalog_json、profile 或其它字段写回。

这种 Plan 的 partial resume 重放原审核的真实 takeover 候选，再执行获批 cleanup 和路由 patch，最终 bytes 仍须等原 token.final；因此 config 已发布但 auth 后段失败时，可以继续清理，而不会因 fresh takeover 想恢复已删除字段而覆盖最终配置。旧备份和用户当前登录保持原有保护。

最终定向验证：`cargo test --manifest-path src-tauri/Cargo.toml native_router::tests` 32/32；`cargo test --manifest-path src-tauri/Cargo.toml native_auth_tests` 28/28。新增五项覆盖真实 NativeOff takeover+global/profile、metadata/auth 来源变化拒绝、未知IDs和伪造primary拒绝、独立native proxy的On配置、组合后真实认证锁故障与partial继续。app 测试编译带9个既有/未使用helper warnings，无编译错误。没有执行真实 Codex、AiMaMi、真实钥匙串或在线 OAuth 刷新。

剩余明确边界：auth-primary 与 native auth 多步骤效果的 ledger、其它 primary config 算法、full_all 多轮跨模块效果仍归上层后续审阅；本 token 对未知IDs及未批准非config变化直接拒绝，不将它们收入已审scope。
