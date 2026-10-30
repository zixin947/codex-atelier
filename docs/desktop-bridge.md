# Desktop 插件开关与可选热重载

当前需求：在 Atelier 点击开关后，不重启 Desktop、不打开 Desktop 设置；新建对话加载最新配置，按宿主提示审阅并信任 Hook。旧对话不要求立即变化。实现已移除辅助功能控制路径，不需要系统辅助功能权限。

## 普通开关流程

1. 使用受支持 Desktop 安装包中的内置引擎读取插件和配置版本，不要求另外安装 CLI。
2. 点击开关后，通过短时 app-server 的 `config/value/write` 保存单个受管插件的 `enabled`，携带 `expectedVersion`，再读回实际状态。
3. 写入不依赖当前 Desktop 的共享宿主连接。私有 stdio、无控制入口或旧桥接失效不再导致普通开关变灰。
4. 用户在 Desktop **新建对话**加载最新配置，无需重启应用。恢复旧对话不等于新建对话；旧对话可能继续保留原先的插件和上下文。
5. 首次使用、定义变化时，由用户在 Desktop 中审阅并信任当前 Hook。相同定义已有信任不需要重复批准。关闭开关保留信任记录，重新打开也不会自动批准新定义。
6. 写入前仅保存原布尔值与配置版本。回退仍检查版本和插件来源；版本冲突、重复安装、项目或策略覆盖不能当作成功。

不会打开 Desktop 设置、发送聊天消息、申请辅助功能权限、修改安装包、自动信任 Hook 或重启 Desktop。用户级插件配置仍可能被使用相同 `CODEX_HOME` 的其他客户端共享。

产品当前只面向 Desktop；内置引擎通过系统或用户 Applications 中的 Codex / ChatGPT 包定位。macOS 应用包为 `src-tauri/target/release/bundle/macos/Codex Atelier.app`。

## 配置读回与执行证据

“已保存并读回”证明用户级开关保存成功，不等于真实 Desktop 的 Hook 已执行或旧对话已卸载。回执明确提示新建对话生效，不伪造实时控制连接。

此前使用 Desktop 内置引擎 `0.164.0-alpha.11.1` 的隔离实验：宿主进程保持运行，先启用并运行对照，再由另一个短时宿主保存停用配置；旧对话仍执行 3 次 Hook，同一宿主新建对话执行 0 次，新宿主新建对话也执行 0 次。见 [原始执行验证](hook-execution-verification.md)及[隔离证据](evidence/hook-execution-desktop-engine-20261027.json)。这是历史隔离引擎证据，不是本轮真实 Desktop UI 验收。

**本轮真实 Desktop 操作、开启后新对话发现、信任交互和实际 Hook 执行均留给用户测试。** 项目配置、管理策略或其他来源副本可能影响结果；已有上下文和正在运行的 Hook 无法撤回。

## 可选共享宿主热重载

已有兼容共享宿主时，沿用 `desktop_bridge.rs`：检查私有路径权限、macOS 对端 PID、Desktop 到该入口的内核端点对应、引擎身份及用户目录，再提交 `config/batchWrite` / `reloadUserConfig=true`。收到确认才显示覆盖会话数量。没有此入口时仍允许保存新对话配置，不能声称旧对话同步变化。

不提供连接卡片、配对弹窗或兼容启动按钮，不自动启动 daemon、启用远程控制或重启 Desktop。私有 stdio 的管道不被外部接管。历史桥接的失效记录仍拒绝建立控制连接，但不阻止新对话配置保存。

专用桥接启动代码仍供已有集成使用，不是本页开关的必要流程。连接记录只含协议版本、引擎 PID 和 socket 路径，目录 0700、文件和 socket 0600，不保存聊天、凭据、配置全文或信任哈希。

## AiMaMi 源码核对

核对 `borawong/AiMaMi` 提交 `add37271e29ba81ee17f15f444a24925af50bb87`，没有运行其程序：

- [process.rs](https://github.com/borawong/AiMaMi/blob/add37271e29ba81ee17f15f444a24925af50bb87/src-tauri/src/platform/process.rs#L62-L69)：macOS 以 `pgrep -x Codex` 判断运行状态；Windows 枚举进程并读取可执行文件路径。
- 同文件 [launch_codex_app](https://github.com/borawong/AiMaMi/blob/add37271e29ba81ee17f15f444a24925af50bb87/src-tauri/src/platform/process.rs#L454-L501)：启动时先使用 Bundle ID，再回退系统/用户 Applications 路径；重启是独立函数。不能为了自动检测而照搬启动命令。
- [paths.rs](https://github.com/borawong/AiMaMi/blob/add37271e29ba81ee17f15f444a24925af50bb87/src-tauri/src/platform/paths.rs#L91-L99)：使用 `CODEX_HOME` 或默认 `~/.codex` 定位本地资料。
- [repository.rs](https://github.com/borawong/AiMaMi/blob/add37271e29ba81ee17f15f444a24925af50bb87/src-tauri/src/core/repository.rs#L414-L437)：同步当前文件登录到账户目录。该同步不是当前 Desktop 会话的身份或控制权证明。

所核对的公开路径没有展示“发现进程后直接热切当前会话 Hook”的实现。公开 main 缺少若干被引用的核心模块，因此不对缺失部分下结论。Atelier 借鉴自动识别的产品流程，独立实现校验，不复制其退出、强杀、LaunchAgent 或后台任务安装逻辑。本机安装包的静态元信息显示可执行名称为 ChatGPT，故不能只硬编码旧名称 Codex。

## 验证与用户验收

当前验证记录见本轮交付。使用临时配置的后端往返测试检查开关、回退、版本冲突和信任独立性；前端 Mock 检查无宿主连接仍可切换、不再出现授权入口、读回回执和错误状态。编译打包与浏览器检查均不代替真实 Desktop 验收。

2026-10-29 本轮验证：

- `pnpm exec vitest run tests/plugins.test.ts`：16 项通过。
- `cargo test --manifest-path src-tauri/Cargo.toml plugins::tests::isolated_codex_roundtrip -- --ignored --exact --nocapture`：1 项通过，只使用临时 `CODEX_HOME`、本地测试插件，不运行模型或真实客户端。
- `pnpm tauri build --bundles app`：类型检查、前端构建、Rust release 编译与 macOS 打包通过（17.86 MiB）。
- Prettier、Rust 格式检查及 `git diff --check` 通过。
- ego-browser 离线 Mock：1280×800 和 1440×900 无横向溢出；共享宿主状态为 error 时仍可操作两个开关，无授权入口，保存后提示新建对话。截图：[1280](screenshots/desktop-new-conversation-mock-1280.png)、[1440](screenshots/desktop-new-conversation-mock-1440.png)。
- 已重开 Atelier，未操作、重启或替换真实 Codex Desktop；真实新对话和 Hook 执行由用户验收。

用户流程：在 Atelier 的「行为插件」切换 → 等待“已保存并读回” → 在 Desktop 新建对话 → 若出现新 Hook 审阅提示，核对来源后自行决定是否信任。关闭后同样新建对话；已批准且定义未变的 Hook 不需要反复信任。

参考：[OpenAI Hook 信任](https://learn.chatgpt.com/docs/hooks)、[配置接口](https://learn.chatgpt.com/docs/app-server)。
