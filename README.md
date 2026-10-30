<a id="codex-atelier"></a>

<div align="center">

<img src="docs/readme/hero.svg" width="100%" alt="Codex Atelier — 为专注，留一间工作室。Codex 独立伴侣工具，当前为演示原型。" />

**Codex Desktop 的独立伴侣工作站** · `v0.1.0` 交互原型

把复杂留给工作台，把专注留给你。

现阶段产品只面向 **Codex Desktop**；使用桌面应用自带的引擎，不要求另装 CLI。CLI 产品接入后置。用户级配置仍可能被其他使用相同 Codex 目录的客户端共享，不能把配置状态当成 Desktop 当前会话状态。

<p>
  <img src="https://img.shields.io/badge/Tauri-2-8fd6a0?style=flat-square&amp;labelColor=5a544a" alt="Tauri 2" />
  <img src="https://img.shields.io/badge/Vue-3-8fd6a0?style=flat-square&amp;labelColor=5a544a" alt="Vue 3" />
  <img src="https://img.shields.io/badge/TypeScript-strict-8fd6a0?style=flat-square&amp;labelColor=5a544a" alt="严格 TypeScript" />
  <img src="https://img.shields.io/badge/Status-Prototype-f08a5d?style=flat-square&amp;labelColor=5a544a" alt="当前阶段：原型" />
</p>

<p>
  <a href="#start"><img src="docs/readme/key-start.svg" width="184" alt="开始体验" /></a>
  <a href="#preview"><img src="docs/readme/key-preview.svg" width="184" alt="界面预览" /></a>
  <a href="#roadmap"><img src="docs/readme/key-roadmap.svg" width="184" alt="功能进度" /></a>
  <a href="#development"><img src="docs/readme/key-docs.svg" width="184" alt="开发文档" /></a>
</p>

</div>

> [!IMPORTANT]
> 当前主工作台、MCP/Skills 示例、诊断和备份仍为**演示数据**。「行为插件」页已接入本机 Ponytail / Stop That Shit 的固定版本安装、加载开关、回退与 Hook 信任只读检测；会话实际效果仍需在 Codex 中验证。新增 [Codex 账号管理](docs/codex-accounts.md)（添加/导入、额度、切换与批量管理）、「模型与连接」及本地网关实现，尚未进行真实运行验收，详见[多账号与网关说明](docs/local-gateway.md)。

<a id="about"></a>

## 简介

<img src="docs/readme/section-intro.svg" width="100%" alt="01 · 把环境收好，把专注留下 / ABOUT THE ATELIER" />

模型、Provider、MCP、Skills、会话、配置……随着 Codex 工作流逐渐丰富，环境管理也需要一个清楚、顺手的位置。

**Codex Atelier** 希望把这些外置能力放进同一间工作室：看清当前环境，整理扩展，理解配置变化，并在每次调整之前保留退路。它是独立伴侣工具，Codex 继续负责聊天与 Agent 执行。

- **一个有主次的工作台**：环境状态、当前连接、扩展概况与诊断摘要集中呈现，先看到值得关注的事。
- **两套可切换的界面**：终末地纸墨工业风与千禧机壳；在设置中即时切换，并记住外观偏好。
- **能亲手体验的原型**：搜索、筛选、详情、开关、弹窗和通知都可以操作，每处演示都有明确标识。
- **为真实接入留好边界**：页面与数据适配器分离，未来的配置变更需要经过预览、备份、确认与校验。

这是独立项目，与 OpenAI 无官方隶属关系。

<a id="preview"></a>

## 界面预览

<img src="docs/readme/section-preview.svg" width="100%" alt="02 · 走进工作间 / INTERFACE PREVIEW" />

### 行为插件 · 真实本机开关

在独立的「行为插件」页安装 Ponytail 5.3.2 与 Stop That Shit 0.2.5，并一键启停插件加载。安装默认停用，写入后读回验证；可回退上次切换，外部变更时停止覆盖。刷新会通过 Desktop 内置引擎只读查询 Hook 信任状态，区分已信任、待审阅、定义变化、未加载和读取失败，不自动授权。浏览器预览禁用真实操作。详见[行为插件使用与边界](docs/behavior-plugins.md)。

[Desktop 插件开关](docs/desktop-bridge.md)：开关在 Atelier 内后台保存并读回，普通启动无需控制宿主连接、无需辅助功能权限，也不会打开 Desktop 设置。切换后新建 Desktop 对话加载最新配置，无需重启应用；首次使用或定义变化时，按 Desktop 提示审阅并信任 Hook，已有信任保留。旧对话不保证同步变化；真实 Desktop 新对话的加载与执行仍由用户验收。

[插件模式控制](docs/plugin-modes.md)：Ponytail 可设置新对话默认 Lite / Full / Ultra / Off；STS 可选择已运行过该插件的对话，设置任务模式与 Watch / Guard / Lock / Off，支持文件边界、预览确认、版本冲突检测和上次模式回退。模式写入后读回，STS 使用固定上游的解析器与会话锁，保留委派账本。原始 Hook 已在临时目录中验证，真实 Desktop 由用户测试。

![插件模式面板，离线 Mock 界面验证，未连接真实 Desktop](docs/screenshots/plugin-modes-mock-1440.png)

![macOS 原生行为插件页：Ponytail 的三个 Hook 已信任，STS 停用且未枚举](docs/screenshots/behavior-plugins-trust-native.png)

### 终末地 · 野外终端

新增纸灰、墨黑与信号黄的工业风皮肤，直接移植原版动态等高线背景、鼠标扰动与 ENDFIELD 背景字标，保留原有页面和账号卡片排版。右上角 **皮肤** 按钮可直接切换“终末地 / 千禧机壳”；未保存偏好时默认野外终端。

终末地启动开屏默认开启：参考横版开屏构图，采用圆环仪表、横排 CODEX ATELIER 大标题、双侧状态编号与底部横向进度线，使用独立绘制的 Atelier 标识。在 **应用设置 → 启动加载动画** 可关闭或预览，支持 Esc 跳过和系统减少动效。百分比仅表示本地界面启动动画，不代表真实环境或账号检测。实现及许可见 [皮肤说明](docs/endfield-theme.md#启动开屏)。

![终末地风格主工作台，1440×900，浏览器示例数据预览](docs/screenshots/endfield/dashboard-1440.png)

[账号卡片与额度浮层](docs/endfield-theme.md#预览) · [皮肤选择](docs/screenshots/endfield/settings-1440.png) · [设计令牌与验证边界](docs/endfield-theme.md)

本组截图来自隔离浏览器，账号为无凭据示例数据。macOS `.app` 已打包，新皮肤的原生渲染留待用户验收。

### 千禧机壳 · 主工作台

深色仪表屏是视觉中心，连接与扩展紧随其后，诊断建议和快捷操作留在侧手边。截图中的状态全部来自 Mock adapter。

![Codex Atelier 主工作台，展示示例环境、连接档案、扩展与诊断摘要](docs/screenshots/dashboard-1440.png)

<details>
<summary><strong>扩展管理 · MCP 与 Skills</strong></summary>

按分类、关键词和启用状态查找扩展，在详情中查看示例工具与权限声明。启用或停用只改变本次运行的演示状态。

![扩展管理：搜索、分类、状态筛选与详情面板](docs/screenshots/extensions-1440.png)

</details>

<details>
<summary><strong>诊断与回滚 · 从问题到变更预览</strong></summary>

查看分类检查、问题说明和风险，再进入演示差异预览。修复与恢复均需明确确认，完成后只显示模拟通知。

![诊断页：分类检查、风险说明与备份档案](docs/screenshots/diagnostics-1440.png)

![变更确认：修改前后差异和明确确认门槛](docs/screenshots/confirmation-1440.png)

</details>

<details>
<summary><strong>原生窗口与较窄桌面窗口</strong></summary>

macOS Tauri 原生窗口已实际启动并验证。前端在 1440×900、1280×800、960×700 下检查过横向溢出；较小窗口使用内容滚动，保留桌面导航。

![macOS 原生 Tauri 工作台窗口](docs/screenshots/native-dashboard.png)

![1280×800 工作台](docs/screenshots/dashboard-1280.png)

</details>

<a id="start"></a>

## 开始体验

<img src="docs/readme/section-start.svg" width="100%" alt="03 · 从这里开始 / GET STARTED" />

当前通过源码运行。仓库包含前端与 Rust 锁文件；本地 macOS 调试包已验证，尚未提供正式签名安装包。

### 准备环境

- **Node.js 22.12+**，项目初始化与验收使用 `24.16.0`。
- **pnpm 10**，仓库声明版本为 `10.28.0`。
- **稳定版 Rust 与 Cargo**，以及对应平台的 [Tauri 2 前置依赖](https://v2.tauri.app/start/prerequisites/)。macOS 需要 Xcode Command Line Tools。

### 启动桌面应用

```sh
git clone https://github.com/zixin947/codex-atelier.git
cd codex-atelier
pnpm install --frozen-lockfile
pnpm tauri dev
```

只想预览前端，可运行 `pnpm dev`，然后访问 `http://127.0.0.1:1420`。浏览器预览与桌面验证是两项独立的检查。

### 构建

```sh
pnpm build                            # 前端产物：dist/
pnpm tauri build --debug --bundles app # 已验证的 macOS 本地调试包
pnpm tauri build                      # 发布模式构建，尚未完成发布验收
```

macOS 调试包位于 `src-tauri/target/debug/bundle/macos/Codex Atelier.app`。该产物未签名、未公证；Windows/Linux 构建尚未验证。当前仓库的 bundle 配置面向 macOS `.app`。

<a id="roadmap"></a>

## 功能进度

<img src="docs/readme/section-roadmap.svg" width="100%" alt="04 · 正在建造什么 / PROJECT ROADMAP" />

| 模块                  | 现在可以体验                                                  | 真实能力进度                                  |
| --------------------- | ------------------------------------------------------------- | --------------------------------------------- |
| 行为插件              | Ponytail / STS 安装、启停、模式选择、读回与回退               | 模式已通过隔离 Hook 验证；真实 Desktop 待验收 |
| 主工作台              | 环境、连接、扩展与诊断概览；演示工作配置切换                  | 数据全部为 Mock                               |
| MCP 与 Skills         | 分类、搜索、筛选、详情、演示开关                              | 未安装或执行真实扩展                          |
| 诊断与回滚            | 分类检查、风险说明、差异与恢复预览、确认流程                  | 未扫描、备份或修改本机                        |
| 账号与额度            | Codex 登录/导入、额度与订阅信息、当前文件核对、批量管理及回退 | 真实实现，未运行验收                          |
| 模型、Provider 与代理 | API Provider、本地网关与同类账号自动切换                      | 真实实现，未运行验收                          |
| 会话与历史            | 规划状态页                                                    | 尚未读取会话数据库                            |
| 人设、长期记忆与桌宠  | 规划状态页；小型桌宠展示占位                                  | 尚未实现                                      |

**演示状态仅保存在内存中，刷新或重启即可复位。** 设置中可关闭演示交互，禁用扩展开关与模拟确认。设置中可切换野外终端与千禧机壳两套浅色皮肤；深色皮肤仍未开放。

<a id="development"></a>

## 开发与文档

<img src="docs/readme/section-dev.svg" width="100%" alt="05 · 为下一步留好接口 / DEVELOPMENT" />

采用 **Tauri 2 + Rust、Vue 3 + TypeScript + Vite、Vue Router 与 Pinia**。Vue 使用 Composition API 与 `script setup`，TypeScript 开启严格检查。

```text
src/
├── app/                 路由与应用状态
├── features/            工作台、扩展、诊断与规划页
├── shared/components/   图标、面板和弹窗
├── services/            可替换的 Mock adapter
├── types/               服务与配置事务契约
└── styles/
    ├── tokens/          颜色、字体、间距、材质、尺寸与动效
    └── themes/          可切换皮肤与背景样式
src-tauri/               Rust 窗口与受限插件管理接口
tests/                   关键原型交互测试
```

| 资料                                    | 内容                                 |
| --------------------------------------- | ------------------------------------ |
| [设计决策](docs/design.md)              | 视觉方向、原型取舍、性能与维护假设   |
| [验收记录](docs/verification.md)        | 已执行的检查、原生验证与未验证项     |
| [设计令牌](src/styles/tokens/theme.css) | 机壳、面板、屏幕、字体和控件基础规范 |
| [野外终端皮肤](docs/endfield-theme.md)  | 令牌、切换、截图、来源与验证边界     |
| [Mock 服务](src/services/mock.ts)       | 示例扩展、检查、备份和变更预览       |
| [事务契约](src/types/atelier.ts)        | 未来真实配置修改必须保留的流程边界   |

### 验证命令

```sh
pnpm typecheck
pnpm test
pnpm build
pnpm format:check
cargo check --manifest-path src-tauri/Cargo.toml
cargo fmt --manifest-path src-tauri/Cargo.toml --check
```

项目初始化验收中，6 项交互测试、类型检查、前端构建、格式检查、Rust 检查及 macOS 调试打包均通过。这里记录的是已执行的本地验收结果，并非持续集成状态；完整限制见[验收记录](docs/verification.md)。

欢迎通过 [Issues](https://github.com/zixin947/codex-atelier/issues) 提交视觉建议、交互问题与功能讨论。反馈界面问题时，请附窗口尺寸、复现步骤和截图，并移除私人数据。

<a id="boundaries"></a>

## 设计与安全边界

<img src="docs/readme/section-safety.svg" width="100%" alt="06 · 每次变化，都应该清楚 / BOUNDARIES &amp; DESIGN" />

**先看清，再调整；每一步，都应该能说明白。**

当前应用没有遥测、自动登录或后台常驻服务。[Codex 账号管理](docs/codex-accounts.md)支持当前文件登录自动识别、独立登录、导入、额度和带预览/备份的文件凭据切换；进入账号页会只读检测有效 Codex 目录，将当前账号副本同步到 Atelier，目标 auth.json 写入仍需切换预览和明确确认。账号与网关模块仅在用户主动授权、同步模型或转发请求时联网，凭据保存于 Atelier 自己的受限本地文件，未加密；详见[存储与安全边界](docs/local-gateway.md#数据与安全边界)。行为插件页通过官方 Codex 接口读取插件状态、写入两个插件的启用字段；不会主动读取 `auth.json`、系统钥匙串或会话数据库，不接管系统代理、LaunchAgent、人设与记忆目录。

Rust 端提供受限的插件、账号与网关命令，通过固定官方接口管理各自能力，不暴露任意 shell 执行接口。用户启用额度自动刷新后，Rust 应用级任务在切换页面、最小化和隐藏窗口时继续查询，退出 Atelier 后停止；不安装独立后台服务。Tauri 主窗口仅开放后台状态事件的监听与取消监听，资源与连接受 CSP 约束。

账号的文件凭据切换已实现以下预览、备份与读回流程，但仍待用户运行验收；诊断页的其他配置修复仍为演示：

```text
读取现状 → 检测外部变更 → 预览差异 → 备份
                                     ↓
                       明确确认 → 写入 → 校验
                                         ↓
                                      必要时回滚
```

诊断演示页的路径与版本为示例；真实账号切换页会显示实际目标路径。不要将两者混淆。

<a id="acknowledgements"></a>

## 灵感与致谢

<img src="docs/readme/section-thanks.svg" width="100%" alt="07 · 灵感与致谢 / ACKNOWLEDGEMENTS" />

感谢 [MaiBot](https://github.com/Mai-with-u/MaiBot) 的 millennium「千禧主题」带来的视觉灵感，以及其 README 的展示组织方式。Atelier 借鉴米色机壳、下沉屏幕与键帽的视觉语言，独立实现界面、封面、图标和 README 素材；没有复制上游 Dashboard 代码、角色、品牌或整套素材。

新增终末地风格皮肤参考 [ymh0000123/dsh-theme-endfield](https://github.com/ymh0000123/dsh-theme-endfield) 的公开主题令牌与样式方向，适配 Atelier，并直接移植其原版等高线背景实现。保留上游 MIT 署名，详见 [第三方说明](THIRD_PARTY_NOTICES.md)。没有引入上游的宿主注入逻辑或游戏美术素材。

本项目使用的框架与工具包括 [Tauri](https://v2.tauri.app/)、[Vue](https://vuejs.org/)、[Vite](https://vite.dev/) 与 [Pinia](https://pinia.vuejs.org/)。第三方依赖遵循各自许可证。仓库目前尚未指定项目许可证。

<div align="center">

**MAKE ROOM FOR IDEAS.**

一间安静的工作室，一套清楚的工作流。

[回到顶部](#codex-atelier)

</div>
