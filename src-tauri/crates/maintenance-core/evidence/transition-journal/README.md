# Router transition journal 恢复证据

输入为离线 AiMaMi.arm64，SHA-256 `b2a4b1b2b4babd2518e02c0f0d6203ad19e779909a47b3b643844ef403fe95de`。静态恢复不等于运行原程序，也不是原作者 Rust 源码。

- `read_checked @100790b68`：relay 目录 `router-transition-journal.json`；symlink_metadata 只准普通文件；take(1MiB+1)；先 Envelope schemaVersion=1 再完整反序列化。
- `TransitionJournal deserialize @100272a90`：u8 schemaVersion、target、phase、i64 startedAtMs/updatedAtMs 必填；reason Option<String> 可缺省。target 逐字为 routerOn/nativeOff，没有 noAccount 变体。
- `write_inner @10078e254`：普通写不得覆盖 committing；相同目标保留起始时间，改变目标重设；实际 private atomic publication；成功写和 pending 拒绝均追加脱敏 event。
- `begin_commit @100790988`：committing 发布后 require_sync；目录同步失败仍保留已经发布的恢复记录。
- `read @100792574`：checked read 错误记录 warning 并返回 None；in_flight @100792720 精确匹配 retrying、committing、reconciling、stopping_codex。
- `commit_pending @1007911b8`：checked read，错误传播，只匹配 committing。
- `clear_inner @10078dd58`：普通清除拒绝 pending；指定匹配 target 可清除（用于验证/恢复），目标不匹配拒绝。删除后目录同步失败返回 ROUTER_JOURNAL_REMOVED_SYNC_UNCONFIRMED，不能误称原记录仍在。
- `append_event @10078ebd8`：独立 evidence/transition-events 记录完整有限 source/reason 分类和最近200行；追加失败不改变主转换结果。

应用层已有 preview/apply 门禁：诊断读取活跃阶段；确认前 checked read，损坏/未来 schema 不被普通修复覆盖。完整 Manager commit/补偿/恢复驱动仍未完成。

JournalIo 仅用于复现发布后同步失败。LocalJournalIo 是实际文件实现，不接受调用者的“已经写成功”布尔值。所有自动验证在临时目录；没有读取真实 Codex。
