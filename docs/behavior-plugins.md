# 行为插件：Ponytail 与 Stop That Shit

Atelier 的「行为插件」页是实际本机管理功能，与其他页面的 Mock 开关独立。浏览器仅预览界面，不伪造本机安装状态或提供可操作的真实开关。

![macOS 原生窗口，Ponytail 与 STS 已安装且停用](screenshots/behavior-plugins-native.png)

## 使用

1. 打开 Tauri 桌面应用，进入侧栏「行为插件」。需要本机 Codex CLI 与 Git。
2. 点击「安装固定版本」，查看来源及版本，再安装。新安装默认停用。
3. 点击对应加载开关。界面仅在 Codex 写入并读回有效状态后显示已启用/停用。
4. 新建 Codex 会话，在 CLI 的 `/hooks` 中按宿主提示自行审阅、信任上游 Hook 定义。Atelier 不替你接受信任，也不声称知道当前信任状态。
5. 新建 Codex 会话验证插件加载。关闭不会删除当前聊天已注入的规则，也不会向聊天发送命令。

Ponytail 的 lite/full/ultra 与 STS 的 review/change/watch/off 是插件自己的会话模式。本页开关管理**插件是否加载**，没有替代会话模式选择。STS 插件启用后仍需要明确任务指令才能进入 Guard 模式。

## 固定来源

| 插件           | 官方仓库                | 固定版本 | 固定提交                                 |
| -------------- | ----------------------- | -------- | ---------------------------------------- |
| Ponytail       | DietrichGebert/ponytail | v5.3.2   | 16352ac229eb08a9598dcae48fe9f2b342567d86 |
| Stop That Shit | lennney/stop-that-shit  | 0.2.5    | 8bc1ac558167121e7f28d9fb0bec05c039952206 |

来源保存在 `integrations/behavior-plugins.json`。安装下载官方 GitHub 源码至 Atelier 应用数据目录，检查 Git 提交、工作树与插件声明，再通过 Codex CLI 注册 `atelier-companions` 本地市场并安装。下载仓库中保留上游 MIT 许可证；上游实现没有复制进 Atelier 源码，也没有重新包装成自己的规则引擎。

已有官方市场安装时优先管理它，不强制重装或降级；同时存在官方与 Atelier 来源时禁用切换，要求用户在 Codex 中保留一个来源，以免同名规则重复加载。其他市场中名字相似的插件不在管理范围内。

## 写入与回退

- 只暴露四个受限 Tauri 命令：状态、安装、启停、回退。插件参数是固定枚举；不接收 shell、任意命令、任意仓库或文件路径。
- Codex 可执行文件从 PATH、常见 Homebrew 路径或 nvm 安装中发现，Finder 启动时补齐对应 Node 的 PATH。
- 状态使用 `codex plugin list --json` 及短时 stdio app-server 的 `config/read`。只向前端返回这两个插件的状态和版本，不返回完整配置内容。
- 启停通过官方 `config/value/write` 写入单个 `plugins."<id>".enabled`。携带配置版本令牌；过期状态停止写入，不覆盖外部变更。
- 写入前保存原启用值和官方配置版本；成功后保存写入版本。回退仅还原该布尔值，不复制整份配置或凭据。记录存放应用数据目录，macOS 新文件权限为 0600。
- 回退仅在配置版本和插件来源仍匹配时可用；有后续外部变更时，刷新并显式切换，不覆盖旧快照。
- Codex 用户级配置可能被项目或管理策略覆盖；写入返回 `okOverridden` 或读回不匹配会显示错误，不能冒充会话生效。
- 每次操作结束会回收短时子进程；没有常驻 daemon 或 WebSocket 监听。下载需要 GitHub 网络，启停不请求模型。

Atelier 不主动读取 auth.json、钥匙串或会话数据库，不接管 Provider、代理、LaunchAgent、人设和记忆。Codex 官方子进程按其自身机制加载必要配置；Atelier 不展示、复制或记录其中的凭据。这里授权范围由用户本轮要求扩展到两个插件的安装与启用配置，其他模块仍保留演示边界。

## 验证

```sh
pnpm test
pnpm build
cargo test --manifest-path src-tauri/Cargo.toml
cargo test --manifest-path src-tauri/Cargo.toml -- --include-ignored
```

最后一条包含真实 Codex CLI 的隔离测试：使用临时 CODEX_HOME、本地无 Hook 的测试插件，检查两个插件的状态读回、切换、回退、过期版本拒绝、无关配置和注释保留。不会使用真实账号、运行模型或信任 Hook。

当前支持验收的宿主是 macOS + Codex CLI 0.164.0。Windows/Linux 可执行文件定位、其他 CLI 版本、Hook 信任和真实模型任务行为尚未验收。

## 参考

- [Codex CLI 插件管理](https://learn.chatgpt.com/docs/cli/reference)
- [Codex 配置接口](https://learn.chatgpt.com/docs/app-server)
- [Codex Hook 信任](https://learn.chatgpt.com/docs/hooks)
- [Ponytail](https://github.com/DietrichGebert/ponytail)
- [Stop That Shit](https://github.com/lennney/stop-that-shit)

## 2026-10-26 实际验收

- 前端：11 项测试通过，覆盖原有原型与新增真实插件页；类型检查、前端构建与格式检查通过。
- Rust：4 项测试通过（含真实 Codex CLI 的隔离配置测试），Rust 检查和格式检查通过。
- macOS 原生窗口：通过页面安装了两个固定版本，Ponytail 5.3.2 与 STS 0.2.5 的安装版本已由 Codex CLI 读回。
- 原生操作：Ponytail 停用→启用→回退停用；STS 停用→启用→停用，均观察到读回状态变化。最后两者均停用。
- 浏览器：1440×900、1280×800、960×700 无横向溢出；真实操作全部 disabled，未伪造已安装状态。
- 视觉修正：将加载开关与主要操作前移到描述下方，减少寻找开关的滚动距离。
- `pnpm tauri build --debug --bundles app` 通过，生成可启动的 macOS `.app`；本地调试构建尚未签名、公证或验证跨机器分发。
- 未确认 Hook 信任、未启动模型验证插件实际行为；没有声称当前聊天已受这些规则影响。
