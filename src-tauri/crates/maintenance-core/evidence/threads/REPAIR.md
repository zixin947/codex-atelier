# 线程原生修复恢复证据

恢复输入仅为离线 `maintenance-full/AiMaMi.arm64`，SHA256 `b2a4b1b2b4babd2518e02c0f0d6203ad19e779909a47b3b643844ef403fe95de`。下列 C 文件均位于该目录的 `codex-direct/decompiled/`；诊断 wrapper 位于 `pseudocode/`。没有运行原客户端，没有读取真实 config、auth、会话或钥匙串。测试只使用 `tempfile` 和合成 SQLite/JSONL/zstd。

## 已交付 API

`src/threads/repair.rs` 独立实现真实 SQLite 和文件变更，公开返回 `RepairError`，没有依赖调用者提供状态布尔值。

- `plan_missing_rollout_repair(dbs, trusted_backup_root, backup_directory)` → `MissingRolloutRepairPlan`；`apply_missing_rollout_repair(&plan)` 恢复有身份凭据的备份或归档无备份线程，最后重新运行真实缺失检查。
- `plan_provider_convergence(dbs, ProviderTarget::{Official, Router}, backup_directory)` → `ThreadWriteRepairPlan`；目标分别为 `openai`/`aimai1`。
- `plan_model_catalog_convergence(dbs, catalog_directory, backup_directory)` → 同一类型，实际读取 `codex_router_catalog.json`、构建 catalog 索引并计算替代模型。
- `apply_thread_write_repair(&plan)` 执行相应 SQL 和真实文件写入；provider 失败回退当前数据库的已修改历史；model 单个 rollout 失败记录 warning，继续提交数据库模型。SQL/commit 失败回退本次已改历史。
- `fit_record` 恢复原生分页/ordinal 字节布局保护；`load_model_restore_journal` 恢复 v1/v2 journal 验证（均要求明确的 borrowed_model 键）。

模块由集成方在 `threads/mod.rs` 声明 `pub mod repair;`；本任务没有修改该文件、Cargo、共享类型或应用入口。集成测试通过 path module 编译新文件。

## 原生来源与覆盖分支

| 行为 | 原生证据 |
|---|---|
| provider 活跃主线程 SQL、受管/官方归属、排除 subagent | 已恢复 `threads::collect_provider_convergence_targets`；原函数 `0x1008c8424` 与 predicate `0x10050e2f4` |
| 缺失引用先要求修复历史，不能直接改 DB provider | `converge_threads_to_provider_with_reads.c:575-599`；`require_existing_rollout@0x1008b51a0` ASM 的 None/missing 两个 error 分支，机器前缀 `ROUTER_HISTORY_REPAIR_REQUIRED` |
| provider first/last session_meta、latest native settings、外部归属保留 | `patch_convergence_rollout_target@0x1008beda0`，C:310-383、后续 native settings 分支 |
| provider SQLite 写入、失败历史回退、提交后 checkpoint | `converge_threads_to_provider_with_reads@0x1008cbe30`，C:1225、1242、1283、1373-1380、1531、1569 |
| 返回官方时收集 instruction 修复目标，并非删除 provenance | `InstructionRepairTarget_from_meta@0x1008b3b98`；`patch_convergence_rollout_closure@0x1008b5b54` 仅更新目标摘要；`reconcile_threads_with_reads.c:245-270` 的指令/推理清理受独立选项控制，provider diagnostic wrapper 传两个 false |
| catalog 归属/模型判定与空模型列处理 | 复用 `consistency::load_router_catalog_model_index`、`collect_router_model_targets`、`ModelDisposition`，未使用自造模型 allowlist |
| model 只改最后匹配 meta 的 model，不改 first/provider/其他字段 | `converge_router_thread_models_with_reads@0x1008d2e0c`，C:868-1015 |
| model rollout patch 失败但 DB 更新提交 | 同上 C:1078-1107；SQL `UPDATE threads SET model = ?2 WHERE id = ?1` C:684 |
| model SQL/commit 失败恢复已修改的历史 | 同上 C:709、1237 的 `with_first_session_meta_rollout_rollback` |
| 无账号模式在 DB 事务之前写原模型 journal | 同上 ASM `0x1008d35c4` 检查 no_account byte，`0x1008d360c` load，C:1275-1410 merge/write；不存在时默认 version=2 |
| journal JSON 字段 thread_id/original_model/borrowed_model，borrowed_model 为明确键 | `NoAccountModelRestoreEntry_serialize@0x1008b2e90` C:33、60、65；`NoAccountModelRestoreEntry_deserialize@0x1008f1a78` 证明 original_model 为 nullable string；仅支持 serialize 明确的 borrowed_model 字段，未假设相邻 general ModelRestoreEntry 的 fallback_model 同时为此类型的 alias |
| journal v1/v2、8 MiB、非普通文件/重复/空 ID、borrowed 模型校验 | `load_no_account_model_restore_journal@0x1008c94c4` C:65-72、96、138-178、228-245；relay slug `0x100367e50..0x100367f34` ASM，复用 catalog provider ID validator |
| journal private atomic publication、写入大小限制 | `write_no_account_model_restore_journal@0x1008cb97c` C:95-147；0600、fsync file+parent directory |
| 分页历史不能增长已有记录，缩短用空格补齐原字节数 | `fit_record@0x1008af6b4` / `pad_record@0x1008af874`：top-level ordinal 存在（含 null），history_mode=paginated，或非 null history_base |
| plain 优先、压缩表示保持、变更前校验 | `rollout_history_rewrite@0x1008b079c`，复用 `resolve_rollout`；本模块对比预览时的完整原始物理字节，变更后仍写原物理表示 |
| 缺失恢复 exact filename + first-meta ID；source 表示变更拒绝；same-dir stage/sync/rename | `repair_missing_rollouts_for_convergence@0x1008cef90` C:412-427、545、577-651、707；`find_trusted_rollout_backup@0x1008b62dc` |
| 不可恢复归档、事前备份、checkpoint、审计失败仅 warning、最终复检 | 同上 C:1353-1477、1920、1979+、2565-2722；SQL `UPDATE threads SET archived = 1 WHERE id = ?1 AND COALESCE(archived, 0) = 0` |

## 适配边界

- 接口全部显式传路径，原生 `CodexPaths` 的隐式位置查找由集成方提供；模块没有读取用户 home。
- plan/apply 是 Atelier 的可审阅适配层，包含原始 DB 字段与文件字节比较。原生没有这一公开 JSON 计划协议。
- 原生 provider 可并发 patch；这里串行执行，DB 事务按数据库分组。已经提交的前一个数据库不因后一个数据库失败撤销。
- 原生流式 rewrite；计划保存完整物理字节，压缩文件完整解压再重编码（zstd level 3），语义内容/原物理表示保持，压缩字节不要求相同。
- 增加显式备份目录中的 DB 快照和 rollout 副本；DB 使用 `VACUUM INTO` 包含已提交 WAL 内容。原生 missing archive 使用主 DB 文件复制；此适配避免只复制开放主文件导致遗漏 WAL。
- 原生的 mtime 恢复为 best effort；这里相同。错误 machine prefix 保留，部分路径/SQLite 包装信息为 Rust 适配器文本，不声称所有人类可读字符串逐字一致。
- 本模块提供底层 convergence producer。应用层应在执行后运行已有 provider/model 真实诊断，按原 wrapper 决定最终成功报告。缺失 rollout apply 自带最终复检。
- `load_model_restore_journal` 校验 symlink_metadata；对抗恶意并发替换 journal 的 O_NOFOLLOW 平台细节尚未逐条移植。
- 已追加 poisoned instruction source、真实文件修复和迁移快照清理，见下节。private reasoning cleanup 是另一独立选项，不由本诊断修复入口冒充执行。
- Journal 是否还接受 `fallback_model` alias 为单列待核项。当前只支持有明确 serializer 证据的 `borrowed_model`，没有扩展输入接受面。

## 验证

实际执行：`cargo test --manifest-path src-tauri/crates/maintenance-core/Cargo.toml --test threads_native_repair`，30 passed。

夹具覆盖 provider 首/中/尾记录选择、native settings、外部 provider、身份不符、压缩物理表示、预览过期、真实 SQLite trigger 失败回退、model passthrough/最后 meta/分页增长失败后数据库提交/单文件失败警告、journal chain/版本/重复 ID/限额/路径类型/0600，以及 missing restore/archive/外部线程保留/备份身份再次检查。

## 后续交付：response item IDs

新增 `plan_response_item_id_repair(dbs, backup_directory)` / `apply_response_item_id_repair(&plan)`，以及可独立复用的 `canonical_response_item_id`、`rewrite_response_item_record`、`count_invalid_response_item_ids`。实际读取活跃主线程（所有 provider），按记录路径去重，preflight 错误阻止执行。DB 不变。

- `canonical_id_for_item@0x1008db528`：仅 `tool_search_call` 的 `ts_` + 12 个小写 hex → `tsc_`；`custom_tool_call` 的非空 ASCII `[A-Za-z0-9_-]` 后缀 `ct_`/`fc_` → `ctc_`。`function_call`、已 canonical ID、错误长度/字符均不变。字符串常量 `0x1013b6297=ctc_`、`0x1013b839c=tsc_`。
- `is_candidate_response_id_bytes@0x1008daf28`：原始 bytes 仅扫描 `tool_search_call`+`ts_` 或 `custom_tool_call`+(`ct_`/`fc_`)。候选格式错误报错，其他 malformed JSON 原字节保留。
- `rewrite_rollout_record_bytes@0x1008dc290` C:240-344：只替换原始记录中唯一的已验证 JSON string literal。不存在/出现多次报错；避免整行 JSON serialize 改掉其他字节。CRLF/LF/无换行保持。
- `rollout_history::has_positions@0x1008afc44` ASM：全局位置保护只看**第一条 session_meta**，不是所有记录；三种保护条件同 `fit_record`。`repair_invalid_ids_in_rollout.c:1294-1305` 和 compressed closure C:275-315 再要求修改后的整条记录长度不变。
- `repair_invalid_ids_in_rollout@0x1008dd29c`：原始物理 bytes + mtime 比较；SHA256 使用 `sha2`，真实私有 JSON 备份先发布，之后原子替换历史、重算剩余 ID。多文件逐个尝试，部分失败返回明确包含已修改数量和备份路径的错误。
- `response_id_backup_to_vec_pretty@0x10087b15c` C:46-86 确定字段 `schemaVersion=1/sourcePath/sourceLength/sourceSha256/replacements`；`RolloutRepairReplacement_serialize@0x1008dafe8` C:33-65 确定 `line/beforeId/afterId`，没有凭空附加 type。`allocate_rollout_repair_backup_path@0x1008db2ac` 确定 stem + unique token + `.response-item-id-repair.json`。适配使用纳秒+原子计数 token 替代原生 UUID，文件名唯一性协议并非逐字复制。
- plain/zstd 保持原物理表示。压缩文件恢复 mtime 为 best effort；plain 原子替换的时间戳允许更新，与对应 native 路径一致。

新增 6 个真实场景：边界映射、原字节/CRLF/重复字面量拒绝、SHA256 原生 schema 备份与 DB 不变、压缩和 first-meta 全局位置约束、ordinal padding/Unicode escape 非候选、预览后变更拒绝。

## 后续交付：legacy strict cleanup

新增 `plan_legacy_provider_cleanup(dbs, backup_directory)` / `apply_legacy_provider_cleanup(&plan)`，以及 `is_legacy_or_miscased_provider`、`inspect_legacy_thread_visibility`。

- predicate `0x1008ba02c..0x1008ba18c` ASM：exact `aimai1` / `aimami_codex_router` 或 ASCII case-insensitive openai 且不是 exact `openai`。**不包括 modern aimami_relay_*。**
- `cleanup_orphan_router_threads_strict@0x1008c6b80` C:313-321：`PRAGMA wal_checkpoint(FULL);`，然后 `SELECT DISTINCT rollout_path FROM threads WHERE rollout_path IS NOT NULL AND rollout_path <> '' AND COALESCE(archived, 0) = 0`。该 SQL 不排除 subagent、也不按 provider 过滤。
- 同上 C:490-593：first/last meta 中分别符合 legacy predicate 的 provider 改为 `openai`，中间 meta / native settings 不改；mixed 外部归属也逐 anchor 判定，不套用普通 provider convergence 的整体归属过滤。缺失历史跳过，读取/解析错误保留错误。
- 同上 C:818-842：真实单条 autocommit `UPDATE threads SET model_provider = ?1 WHERE model_provider = ?2 OR model_provider = ?3 OR (model_provider LIKE 'openai' AND model_provider <> ?1)`，参数 openai / aimai1 / aimami_codex_router。**此 UPDATE 无 archived/source 条件，归档和 subagent DB 行也归一。** TRUNCATE checkpoint 后按原 COUNT SQL 复检。
- 同上 C:967-1020：SQL update 失败或 count 非零，回退已修改历史。历史 patch 阶段发生错误时原生不会回退此前成功历史；这里相同，并保存显式备份，不声称跨文件全原子。
- `inspect_router_thread_visibility@0x1008bc6d4` C:188、324、433-477、534：legacy COUNT 包括所有 DB 行；第二计数只看 active exact-openai 并排除 source 中的 literal `"subagent"`。原生将打开/查询错误写日志并继续，适配另外返回 `errors` 便于 UI 明确呈现，不能只凭计数 0 掩盖读取失败。

新增 4 个真实场景：全表 DB 与活跃历史边界区别、mixed 外部 anchor、真实 SQLite trigger 失败回退/缺失路径跳过、legacy predicate 精确边界。

## 后续交付：poisoned instructions 与真实模板来源

新增 `plan_poisoned_instruction_repair(dbs, relay_directory)` / `apply_poisoned_instruction_repair(&plan)`；返回的 `InstructionRepairSummary` 包含实际写入、snapshot string 更新数、已知剩余问题数、warnings 和是否实际写入 minimal fallback。`find_reference_instructions` 返回所选 path/物理表示/行号/完整文本来源凭据，调用者不能传一个模板或“健康”布尔值代替读取。

- `collect_active_rollout_paths@0x1008b6900`（C:220-288 与 ASM `0x1008b6a14/0x1008b6b6c/0x1008b6de0..0x1008b6e48`）：read-only SQLite，busy_timeout=5000ms；准确 SQL 是 `SELECT rollout_path FROM threads WHERE archived = 0 AND rollout_path != ''`，**与诊断主线程 collector 不同**，不限制 provider/subagent，也不把 NULL archived 当作 0。按输入数据库顺序、SQLite 原返回顺序收集，精确路径字符串去重保留首次，DB/row 错误忽略。
- `find_reference_instructions@0x1008b5cb8` C:65、110-176、210-232：最多 50 个返回路径，每路径最多前 5 行；允许 plain/zstd。候选必须是 type=session_meta 的 `/payload/base_instructions/text` string，原 UTF-8 字节数严格 >1000，trim_start 后不以 `You are judging one planned coding-agent action`（47 bytes）开头。首个符合者原文本整体复用，**没有按 model/project 分组**。调用者的“首个健康提示”推断不足以替代这些阈值和范围。
- `session_meta_instruction_issue@0x100393c6c..0x100393ea4` ASM：invalid JSON / 非 session_meta → Healthy；空/缺失/非 string text 经 trim 为空 → Missing；trim_start 后前缀匹配 → Poisoned；其余 Healthy。短 healthy 文本不需要修复，但不能作为完整来源模板。
- `fix_thread_poisoned_instructions` 原 wrapper C:279-455：find_reference 返回 Option<String>；随后 BTreeMap 只是在构建 JSON `{ "text": reference }`，**不是模板分组缓存**。不存在来源时逐字 minimal fallback 为 `Follow the user's instructions carefully. Respond helpfully and use tools when appropriate.`。
- wrapper C:525-586 + `rewrite_session_meta_base_instructions@0x10039da78` ASM：主线程按路径去重，仅 FIRST session_meta；整个 base_instructions 对象替换为 text 对象（旧 provenance 不保留），其他字段/后续记录保持。先把原首 meta 行写入 `relay_directory/guardian-prompt-fix-backup/{thread_id}.jsonl.bak`，不是把整个压缩文件误写为 jsonl.bak。
- `replace_first_session_meta_line@0x1008bb26c..0x1008bb588` ASM：发布前重新读首 meta 并比较，`0x1008bb480` 调用 fit_record，`0x1008bb4e4` 调用原子 rewrite。分页增长失败保留历史；本模块另外保留 staged bytes 比较，zstd 仍是 zstd。
- wrapper C:813-825、1010-1169：清理 `relay_directory/thread-router-migrations/current.json` 和 `history/*.json`。仅遍历 top-level entries 数组中 object 的 string values；string 自身为待修 session_meta 时替换该字符串。JSON pretty serialize 后原子写回；该路径允许字符串长度增长，不对 snapshot JSON 字符串套用 rollout 字节偏移限制。适配增加一份 snapshot 原文件备份。
- 各文件失败继续其他候选；apply 后重新构建真实计划以复检。已知 protected issue 计入 remaining_issues；读取失败通过 warnings 保留，不把 unknown 计数伪装为健康。

新增 5 个合成场景：issue type/prefix、SQL 原顺序/NULL archived/路径去重、五行/50 路径/1000 UTF-8 字节严格边界、压缩 first-meta 与迁移快照真实清理/备份、minimal fallback 与 protected history 未解决问题。当前 30 tests passed。

最终集成快照实际执行：`cargo test --manifest-path src-tauri/crates/maintenance-core/Cargo.toml --quiet`，190 tests passed（73+39+19+2+6+7+14+30），0 failed；auth/process 的 path-module 测试存在 unrelated dead_code warnings。独占 Rust 文件 `rustfmt --check` 通过；对三个新文件逐一 `git diff --no-index --check /dev/null <file>` 没有空白错误输出（exit 1 为存在新增内容的正常 diff 状态）。没有执行真实客户端 E2E。


## Atelier 应用适配与最终复检

新增 `src/maintenance/diagnostics/native_threads.rs`，六组检查/修复使用显式 `Context` 路径；不调用 `Context::native`、真实客户端或隐式用户目录。公开给 Coordinator 的接口为 `handles`、`prepare`、`prepare_with_backup`、`prepare_sequence`、`validate_sequence`、`apply_sequence`、`inspect`、`preview_lines` 与 `PreparedAction::id`。计划体私有且不实现 Serialize/Debug，预览仅返回目标计数和行为，不暴露会话正文或指令模板。

- `snapshot::with_snapshot_path` 在现有主文件/WAL 稳定复制校验上暴露回调内有效的临时 DB 路径。producer 与执行后诊断均只打开副本；源 SQLite 不由预览创建 SHM。
- 依 Atelier 已有 `inspect::resolve` 契约，相对 rollout 路径仅在私有数据库副本投影为 `ctx.home` 下绝对路径，且移除副本上的 path update triggers。每行原始路径另存，计划的 DB CAS expected 值映回原字符串，实际 IO 使用独立绝对路径；不改 cwd，也不把真实 DB 路径持久改为绝对路径。这是 Atelier 兼容适配，不声称 AiMaMi 原生采用相同 home-relative 规则。真实源库 triggers 保留，实际 SQL 错误仍触发真实失败处理。
- trusted missing rollout backup root 为 `ctx.home/codexmate` 全树。证据：`repair_missing_rollouts_for_convergence.c:265-266` 从 `CodexPaths +0x158/+0x160` 取值并于 `:415` 传给 finder；`CodexPaths_from_home.c:116-120` 构造 home/codexmate，末尾 `param_1[0x2b]/[0x2c]` 写入该值。不能只扫描 Atelier 自己的 backups，亦不能随意 relink sessions 同名候选。
- catalog 从显式配置路径或 relay 默认位置复制成私有 native alias。`plan_model_catalog_convergence_with_journal_directory` 将 actual catalog directory 与 actual `ctx.home/codexmate/relay` journal directory 分离；自定义 catalog 不移动 journal。所有修复备份统一写入本次 Ticket 唯一目录，poisoned snapshots 亦包括在内。
- 所有原始 DB/WAL/文件依赖先验证；组合动作只在已批准 ID 和 DB/row/file 目标集合内 reprepare，missing 先于依赖 provider。新目标要求重新预览，前序已完成变更和备份不谎称全部回滚。执行结果必须通过实际 provider/model/history producer 的最终快照复检；不能由 changed > 0 判成功。
- 缺失 `source` 列的旧测试 schema 已补成现代真实 schema，生产未知 schema 仍报告 warning，不猜测其线程为主线程。普通 provider convergence 按原生历史归属，不能强制改为当前配置中的任意 provider；外部 provider 历史保留。

### 旧版诊断 wrapper 与严格清理的区别

`check_legacy_migrated_threads@0x100391470`（`pseudocode/decomp-check_legacy_migrated_threads.c:94-104,323`）路由开启时排除合法 `aimai1`，仅检查 `aimami_codex_router` 和 miscased openai；路由关闭时加入 `aimai1`，仅此时提供修复。常量 `0x10130ee5d` 的准确查询另限制 active 且 source 不含 literal `"subagent"`。新增 `inspect_legacy_migrated_threads` 复现此 wrapper 的 unique-ID 计数，不能拿 strict cleanup 的全表 residual count 替代诊断。实际 strict cleanup 的全表 UPDATE 行为保持前节证据不变。

### 本次实际验证

- `cargo test --manifest-path src-tauri/Cargo.toml native_threads_tests --no-default-features --quiet`：15 passed。覆盖 committed WAL 与无源 SHM、relative plain/zstd、native可信备份恢复、独立 catalog/journal 路径、DB/历史/WAL 过期、missing/provider组合、真实SQL trigger后序失败与部分结果、外部历史保留、指令预览脱敏与Ticket备份、response真实ID/备份、路由开关legacy边界、新snapshot目标拒绝、私有projection trigger隔离。
- `cargo test --manifest-path src-tauri/Cargo.toml diagnostics::snapshot --no-default-features --quiet`：3 passed。
- `cargo test --manifest-path src-tauri/crates/maintenance-core/Cargo.toml --test threads_native_repair --test threads_diagnostics --test threads_consistency --quiet`：31 + 14 + 7 = 52 passed。
- 更早一次全部 diagnostics：168 passed / 1 failed，全部线程旧案例通过；唯一失败是独立 takeover TOML fixture，由 root 负责。此记录不代表之后并行改动的最终整套结果。
- 全部测试仅合成临时 SQLite/plain/zstd/目录，未运行真实客户端 E2E。Journal `fallback_model` alias 仍为上节明确待核项。
