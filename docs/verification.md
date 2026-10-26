# 本轮验收记录

日期：2026-10-25（Asia/Shanghai）。以下为初始化阶段的历史验收。2026-10-26 新增 Ponytail / STS 本机管理的最新范围、操作与验证见 [行为插件验收](behavior-plugins.md)。

## 环境

- Node 24.16.0、pnpm 10.28.0、Rust/Cargo 1.99.0。
- Apple Command Line Tools 可用。
- 实际锁定 Vue 3.5.44、Vite 8.3.5、Tauri CLI 2.12.4 / Rust 2.12.5、Vue Router 4.6.4、Pinia 4.1.0。
- 依赖安装曾遇 npm ECONNRESET，自动重试后成功。补齐 Pinia 所需 devtools peer，无全局升级。

## 自动验证

- `pnpm typecheck`：通过，严格 TypeScript。
- `pnpm build`：通过，生成 dist。
- `pnpm test`：6 项通过：适配器隔离、未知扩展拒绝、问题专属差异、搜索/筛选/开关/分类、确认门槛与模拟通知、加载失败重试、设置禁用边界（部分场景同一测试覆盖）。
- `pnpm format:check`：通过。格式化曾破坏一处模板多语句处理，已提取独立函数并重新通过测试/构建。
- `cargo check --manifest-path src-tauri/Cargo.toml`：通过。
- `cargo fmt --manifest-path src-tauri/Cargo.toml --check`：通过。
- `pnpm tauri build --debug --bundles app`：通过，生成本地 macOS 调试 `.app`。
- 初次 Rust 检查发现缺少默认图标；已补独立生成的 RGBA 图标，重新检查和原生构建通过。

## 浏览器与视觉检查

使用 Ego Lite 实际渲染，使用 DOM 返回的 `innerWidth/innerHeight` 校准到目标 CSS 尺寸，再通过 CDP 获取截图。浏览器缩放不是 100%，截图使用补偿后的设备参数；尺寸以页面实测值为准。

- 工作台、扩展、诊断各自检查 1440×900、1280×800、960×700；九组均无整页或主内容横向溢出。结果见 `viewport-checks.json`。
- 内容纵向滚动属于设计行为，侧栏、顶栏、状态栏固定保留；较小窗口不折叠为手机导航。
- 实际点击验证：扩展导航、搜索 GitHub、空状态重置、停用筛选、分类切换、详情与状态开关、通知、修复预览、确认勾选、模拟完成、规划页。
- 实际键盘验证：Tab 焦点留在原生 dialog 内；Escape 关闭并返回触发按钮（浏览器验证）；关闭演示交互后扩展开关 disabled。
- 截图审阅后修正：辅助文字大小与对比度、仪表内部留白、控件间距、桌面独立滚动层级。
- 图标为自绘统一 SVG 线条；中文为系统字体；无远程字体、图库或上游素材。

## 原生桌面验证

成功启动 `Codex Atelier.app`，通过 macOS 原生可访问性树确认 `tauri://localhost` 正在渲染主工作台，示例扩展为 05/07。实际点击原生窗口的扩展导航，确认到达 `tauri://localhost#/extensions` 并显示列表、详情与开关。随后在最终原生构建中点击诊断修复预览，确认未勾选时按钮禁用，勾选后完成模拟，并观察到明确的“未写入配置”通知。原生截图为 `screenshots/native-dashboard.png` 和 `screenshots/native-confirmation.png`。

## 未验证与限制

- 未验证 Windows/Linux 构建或原生交互。
- 未进行发布签名、公证、DMG 安装、自动更新、性能压力测试或完整辅助技术审计。
- reduced-motion 已实现 CSS 分支，未开展多系统辅助功能完整验收。
- 真实账号、额度、Provider、代理、MCP 执行、数据库、配置接管、备份和回滚均未实现，也没有进行这些真实操作。
- 深色调色板仅预留，没有声称双主题完成。
- Ego Lite 提示存在可用更新，本轮未升级浏览器工具。
