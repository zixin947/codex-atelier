# AiMaMi 后端直接恢复记录

2026-11-08，按用户要求由 Codex 直接接手原生后端分析。复用已分析的 Ghidra 数据库 `AiMaMi.arm64.0`，只读批量导出函数，并用 ARM64 调用目标、常量和返回分支交叉核对。没有继续派发给 DSH，没有使用 Computer Use。

目标离线副本 SHA-256：`b2a4b1b2b4babd2518e02c0f0d6203ad19e779909a47b3b643844ef403fe95de`。分析产物仍在本机 `relay-recovered/maintenance-full/codex-direct/`；可编译实现已移入 `src-tauri/crates/maintenance-core/`。

## 恢复与接入

| 部分 | 已落地的真实逻辑 | Atelier 当前接入 |
| --- | --- | --- |
| 配置检查 | 四项检查，词法状态、managed 边界、Provider 归属、URL 脱敏、实际文件引用、RouterOn/NativeOff 配置契约 | profile/global 已替换实际诊断入口；第三方与 stale 的完整状态适配仍待接入 |
| 配置修改计划 | global/profile/third_party/stale 四类文本变更；不是空回调 | profile/global 接入预览、确认、备份和复检；Manager 重协调的完整等价链仍待补齐 |
| Catalog | 完整内容构造、schema/凭据/revision 迁移、签名、原子发布与耐久性确认 | 原生持久化来源与实际 Atelier Responses Gateway 均已适配；实际诊断及单项目录/TOML修复入口已接通 |
| 线程与项目 | 实际 SQL、主线程过滤、plain/zstd、可信备份、Provider/模型匹配、六组具体修复 | 六组线程检查与计划/执行已替换旧近似实现；私有投影、连续修复和最终复检已接；完整项目状态协调仍待完成 |
| 认证 | accounts/backups 原生恢复、unlock 新旧 marker/backup、takeover 新旧 schema、隔离与锁 | 实际预览/确认/修复入口已接通；凭据字节不进入 IPC |
| 旧库 | 根目录 DB、WAL、SHM 的逐文件 copy/remove 与失败残留 | 已接入流式摘要、备份读回、复检；不合并或修改活库 |
| 转换日志 | 两种 target、精确四种活跃阶段、pending commit 保护、目录同步失败语义、脱敏事件与200行轮转 | 实际诊断和应用入口使用真实日志门禁；生产 HostManager 已接实际转换，完整模式/组合仍待验收 |
| 修复门控 | 完整 quiet-window 分类、按报告推导 Catalog/transition/reconcile 条件 | 静默窗口分类已接入；生产 Manager 已连接转换/补偿/重新拉起，完整 RouterOn/批量组合仍未完成 |

这批恢复不等于全部 35 项诊断与全部修复行为已经一比一完成。

## 直接证据修正的旧稿错误

1. `fix_requires_quiet_window` 的共享返回块并非未确认。`0x10037dc64..0x10037dc84` 对精确字符串匹配返回 1，否则返回 2。rollout orphan、项目状态、线程 Provider、线程模型目录四项均需要静默窗口。
2. `0x10070b0fc` 只匹配 `router_postcondition && fixable && status != "ok"`，不是 `db_orphan_providers`。孤儿 Provider 的 Catalog 准备是另一条门控。
3. `PreparedRepairCatalog::prepare` 仅在选中的可修复、非正常孤儿 Provider 项上进入，不是所有修复无条件准备。
4. 单项 `router_postcondition` 有直接协调路径。一般修复也并非无条件先协调、再 transition。
5. `RepairAudit::step @0x1004a0768` 实际调用 `CodexProcessLifecycle::launch`。仅因主函数里没看到直接 launch 就断言不重新拉起，是遗漏包装函数。
6. 全局配置检查不依据本地 Provider/profile/模型白名单删除配置。实际检查空推理档位、特定缺失的外部目录引用；原生加载器承担其余支持性判断。

7. TransitionFailure 里的两个字段是阶段 u8 与尝试次数，不是两个 bool。Manager 在 stage>=2 或 attempt>=2 的转换失败路径进入补偿；普通单项修复错误不因此补偿。该结论由 run_repair 的写入点与主函数消费共同核对。
8. Manager 初始 `Option<String>` 阻断来自构造阶段 storage::load 错误（RelayManager::new @100688540），不是把任意 transition journal 的存在当作该字段。应用现有预览门禁是独立的 Atelier 边界，不能用它宣称完整原生 Manager 已恢复。

9. 全局引用检查的保护根由原生 `CodexPaths +0x248/+0x250` 提供，为 relay 目录，不是整个 home。Native Catalog 的默认输出根也是 relay。工程侧已修正 Context 映射，并把配置输出路径和引用保护根分为两个参数；Gateway 的独立目录布局保留。

Manager 的恢复门控在 `maintenance-core/src/policy.rs`，实际 launch/compensate 包装函数的完整原生调用证据保留在分析目录。尚未将不完整的旧 Manager trait 草稿接到真实客户端。

## 尚缺的关键链

- 完整 RouterOn/NativeOff 意图持久化、路由转换驱动、pending commit 恢复、目标态复验及补偿；转换日志本身不能顶替该流程。
- `fix_all` 全分支、多轮诊断、孤儿 Provider 接管、前置 Catalog 准备门控、后置 reconcile、普通失败与转换失败不同的补偿条件。
- 原生进程命令与完整 Manager 静默窗口、fence、条件重新拉起的最终接线。进程模块自身已实现并在 fake runner 中验证。
- 官方目录获取/新鲜度、完整项目状态、清理/导出/发送/Image 的逐项安装版语义审计。
- fallback/no-account journal 的原生 serde 语义已补证（必填 nullable、deny_unknown_fields、版本和别名）；完整模式恢复仍须继续。
- 真实客户端差分验证留给用户；本批从未执行真实配置/凭据/会话库读取或操作客户端。

## 应用适配的明确差异

- 原生 Catalog 的 `provider::model` slug 不能直接送给当前 Atelier Gateway。适配器依据真实 Gateway `/v1/models` 和 Responses 转发实现，将它转换为实际可路由的原始模型 ID；没有把异构 Gateway JSON 交给缺字段默认的 RelayState 解析器。
- 已存在的相对 rollout 路径按 Atelier 的 home-relative 契约只在私有 DB 投影中展开；实际数据库仍保留原始相对字符串，并以原字符串执行 CAS。没有声称原版也按此规则解析。
- 未知旧 schema 缺 `source` 时不伪装成主线程；报告未完整读取，避免错误接管子线程。
- 默认不从测试中探测真实安装包。调用者可显式提供 bundle，从实际捆绑 CLI 计算签名；None 准确表示不可用。

## 验证

使用 `cargo test --offline --manifest-path src-tauri/crates/maintenance-core/Cargo.toml` 检查恢复算法；使用 `cargo test --offline --manifest-path src-tauri/Cargo.toml maintenance::` 检查工程维护链。所有数据均来自临时目录和合成记录。

未读取、修改或测试真实 Codex 配置、认证、会话库和钥匙串；未启动、结束、重启真实 Codex/ChatGPT/AiMaMi；未改 UI。

## 生产 HostManager 接线（本检查点）

新增 app `native_manager.rs` 将已有配置/认证/线程修复、真实 Router 原语与核心 HostManager/转换引擎连接。Tauri 的诊断、预览和确认入口已使用它。Gateway 来源由实际 Catalog 和单独 schemaVersion=1 的 RouterIntent 表达；无来源是显式不存在，不能伪装成默认 RelayState。NativeOff 实际 Native 源沿用其状态和 commit receipt。

`repair::MutationGuard` 实际持有维护配置锁和线程修改锁，内部修复/路由协调复用该 guard，避免自己拦截自己的转换日志及锁重入。根目录、过期票据、配置/目录/源存在状态与 SHA-256、原生 typed 依赖和最终预览效果均保留核验。

容量预检与完整 Manager/持久化/恢复日志证据已保存到 maintenance-core/evidence 下，原来的 DSH 候选 trait 不进入生产。第三方配置检查/修改接入实际核心算法；来源损坏不会继续沿用旧目录 heuristic 的成功结果。

仍需继续的具体缺口：Native RouterOn proxy 宿主、Native primary 共享配置与路由认证/接管组合、完整 fix_all 的多轮/交叉数据库 effect ledger、孤儿 Provider 全接管与真实官方目录协议和新鲜度，以及维护工具全部安装版语义逐项验收。不得用 scoped 测试通过宣布迁移完成。

同批后续：旧宽泛 orphan 遍历已替换为真实受管候选集、官方目录与具体模型/effort 接管，健康 foreign Provider 不参与。批量执行以真实 SQLite/文件后验签发不透明 effect token，线程适配器只消费原始依赖匹配的批准前缀；保留外部变化和触发器失败回滚。官方 model/list 代码使用明确固定 stdio 参数，config/read 前后核来源和分页；测试显式 FakeRpc，默认禁止发现或启动本机 app-server。Native global/profile primary 与 takeover 也有独立纯组合 token 和真实后继计划，不采用任意三路 merge。app 维护过滤套件最终 331 项通过，原生二进制构建通过。
