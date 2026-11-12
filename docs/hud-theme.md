# 环形 HUD 皮肤

本次改动影响范围是界面皮肤、导航外壳和嵌入页面的样式选择模块。

## 范围与决定

- 新增 `hud`，与终末地、千禧机壳并列；沿用原有偏好存储，默认选择不变。
- 首页以 14 个现有模块组成环形入口。点击主工作台也能展开；返回环形概览时收回导航。
- 功能页使用左侧导航与宽正文面板。主工作台保留概览、请求、会话、上下文四个目录。
- 同一个 `RouterView` 始终承载业务页面。皮肤切换与展开本身不会重建页面，保留表单输入与当前路由。
- 正文保持正视、正常字号与滚动；最小原生窗口为 960×700。小于 760px 时导航改为紧凑网格。
- 不增加接口或依赖。环形导航只接收现有模块数据及展开状态，业务状态仍由原页面维护。
- 沿用已提交的 `e1d0b5c` 基线，保留本机现有的 ChatGPT 用量工作台与品牌更新。

## EndfieldHUD 复用

参考提交 `19de06a45a6fb701cac1be4a1571111a8813b798`：

- `Sources/HUDDesignTokens.swift`：银灰面板色、黄色渐变、标准缓动、悬停 220ms 与按压 100ms。
- `Sources/HUDDesignButtons.swift`：渐变面板、轻微悬停抬升和按压反馈，适配为 Vue/CSS。
- 环形 SVG 与展开逻辑由 Atelier 实现；模块布局及按钮几何已在后续按上游移植。加入环层往复转动、指示灯呼吸、装饰层鼠标视差、按钮错峰进场、导航展开/收回与正文擦入。正文没有透视或缩放。
- 提供减少动态效果、精细指针悬停限制、键盘焦点和强制颜色处理。
- 原生 Swift/Core Animation 渲染器未直接移植。游戏贴图、提取的 Watch 资源、字体与着色器未打包。

上游 MIT 版权与全文见 [`public/licenses/EndfieldHUD.txt`](../public/licenses/EndfieldHUD.txt)，随前端资源一起分发。

## 验证

- `pnpm exec vitest run tests/hud-theme.test.ts tests/prototype.test.ts tests/startup.test.ts tests/workbench.test.ts tests/threads-ui.test.ts`：22 项通过。
- 类型检查与生产构建通过；有既存的 React 指令、包体积与 Rust 未使用代码警告。
- `git diff --check` 通过。
- 浏览器检查 1440×900、1280×800、960×700；环形入口在窗口内，账号与线程正文无横向溢出。账号检查仅使用浏览器内存中的 `.invalid` 示例数据；线程使用现有演示服务。
- 验证三款皮肤切换、当前页及输入保留、选择持久化、主工作台展开、返回环形概览与四个工作台目录。
- macOS release 包已构建、同步至本机应用；原生界面确认三款皮肤入口、新皮肤选中与工作区展开/收回。
- 本次安装复用已安装的 `atelier-router` 二进制，SHA-256 比对一致；未修改后端源码或业务数据。
- 未执行真实账号切换、额度请求、线程修复或业务端到端验收。浏览器预览的本机引擎未连接状态不代表原生服务验收。

## 截图

- [环形首页 / 1440](screenshots/hud/home-1440.png)
- [环形首页 / 960](screenshots/hud/home-960.png)
- [线程页 / 1440](screenshots/hud/threads-1440.png)
- [线程页 / 960](screenshots/hud/threads-960.png)

首页截图为浏览器未连接本机引擎的状态；线程截图全部为演示内容。

## 本机安装与回退

本轮使用以下命令打包，构建前将当前已安装的路由引擎复制到被忽略的 `src-tauri/binaries/atelier-router`，保持引擎字节不变：

```sh
CARGO_TARGET_DIR='/Users/Zhuanz/code case/code case/codex atelier/src-tauri/target' pnpm tauri build --bundles app --config '{"build":{"beforeBuildCommand":"pnpm build"}}'
```

常规完整构建仍使用 `GO_BIN=/path/to/go pnpm tauri build --bundles app`。本机缺少 Go 依赖缓存，完整引擎构建的下载过程已停止；本轮不把该过程记为通过。

更新后的程序位于 `/Users/Zhuanz/Applications/Codex Atelier.app`。
旧应用保留于 `/Users/Zhuanz/Applications/.atelier-backups/before-orbital-hud-20261111-124254.app`。
回退外观可直接选择原有皮肤；回退程序可在退出 Atelier 后恢复该备份。用户数据目录未改动。

## 动效补齐

- 适配 `HUDDesignMotion.swift` 的 36ms 错峰与淡入，及 `HUDSubsectionTransition.swift` 的 260ms 有限擦入。
- 适配 `HUDMechanicalArtwork.swift` / `SystemHUDView.swift` 的环层往复参数：每程 `41/6` 秒、两层分别为 −15° 与 +25°，指示灯呼吸每程 1.7 秒。
- 鼠标视差只作用于装饰环层；工作区、输入和数据保持正视。
- 展开/收回通过导航位置的 FLIP 动画呈现；正文只使用透明度和裁切，不缩放文字，不重建业务页面或等待动画再导航。
- 连续切换从当前显示位置重新开始；键盘动作立即落定，系统减少动效立即取消转场和环境动画，页面隐藏时暂停。
- 追加执行 `pnpm exec vitest run tests/hud-theme.test.ts tests/prototype.test.ts tests/workbench.test.ts`，10 项通过；生产构建通过。
- 浏览器实际观察到展开/收回的 15 个有限动画，连续反向点击后动画和临时裁切均清理，键盘操作后有限动画为 0，减少动效时 HUD 动画为 0。装饰层 PointerEvent 路径确认产生透视变换，正文变换为 `none`。
- 此处为原生动画逻辑在 Vue/CSS/WAAPI 上的适配，未集成原生 Metal/Watch 渲染器。

动效补齐版本已再次打包并同步至本机应用，原生确认展开、收回及环层变化。旧程序备份位于 `/Users/Zhuanz/Applications/.atelier-backups/before-hud-motion-20261111-131902.app`。

[展开与收回动图](screenshots/hud/motion-preview.gif)来自浏览器实际动画的逐帧捕获，放慢播放便于观察；页面未连接本机引擎。

## 空白环形概览

环形概览默认不显示中央正文面板或工作台目录，也不选中模块；点击模块后显示，返回概览后隐藏。使用 `v-show` 保留页面状态，原有两个皮肤不受影响。隐藏时不播放面板入场动画。

两项 HUD 针对性测试、类型检查、生产构建、macOS 打包通过；浏览器在 1440×900 和 960×700 确认默认隐藏、点击展开、返回隐藏与无横向溢出。当前首页 PNG 已更新；上面的动图是此前版本。

本机应用已同步；本轮回退程序：`/Users/Zhuanz/Applications/.atelier-backups/before-empty-hud-20261111-140536.app`。

## 地图与原版按钮布局

按用户提供的参考图补入静态地图，并移植 EndfieldHUD `HUDNavigation.swift` 的 1000×640 版式与按钮路径：

- 左侧四块 180×84 面板，x 偏移为 54 / 16 / 22 / 60，y 间距为 95。
- 右侧 78×74 双列小按钮，使用半径 306 的圆弧计算每排 x 位置，列间距 84、行间距 90。
- 底部两块 144×65 金色按钮使用半径 208 的镜像圆弧路径；面板的斜边与圆角也按原版公式生成。
- 功能入口仍为 Atelier 的 14 个模块；左侧 4 个、右侧 8 个、底部 2 个。
- 中央地图仅为占位，进入模块后隐藏，返回概览恢复；不展示业务正文，不读取定位、不联网加载地图。
- 地形线来自上游 `Resources/WorldMap/Terrain.bin`（NOAA ETOPO 2022，CC0）；区域轮廓采用 Natural Earth 公有领域数据。静态 SVG 与来源分别位于 `src/assets/hud/terrain-map.svg`、`public/licenses/HUD-map.txt`。
- 浅色面板、原版分层背板和圆弧形状保留；时钟使用实际上海时间，底部身份卡仅显示 Atelier 品牌，不含用户账号数据。
- 两项 HUD 针对性测试和类型检查、生产构建通过；1440×900 / 960×700 检查地图、按钮边界与正文展开收回。

本次改动影响范围是 HUD 导航外观与地图占位模块。

地图版已同步本机应用；程序回退备份：`/Users/Zhuanz/Applications/.atelier-backups/before-reference-map-hud-20261111-142631.app`。


## 原生复用接入（已验收）

原因与修复：`@_cdecl` 函数的可见性默认为 internal，Swift 不会为非 public 符号生成导出表；
此前 `nm -gU` 里只有 Swift mangled 名，所以 `dlsym("atelier_hud_create")` 拿不到符号。
三个宿主入口与 `RouteCallback` 改为 `public` 后重新编译，导出表出现
`T _atelier_hud_create / _atelier_hud_update / _atelier_hud_destroy`，加载成功。

实机验收（原生窗口，1440×898）：

- 环形首页由上游 `HUDNavigation` + `WorldMapSurface` + `HUDMechanicalArtwork` 绘制，
  网页版 `.hud-navigation` 在原生就绪后隐藏，回退路径仍保留给浏览器和非 macOS。
- 点击左侧「账号与额度」经 `atelier:hud-navigate` 回到 Vue 路由，打开的仍是我们的账号页
  （演示数据 1 个账号，未做真实账号请求）。
- 地图拖动改变视图；点击后滚动改变缩放。两者都在截图层面确认画面变化。
- 皮肤切到终末地再切回环形 HUD，原生视图与网页皮肤都正常切换。
- 未完成：`HUDSourceWatchView` / Metal 外壳未接入；按钮悬停亮度动效只在指针真实移入时触发，
  本轮的后台光标移动未能稳定复现截图证据，因此不记为已验证。

## 原生复用接入尝试（未验收，已修复）

此前交付的地图为静态 SVG，按钮动效为 Vue/CSS/WAAPI 适配，不能视为直接运行上游原生组件。

本轮加入固定上游版本的 Swift 源码、地图与字体资源，并尝试通过 macOS 原生视图接入上游 `HUDNavigation`、`WorldMapSurface`、`HUDMechanicalArtwork`、`HUDMotionController`。未接入完整 Metal / Watch 外壳。原始来源及局部修改见 `vendor/endfield-native/PROVENANCE.json`。

- 原生源码编译、前端类型检查与构建、应用打包完成；HUD 与 prototype 共 8 项测试通过。
- 实机加载失败：动态库缺少导出符号 `atelier_hud_create`。原生地图交互、按钮特效及导航回调均未通过实机验收，不能标记为交付完成。
- 达到本轮三轮验证预算后停止排错，保留接入代码为未验收状态。
- 本机程序已恢复至 `/Users/Zhuanz/Applications/.atelier-backups/before-native-endfield-20261111-151303.app` 对应的上一可运行版本，确认首页可打开且没有原生加载错误；该版本仍是静态地图及 Web 适配效果。
- 失败程序保留于 `/Users/Zhuanz/Applications/.atelier-backups/failed-native-endfield-20261111-restore.app`。

本次改动影响范围是 HUD 外观、macOS 原生视图接入与打包资源模块。

## 功能页样式统一（2026-11-11）

对照原生首页暴露的三处断裂已修复：

- 绿色残留：HUD 继承终末地 token，账号页标题、开关、选中描边都是森林绿。HUD 皮肤下把
  `--green` / `--positive` / `--mint` 重映射到墨色体系，当前账号卡改为黄底黑字高亮。
- 圆角混乱：线程胶囊 18-20px、维护卡 12px、路由卡 14px 与 `--radius: 0` 并存。HUD 下统一为 2px；
  switch 轨道保持胶囊形（控件本身语义）。
- 空状态：未连接引擎时只有一段 18px 内边距的灰字。HUD 下居中排版，`MODULE / STANDBY` 眉题 +
  26px 标题 + 说明 + 黄色主按钮。
- 侧栏材质：展开后的导航条原来是一层 CSS 渐变加切角，与首页原生按钮明显不同源。现改为
  `HUDNavigationEntry.updateAppearance` 的参数化复刻：白色 0.88 板材、0.25 白描边、#fff 内框与
  顶部高光、选中/悬停按 `sidePlateColor` 加分层 hover 光、按压下沉。选中项黄底黑字。
  这是 CSS 对上游材质参数的转写，不是新的视觉设计。

验证：HUD 主题测试、生产构建、打包通过；实机检查首页 → 主工作台空态 → 账号 → 行为插件 →
线程 → 返回概览，标题绿色消失，胶囊与卡片圆角归一，侧栏与首页按钮材质连续。

本次改动影响范围是 HUD 皮肤样式模块（`src/styles/themes/hud.css`）。
