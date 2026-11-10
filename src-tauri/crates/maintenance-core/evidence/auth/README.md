# 认证、解锁认证残留、接管备份恢复的原生证据

目标：AiMaMi 1.2.7，`AiMaMi.arm64` SHA-256 `b2a4b1b2b4babd2518e02c0f0d6203ad19e779909a47b3b643844ef403fe95de`。`native/` 保留只读 Ghidra C 与直接 llvm-objdump 证据；没有运行目标，也没有读取真实 auth、配置、会话、数据库或钥匙串。所有测试创建显式 TempDir。

## 稳定 API

- `auth::AuthContext::new(codex_home, RouterMode::{Off, Native, NoAccount})`：根路径显式传入，不发现 home、不重建原生 Manager、不调用系统凭证服务。
- `auth::check_auth_integrity`、`auth::plan_auth_integrity`、`auth::apply_auth_integrity`：检查、预览、应用后重新检查。应用成功但返回 Warning/Error 仍然是修复未达标。
- `auth::unlock::{check_router_unlock_auth_residue, plan_router_unlock_auth_residue, apply_router_unlock_auth_residue}`：包含旧版 virtual-auth-marker 清理。应用持 `accounts/.live-auth.lock`；如上层已持锁，使用 `auth::apply_plan`，随后重新检查，避免重复获取同一独占锁。
- `auth::takeover::{check_takeover_backup_orphan, plan_takeover_backup_orphan, apply_takeover_backup_orphan}`：严格备份解码、隔离损坏文件、冲突保留恢复、发布与真实后检查。预览返回 `TakeoverPlan.files` 和不含凭据的 `RestoreReport`。隔离的应用应使用专门 apply，保证复制校验发生在删除之前。
- `AuthPlan.changes` 是按顺序的文件 before/after；`observations` 包含未修改但参与判断的依赖。两者含认证原始字节，均刻意没有 Debug/Serialize，调用层不得把它们原样放进日志/IPC 预览。预览可显示路径、操作和字节数量。

## 已还原分支与证据

| 行为 | 原生证据 |
|---|---|
| auth=`<root>/auth.json`，普通认证备份=`<root>/accounts/backups`，relay=`<root>/codexmate/relay` | `native/CodexPaths_from_home.c:112-210, 269-316`，构造器 `0x1008efb84`；只采用字段投影，未执行构造器的迁移副作用 |
| metadata 任意错误视为 Missing/Warning；读取错误、空白、无效 JSON 为 Error；typed managed virtual 优先；access token 优先 API key；无 token 时原样 `sk-` 前缀即 API 模式 | `native/check_auth_integrity.c:89-130, 156-245, 324-579`，`0x100375360` |
| 占位符：原始 UTF-8 字节数小于 10，或 Unicode lowercase 后命中已恢复列表；不 trim | `placeholder.asm`，`0x100388ce8-0x100388f50` |
| JWT 仅恰好 3 段、前两段 URL_SAFE_NO_PAD base64 可解码；第三段不验证，不检查签名、claims 或过期 | `plausible-jwt.asm`，`0x100371fa8-0x10037209c`；engine 数据 `0x10130ca4d` 开头 `00 00 02`，alphabet 为 `...789-_` |
| typed AuthFile 的字段类型/default、managed flag/虚拟 key/`-unlock`+固定 account UUID；OAuth 可切换条件仅 trim 后非空、不以 `__` 开始、至少两个点，且不是 API/managed | `native/AuthFile_deserialize.c:745-789`；`native/is_aimami_managed_virtual_auth.c:50-208` `0x1001f0f9c`；`native/is_switchable_oauth_auth.c:22-64` `0x1001efc84` |
| timestamp null/空串为 None；非数值字符串 trim 后接受；数值大于 1e11 除 1000、截断秒、chrono 范围校验 | `native/deserialize_optional_timestamp.c:105-195` `0x1001f0b8c`；`native/timestamp_normalize.asm` `0x1001ed9c0` |
| auth 恢复仅路由 Off；非递归 read_dir，跳过 entry 错误，只选 is_file 且 extension 精确为 `json`；stable mtime 倒序，metadata/modified 失败回落 epoch | `native/fix_auth_integrity.c:82-132` `0x100373af8`；`native/auth_candidates_try_fold.c:100-135` `0x10080b34c`；`native/auth_candidates_sort_compare.c:64-151` `0x100910780` |
| 候选允许非占位符但格式异常的 token；首个合格候选复制后即重新检查并返回，复制失败/后检异常均不继续旧候选 | `native/fix_auth_integrity.c:132-274` |
| live enum：Missing/VirtualApiKey/FlaggedUnlock/User/Unreadable；virtual 检查 trim 后 APIKEY+虚拟前缀+tokens 非对象或空对象，先于 flag；无效 JSON 也 Unreadable | `native/live_auth_state.c` `0x1008a8b64` |
| has_residue 包括 marker、backup 或 live enum 1/2；因此三种 RouterMode 的这些残留都报 Error；NoAccount 启用时禁止修复 | `native/residue.asm` `0x1008a89f8`；`native/check_router_unlock_auth_residue.c:20-35, 164-169, 235-239`；`native/fix_router_unlock_auth_residue.c:53-56` |
| 新 marker：apiKey/email/accountKey Option<String>、createdAt i64、backedUpPreviousAuth bool；全字段默认。marker 解析失败按 orphan 恢复；User 保留现有 auth 并清 sidecars，Unreadable 全保留 | `native/RouterUnlockAuthMarker_deserialize.c:494-517, 540-552, 696-724, 771-780` `0x10026a024`；`native/auth_cleanup_unlocked.c:183-379, 438-602` `0x1008a8e88` |
| 新备份 raw 用户非空 key 优先接受；managed 返回 None 删除虚拟认证；其他无用户 OAuth/key 的备份报错保留；旧版备份必须先 typed parse、拒绝 managed 和原样虚拟 key 前缀，再接受 is_api_key_auth 或可切换 OAuth | `native/auth_read_cleanup_backup.c:206-400, 429-520` `0x1008a9f94`；`native/legacy_read_user_owned_backup.c:116-203` `0x100201ec4` |
| 旧 marker apiKey 必需，backedUpRealAuth 默认 false；只有当前 key 与 marker 完全相等且 tokens 缺失/null 才覆盖/删除，tokens={} 不算；用户 auth 保留；现有 auth 无法读/解析则 sidecars 保留 | `native/legacy_auth_cleanup.c:213-326, 432-496, 574-678` `0x100202408` |
| .live-auth.lock 位于 accounts；read/write/create、0600；独占锁阻塞竞争按 25ms 重试、20s 截止；Interrupted 重试 | `native/auth_cleanup_for_native_off.c:71-80, 101-171` `0x1008aaeb8`；`native/lock_open.asm` `0x1007851a4`；`native/live_auth_lock_acquire.c:46-99` `0x100785384` |
| takeover orphan 检查仅 metadata 任意成功，路由 Off 且存在才 Warning；路由 On 禁止修复 | `native/backup_exists.c` `0x1001f5724`；`native/check_takeover_backup_orphan.c` `0x10038fafc`；`native/fix_takeover_backup_orphan.c:61-90` `0x1003885e8` |
| takeover 两备份必须 symlink_metadata 为 regular file；NotFound 为空；不接受 symlink/目录。schema 错误而 JSON 有效是 Unsupported，保留原件；非 JSON 损坏才可隔离 | `native/auth_read_backup_bytes.c:28-127` `0x1001f7ab4`；`native/auth_decode_backup_string.c:1018-1050` `0x1001f620c`；`native/auth_decode_backup_value.c` `0x1001f584c` |
| 当前 schema version=1、createdAtMs i64、topLevelLines、originalConfig 必需，sectionBlocks 默认空；旧 schema version 默认 1、lines 必需；均 denyUnknown；section key/lines 必需且 denyUnknown | `native/auth_decode_backup_string.c:448-495, 509-709, 809-821`；`native/auth_decode_backup_value.c:166-254`；`native/TakeoverSection_next_element_seed.c:181-215, 249-278` `0x100237ac0` |
| 隔离名 `{原文件名}.damaged-{epoch_ms}-{0..99}.bak`；先检查两备份可恢复 schema，再 private copy+require_sync+新旧 bytes 复核，才删除原件 | `native/quarantine_damaged_takeover_backups.c:99-131, 164-260, 342-350` `0x1001fb728` |
| 恢复不整文件回写 originalConfig：按当前非托管顶层 key 与 section 冲突跳过；相同 key 的后续备份行跳过；section 公共 dot 前缀相等视为 overlap；忽略 AiMaMi-owned 恢复项及含 owned 子 header 的整个块 | `native/config_takeover_restore.c:235-637` `0x1001fd5cc`；`native/takeover_restore_topline_closure.c:27-96` `0x1001ff460`；`native/section_overlap.asm` |
| owned model namespace、provider/profile 名、router base URL、精确自身 catalog 路径来自真实上下文；section 简单双引号 toggle，normalize 简单 split('.')+trim+trim_matches('"')，不改进单引号/转义行为 | `native/is_aimami_owned_top_level_line.c:242-318` `0x1001fcb88`；`native/is_aimami_owned_model_section.c` `0x1001fc9ec`；`native/takeover_first_part.asm`；`native/normalize_section_map.asm`；`native/catalog_path.asm` |
| 恢复顶层行置前，恢复 section 插入第一个 general managed begin 之前（找不到则末尾）；保留现有 managed 块；只有实际恢复项时 tidy（去首尾空行、保留内部空行、统一行尾并追加 LF）；TOML 成功且 config 同步后才删除活动备份 | `native/config_takeover_restore.c:637-982`；`native/tidy_blank_lines.c:43-149` `0x1001f77e8`；`native/validate_config.asm` `0x1005c1f80` |
| 仅有效但 lines=[] 的旧备份且无当前备份时原生提前返回、不删除旧备份，后检仍 Warning | `native/config_takeover_restore.c:183-231` |

## 预览事务适配边界

恢复的内容选择、判断、排序、后检查均由本模块执行，没有调用者 should_fix/known_status 替代逻辑。生产接入仍由上层完成：注册模块/IPC、加载真实路由状态、显式选择授权根、安静窗口与全局 CodexMutationGate、确认、事务备份/回退、刷新诊断。没有执行真实客户端验收。

为适配预览事务，本实现先完成全部读取/候选构造，再 apply；因此不会复刻原生“已清新 sidecars 后旧 marker 报错”或“已隔离损坏备份后候选 TOML 报错”的部分写入。错误明确返回，原文件保持。原生 ensure_directories 的全 Manager 目录脚手架没有整套迁移；apply 仅创建实际锁父目录，文件 publication 的父目录须由授权根/现有 relay 目录提供。锁/atomic publication 是 Rust std 的本机等价实现，不声称逐字恢复 AiMaMi atomic_write 内部 failpoint/日志体系。

精确原生中文 auth label 已从 heap 常量和 DAT 恢复为 `AuthCheck::label`。其他错误保留类型化原因和相关路径；尚未逐字迁移所有动态中文诊断 detail、底层 OS error 包装与日志文本，不把这些展示差异当作成功。

## 验证

- `cargo test --manifest-path src-tauri/crates/maintenance-core/Cargo.toml --test auth_native --offline`：39 合成分支测试通过。
- `rustfmt --edition 2021`：仅格式化本目录及专属测试。
- `git diff --check -- src-tauri/crates/maintenance-core/src/auth src-tauri/crates/maintenance-core/tests/auth_native.rs src-tauri/crates/maintenance-core/evidence/auth`：通过。
