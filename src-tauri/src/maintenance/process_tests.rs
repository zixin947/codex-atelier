use super::*;
use std::cell::RefCell;

/// 假 Runner：记录 quit / launch 调用顺序，并模拟进程消失与出现。
struct Fake {
    running: RefCell<bool>,
    calls: RefCell<Vec<&'static str>>,
    quit_fails: bool,
    never_exits: bool,
    never_starts: bool,
}
impl Fake {
    fn new(running: bool) -> Self {
        Self {
            running: RefCell::new(running),
            calls: RefCell::new(vec![]),
            quit_fails: false,
            never_exits: false,
            never_starts: false,
        }
    }
}
impl Runner for Fake {
    fn running(&self, _path: &Path) -> Result<bool> {
        Ok(*self.running.borrow())
    }
    fn quit(&self, _path: &Path) -> Result<()> {
        self.calls.borrow_mut().push("quit");
        if self.quit_fails {
            return Err("退出失败".into());
        }
        if !self.never_exits {
            *self.running.borrow_mut() = false;
        }
        Ok(())
    }
    fn launch(&self, _path: &Path) -> Result<()> {
        self.calls.borrow_mut().push("launch");
        if !self.never_starts {
            *self.running.borrow_mut() = true;
        }
        Ok(())
    }
    fn pause(&self) {}
}

fn app() -> PathBuf {
    PathBuf::from("/Applications/ChatGPT.app")
}

#[test]
fn shutdown_quits_and_waits_for_exit() {
    let f = Fake::new(true);
    shutdown(&app(), &f).unwrap();
    assert_eq!(*f.calls.borrow(), vec!["quit"]);
    assert!(!*f.running.borrow());
}

#[test]
fn shutdown_is_noop_when_already_stopped() {
    let f = Fake::new(false);
    shutdown(&app(), &f).unwrap();
    assert!(f.calls.borrow().is_empty());
}

#[test]
fn shutdown_never_kills_when_the_app_does_not_exit() {
    let mut f = Fake::new(true);
    f.never_exits = true;
    let err = shutdown(&app(), &f).unwrap_err();
    assert!(err.contains("尚未退出"));
    // 只尝试优雅退出一次，不做强杀。
    assert_eq!(*f.calls.borrow(), vec!["quit"]);
}

#[test]
fn launch_and_wait_reports_when_the_process_never_appears() {
    let mut f = Fake::new(false);
    f.never_starts = true;
    let err = launch_and_wait(&app(), &f).unwrap_err();
    assert!(err.contains("尚未检测到"));
    assert_eq!(*f.calls.borrow(), vec!["launch"]);
}

#[test]
fn restart_quits_then_launches() {
    let f = Fake::new(true);
    restart(&app(), &f).unwrap();
    assert_eq!(*f.calls.borrow(), vec!["quit", "launch"]);
    assert!(*f.running.borrow());
}

#[test]
fn restart_starts_a_stopped_app_without_quitting() {
    let f = Fake::new(false);
    restart(&app(), &f).unwrap();
    assert_eq!(*f.calls.borrow(), vec!["launch"]);
}
