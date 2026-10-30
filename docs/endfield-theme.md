# 终末地 · 野外终端

Atelier 的可切换浅色皮肤。没有选择过皮肤时默认启用；右上角 **皮肤** 按钮可直接切换“终末地 / 千禧机壳”；**应用设置 → 界面皮肤** 仍保留完整预览和背景选项。偏好仅保存在当前 Atelier 的本地存储 `atelier.appearance.theme`；存储失败时当前窗口仍可切换并显示提示。

## 设计与来源

实际查看了上游界面截图，并阅读 [dsh-theme-endfield](https://github.com/ymh0000123/dsh-theme-endfield) 的 [配色令牌](https://github.com/ymh0000123/dsh-theme-endfield/blob/main/src/client/41-mount-tokens.js) 与 [主题样式](https://github.com/ymh0000123/dsh-theme-endfield/blob/main/src/styles/theme.css)，参考纸灰、墨黑、信号黄、直角和工业制图语言。上游原创代码为 MIT；署名及许可见 [第三方说明](../THIRD_PARTY_NOTICES.md)。

本项目独立编写 Vue/CSS 皮肤，保留 Atelier 标识、现有导航、页面组织和账号卡片排版。主仪表采用墨黑与信号黄，正常状态继续用绿色、告警用暖橙、错误用红色。正文保持系统中文字体，英文标识和状态数字采用现有等宽字体。背景已改为直接移植上游原版等高线渲染：高斯地形场、Marching Squares 提取、闭环与样条平滑、密度/覆盖率校验、24 FPS 缓慢变形及鼠标轨迹扰动。移除了初版手绘 SVG 与额外网格，画布连续覆盖工作站；保留上游 ENDFIELD 背景字标的尺寸、字重、字距和浅色透明度。

背景源码固定于上游提交 `359bfad889b4f3d2e79515c4145f624f34ba84f1`，保留原始注释和 MIT 署名。仅将 DSH 的宿主查找替换为 Vue 工作站元素，并适配本地偏好与生命周期清理；没有接入 DSH 账号、网络、音频或注入宿主。默认使用上游 Canvas2D 路径，未启用 Worker 执行或放宽 Tauri CSP。字标为 CSS 文字，未复制游戏图片或字体资源；这是非官方同人风格皮肤。

## 代码边界

| 文件                                           | 职责                                           |
| ---------------------------------------------- | ---------------------------------------------- |
| `src/app/theme.ts`                             | 主题类型、初始化、偏好读取与保存               |
| `src/shared/components/ThemePicker.vue`        | 皮肤缩略预览与选择按钮                         |
| `src/styles/tokens/endfield.css`               | 颜色、语义状态、阴影、网格、字体令牌           |
| `src/styles/themes/endfield.css`               | 通过 `data-theme="endfield"` 覆盖现有组件外观  |
| `src/shared/background/EndfieldBackground.vue` | Vue 挂载、选项与生命周期                       |
| `src/shared/background/vendor/`                | 原版等高线源码、生成 Worker 源字符串与来源记录 |

覆盖主工作台、扩展、诊断、账号卡片、连接、规划页、弹窗、通知、浮层与控件状态。切换皮肤只改 HTML 主题属性和外观偏好；账号、凭据、Hook、网关与 Rust 命令不受此模块控制。启动时先应用主题再挂载界面。设置中可分别关闭完整背景、动态等高线、鼠标扰动及字标，偏好保存在 `atelier.appearance.endfieldBackground`。减少动态效果和页面隐藏时停止动画；关闭背景或切回千禧皮肤时销毁画布、rAF、监听与观察器。暗底控件的焦点框使用信号黄。

右上角选择器支持点击外部或 Escape 关闭，方向键移动焦点，Enter/Space 选择。首次默认终末地；已保存的选择在重启后保留。

## 预览

以下均为隔离浏览器渲染，**不是原生 Tauri 运行验收**。账号页通过浏览器内存注入无凭据的 `.invalid` 示例数据；自动检测被禁用，没有读取真实登录或调用账号命令。

![1440×900 主工作台](screenshots/endfield/dashboard-1440.png)

[原版背景细节](screenshots/endfield/background-detail-1440.png)：在浏览器中临时隐藏界面层，展示实际画布与字标，不是静态素材替代。

| 页面       | 截图                                                                                                    |
| ---------- | ------------------------------------------------------------------------------------------------------- |
| 较小工作台 | [1280×800](screenshots/endfield/dashboard-1280.png)                                                     |
| 账号卡片   | [1440×900](screenshots/endfield/accounts-1440.png) · [1280×800](screenshots/endfield/accounts-1280.png) |
| 额度浮层   | [1280×800](screenshots/endfield/quota-preview-1280.png)                                                 |
| 重置卡弹窗 | [1280×800](screenshots/endfield/reset-cards-1280.png)                                                   |
| 扩展管理   | [1280×800](screenshots/endfield/extensions-1280.png)                                                    |
| 诊断       | [1280×800](screenshots/endfield/diagnostics-1280.png)                                                   |
| 皮肤选择   | [1440×900](screenshots/endfield/settings-1440.png)                                                      |

[右上角皮肤切换入口](screenshots/endfield/theme-switcher.png)。

## 账号卡片配色重设计

仅对终末地皮肤覆盖账号卡片的视觉样式，保留原有 254×294 卡片尺寸、信息顺序、双额度圆环与操作位置；额外错误说明可增加卡片高度。石灰暖白机壳采用 12px 圆角与轻阴影，内嵌的深色额度屏采用 9px 圆角，按钮与铭牌也使用小圆角。当前账号保留信号黄状态块、顶部短轨与细刻度；Pro 使用深灰金色铭牌，Plus 和重置卡采用灰蓝色，其他套餐有克制的紫灰层次。

额度屏保留终末地黄黑关系：首个窗口使用信号黄、第二个窗口使用青灰色，中低额度使用暖金和橙色，圆环端点圆润。旧快照保持灰色、未知值保持空环，不将示例数据误标为新鲜额度。资料区保持浅色，底栏采用同色机壳与独立按钮托盘，避免大面积深浅硬切；相关额度浮层和重置卡弹窗同步使用柔和边框与圆角。

专用颜色集中在 `src/styles/tokens/endfield-account.css`，组件覆盖集中在 `src/styles/themes/endfield-accounts.css`。账号组件只增加用于错误状态着色的 `data-status` 属性，服务接口、自动刷新、凭据和 Rust 命令未作修改；千禧皮肤保留原有样式。

[卡片细节](screenshots/endfield/account-card-rounded-field-detail.png) · [1440×900](screenshots/endfield/account-card-rounded-field-1440.png) · [1280×800](screenshots/endfield/account-card-rounded-field-1280.png) · [列表视图](screenshots/endfield/account-card-rounded-field-list.png) · [额度浮层](screenshots/endfield/account-card-rounded-field-quota.png)

本次截图使用浏览器内存中的 `.invalid` 示例账号，检测和刷新均替换为离线示例适配器。检查了 1440×900、1280×800、960×700，卡片和列表均无横向溢出；实际操作了选择、悬停、键盘焦点、示例加载动画、额度浮层、重置卡弹窗与 Escape 关闭。根据截图重新平衡石灰外壳、深色屏与黄色识别元素，并压深灰蓝徽标文字改善对比度。切回千禧皮肤时恢复原有浅色标题区与小圆角，减少动效偏好下卡片过渡为 0s。此轮仅验证外观，没有测试真实账号。

当前卡片迭代通过类型检查/前端构建、修改文件格式检查、`git diff --check` 和 macOS 打包；检查的 11 组文字与底色令牌对比度均达到 4.5:1。新版 Atelier 已重启至工作台，账号页的原生数据操作仍留给用户自行验收；本轮未修改账号组件或服务逻辑。

## 账号卡片动效

终末地卡片入场使用 280ms 淡入和 6px 轻移，逐项错开 32ms，延迟上限为 224ms；相同账号节点刷新时不会重复播放。精细指针悬停时抬起 2px，阴影和边框在 180ms 内过渡；刷新中的卡片保持原位。按钮按下下沉 1px，松开平滑归位。键盘焦点保留清晰轮廓，当前账号的黄色短轨轻微展开。

额度圆环首次显示时在 440ms 内绘入；已有数值改变时平滑过渡弧长和颜色，数字及时显示新读数，不生成虚假的计数值。耗尽和未知仍遵循原有区分。动画全部由 CSS 驱动，没有持续扫描、闪烁、额外定时器或网络请求，尺寸、排版和账号服务不变。位移、时长与缓动统一定义于 `endfield-account.css` 令牌。

系统开启减少动态效果时，入场、悬停位移、按钮位移、短轨展开、圆环绘入和过渡全部关闭；原有加载旋转同样由全局减少动效规则关闭。千禧机壳不应用这些新动效。

[入场动效预览](screenshots/endfield/account-card-motion-preview.gif) 为离线示例卡片在浏览器中按动画时间轴采样并编码的 GIF；[悬停截图](screenshots/endfield/account-card-motion-hover-1440.png) 和 [卡片细节](screenshots/endfield/account-card-motion-detail.png) 为实际渲染。检查了入场事件、悬停抬起、示例额度更新后的弧线过渡，以及 1440×900、1280×800、960×700 下无横向溢出。根据检查修正减少动效规则的优先级，复查为无动画、无过渡、无位移。仅使用内存示例账号，不操作真实客户端。

## 额度分档与读数状态

额度颜色根据**剩余百分比**自动变化，不按已用百分比判断：

| 剩余         | 状态   | 终末地外观                 |
| ------------ | ------ | -------------------------- |
| ≥50%         | 充足   | 首窗口信号黄、第二窗口青灰 |
| ≥20% 且 <50% | 留意   | 暖金                       |
| >0% 且 <20%  | 偏低   | 暖橙                       |
| 0%           | 已用尽 | 柔红空环，明确显示 0%      |

环内增加文字状态，避免仅靠颜色传达。不足 1% 显示 `<1%`，不再将少量可用余额四舍五入成 0%。未知读数用虚线空环与破折号，缺少窗口显示“未提供”；非有限数值也视为未知。

显示优先级为刷新中、读取失败、未知、旧快照、当前额度分档。刷新时保留已有数字并显示旋转细环；失败时保留上次快照、置灰数字并用柔红标明“读取失败”，详情说明旧值仅供参考。没有快照时不会虚构一个旧读数。额度过期时显示“旧快照”，不会把旧的 0% 标成当前已用尽。重置时间仍显示在原位置，键盘焦点、点击和悬停可查看完整详情，Escape 关闭。

仅修改额度展示与状态映射，没有改变账号读取、刷新调度、凭据、网关或 Rust 命令。正常卡片仍为 254×294，错误说明仍按需增高。

[分档细节](screenshots/endfield/quota-stages-detail.png) · [1440×900](screenshots/endfield/quota-stages-1440.png) · [1280×800](screenshots/endfield/quota-stages-1280.png) · [列表](screenshots/endfield/quota-stages-list.png) · [刷新中](screenshots/endfield/quota-stages-loading.png) · [读取失败浮层](screenshots/endfield/quota-stages-failure-preview.png)。全部使用浏览器内存中的无凭据示例账号，没有读取真实登录。

本轮根据首版截图稍微放大环内状态字，并将未知轨道改为虚线，保留原尺寸与时间文字。隔离浏览器确认两种目标尺寸和 960×700 无横向溢出；确认刷新中旋转环及刷新后的状态恢复、失败详情保留快照、焦点浮层未超出窗口、Escape 关闭以及列表切换。`pnpm exec vitest run tests/quota-meter.test.ts tests/startup.test.ts tests/prototype.test.ts` 共 29 项通过；类型检查、前端构建、修改文件的格式检查、`git diff --check` 和 `pnpm tauri build --bundles app` 通过。原生账号页与真实额度链路未测试，留给用户验收。

## 启动开屏

以用户提供的横版开屏截图为构图参考，独立绘制黑色网格、圆环刻度、三角铭记、Atelier 标识面板、横排 CODEX ATELIER 大标题、两侧编号和底部彩色横向进度线。没有使用截图里的游戏标识、DeepSeek 品牌图片或桌宠素材，也不声称已找到该截图对应的公开源码。左侧黄色细轨与底部进度线同步推进，百分比固定于左上角；不再使用右侧 END / FIELD 分行海报。

生命周期和退出节奏仍参考上游同一提交的 [30-boot-loader.js](https://github.com/ymh0000123/dsh-theme-endfield/blob/359bfad889b4f3d2e79515c4145f624f34ba84f1/src/client/30-boot-loader.js)。状态文字表示本地界面初始化，不表示真实账号、网络、环境检查或额度刷新进度。启动动画按本项目需求默认开启。

设置 → 界面皮肤 → 启动加载动画可关闭或预览，偏好保存在 `atelier.appearance.startupAnimation`。只在终末地主题每次页面启动时自动播放一次；路由和主题切换不重播。预览会关闭设置弹窗后播放，可按 Esc 跳过；按用户要求，开屏右上角不再显示跳过按钮。系统减少动效时直接跳过，播放中切至减少动效、隐藏窗口或切走终末地也会移除开屏。

`src/app/startup.ts` 管理偏好和单次启动策略，`src/shared/startup/endfield-loader.ts` 管理 DOM、时钟、退出与焦点恢复。开屏在 Vue 挂载前建立，进度达到末段后等待本地界面初始化结束，随后播放 220ms 停留、520ms 扫屏与 620ms 淡出。进度是动画编排值；并非按初始化任务数量计算的精确百分比。6.5 秒强制移除避免初始化或动画时钟卡住；后台粗时钟兼容 rAF 暂停。开屏期间应用区域设为 inert，结束恢复；等高线在开屏结束后挂载以减少启动开销。没有账号命令、音频、联网请求或 Rust 改动。

[1440×900 横版开屏](screenshots/endfield/startup-horizontal-1440.png) · [1280×800 横版开屏](screenshots/endfield/startup-horizontal-1280.png)。截图来自隔离浏览器。

隔离浏览器确认冷启动仅出现一次、完成 100% 与扫屏后移除、路由切换不重播、设置预览会关闭弹窗、Esc 可跳过；关闭后重载没有开屏，减少动效和千禧主题下也不出现。1440×900、1280×800 截图及 960×700 几何检查中没有横向溢出，字标和读数均在窗口内。测试只使用本地示例工作台，没有打开真实账号页或发送账号命令。

横版构图根据首轮截图收小了圆环、放大主标题，并重新检查两种目标尺寸与 960×700。底部横向进度线具有实际推进值；初始化未完成时与百分比一起保持在 95%，不会仅显示一条静态装饰线。设置预览和 Esc 跳过在浏览器中再次通过。

中央铭牌已换为内置图片生成工具制作的像素图：左侧 Codex 字样，右侧采用用户指定的 OpenAI / ChatGPT 结形标志参考。图片随前端离线打包，使用 `image-rendering: pixelated` 保留像素边缘，右上角跳过按钮已移除。参考源、清晰 SVG 和完整编辑提示词见 [像素铭牌说明](codex-pixel-nameplate.md)。[1440×900](screenshots/endfield/startup-pixel-1440.png) · [1280×800](screenshots/endfield/startup-pixel-1280.png) 为浏览器实际渲染；检查了本地图片加载、按钮移除、Esc 和 960×700 下无横向溢出。

像素版同样通过 16 项隔离测试、类型检查/构建、格式检查及 macOS 打包；已重启 Atelier 并查看 [像素铭牌原生截图](screenshots/endfield/startup-pixel-native.png)，确认图片显示且右上角没有按钮。只查看演示工作台与外观设置，没有操作真实 Codex 客户端或真实账号。

`pnpm exec vitest run tests/startup.test.ts tests/prototype.test.ts` 共 16 项通过；前端类型检查与构建、修改文件的 Prettier 检查、`git diff --check` 及 `pnpm tauri build --bundles app` 通过。新版 Atelier 原生窗口已重新启动，并在前台重播横版开屏，记录了 [macOS 实机截图](screenshots/endfield/startup-horizontal-native.png)。此项仅验证外观与播放；未打开真实账号页、执行账号操作或操作真实 Codex 客户端。

## 前期主题验证

- 根据首轮截图缩减主仪表与辅助面板的纵向留白，保留原有分栏；检查文字、边框、浮层和弹窗层级。
- 在 1440×900、1280×800 检查工作台和示例账号页，并检查 960×700 账号页，未出现横向溢出；内容较高时仍在主区域滚动。
- 在隔离浏览器实际切换千禧机壳/野外终端，刷新后读取到已保存的选择；键盘 Space 可切换，焦点框可见；模拟减少动效时过渡时长为 0s。
- 前端类型检查/构建、Rust 检查、格式检查、`git diff --check` 和 macOS `.app` 打包通过。
- 没有运行真实账号检测、额度刷新、登录、切换或重置操作；没有启动新版 Tauri 窗口，也没有操作真实 Codex 客户端。原生渲染由用户验收。

## 原版背景移植验证

- 静态检查确认 vendored 主运行时无未定义的全局依赖，未包含账户读取或网络请求。
- 隔离浏览器确认 Canvas 有实际像素，连续帧像素摘要变化，背景确实在动态绘制。
- 关闭完整背景后画布移除；重新启用和切换主题后均只有一个画布。
- 模拟系统减少动态效果后，跨多个动画帧的像素摘要保持一致。
- 原版几何与绘制算法保留；仅适配宿主、偏好与清理，不宣称与 DSH 不同布局的页面逐像素相同。

右上角入口已在隔离浏览器确认：无保存偏好时默认终末地；点击切换千禧机壳后刷新可恢复选择；键盘可切回终末地，Escape 关闭并返回焦点，外部点击关闭。
