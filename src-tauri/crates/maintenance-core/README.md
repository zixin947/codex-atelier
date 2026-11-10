# Atelier Maintenance Core

从 AiMaMi 1.2.7 arm64 安装包的函数、调用点和常量恢复的 Rust 算法库。输入来自显式路径和类型化状态。库包含真实文件、macOS Keychain 和进程适配器；仅显式调用才执行，没有默认用户目录发现、自动登录或后台运行。测试只用合成数据、MemoryKeychain 和模拟进程，未操作真实客户端。

## 内容

- `config`：profile、第三方残留、全局引用、受管配置一致性检查；四类纯文本修改计划。
- `catalog`：实际文件/slug 检查、完整模型内容构造、持久化 RelayState schema/迁移、私有凭据及 revision 存储、CLI 签名、原子发布与持久化确认。
- `threads`：实际 SQLite/plain/zstd 查询，六组具体修复计划/执行、备份与复检。应用层诊断通过私有数据库投影，避免源 SHM 副作用。
- `auth`：文件认证检查、原生候选恢复、unlock 残留及 takeover 新旧备份恢复、隔离和专用锁。
- `state_db`：真实旧库/WAL/SHM 备份删除、逐文件失败与剩余项检查。
- `image_compat`：原版行级配置识别/合成、mtime+len 缓存、具体工具声明过滤与三次 CAS 写入。
- `process`：原生进程归属、受保护/外部写入者、DB 占用、停止/静默窗口/fence、启动及条件重新拉起；SystemRunner 需显式调用。
- `transition_journal` / `transition_events`：真实恢复记录 schema、提交门禁/持久化和脱敏事件分类、最后 200 行日志保留。
- `policy`：静默窗口分类、Catalog 准备门控、路由协调门控。它不是完整 Manager，也不会执行修复。

配置修改计划的 `remaining_step` 明确区分“发布后复检”和“Manager 重协调后复检”。生成文本不代表已写入，更不代表最终修复成功。

## 验证

在仓库根运行：

```sh
cargo fmt --manifest-path src-tauri/crates/maintenance-core/Cargo.toml -- --check
cargo test --offline --manifest-path src-tauri/crates/maintenance-core/Cargo.toml
```

测试只使用内存输入和临时目录，包括真正的 SQLite、压缩记录和文件读写；没有真实客户端运行差分。

## 证据与边界

目标二进制 SHA-256：`b2a4b1b2b4babd2518e02c0f0d6203ad19e779909a47b3b643844ef403fe95de`。它是既有离线副本，不作为仓库依赖分发。

`evidence/` 保留各恢复分工的原始索引，索引中的 C 文件名、行号和地址对应本机 `relay-recovered/maintenance-full/pseudocode/` 与 `codex-direct/decompiled/`。随库保存的汇编位于各分组的 `asm/`，路径布局与恢复时的索引不同。静态证据不是原产品运行观察，也没有恢复成原作者 Rust 源码。

已接入 Atelier 实际诊断/修复入口的范围与仍待接入的部分见 [恢复记录](../../../docs/research/aimami-direct-backend-recovery.md)。持久化 schema、CLI 签名和原生发布确认已实现；完整 Manager 意图转换/补偿、fix_all 全分支与剩余维护功能仍未全部接通。
