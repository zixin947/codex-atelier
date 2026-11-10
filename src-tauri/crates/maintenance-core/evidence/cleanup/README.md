# 安装版“清理数据”恢复证据

目标为只读副本 `AiMaMi.arm64`，SHA-256：
`b2a4b1b2b4babd2518e02c0f0d6203ad19e779909a47b3b643844ef403fe95de`。
这里的行为结论来自安装包前端、原生反编译和 ARM64 指令；没有运行原客户端，也没有读取真实配置、认证、会话库或钥匙串。

## 已恢复的调用链

| 原生函数 | VA | 本目录证据 |
| --- | --- | --- |
| `commands::system::clean` | `0x1007b9084` | `command_clean.c`, `system_clean.asm` |
| `Repository::clean` | `0x10082600c` | `repository_clean.c`, `repository_clean.asm` |
| `load_registry_for_mutation` | `0x10081d2f8` | `load_registry_for_mutation.c` |
| `load_registry` / `persist_registry` | `0x10080c338` / `0x100810df0` | 同名 C |
| `remove_account_backup_files` | `0x10082fb00` | 同名 C |
| quota load / save / encode | `0x1001e85a8` / `0x1001e92e8` / `0x1001e7fb8` | `quota_store_load.c`, `quota_store_save.c`, `quota_encode_for_save.c` |
| snapshot switchability | `0x100832a08` / `0x1008313e8` | `registry_item_has_switchable_oauth_snapshot.c`, `load_switchable_account_snapshot.c` |
| managed snapshot location / auth snapshot | `0x1001f1bb8` / `0x1001ecaf8` | `auth_ensure_managed_account_snapshot_location.c`, `make_auth_snapshot.c` |
| `CodexPaths::ensure_directories` | `0x1008ee11c` | `codex_paths_ensure_directories.c` |

`frontend-snippet.txt` 保留安装包维护页的 `clean` 调用与三个计数字段；该入口不是删除全部 Codex 历史。原生命令持 Repository mutex，内部取得 `accounts/.account-domain.lock`（20 秒上限、25 毫秒重试）。未发现清理入口停止客户端或取得 CodexMutationGate 的分支。

## 实际文件与失败顺序

传入的 home 是明确路径，不在模块内查找环境变量或用户 HOME。默认映射：

| 用途 | 相对显式 home 的位置 |
| --- | --- |
| 账号索引 | `accounts/registry.json` |
| 可切换 OAuth 快照目录 | `accounts/snapshots` |
| 认证备份 | `accounts/backups` |
| 索引备份 | `accounts/registry-backups` |
| 可丢弃额度缓存 | `codexmate/quota-store.json` |

1. `load_registry_for_mutation` 的 metadata 任意错误得到空 schema 4 索引；metadata 成功后的读取或 JSON 错误终止清理。
2. 索引只保留快照 metadata 存在、父目录 canonicalize 后恰好等于受管 snapshots 根、不是符号链接且是普通文件、能解析为真实 `auth::AuthFile` 并满足 `is_switchable_oauth_auth` 的项。`user-unlock::00000000-0000-4000-8000-000000000001` 特殊账号排除；子目录也不被接受。`make_auth_snapshot` 仅有 managed virtual auth 业务失败，JWT 元数据有 unknown-account fallback，因此没有添加额外 JWT 签名或身份一致性门槛。
3. 活动账号若不在保留集合中即清空；更新索引时间。创建原生 9 个目录（accounts、snapshots、两类 backups、codexmate、skill-backups、custom-instructions、其 history、relay），schema 至少 4，缺失 autoSwitch/api 填默认值；以私有文件发布索引。原生 `persist_registry` 此处 `create_backup=false`，不会为清理新增索引备份。
4. quota 任意读取/解析错误静默得到空 schema 3 缓存；不因该错误覆写原字节。有效缓存先按原生规则规范化窗口，再仅保留存活账号。只有项数减少才更新时间和保存；保存失败添加 `QUOTA_CACHE_CLEANUP_INCOMPLETE`，继续清理备份，不回滚索引。
5. auth backups 再 registry backups：目录 metadata 错误静默为零；read-dir 错误产生无法检查警告。每个直接目录项都调用 remove_file，没有扩展名、类型、保留天数过滤，也不递归。普通目录删除失败，符号链接仅移除链接。计数仅含成功 unlink；枚举项错误与删除错误合并计失败。
6. 返回 `authBackupsRemoved`、`registryBackupsRemoved`、`staleEntriesRemoved` 三个 i32。警告返回顺序为认证备份、索引备份、quota 保存错误；已经完成的发布和删除不撤销。

原生错误码为 `AUTH_BACKUP_CLEANUP_INCOMPLETE`、`REGISTRY_BACKUP_CLEANUP_INCOMPLETE`、`QUOTA_CACHE_CLEANUP_INCOMPLETE`。前两者消息分别是 `Some {label} backups could not be inspected.` 或 `{failed} {label} backup(s) could not be removed.`，label 精确为 `authentication` / `registry`。

## 类型与序列化证据

- Registry struct parser `0x10027a0a0`（`registry_deserialize_struct.c`）和 serializer `0x10080bd84`。RegistryItem leaf parser `0x10025b970` 的七个 missing-field 分支已逐一解二进制常量，见 `registry_item_parser.asm`、`registry-required-fields.txt`；必需字段恰为 accountKey、snapshotPath、email、alias、plan、authMode、createdAt。其余 Option 缺失为 None。
- RegistryItem serializer `0x10082a1e8` 保留 23 字段；两项 provenance None 跳过，其余 Option 依原生输出 null。PlanStateSource 精确为 jwt/remote/usage；SubscriptionStateSource 为 jwt/livePlan/remote，见两个 ASM 和 `source-variants.txt`。
- AutoSwitch parser `0x10026c9e4`：revision i64 默认 0、enabled 默认 false、两项 i32 阈值默认 15/10（`auto_switch_parser.asm`）。ApiConfig parser `0x10025fed8` 仅 usageRefreshEnabled 默认 true；没有旧公开源码的 accountMetadataRefreshEnabled（`api_config_parser.asm`）。
- quota item parser `0x100239f14`（`quota_item_next_element_seed.c`）证明 accountKey、capturedAt、usageSource 必需，其他字段 Option；UsageSource 仅 local/api。Window 为 f64 usedPercent、i32 remainingPercent、Option<i32> windowMinutes、Option<i64> resetsAt。
- AccountTokenStatus parser `0x1002721b8` 的 code 缺失进入 Error::custom（`token_status_parser.asm`）；code 精确 fresh/refreshed/noRefreshToken/refreshReused/refreshFailed（`status_code_visitor.asm`），另外两项 Option 缺失为 None 且序列化跳过。
- quota 正规化保留 primary 需未过期且 windowMinutes 在 1..=360；secondary 需未过期且 windowMinutes 缺失或至少 8640。没有百分比钳制。只有删项触发保存，所以单纯窗口修正不写回。

## 可供适配层使用的接口

`cleanup.rs` 提供真实 `StdCleanupFs`，并非只定义 trait。

- 原生直接执行：`clean(&CleanupPaths)`；已持账号域锁时使用 `clean_under_existing_lock_with_fs(fs, paths, now)`。
- 预览：`prepare(&CleanupPaths) -> Result<CleanupPlan, CleanupError>`；`plan.preview()` 仅返回计数、备份路径、检查警告，不含快照/索引正文。`prepare` 不创建目录或锁文件。
- 应用：`apply(&CleanupPlan)` 会取得域锁；已有锁调用 `apply_under_existing_lock_with_fs`。显式 FS 和时间版本便于临时 fixture 和错误注入。`validate_with_fs` 可在多操作组合写入前复核。
- `CleanupPaths` 每个目录、JSON 路径均可显式指定；`from_home` 只构造路径。`CleanupPlan` 的原文依赖为私有字段，不实现 Debug/Serialize。

计划依赖包括原始索引/额度字节、受管快照内容与文件类型、canonical 路径和批准的备份集合。外部、解锁别名及 symlink 快照不会为了指纹而读取正文；备份 symlink 不读取目标正文。执行前过期计划拒绝。应用中出现新备份或批准文件变化则保留它们，报告 `scope_changed` 与适配层新增的 `CLEANUP_APPROVED_SCOPE_CHANGED`；此前索引/缓存发布保留。这个批准范围协议属于 Atelier，未冒充原生原本就有 preview/CAS。

## 验证与边界

实际执行 `cargo test --manifest-path src-tauri/crates/maintenance-core/Cargo.toml --test cleanup_native --quiet`，16/16 通过。全部使用 tempfile、合成 JSON 和可注入 FS，覆盖真实索引发布、OAuth/API/managed/unlock/外部与 symlink 筛选、活动键、quota 类型/窗口/坏缓存/保存错误、两类备份实际字节、目录保留和部分失败、锁互斥释放、schema/None 序列化、非创建预览、过期计划、执行中范围变化、禁止读取外部快照。

已注册核心公共模块并用公共 API 重跑上述 16 项测试；尚未接生产 store/IPC，由父任务接入 Atelier 的 preview、确认、备份及回执流程。模块没有执行原 CodexPaths 初始化的旧目录迁移。StdCleanupFs 使用 0600 create_new 临时文件、fsync、rename、目录 fsync，实现实际私有发布；原生 Publication 的内部崩溃日志、临时命名与 CoreError 的传输展示不宣称字节等价。目录 fsync 失败可能发生在 rename 之后，和原生部分完成语义一样不能承诺全部回滚。FS seam 先收集 ReadDir 项再顺序删除；原生直接使用迭代器，账号锁与 Atelier 批准范围校验处理本实现的适配边界。
