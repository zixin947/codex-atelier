# Native batch 后 Router 重同步审批

原版 `fix_all` 的认证先执行：`auth_integrity` @`0x1003a6d54`，其后 unlock @`0x1003a720c`，其后 takeover；`codex-direct/decompiled/fix_all.c:832` 单独记录 resync 错误，继续保持具体已完成/失败项目。原版完整阶段恢复见 batch_repair 证据。本文件描述 Atelier 对这些真实原语的审批适配，不宣称原生存在下列 token/receipt 类型。

旧 standalone Router 恢复只接受 catalog→config→auth 的原计划效果前缀，不能解释 full-batch 已执行的 Integrity→Unlock→Takeover→各纯配置准备。新实现仍保留旧 API 行为，为 batch 提供独立具体入口。

- `BoundBatchResyncToken` 仅从原 `ReviewedPlan` 和 `native_coordinator::PreparedBatch` 构建；初审完整合并依赖 union，重叠 before 必须一致。原 source/target/真实 proxy/CLI signature/catalog payload 固定。
- `PreparedBatch` 的真实原三 auth plans 和有限 config candidate 图由 coordinator 独占构造；Router 不接受调用方任意中间 bytes、普通 effect tuple 或 `valid` 布尔值。
- 每个预审 candidate 使用实际 Native `config_patch` 渲染一次并绑定 exact final bytes。无法渲染的分支绑定确定错误；运行时不从当前任意内容重新 render 后签新批准。
- 执行必须提供同一个 PreparedBatch Arc 身份所产生的 opaque `ApprovedBatchPrefix`。该凭据在真实原语执行及原批准字节回读后构造。Router 再核整个 union 当前值只能等 original 或该凭据 exact before→after，并单独要求真实 Native source 不变。
- Auth 重新规划只用原 batch actions 和已证 prefix；来源、候选路径集合、目标、after 均不得扩展。已执行的 Integrity 恢复 A 后，Unlock 的 auth effect 可自然缩为 no-op，不能再用原 unlock 备份 B 覆盖 A。Takeover 已执行后也是 no-op。
- batch 失败分支不偷偷再批准不同的 Takeover 配置。candidate 已包含原始/恢复两起点；对应分支 Router 只执行其预审 final。认证阶段无法建立安全部分效果时，opaque prefix 会 poisoned，后继必须失败。
- `prepare_bound_batch_resync_from_source` 可从真实 Native source 与 batch 候选构造坏 TOML 的只读 seed，不临时写救援配置。seed 不能直接 apply；只有真实 Syntax 发布并回读、获批候选重基后才能成为可执行计划。NativeOn 仍需要真实 proxy handle。
- `apply_bound_batch_resync` 保留既有备份/readback/CAS/required-sync；`ApprovedResyncEffects` 只有批准 exact final hash 在实际文件读回吻合才可用于后续阶段。凭据是完整 original→resync-final 的累计 union（包含原三 auth 和 pure config），覆盖 O→O 相同字节以及真实初审/回读 mtime，防止 pure 改变后 resync 回原字节时漏项。
- 发布后同步未确认或任何部分失败会保留具体 Outcome.completed/failure，并返回 poisoned receipt；validate_current 拒绝后续使用，不能把已发布错误转成功。

稳定 API：

```
prepare_bound_batch_resync(ctx, original, prepared_batch) -> BoundBatchResyncToken
prepare_bound_batch_resync_from_source(ctx, source, target, backup, actual_proxy, prepared_batch)
    -> (ReviewedPlan /* non-executable seed */, BoundBatchResyncToken)
rebase_bound_batch_resync(ctx, original, token, opaque_batch_prefix, backup) -> ReviewedPlan
apply_bound_batch_resync(ctx, original, token, opaque_batch_prefix, backup, MutationGuard)
    -> (Outcome, ApprovedResyncEffects)
```

Token 提供 `config_candidates`（key→exact final 或确定Err）、`approved_dependencies`、`catalog_effect`、`validate_initial`。Receipt 提供 `plan`（same_plan身份）、`paths`、`effect_stamp`、`validate_current`。这些类型不 Debug/Serialize、不经 IPC 输出配置/认证原字节。

测试仅使用临时目录、synthetic auth A/B、真实 core auth plans 和 coordinator 阶段；不读真实客户端数据，不调用真实 Keychain，不 discover 或启动任何系统客户端。新增分支覆盖三 auth 执行后实际重同步、有限 pure candidate、旧序列拒绝、另次相同预览不能替代身份、auth/config/source/候选外部变化、Gateway拒绝、初始union漂移、累计receipt/mtime、坏TOML Syntax seed，以及真实发布后注入目录同步错误保留partial结果。

正式验证：2026-11-08 `cargo test --manifest-path src-tauri/Cargo.toml native_router -- --nocapture`，session48220，exit0；40个匹配测试全部通过，2.87s（新增本模块7项 bound 回归）。较早session87936的6项bound亦全部通过。保留原standalone32项接口行为，没有以batch token替代Gateway或单项审批。`rustfmt --check` 与本范围 `git diff --check` 通过。

明确边界：若选中的原 auth primitive 在坏 TOML 初审时自身无法建立具体计划，PreparedBatch 仍明确返回该错误；本模块的 Syntax seed 只支持已合法建立的 batch，不绕过 auth 审批失败。未知部分文件效果产生的 poisoned receipt 不可用于后继阶段，须由上层 Manager 的恢复流程处理，不能给任意当前 bytes 重新签名。

## ObservedOff：实际缺失来源

SourceNone 的独立路径不构造 Snapshot 或默认 RelayState。`prepare_bound_observed_off(ctx,batch)` 生成同一类型 BoundBatchResyncToken，`apply_bound_observed_off(ctx,token,prefix,backup,MutationGuard)` 生成同一 cumulative ApprovedResyncEffects。

实际绑定的状态来源只有3个：`home/codexmate/relay/state.json`、`appRoot/gateway/catalog.json`、`appRoot/gateway/router-intent.json`。第四个输入是确切 `config.toml` witness，可存在且语法损坏，不臆造第四状态文件。已有Off intent允许保留原文件；On intent、无intent但顶层明确Atelier归属、实际Native/Gateway来源出现、坏既有文件或读取错误均不被当作SourceNone Off。

原生缺省证据：`relay_storage_load.c:160–208` metadataErr default return，:203 router-enabled字节16c为0，:204 17e首字节为0；`relay_state_serialize_2.c:51–54` / :63–66分别确认Enabled/NoAccount字段。这只证明原生 state 缺失时的Off默认；Atelier要求三个来源缺失以及原始config witness的边界是显式适配。badTOML时，coordinator实际mode observer根据这组初始来源/配置判定并固化；当前实现仅对syntax且未选auth的这种默认分支使用，不绕过认证原语自己的预审失败。

ObservedOff 候选复用已证 `native_off_config`（先strip managed blocks，再解析并清理owned字段），并复用既有GatewayOff的Atelier激活/profile/own catalog引用清理规则。保留原user auth、无关配置、既有目录文件和凭据文件；不创建Native state、Gateway catalog、RouterIntent或account。若实际auth Integrity恢复认证，仅来自原来已审核的真实候选文件。

执行完整核opaque batch身份、原deps union和exact candidate，fresh auth严格沿原批准subset；备份/required-sync/readback后调用实际core `verify_native_off_with_depth`（含实际DB/rollout），另核不再激活Atelier provider。没有listener-running或caller-valid布尔值替代目标检查。

最终验证：2026-11-08 session24883，`cargo test --manifest-path src-tauri/Cargo.toml native_router -- --nocapture` exit0，45/45匹配测试通过，3.36s。新增5项ObservedOff回归：SourceNone auth3+syntax真实执行；来源出现/外部bytes/实际On拒绝；persisted intentOff撤Atelier激活但保旧catalog/auth；坏TOML明确Atelier/不可读source不当absence；三来源缺失badmanagedsyntax按实际defaultOff witness执行Syntax→resync而不创建来源或auth。
