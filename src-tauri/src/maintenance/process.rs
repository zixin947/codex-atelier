#[path = "native_process.rs"]
mod native;
pub(super) use native::{apply_native, prepare_native, PreparedAction, ProcessPreview};
#[cfg(test)]
#[path = "native_process_tests.rs"]
mod native_tests;
use super::*;
use std::{
    path::Path,
    process::{Command, Stdio},
    thread,
    time::{Duration, Instant},
};
pub(super) fn discover() -> Option<PathBuf> {
    #[cfg(target_os = "macos")]
    {
        crate::desktop_runtime::applications()
            .into_iter()
            .find(|p| executable(p).is_file())
    }
    #[cfg(not(target_os = "macos"))]
    {
        None
    }
}
fn executable(path: &Path) -> PathBuf {
    path.join("Contents/MacOS")
        .join(path.file_stem().unwrap_or_default())
}
pub(super) trait Runner {
    fn running(&self, path: &Path) -> Result<bool>;
    fn quit(&self, path: &Path) -> Result<()>;
    fn launch(&self, path: &Path) -> Result<()>;
    fn pause(&self);
}
pub(super) struct SystemRunner;
fn bounded(mut cmd: Command) -> Result<()> {
    let mut child = cmd
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .map_err(|_| "无法执行 Desktop 管理命令")?;
    let deadline = Instant::now() + Duration::from_secs(10);
    loop {
        if let Some(status) = child.try_wait().map_err(|_| "无法检查管理命令状态")? {
            return if status.success() {
                Ok(())
            } else {
                Err("Desktop 管理命令失败，请检查系统权限".into())
            };
        }
        if Instant::now() > deadline {
            let _ = child.kill();
            let _ = child.wait();
            return Err("Desktop 管理命令超时；未强制结束客户端".into());
        }
        thread::sleep(Duration::from_millis(50));
    }
}
impl Runner for SystemRunner {
    fn running(&self, path: &Path) -> Result<bool> {
        #[cfg(target_os = "macos")]
        {
            let output = Command::new("/bin/ps")
                .args(["-axo", "comm="])
                .output()
                .map_err(|_| "无法检查 Desktop 进程")?;
            if !output.status.success() {
                return Err("无法检查 Desktop 进程".into());
            }
            let target = executable(path);
            Ok(String::from_utf8_lossy(&output.stdout)
                .lines()
                .any(|l| l.trim() == target.to_string_lossy()))
        }
        #[cfg(not(target_os = "macos"))]
        {
            let _ = path;
            Err("此平台尚不支持 Desktop 重启".into())
        }
    }
    fn quit(&self, path: &Path) -> Result<()> {
        let mut cmd = Command::new("/usr/bin/osascript");
        cmd.args([
            "-e",
            "on run argv\ntell application (item 1 of argv) to quit\nend run",
        ])
        .arg(path);
        bounded(cmd)
    }
    fn launch(&self, path: &Path) -> Result<()> {
        let mut cmd = Command::new("/usr/bin/open");
        cmd.arg(path);
        bounded(cmd)
    }
    fn pause(&self) {
        thread::sleep(Duration::from_millis(200))
    }
}
#[cfg(test)]
pub(super) fn restart(path: &Path, runner: &dyn Runner) -> Result<()> {
    if runner.running(path)? {
        runner.quit(path)?;
        let mut stopped = false;
        for _ in 0..40 {
            if !runner.running(path)? {
                stopped = true;
                break;
            }
            runner.pause()
        }
        if !stopped {
            return Err("Desktop 尚未退出；请保存工作后手动退出。未强杀或重复启动".into());
        }
    }
    runner.launch(path)?;
    for _ in 0..40 {
        if runner.running(path)? {
            return Ok(());
        }
        runner.pause()
    }
    Err("启动命令已发送，但尚未检测到目标进程".into())
}

/// Fixed-target process control; neither a command string nor an arbitrary PID crosses IPC.
#[cfg(test)]
#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) struct ProcessEntry {
    pub pid: u32,
    pub executable: PathBuf,
}
#[cfg(test)]
pub(super) trait ForceRunner {
    fn processes(&self) -> Result<Vec<ProcessEntry>>;
    fn terminate(&self, pid: u32) -> Result<()>;
    fn pause(&self);
}
#[cfg(test)]
fn parse_processes(text: &str) -> Vec<ProcessEntry> {
    text.lines()
        .filter_map(|line| {
            let (pid, executable) = line.trim().split_once(char::is_whitespace)?;
            let pid = pid.parse::<u32>().ok()?;
            let executable = PathBuf::from(executable.trim());
            (pid > 1 && executable.is_absolute()).then_some(ProcessEntry { pid, executable })
        })
        .collect()
}
#[cfg(test)]
fn belongs_to_app(entry: &ProcessEntry, app: &Path) -> bool {
    // The bundle boundary includes its WebKit/helper/server executables, not similarly named apps.
    entry.executable.starts_with(app.join("Contents")) && entry.pid != std::process::id()
}
#[cfg(test)]
pub(super) fn force_kill(path: &Path, runner: &dyn ForceRunner) -> Result<usize> {
    if !path.is_absolute()
        || !matches!(
            path.file_name().and_then(|s| s.to_str()),
            Some("ChatGPT.app" | "Codex.app")
        )
    {
        return Err("强制结束目标不是已支持的 Desktop 应用".into());
    }
    let targets: Vec<_> = runner
        .processes()?
        .into_iter()
        .filter(|p| belongs_to_app(p, path))
        .collect();
    let mut changed = 0;
    for target in targets {
        // Recheck the executable before each signal; a PID can have been recycled since the preview.
        if runner
            .processes()?
            .iter()
            .any(|p| p == &target && belongs_to_app(p, path))
        {
            runner.terminate(target.pid)?;
            changed += 1;
        }
    }
    for _ in 0..50 {
        if !runner.processes()?.iter().any(|p| belongs_to_app(p, path)) {
            return Ok(changed);
        }
        runner.pause();
    }
    Err("仍检测到 Desktop 进程；可能被系统重新启动，请核对客户端状态".into())
}

#[cfg(test)]
mod force_tests {
    use super::*;
    use std::cell::RefCell;
    struct Fake {
        entries: RefCell<Vec<ProcessEntry>>,
        killed: RefCell<Vec<u32>>,
        recycle: bool,
    }
    impl ForceRunner for Fake {
        fn processes(&self) -> Result<Vec<ProcessEntry>> {
            let result = self.entries.borrow().clone();
            if self.recycle {
                self.entries
                    .borrow_mut()
                    .iter_mut()
                    .for_each(|p| p.executable = PathBuf::from("/unrelated/tool"));
            }
            Ok(result)
        }
        fn terminate(&self, pid: u32) -> Result<()> {
            self.killed.borrow_mut().push(pid);
            self.entries.borrow_mut().retain(|p| p.pid != pid);
            Ok(())
        }
        fn pause(&self) {}
    }
    #[test]
    fn force_kill_matches_bundle_boundary_and_does_not_launch() {
        let f=Fake{entries:RefCell::new(parse_processes("21 /Applications/Codex.app/Contents/MacOS/Codex\n22 /Applications/Codex.app/Contents/Frameworks/Helper.app/Contents/MacOS/Helper\n23 /Applications/Codex.app-copy/Contents/MacOS/Codex\n24 /Applications/Other.app/Contents/MacOS/Codex\n")),killed:RefCell::new(vec![]),recycle:false};
        assert_eq!(
            force_kill(Path::new("/Applications/Codex.app"), &f).unwrap(),
            2
        );
        assert_eq!(*f.killed.borrow(), vec![21, 22]);
        assert_eq!(f.entries.borrow().len(), 2);
    }
    #[test]
    fn recycled_pid_and_unrecognized_app_are_never_signalled() {
        let f = Fake {
            entries: RefCell::new(parse_processes(
                "25 /Applications/ChatGPT.app/Contents/MacOS/ChatGPT",
            )),
            killed: RefCell::new(vec![]),
            recycle: true,
        };
        assert_eq!(
            force_kill(Path::new("/Applications/ChatGPT.app"), &f).unwrap(),
            0
        );
        assert!(f.killed.borrow().is_empty());
        assert!(force_kill(Path::new("/Applications/AiMaMi.app"), &f).is_err());
    }
}
