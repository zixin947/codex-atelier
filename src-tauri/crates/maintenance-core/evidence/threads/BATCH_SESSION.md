# 线程原生批量 Session

`native_threads_session.rs` 将六组实际修复逐项派发：missing → provider → model → poisoned → response IDs → legacy。每个初审选中 ID 只能调用一次；首项不会提前执行其余动作。可证明的普通失败仍可继续后项；无法确认的部分变更会污染 Session 并阻止后续写入。

`begin_reviewed_after_batch_prefix` 消费真实 opaque coordinator/resync/orphan 证书，逐原始依赖核 before，再核当前 after。Router 证书是累计 original→final，不能当 pure→final 重复串接。fresh plan 仅允许原批准 union 内目标收缩。全体原始依赖首写前校验仍由外层 repair 执行。

实际文件读回必须是原件或原生 plan 的精确 replacement；DB 私有副本保留原始 schema/triggers，重放实际 SQL，并与全部 schema/表/列/typed values 的逻辑 digest 比较。只在真实后验吻合后接受物理 DB/WAL hash/mtime；外部无关列变化也不会被签成合法效果。

`ApprovedThreadEffects` 构造私有；`absorb_global`、`absorb_ordinary`、`absorb_project` 只接各真实 producer 的 opaque 证书并核连续边。`Session::verified_effects` 先核完整 ledger，再补原始 plan 只读依赖的 original→ledger edge；不从任意 current 重新签名。此证书交最终 Manager commit。

原生各组 SQL、归属、backup、plain/zstd 处理证据见本目录 REPAIR.md 与同目录 C；Session 的 CAS/审阅协议是 Atelier 适配，并不冒充原版额外功能。

实际临时测试 `cargo test --lib maintenance::diagnostics::native_threads_tests -- --quiet`：28/28（`/tmp/atelier-native-thread-session-final.log`）。包括逐项实执行、SQL trigger 回滚后继续、missing→provider、真实 orphan 前缀+zstd、顺序/重复拒绝、后验外部 DB 改动污染、只读最终 receipt 与晚到变化拒绝。没有操作真实客户端或真实用户数据。
