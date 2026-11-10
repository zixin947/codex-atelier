# 第三方说明

## 终末地风格的视觉参考

“终末地 · 野外终端”皮肤参考了公开的终末地风格界面的视觉语言（纸墨表面、信号色、测绘网格、深色仪表），并由本项目独立实现：语义词令牌、组件样式、原创背景渲染器和原创开屏动画均为本项目自行编写，不包含任何上游代码、游戏素材、官方 Logo 或字标。

设计规范的整理与色板建议来自仓库内的 `skills/endfield-ui-design/`，该目录下的说明与示例为独立编写的设计参考，不是官方 UI 规范；其中的研究截图仅作研究用途，不进入构建产物。

本项目早先版本曾移植 `ymh0000123/dsh-theme-endfield`（MIT）的等高线运行时、开屏时序与 ENDFIELD 字标样式，并引用由 OpenAI Blossom 标志演绎的像素铭牌。这些内容已在本次前端重写中全部移除：

- `src/shared/background/vendor/` 下的等高线运行时及其 Worker 源片段已删除，替换为原创测绘网格渲染器 `src/shared/background/atelier-background.ts`。
- 开屏动画重写为原创实现，保留原有的本地初始化衔接与可跳过行为，不再使用上游时序或字标。
- `src/assets/branding/codex-pixel-nameplate.png` 已删除，开屏改用本项目绘制的 SVG 标记。

上游许可仅覆盖其原创代码；游戏名称、商标及截图等不因该许可获得授权。若后续重新引入任何第三方代码，应在此处恢复相应许可全文。

## 字体

界面使用系统字体栈（`PingFang SC`、`Microsoft YaHei`、`system-ui` 与平台等宽字体），不嵌入或分发第三方字体文件。

## cockpit-session-compat

`src-tauri/src/` 下的 cockpit 兼容层来自 cockpit-session-compat crate，采用 CC BY-NC-SA 4.0 许可，仅用于本地线程工作区的兼容处理。该部分不随本次前端重写变动；使用与分发须遵守其非商业与相同方式共享条款。
