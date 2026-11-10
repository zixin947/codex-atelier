# AiMaMi 1.2.7 Image 兼容维护与声明过滤

输入为离线 `AiMaMi.arm64`，SHA256 `b2a4b1b2b4babd2518e02c0f0d6203ad19e779909a47b3b643844ef403fe95de`。本目录 `.c` 为同二进制只读 Ghidra 导出，`.asm` 为离线 otool 导出；没有执行真实客户端、真实 ps/lsof/quit/kill/open，没有读取真实 config/auth/会话数据库。实现 `../../src/image_compat.rs`，模块内仅合成输入和临时目录测试。

## 前端真实合同

安装包提取资源 `maintenance-page-E3Lizhob.js` SHA256 `c592c07002e92a1c8770d8641f568004c127899ba4885974133a01875b500549`，片段见 `frontend-snippets.txt`。开关 query 调 `getImageCompat().data.enabled`，mutation 调 `setImageCompat(bool)`，成功后 invalidate imageCompat 再读。Claude Web Search 开关在同页 disabled，不能把该占位声明成已实现命令。原版 Image 开关没有独立的偏好数据库或 JSON setting。

## 命令和文件语义

| 行为 | 原版证据 |
| --- | --- |
| getter resolve CodexPaths 后直接读 config.toml，结果包装 enabled | `get_image_compat@1007b3e60`，`get_image_compat.c:24-27` |
| setter持共享 CodexMutationGate，resolve/from_home，update_text，返回请求flag | `set_image_compat@1007b4204`，`set_image_compat.c:71-73,107-110,170`；没有保存utility preference，也没有stop/restart |
| getter扫描原始UTF8行：trim、exact `[features]`，key前缀后trim_start必须`=`，RHS trim exact `false`才开启 | `read_image_compat_enabled@100366dbc` / `read_image_compat_enabled.c`；精确分支 `read_enabled.asm:100366f28..100366f9c`。任何IO/UTF8错误返回false；不要求整个输入是合法TOML |
| compose开启只处理第一个exact features段；更新该段最后一个匹配assignment，或插在header后一行 | `compose_image_compat_flag@1007b5bf0`，C:266-359；`compose.asm:1007b5cd4`遇下一header停止，`:1007b5dac`保存最后index |
| 无exact段时追加header+flag；原文非空且没有LF结尾时先插空行；统一lines/join LF并追加LF | compose C:222-259,361-375；ASM `1007b5eb8..1007b5fbc` |
| compose关闭移除所有exact features段中的匹配assignment（不限制原RHS布尔）；无其它非注释非空内容的features段移除header及其紧接空行，注释保留 | compose C:61-215；ASM `1007b60fc..1007b629c`。`image_generation=`空RHS仍匹配；trim返回合法非空指针，不能将Ghidra的pointer检查误解成RHS必须非空 |
| 原生通用update_text：create_dir_all(parent)、read/compose、相同直接成功、再读CAS，不同重试，最多3次；然后private atomic publish+ack | `image_config_update_text@1005c1834` C:51-77,114-124,161-171,208-226；未调用TOML parser，没有原生backup |
| read_text仅NotFound变空串，其它错误保留 | `image_config_read_text@1005c223c` C:18-65；不等于getter的所有错误转false |
| relay缓存只含单个path+Option modified+Option len+enabled，metadata变化才重读；poison fallback直接读 | `image_compat_enabled@100366934` C:64-100,141-192；无TTL/内容hash，setter不写独立cache状态 |

这意味着 `[features] # comment`、`[ features ]`、quoted key、`false # inline` 均不是getter开启形式。重复key不会由TOML parser拒绝；某个可识别false即开启。compose也不会校验或重排其它TOML结构。原生格式细节和奇异无效输入已作为明确行为实现，不用现有Atelier测试推断原版。

## 声明过滤

`strip_image_generation_tools@100367028` 仅访问顶层tools和input数组；`filter_input_retain.asm@10055c330` 仅处理type exact additional_tools且tools为数组，过滤后空数组则删除该input记录。计数仅为删除声明数，删除原本空的additional_tools本身不计数。声明predicate `filter_tool_predicate.asm@10055e9e4..10055eb34`：type exact image_generation，或 type exact namespace 且 name exact image_gen。不递归其它工具字段、不删除普通input_image，**不改tool_choice**。原Atelier额外删除tool_choice的逻辑已移除。

## Atelier接线与适配边界

- `maintenance/image_config.rs` 继续提供既有确认票据、备份、CAS、私有atomic发布与读回；预览compose改用核心真实行算法，UTF8错误仍拒绝。原生setter的3次rebase独立由核心set_image_compat提供；有用户已批准预览时仍按Atelier原票据拒绝外部变更，不暗自扩大批准内容。
- 执行回执与maintenance_utility_settings的imageCompat来自实际config getter。旧utility-settings.json中的imageCompat输入被忽略；诊断接收地址继续读写，之后的settings写入仅持久diagnosticEndpoint。
- `gateway::Store::new(root)`只创建本地gateway store，不解析HOME/CODEX_HOME，也不调用任何配置getter。`new_with_config(root, explicitPath)`才注入Image来源；生产GatewayState显式解析CODEX_HOME/config.toml（或HOME/.codex/config.toml），测试仅注入临时路径。每次请求使用原生path/mtime/len cache决定是否复用核心filter。
- ZIP生成、保存、发送的生产IPC均显式传config路径；manifest实时扫描得到imageCompat布尔值。没有显式来源的fixture/helper输出null和imageCompatSource=unknown，避免把未观察的状态写成false。导出不包含路径或原始配置。
- 核心StdImageCompatFs提供实际0600临时文件、file fsync、同目录rename、parent fsync。原版Publication的内部标记/临时文件命名和日志/CoreEnvelope传输没有逐字复制；错误在publish后目录fsync失败时仍可能伴随已发布文件，不声称失败可自动回滚。应用层既有backup/readback契约保持。
- 应用层既有1MiB配置读取上限、普通目录/文件检查与预览有效期为Atelier兼容约束，不冒充原版update_text存在同样限制。原生System环境路径解析和真实Desktop是否重新读配置没有执行E2E。

## 实际验证

- `cargo test --manifest-path src-tauri/crates/maintenance-core/Cargo.toml --lib image_compat --quiet`：10 passed。覆盖词法边界/重复key、最后assignment选择/空RHS、append/LF、删除空段/保留注释、filter精确位置与shape、真实0600/private文件和missing-disable不创建config、invalid UTF8、metadata cache原生stale边界、3次CAS冲突/IO错误/不发明TOML校验。
- `cargo test --manifest-path src-tauri/Cargo.toml maintenance::utilities::tests --no-default-features --quiet`：10 passed。包括旧偏好失效、默认Store无隐式来源、endpoint持久边界、ZIPmanifest的null/false/true/false实值、过滤保留tool_choice、合成HTTP发送与ZIP有效性。
- `cargo test --manifest-path src-tauri/Cargo.toml maintenance::image_config::tests --no-default-features --quiet`：4 passed，包含实际配置bytes与备份断言。
- root运行的loopback网关测试终态1/1 passed（`/tmp/atelier-image-http-latest.log`）：临时config false→true→false，三次真实HTTP请求上游捕获分别为透传→过滤→透传，input images保持，不写utility image偏好。只有本机合成服务，无真实客户端。
- root报告最终core整套268 passed（包含本模块10项）；生产cargo check通过，仍有38项现存warnings，不声称零warning。最后maintenance全套由root独立记录。
