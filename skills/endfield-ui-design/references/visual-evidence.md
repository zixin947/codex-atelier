# 画面证据

以下八张图来自 2026-11-02 的公开网页浏览截图。它们用于视觉研究，未操作游戏客户端。哈希、来源、日期与范围见 [manifest.json](evidence/manifest.json)。

## 官方网页

| 编号 | 画面                                           | 可以观察                                 | 不能推出                 |
| ---- | ---------------------------------------------- | ---------------------------------------- | ------------------------ |
| V01  | [官网首屏](screenshots/official-home.png)      | 白底、版本主图、左侧轨、右下下载坞       | 全游戏只有当前角色主题色 |
| V02  | [干员情报](screenshots/official-operators.png) | 圆形头像列、括号姓名、成对标签、角色大图 | 游戏角色养成页完整流程   |
| V03  | [世界观](screenshots/official-lore.png)        | 深底点云、圆环刻度、正文与分页分区       | 官方所有深色界面参数     |
| V04  | [玩法介绍](screenshots/official-gameplay.png)  | 巨字、黄色竖签、媒体框与章节             | 视频内容，采样时未显示   |

来源分别为 [首页](https://endfield.gryphline.com/en-us)、[干员](https://endfield.gryphline.com/en-us#operator)、[世界观](https://endfield.gryphline.com/en-us#lore)、[玩法](https://endfield.gryphline.com/en-us#gameplay)。图中保留 Cookie 提示与网页布局。

## 第三方文章中的游戏画面

来源：[Z动效研究所文章](https://www.zcool.com.cn/work/ZNzM0MTgxMzI%3D.html)。这是文章嵌入实机 GIF 的截图，具体游戏平台/构建未独立确认，不是本 Skill 自己运行游戏得到的证据。

| 编号 | 画面                                               | 可以观察                             | 局限                                 |
| ---- | -------------------------------------------------- | ------------------------------------ | ------------------------------------ |
| V05  | [HUD](screenshots/game-hud-zcool.png)              | 四周信息停靠、中央留场景、地图与技能 | 不是全套 HUD 状态                    |
| V06  | [装备/活动转场](screenshots/game-panels-zcool.png) | 浅色资料面板、分类与深色活动         | 动画中间帧，不能据此判文字终态对比度 |
| V07  | [活动与按钮](screenshots/game-activity-zcool.png)  | 活动目录、大主图、右侧奖励、胶囊行动 | 不能把活动主题色当全局色             |

## 社区实现

V08：[ReEnd 文档站](screenshots/community-reend.png)，来源 [ReEnd docs](https://reend-components.pages.dev/en/docs)。可看组件分类、目录和主题强度；其 React/深色战术风不等于官方标准。

## 看图方法

按任务选两三张相关图，用可用图像查看器实际查看。记录主体位置、阅读顺序、背景/正文层级、形状、信号色和状态。明确“看见了什么”与“推测了什么”。

设计数据卡时优先看 V02、V03、V07；做主工作台看 V01、V03、V05；只做按钮结合 V07 与 [官方实测](official-measurements.md)，不要从 V08 复制一整套组件。

## 待补

完整主菜单、地图交互、角色养成、通讯、设置、真实按钮全部状态、音效时间轴、当前版本多平台对照。云游戏页当次正文为空，没有可用截图。未包含的内容不填成已观察。

## 权利

图片包含原官方美术和作者网页，属于参考材料，不是本包原创，也不作为生产 assets。具体使用见 [权利边界](rights.md)。
