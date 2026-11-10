# Codex 项目状态原生迁移

只读 AiMaMi arm64 输入 SHA-256：`b2a4b1b2b4babd2518e02c0f0d6203ad19e779909a47b3b643844ef403fe95de`。所有执行测试使用临时 Context、SQLite、plain/zstd 历史；没有读取或操作真实客户端。

## 原生证据

| 行为 | 原生 VA | 本目录文件 |
| --- | --- | --- |
| 检查 / 修复外层 | `0x100391c44` / `0x10038e4b8` | `check_project_state_integrity.c`, `fix_project_state_integrity.c` |
| inspect / repair_if_needed / repair | `0x1003b5b34` / `0x1003b0528` / `0x1003b357c` | `codex_project_state_inspect.c`, `project_state_repair_if_needed.c`, `project_state_repair.c` |
| 单个活动 DB | `0x1003b028c` / resolver `0x1008eeff8` | `active_db_path.c`, `resolve_codex_state_db.c` |
| 候选目录 / 评分 / 比较 | `0x1008ee7b8` / `0x1008ebccc` / `0x1008f349c` | `state_db_search_dirs.c`, `state_db_score.c`, `project_active_db_compare.c` |
| roots/hints payload / cwd 回填 | `0x1003b25a4` / `0x1003b1444` | `collect_project_state_from_active_db.c`, `collect_cwd_backfill_candidates.c` |
| 数组字符串收集 | 导出头部 VA | `project_string_array_collect.c` |
| 纯路径归一 / merge_unique | `0x1003b0db0` / `0x1003aff10` | 原始 symbol-map 与 ASM 核对；核心模块同名函数注释 |

Native 输入只有一个活动数据库，不把全部历史 DB 合并为新架构。显式 sqlite_home（配置优先、环境回退，相对 cwd）选单目录；否则目录顺序为 home/sqlite（preferred=true）、home（false），每目录取数值最高 state_N.sqlite。多候选按真实评分比较：两者 SQL probe 成功时先版本、MAX updated 时间（秒饱和转毫秒）、recency_at_ms/updated_at_ms 字段等级、活跃计数、DB/WAL 最大 mtime 纳秒、目录 preference；其余分支按原生 probe 分类、mtime 与版本处理。评分只在私有副本打开 SQLite，源库不会创建 SHM。语义 SQLite probe 75ms busy timeout，指定瞬时错误最多三次、间隔 25ms。

## 真实检查与合成规则

- 检查复用已恢复 `threads::inspect_project_state_with_db` 和 `check_project_state_integrity`；读取失败优先、DB 错误其次，都不能修复。缺文件且 DB 可重建 roots 会报错并可重建；缺文件又没有项目 cwd 则健康且不创建空文件。有效 UTF8 的损坏 JSON/非对象可由修复叶子重建，不丢失原始备份。
- active_threads 统计所有未归档行。已有 cwd 的 roots/hints 不按 source 或 projectless-thread-ids 排除；生成的 Documents/Codex/YYYY-MM-DD 路径仍可进入 hints，但不新加为 roots。归一只 trim、去 `\\?\` 前缀、去末尾 slash/backslash；相对 cwd 和不存在的目录也保留，不擅自加绝对路径/文件存在门槛。
- cwd 回填要求 id/cwd/rollout_path 列；只选未归档、空 cwd、非空 rollout_path。排除显式 projectless ID 和 source 包含带双引号的 `"subagent"` 子串；缺 source 表达式是空字符串，不能伪称它是严格 main-source parser。只取首个 session_meta，身份必须匹配；读取/JSON/身份失败跳过该项；cwd 为空或生成 projectless 路径不回填。
- 原生无时间列会生成 `ORDER BY 0 DESC` 并得到 SQLite 错误；未继续使用旧 adapter 的 `ORDER BY id` 近似。没有 id/cwd 列则空 payload。row/type 错误保留原错误分支，未加旧版 1000 行检查上限。
- `merge_unique` 先现有 saved roots 再 DB 的排序 roots，保留首个归一化值。project-order 合并现有 order 和新 roots，不删除仍不在 roots 的旧 order。active roots 只保留属于新 roots 的项（重复项可保留），空则取首 root。hints 保留现有任意 JSON 值，DB 同 ID 的 cwd 覆盖；未知顶层字段与 projectless 标记保留。没有任何可用 roots 时不发布 JSON。

## 真实写入顺序与适配 API

原版先在单 DB 事务执行 `UPDATE threads SET cwd = ?1 WHERE id = ?2 AND COALESCE(cwd, '') = ''`，commit 成功后若有更新则 best-effort `PRAGMA wal_checkpoint(FULL);`，再重新采集真实 DB payload。随后已有 JSON 先备份，合成四个项目字段、pretty JSON 原子发布、读取/解析并由外层重新检查稳定性。JSON 备份/发布失败不会撤销已经提交的 cwd。

`CodexPaths_from_home.c` 的 +0x248 字段为 home/codexmate/relay，所以原生备份目录是 `home/codexmate/relay/project-state-backups`；文件名为 UTC `%Y%m%d-%H%M%S`、点、UUID simple、`.codex-global-state.json.bak`。缺 JSON 不生成虚构原件备份。

app `project_state.rs` 暴露：

- `inspect(ctx) -> DiagnosticItem`：即使缺 JSON 也运行真实 producer。
- `prepare(ctx) -> Result<Prepared>`、`preview_lines(&Prepared)`；Prepared Clone，私有正文不 Debug/Serialize，getter 提供 state_path/before/after/database/cwd_updates/diagnostic。
- `validate(ctx, &Prepared)`：核显式根、运行路径参数、实际活动库、所有 DB/WAL/配置/项目文件与回填来源原指纹，再复核实际合成效果。
- `apply_under_manager(ctx, &Prepared, &MutationGuard, explicit_backup_root)`：保持真实 Desktop/在途请求静默检查；实际事务、文件发布、再次读取原生检查，返回 success、cwd_updated、state_written、backups、partial_effects_possible 与 diagnostic。

Atelier 适配边界：仅私有 SQLite 副本投影相对 rollout_path 为 ctx.home 下绝对 IO 路径；source DB 的原始字符串始终保留并做 CAS，不修改真实 path。副本投影不执行应用 triggers。Ticket 可指定备份目录并额外备份源 DB/WAL；这不冒充 Native 本身会备份 DB。写前 CAS 与批准范围协议比原版更明确；cwd commit 后实际 payload 不符合审阅结果（包括 trigger 额外项目效果）时保留已完成 DB 效果、停止 JSON 发布并明确报告部分完成。私有快照采用现有 Atelier 128MiB DB/64MiB WAL/8MiB 项目 JSON 界限及稳定性检查；非普通文件、无效 WAL 会明确失败，不声称 native 的每个 OS 错误展示/内部 crash-journal 字节等价。

## 验证

专用临时测试覆盖缺文件重建、单活动 DB 与 configured_home/版本/SQL 评分、普通和 zstd 元数据回填、原始相对 DB path 保留、unknown/null/plain subagent 与显式带引号 subagent 的真实区别、projectless cwd 的 roots/hints 区别、现有项目合并和 JSON 值保留、坏 JSON 原字节备份、无 roots 不创建空修复、原生 ORDER BY 0 错误、源 WAL/SHM 不变、过期项目/数据库/rollout CAS、实际 SQL trigger 回滚，以及 cwd commit 后外部 JSON 变化/额外 trigger effects 的部分完成报告。

生产 inspect/repair 已接真实 Prepared。旧 inspect_existing/cwd_repairs 只保留过渡读兼容。

批量适配新增 `apply_after_approved_prefix(ctx, prepared, guard, backup_root, &ApprovedThreadEffects)`：先核每项原依赖的原始 before 与已认证 opaque edge、实际 current 与 edge after；再重建计划，仅允许原批准目标收缩。原始潜在空 cwd 行仅在原缺失历史确由批准 missing-rollout 恢复时允许产生新 cwd，不接受任意当前来源。返回 opaque `ApprovedProjectEffects`，只有实际文件 exact bytes 和原始 schema/全表 SQL oracle 读回吻合后才签署物理指纹，供 Session.absorb_project 串接。

本轮实际命令 `cargo test --lib maintenance::diagnostics::project_state_tests -- --quiet`：18/18，通过日志 `/tmp/atelier-project-state-prefix-final3.log`。新增覆盖真实 provider 前序、zstd missing 恢复、未经批准外部 DB/history 变化。
