# 孤儿 Provider 接管：原生证据与生产适配

日期：2026-11-08。范围为 `db_orphan_providers` / `rollout_orphan_providers` 的真实候选集合与配套修复；不以当前 Provider 覆盖全部线程。安装版证据根 `A=/Users/Zhuanz/Documents/New project 5/relay-recovered/maintenance-full`，SHA256 同 `maintenance-production-gap-audit.md`。

## 已恢复的实际规则

| 规则 | 原生证据 | 生产实现 |
| --- | --- | --- |
| 归属是封闭管理集合，不由任意 foreign 缺配置推断 | `threads/orphan-filter.asm @1003d4efc`；`check_db_orphan_providers @10037c310` | core `threads/mod.rs:196` 与 `native_orphans.rs:19`；按主任务明确授权额外映射 Atelier 自己的 `atelier/codex_atelier/atelier_*`，不扩展到其他第三方。 |
| valid IDs 来自实际 state provider IDs、结构化 config section、恒有 openai/OpenAI；On 才加入 aimai1 与隔离项 | `diagnostic_valid_provider_ids @10039264c`，C `:78`、`:111`、`:147` | `native_orphans.rs:105` → core `diagnostic_valid_provider_ids_from_sources`；Gateway 意图使用真实 router-intent，不拿 runtime.running 代替。 |
| quarantine 只取 takeover section 的 model_providers 第一段，跳过空值与 AiMaMi 管理 ID | `quarantined_model_provider_ids.c @1001fd0f4:95` | `native_orphans.rs:145`；读错按原 valid-set 编排忽略该增量，未造空 Native state。 |
| DB orphan 与 rollout 首/末锚点 orphan 是独立候选并集 | `fix_orphan_thread_providers_with_catalog.c @10039e0c0:985`、`:1190`；rollout check `@1003a4ba8:280` | `native_orphans.rs:168`，实际主线程私有快照，保留 raw provider 拼写。 |
| 同历史的外部 DB/首末/native-settings 归属拒接；多个 thread ID 引用同一历史也停止 | 主 fix C `:1413`、`:1422`、`:1428`、`:4703` | 同物理 plain/zstd 表示统一分组；健康 foreign 独立历史不进入修改集合。 |
| 修复目标为官方 openai，而不是当前 profile/provider | 主 fix 的 adoption/SQL/verify 分支与 `:4328` 两检查后验 | `native_orphans.rs:615`；仅选中组更新 DB 与历史。 |
| 官方池必须真实获取并验证来源，NoAccount On 拒绝 | `PreparedRepairCatalog::prepare @100793238:78`、`:120`；`validate_source @100792a50` | `native_orphans/catalog.rs:132`、`:183`。Gateway raw inventory 不作为官方池。 |
| 默认官方 source：有效 config object，无 model_catalog_json 覆盖，openai 默认/指定官方 base URL；Native aimai1 须 On 非 NoAccount、loopback/path/port、requires_openai_auth、revision header | validate_source C `:45`、`:90`、`:110`；短 URL closure `@100792ec0` | 保留 C 真实 top-level openai_base_url 判断；没有增加 C 不存在的 nested openai 子表拒绝规则。Native source 用实际 RelayState 与 revision。 |
| RPC 固定 app-server --stdio，CODEX_HOME 明确；初始化后使用 JSON RPC | `AppServerClient::connect @10077a610:154`、`:530`、`:679` | catalog `:36`、`:50`；无 shell 拼接或任意命令 IPC。 |
| config/read(includeLayers=true) → model/list(includeHidden=true,limit=100,cursor) ≤20 页 → 再读 config，前后实际 config 与来源一致；20 秒总预算，每请求最多10秒 | `read_model_list @1004371d0:297`、`:375`、`:419`、`:462`、`:862`；closure `@100438304` | catalog `:183`。重复 cursor / 20页后仍有 next / config 改变均错误。cursor 每轮 take，所以合法末页不会残留上一页游标。 |
| 排除 hidden、aimami_relay_*、含 :: 的模型；isDefault priority0，其余按原顺序+1 | Map 实体 `repair_catalog_models_try_fold.c @100510110:103`、`:290`；native catalog `@100371908:120` | catalog `:231`、`:274`。使用真实 pool 的第一个默认项，否则第一个可用项。 |
| 优先保留目录内候选模型，否则实际默认模型；effort 必须得到目录支持，否则目录默认，仍不可确认则失败 | `orphan_model_candidates_try_fold @10050efb4`；`orphan_effort_candidates_try_fold @10050e110`；主 fix C `:4750`、`:4852`、`:4872`、`:5103` | `native_orphans.rs:417`。原字符串参与 membership，不把任意 padded model 自动 trim 成已确认值。 |
| 原生 adoption 修改首/末 session_meta，不修改 native-settings 事件；effort 键是 model_reasoning_effort | `apply_thread_metadata_rollout_adoption @1008ca4d0:77`、`:92`；`patch_session_meta_line @1008b472c:87`、`:295` | 对应 build metadata patches；其余会话正文原字节保留，`.zst` 保持压缩表示，保护记录使用 core fit_record。 |
| 原生有恢复文件、rollout patch、DB 事务、rollback 与两检查最终复验 | 主 fix C `:2314`、`:2915`、`:3119`、`:3410`、`:3834`、`:4328` | `apply_verified :698`：实际备份/私有恢复记录、DB/WAL CAS、具体事务更新、读回、失败回滚和新 select 后验；不会仅凭 status/复制成功返回成功。 |

以上 C 名称均在 `A/codex-direct/decompiled/`，原始 wrapper 在 `A/pseudocode/decomp-fix_orphan_thread_providers.c @10038e1d8`。`orphan_provider_fix_ids @10037a174` **只返回两项诊断 ID，不是线程候选集合**，不能误用。

## 生产连接和 all 批次

- `inspect.rs:1095` 调用新 adapter 覆盖两项；`repair.rs:235` 具体准备计划。旧的按 `t.rows` 全量对齐当前 Provider 分支已经删除。
- `Ticket::orphan_catalog` 向 Manager 复用已实际准备且绑定 config/auth/state 原始依赖的 opaque OfficialCatalog，避免第二次启动 app-server 或错用普通 Router catalog。
- `repair.rs:589` 首次写入前验证完整原始线程依赖与孤儿计划；`:628` 执行孤儿计划，`:639` 调 `native_threads::apply_sequence_after_approved_prefix`。临时“同 DB 组合直接拒绝”已移除。
- opaque `ApprovedEffects` 只能由真实执行器签发；getter 只返回原 before → 经后验 after 的 length/SHA256/mtime。后续线程 API 还核原 before 属于本次计划、当前等于批准 after、其他依赖保持原值，fresh scope 不超出原审核全集。
- DB 物理字节因 SQLite/WAL/checkpoint 改变不能直接取任意“执行后当前 hash”当批准效果。计划在私有副本计算完整 schema/表数据的 SQL 效果投影；执行后、关闭写连接后再次读稳定私有快照验证投影，才签物理 after stamp。失败且确认回滚也只签回滚后的已证明效果，并保留 `failure`，不会声称修复成功。
- 私有投影为计算审核范围，临时删除/恢复该私有副本上的 thread triggers；真实源库 trigger 始终保留并实际执行。触发器拒绝导致真实失败/回滚，既有 all 失败用例仍报告 failed IDs、保存备份，并保持原始历史字节。

## API 与验证边界

`native_orphans.rs` 公开到 diagnostics 的接口是 `inspect/select`、`prepare`、`prepare_with_catalog`、`prepare_catalog_with_rpc`、`validate_plan`、`preview_lines`、`apply_verified`、`ApprovedEffects::effect`。OfficialCatalog 的内部字段不对兄弟模块开放；必须从实际 source guard + model/list 或显式 FakeRpc 得到。

开发测试默认 `#[cfg(test)] StdioRpc::connect` 直接错误，发现 bundle 和启动实际 CLI 的代码仅非 test 路径可达。没有读取/操作真实 Codex、auth、配置、数据库、钥匙串或会话；所有文件都是临时合成 fixture，官方池通过 FakeRpc，不做线上请求。

实际执行：

- `cargo test --offline --manifest-path src-tauri/Cargo.toml native_orphans --bin codex-atelier`：19 项通过，包含混合健康 foreign/openai/有效 Atelier/孤儿、DB/rollout 独立并集、压缩别名混合归属、源列缺失/子线程、依赖外写、真实临时 DB 接管、模型保留/回落、effort、trigger失败回滚、2页/20页/重复游标与 config变化。
- `cargo test --offline --manifest-path src-tauri/Cargo.toml maintenance::diagnostics::tests --bin codex-atelier`：49 项通过；原四个 all 失败回归保留行为断言，仅改为显式 FakeRpc 官方池预览。
- `cargo check --offline --manifest-path src-tauri/Cargo.toml --bin codex-atelier`：exit0；全工程有既有和未用接口 warnings，不声称零 warning。
- owned tracked 文件 `git diff --check`、新增三个 Rust 文件 no-index whitespace 检查无输出。

## 仍未等同于全量维护完成

真实 Desktop / 捆绑 CLI E2E、真实官方认证和 model/list 请求未执行，遵守用户验收边界。全量 35 项仍有 freshness、正常模式写探针、project/history/native-off 等审计缺口。

本实现使用 Atelier 票据目录下的 `adoption-recovery.json` 和完整备份表达已审核效果，不伪造安装版私有恢复文件的兼容 schema；安装版 `thread-provider-adoptions` 恢复记录的跨重启自动恢复格式/消费者及极大历史的原生分页写策略，还须逐项恢复并接入，当前失败时保留具体记录和错误。不得把本次 P1 健康 Provider 保护与 all 组合回归通过，转述为所有原生错误分支和持久恢复语义都已等价。
