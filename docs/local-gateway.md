# 本地网关与多账号管理

此模块提供真实实现，但本轮仅完成静态检查与编译。遵照用户要求，没有启动网关、执行登录、发送推理请求或连接真实 Desktop；下面的运行流程由用户手动验收。

## 页面分工

- **模型与连接 → 管理网关官方授权**：通过官方 Sign in with ChatGPT 授权添加订阅账号，重新授权、同步可用模型、选择当前账号、设置候选池和优先级、退出授权及移除档案。
- **模型与连接**：添加 OpenAI Responses 兼容的 API Key / 第三方中转 Provider，编辑地址与模型目录，管理本地网关和自动切换。
- 浏览器预览不读取本机账号，也不提供模拟成功的账号操作。真实能力仅在 Tauri 应用中可用。

不展示推算的套餐余额。请求与 Token 统计仅覆盖本次 Atelier 运行；套餐用量前往 [ChatGPT 官方用量页](https://chatgpt.com/settings/usage) 查看。

## 手动使用

1. 在网关官方授权子页面添加订阅账号，点击“打开官方授权页面”，完成官方授权，再点击“同步模型”；或者在连接页添加 Provider，填写含 API 路径的 Base URL、API Key 和模型 ID。
2. 选择当前账号，在连接页选择该账号支持的模型，保存路由设置。修改已有 Provider 地址或凭据前需先停止网关。
3. 手动启动网关，默认地址为 `http://127.0.0.1:18910/v1`。地址与端口以页面显示为准。
4. 使用“接入配置预览”查看 Codex Provider 配置。网关访问密钥只在主动显示时返回前端；每次重新启动网关都会更换。
5. Desktop 可通过“通过网关启动 Desktop”连接。必须先由用户退出已有 Desktop；Atelier 不会强制关闭它。启动只向新进程传递 Provider、模型和本地密钥，不写入 Codex 的持久配置。
6. CLI 可由用户参照预览配置手动连接。Atelier 本轮不自动改写 CLI 配置，也不控制正在运行的 CLI 会话。

网关依附于 Atelier 进程；退出 Atelier 后不能继续转发。停止网关会停止接受新请求并等待已有请求结束。重新启动网关后，客户端需要更新本地密钥或重新通过 Atelier 启动。

## 自动切换的实际范围

默认关闭自动切换。开启后，以当前账号所属类型为候选池，优先当前账号，其余按优先级数值从小到大选择。候选账号必须启用、授权可用、未处于冷却期且包含请求的模型。每次最多尝试 1–5 个账号。

- 订阅账号与 API Key 账号不自动互换，避免静默改变计费方式。
- 上游明确返回 HTTP 401 或 429、尚未开始输出且上下文未绑定账号时，才尝试下一候选。
- 网络失败、执行状态未知、已开始的流式响应或流内错误不会自动重放。
- 授权刷新失败会明确返回错误，不承诺所有失败都能无感切换。
- 429 根据 Retry-After 冷却；套餐共享额度耗尽会暂停该账号，需要用户后续手动重新启用。不能由此推断整个套餐已经耗尽。
- 前序响应与加密上下文绑定原账号；归属未知或原账号不可用时拒绝跨账号续传。

## 协议与兼容限制

本地入口提供 Responses HTTP 转发、模型目录与 API Key 通道的 compact 转发；不提供 Chat Completions、Claude 或 Gemini 原生协议转换，第三方中转必须兼容 Responses。

订阅通道使用官方 Sign in with ChatGPT 的 `resource.invoke` / `chatgpt.tokens.use.direct` 授权，推理地址为公开 `https://api.openai.com/v1/responses`。它不复用 Codex 登录文件，不调用 ChatGPT 私有 backend-api。

订阅通道要求完整 input 数组、stream=true、store=false；不支持 HTTP previous_response_id、compact 和官方尚未开放的字段或托管工具。命名空间中的 function/custom 与受支持的网页搜索按官方能力约束处理。通用 Codex Desktop 的具体请求能否满足该订阅接口，**仍待用户实际验收**，不能仅凭启动参数认定完全兼容。

网关不支持 WebSocket；生成的 Provider 配置显式关闭 WebSocket。只有 response.completed 被记为成功，HTTP 200 本身不等于推理完成。

官方参考：

- [开源应用接入](https://developers.openai.com/siwc/token-sharing-open-source)
- [登录流程](https://developers.openai.com/siwc/sign-in)
- [账号与会话](https://developers.openai.com/siwc/profiles-and-sessions)
- [模型与推理](https://developers.openai.com/siwc/models-and-inference)
- [预览限制](https://developers.openai.com/siwc/preview-limitations)
- [错误与恢复](https://developers.openai.com/siwc/errors-and-recovery)

## 数据与安全边界

数据仅写入 Tauri 的应用数据目录下 gateway 子目录，macOS 通常为 `~/Library/Application Support/com.codexatelier.desktop/gateway`。

- catalog.json 保存账号名称、邮箱、模型目录、路由设置和可公开状态。
- credentials 保存该应用自己的授权令牌或 API Key。当前使用 Unix 0700 目录与 0600 文件权限，**不是加密保险库**，没有接入系统钥匙串；非 Unix 环境拒绝使用此存储实现。
- 凭据整组原子替换，账号级跨进程锁保护刷新令牌轮换；目录修订号用于拒绝覆盖外部变更。凭据文件和目录文件不是单一数据库事务，写入失败需刷新并核对状态，必要时重新授权。
- OAuth 使用随机 state/nonce、PKCE S256、本地回环回调和 ID Token 签名、issuer、audience、期限与身份校验。授权只在用户点击时发起；刷新只在用户发起的模型同步或网关请求需要时执行。
- 本地网关仅绑定 IPv4 loopback，转发需要本地 Bearer 密钥，拒绝带 Origin 的浏览器请求，限制请求大小和并发数；上游不跟随重定向。
- 不保存请求正文和推理输出。页面路由记录仅包含账号、时间、状态、错误码与尝试次数。
- 移除账号只删除 Atelier 本地资料；“退出授权”会尝试官方撤销并清除本地令牌，远端撤销未确认时显示提醒。
- 网关模块不读取或改写现有 Codex auth.json/config.toml、会话数据库、系统代理、LaunchAgent 或记忆目录。无遥测、无开机自启、无后台常驻服务。

## 代码位置

- `src/features/accounts`、`src/features/connections`：独立管理页面。
- `src/services/gateway.ts`、`src/types/gateway.ts`：类型明确的前后端接口。
- `src/styles/gateway.css`：沿用现有机壳、面板和屏幕令牌。
- `src-tauri/src/gateway`：OAuth、受限存储、网关转发与 Tauri 命令。
- `src-tauri/src/desktop_bridge.rs`：用户主动启动 Desktop 时的进程级连接覆盖。

## 用户验收清单（尚未执行）

- 官方授权、取消、过期、重新授权、身份不一致与退出授权。
- Provider 编辑、修订冲突、模型同步与错误提示。
- HTTP/SSE 转发、取消请求、流内失败、401/429 切换与不重放行为。
- 当前账号类型隔离、加密上下文与 previous_response_id 归属约束。
- Desktop 手动启动、模型调用、工具调用及长对话兼容性。
- 1440×900、1280×800 和较窄桌面窗口的渲染、焦点、弹窗与滚动。

本轮没有运行上述测试，没有新增实际渲染截图；历史截图不能代表新增页面。

## 本轮编译记录

- `pnpm build`：TypeScript 检查与 Vite 生产构建通过。
- `cargo check --manifest-path src-tauri/Cargo.toml`：通过。
- `pnpm tauri build --bundles app`：macOS release 构建与应用打包通过。
- `cargo fmt --manifest-path src-tauri/Cargo.toml --check`、新增/修改前端与文档的 Prettier 检查、`git diff --check`：通过。
- 未运行任何测试、应用或客户端。上述结果只证明类型、编译、格式与打包，不证明运行行为。
- 本地产物：`src-tauri/target/release/bundle/macos/Codex Atelier.app`。

普通 Codex 登录、额度与文件凭据切换是独立模块，详见 [Codex 账号管理](codex-accounts.md)；其明确确认后的 auth.json 写入边界不等于网关自动接管。
