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

## magpie 路由展示

来源：[yetone/magpie](https://github.com/yetone/magpie)，固定提交 `e6f151507f3c70f483d4bd476fa69faef155fe8c`。

直接移植 `internal/gui/assets/plugins/routing/routing.css` 的 stage、候选列表、计数及尝试日志样式片段，保留原注释；`client.js` 中对应 DOM 结构和交互适配为 `src/features/routing/Routing.vue`。移植范围限路由观测页面，不包括 magpie 的整个 GUI、路由组编辑器、网关策略或协议转换。配色改接 Atelier 令牌，品牌改为 Atelier，事件来自 Atelier 自身转发链路；没有调用 magpie 进程或读取其配置。

MIT 许可全文保留在 `src/features/routing/vendor/ATELIER-LICENSE.txt`，同时打包进页面的「来源与许可」。

```text
MIT License

Copyright (c) 2026 yetone

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
```

## magpie 协议与路由核心

`vendor/atelier` 固定于同一上游提交 `e6f151507f3c70f483d4bd476fa69faef155fe8c`，直接保留 gateway/provider/routes 的依赖闭包及上游测试；原始 GUI、API、供应商、账号和路由文件保持上游原样；先前的账号适配挂接已撤回。Atelier 仅增加宿主入口、本机认证传输、独立皮肤覆盖，以及暴露原有关闭顺序的 `AtelierShutdown` 方法。原 MIT 许可、上游第三方声明及来源清单分别保留在 `vendor/atelier/LICENSE`、`THIRD_PARTY_NOTICES`、`PROVENANCE.json`。核心已集成并同步本机安装版；应用资源中同时保留 MIT 许可与上游第三方声明。真实供应商、订阅与 Desktop 全链路尚未验收。
