# EndfieldHUD 原生呈现依赖

来源：DDDuoDuo/EndfieldHUD，固定提交 `19de06a45a6fb701cac1be4a1571111a8813b798`。

`Sources/` 保留上游 Swift 模块，排除程序入口 `main.swift`，以动态库方式编译。保留完整模块是为了继续使用原有类型依赖，避免为依赖伪造兼容桩或重新实现渲染器。Atelier 的入口只有 `src-tauri/native-hud/AtelierHud.swift`；不创建上游 `AppDelegate`、账号、更新、音频或文档服务。

直接运行的上游组件：

- `HUDNavigation`：按钮面板、背板、图标分层，以及有限亮度激活和选中动画。
- `WorldMapSurface` / `WorldMapRasterController` / `WorldMapRasterPainter`：地图相机、拖动、缩放、地形绘制和中心区域高亮。
- `HUDMechanicalArtwork` / `HUDDepthPlane` / `HUDMotionController`：原生图层、视差与环境动画。
- `HUDFontRegistry` / `HUDFontResolver`：进程私有字体加载与原生字体排版。

本次未接入 `HUDSourceWatchView` / Metal 游戏外壳，不能把此集成称为整个原版 HUD 的逐像素搬运。网页界面仅是浏览器及非 macOS 的兼容预览；macOS 皮肤概览由原生组件接管。

本地修改仅有：

1. `HUDNavigation.swift` 增加模块白名单和标题覆盖接口，对接现有 14 个路由。
2. `HUDResources.swift` 将包内资源路径限定在 `hud-native/`，避免与 Atelier 其他资源混用。

代码遵循上游 MIT 许可。原始文件 SHA-256、来源和本地修改记录见 `PROVENANCE.json`。
字体保持原文件，许可位于 `Resources/Fonts/`；只供当前进程使用，不安装到系统。
`Resources/WorldMap/Terrain.bin` 为上游 NOAA ETOPO 2022 / CC0 数据；`Countries.bin` 按上游格式从 Natural Earth 公有领域数据编码，来源见 `public/licenses/HUD-map.txt`。

构建：macOS 上运行 `node scripts/build-native-hud.mjs`。Tauri 的 `build.rs` 自动调用此构建；产物与资源写入被忽略的 `src-tauri/binaries/hud-native/`，由 macOS 专用 Tauri 配置打包。Swift 使用当前工具链支持的 Swift 5 语言模式，不要求系统安装第三方框架。
