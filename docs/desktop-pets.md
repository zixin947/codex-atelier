# 桌宠管理

本次改动影响范围是桌宠管理模块，以及侧栏导航、路由与应用资源打包。

## 决定与范围

- 用户确认采用独立「桌宠」管理页，桌宠继续作为透明浮窗运行。
- 比较过页面嵌入；选择复用上游浮窗，保留其互动、窗口与设置行为。
- 单机使用，状态按需查询，不轮询、不自动安装；安装期间显示忙碌状态。
- 采用内置 v0.3.0 运行时，通过固定操作调用上游脚本，不提供任意命令入口。
- 默认由用户手动启动；上游 macOS 安装器本身会注册 LaunchAgent，安装时按需展示提醒并由用户确认。
- 数据由上游保存于有效 CODEX_HOME 下的 whale-widget，沿用现有配置与账本。Atelier 不回传 IPC 令牌、账户信息或安装日志。

## 使用

在 macOS Atelier 侧栏选择「桌宠」。首次安装需 Node.js 24+、npm、Xcode Command Line Tools 和网络。点击卡片中的安装入口，查看简短提醒后点击「确认安装」。安装可能持续数分钟。

「独立桌面」「跟随 Codex」「显示桌宠 / 设置入口」使用上游控制接口。显示后从托盘或人物菜单进入设置。「停止桌宠」调用上游停止接口，保留数据。运行状态表示 IPC 服务可达，不承诺窗口当前可见。

浏览器仅提供预览，禁用本机操作。当前接入仅支持 macOS；未新增 Codex 技能或 MCP 注册。

## 兼容与回退

小龙娘与旧小鲸鱼共用 api-balance-whale 标识及数据目录，安装会更新共用的跟随组件。上游安装器创建备份与回执，回退使用上游 scripts/rollback-macos.mjs；不要删除旧程序目录或私有回执。删除 Atelier 页面不等于卸载后台服务。

上游来源、固定 revision 和许可见 vendor/gpt-dragon-girl/ATELIER-UPSTREAM.json、LICENSE、PROVENANCE.md 及 THIRD_PARTY_NOTICES.md。macOS 安装脚本包含本地适配，具体改动见 ATELIER-UPSTREAM.json；后续升级应整体核对上游接口与许可。

## 初次接入验证

- `pnpm test -- tests/desktop-pets.test.ts`：Vitest 实际执行全部 11 个测试文件，共 97 项通过，含新增的两项桌宠交互测试。
- `pnpm build`、`cargo check`、`pnpm tauri build --bundles app`：通过。现有编译警告仍在。
- `git diff --check`：通过。
- 浏览器预览：立绘资源可加载，无横向溢出，本机按钮禁用。
- 已同步并重启本机 release 应用，在真实原生页面确认「桌宠」栏目与 Node 状态查询正常，当前显示「待安装组件」。截图：screenshots/desktop-pets-native.png。
- 初次交付时未执行上游安装器；后续实际安装验证见下方修复记录。

## 安装故障修复

原 npm 依赖安装成功，但 Electron 默认下载缺少明确超时和界面进度，GitHub 下载连接不稳定时会长时间等待。macOS 下载现使用系统 curl，连接超时 15 秒、下载上限 120 秒，沿用 @electron/get 的 SHA-256 校验；默认使用 npmmirror 的 Electron 镜像，保留 ELECTRON_MIRROR 覆盖。解压使用系统 ditto。安装阶段通过固定文案事件显示，不向界面发送原始安装日志。

桌宠运行文件安装到 Atelier 数据目录中固定版本的子目录，避免共享构建输出或应用更新覆盖正在使用的资源。应用另存至用户 Applications/Codex Atelier.app。

本机实际验证：Electron 44.3.0 归档校验成功、安装器完成签名/探针编译/LaunchAgent 注册；独立桌面调用返回成功，IPC 服务状态为运行中。余额及多屏跟随未验证。

修复验证：两项桌宠交互测试通过，前端构建与 macOS release 打包通过，原生应用显示“服务运行中”，已在 Electron 原生窗口看到小龙娘及其面板。修复后的应用已打开，位于用户 Applications/Codex Atelier.app。

## 订阅识别修复

原实现要求 provider ID 必须是 openai，导致自定义 provider 名称但仍直连官方 API、使用 ChatGPT 登录的配置被误判。现改为核对官方 HTTPS origin、无 API key 及 ChatGPT 登录。隔离验证覆盖自定义名称直连官方、第三方地址和 API key 三种真实边界，均通过。本机读取到一份有效周额度快照；未读取到有效 5 小时快照时仍显示未观测，不推算或补造。已同步运行目录与用户 Applications 内的应用资源并重启桌宠。

## 复用 Atelier 账号与额度

本次改动影响范围是桌宠额度交接与 Accounts 的只读额度导出模块。Accounts 继续负责当前文件账号识别、官方额度查询及后台刷新，桌宠不再独立认定订阅账号。

账号模块将当前账号的窗口百分比、重置时间、查询时间及过期状态原子写入私有交接文件。文件不包含账号 ID、邮箱、账户名或凭据；仅附带当前 auth 文件的单向摘要用于防止切换账号后串用旧额度。桌宠在持久化读取边界验证摘要与窗口字段；过期保留明确旧快照状态，来源不匹配时不回退到其他账号。

路径通过安装环境 ATELIER_QUOTA_FILE 传给 LaunchAgent 与桌宠，桌宠不读取 Atelier 账号目录。独立桌宠仍可在 Atelier 退出后显示最后一次额度，但超时后标记旧数据；自动更新沿用 Atelier 原有后台刷新设置。桌宠页「刷新状态与额度」调用 Accounts 当前账号识别与额度刷新，常规状态查询仅发布现有结果。

运行文件升级为 atelier3 目录，旧目录保留供上游回退。只更新交接与安装接口，API 余额和自定义互动保持上游行为。

统一额度接入验证：3 项桌宠前端交互测试通过；Rust 额度导出测试通过（含私有字段排除与账号变化清空）；3 项消费者验证通过（双窗口、身份变化拒用、过期标记）；前端构建与 macOS release 打包通过。原生点击「刷新状态与额度」成功调用 Accounts 查询，本机返回一个每周窗口、secondary 为 null；5 小时窗口未由官方接口提供，桌宠不推算。本机消费者已确认 source=atelier，已更新用户 Applications 内应用和 atelier3 桌宠服务。

## PR 提交前验证

- `pnpm test`：12 个测试文件、101 项测试通过，包含桌宠交互与额度交接回归测试。
- `pnpm build`：通过。
- `cargo test --manifest-path src-tauri/Cargo.toml accounts::`：27 项通过，1 项原有忽略项。
- `git diff --cached --check`：通过。
- 未验证：Windows/Linux 接入、真实 API 余额、多屏与睡眠唤醒、上游回滚的原生完整流程。

## 控制卡精简

桌宠页改为单张小龙娘控制卡，集中显示立绘、版本、状态以及独立桌面、跟随、显示、停止、刷新操作。移除常驻的组件依赖、兼容长说明及来源数据面板；相关资料保留在本文档与上游许可文件。安装提醒和确认按钮只在展开安装入口后出现。

本次改动影响范围是桌宠页面模块。3 项交互测试及原生打包通过，已同步本机应用并确认单卡片布局。截图见 screenshots/desktop-pets-card-native.png。
