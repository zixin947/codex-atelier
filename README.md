# Codex Atelier

Codex 的独立伴侣工作站。Tauri 2 + Rust、Vue 3 + TypeScript + Vite、Vue Router、Pinia。
当前为可运行的桌面前端原型，所有环境、模型、扩展、检查和备份信息都是**演示数据**。

## 运行

需要 Node.js 22.12+（本轮使用 24.16.0）、pnpm 10、稳定版 Rust，以及平台的 [Tauri 2 前置依赖](https://v2.tauri.app/start/prerequisites/)。macOS 需要 Xcode Command Line Tools。没有全局升级需求。

```sh
pnpm install --frozen-lockfile
pnpm tauri dev               # 原生桌面开发窗口
pnpm dev                     # 浏览器预览，http://127.0.0.1:1420
pnpm typecheck
pnpm test
pnpm build                   # 产物 dist/
cargo check --manifest-path src-tauri/Cargo.toml
cargo fmt --manifest-path src-tauri/Cargo.toml --check
pnpm tauri build             # 当前平台 .app（macOS）
pnpm tauri build --debug --bundles app # 本轮已验证的调试包
```

macOS 调试包：`src-tauri/target/debug/bundle/macos/Codex Atelier.app`。这是本地未签名的调试构建，尚非发布安装包。

## 当前范围

| 状态   | 内容                                                                                                                            |
| ------ | ------------------------------------------------------------------------------------------------------------------------------- |
| 已实现 | 原生窗口、路由、设计系统、主工作台、扩展搜索/分类/状态筛选/详情、开关、通知、设置弹窗、键盘焦点、检查筛选、差异预览、确认流程   |
| Mock   | 环境/连接/模型、7 个扩展、4 项检查、3 份备份、配置档案切换、修复/恢复。只修改内存，刷新复位；关闭演示交互会禁用模拟修改         |
| 未实现 | 真实账号/额度、Provider/代理接管、真实扩展安装执行、会话导入、人设/长期记忆/桌宠功能、真实扫描/备份/写入/回滚、持久化、深色皮肤 |

桌宠位置只是一个小展示占位。规划模块明确标注未实现。

## 结构与设计

- `src/app`：路由和 Pinia 应用状态。
- `src/features`：工作台、扩展、诊断、规划页。
- `src/shared/components`：统一几何图标、机壳面板、原生 dialog 弹窗。
- `src/services/mock.ts`：唯一数据适配器，不读取外部数据。
- `src/types/atelier.ts`：服务契约与未来配置事务边界。
- `src/styles/tokens/theme.css`：颜色、字体、间距、尺寸、边线、材质阴影、动效令牌；深色调色板只预留，不开放切换。
- `src/styles/main.css`：共享键帽、下沉屏、布局和控件状态；1440×900 / 1280×800 / 960×700 桌面适配。主内容独立滚动，导航和状态栏保持可见。
- `src-tauri`：最小 Rust 窗口启动。没有注册业务命令或特权插件。

视觉参考为 [MaiBot millennium 展示](https://github.com/Mai-with-u/MaiBot/blob/main/depends-data/webui-millennium-showcase.png) 与主题说明：仅借鉴米色机壳、压边键帽、下沉仪表屏和克制的薄荷绿语言。代码、图标、标识和装饰独立实现，没有复制其 GPLv3 Dashboard 代码、品牌、角色、字体或素材。系统字体保证中文正文清晰，等宽字用于仪表标签与数字。

## 安全边界

应用无遥测、自动登录、后台常驻或凭据验证。不读取或修改 auth.json、钥匙串、Codex 会话数据库、`~/.codex/config.toml`、代理、LaunchAgent、人设或记忆目录。不暴露任意 shell；Tauri capabilities 的权限列表为空，CSP 限制资源与连接。

`ConfigurationTransaction` 仅定义未来的读取快照 → 外部变更检测 → 差异预览 → 备份 → 明确确认 → 写入 → 校验 → 回滚接口，本轮没有生产实现。差异预览使用虚构路径与虚构配置，不代表真实 Codex 配置格式。模拟成功仅显示通知，不伪造真实修复或备份。

## 验收资料

截图与实际验证边界见 [docs/verification.md](docs/verification.md)，尺寸检查结果见 `docs/viewport-checks.json`。

![工作台](docs/screenshots/dashboard-1440.png)
![扩展](docs/screenshots/extensions-1440.png)
![诊断](docs/screenshots/diagnostics-1440.png)
![确认流程](docs/screenshots/confirmation-1440.png)
