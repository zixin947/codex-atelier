# 维护工具验证记录

本轮仅使用隔离数据与模拟进程，未执行真实 Codex 的诊断、修复、重启或强制结束。

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
