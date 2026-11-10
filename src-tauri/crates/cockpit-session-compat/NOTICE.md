# Cockpit Tools 原样移植

上游：jlcodes99/cockpit-tools，提交 02498ace91832248f0c6d43f6ea3894a2514aed7。

upstream/ 内文件原样保留，SHA-256 见 UPSTREAM-SHA256.json。版权归上游作者及贡献者；本部分按 CC BY-NC-SA 4.0 使用与分发。许可：https://creativecommons.org/licenses/by-nc-sa/4.0/ 。原始许可说明保存在 UPSTREAM-README.md。

Atelier 的宿主适配单独位于 src/，不得将这些适配声称为上游原始实现。仅包含线程可见性/历史索引修复及必要支持文件，不引入上游账号登录、代理或应用启动器。

## 宿主适配边界

- 原版修复核心及其测试原文件不改；Atelier 传入明确的目录、线程 ID、目标与快速/深度模式。
- 平台层单独适配：备份放入 Atelier 管理目录，使用 SHA-256 目录分区；保留备份，不执行上游自动裁剪。
- 进程检查在 macOS 上保守阻止任意仍运行的 Codex/ChatGPT 引擎，不实现上游多实例精准归属；Windows 未验证。
- 官方元数据重建调用原版代码，但必须由宿主明确提供引擎路径；未提供时返回失败，不回退启动真实客户端。测试用临时目录，不授权真实引擎。
- 不读取上游账号/实例仓库和认证资料。账号启动恢复接口明确返回不支持；相关上游测试单独排除，不能声称完成全产品移植。管理代理注入不接入，使用宿主进程既有环境。
- Atelier 外层新增预览有效期、来源指纹、确认票据和恢复新数据保护。这些不是上游原文件的一部分。

验证上游测试的命令：`cargo test --manifest-path src-tauri/crates/cockpit-session-compat/Cargo.toml -- --test-threads=1 --skip desktop_api_key_launch_restores_onboarding_state_without_project_metadata`。原生隔离 Codex 测试保留上游 ignored 标记。


### 正常修复入口纠正

移除 Atelier 自加的正常修复预览快照一致性门槛及过期拦截。执行时由原版核心重新扫描、备份和处理并发；无关 WAL 更新、检查点和会话追加不再因整库哈希变化被直接拒绝。目录归属/票据绑定仍保留，数据库快照恢复仍保留新数据保护，避免用整库备份覆盖后续聊天。修复后的恢复校验无法读取时仅提示恢复受限，不把已执行的上游修复误报为执行失败。

隔离回归覆盖：预览后 WAL 更新、checkpoint、追加日志和标题更新后仍可实际修复；会话字节保持，新增数据阻止快照恢复。没有操作真实 Desktop。
