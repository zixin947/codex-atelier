# magpie 原版模型区与路由引擎

## 当前范围

固定上游提交 `e6f151507f3c70f483d4bd476fa69faef155fe8c`。`vendor/magpie/PROVENANCE.json` 记录来源、归档校验与 92 个内部包；7879 个上游原文件保持原样，其中 GUI/API 包为 2324 个文件。新增的四个 Go 文件只负责宿主入口与关闭生命周期。

本次改动影响范围是模型区导航、原版页面皮肤、本机认证传输、引擎生命周期，以及 Desktop 启动桥和维护 RPC 兼容修正。

## 页面和后端边界

- 「模型与连接」打开原版 Agent 页；「供应商、路由、路由组、网关」分别打开原版对应视图。
- 原 HTML、JS、API 处理器、账号实现与路由算法均保持原样；没有将这些业务逻辑重写为 Vue/Rust。
- `native-theme.css` 覆盖终末地颜色、字体和控件材质。`native-content.css` 按用户要求，仅对四个模型页隐藏原版导航和侧栏占位；右侧内容顺序与控件布局不变。
- 已有 `magpie://` 本机传输向固定回环管理服务附加会话认证，避免 WKWebView 第三方 Cookie 限制。管理密钥不交给嵌入页，也无须用户手工输入。静态资源与业务请求沿用原 API，HTML 只追加样式标签。
- 原版模式启用原生账号发现，直接使用原供应商与路由组存储；启动不以 Atelier 的旧账号档案重建这些资料，也不走旧 Rust 订阅桥或 Image 请求准备接口。
- 旧连接页、Rust 网关及对应兼容命令仍保留在源码中，不是这五个侧栏入口的业务实现。早期路由设计和截图见 `routing-design.md`，仅作为历史资料。

## 协议与策略

原 `gateway.Server.Handler` 处理 Chat Completions、Responses/SSE/WebSocket、Anthropic、Gemini、Ollama、图像、音频、视频、嵌入、重排、Decisions、MCP、搜索及原生透传入口。

原有 12 种内置策略保留：`smart`、`manual`、`order`、`rotate`、`usage`、`pace`、`weight`、`free`、`fast`、`cheap`、`drain`、`expiring`。第三方插件不等于随包安装。

进程和推理监听由 Atelier 管理，只监听回环地址；推理密钥不开放原上游进程更新、密钥签发、额度重置或剖析接口。管理页面由单独的本机认证通道访问。

## 数据与构建

- 原引擎使用 Atelier 私密数据目录内 `gateway/routing-core` 的独立 portable root，不依赖安装 magpie 应用。
- 原版账号发现、刷新、Agent 连接/断开等操作遵循原后端行为；连接 Agent 会写入相应 Agent 配置，应按原页面提示操作。
- 迁入原版模式时，已备份清理早期宿主生成的一条 `atelier-managed-codex` 临时供应商引用。未批量迁移真实线程，也未执行真实账号切换。
- 构建需要 Go 1.26.8、Rust/Tauri 工具链；macOS 完整 GUI 包还需要系统 SDK/原生链接环境。可通过 `GO_BIN` 指定 Go；模块版本与校验由原 `go.mod`/`go.sum` 固定。

```sh
GO_BIN=/path/to/go pnpm tauri build --bundles app
```

`beforeBuildCommand` 先执行 `scripts/build-router.mjs`，再构建前端。产物 `src-tauri/binaries/atelier-router` 不纳入 Git，打包时复制为应用资源；同时打包原 MIT 许可和上游第三方声明。

本机安装版已同步并保留另一条工作线的桌宠功能；桌宠集成不包含在本 PR 中。

## Desktop 与维护修正

- 通过网关启动 Desktop 时保留现有提供商 ID（含命名 profile），不再强制改成 `atelier_gateway`；当前 CLI 点分覆盖只接受字母、数字、下划线、连字符的 ID，其他 ID 会明确拒绝。
- 对启动的 Desktop 及其子进程追加回环地址到 `NO_PROXY`/`no_proxy`，不修改系统代理。
- 维护 RPC 初始化补齐必填 `version`；官方目录识别允许经过端点、认证和 Header 检查的官方连接别名。

## 验证与限制

已执行原资源/POST API 自动认证、样式注入保持原结构、原版启动保留供应商/路由组文件的测试；本地安装版已逐页确认四个入口、原版侧栏隐藏和内容显示。具体提交前验证结果见 PR。

历史筛选测试覆盖多个上游协议转换与路由策略，但没有执行上游全量测试，亦没有主动验收原生 WebSocket 握手。隔离 CLI 连通探针曾到达本机假上游，因假 SSE 缺少 `total_tokens` 而退出，不能作为完整推理成功证据。

真实供应商推理、真实令牌过期后的在线刷新、账号切换/Agent 接管写入、实际修复票据应用、计费/额度行为和 Desktop 长会话均未完整验收。构建或源码一致不代表这些场景已通过。

## 回退

停止网关、退出 Atelier，恢复此前应用包即可回退程序。保留 `gateway/routing-core` 目录及其备份；不要删除凭据或直接覆盖 Agent 配置。已由原版页面写入的 Agent 配置，应使用其原断开/恢复入口或已保存备份恢复。此 PR 不合入桌宠或其他开放 PR，也不包含远端发布。
