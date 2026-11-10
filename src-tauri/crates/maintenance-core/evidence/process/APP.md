# Atelier 原生进程命令适配

仅静态分析 AiMaMi 1.2.7 arm64，SHA256 `b2a4b1b2b4babd2518e02c0f0d6203ad19e779909a47b3b643844ef403fe95de`。没有运行被分析客户端、真实 ps/lsof/quit/kill/open，也没有读取真实 config/auth/会话数据库。

## 原版命令调用链

- `commands::system::force_kill_codex@0x1007b3a94`：`command_force_kill_codex.c:71-77` 获取 CodexMutationGate，随后直接 `CodexProcessLifecycle::force_close_all`。不是仅匹配选定 bundle 的旧 Atelier 强杀。
- `commands::system::restart_codex` 的实际 blocking closure 内联于 `BlockingTask::poll@0x100476ed8`。`restart_codex_blocking_poll.c:141` 持 CodexMutationGate，`:180-183` 准确调用 `prepare_transition(...,5,0,0)`，仅成功才 `launch`。离线 ARM64 `0x100476fd8..0x100476fe8` 设置 w1=5/w2=0/w3=0，即 Duration{secs:5,nanos:0} 与 policy0 ForceCloseAll。不能用 policy1 DesktopTransition 推测替代。
- engine policy0 选取全部原生相关进程，包括 CLI，排除受保护进程；另合并 native state DB holders。3秒/300ms强制重试，清空后检查受保护/归属未知持有者。restart正常退出阶段最多5秒，再走同一 policy0 fallback；之后原生 launch 先open、观测主PID、再次open激活；任一步失败传播。底层证据见 README 的逐函数表。

## 应用接口

`src/maintenance/process.rs` reexport `PreparedAction`、`ProcessPreview`、`prepare_native(action)`、`apply_native(&plan)`；实际实现为同目录 `native_process.rs`。生产 IPC 保存 typed plan，调用者不能传 PID 或 shell 命令执行。可测试入口 `prepare(&ProcessContext,&impl Runner,action)`、`validate(&plan,&impl Runner)`、`apply(&plan,&impl Runner)` 全部显式依赖。

`ProcessPreview` 的 camelCase IPC 字段为 action/scope/targets/externalCli/protectedDbHolders/codexHome/sourceDirectories/sourceDatabases/bundle。目标给 PID 与可执行名称，完整 command identities 只存私有计划，不实现 Debug/Serialize；不把 CLI 可能携带的会话 prompt 参数返回预览。ForceKill 没有发现 Desktop bundle 时仍可预览并结束 CLI。Restart 要求实际 bundle ID/executable 校验通过。

源目录准确采用原生有效 config.sqlite_home > CODEX_SQLITE_HOME > home/sqlite+home，relative sqlite_home 使用捕获 cwd；candidate列为实际原生numeric排序列表。apply前校验上下文/config摘要/来源目录/DB集合/安装与主路径；已退出目标允许消失，新目标或 PID command identity变化要求重新预览。执行窗口内的相关进程重生属于预览中明确说明的全相关进程scope，仍由真实原生重试流程处理。该 preview/identity CAS 是 Atelier 的授权适配，不声称原版存在同样ticket协议。

受保护持有者不会被结束；按原生顺序，它们可以在其它相关进程已经结束后导致整体失败。错误明确保留不可回滚边界；restart stop失败不会继续launch。成功依赖真实 engine 的相关进程清空/holder检查或实际main PID观察；不把发出命令冒充已完成。外层 IPC 负责与其它维护事务使用相同 mutation gate。

原来 bundle-only ForceRunner 和无强杀 restart helper 仅保留为旧fixture的 cfg(test)兼容代码；生产不再包含窄bundle-only signal runner，也不以旧helper路径执行新ticket。

## 合成验证

`cargo test --manifest-path src-tauri/Cargo.toml process::native_tests --no-default-features --quiet`：8 passed。覆盖 CLI-only无bundle、self/无关进程排除、官方IDE扩展保护与真实holder失败、5秒policy0强制fallback与两次open、protected阻止restart、PID/config过期、配置目录优先与numeric DB排序、3秒强杀超时、第二次open失败。Runner使用合成进程表/虚拟时钟，所有磁盘输入为本次临时目录；未调用SystemRunner。

另实际执行底层 `cargo test --manifest-path src-tauri/crates/maintenance-core/Cargo.toml --test process_native --quiet`：34 passed。应用 `cargo test --manifest-path src-tauri/Cargo.toml maintenance:: --no-default-features --quiet`：208 passed（0 failed）；这是本次并行集成快照，不代表真实客户端E2E。
