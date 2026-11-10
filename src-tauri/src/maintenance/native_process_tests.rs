use super::native::*;
use atelier_maintenance_core::process::{
    BundleField, CommandOutput, Operation, ProcessContext, Runner,
};
use std::{
    cell::{Cell, RefCell},
    collections::{BTreeMap, BTreeSet},
    fs, io,
    path::{Path, PathBuf},
    time::Duration,
};
struct Fake {
    root: PathBuf,
    entries: RefCell<BTreeMap<u32, String>>,
    holders: RefCell<BTreeSet<u32>>,
    events: RefCell<Vec<String>>,
    time: Cell<Duration>,
    installed: bool,
    refuse_quit: Cell<bool>,
    refuse_signal: Cell<bool>,
    open_failure: Cell<usize>,
    open_count: Cell<usize>,
}
impl Fake {
    fn new(installed: bool) -> Self {
        let root =
            std::env::temp_dir().join(format!("atelier-native-process-{}", uuid::Uuid::new_v4()));
        fs::create_dir_all(root.join("sqlite")).unwrap();
        Self {
            root,
            entries: RefCell::new(BTreeMap::new()),
            holders: RefCell::new(BTreeSet::new()),
            events: RefCell::new(vec![]),
            time: Cell::new(Duration::ZERO),
            installed,
            refuse_quit: Cell::new(false),
            refuse_signal: Cell::new(false),
            open_failure: Cell::new(0),
            open_count: Cell::new(0),
        }
    }
    fn ctx(&self) -> ProcessContext {
        ProcessContext::new(self.root.clone(), Some(self.root.join("synthetic-user")))
    }
    fn add(&self, pid: u32, command: &str) {
        self.entries.borrow_mut().insert(pid, command.into());
    }
    fn out(success: bool, text: String) -> io::Result<CommandOutput> {
        Ok(CommandOutput {
            success,
            stdout: text.into_bytes(),
            stderr: vec![],
        })
    }
}
impl Drop for Fake {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.root);
    }
}
impl Runner for Fake {
    fn output(&self, op: &Operation) -> io::Result<CommandOutput> {
        match op {
            Operation::Processes => Self::out(
                true,
                self.entries
                    .borrow()
                    .iter()
                    .map(|(p, c)| format!("{p} {c}"))
                    .collect::<Vec<_>>()
                    .join("\n"),
            ),
            Operation::CommandForPid(pid) => Self::out(
                true,
                self.entries.borrow().get(pid).cloned().unwrap_or_default(),
            ),
            Operation::FindBundles => Self::out(true, String::new()),
            Operation::BundleValue { bundle, field } => Self::out(
                self.installed && bundle == Path::new("/Applications/ChatGPT.app"),
                match field {
                    BundleField::Identifier => "com.openai.codex",
                    BundleField::Executable => "ChatGPT",
                }
                .into(),
            ),
            Operation::IsAppRunning => Self::out(
                true,
                self.entries
                    .borrow()
                    .values()
                    .any(|c| c.contains("/ChatGPT.app/Contents/"))
                    .to_string(),
            ),
            Operation::QuitApp => {
                self.events.borrow_mut().push("quit".into());
                if !self.refuse_quit.get() {
                    self.entries
                        .borrow_mut()
                        .retain(|_, c| !c.contains("/ChatGPT.app/Contents/"));
                }
                Self::out(true, String::new())
            }
            Operation::OpenBundle(_) => {
                let n = self.open_count.get() + 1;
                self.open_count.set(n);
                self.events.borrow_mut().push(format!("open{n}"));
                if self.open_failure.get() != n {
                    self.add(801, "/Applications/ChatGPT.app/Contents/MacOS/ChatGPT");
                }
                Self::out(self.open_failure.get() != n, String::new())
            }
            Operation::DbHolders(paths) => {
                assert!(paths.iter().all(|p| p.starts_with(&self.root)));
                Self::out(
                    true,
                    self.holders
                        .borrow()
                        .iter()
                        .map(u32::to_string)
                        .collect::<Vec<_>>()
                        .join("\n"),
                )
            }
        }
    }
    fn signal_kill(&self, pid: u32) -> io::Result<()> {
        self.events.borrow_mut().push(format!("kill{pid}"));
        if self.refuse_signal.get() {
            Err(io::Error::new(
                io::ErrorKind::PermissionDenied,
                "synthetic denied",
            ))
        } else {
            self.entries.borrow_mut().remove(&pid);
            self.holders.borrow_mut().remove(&pid);
            Ok(())
        }
    }
    fn exists(&self, p: &Path) -> bool {
        p.starts_with(&self.root) && p.exists()
    }
    fn is_file(&self, p: &Path) -> bool {
        (self.installed && p == Path::new("/Applications/ChatGPT.app/Contents/MacOS/ChatGPT"))
            || (p.starts_with(&self.root) && p.is_file())
    }
    fn read_text(&self, p: &Path) -> io::Result<String> {
        assert!(p.starts_with(&self.root));
        fs::read_to_string(p)
    }
    fn read_dir(&self, p: &Path) -> io::Result<Vec<PathBuf>> {
        assert!(p.starts_with(&self.root));
        Ok(fs::read_dir(p)?
            .filter_map(|e| e.ok().map(|e| e.path()))
            .collect())
    }
    fn process_id(&self) -> u32 {
        999
    }
    fn now(&self) -> Duration {
        self.time.get()
    }
    fn sleep(&self, d: Duration) {
        self.time.set(self.time.get() + d)
    }
}
#[test]
fn force_preview_and_apply_include_cli_without_installed_bundle() {
    let f = Fake::new(false);
    f.add(21, "/opt/homebrew/bin/codex --prompt PRIVATE_BODY");
    f.add(22, "/synthetic/bin/codex-helper");
    f.add(23, "/Applications/Other.app/Contents/MacOS/Other");
    f.add(999, "/opt/homebrew/bin/codex");
    let p = prepare(&f.ctx(), &f, "forceKill").unwrap();
    assert_eq!(
        p.preview()
            .targets
            .iter()
            .map(|p| p.pid)
            .collect::<Vec<_>>(),
        vec![21, 22]
    );
    assert_eq!(p.preview().external_cli.len(), 1);
    assert!(p.preview().bundle.is_none());
    assert_eq!(
        p.preview().source_directories,
        vec![f.root.join("sqlite"), f.root.clone()]
    );
    let serialized = serde_json::to_string(p.preview()).unwrap();
    assert!(!serialized.contains("PRIVATE_BODY"));
    assert!(f.events.borrow().is_empty());
    let out = apply(&p, &f).unwrap();
    assert_eq!(out.changed, 2);
    assert_eq!(*f.events.borrow(), vec!["kill21", "kill22"]);
    assert!(f.entries.borrow().contains_key(&23));
    assert!(f.entries.borrow().contains_key(&999));
}
#[test]
fn protected_extension_is_not_killed_and_actual_holder_blocks_completion() {
    let f = Fake::new(false);
    fs::write(f.root.join("sqlite/state_8.sqlite"), b"synthetic opaque db").unwrap();
    f.add(31, "/opt/homebrew/bin/codex");
    f.add(
        32,
        "/synthetic/.vscode/extensions/openai.chatgpt-1/bin/codex --prompt SECRET",
    );
    f.holders.borrow_mut().insert(32);
    let p = prepare(&f.ctx(), &f, "forceKill").unwrap();
    assert_eq!(p.preview().targets.len(), 1);
    assert_eq!(p.preview().protected_db_holders[0].pid, 32);
    assert_eq!(p.preview().external_cli.len(), 2);
    let e = apply(&p, &f).err().unwrap();
    assert!(e.contains("codex_protected_db_holders"));
    assert!(!e.contains("SECRET"));
    assert_eq!(*f.events.borrow(), vec!["kill31"]);
    assert!(f.entries.borrow().contains_key(&32));
}
#[test]
fn restart_uses_native_five_second_policy_zero_force_fallback_then_double_open() {
    let f = Fake::new(true);
    f.add(41, "/Applications/ChatGPT.app/Contents/MacOS/ChatGPT");
    f.add(42, "/opt/homebrew/bin/codex");
    f.refuse_quit.set(true);
    let p = prepare(&f.ctx(), &f, "restart").unwrap();
    assert_eq!(p.preview().targets.len(), 2);
    assert!(f.events.borrow().is_empty());
    let out = apply(&p, &f).unwrap();
    assert_eq!(out.action, "restart");
    assert!(f.time.get() >= Duration::from_secs(5));
    assert_eq!(
        *f.events.borrow(),
        vec!["quit", "kill41", "kill42", "open1", "open2"]
    );
    assert!(out.summary.contains("801"));
    assert_eq!(out.checks[0].status, "pass");
}
#[test]
fn protected_holder_after_stop_prevents_restart_launch() {
    let f = Fake::new(true);
    fs::write(f.root.join("state_9.sqlite"), b"synthetic").unwrap();
    f.add(51, "/Applications/ChatGPT.app/Contents/MacOS/ChatGPT");
    f.add(52, "/unrelated/sqlite-inspector");
    f.holders.borrow_mut().insert(52);
    let p = prepare(&f.ctx(), &f, "restart").unwrap();
    assert!(apply(&p, &f).err().unwrap().contains("未启动客户端"));
    assert_eq!(*f.events.borrow(), vec!["quit"]);
    assert_eq!(f.open_count.get(), 0);
}
#[test]
fn changed_identity_and_source_configuration_require_fresh_preview() {
    let f = Fake::new(false);
    f.add(61, "/opt/homebrew/bin/codex --version");
    let p = prepare(&f.ctx(), &f, "forceKill").unwrap();
    f.add(61, "/opt/homebrew/bin/codex --other");
    assert!(apply(&p, &f).err().unwrap().contains("PID身份"));
    assert!(f.events.borrow().is_empty());
    f.add(61, "/opt/homebrew/bin/codex --version");
    fs::write(f.root.join("config.toml"), "sqlite_home = 'alternate'\n").unwrap();
    assert!(apply(&p, &f).err().unwrap().contains("来源目录"));
    assert!(f.events.borrow().is_empty());
}
#[test]
fn explicit_sqlite_home_excludes_default_dirs_and_preview_retains_numeric_order() {
    let f = Fake::new(false);
    let alt = f.root.join("alternate");
    fs::create_dir(&alt).unwrap();
    fs::write(f.root.join("config.toml"), "sqlite_home = 'alternate'\n").unwrap();
    for n in [2, 12] {
        fs::write(alt.join(format!("state_{n}.sqlite")), b"opaque").unwrap();
    }
    fs::write(f.root.join("sqlite/state_99.sqlite"), b"opaque").unwrap();
    let mut ctx = f.ctx();
    ctx.cwd = Some(f.root.clone());
    ctx.sqlite_home_env = Some("ignored".into());
    let p = prepare(&ctx, &f, "forceKill").unwrap();
    assert_eq!(p.preview().source_directories, vec![alt.clone()]);
    assert_eq!(
        p.preview().source_databases,
        vec![alt.join("state_12.sqlite"), alt.join("state_2.sqlite")]
    );
    assert_eq!(apply(&p, &f).unwrap().changed, 0);
}
#[test]
fn force_timeout_and_launch_failures_are_actual_failures() {
    let f = Fake::new(false);
    f.add(71, "codex");
    f.refuse_signal.set(true);
    let p = prepare(&f.ctx(), &f, "forceKill").unwrap();
    assert!(apply(&p, &f).err().unwrap().contains("未启动客户端"));
    assert!(f.time.get() >= Duration::from_secs(3));
    assert!(f.events.borrow().len() > 1);
    let launch = Fake::new(true);
    launch.open_failure.set(2);
    let p = prepare(&launch.ctx(), &launch, "restart").unwrap();
    assert!(apply(&p, &launch).err().unwrap().contains("启动阶段未通过"));
    assert_eq!(*launch.events.borrow(), vec!["open1", "open2"]);
}
#[test]
fn restart_needs_a_valid_bundle_and_new_targets_cannot_silently_join_initial_approval() {
    let f = Fake::new(false);
    assert!(prepare(&f.ctx(), &f, "restart").is_err());
    let p = prepare(&f.ctx(), &f, "forceKill").unwrap();
    f.add(81, "codex");
    assert!(apply(&p, &f).err().unwrap().contains("重新预览"));
    assert!(f.events.borrow().is_empty());
}
