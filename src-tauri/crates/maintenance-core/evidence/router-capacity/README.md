# 转换前磁盘容量检查证据

离线目标为 AiMaMi 1.2.7 的 arm64 切片，SHA-256 `b2a4b1b2b4babd2518e02c0f0d6203ad19e779909a47b3b643844ef403fe95de`。未执行目标程序。这里保留 Ghidra 反编译记录，不是原始 Rust 源码。

- `ensure_transition_rollout_write_capacity`：`0x1008d54e8`。
- `rollout_history_logical_len`：`0x1008afa38`，普通文件实际长度；压缩记录流式解压到 sink 计数。
- `require_existing_rollout`：`0x1008b51a0`，路径/压缩替代解析，不额外猜测会话身份。
- `rollout_path_identity`：`0x1008b3298`，解析后 canonicalize，失败保留实际路径。

实现为 `../../src/router_capacity.rs`，在严格转换前由 `router_transition` 调用。预算为恢复复制和必要数据库备份的物理字节数、32 MiB 保留空间，以及最大的四个活动记录逻辑长度之和；恢复复制目标按真实路径去重。严格模式要求收敛记录存在；缺失记录修复模式测量已验证的可信备份，无法恢复时计数据库备份。

可用容量由现有 `fs2::available_space` 查询；原版对该查询错误采取 best effort，本实现保留 `None` 和警告，不把错误改成虚构的可用空间。容量算法不发布修复文件；SQLite 只读连接仍可能由 SQLite 管理读取所需的边车状态。

验证仅用临时 SQLite、普通/压缩记录、可信备份及显式临时卷；12 项独立测试通过。未读取真实会话库或启动客户端。
