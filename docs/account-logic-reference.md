# 账号卡片逻辑对齐

本轮范围是账号卡片及其直接管理流程：真实昵称与套餐资料、订阅状态/日期、当前文件账号识别、额度刷新和缓存、已有账号同步、定时刷新、复制与删除。没有扩展为自动修改真实客户端、后台守护进程或自动切换 Agent 会话。

## 参考与实现来源

- AiMaMi 的 [RegistryItem 与账号同步/状态保留](https://github.com/borawong/AiMaMi/blob/main/src-tauri/src/core/repository.rs) 公开了 account_name、workspace_name、profile_name、订阅是否有效、到期时间、续订标记、当前账号键和额度缓存等数据契约。此前“没有这部分逻辑源码”的说法过于宽泛：缺少的是公开账号卡片组件及部分依赖模块，仓库中确有相关数据与同步逻辑。
- AiMaMi 的 [刷新设置](https://github.com/borawong/AiMaMi/blob/main/src/hooks/use-auto-refresh.ts) 提供 30 秒、1 分钟、3 分钟和 5 分钟选项；本项目以页面计时器实现，默认手动，不启动后台服务。
- 订阅读取协议参考另一公开实现的 [订阅快照解析](https://github.com/wen495033653/codex-switch/blob/main/src-tauri/src/accounts/usage/state/subscription.rs) 和 [请求处理](https://github.com/wen495033653/codex-switch/blob/main/src-tauri/src/accounts/usage/client.rs)。AiMaMi 未公开的订阅请求实现不能被描述为已经直接移植。
- 账户与额度仍通过 [官方 app-server](https://learn.chatgpt.com/docs/app-server) 处理。

采用独立的 Vue/Rust 实现，未复制上游 React 组件、品牌、素材或整套管理程序。设计与安全边界以本项目为准。

## 字段与刷新

登录快照提取昵称、工作区/配置名称、套餐及 chatgpt_subscription_active_until；不从令牌 exp、额度重置时间或购买价格猜订阅到期。导入信息是未经在线校验的快照，界面标注来源。旧账号打开页面时，从 Atelier 已保存的凭据中补齐显示字段，不读取或改写真实客户端凭据。

手动刷新分两条独立结果：

1. 官方账户/额度读取。account/read 不强制刷新令牌；不额外生成推理。
2. 使用 Atelier 管理的普通 Codex 登录，查询固定的 ChatGPT 订阅接口；无重定向、15 秒上限、有界 JSON，使用 app-server 返回的客户端标识。该接口不是稳定公开 API，403/404、格式变化或网络错误都可能发生，尚未在真实账号上验收。

订阅请求不使用网关的 Sign in with ChatGPT 授权。其新数据优先于旧登录快照，昵称等可用字段缺失时保留旧值；更新失败保留上次信息，并显示旧数据与错误来源。接口未提供的布尔值保持未知，不转为 false；令牌快照过期不直接宣称订阅已失效。

定时刷新只查询额度，避免频繁拉取订阅信息。默认关闭；用户选择后仅在账号页可见且没有编辑/确认弹窗时执行，不重叠请求，离开页面停止。页面本地计时用于把超过五分钟或已过重置时间的额度快照标为旧数据，不触发网络请求。

## 当前账号与本地同步

进入账号页自动将当前文件登录同步到列表，参考 AiMaMi 公开的 load_local_state_synced / sync_local_runtime_state / sync_current_auth_into_registry 流程，独立实现。旧的手动“本地导入”流程已移除。

当前来源是有效 CODEX_HOME，未设置时用默认目录；不把另一个目录里的旧登录作为回退。账号页可见时每 10 秒以及窗口焦点/可见性恢复时检查，弹窗和操作期间跳过，离开页面清理监听和计时器。文件签名未变时只检查元信息，不重复读取来源凭据或写入副本。

首次发现自动加入卡片；已有身份自动去重，源凭据改变后同步 Atelier 副本，保留本地别名、分组、备注、订阅信息和旧额度。前后签名不一致则撤下当前标记、稍后重试，不将中途变化的来源标为当前。所有写入仅针对 Atelier 独立目录。检测本身不执行联网或令牌刷新。

当前账号仅指文件登录，不证明运行中的 Desktop、钥匙串或 Agent 会话身份。不可读取/不支持的来源明确显示原因，保留已有卡片。按用户要求，本轮不运行真实自动检测，原生运行验收仍由用户完成。

## 卡片操作

- 昵称优先本地名称，其次真实快照昵称/配置名称；未知不拼接邮箱充当真实昵称。
- 当前标记依据实际文件观察，与 last_selected_id 的操作历史分离。
- 左侧 5 小时、右侧周额度按实际窗口定位，不伪造缺失窗口。
- 刷新时卡片与圆环转动；成功停止且不弹多余成功对话框，部分失败单独报告。
- 复制仅复制账号名称，不复制登录令牌。
- 当前文件账号禁止删除，后端同样校验；普通删除仍须确认，只影响 Atelier 数据。
- 切换和回退仍经过原有的目标预览、备份、明确确认与读回流程。

## 验证与未验证

本轮仅执行类型检查、格式检查、Rust 检查与 macOS 构建。遵照用户要求，不运行测试，不执行真实账户刷新、订阅查询、导入同步、切换、删除或回退。请求兼容性及当前账号变化时的运行效果由用户验收，不能因编译成功宣称与 AiMaMi 完全等效。

## 套餐徽标

独立的 plan.ts 把原始套餐标识转换为显示名称：Plus、Team、Business、Enterprise、Edu，以及 Pro 档位。prolite / pro-5x 为 Pro $100；pro / pro-20x 为 Pro $200；promax / pro-500 为 Pro $500。美元是套餐档位标签，不是用户实际扣费、税费或本币价格。没有标识保持未知，新标识保留原文，不从订阅日期猜 Free。

档位标识参考 [Cockpit Tools 的类型与徽标映射](https://github.com/jlcodes99/cockpit-tools/blob/main/src/types/codex.ts)，当前 Pro 档位与 [官方定价](https://learn.chatgpt.com/docs/pricing) 核对。此处独立实现映射，不直接复制上游整套账号逻辑。

## 重置倒计时和重置卡

参考 Cockpit Tools 的 [重置卡到期组件](https://github.com/jlcodes99/cockpit-tools/blob/main/src/components/CodexResetCreditExpiry.tsx) 与 [时间判定](https://github.com/jlcodes99/cockpit-tools/blob/main/src/utils/codexResetCreditExpiry.ts)，按 Vue 组件和现有刷新流程独立实现；不是声称移植了未公开的 AiMaMi 账号组件。

数据结构采用 [官方 app-server 协议](https://learn.chatgpt.com/docs/app-server)：availableCount 是权威总数，credits 为 null 表示未取得明细，空数组表示返回的明细为空；明细数量可能少于总数。expiresAt 明确 null 才显示无到期限制，缺少或无效保持未知。最近到期只考虑可用记录；已用/处理中/未知状态不作为下一张可用卡计时依据。详情最多保存 256 条有界显示记录，不回显不透明卡 ID。

卡片票券徽标显示真实可用数量，替换此前固定“1 个目录”的占位。圆环浮层支持悬停、聚焦、点击固定、Escape 和外部点击关闭。预览默认只读缓存，刷新由用户点击；没有接入使用/消耗重置卡的命令。

本次只做类型检查、前端构建、Rust 检查、格式检查及原生打包，不执行真实额度请求或重置卡消耗，真实接口兼容性由用户验收。
