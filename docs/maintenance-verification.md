# 维护工具验证记录

本文件按批次保留历史结果。开头是较早 UI/脚手架批次，最新后端迁移验证在末尾；历史成功不代表当前完整原版功能已对齐。所有开发验证仅使用隔离数据与模拟进程，未执行真实 Codex 的诊断、修复、重启或强制结束。

- `pnpm test`：82 项通过。
- `pnpm build`：TypeScript 与 Vite 构建通过。
- `cargo check --manifest-path src-tauri/Cargo.toml`：通过。
- `cargo test --manifest-path src-tauri/Cargo.toml maintenance:: -- --test-threads=1`：65 项通过。
- 网关 HTTP 转发隔离集成测试：1 项通过，确认同一网关下一次请求立即使用 Image 开关。
- `pnpm tauri build --bundles app`：macOS 原生 .app 构建通过，22.99 MiB；未启动真实客户端测试。
- `git diff --check`：通过。
- 浏览器实测 1440×900、1280×800 与 960×700；后两尺寸检查无横向溢出，1280 诊断弹窗边界位于窗口内。
- 实际操作了诊断、详情、批量修复预览及确认、修复后复检、Escape、发送表单；确认修复结果焦点和预览标题焦点。
- 根据截图增加诊断分组固定标题、异常项淡色底，并修复步骤切换焦点。减少动效偏好匹配已确认。
- 原生保存文件 helper、ZIP 解包/CRC、测试 HTTP 接收端、发送失败、进程范围、配置回退、数据库事务、WAL 完整性及源文件不变均有隔离验证。

原生系统保存对话框、真实账号环境、真实 Desktop 修复及进程控制未执行，留给用户验收。没有上传到第三方接收端。未提交或推送此次改动。

截图位于 `docs/screenshots/maintenance/`，均为演示数据。原生构建结果另见交付说明。

## 2026-11-07：配置语法迁移增量

本次在合成配置和临时目录中验证配置语法修复的实际 `repair::preview` / `repair::apply` 调用链，没有启动或操作真实 Desktop，也没有读取真实账号、配置或会话库。

- `cargo test --manifest-path src-tauri/Cargo.toml syntax_workflow_tests -- --nocapture`：6 项通过。
- `cargo test --manifest-path src-tauri/Cargo.toml maintenance::diagnostics -- --test-threads=2`：66 项通过，包含上述 6 项，不能相加统计。
- `git diff --check`：通过。

接入回归验证了计划文本、预览保持原配置与备份目录不变、外部更新时拒绝覆盖、确认后写入剥离文本、逐字备份原文件、单项与全部修复入口，以及抢救后仍无效时返回 `failed_ids` 而非虚报修复成功。

新增回归位于 `src-tauri/src/maintenance/diagnostics/syntax_workflow_tests.rs`。另修正原集成测试的备份读取方式：按返回的 `backup_path` 检查 `0000.bin`，保留原始字节断言。

本次未重复前端、原生打包和真实 Desktop 验收。通过结果仅证明当前配置语法切片与已有诊断回归通过；认证恢复、模型目录、线程污染恢复、状态库与完整批量调度等缺口仍在继续迁移，尚未完成全量维护功能对齐。

## 2026-11-07：线程提示污染恢复模块增量

- `cargo test --manifest-path src-tauri/Cargo.toml poisoned_workflow_tests -- --nocapture`：5 项通过。LF、CRLF 合成记录验证替换元数据后没有插入空记录，后续记录字节保持不变；测试曾复现重复换行，修正后通过。另两项以临时 SQLite、明文与压缩 rollout 验证单一健康来源的实际预览、确认、备份与复检，健康来源不被修改；新增归档诱饵记录验证归档线程不会成为恢复来源。
- `cargo test --manifest-path src-tauri/Cargo.toml poisoned_instructions -- --nocapture`：7 项隔离单元测试通过，使用内存文件系统替身。
- 接入修正后，`cargo test --manifest-path src-tauri/Cargo.toml maintenance::diagnostics --quiet -- --test-threads=2`：79 项通过，包含上述测试。

基础 `preview` / `apply` 接入已通过上述样本验证，原有位置锚点保护回归也已恢复通过；当前实现拒绝改写受保护布局，不代表已恢复上游的位置索引处理语义。来源收集已改为专用查询，归档过滤有独立样本验证；模板缓存键的含义、多个健康来源的最终选择顺序及相关 fallback 分支证据仍待补齐。不能把基本样本通过视为与上游全量行为一致。未读取真实会话或执行真实客户端操作。

## 2026-11-07：通用文件创建、删除与回滚接入

`FileChange` 的前后状态改为 `Option<Vec<u8>>`：`None` 表示不存在，`Some([])` 表示真实空文件。确认流程支持创建、写入及删除；清单版本 2 记录操作种类、前后存在性和摘要。创建不会伪造空原件备份，删除会先逐字备份原件。后续步骤失败时逆序恢复；回滚发现外部修改则保留外部文件并报告。

- 恢复了 DSH 误回退的配置语法、线程指令修复接线；之前的 6 项语法与 5 项线程工作流测试继续通过。
- 新增 7 项隔离接入测试：真正删除并备份、未选文件保留、写空文件、创建及清单、外部创建拒绝覆盖、后续失败回滚创建与删除、删除后外部重建保留、悬空符号链接拒绝。第一项同时覆盖删除与未选文件保留。
- 注册独立 `auth_restore` 模块后运行 `cargo test --manifest-path src-tauri/Cargo.toml maintenance::diagnostics --quiet -- --test-threads=2`：93 项中 92 项通过、1 项失败。失败为 `auth_restore::tests::multiple_candidates_use_deterministic_order`，只有带后缀备份时错误返回 `NoBackup`；已交给 DSH 继续按原版语义修正，未隐藏或放宽断言。
- `git diff --check`：通过。

这些结果证明通用接入能力，不代表残留清理或认证恢复的业务入口已经接通，也不代表完整维护栏目一比一完成。未重复前端或桌面测试，未修改 UI，未读取或修改真实 Codex 数据。

## 2026-11-07：残留清理业务接入

依据 DSH 的 `12-residual清理.md`，将原版 `codexmate/relay/thread-router-migrations/current.json` 和 `codexmate/relay/relay-state-cache.json` 接入检查及修复，同时兼容 Atelier 自有 gateway 下的对应产物。路由使用中返回正常并保留；确认关闭后才允许备份、删除、读回及复检。单选只处理选中类型；`legacy_migrated_threads` 继续单独处理，未用删除记录代替迁移修复。

- `cargo test --manifest-path src-tauri/Cargo.toml residual_workflow_tests --quiet -- --test-threads=2`：6 项通过，覆盖缓存单选、清单单选、全部残留、路由启用保留、预览后启动路由拒绝、外部更新保护。
- `cargo test --manifest-path src-tauri/Cargo.toml maintenance:: --quiet -- --test-threads=2`：122 项中 120 项通过、2 项失败。两项均在 DSH 正在调整的 `auth_restore` 模块：`invalid_backup_structure_is_rejected` 和 `multiple_candidates_use_deterministic_order`。当前同目录候选枚举与合成 fixture 路径不匹配，已反馈 DSH；认证入口尚未接通，不据测试变化声称迁移完成。
- `git diff --check`：通过。

本次未进行真实账号、配置、状态库或进程操作。残留工作流通过不代表状态库、catalog、Manager 与其他维护功能已完整对齐。

## 2026-11-07：认证恢复工程接入

DSH 已交接 `auth_restore.rs` 的工程修复。修正了两个合成测试的目标与来源目录不一致问题，并把 `Path::ends_with(".1.json")`（比较路径组件）改为断言完整文件名，保留“必须选择第一个候选”的要求。

新增 `auth_plan.rs` 将 DSH 的可注入恢复函数写入操作捕获为内存计划，随后使用统一修复流程确认、备份、写入、读回及复检。恢复来源摘要在确认前校验；IPC 预览不包含凭据正文。文件认证之外的存储模式不走文件恢复，路由使用中不覆盖认证。

- `cargo test --manifest-path src-tauri/Cargo.toml auth_workflow_tests --quiet -- --test-threads=2`：5 项通过，覆盖预览纯读取、逐字恢复及原件备份、缺失认证创建、来源外部变化、路由启动保护和 keyring 模式排除。第一项同时覆盖预览与确认。
- `cargo test --manifest-path src-tauri/Cargo.toml maintenance:: --quiet -- --test-threads=2`：127 项全部通过（包含上述测试）。修正了上次记录的两个认证失败，不删除或忽略测试。

这证明当前 DSH 恢复算法的工程接入通过，不证明候选目录/排序/过滤已与原版全部分支一致；DSH 的 09 号文档仍需与模块结论统一并补证。未读取真实认证文件或钥匙串，未联网验证凭据，未执行真实 Desktop 验收。

## 2026-11-07：状态库及模型目录初稿复核

DSH 生成并注册了 `state_db_relocation.rs` 和 `catalog_rebuild.rs`，当前二者均未接入生产修复入口。状态库初稿新增 WAL 测试曾错误嵌入函数，导致编译失败；工程侧已修复函数/测试模块作用域，保留测试断言。

- `cargo test --manifest-path src-tauri/Cargo.toml maintenance:: --quiet -- --test-threads=2`：136 项全部通过。
- `cargo check --manifest-path src-tauri/Cargo.toml --quiet`：通过，仍有未接入模块及重复适配器的 unused/dead_code 警告。
- `git diff --check`：通过。

新增 9 项内存模块测试仅证明初稿当前行为，不证明原版等价。状态库实际目录门控及 WAL/SHM 分支、模型目录真实 schema/字段/默认值/别名/排序已反馈 DSH 继续逆向；缺充分证据前不把初稿接入当成已完成。DSH 上游 502 明确终止后，已在同一通道续接任务，没有因观察超时或慢运行重启。

认证工作流另补“预览后发生新登录”的回归，分别覆盖原认证存在和缺失两种起点，确认保留外部登录、不创建修复备份、不改来源。`cargo test --manifest-path src-tauri/Cargo.toml auth_workflow_tests --quiet -- --test-threads=2`：6 项通过。此次只追加测试，未重复整套检查，不能将 6 与前述 136 相加。

## 2026-11-07：撤回初稿中的无依据行为

DSH 的新 `13-catalog_schema.md` 明确区分模型目录和 relay-state：原初稿的 `activeByIde`、`schemaVersion` 等混入了另一文件结构，`providerId` 与 `tier="relay"` 也没有足够依据。工程侧移除了从 slug 列表臆造目录的实现，改为独立的 `publish_prepared_catalog` 阶段，要求上游提供完整预生成 JSON，序列化时保留全部字段、类型与输入顺序。真正的模型构造、默认值、别名和排序仍未实现，不能将该发布层替代完整重建功能。

- `cargo test --manifest-path src-tauri/Cargo.toml catalog_rebuild --quiet -- --test-threads=2`：4 项通过，覆盖描述信息完整保留、relay-state 拒绝作为模型目录、准备失败不发布和写入失败传播。
- DSH 撤回了“字符串池有 WAL/SHM 即证明同步删除”的初稿推断。依据修订后的 11 号报告，研究模块暂时只建模主库 copy/remove；其内存测试已改为明确验证“不虚构边车副作用”，保留 WAL/SHM 原字节，不将此视为真实 SQLite/WAL 验收。原三文件删除断言依赖被撤回的推断，不再作为验收依据。完整 WAL/SHM、多旧库和回滚语义仍待还原，模块继续未接入生产。
- 最新 `cargo test --manifest-path src-tauri/Cargo.toml maintenance:: --quiet -- --test-threads=2`：138 项全部通过，包含前述测试；`git diff --check` 通过。

DSH 曾再次越过工程分工修改仓库，已由操作者停止该轮，并将其界面访问模式改为“工作区内修改”；DSH 明确确认仅写 `relay-recovered/maintenance-full/` 分析产物，随后继续原版分支追踪。此停止源于实际越界修改，不是慢运行或观察超时。没有改通道，没有操作真实 Codex。

## 2026-11-07：无需停止 Desktop 的两项残留修复

DSH 的 `14-fix_requires_quiet_window.md` 将 `residual_cache`、`residual_manifest` 裁定为返回 0（无需静默窗口）。新增 `quiet_window.rs`，仅为已确认的两个例外放开运行中 Desktop 的限制；其他已证实项要求静默，未解共享分支继续保留原有要求，不将全部 35 项检查冒充已知修复 ID。

`repair::preview` 现在为仅包含这两项的计划返回 `requires_desktop_closed=false`；`apply` 重新依据实际 item_ids 判断，而非无条件拒绝运行中的 Desktop。路由关闭、外部变更、备份、读回和复检条件保留；未增加任何停止或重启命令。现有界面已经读取该字段，本次没有修改 UI。

- `cargo test --manifest-path src-tauri/Cargo.toml residual_workflow_tests --quiet -- --test-threads=2`：9 项通过。新增用例以 `desktop_running=true` 的合成上下文验证单项及全部残留修复、混合认证仍要求退出、路由重新启用仍拒绝。
- `cargo test --manifest-path src-tauri/Cargo.toml maintenance:: --quiet -- --test-threads=2`：142 项全部通过。
- `cargo check --manifest-path src-tauri/Cargo.toml --quiet`：通过，仍有未接入研究模块和重复适配器的警告。
- `git diff --check`：通过。

没有调用真实进程发现/停止/重启。14 号报告的共享 case ID 与返回值消费分支仍需继续核对；完整 Manager、fix_all、多轮补偿与其余原版语义仍未完成。

## 2026-11-07：DSH 工具启动环境恢复

DSH 新的 Manager 分析被 Ghidra 的 JDK 发现/保存步骤阻塞，日志为 `Unable to prompt user for JDK path, no TTY detected`。工程侧仅检查了公开工具启动脚本和现有 JDK，未运行反编译器、未打开 AiMaMi 或其分析项目。

- 既有 Zulu JDK `java -version`：exit 0，21.0.4 aarch64；Ghidra 本地配置要求最低 21，无需安装或升级。
- 用项目内独立 `user.home` 运行 `LaunchSupport ... -jdk_home -save`：exit 0，返回既有 JDK 路径；目标文件与分析项目均未传入。
- 创建 `relay-recovered/maintenance-full/tools/ghidra-local-env.sh`，仅为工具子进程设置 JDK、PATH 和项目内 Java 用户目录；已通过同一启动辅助验证，未改全局 HOME、客户端配置或系统 Java 设置。此脚本属于分析工作区，不是 Atelier 应用功能。
- DSH 上游随后明确返回 502 并终止，已在原通道提交同一任务续接，保留上述局部环境与工作区权限。

15 号 Manager 报告的 `all` 入口、运行状态方向和停止/等待方法存在缺口，已逐项反馈 DSH；未以有序符号列表替代完整状态机。此次没有产品源码变化，不重复运行上一轮 142 项维护测试。

续接后，DSH 界面确认 Ghidra 已恢复，并实际进入新的 Manager 辅助函数反编译及结果读取；随后发现地址需要校正，正在以修正地址继续同一分析任务。该状态仅证明工具执行恢复，不证明 Manager 分支已完整还原或已接入。目标分析始终由 DSH 执行，工程侧未加载目标。

DSH 后续已生成新的 `decomp-fix_diagnostic_issue_audited_main.c`（22:45:27，62,309 字节；工程侧只检查文件名、时间和长度，未阅读或分析反编译正文）。当前任务已从辅助函数和调用清单推进到主函数本体，仍由 DSH 核对实际分支。另补正定位要求：不能一律排除 Rust async 的 poll/业务闭包，只应排除无关格式化闭包或纯包装器。尚未据此宣称完整 Manager 已完成或接入。

## 2026-11-07：确认阶段转换门禁

依据 DSH 的 Manager v3 语义报告中入口转换门禁，诊断和修复确认统一读取转换标记及运行时 starting/stopping 状态。预览后才开始转换也会在任何备份或写入前拒绝，包括无需停止 Desktop 的残留修复。此处仅接入已确认的门禁，不替代尚未交付的完整 Manager。

- `cargo test --manifest-path src-tauri/Cargo.toml maintenance:: --quiet -- --test-threads=2`：145 项通过，0 失败；新增三项均使用合成上下文，覆盖转换标记、starting/stopping 和无需静默例外。
- `git diff --check`：通过。
- 未调用真实客户端进程或修改真实客户端文件；完整 Manager、批量分发及补偿仍未验证完成。

已向同一 DSH 任务下达源码交付要求：恢复模块和源分支对应测试仅写入分析工作区，先交付 Manager 与 fix_all 的可编译实现。未恢复的条件必须明确标记，不能由调用方布尔值或空适配器掩盖；工程侧继续负责接入 Atelier，未改变完整维护栏目范围。

## 2026-11-07：独立恢复包工程复核

DSH 已在分析工作区生成 `recovered-maintenance/` crate，尚未接入 Atelier。初稿仅有 Manager trait，`fix_all` 缺少实际重同步与二次诊断，且丢弃子结果的 `failed_ids`；工程侧已提交具体源码复核清单。随后候选增加 Manager 函数、两次诊断调用和失败项合并，但入口转换门禁、快照消费、变更判断、协调与补偿条件仍不完整；部分分发表仍用候选列表替代确切分支。其他模块引用“完整实现见 Atelier”不能作为恢复完成证据。

- 对该独立 crate 执行 `cargo test --offline --quiet`：首先因锁定的 uuid 版本未缓存而失败，不是源码验证通过。
- `cargo test --locked --quiet` 下载所需依赖后编译失败：`manager.rs` 缺少 `fix_issue` 导入，`FixResult` 缺少候选中使用的 `Default`。
- 工程侧仅修复上述导入和 derive，不改变恢复逻辑。随后 `cargo test --offline --locked --quiet`：12 项通过，0 失败。该测试集未覆盖完整 Manager 控制流和原版批量修复，不构成语义对齐证明。
- `git diff --check`：通过。此次没有产品源码接入、UI 修改或真实客户端操作。

复核反馈写入分析工作区 `recovered-maintenance/ENGINEERING_REVIEW.md`。DSH 原轮界面明确停止后，已在同一通道续接；窗口控制出现观察/输入异常，经重建 CUA 连接后确认消息送达、DSH 已读取复核文件并继续执行，没有重启 DSH 或客户端。

## 2026-11-08：错误与补偿分支修订

用户修复通道并恢复目标后，DSH 的 `deepseek-v4.1-flash` 已产生实际分析输出。其新语义报告纠正旧草稿：普通单项修复及后置协调错误不触发补偿，转换失败且变更已开始才进入补偿；补偿失败时追加恢复错误。工程复核已明确撤回“所有 Err 都应补偿”的可能误解，不能按审查者预设改变原版行为。

DSH 已在独立恢复包增加对应控制流和共享有序 trace。工程侧实际执行 `cargo test --offline --locked --quiet manager::tests`：10 项通过，0 失败。测试范围为候选 Manager 的错误分支与顺序，不涵盖完整原版状态结构、批量分支或真实客户端。此次只审查 DSH 交付的 Rust 和语义报告，没有执行目标逆向工具。

当前仍不得接入生产：`PostFixReconcileSignal` 来源、转换阶段两个字段的语义及主错误载荷、锁的持有/释放契约、批量路径的快照与补偿、resync 实现体及 35 项检查编排仍不完整。DSH 已交付 `UNRESOLVED.md` 登记缺口，并收到连续恢复下一批的任务；不会将 trait 或未知标志替代为假成功。Atelier 产品源码、UI 与真实客户端此次均未改动。


## 2026-11-08 Codex 直接恢复批次

本批只改 Rust 与恢复证据，没有改 UI，也没有运行针对真实客户端的诊断/修复。

- `cargo test --offline --manifest-path src-tauri/crates/maintenance-core/Cargo.toml`：73 个单元测试 + 7 个线程一致性集成测试 + 14 个线程/项目集成测试，共 94 项通过。
- `cargo test --offline --manifest-path src-tauri/Cargo.toml maintenance:: -- --nocapture`：146 项通过。覆盖本批 profile/global 实际预览、确认、原件备份、外部变化校验和复检；其余测试覆盖先前维护功能。
- `cargo check --offline --manifest-path src-tauri/Cargo.toml`：通过，有 22 条未使用代码/变量等警告，不能据此声称零警告。
- `cargo fmt --manifest-path src-tauri/crates/maintenance-core/Cargo.toml -- --check`：通过。
- `git diff --check`：通过。

原生 AiMaMi 运行差分、真实 Desktop 行为、新核心的完整生产状态适配及 Manager/批量修复链尚未验证。测试数量不代表 35 项维护功能全部对齐；具体范围见 `research/aimami-direct-backend-recovery.md`。

## 2026-11-08：真实恢复模块、应用接线与组合修复

这一批由 Codex 直接分析离线 arm64，复用 Ghidra 数据库并以短 ARM64 分支核对；没有等待 DSH 或使用 Computer Use。恢复的实现均在仓库内，完整原版维护功能仍未全部完成。

实际执行的检查点：

- `cargo test --offline --manifest-path src-tauri/Cargo.toml maintenance:: --bin codex-atelier -- --test-threads=2`：223 项通过、0 失败。覆盖实际诊断、预览/确认入口、六组线程修复、认证/接管残留、共享配置组合、旧库三文件清理、进程命令与现有维护功能；这是当前接线检查点，不是 35 项原版功能的全量验收。
- `cargo test --offline --manifest-path src-tauri/crates/maintenance-core/Cargo.toml`：268 项通过（84 单元 + 39 auth + 29 catalog + 34 process + 6 state_db + 7 threads_consistency + 14 threads_diagnostics + 31 threads_native_repair + 13 transition_events + 11 transition_journal；文档测试 0）。这些不能与上面的 app 数量相加推导功能完整性。
- `cargo test --offline --manifest-path src-tauri/Cargo.toml image_compat_integration --bin codex-atelier`：1 项本机 loopback HTTP 测试通过。临时 config.toml 的 false→true→false 控制同一运行网关的下一次请求，恢复透传无需重启。未连接真实 Provider 或使用真实凭据。
- `cargo check --offline --manifest-path src-tauri/Cargo.toml`：通过；检查点仍有未使用候选适配器/测试兼容代码等警告，没有声称零警告。
- `git diff --check`：通过。恢复证据和新增未跟踪源码由各分组另查空白格式；未进行 commit/push。

关键真实效果：

1. 多项 config 修复只发布一次已审核的合并结果。实际 `all` 入口验证 takeover 恢复字段、global 清理与 profile 移除共同生效，保留无关表/注释及原件备份。
2. 缺少 auth.json 时，实际 `all` 入口先从 accounts/backups 恢复 A，再清理 unlock 旧备份 B；最终仍为 A，B 只在已审核清理范围中删除。不是各自拿旧快照互相覆盖。
3. 线程诊断只使用私有 DB/WAL 副本；缺失记录恢复/归档、Provider/模型收敛、历史 ID、legacy 和污染指令经过真实文件/DB 修复与最终检查。未知旧 schema 不猜主线程。
4. 旧状态库按主库/WAL/SHM 分别备份后删除，保留失败源并复检。40MiB 合成旧库证明流式路径；symlink 测试证明只删除旧链接，目标原字节保留。
5. 转换提交记录不会被普通操作覆盖；directory fsync 失败时区分已经发布/已经删除的状态。事件只记录有限 source/reason 分类，不保存原始敏感错误文本。
6. 强杀/重启生产 IPC 使用实际 native lifecycle、类型化进程预览和配置/线程共享锁。原版 restart 的 5 秒 graceful wait + policy0 fallback 用 fake runner 验证，未调用真实 SystemRunner。
7. Image 开关以真实 config.toml 行级读取/合成为准，不以独立 JSON 偏好冒充生效；默认临时 Store 不发现 HOME，生产显式传配置路径。保留原版 tools/additional_tools 过滤及 tool_choice 原值。

真实 Codex 配置/认证/会话库/Keychain 未读取或修改；真实 Codex/ChatGPT/AiMaMi 未启动、停止或重启；没有真实账号/代理接入测试。此次没有修改 UI 设计、没有新窗口截图或原生桌面验收。完整 Manager 转换/恢复/补偿、router_postcondition、剩余原版维护语义仍是后续工作，不能用这些通过结果替代。

本批另据原生参数偏移修正目录边界：Native Context 的模型目录根为 `home/codexmate/relay`；全局引用排除也仅保护该受管目录，不保护整个 Codex home。新增 `fix_config_global_validity_at(config_text, config_path, managed_root, fs)` 分开实际输出路径与判断根，避免把配置写到 relay 目录。实际入口新增回归证明 home 内、relay 外的缺失引用会清理，relay 内受管引用保留；7 项 native_workflow_tests 通过。数据库显式目录进一步统一 config→env→cwd 优先级，未展开 `~`；缺省跨目录选库的完整原版算法仍待验证，不据此声称全部恢复。

- `cargo test --offline --manifest-path src-tauri/Cargo.toml configured_database_directory_uses_native --bin codex-atelier`：1 项通过，显式配置优先于环境，空配置退环境，relative 按注入 cwd，波浪号保留为字面路径。该测试是在上述 223 项检查点后新增，不将它误写成已经重新跑过的整套 224 项。


## 2026-11-08：生产 HostManager 与路由提交接线

本检查点已将 Tauri 的诊断、预览和确认修复入口切到真实 `native_manager`。测试只使用临时目录、合成认证/会话、FakeRunner 和本机 loopback；没有启动、停止或重启真实 Desktop，也没有读取真实配置/凭据/会话库。测试默认禁止新孤儿 Provider reader 发现或启动系统 app-server，协议测试显式注入 FakeRpc；非测试构建保留真实固定命令实现。

实际验证：

- `cargo test --offline --manifest-path src-tauri/crates/maintenance-core/Cargo.toml`：385 项通过（含 Manager 65、转换 13、目标验证 11、容量 12、原生清理 16），文档测试 0。这是核心检查点，不与 app 数量相加推断覆盖。
- `cargo test --offline --manifest-path src-tauri/Cargo.toml maintenance::diagnostics::native_manager::tests --bin codex-atelier -- --test-threads=2`：18 项通过，含真实 NativeOff 状态提交、Gateway RouterOn 回执、配置修复后路由收敛、模拟 Desktop 退出和重开、未知/错误/不匹配回执保护、来源从无到有的 CAS、实际 auth 阶段失败后的受约束续做补偿。
- `cargo test --offline --manifest-path src-tauri/Cargo.toml maintenance:: --bin codex-atelier -- --test-threads=2`：此前接线检查点 293 项通过。随后加入孤儿 Provider 具体迁移并替换旧宽迁移后，最新整套为 **307 通过、4 失败**；4 项失败均暴露共享数据库批量组合的待完成工作（压缩记录、Provider+tool ID 合并、外部变化与 SQLite 失败恢复），不能改用小套测试宣称全绿。完整 `fix_all` 尚未验收。
- `cargo check --offline --manifest-path src-tauri/Cargo.toml --bin codex-atelier`：通过；该检查点 58 条未使用代码/变量等警告，不宣称零警告。
- `cargo fmt --manifest-path src-tauri/crates/maintenance-core/Cargo.toml -- --check`：通过。
- `git diff --check`：通过；新增文件仍需在各分工冻结后单独复核，不以只检查已跟踪文件替代。

18 项流程测试实际证明的错误分支：NativeOff 配置先发布、后续 auth 锁因临时目录故障失败，保留首个错误；只有 journal 已真实记录 failed 后解除合成故障，补偿重新核验有序批准 effect 前缀，清理旧 marker/备份，保留当前用户 A 与原配置备份，最终提交真实 Native receipt。没有用 UI 状态或返回 Err 代替文件/收据/日志读回。

仍未验证/未完成：全部 Native RouterOn 宿主、完整 Native primary 配置与认证组合、孤儿 Provider/多轮批量修复的共享数据库 effect ledger、官方目录新鲜度，以及其余维护工具逐项安装版差分。真实 Desktop E2E 由用户验收；本轮无新增 UI/窗口视觉验证、无提交/推送。

### 同批后续：批量交叉写入与 Native 配置组合已验证

上段 307 通过 / 4 失败是中间检查点，以下实测优先于该失败状态：

- 运行 `cargo test --offline --manifest-path src-tauri/Cargo.toml maintenance:: --bin codex-atelier -- --test-threads=2`：**331 项通过、0 失败、36 项未进入该过滤范围**。日志检查点 `/tmp/atelier-maintenance-manager-final.log`。
- 4 个旧失败案例的行为断言全部保留，改为显式 FakeRpc 官方目录注入；测试没有发现或启动系统 app-server。孤儿修复实际执行 SQL/文件变更并生成不可由调用者伪造的批准前缀，后续线程计划逐一核原始 before / 实际 after 后重新规划，只允许原已批准目标全集内执行。
- Native Manager 专项现为 **20 项通过**（包含在 331 中，不另加计数）：新增修复后重新读取实际 sqlite_home，避免语法恢复后仍检查旧目录而发布假健康回执；以及 global 清理后 takeover 恢复配置不会把空 reasoning 字段加回。
- Native Router 原语专用 **32 项通过**、Native Threads **20 项通过**、第三方配置适配 **11 项通过**、孤儿 Provider **19 项通过**均包含在该 app 套件内。
- `cargo build --offline --manifest-path src-tauri/Cargo.toml --bin codex-atelier`：**通过**，产生原生可执行文件；当前非测试构建有 **61 条警告**，不声称零警告。只构建，不运行客户端或 Atelier 窗口。
- `git diff --check` 与 root 本批 Rust 文件的 rustfmt 检查通过。新增容量证据仅去除导出尾部空行/空格，不改反编译语句。

尚未完成完整迁移。新增 Native primary token 仅证明具体 global/profile 纯算法与 takeover 的已批准组合，不支持把任意当前配置当新授权；auth-primary 交叉效果、完整多轮 fix_all、Native RouterOn 宿主、官方目录新鲜度、项目状态完整分支等仍需继续。真实 Desktop 端到端运行留给用户，未因测试全绿改为已验证。


## 2026-11-08 本轮生产批次接线检查点

- `native_manager::preview("all")` 现直接生成 `native_batch::ReviewedBatch`，不再把旧的 combined single-item ticket 当作原生全部修复。
- `apply_primary` 实际执行原生 batch，保留 fixed/failed IDs 与备份路径；最终提交前验证同一预览签发的 sealed effects/config receipt。失败不会改成成功，也不从任意当前配置生成批准。
- 生产 Manager 合成端到端：25/25 通过，日志 `/tmp/atelier-manager-native-batch-final.log`。新增覆盖完整 all 语法恢复与提交、仅残留批次不退出模拟客户端、预览后新增认证候选首写前拒绝。
- 批次生产适配专项：10/10 通过，日志 `/tmp/atelier-production-native-batch.log`。
- 官方目录真实 loopback HTTP 联调：4/4 通过，日志 `/tmp/atelier-official-http-integration.log`，验证选中身份、失败无旧目录回退、参数检查与来源切换后的过期请求拒绝。
- 所有上述测试只使用显式临时根、FakeRunner/FakeRpc 与 loopback HTTP；未读取或操作真实客户端、认证、会话库，未执行真实 Desktop 验收。
- `git diff --check` 已通过。全量维护套件本轮未重新执行，不能把上述专项之和作为全量通过数。
- 未完成：批次后 Full 验收确实仍需要额外路由重写时，尚缺预审的有限后继计划；目前明确保留进展与恢复记录。35项整体语义对齐、原生宿主目录观测和跨重启恢复仍须继续核验，不能据此宣称全量迁移完成。

- 本轮最终原生编译：`cargo build --offline --manifest-path src-tauri/Cargo.toml --bin codex-atelier` 实际通过（18.33秒），日志 `/tmp/atelier-native-batch-production-build.log`；74条编译警告尚未清理。仅编译，未启动或打包应用。

## 2026-11-08 余项补齐后的最终回归（优先于以上检查点）

### 已实现与接线

- `native_batch` 在预览时封存有限后继候选及 Global 精确分支；`native_router` 校验 source/intent、owner、目录成员和累计效果依赖，认证重新计划仅允许缩小原批准范围，目录payload不得变化。`native_manager` 接最终Full失败→一次性后继→累计凭据→提交链。失败先撤销旧凭据，不用任意当前字节续签。
- 新增生产用例以 Global-only 保留旧managed model 的真实文件状态触发最终Full失败，明确检查 `final-router/successor-config-original.json`、最后配置及实际intent receipt；不是仅注入验证布尔值。Manager专项26、batch专项13及owner拒借反例均包括在最终应用维护套件中，不重复加总。
- `thread_observation` / core `threads::history` 替换旧history近似，源DB只用私有快照；结构扫描受硬字节预算限制，partial/缺文件/错误不假报完整。history仅观察，不自动修复。相较原版managed/openai producer，应用复用当前主线程collector会覆盖external活跃主线程，这是明确适配差异。
- 目录观察按原版四目录及磁盘阈值实现；config readiness只有metadata，纠正旧“原版替换探针”判断。实际目录探针走独立预览/确认、5分钟单次票据和互斥；缺目录不创建、正式config/DB不修改、对象变更拒绝、清理失败显式报告。只读观察接受祖先路径alias，写探针仍拒绝祖先symlink，不静默canonicalize生产目标。
- expiry覆盖missing/过期/近到期/refresh说明；core新增不迁移的`observe_api_key`，仅显式授权legacy观察器调用。Native revision与Gateway真实schema分开处理，未授权legacy不读取Keychain。
- 清理备份增加SHA-256、dev/inode、目录身份，并在每项删除前重新校验；不扩大集合，失败保留，不删除profile凭据和last-switch。最后核验至path unlink仍不是OS原子事务，不宣称消除全部瞬时竞态。

### 本轮实际执行

| 命令或检查 | 结果 |
| --- | --- |
| `cargo test --offline --manifest-path src-tauri/Cargo.toml maintenance:: -- --test-threads=2` | **452通过、0失败，40项不在该过滤范围**；`/tmp/atelier-maintenance-final.log` |
| `cargo test --offline --manifest-path src-tauri/crates/maintenance-core/Cargo.toml` | **414通过、0失败**，含3项新增只读凭据测试；`/tmp/atelier-core-final.log` |
| `cargo build --offline --manifest-path src-tauri/Cargo.toml --bin codex-atelier` | 通过，75条警告；`/tmp/atelier-build-final.log` |
| `cargo check --offline --manifest-path src-tauri/Cargo.toml --bin codex-atelier` | 通过；`/tmp/atelier-maintenance-check.log` |
| `pnpm test -- tests/maintenance.test.ts` | 实际脚本跑了全部8个测试文件，**84/84通过**，其中维护21项 |
| `pnpm typecheck` / `pnpm build` | 均通过 |
| 限定修改文件rustfmt / `git diff --check` | 通过；未对既存dirty tree做全仓自动格式化 |
| 浏览器演示 | 查看35项诊断与独立写检查预览；明确风险、未勾选确认按钮禁用；截图已检查1280×720排版。仅Mock，无真实文件写入 |

中间应用回归曾因只读观察错误继承探针的no-follow限制导致macOS `/var` alias下配置不可修复，以及新路由用例未真实触发终态Full失败而失败；修正观察/探针边界和用例触发条件后，最终全套已重跑通过。没有删掉旧配置修复断言来绕过失败。

### 仍未完成/未验证

- NativeOff原版筛选closure尚未恢复，当前明确warning/unknown，不按空白、双冒号或在线allowlist猜测；该项不算等价完成。
- Native RouterOn独立owned proxy及官方目录生产请求宿主仍缺；Gateway handle/scope不能冒充。
- 生产Native legacy Keychain未获授权，维持未确认；未在线验签/刷新，不声称真实登录验收。
- 未打开源DB RW，源库写入/sidecar能力仍未验证；目录探针不代表该能力。
- 跨重启批准恢复协议、35项逐分支安装版差分、真实Desktop E2E均未完成。
- 未签名、公证、打包替换应用、提交或推送；此为上述检查点状态。本轮使用临时fixture/FakeRunner/FakeKeychain/loopback，不操作真实Codex。

## 2026-11-08 WAL修复失败复现与重复扫描修正（最新检查点）

### 原因和修正

用户报告最终 `state_5.sqlite-wal` 效果变化，恢复又被同一错误阻断。新增 `all_wal_missing_history_repair_commits_without_verification_sidecar_changes`，仅临时 WAL 数据库、缺失历史行和 FakeRunner，即在修改前复现同样的两段错误；日志 `/tmp/atelier-wal-repro-test.log`。不是用户真实数据的现场追踪，不能断言所有真实失败均为这一原因。

- 最终路由验证直接打开源SQLite，可能为已关闭WAL库创建空WAL/SHM，改变已记录物理效果；转换容量预检亦有同样问题。应用Host现将两者都通过 `snapshot::with_verification_paths` 提供的有生命周期私有DB/WAL副本读取。容量算法、磁盘检查和路由不变量未删除。core默认宿主行为保留，应用宿主提供真实的私有快照容量检查，不是假成功回调。
- Native/Gateway与无路由来源Off的最终验证，包括ObservedOff中间/后继验证，均使用私有副本；源文件前后稳定性检查仍在。保留真实源路径错误说明，不用immutable丢弃已提交WAL。
- 回执仍按实际批准效果校验；真正外部改写继续拒绝整批提交。恢复不再让冲突的WAL回执遮蔽独立源路由观察：只接受原批准配置候选并检查恢复门禁和真实Full目标；已符合源目标时不重放批次、不写回执、不覆盖数据库；不符合则明确需重新检测，保留材料，不声称整批回滚。
- 预览复用同轮普通报告，删除Manager和batch准备之间的额外全量扫描；六项线程检查复用同一轮私有投影。跨写入阶段不复用旧报告，不删除原版修复前/重同步后的必要诊断。

### 原版证据及差异

原版 `decomp-fix_all.c:131,879` 明确有两轮阶段性诊断；`decomp-fix_diagnostic_issue_audited_main.c:1207,1284,1301` 显示all/单项派发及后继审计，之后有客户端恢复分支。现有固定批次顺序和有限重试保留。私有快照、预审票据和累计效果校验是Atelier适配，不冒称从反编译直接得到这些机制；本次修正其与SQLite生命周期的冲突，而非宣称完全复制原版。

### 验证

- `cargo test --offline --manifest-path src-tauri/Cargo.toml maintenance:: -- --test-threads=2`：最终 **457通过、0失败**，含最后恢复门禁补充；`/tmp/atelier-wal-maintenance-verified.log`。
- `cargo test --offline --manifest-path src-tauri/crates/maintenance-core/Cargo.toml`：**414通过、0失败**；`/tmp/atelier-wal-core-final.log`。
- `cargo test --offline --manifest-path src-tauri/Cargo.toml accounts:: -- --test-threads=2`：**25通过、1忽略、0失败**；`/tmp/atelier-wal-accounts-final.log`。忽略既有真实一分钟定时用例。
- `pnpm test`：**92通过**；`pnpm build`（含TypeScript检查）通过；`/tmp/atelier-wal-frontend-final.log`、`/tmp/atelier-wal-build-final.log`。
- `cargo check --offline --manifest-path src-tauri/Cargo.toml --bin codex-atelier`：通过，**78条警告**，未清理；`/tmp/atelier-wal-check-final.log`。该命令为最后恢复门禁补充前的编译检查点，最终维护测试已重新编译该改动。
- 测试覆盖Native/无来源Off、保持连接的已提交WAL与连接关闭边车缺失状态，验证和容量预检不改源DB/WAL/SHM；外部改写继续拒绝，独立源路由观察不生成整批健康回执；未批准配置不能借恢复被覆盖。

### 合成性能对照（debug、单次样本，不是原版跑分）

`/tmp/atelier-wal-maintenance-final.log` 中测试先执行旧结构再执行新结构，存在缓存和并发测试噪声，不把单次毫秒值作为生产SLA。

| 数据/阶段 | 旧调用结构 | 新调用结构 | 确定性变化 |
| --- | --- | --- | --- |
| Global-only预览 | 17.90 ms | 5.59 ms | 普通诊断3次→1次，批准项目相同 |
| 32线程，每线程32条约512字节合成事件，六项检查 | 102.17 ms | 70.58 ms | 私有快照6份→1份，六项结果完全相同 |
| 修正后Global-only确认执行 | 无成功基线 | 228.55 ms | 使用FakeRunner，不代表真实客户端停启时间 |

尚未测量用户真实数据规模、真实Desktop修复、与AiMaMi同数据差分或完整UI进度；不能声称已经追平原版。未执行真实修复、未清理用户恢复材料、未打包替换当前运行应用、未提交或推送。

## 2026-11-08：操作级共享观察（当前代码检查点）

### 实现与边界

- 普通诊断及同轮完整目标检查共用操作级观察；预览的普通报告与目标检查同轮复用。真实写入规划、确认执行和修复后验证不沿用上一阶段缓存；每个独立验证入口建立新的观察轮次。
- `threads::observation` 一次流式解析同时产生首条身份/cwd、首尾Provider、最新settings、基础指令状态、前五行参考指令、工具ID计数和历史结构摘要。只保留必要字段/摘要；参考指令仅本轮内存使用，不持久化正文或令牌。
- 规范化路径与Unix设备/inode合并同物理文件读取；plain优先规则保持。Provider预取在观察轮内不另开绕过缓存的工作线程。没有作用域时独立入口仍使用原算法，可作为对照。
- 每次缓存复用以及报告返回前重核文件长度、修改时间和Unix身份/ctime；重新解析plain/zstd选择，文件替换、流中改写、压缩路径新增plain均拒绝旧观察。源DB/WAL也在返回前重核。
- DB/WAL只从稳定私有副本观察，已提交WAL通过私有SQLite合并为本轮只读基线；各可变路径投影从基线另建副本，不能污染共享基线。主线程collector在同一私有路径复用行结果；不同SQL筛选和不同可变投影仍独立查询，不声称所有线程索引SQL只执行一次。
- 移除随后被原生结果覆盖的旧线程扫描；损坏配置分支保留unknown。诊断不再为健康工具ID文件准备写计划；健康指令历史和参考模板读取共用摘要。孤儿Provider和项目索引诊断不再为封存修复依赖而重复哈希所有历史。
- 总解压观察上限512 MiB、单文件64 MiB、单记录`0x1800001`，另有摘要/字段/行数上限。超限或读取不完整时不由该观察证明Provider/工具ID健康。history仍按自身逐文件公平预算、行上限、drain、尾换行、ordinal和首次history_base规则重放摘要，不混同其他检查的完整性规则。
- 原Native/Gateway归属与外部Provider规则未改。本次没有处理先前真实对照中的3条最终Provider差异，也没有恢复NativeOff尚未确认的模型筛选closure。

### 当前验证

所有本批测试仅使用临时目录、合成SQLite/rollout和既有FakeRunner/loopback。没有读取真实登录凭据或真实会话，也没有确认真实修复。

- `cargo test --offline --manifest-path src-tauri/Cargo.toml maintenance:: -- --test-threads=2`：限定格式整理和最后WAL变更测试后 **459通过、0失败**；`/tmp/atelier-observation-maintenance-verified.log`。
- `cargo test --offline --manifest-path src-tauri/crates/maintenance-core/Cargo.toml`：**420通过、0失败**；`/tmp/atelier-observation-core-final.log`。包括6项新观察测试；不与子集计数相加。
- `cargo test --offline --manifest-path src-tauri/Cargo.toml accounts:: -- --test-threads=2`：**25通过、1忽略、0失败**；`/tmp/atelier-observation-accounts-final.log`。忽略既有真实一分钟用例。
- `pnpm test`：**92通过**；`pnpm build`（包含TypeScript检查）通过；`/tmp/atelier-observation-frontend-final.log`、`/tmp/atelier-observation-build-final.log`。
- `cargo check --offline --manifest-path src-tauri/Cargo.toml --bin codex-atelier`：通过，**93条警告**；`/tmp/atelier-observation-check-final.log`。移除旧扫描调用暴露了额外dead_code，本批未做无关清理。此检查点在最后仅测试补充和限定格式整理之前；随后维护测试已重新编译最终代码。
- 限定本批Rust文件执行`rustfmt --edition 2021 --config skip_children=true --check`通过；`git diff --check`通过。未整仓格式化。
- history差分矩阵覆盖plain/zstd、空文件、CRLF、无尾换行、坏JSON/UTF8、重复/下降ordinal、首次base、不同字节/行/drain预算；专测原生超长行、首尾meta/settings与工具ID共享、硬链接去重、替换/流中改写/plain优先变化、预算不足、新轮重读。
- 完整生产report合成测试：12个混合plain/zstd历史，观察计数12次打开、解压字节等于文件逻辑长度之和；源DB稳定快照1次，源DB字节及边车存在性不变。下一轮新增坏记录被发现。计数来自观察器，不是操作系统全进程I/O追踪。
- 新WAL作用域测试：同轮两次读取共用1份快照，末尾外部提交令本轮失败；下一轮重新建1份快照且读到提交。既有WAL修复、预览变化/CAS与恢复门禁回归继续通过。

### 合成性能与未完成验收

`/tmp/atelier-observation-maintenance-final.log`，debug、先独立后共享的单次样本，含文件系统缓存及并发测试噪声；不是用户真实数据或AiMaMi对照跑分。

| 样本 | 独立调用结构 | 本轮共享结构 | 观测 |
| --- | --- | --- | --- |
| 32线程×32条约512字节合成事件，六项检查 | 109.71 ms | 51.60 ms | 快照6→1；共享历史打开32次；解压589078字节；保留摘要估计1295650字节；六项结果相同 |
| Global-only预览 | 18.79 ms | 5.76 ms | 普通诊断3→1；批准项目相同 |
| 合成Global-only确认执行 | 无成功基线 | 263.04 ms | FakeRunner，不代表真实客户端停启 |

摘要值是实现内估计，不是进程RSS；旧路径读取/解压字节与峰值RSS尚未通过OS追踪测量，不能宣称完整I/O或内存对比已完成。超过观察预算时覆盖可能比原先无界的Provider/指令读取更保守，这是明确安全限制，不标为健康。

异常文件为判断布局可修复性、真正封存写计划、依赖哈希/CAS、备份及修复后读回仍允许额外读取；缺失历史的备份候选、迁移快照JSON、配置/目录也不属于“一次rollout观察”承诺。没有宣称整个预览/确认/修复全过程只打开每个文件一次。

本批未打包替换运行应用，未重新测真实Desktop耗时、未执行真实修复或新的AiMaMi同数据差分；不能据上述合成样本声称真实约10秒已降至某个秒数或已追平AiMaMi。未删除恢复材料，未提交或推送。

## 2026-11-08：真实失败后的原版规则复用修正（最新检查点）

用户在上一版真实修复中再次遇到Full验证失败：3条Provider不一致、2条历史观察不完整；预览Provider改动为数据库行0、文件0。以下修正只在隔离数据验证，尚未读取这些真实历史，不能认定截图中的全部失败已解决。

### 来源核对与直接复用范围

- 本机 `/Users/Zhuanz/Documents/AiMaMi/src-tauri/src` 的公开源树不含安装版的维护修复模块。可用材料是安装版反编译C/ARM64与 `relay-recovered/maintenance-full/codex-direct/threads` 的Rust还原实现，不是完整上游原始Rust源码。
- 对照原恢复版 `threads/src/consistency.rs`，保留原SQL筛选、`apply_provider_convergence_observation`、首尾meta/settings读取及Provider计数规则；生产crate主要差异为模块路径和本轮观察入口。没有用旧 `recovered-maintenance` 的不完整Manager草稿替换现有链路。
- NativeOff最终检查现直接调用既有 `verify_provider` → `inspect_thread_provider_convergence_with_reader`，不再对原生/openai行重复套用Atelier的“所有历史锚点必须等于openai”规则。外部历史保留、corrupt-only不单独阻止原版目标门禁的已证实规则随原入口保留；诊断仍单独展示损坏。Atelier自有行继续独立检查，未通过放宽Atelier归属让目标变绿。
- 对应证据：`maintenance-core/evidence/router-verification/README.md` 的原版gate `@100362e6c..e88`，以及 `codex-direct/threads/CONSISTENCY.md` 和原版 `apply_provider_convergence_observation`。复用的是已恢复实现；私有快照、预览票据和写前保护仍是Atelier适配，不冒称原版原样代码。

### 已复现与修正

- `/tmp/atelier-source-provider-repro.log`：修正前实际复现“原生规划器保留外部历史且零改动，Atelier最终验证却报Provider不一致”。修正后同一断言通过，外部文件原字节不变。
- `/tmp/atelier-source-coverage-repro.log`：修正前实际复现“结构摘要耗尽后，已经完整读取的Provider证据也被丢弃”。现在结构摘要达到限制后停止积累结构记录，但在原字节/记录上限内继续流式读取其他证据；history仍明确partial，不将结构覆盖标为完整。
- 普通观察不能覆盖时，Provider专项直接复用原版首meta/普通文件倒序尾部/zstd重开流式遍历及原provider解析函数，不使用部分流推断最新锚点。专项仍有单文件64 MiB读取预算和操作级512 MiB预留预算，错误明确指出字节/行/倒序尾部上限；预算未覆盖时仍拒绝验证。此为有界适配差异，不是原版无界行为。
- 正常完整文件保持一次共享观察；专项回退允许额外打开与解压，独立缓存其结果并核对源稳定标记。`observation::metrics`未包含专项回退的全部打开/解压量，不能用该计数证明异常文件也只读一次；专项预算按每文件预留额计费，并非OS实际I/O计数。
- NativeOff预览与确认首写前复用原生Provider观察：无法读取时拒绝；存在目标差异而批准计划没有实际Provider/缺失历史/legacy效果时拒绝。单项票据依据实际线程计划效果，而不是只看项目ID。有Atelier自有线程却没有对应NativeOff迁移能力时明确拒绝，不在写后才发现。
- 仍保留最终Full验证、外部变更CAS、备份和恢复记录。当前保护没有证明所有非零计划都覆盖全部失败分支；Gateway RouterOn线程收敛/迁移仍未完整实现，不能宣称全模式一比一迁移完成。

### 真实写入链的隔离验收

- `native_all_repairs_owned_drift_and_preserves_external_history_through_full_commit`：在临时DB中同时放入外部Provider历史和受管漂移zstd历史；实际走Manager预览/确认、批次写入、Full复检及回执校验。受管文件从aimai1改为openai，外部文件字节不变，报告最终目标ok，回执有效。进程使用FakeRunner，不是用户真实修复。
- `native_provider_preflight_rejects_zero_effect_and_preserves_external_history`：零效果不能解决真正漂移时写前拒绝；真实受管修复后重检通过；外部历史零改动保留。
- `direct_native_off_preview_rejects_unplanned_atelier_threads_before_writes`：缺少对应Gateway线程迁移计划时预览拒绝，配置/DB/历史原字节、备份目录和意图不存在状态不变，无进程调用。
- `confirmation_rechecks_unplanned_provider_rows_before_process_or_file_changes`：预览后新增未批准Atelier线程，确认拒绝；源配置/DB不变，不建立备份/回执，不操作进程。
- Provider回退扩展测试覆盖65 MiB普通文件与最新settings、plain/zstd无效settings、源改写、历史摘要耗尽；原有读取次数、WAL边车和预览变化保护回归继续通过。
- 测试开发曾因plain/zstd样本同名触发plain优先、未配置受管Provider定义而触发FakeRpc保护、未制造可修复项目而过早返回等失败。分别修正fixture隔离与前置条件，未删除断言，也未启用真实Codex或真实目录RPC。

### 最终验证

- `cargo test --offline --manifest-path src-tauri/Cargo.toml maintenance:: -- --test-threads=2`：**464通过、0失败**；`/tmp/atelier-source-maintenance-final.log`。
- `cargo test --offline --manifest-path src-tauri/crates/maintenance-core/Cargo.toml`：**424通过、0失败**；`/tmp/atelier-source-core-final.log`。
- `cargo test --offline --manifest-path src-tauri/Cargo.toml accounts:: -- --test-threads=2`：**25通过、1既有忽略、0失败**；`/tmp/atelier-source-accounts-final.log`。
- `pnpm test`：**92通过**；`pnpm build`（含TypeScript检查）通过；`/tmp/atelier-source-frontend-final.log`、`/tmp/atelier-source-build-final.log`。
- `cargo check --offline --manifest-path src-tauri/Cargo.toml --bin codex-atelier`：通过，**93条警告**，未做无关清理；`/tmp/atelier-source-check-final.log`。
- 仅本批Rust文件限定格式整理；最终`rustfmt --check`和`git diff --check`结果见交付核对。

本批未打包替换、未读取真实凭据或会话正文、未确认真实修复、未删除备份或恢复材料、未提交推送。截图中5条历史的实际归属/大小/覆盖原因、真实Desktop成功结果和速度仍未验证；本批不能称为用户现场修复成功。

## 2026-11-08：写前预算在真实数据上拦截（当前最新修正）

用户替换上一版后，真实“全部修复”再次未开始。截图错误为写前核对拒绝：两条真实rollout分别报 `native provider traversal line/byte budget exhausted` 与 `native provider reverse-tail budget exhausted; not verified`。即该写前保护按设计在首写前停止，未产生任何写入；但拒绝原因是本实现自加的单文件64 MiB专项读取上限，不是原版行为。原版 `session_anchors` 没有该上限，会完整读取这两个文件。

### 修正

- `threads::observation::native_provider` 第一次仍在共享预算内读取；预算耗尽（错误含 `budget exhausted`）时按原版方式无上限重读同一文件，仍保留读取前后源文件稳定标记核对与同轮缓存。损坏（Corrupt）、源变化、无效settings等非预算错误不回退。
- 上批文档中“预算未覆盖时仍拒绝验证”的描述自此不再成立：预算耗尽不再拒绝，只有原版本身无法得出锚点时才报错。有界快路径保留，超限文件按原版完整流式读取（内存仍有界，时间与原版同量级）。
- 新增 `provider_budget_exhaustion_falls_back_to_unbounded_native_read`：预算错误触发无上限重读、结果入同轮缓存、二次调用不再重读、稳定核验通过。既有无效settings、源变化、65 MiB尾部等回归继续通过。

### 验证（仅此修正）

- `cargo test --offline --manifest-path src-tauri/crates/maintenance-core/Cargo.toml --lib observation_tests`：**10通过、0失败**；`/tmp/atelier-provider-fallback.log`。
- `cargo test --offline --manifest-path src-tauri/crates/maintenance-core/Cargo.toml`：**425通过、0失败**；`/tmp/atelier-provider-fallback-core.log`。
- `cargo test --offline --manifest-path src-tauri/Cargo.toml maintenance:: -- --test-threads=2`：**464通过、0失败**；`/tmp/atelier-provider-maintenance.log`。
- `pnpm test`：**92通过**；`pnpm build`（含TypeScript检查）通过；`/tmp/atelier-provider-frontend.log`、`/tmp/atelier-provider-build.log`。
- 未读取用户真实历史正文；两条真实文件能否通过核对、修复是否最终成功，仍需用户在替换后的应用里重新执行确认。本批随后按用户既有授权直接打包替换并重新拉起。

### 真实数据复查（2026-11-08 22:16，用户机器）

替换预算回退版后，用户真实数据的系统诊断显示：**路由完整目标状态（Full）正常**，详情“实际配置与目标状态检查通过；未验证上游生成”；线程已收敛到官方 provider、rollout 文件完整、失效受管 Provider、旧版线程 Provider、迁移清单残留均正常；按钮为“暂无可自动修复项”。此前阻塞修复的“3条Provider不一致 + 2条历史观察不完整”不再出现。助手已亲自在应用中展开该项核对详情文本。

剩余项与 AiMaMi 同数据行为一致：会话历史结构异常（原版同样不自动改写）、线程系统指令/历史响应项ID留意、官方直连模型规则未恢复（如实unknown）、中转密钥与目录写权限未确认。无法区分用户是否实际执行过修复批次；但当前状态通过了此前失败的同一Full检查，且无待修复项。本记录为真实机器观察，不是受控实验结论。
