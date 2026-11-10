# 参考导航

证据采集于 2026-11-02。除明确标注官网实测或画面观察的内容外，本包的尺寸、色板、阈值、时长与实现方式都是设计建议，不是游戏官方规范。

## 阅读路径

| 任务       | 路径                                                           | 产物                           |
| ---------- | -------------------------------------------------------------- | ------------------------------ |
| 新建页面   | foundations → screen-families → layout → 相关组件 → validation | 主界面与关键状态               |
| 只改颜色   | color → official-measurements → implementation                 | 语义令牌和受影响控件           |
| 只改按钮   | controls → shape-iconography → motion                          | 按钮完整状态                   |
| 点击反馈   | motion → controls → validation                                 | 点击、重复点击、键盘与减少动效 |
| 账号与额度 | data-display → overlays-feedback → color                       | 数据状态清楚的卡片             |
| 主题迁移   | foundations → implementation → validation                      | 保留业务的分层主题             |
| 研究素材   | visual-evidence → source-registry → rights                     | 可追溯观察                     |
| 只做审查   | workflows → validation → 对应专题                              | 定位、影响、证据、建议         |

## 专题

- [基础](foundations.md)：载体、识别性和误区。
- [色彩](color.md)：角色、表面、状态、对比度。
- [字体](typography.md)：标题、正文、数字、日志。
- [形状](shape-iconography.md)：切角、圆角、描边、图标。
- [布局](layout.md)：构图、导航、桌面窗口与滚动。
- [动效](motion.md)：交互、转场、加载、开屏与生命周期。
- [操作控件](controls.md)：按钮、导航、表单、选择与开关。
- [数据展示](data-display.md)：卡片、额度、表格、图表、日志。
- [浮层与反馈](overlays-feedback.md)：弹窗、详情、通知、空状态、错误。
- [界面家族](screen-families.md)：十类结构的适用条件。
- [背景与图像](background-imagery.md)：纹理、主图和动画背景。
- [实现](implementation.md)：令牌、CSS/SVG、主题隔离与 Vue。
- [工作流](workflows.md)：各任务的输入与交付。
- [验证](accessibility-validation.md)：无障碍、尺寸、交互和截图。
- [示例](examples.md)：典型请求与完成尺度。

## 证据

- [官网实测说明](official-measurements.md)、[原始读数](evidence/official-measurements.json)。
- [画面证据](visual-evidence.md)：八张来源截图与可观察范围。
- [来源索引](source-registry.md)、[仓库元数据](evidence/repository-inventory.json)、[源码快照](evidence/source-snapshots.json)。
- [权利边界](rights.md)：研究图片、源码、字体与商标。
- [项目参考](atelier-profile.md)：仅 Codex Atelier 适用。

游戏客户端全部流程、音效、完整动效时间轴、登录后页面和全部版本主题未完成采样；云游戏页面当次为空。精确复刻某一画面时，获取该版本真实参考，不用社区参数补成官方值。
