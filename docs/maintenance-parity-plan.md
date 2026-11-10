# 维护工具：安装版功能对齐

## 目标与已确认范围

以本机 AiMaMi 1.2.7 安装包为行为参考，实现维护栏目全部后端功能。当前阶段不改 UI；原版逆向、Atelier 工程实现与隔离测试现由 Codex 直接执行，复用现有离线 Ghidra 数据库和 ARM64 证据，不再等待 DSH。真实 Codex Desktop 的操作验收由用户完成。

- 系统诊断采用弹窗，打开自动检查、四组结果、逐项详情、单项/全部修复、修复日志和复检。
- 覆盖安装版的全部 35 项检查及其完整修复行为；仅有检查 ID 或相似文案不算功能对齐。
- 补齐诊断包导出、诊断信息发送、清理、强制结束 Desktop、图片兼容、重启入口。
- Claude 联网兼容与上游一样标明尚未开放；保留现有重建注册表功能作为次级入口。
- 依赖 Atelier 自己的账号、网关、线程模型，不创建 AiMaMi 的受管状态，也不把 AiMaMi 接收服务作为默认上传地址。

## 实现决策

1. 使用独立诊断服务与结构化返回值；前端不自行推断修复成功。
2. 上游写入探针、待提交状态恢复等行为需要从二进制恢复真实分支，再通过可注入适配器实现与隔离验证。不能因为禁止测试真实客户端而永久省略功能；当前只读检查属于尚待对齐的现状。
3. 修复延续 Atelier 的预览、明确确认、外部变化检查、备份和读回验证；完整 Manager 的静默窗口、进程协调及重启能力仍需迁移。开发验证使用模拟进程，不能操作真实客户端。
4. Router/Catalog、认证、状态库等缺少配套依赖时，补齐类型明确的适配接口与业务实现，不以“不适用”“只读指导”代替原功能。未解语义登记为未完成，不猜测实现，也不以现有弱校验通过作为一比一证据。
5. 调试包采用允许字段列表，不打包原始凭据、钥匙串、会话正文或任意日志。发送地址由用户填写，发送前显示内容范围并确认。
6. 图片兼容偏好接入 Atelier 网关请求处理；保存偏好不能代替请求行为验收。

## 运行与验证假设

单用户桌面应用，诊断和修复在工作线程运行，界面操作互斥，过期结果不覆盖新弹窗。读取和导出有体积限制；异常和部分完成保留具体结果。无需新增后台服务或遥测。

验证使用临时目录、合成账号/线程数据、模拟进程控制和本机测试 HTTP 接收端。不得运行针对真实 Codex 的诊断、修复、强制结束或重启。浏览器查看演示数据，原生编译验证不等于真实 Desktop 功能验收。

## 来源与证据边界

参考本机安装包提取的实际维护页 JavaScript、Rust 符号和 arm64 调用关系。已确认结构与关键流程，不宣称恢复了完整上游 Rust 源码。具体覆盖和差异以实现后的 `maintenance-tools.md` 为准。

## 当前接入与未完成项（2026-11-08）

已接生产诊断及确认修复入口：配置语法抢救、profile/global、文件认证备份恢复、路由认证残留与 takeover 备份恢复、旧根目录数据库及 WAL/SHM 清理，以及六组原生线程修复。模型目录改为实际 RelayState/Atelier Gateway 类型输入，生成完整目录并实际发布和修正 TOML 引用，不再接受预生成 JSON 作为功能替身。

- 认证备份确证来自 `accounts/backups/*.json`，按原生修改时间倒序；实际先复制第一个合格候选再复检，不能将“复制过”报告为登录有效。没有访问真实 Keychain。
- 线程修复使用源 DB/WAL 的私有快照诊断，执行时重核输入；多项先核对原始目标全集，再按顺序重新计划。无可信历史备份的缺失记录按原版归档；不伪造会话正文。旧表缺少 `source` 时不能推断主线程，明确报告覆盖不足。
- 旧库清理已恢复并接入真实三文件循环；精确 `sqlite_home` 归属、备份成功后才删除、边车部分失败和复检均已实现。大库流式摘要/备份，不继承文本体积上限。
- Catalog 的持久化 schema、凭据迁移、revision 文件、CLI 签名、atomic publication 与 acknowledge/require_sync 已有具体实现。Atelier Gateway 按实际 Responses 转发契约映射，输出原始可路由 model ID；原生 `provider::model` 不会直接送给不支持它的 Gateway。
- 转换日志和脱敏事件日志已具体实现；诊断与确认入口读取真实阶段，提交未完成时拒绝普通覆盖/清除。该能力尚不能代表完整 RouterOn/NativeOff 转换已接通。
- 强杀/重启命令已接原生 lifecycle 与类型化进程预览，静默窗口/fence/重新拉起条件已有 SystemRunner 与模拟测试；这些能力已由生产 native_manager 入口调用共享 HostManager；模拟 Desktop 的正常退出、静默、验证后重开已执行。Native RouterOn 的独立 owned proxy 宿主和全部组合仍未完成。
- `fix_all` 的共享配置组合已接入：takeover、global/profile、Gateway HTTP 契约与目录路径必须合并为一次已审核发布，后续步骤不能以旧快照覆盖前序结果。认证恢复→Unlock 的同文件冲突也按已证实顺序解决；这只是局部修复协调器，不是完整 Manager。
- 生产诊断/预览/确认入口已切到 native_manager，共享 HostManager 调用真实静默/fence/重试、目标态验证与提交。Atelier 的 Gateway Catalog 和 RouterIntent 保持独立 schema；没有伪造原生 RelayState。原生 NativeOff 使用实际 Native 状态和原生 commit ID。direct 后置修复失败按阶段执行补偿；已知 partial effects 的续做必须通过有序前缀和最终效果核验。Native RouterOn 独立 owned proxy、Native primary 共享配置组合及完整 fix_all 仍未验收。
- 孤儿 Provider 的接管、官方模型目录获取/新鲜度、项目状态、维护清理/导出/发送/Image 等仍须逐项对照原版审计；已有 Atelier 实现和通过的测试不能替代该审计。fallback 和 no-account 恢复日志的 schema/必填可空字段/未知字段拒绝已核实并接入严格 Manager guard；不等于相应完整模式转换已接入。
- 未执行真实 Desktop E2E，也没有本轮前端/原生窗口视觉验收；真实客户端操作留给用户。

全量目标持续有效。任何“已接入”只代表当前调用链；不等于全部 35 项检查与全部修复语义完成对齐。历史测试记录保留在验证文档，最新验证范围单独记载，不叠加测试数量冒充完整覆盖。

### 本轮补齐的执行边界

- 配置替换 readiness 按原版仅观察 metadata/类型/权限；纠正此前“原版有实际配置替换探针”的错误。目录检查准确覆盖 home、sessions、codexmate、codexmate/relay，并独立查询剩余空间。
- 新增独立的目录写入检查预览/确认票据；普通诊断和修复预览不写探针。确认后只独占创建随机私有文件、写入/同步并核验身份清理，不创建缺失目录，不改配置，不打开源 DB RW。源库写入/sidecar能力仍未验证，不能把探针成功当作数据库可写。
- history 改为独立有界扫描，覆盖 JSON/UTF8、unsigned ordinal、首 meta 的 `history_base.end_ordinal_exclusive` 与文件名引用、plain/zstd及部分覆盖度。私有快照仍保持源 DB/WAL/SHM不变；不加入自动修复。
- NativeOff 原版模型筛选 closure 尚未恢复，删除旧“模型含空白”的假近似，改为明确未确认。此项仍未完成原版等价，而非已修复。
- 令牌检查恢复缺access、过期/临近到期阈值和refresh说明；只解码声明、不联网刷新。Native revision 按core读取；legacy仅提供显式授权的只读观察器，不使用会迁移写盘的 `get_api_key`；无Keychain授权时维持未确认。
- 清理按Atelier自身Catalog/profile/内嵌quota映射，不套原生schema；备份指纹增加SHA-256与对象/目录身份，每项删除前复核，不扩大预审集合，保留backup与last-switch。最后一次核验至path unlink之间不构成OS原子事务，不能宣称消除全部竞态。
- Native真实代理及官方请求宿主缺失时，拒借Gateway handle/scope，不伪造宿主或完成状态。跨重启授权恢复协议不在本轮新增范围。

## 官方兼容资料的使用边界

2026-11-07 已核对 [OpenAI 配置参考](https://learn.chatgpt.com/docs/config-file/config-reference) 中的 `model_catalog_json` 入口，以及 [Models and inference](https://developers.openai.com/siwc/token-sharing-open-source/models-and-inference) 对 `models`、`slug`、`display_name` 的说明。文档可用于核对消费端公开约定，不能证明 AiMaMi 1.2.7 的模型构造、内部字段类型、默认值、别名或排序语义；这些仍需从目标安装包逐项核对。此次没有请求真实模型目录、读取真实令牌或执行推理请求。

## Codex 直接恢复进展（2026-11-08）

配置检查、纯文本修改计划、目录检查/内容构造以及静默窗口/Manager 门控已移入独立 Rust crate。profile/global 两项已接实际诊断与确认修复入口；global 不再按本地白名单删除未知 Provider。原程序的 RouterOn/NativeOff 状态管理仍不能用 Atelier 网关 start/stop 直接代替。

详细代码、证据、接入边界见 [直接恢复记录](research/aimami-direct-backend-recovery.md)。本批没有改 UI，没有读取、写入、启动或重启真实 Codex 客户端。

## 生产 Manager 接线检查点

`maintenance_run_diagnostics`、`maintenance_preview_repair`、`maintenance_apply_repair` 现在分别调用 native_manager 的 report/preview/apply_native。确认票据保存真实来源、存在状态、原始字节和有界计划；修复与路由协调共享实际双文件锁。只读预览不退出进程、不创建意图、不启动网关。

- 正常修复：阶段验证通过后发布真实路由回执，不把内存状态或持久化 running 字符串当存活证明。
- 提交异常：没有回执或 Full 复检失败时保留 pending；只有匹配目标、实际回执与完整复检同时成立才清除 Atelier 待提交记录。
- 来源变化：从不存在到新增 Native 状态也会使票据失效，不能只比较 On/Off 目标。
- NativeOff 来源确实不存在时仍检查实际 native auth/unlock/takeover 残留，不创建空 Native 状态补齐验证参数。
- 当前仍有功能缺口，见生产覆盖事实表；这些接线不构成全部 35 项原版诊断/修复对齐完成。

后续检查点已验证孤儿 Provider → 线程批量变更前缀衔接，旧四项交叉写入回归恢复通过；Native global/profile 与 takeover 的具体组合也已接入，避免恢复阶段加回已删除字段。它们不替代剩余 auth-primary、多轮 fix_all 和 Native RouterOn 等全量验收。最新 app 维护套件 331 项通过、原生二进制构建成功，详细未完成项继续保留。
