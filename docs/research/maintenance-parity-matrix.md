# 维护工具 · 系统诊断条目矩阵（AiMaMi 1.2.7 对齐基线）

本文件是 `/maintenance` 系统诊断的权威条目表。所有文案取自 `AiMaMi.arm64` 的只读常量区，通过 `asm/*.asm` 中的 `adrp` + `add` 指令定位到具体 check 函数引用的字符串地址，不依赖现有 Atelier 实现反推。

原始提取产物：

- `docs/research/maintenance-evidence/arm64-cjk-constants.txt`：二进制内全部含中文的 UTF-8 常量片段，1,061 条，带文件偏移。
- `docs/research/maintenance-evidence/asm-strings.json`：24 个函数的字符串引用表，含 arm64 虚拟地址。

## 一、返回结构（已确认）

```text
DiagnosticItem  { status, label, fixable }        // detail 另有独立字段
DiagnosticReport { hasIssues, transitionInFlight, items }
FixResult        { fixedIds, failedIds, details }
```

字段名证据：`0x130ce07` 起的常量区连续出现
`fixedIds failedIds details DiagnosticItem status label fixable DiagnosticReport hasIssues transitionInFlight`。

`status` 取值：`ok` / `warning` / `error` / `skipped`（`0x130cbbc` 附近包含 `warning`、`error` 与 `ok`）。

## 二、过渡态包装（已确认，Atelier 尚未实现）

智能路由切换期间，条目 label 会被整体包裹，而不是只在顶部提示：

```text
智能路由正在切换（线程收敛进行中，线程越多/磁盘越慢耗时越长）。
该项是过渡态，收敛完成后会自动消失——请勿在切换过程中点击修复或反复开关。
原始检测：<原始 label>
```

证据：`0x130e015`、`0x130e02b`、`0x130e045` 等条目 id 之后连续出现该模板，末尾接 `原始检测: ` 再拼原始 label。

含义：过渡态的条目 `fixable` 必须为假，且不参与「全部修复」计数。

## 三、分组（原版前端分组表，29 项具名 + 6 项回退 runtime）

原版前端硬编码分组，未知 id 回退到 `runtime`：

| 组 id     | 中文标签     | 条目数                |
| --------- | ------------ | --------------------- |
| `config`  | 配置         | 12                    |
| `auth`    | 认证         | 4                     |
| `runtime` | 运行时与线程 | 12（+6 项未具名回退） |
| `env`     | 环境与磁盘   | 1                     |

后端实际条目 35 项。6 项未在前端分组表中的是：`router_postcondition`、`thread_model_catalog_consistency`、`legacy_migrated_threads`、`project_state_integrity`、`router_recovery_record`、`native_off_invalid_openai_thread_models`。

## 四、29 项具名条目文案（有证据）

文案格式为「label 变体 → detail」。同一项多个 label 表示不同状态分支。

### config 组

| id                           | label 变体                                                                                                                                                                                                                                                     | detail / 说明                                                                                                                                                | 证据                                    |
| ---------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------ | --------------------------------------- |
| `config_toml_syntax`         | TOML 语法正常 / TOML 语法正常（空文件） / config.toml TOML 语法错误 / TOML 语法（文件不存在，跳过）                                                                                                                                                            | 可解析；错误时可由系统诊断抢救重建                                                                                                                           | `0x130db57`、`0x130db77`                |
| `config_third_party`         | 无第三方注入残留 / 检测到第三方域名重定向残留 / 检测到第三方工具残留 / 配置无法读取 config.toml / 文件不存在或无读取权限                                                                                                                                       | 修复后写回「已清理第三方注入并重写配置」                                                                                                                     | `0x130da82`、`0x130d360`                |
| `config_profile_conflict`    | 无 legacy 顶层 profile / legacy 顶层 profile 冲突                                                                                                                                                                                                              | Codex 0.134+ 已不再支持 config.toml 顶层 profile；智能路由应改用 model_provider + model_catalog_json 的桌面端路由契约。顶层 profile 配置（文件不可读，跳过） | `0x130ecd1`、`0x130ed15`、`0x130e891`   |
| `config_stale`               | Codex 配置一致性正常 / 路由已关闭但 config.toml 仍有生效的 AiMaMi 路由配置 / 路由已启用但 config.toml 不符合 RouterOn 契约 / Codex 配置一致性（文件不可读，跳过）                                                                                              | 修复后写回「已清理残留注入并重写配置」                                                                                                                       | `0x130ca1d`、`0x130cbbc`、`0x130cbc3`   |
| `router_http_contract`       | aimai1 HTTP-only 路由契约正常 / 缺少 aimai1 HTTP-only provider / 路由已启用，但 config.toml 中没有 [model_providers.aimai1] / Router HTTP 契约（路由未启用，跳过） / Router HTTP 契约（TOML 损坏，等待语法修复） / Router HTTP 契约（config…）                 | 修复后写回「已重新写入 aimai1 HTTP-only 路由契约」                                                                                                           | `0x130e215`、`0x130dbce`                |
| `takeover_backup_orphan`     | 无 takeover 备份残留 / 路由已关闭但 takeover 备份仍存在 / 第三方/用户路由配置仍处于 AiMaMi 隔离备份中 / 路由启用中 takeover 备份属于正常接管态                                                                                                                 | 修复后「已还原 takeover 备份并删除残留备份文件」                                                                                                             | `0x130eba1`、`0x130e594`                |
| `catalog_path_validity`      | Catalog 路径有效 / model_catalog_json 指向非 AiMaMi 路径 / model_catalog_json 指向第三方工具残留路径 / model_catalog_json 指向的文件不存在 / config.toml 缺少 model_catalog_json 引用 / 路由已启用但配置中没有 catalog 路径 / Catalog 路径（路由未启用，跳过） | 修复后「已重新同步 config.toml 中的 catalog 路径」                                                                                                           | `0x130e723`、`0x130edf…`、`0x130dfdf`   |
| `catalog_integrity`          | Catalog 完整 / Catalog 文件缺失 / Catalog 文件为空 / Catalog 缺少 models 数组 / Catalog 文件 JSON 格式损坏 / Catalog 文件无法读取 / Catalog 与卡片模型及历史兼容映射不一致 / Catalog 完整性（路由未启用，跳过）                                                | 缺失与损坏时「路由已启用但 catalog 不存在，Codex 菜单无法显示中转模型」                                                                                      | `0x130d6cb`、`0x130d69f`、`0x130d29b`   |
| `official_catalog_freshness` | 最近一次官方目录获取成功 / 官方目录（当前模式无需获取） / 官方目录刷新失败，未返回替代列表 / 尚未观测到目录请求，不能据此确认官方目录可用 / 等待 ChatGPT 原生目录请求                                                                                          | 已有有效目录由 ChatGPT 原生管理；等待下次原生刷新，无可用目录时不保证混合菜单                                                                                | `0x130f9a4`、`0x130f8ea`                |
| `config_global_validity`     | Codex 配置一致性正常 / 路由目标态存在 N 项偏差                                                                                                                                                                                                                 | 项目状态与配置维度分开报告，偏差由 config_stale 报告                                                                                                         | `0x130ca1d`、`0x101520b52`              |
| `config_access`              | config.toml 可正常读取 / config.toml 尚不存在（正常） / config.toml 无法读取                                                                                                                                                                                   | 无法读取时：路由不探测或刷新 OpenAI Token，登录交给 ChatGPT                                                                                                  | `0x130e215`、`0x130e222`                |
| `config_replace_readiness`   | config.toml 权限未显示只读 / config.toml 为只读 / config.toml 不是普通文件 / 无法检查 config.toml 可替换性                                                                                                                                                     | 只读：Windows 只读属性会导致路由开启/关闭写入 config.toml 时被拒绝访问。请先解除只读属性后重试。非普通文件：不是普通文件，智能路由无法安全原子替换           | `0x130f17b`、`0x130f1a8`、`0x1015216f8` |

### auth 组

| id                           | label 变体                                                                                                                                                                                                                                          | detail                                                                                                                                                                                                               | 证据                                                                  |
| ---------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | --------------------------------------------------------------------- |
| `auth_integrity`             | auth.json 正常 / auth.json 不存在 / 文件内容被清空，Codex 无法正常鉴权 / auth.json access_token 为空 / OAuth 凭证缺失，智能路由透传官方模型将返回 401。可能被第三方工具清空 / auth.json 正常（API Key 模式） / auth.json access_token 格式异常      | 不存在：未发现文件型认证；ChatGPT 可能使用系统凭证存储，请在原生客户端确认登录状态。无可用备份：请在 AiMaMi 账号管理中重新登录 ChatGPT 以恢复 auth.json。路由模式不恢复或覆盖 OpenAI 认证，请在 ChatGPT 中管理登录。 | `0x130cc9c`、`0x130cf9d`、`0x130d02b`                                 |
| `api_key_integrity`          | API 密钥完整 / N 个 Provider 密钥缺失 / N 个 Provider 密钥存储异常                                                                                                                                                                                  | 缺失：缺失密钥：…。请在编辑页面重新输入 API Key。异常：密钥读取失败：…。请先检查本地密钥存储及目录权限后重试，读取失败不等于密钥缺失                                                                                 | `0x130b5a0`、`0x1015203b4`、`0x101520302`、`0x10152033e`              |
| `auth_token_expiry`          | Token 过期检测（无 auth.json，跳过） / Token 过期检测（API Key 模式，不适用） / Token 过期检测（JSON 损坏，跳过） / Token 过期检测（无法读取，跳过） / access_token 有效（剩余 N 天 N 小时） / access_token 将在 N 分钟后过期 / access_token 已过期 | 无法解析 exp：token 格式异常，无法提取 exp claim。已过期且有 refresh_token：有 refresh_token，账号管理页刷新或切换账号即可自动续期。已过期且无 refresh_token：需要重新登录 ChatGPT                                   | `0x130d4b2`、`0x130d537`、`0x1015203f4`、`0x101520451`、`0x101520479` |
| `router_unlock_auth_residue` | 无虚拟账号残留 / 路由已关闭但虚拟账号恢复文件仍残留 / 路由已关闭但虚拟账号 auth 仍残留 / 当前 auth.json 未被虚拟账号占用，但 marker/backup 未完成清理                                                                                               | 可安全执行修复以收敛 NativeOff 状态。已残留时 Codex 可能误以为当前是可用账号，但官方模型透传会失败；应恢复真实 auth 或删除虚拟 auth                                                                                  | `0x130f9be`、`0x130f976`                                              |

### runtime 组

| id                                        | label 变体                                                                                                                                       | detail                                                                                                                                                                                                                                                                                   | 证据                                                                                   |
| ----------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------ | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | -------------------------------------------------------------------------------------- |
| `proxy_running`                           | 本地代理运行中 / 本地代理（智能路由未启用，跳过检测） / 本地代理未运行                                                                           | 未运行：请重启 AiMaMi 让代理自动拉起                                                                                                                                                                                                                                                     | `0x130ce07`、`0x1015201dc`、`0x130ce22`                                                |
| `thread_missing_rollouts`                 | 无示例线程 rollout 文件完整 / N 条活跃线程缺少 rollout 文件                                                                                      | 线程数据（数据库不存在，跳过）                                                                                                                                                                                                                                                           | `0x130f0f2`、`0x101521442`、`0x130dcc1`                                                |
| `thread_history_integrity`                | 线程历史结构自洽 / 线程历史结构存在异常 / N 条记录结构异常 / N 条历史继承引用指向已不存在的 rollout / N 条无法读取                               | 本轮完整检查 N 条，另有 M 条仅部分检查（结论仅对已检查部分成立）。另有 M 条历史文件缺失，已交由「线程缺历史文件」单独报告。这些线程可能无法正常分页打开；AiMaMi 不会自动改写历史，请从备份恢复。                                                                                         | `0x130f288`、`0x101521735`、`0x101521796`、`0x1015217ea`、`0x101521803`、`0x101521852` |
| `thread_provider_consistency`             | 未发现失效的 AiMaMi 线程 provider / 无法完整读取线程数据库                                                                                       | 仅核对 AiMaMi 管理的引用；其他原生、企业或自定义 provider 保持原样，不由本地定义缺失推断无效。                                                                                                                                                                                           | `0x130dcc4`                                                                            |
| `thread_model_catalog_consistency`        | （无独立文案，回退 runtime）                                                                                                                     | 与 Catalog、卡片模型及历史兼容映射比对一致性                                                                                                                                                                                                                                             | `0x130e077`                                                                            |
| `thread_poisoned_instructions`            | 线程指令未发现污染 / N 条线程包含 M 个旧版调用项 ID                                                                                              | 旧版 AiMaMi 曾为工具搜索和自定义工具写入官方不接受的 ID 前缀，切换官模续聊时会返回 invalid_value。修复只改精确命中的 tool_search_call.id 和 custom_tool_call.id，保留 call_id、工具结果和对话内容。                                                                                      | `0x130e097`、`0x1015219e4`、`0x101521a10`                                              |
| `thread_response_item_ids`                | 线程调用项 ID 健康（已检查 N 个） / 线程调用项 ID 健康（数据库不存在，跳过） / 无法完整检查线程调用项 ID / N 条线程无法检查调用项 ID             | 这些 rollout 无法完整读取，未做任何修改；另有 N 条线程无法检查，保持原样                                                                                                                                                                                                                 | `0x101521916`、`0x130f2d6`、`0x101521947`、`0x10152196c`、`0x1015219b1`                |
| `legacy_migrated_threads`                 | 无旧版线程迁移残留 / 旧版 AiMaMi 或第三方工具将这些线程的 provider 改为了内部路由标识                                                            | 修复：迁移至当前 provider                                                                                                                                                                                                                                                                | `0x130ef27`                                                                            |
| `db_orphan_providers`                     | 线程数据（数据库不存在，跳过） / 未发现失效的 AiMaMi 线程 provider                                                                               | 与 thread_provider_consistency 共享文案区                                                                                                                                                                                                                                                | `0x130da82`、`0x130ddbf`                                                               |
| `rollout_orphan_providers`                | 无第三方注入残留 / 检测到第三方域名重定向残留                                                                                                    | 与 config_third_party 共享文案前缀                                                                                                                                                                                                                                                       | `0x130da95`                                                                            |
| `residual_manifest`                       | 无残留迁移 manifest / 路由已关闭但迁移 manifest 未归档 / 迁移 manifest 存在（路由启用中，正常）                                                  | 未归档：可能上次关闭中断，清理后不影响功能。修复后「已将残留 manifest 归档到 history」                                                                                                                                                                                                   | `0x130d8bc`、`0x130d8d4`、`0x130d8ff`、`0x130d300`                                     |
| `residual_cache`                          | 无残留 relay-state-cache / relay-state-cache 正常 / relay-state-cache 引用了已删除的 provider / relay-state-cache 文件无法读取                   | 引用已删 provider：清理后下次路由开关会重新缓存。修复后「已删除 relay-state-cache」                                                                                                                                                                                                      | `0x130d16a`、`0x130d185`、`0x130d19d`、`0x130d1f7`、`0x130cd6d`                        |
| `codex_process_state`                     | Codex 运行中 / Codex 未运行 / 已运行超过 7 天，建议重启 Codex                                                                                    | 未运行：当前未检测到 Codex 进程。如需使用智能路由，请先启动 Codex。运行中详情含「运行时长：N 天 N 小时」                                                                                                                                                                                 | `0x130dc26`、`0x130dc39`、`0x101520903`、`0x1015208e2`                                 |
| `state_db_relocation`                     | 活库不在 sqlite/ 子目录，无需清理 / 无旧库残留 / 检测到双布局线程数据库 / N 本非当前线程数据库 / 当前活库已确认在 sqlite/ 子目录，根目录残留旧库 | 双布局：Codex 未通过 sqlite_home 明确当前数据库目录，AiMaMi 将保留两处数据库，避免在锁竞争或版本切换期间误删仍在使用的线程库。根目录孤儿库：通常是旧版 Codex 迁移后留下的废弃库，Codex 不会再读它，删除或保留都不影响当前线程，但它的记录仍会被「线程缺历史文件」「provider 未收敛」计入 | `0x130d9c7`、`0x130dedf`、`0x101520a64`、`0x101520d01`                                 |
| `router_recovery_record`                  | 路由恢复记录可正常读取 / 路由恢复记录需要单独处理                                                                                                | 恢复标记已保留；请重试当前路由开关或修复目标态，重新验证并提交后才可自动启动 ChatGPT                                                                                                                                                                                                     | `0x130d966`、`0x130d97c`、`0x130e3dc`                                                  |
| `router_postcondition`                    | RouterOn 目标态稳定 / NativeOff 目标态稳定 / 目标态非配置维度稳定（配置偏差由 config_stale 报告） / 路由提交仍待确认                             | 待确认时不得自动启动 ChatGPT                                                                                                                                                                                                                                                             | `0x130e3dc`、`0x130e489`                                                               |
| `project_state_integrity`                 | Codex 项目索引正常 / Codex 项目索引异常 / 项目索引需要修复 / Codex 项目索引无法体检 / Codex 项目索引无法读取                                     | 输出含 roots / active_roots / hints 计数与 cwd_backfill；读取失败：读取线程库失败 / .codex-global-state.json 读取失败，项目入口/线程归属可能异常                                                                                                                                         | `0x130f062`、`0x1015212c3`、`0x101521332`、`0x10152134d`                               |
| `native_off_invalid_openai_thread_models` | NativeOff 官方线程模型正常 / 无法完整读取 NativeOff 官方线程模型 / NativeOff 官方线程模型检查（路由启用中，跳过）                                | 检查旧对话是否保留中转模型选择；统计「N 条旧对话保留了中转模型选择」                                                                                                                                                                                                                     | `0x130e11b`、`0x130f700`、`0x130f7ba`                                                  |

### env 组

| id                    | label 变体                                                                                                                                                                    | detail                                                                                                                                                                                                  | 证据                                                                                                |
| --------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------------------------- |
| `codex_home_writable` | 文件系统正常（剩余 N） / 文件系统正常（空间检测不可用） / 文件系统异常（N 处问题） / 只读 / 目录不存在 / 目录不可写 / 磁盘空间严重偏低（剩余 N） / 磁盘空间严重不足（剩余 N） | 空间阈值：不足 1 GB 时普通读写仍可能成功，但批量线程迁移会在写入前单独校验所需临时空间；不足 100 MB 请尽快清理；不足 10 MB 时 ChatGPT 和 AiMaMi 可能无法安全写入数据。state_5.sqlite 无法以…模式打开：… | `0x130d87c`、`0x101520587`、`0x101520c36`、`0x101520c0d`、`0x101520bea`、`0x130e4e9`、`0x10152055b` |

## 五、修复序列（来自 08 号裁定文档与 fix_all.asm）

`fix_all` 的执行顺序（证据：`asm/fix_all.asm` 的 `bl` 序列）：

1. `run_diagnostics_with_mode`（先跑一次诊断）
2. `fix_requires_quiet_window`（判定是否需要 ChatGPT 静默窗口）
3. 直接修复三项：`fix_auth_integrity`、`fix_router_unlock_auth_residue`、`fix_takeover_backup_orphan`
4. 条件 step 修复五项：`config_toml_syntax`（含 fix-db-orphan 抢救注释）、`catalog_integrity`、`residual_manifest`/`config_stale`、`router_http_contract`、`catalog_path_validity`
5. 复检一次 `run_diagnostics_with_mode`
6. 过滤已解决项
7. `fix_orphan_thread_providers`（需要静默窗口）
8. 剩余项逐项派发给 `RepairAudit::fix` 主分发器

派发顺序常量（`0x130dfea`）：

```text
router_postcondition → config_profile_conflict → takeover_backup_orphan →
router_unlock_auth_residue → thread_missing_rollouts → thread_provider_consistency →
thread_model_catalog_consistency → thread_poisoned_instructions →
thread_response_item_ids → legacy_migrated_threads → config_global_validity →
project_state_integrity
```

## 六、与 Atelier 现状的差异清单

| 差异         | 现状             | 原版                                                     |
| ------------ | ---------------- | -------------------------------------------------------- |
| 条目数       | 35               | 35（一致）                                               |
| 条目 id 集合 | 与二进制完全一致 | —                                                        |
| 分组标签     | 已对齐           | 配置 / 认证 / 运行时与线程 / 环境与磁盘                  |
| 条目文案     | 35 项全部取原文  | 由 `every_label_matches_the_original_constant_text` 固定 |
| 过渡态       | 已对齐           | 逐条包裹 label 并附「原始检测：」                        |
| 单项修复     | 已对齐           | 一键执行，无预览                                         |
| 修复后       | 已对齐           | 自动复检并重渲染                                         |
| 全部修复按钮 | 已对齐           | 「全部修复（N 项）」，N 为 fixable 且非 ok 的条目数      |
| 全部正常态   | 已对齐           | 直接显示对勾与「恭喜您，目前诊断一切正常。」             |

判定条件方面，已按反编译逐条核对通过的是：`config_third_party`、`rollout_orphan_providers`、
`config_toml_syntax`、`config_profile_conflict`、`config_stale`、`codex_home_writable`（含四档磁盘阈值）、
`config_access`、`proxy_running`、`catalog_path_validity`、`catalog_integrity`、`config_global_validity`、
`codex_process_state`、`official_catalog_freshness`、`takeover_backup_orphan`、`router_unlock_auth_residue`。
其余条目的 label 文案已对齐，内部判定分支仍待逐函数核对，见第七节。

### 状态取值（2026-11-10 已统一）

原版 `DiagnosticItem.status` 只有两个字节值：`0 = ok`、`1 = 非 ok`，序列化名为 `ok` / `warning` / `error`。
证据：`asm/ok_item.asm` 中 `strb wzr, [x22, #0x60]` 写入 0；`run_diagnostics_with_mode.asm:1003841a8`
写入 1；常量区 `0x130cbbc` 为 `warning` + `error`。

因此「跳过检测」在原版是 **ok**，不是独立状态。Atelier 此前自造的 `skipped` 与 `unknown`
已全部并入 `ok`（跳过类）与 `warning`（未确认类）。前端 `DiagnosticStatus` 同步收紧为三值，
并由 `every_label_matches_the_original_constant_text` 与 `groups_match_the_original_frontend_table` 守住。

## 七、尚未取得证据的部分

以下需要逐函数继续反编译，当前不宣称已恢复：

1. `router_postcondition` 的提交确认判定条件与「恢复标记」字段。
2. `project_state_integrity` 的 roots / hints / cwd_backfill 具体算法。
3. `native_off_invalid_openai_thread_models` 的模型映射比对规则。
4. 12 项 step 修复各自的前置条件，尤其 `config_toml_syntax.fix-db-orphan` 的触发条件。
5. `fix_requires_quiet_window` 的精确 ID → 0/1/2 表（14 号文档已给表，但未与本轮 id 集合逐项核对）。

## 2026-11-12 实机修复与范围调整

- 用户明确删除 API 密钥完整性检查（不属于修复范围），当前生产诊断 34 项；不读取旧版密钥。
- 全量流式观察实机覆盖 145/145 线程，约 596 MB 解压数据，无预算缺口或读取错误。
- 备份佐证恢复一个损坏历史：150 处 LF 恢复为 U+2028；525 条严格 JSON 记录，其他字节和新增记录保留。
- 旧 Rollout provider 已恢复为经配置验证的 openai_https，当前 gpt-6.1-sol 保留。
- 最终安装版实机诊断摘要 34 项均 ok；不代表上游生成、所有维护页功能或所有故障分支均已验收。
- Provider 修复已写入但首次结果刷新出现来源变化错误，随后独立复检通过；该刷新竞争仍需单独回归。
- 最新前端 111 测试和类型检查通过；提交前 Rust 全量 582 通过、4 忽略，maintenance-core 全套通过。
