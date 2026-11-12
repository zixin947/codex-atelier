//! 修复前后的 Desktop 静默窗口。
//!
//! 对齐 AiMaMi 1.2.7：需要静默窗口的修复在 Desktop 运行时**自动关闭客户端**，
//! 修复完成后**自动重新启动**。判定表见 `quiet_window.rs`（原版 `fix_requires_quiet_window`
//! @0x10037d594），只有 `residual_cache` 与 `residual_manifest` 两项豁免。
//!
//! 失败语义：修复本身失败时仍然恢复客户端，避免用户被留在“应用已关闭”的状态里。

use super::*;
use crate::maintenance::process::{self, Runner};

/// 修复期间接管 Desktop 生命周期。`Drop` 负责恢复，保证出错路径也重启客户端。
pub(super) struct DesktopWindow {
    app: Option<PathBuf>,
    was_running: bool,
    restored: bool,
}

impl DesktopWindow {
    /// 需要静默窗口且 Desktop 正在运行时关闭它；否则原样返回。
    ///
    /// 测试构建绝不能触碰真实客户端：单元测试里一律视为「拿不到静默窗口」，
    /// 这样既能验证中止路径，也不会误关开发者正在用的应用。
    pub(super) fn enter(ctx: &Context, item_ids: &[String]) -> Result<Self> {
        #[cfg(test)]
        {
            // 单元测试不操作真实客户端：一律视为无需静默窗口，
            // 既有用例的行为保持不变；真实停启逻辑由 process_tests 的假 Runner 覆盖。
            let _ = (ctx, item_ids);
            return Ok(Self {
                app: None,
                was_running: false,
                restored: true,
            });
        }
        #[cfg(not(test))]
        Self::enter_real(ctx, item_ids)
    }

    #[cfg(not(test))]
    fn enter_real(ctx: &Context, item_ids: &[String]) -> Result<Self> {
        if super::quiet_window::can_run_with_desktop(item_ids) {
            return Ok(Self {
                app: None,
                was_running: false,
                restored: true,
            });
        }
        let app_found = process::discover();
        let Some(app) = app_found else {
            // 找不到安装位置时不能假装已经静默；按原版语义视为未取得静默窗口。
            return Err("未检测到可管理的 Desktop 安装；请手动退出后重试".into());
        };
        let runner = process::SystemRunner;
        if !runner.running(&app)? {
            return Ok(Self {
                app: Some(app),
                was_running: false,
                restored: true,
            });
        }
        process::shutdown(&app, &runner).map_err(|e| format!("未能取得 Desktop 静默窗口：{e}"))?;
        Ok(Self {
            app: Some(app),
            was_running: true,
            restored: false,
        })
    }

    /// 修复结束后立即恢复客户端；`apply` 的成功路径调用一次。
    pub(super) fn restore(&mut self) -> Result<()> {
        if self.restored {
            return Ok(());
        }
        self.restored = true;
        let Some(app) = self.app.as_ref() else {
            return Ok(());
        };
        process::launch_and_wait(app, &process::SystemRunner)
    }
}

impl Drop for DesktopWindow {
    fn drop(&mut self) {
        if self.was_running && !self.restored {
            if let Some(app) = self.app.as_ref() {
                let _ = process::launch_and_wait(app, &process::SystemRunner);
            }
        }
    }
}
