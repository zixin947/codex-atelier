# AiMaMi 线程诊断直接恢复

Rust crate（复用同一 codex-direct/config 中的已恢复 lexical scanner）。离线输入 SHA-256：`b2a4b1b2b4babd2518e02c0f0d6203ad19e779909a47b3b643844ef403fe95de`。仅接收调用方显式传入的数据库、rollout、备份根路径和有效 provider ID 快照；没有读取当前用户目录的默认值。没有启动 AiMaMi/Codex、读取真实数据库/认证文件、修改 Atelier 或既有 recovered-maintenance。

## 已恢复

- `collect_diagnostic_main_threads`：只读 `0x8001` 打开每个 DB；3 秒 busy timeout 失败忽略；通过 `PRAGMA table_info(threads)` 探测 `model`/`reasoning_effort` 两可选列，SQL 不裁剪 provider，空白 provider 变 None，过滤包含 `"subagent"` 的 source。任何 DB/行错误令整个收集失败。
- `check_db_orphan_providers`：只认 `aimami_relay`、`aimami_relay_` 前缀、`aimami_codex_router`、`aimai1`，且必须不在传入有效集合。按 thread ID 跨 DB 去重，示例排序去重取前 5；非 AiMaMi provider 缺失不会报错。
- `collect_provider_convergence_targets`：独立 SQL，仅含 `model` 可选列；过滤 subagent 和外部 provider，额外接受 ASCII 大小写无关的 `openai`；空白 model 变 None，非空不裁剪。
- `resolve_rollout`：`.jsonl.zst` 先转换普通 `.jsonl`，普通表示优先，其后尝试 `.jsonl.zst`；只遇 NotFound 继续，其他 IO 错误和非普通文件立即错误。这里不解析 rollout 内容。
- `rollout_file_belongs_to_thread`：使用真实 plain/zstd reader，跳过无效 UTF8/JSON，读取第一条有效 `type=session_meta` 并严格比较 `/payload/id`；第一条 meta 不匹配不会继续寻找后面的匹配。
- `find_trusted_rollout_backup`：递归目录栈，忽略目录/entry/file_type 错误；不遍历符号链接，候选必须为与 DB 路径完全同名的普通文件并通过上述归属校验。
- `inspect_missing_rollouts_for_convergence`：真实 DB/FS 检查及计数。DB metadata 失败跳过；DB 打开/查询错误累积；rollout path=None 直接计不可恢复；文件不存在再查可信备份；rollout IO 错误单列 errors、不计 missing；示例最多 3 条。
- `check_thread_missing_rollouts` 和 `check_project_state_integrity`：恢复原诊断返回分支、文案、优先级、fixable。

新增 provider/model 一致性、cache、索引、真实 anchors 的入口和完整证据见 [CONSISTENCY.md](CONSISTENCY.md)。

## 当前未恢复边界

- `diagnostic_valid_provider_ids` 的集合编排已恢复，并用 `diagnostic_valid_provider_ids_from_sources` 接入真实 config 读取及 config crate 的原 lexical scanner。隔离 provider IDs 的文件生产逻辑属于 config_takeover 子系统，当前明确要求调用方传入其 Result 快照；失败仅忽略，不伪造数据。
- `codex_project_state::inspect` 已恢复为 `inspect_project_state`：JSON读取/规范化/计数/决策、真实 SQLite roots/hints/active_threads、真实 rollout CWD backfill（校验 session id、排除 projectless IDs/subagent/生成项目目录）均可执行。active DB 路径作为显式输入；全局路径发现由调用者提供。`inspect_project_state_with_db` 另保留类型化快照边界供组合调用。
- 路径发现 `CodexPaths::all_codex_state_dbs` 不在本分工恢复；调用者必须提供候选列表。该设计明确暴露范围，不会暗中扫描真实用户目录。
- 静态证据足以确定上述分支；测试验证移植行为，未执行原二进制作差分比较。rusqlite/zstd 依赖版本可能使底层错误文本与原构建不同；业务前缀保留。

## 源码证据

| 恢复行为 | 原证据 |
|---|---|
| 主线程 SQL、可选列、过滤、错误 | `../../pseudocode/decomp-collect_diagnostic_main_threads.c`：行 188–335（打开/列查询）、行 443–525（可选列和 SQL）、行 698–795（过滤和空白处理），入口 `0x100393ea4` |
| orphan 分支与 ID 去重、前5例 | `../../pseudocode/decomp-check_db_orphan_providers.c`：行 81–185、200–385，入口 `0x10037c310`；管理范围判定见本目录 `orphan-filter.asm` `0x1003d4efc..0x1003d50d4` |
| missing inspection 主循环 | `../../pseudocode/decomp-inspect_missing_rollouts_for_convergence.c`：行 121–321、388–398，入口 `0x1008d6c44` |
| convergence source/provider/model 过滤 | `convergence-filter.asm` `0x10050e3b0..0x10050e518`；`convergence-provider.asm` `0x1008b799c..0x1008b7b50` |
| plain/zstd 候选顺序与 IO | `rollout-resolve.asm` `0x1008b01e8..0x1008b06c0` |
| backup 遍历/文件名/身份校验 | `trusted-backup.asm` `0x1008b62dc..0x1008b6798` |
| 第一条 meta 和严格身份 | `first-session-meta.asm` `0x1008b7540..0x1008b7998`；`rollout-thread-identity.asm` `0x1008ba190..0x1008ba2e8` |
| readonly 打开5秒超时并传播错误 | `open-readonly.asm` `0x1008b3938..0x1008b3b50` |
| 有效 provider 集合编排 | `../decompiled/diagnostic_valid_provider_ids.c` `0x10039264c`；config lexical scanner 复用 `../config/src/lib.rs` |
| 项目 DB/rollout cwd backfill | `../decompiled/collect_project_state_from_active_db.c` `0x1003b25a4`；`../decompiled/collect_cwd_backfill_candidates.c` `0x1003b1444`；`../decompiled/is_codex_generated_projectless_dir.c` `0x1003b2284` |
| 项目 JSON/决策 | `../decompiled/codex_project_state_inspect.c` 行 72–220、230–449；标准化 `../decompiled/normalize_codex_path.c` `0x1003b0db0`；数组过滤 `0x10053c31c` |
| missing wrapper | `../../pseudocode/decomp-check_thread_missing_rollouts.c`：行 38–111，入口 `0x1003921a4` |
| project wrapper | `../../pseudocode/decomp-check_project_state_integrity.c`：行 61–220，入口 `0x100391c44` |

`constants.txt` 为 Mach-O 映射读取的有界字符串片段；Rust 格式化字节码中的替代字符不是最终文案。`read_constants.py` 只读取固定离线副本的显式 VA。

## 验证

```
cargo fmt --manifest-path codex-direct/threads/Cargo.toml
cargo test --offline --manifest-path codex-direct/threads/Cargo.toml
```

所有测试仅在 `tempfile` 临时目录或内存 SQLite 中执行。目前 21 个合成测试全部通过（旧14 + consistency新增7）。覆盖归档/子线程排除、可选列、保留非空原文本、数据库只读与错误边界、外部 provider 不误报、跨 DB ID 去重、普通/压缩表示优先级、非文件错误、备份同名且身份可信、第一条 meta 不匹配、missing/recoverable/unrecoverable/error 独立计数。

特意保留原 SQL 的边界：缺 updated_at_ms/updated_at 时原排序表达式为 `ORDER BY 0 DESC`，SQLite 将其视为无效列索引，返回 DB warning；没有擅自改写为常量表达式。项目 DB 的 COUNT 查询失败单独吞为 0；主 cwd 查询失败则清空该 DB 快照并记录错误，回填查询失败单独吞为 0。
