# 官方目录请求与新鲜度

输入仅静态 AiMaMi.arm64，SHA256 `b2a4b1b2b4babd2518e02c0f0d6203ad19e779909a47b3b643844ef403fe95de`。
原始材料位于 `/Users/Zhuanz/Documents/New project 5/relay-recovered/maintenance-full`；下列 C 路径相对此根。

## 已核原生行为

- `pseudocode/decomp-run_diagnostics_with_mode.c:3346`–3438：仅 RouterOn 且非 NoAccount 读取官方状态；其它模式显示“当前模式无需获取”。未观测状态仍是 UI `ok`、等待原生目录请求，detail 明确“不据此确认官方目录可用”。失败是 `warning`，最近成功是 `ok`。所有分支 `fixable=false`，没有人工修复 ID。
- `codex-direct/decompiled/official_models_handler_poll.c` @`0x1006ff084`，C:720–855：唯一解码的 `client_version`、长度 <=80、合法 semver；无效为 HTTP400；原生模式未开启/NoAccount 为409。官方 endpoint 字符串 @`0x1013a2e3a` 是 `https://chatgpt.com/backend-api/codex/models`。外层总超时4秒。
- 同文件 C:1069–1190：`official_headers` 校验身份；新建 reqwest client、redirect none、timeout4秒；固定 URL 追加 `client_version`。
- 同文件 C:1280–1460：HTTP仅200成功；Content-Length和累计chunk都限制8MiB；过大 TooLarge，JSON错误 InvalidCatalog，传输错误 Network/Timeout；读取 upstream ETag。
- `official_headers.c` @`0x100677470`：必须精确 `Bearer `，token.trim非空且不是 `aimami-` / `sk-aimami-`；只复制 authorization / chatgpt-account-id / user-agent / originator / version / openai-organization / openai-project / x-openai-internal-codex-residency。cookie及普通API key不作为官方来源。
- `validate_official_models.c` @`0x1006786ec`：必须 models 数组。slug必须字符串且trim非空，排除任何 `::` 和 `aimami_relay_` 前缀；原slug精确去重、保留首次原JSON/顺序/额外字段。没有 hidden 排除，也没有将slug.trim后重写。纯validator允许空数组；caller C:1623–1669将空结果判InvalidCatalog，保留真实droppedEntries。
- `official_catalog_status_serialize[_struct].c` @`0x1006783bc/0x100678520`：始终5字段 available bool、checkedAt i64毫秒、modelCount usize、droppedEntries usize、failure Option。未观测是外层运行态无status，不把failure=null误作成功。
- `catalog_failure_serialize[_struct].c` @`0x100677ad0/0x100677fc0`：internally tagged kind: missingAuth、http{status:u16}、network、timeout、invalidCatalog、tooLarge。
- handler C:1990附近：MissingAuth=>401，HTTP400..599保留，否则502，Timeout=>504，成功200。失败body code=`official_catalog_unavailable`，message=`Official model catalog refresh failed. No replacement catalog was returned.`。
- handler C:2140–2193 / ASM`0x100701414..0x10070146c`：返回时必须仍On非NoAccount、configuration generation一致；最新request sequence才能更新status。旧请求可返回自身结果，但不覆盖新观察。模式/代变化返回409。

## 生产实现与 Atelier 边界

`src-tauri/src/maintenance/diagnostics/official_catalog.rs` 完整实现实际HTTP、过滤/校验/状态和有序发布，不调用 app-server，不用Gateway raw model inventory冒充官方。生产endpoint固定，只有模块私有测试入口可替换loopback/timeout。

`Scope::native`只读实际RelayStateDocument；`Scope::gateway`只读实际Catalog与已选profile，API key模式不适用。Gateway配置scope hash显式包含host、settings、profiles配置以及已选profile，排除请求计数/lastError等运行态噪声。这是Atelier适配，不冒充原生revision算法；Native使用真实native_catalog_revision。

`OfficialCatalogRuntime` 不读取或写入用户目录/凭据。状态仅来自实际观测请求，未观测ready=false。Atelier新增本进程filtered JSON读回缓存，最近失败后不返回旧payload，跨配置scope清除。原生没有已证TTL，原文将有效目录缓存归ChatGPT管理；此处没有杜撰TTL或磁盘cache schema。Native诊断文案保留原所有权，Gateway成功文案明确本进程读回缓存。

调用方在真实配置发布/来源切换后调用 `configure(&Scope)` fence；`observe_request(&Scope,&HeaderMap,actual_client_version)`或原生HTTP适配的`observe_query`不读取配置、猜客户端版本或访问Keychain。缺真实client_version必须等待，不使用假默认。不同scope观察被拒，不能回滚已配置的运行态来源。

本模块只恢复官方目录获取/新鲜度；原生HTTP router将relay条目合并并生成etag的完整响应封装属于proxy宿主，不能把本模块filtered官方JSON当作已实现全部原生菜单。

## 验证

`official_catalog_tests.rs`：typed真实state模式、Gateway类型区分、完整header白名单、exact-slug过滤、loopback真实GET及query/etag/raw保留、缺身份无HTTP、无效版本/重复query、302不跳转、成功后失败不兜底、全无效dropped计数、advertised/chunked8MiB、真实网络失败/timeout、最新sequence及配置epoch fence。

仅synthetic headers、typed JSON和临时loopback。没有发现/启动系统app-server，没有读取真实auth.json、state、会话或调用真实Keychain。

正式验证：`cargo test --manifest-path src-tauri/Cargo.toml official_catalog -- --nocapture`，2026-11-08 session55792，exit0；14/14（本模块13项 + 已有native_orphans同名source-CAS回归1项），0.52s。编译有12项其它模块既有warning；本模块无编译错误。
