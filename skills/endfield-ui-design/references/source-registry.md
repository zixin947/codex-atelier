# 来源索引

调查日期：2026-11-02。核验深度分别记录；“元数据”仅表示仓库可访问、默认分支与许可标识查询成功。原始 [21 仓库记录](evidence/repository-inventory.json) 与 [5 个提交快照](evidence/source-snapshots.json) 可离线查看。

## 官方入口

| 来源                                                                    | 核验深度                    | 用途                       |
| ----------------------------------------------------------------------- | --------------------------- | -------------------------- |
| [国际服官网](https://endfield.gryphline.com/en-us)                      | 实际画面与部分 DOM 计算样式 | 主视觉、导航与组件         |
| [国服官网](https://endfield.hypergryph.com/)                            | 入口确认，当次抓取超时      | 中文版本入口               |
| [云游戏页](https://endfield.hypergryph.com/cloud)                       | 文档加载完成但正文为空      | 待补，不冒充实测           |
| [影像资料](https://endfield.gryphline.com/en-us/video)                  | 搜索索引                    | 继续查找官方演示           |
| [公测前瞻研发通讯](https://endfield.hypergryph.com/news/8564)           | 已读，2026-01-17            | 系统与界面优化的历史依据   |
| [Beta Test II 研发通讯](https://endfield.gryphline.com/en-us/news/1340) | 已读，测试期资料            | 蓝图、批量、地图与平台差异 |
| [Gameplay Demo](https://www.youtube.com/watch?v=2h85NtIwduY)            | 索引，未完整播放            | 2022 早期设计对照          |
| [PS5 Demo](https://www.youtube.com/watch?v=2GjOGijE9fM)                 | 索引，未完整播放            | PS5 输入与 HUD 线索        |

官网四类具体画面和锚点见 [visual-evidence](visual-evidence.md)。

## 重点实现

| 编号 | 项目                                                                              | 许可标识 | 已读范围 / 价值                                          |
| ---- | --------------------------------------------------------------------------------- | -------- | -------------------------------------------------------- |
| S01  | [icaruszezen/endfield-ui](https://github.com/icaruszezen/endfield-ui)             | MIT      | 总纲、测量、来源和项目调研；非官方规范，按证据等级阅读   |
| S02  | [Statrue/endfield-design-skill](https://github.com/Statrue/endfield-design-skill) | MIT      | README；任务/家族/主题的设计方法，未安装                 |
| S03  | [ymh0000123/dsh-theme-endfield](https://github.com/ymh0000123/dsh-theme-endfield) | MIT      | 设计说明；纸墨、信号色、主题对比度，原型项目已有背景来源 |
| S04  | [VBeatDead/ReEnd-Components](https://github.com/VBeatDead/ReEnd-Components)       | MIT      | 文档站和按钮源码；组件状态、分层令牌，React 实现         |
| S05  | [Talos-Pioneers/ui](https://github.com/Talos-Pioneers/ui)                         | AGPL-3.0 | 仓库页和目录；Vue 工具站结构，未复制代码                 |

可定位的源码：

- [DSH 设计说明](https://github.com/ymh0000123/dsh-theme-endfield/blob/ada9396a957633a40bdad89ff084b0136b9515b4/docs/design-language.md)。
- [ReEnd 按钮](https://github.com/VBeatDead/ReEnd-Components/blob/959fc100982be7cd095062cf8371d2f9f523c66b/src/components/ui/button.tsx)。
- [非官方总纲](https://icaruszezen.github.io/endfield-ui/design/overview/)、[测量](https://icaruszezen.github.io/endfield-ui/design/references/measurements/)、[官方目录](https://icaruszezen.github.io/endfield-ui/design/references/official-sources/)。

本包重新组织并独立编写操作指南，没有复制这些项目的整套指南或组件源码。社区文档的“实测”不是本包的独立实测，后者单独存于 evidence。

## 更多主题与工具

| 项目                                                                                                    | 许可标识    | 核验与用途                             |
| ------------------------------------------------------------------------------------------------------- | ----------- | -------------------------------------- |
| [Yue-plus/astro-endfield](https://github.com/Yue-plus/astro-endfield)                                   | MIT         | 仓库页，内容站布局                     |
| [Yue-plus/endfield_icons](https://github.com/Yue-plus/endfield_icons)                                   | MIT         | 元数据，图标整理线索；图形权利另论     |
| [Naptie/endfield-docmaker](https://github.com/Naptie/endfield-docmaker)                                 | MPL-2.0     | 仓库页，档案/公文版式                  |
| [Tassel97/EndfieldTheme](https://github.com/Tassel97/EndfieldTheme)                                     | 未识别      | README，Flutter 控件和减少动效         |
| [rison114514/dsh-endfield-ui](https://github.com/rison114514/dsh-endfield-ui)                           | 未识别      | 元数据，工作台主题                     |
| [INnoVationEE/dsh-endfield-theme](https://github.com/INnoVationEE/dsh-endfield-theme)                   | 未识别      | 元数据，终端与用量组件                 |
| [Longxiangjunlin/dsh-endfield-theme](https://github.com/Longxiangjunlin/dsh-endfield-theme)             | NOASSERTION | 元数据，深色与开屏线索                 |
| [Xiaoli-Asuna/dsh-theme-endfield-style](https://github.com/Xiaoli-Asuna/dsh-theme-endfield-style)       | MIT         | 公开介绍，浅深表面与侧栏               |
| [LaoBiDeng321/Endfield-Style-Skill](https://github.com/LaoBiDeng321/Endfield-Style-Skill)               | MIT         | 元数据，开屏与改造线索；未安装         |
| [ChuwuYo/Endfield-Pomodoro](https://github.com/ChuwuYo/Endfield-Pomodoro)                               | MIT         | 元数据，聚焦工具                       |
| [cloud-oc/endspace](https://github.com/cloud-oc/endspace)                                               | Apache-2.0  | 元数据，NotionNext 主题                |
| [SpicySugar16/Endfield-Belt-Font](https://github.com/SpicySugar16/Endfield-Belt-Font)                   | OFL-1.1     | 元数据，同人字形；旧作者地址发生重定向 |
| [leostudiooo/typst-touying-theme-endfield](https://github.com/leostudiooo/typst-touying-theme-endfield) | MIT         | 元数据，标题/页码/图表                 |
| [NCreeper233/endfield-logo-maker](https://github.com/NCreeper233/endfield-logo-maker)                   | MIT         | 元数据，字标生成线索                   |
| [GlacierGlimmer/zmd-charge-plus](https://github.com/GlacierGlimmer/zmd-charge-plus)                     | MIT         | 元数据，小型状态 HUD                   |
| [palmcivet/awesome-arknights-endfield](https://github.com/palmcivet/awesome-arknights-endfield)         | MIT         | 已读设计列表，继续发现入口             |

未识别/NOASSERTION 不代表开放复用。许可证以具体文件为准，库的许可不能覆盖游戏 Logo、美术或字体。

## 字体与分析

- [Z动效研究所](https://www.zcool.com.cn/work/ZNzM0MTgxMzI%3D.html)：已读文字并查看游戏 GIF，第三方分析。
- [机核手柄交互分析](https://www.gcores.com/articles/211635)：已读，2026-03-08 用户投稿；历史体验不等于当前缺陷。
- [GameFontLibrary](https://www.gamefontlibrary.com/games/arknights%3A-endfield)：已读并跟随字体来源，第三方识别。
- [Novecento Sans](https://fonts.adobe.com/fonts/novecento-sans)、[Iosevka](https://typeof.net/Iosevka/)、[HarmonyOS Sans](https://github.com/huawei-fonts/HarmonyOS-Sans)：打开过来源页，未下载或打包字体。

## 更新时记录

来源 ID、URL、检查日期、载体、平台/构建、视口、已查看范围、测量方式和许可文件。官方标题、仓库星数、分支和许可证会变化，重新使用时按需要复核。不自动全网爬取、安装 Skill 或提取游戏文件。
