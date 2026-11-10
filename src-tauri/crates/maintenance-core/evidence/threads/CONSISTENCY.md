# provider/model 一致性直接恢复

稳定模块：`src/consistency.rs`。新增 7 个合成测试，连同旧 14 个全部通过（21 个），`cargo fmt --check` 通过，无编译 warning。

## 可直接组合的入口

- `check_thread_provider_consistency_with_cache(dbs, router_enabled, &mut DiagnosticRolloutProviderCache)`：实际 SQLite → rollout anchors → cache → 原 observation 计数 → 诊断；目标只从路由状态选 `aimai1` / `openai`。
- `check_thread_model_catalog_consistency_from_sources(dbs, catalog_dir, router_enabled)`：实际读取 `catalog_dir/codex_router_catalog.json`、构造索引、读取 aimai1 活跃主线程及 rollout 身份/归属，产出诊断。关闭路由直接跳过，不读文件。
- 底层 `inspect_thread_provider_convergence_with_reader` / `inspect_router_thread_model_catalog` 返回类型化统计；旧 API 保留。

## 原判断/计数

provider 仅接管 managed/native 的 first/last/native anchors：任何一个非空 provider 为外部归属，整个 observation 跳过 DB/rollout drift 计数。大小写无关的 `OPENAI` 被视为 native 归属，但与目标 `openai` 做精确比较仍是不一致。每线程 DB 和 rollout 可各计 1，标题计两者之和。missing/path=None、corrupt、Other error 分开统计；这些分支不计 DB mismatch。仅 corrupt 且无其他问题返回 warning、不提供修复；其他 drift/missing/error 返回原 error/fixable。最多 3 例。

Cache 以原始 `PathBuf` 为键，缓存成功、无 anchor、Corrupt、Other，不通过 mtime 自动失效；同一 cache 重读保持快照。未缓存候选少于 8 不预取，达到 8 使用 4 个 worker。移植用 Rust scoped worker 替代原 Rayon pool，维持阈值/并发数和失败后 lazy read 的结果行为，不引入 Rayon 运行时依赖。

普通 JSONL 按原 1 MiB 反向块扫描尾部（支持跨块长行），选 first meta 同一 id 的最后 meta，只捕获它之后最新 native settings；zstd 重开 reader，跳过 first meta 结束偏移后正向流读，遇新匹配 meta 清空更早 native settings。无效 UTF8/JSON忽略。first meta 缺非空 id 为 Corrupt；first/last model_provider 允许缺失/null，空白/其他类型为 Other；存在 native settings 时必须有非空 provider。

model 检查只选 `model_provider = 'aimai1'` 的活跃主线程，排除 subagent；缺 model 列返回空。索引接受 catalog 精确 slug；正常账号模式允许所有非空、无 `::`、且不是 AiMaMi provider ID 的裸模型直通，不添加模型品牌 allowlist。其他情况先查首个 suffix alias，再回退首个 managed qualified slug；无 managed default 且 no-account 为真时回退首个 canonical slot。No-account flag 存在必须 bool；缺 flag 由非空 models 且每项均为精确 canonical slot 推导。

模型不一致仅在历史可处理时计入；missing rollout 不阻塞该项（另有缺历史检查），Corrupt 或外部 provider 历史跳过。DB id 与 first meta id 不匹配、最新 native settings.thread_id 显式指向其他任务均报告 Other 错误，保留历史。官方裸模型在检查历史之前直接 accepted。

## 静态证据

均为离线副本 `AiMaMi.arm64`，SHA-256 见 README；未启动原程序作动态差分。

| 恢复边界 | 证据文件（相对本目录）与入口 |
|---|---|
| provider wrapper/cache 构造 | `../../pseudocode/decomp-check_thread_provider_consistency_with_cache.c` `0x1003a5d7c` |
| cache get/miss/read/error 复用 | `../decompiled/diagnostic_rollout_provider_cache_read.c` `0x100392d28` |
| 真实候选路径收集 | `../decompiled/collect_existing_convergence_rollout_paths.c` `0x1008d947c` |
| provider inspect | `../decompiled/inspect_thread_provider_convergence_with_reader.c` `0x1008d9ed4` |
| observation 所有计数 | `../decompiled/apply_provider_convergence_observation.c` `0x1008c9d64` |
| anchors 首尾定位/解析 | `../decompiled/find_session_meta_anchors.c` `0x1008b56d0`；`find_last_matching_session_meta_line.c` `0x1008c8848`；`find_last_compressed_session_meta_line.c` `0x1008cabb0`；`matching_session_meta_from_bytes.c` `0x1008beaac`；`capture_latest_settings.c` `0x1008b43f4` |
| anchors provider 字段规则 | `../decompiled/session_meta_provider.c` `0x1008b342c`；`rollout_provider_anchors.c` `0x1008b53b4`；`read_rollout_provider_anchors.c` `0x1008b9b88` |
| model inspect/wrapper | `../../pseudocode/decomp-inspect_router_thread_model_catalog.c` `0x1008c4f60`；`decomp-check_thread_model_catalog_consistency.c` `0x10039d6e4` |
| catalog 索引/disposition | `../decompiled/load_router_catalog_model_index.c` `0x1008ba2ec`；`RouterCatalogModelIndex_disposition.c` `0x1008b3fdc` |
| no-account JSON推导 | 原始 ASM `0x10036cbd4..0x10036cd60` |
| aimai1 SQL target | `../decompiled/collect_router_model_targets.c` `0x1008b710c` |
| 历史可处理与身份 | `../decompiled/has_unwritable_router_model_rollout.c` `0x1008c4dd0`；`router_model_rollout_anchors.c` `0x1008b89d0`；`convergence_anchors_for_thread.c` `0x1008b9d60` |
| CoreError 文案前缀/Corrupt 分类 | 原始 ASM `0x1007fcd0c`、`0x1008b5bd0`：2=IO error、3=JSON error、9=Invalid data、10=Operation failed；仅3和9归Corrupt |

## 覆盖与精确限制

7 个新增测试覆盖：首个 suffix/default、官方裸模型与 no-account、field 类型错误、所有权跳过、大小写与精确 target 比较、first/last/native 选择和重置、真实 zstd、1 MiB 跨块长行、cache 7/8 阈值和快照复用、真实 DB drift 双计数、Corrupt-only warning、model unwritable/foreign/task identity、路由关闭不读 catalog。旧 14 个测试全部保留。

未执行原二进制动态差分；无法据合成测试声称原程序运行结果已观测。线程创建失败、并发改写/截断/替换 rollout 的竞态未做故障注入，底层依赖错误正文可能随 rusqlite/zstd/OS 版本改变。业务分支、分类、前缀和静态字符串已按上述证据移植。没有改写或修复 rollout/数据库；没有读取真实用户路径。
