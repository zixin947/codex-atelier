use super::*;
use crate::{
    accounts::{storage::Store, types::Catalog},
    gateway::storage::{atomic_private, private_dir},
};
use serde_json::json;
use std::{cell::RefCell, fs, path::Path};
struct Fixture(PathBuf);
impl Fixture {
    fn new() -> Self {
        Self(
            std::env::temp_dir().join(format!("atelier-maintenance-test-{}", uuid::Uuid::new_v4())),
        )
    }
    fn populate(&self) -> (String, Store) {
        let store = Store::new(self.0.clone()).unwrap();
        let id = uuid::Uuid::new_v4().to_string();
        store.add(&id,"保留别名",&json!({"tokens":{"access_token":"synthetic-access","refresh_token":"synthetic-refresh","id_token":"synthetic-id","account_id":"synthetic-account"}})).unwrap();
        (id, store)
    }
    fn catalog(&self) -> Catalog {
        serde_json::from_slice(&fs::read(self.0.join("catalog.json")).unwrap()).unwrap()
    }
}
impl Drop for Fixture {
    fn drop(&mut self) {
        if self.0.exists() {
            fs::remove_dir_all(&self.0).unwrap()
        }
    }
}
#[test]
fn snapshot_is_read_only_for_missing_and_existing_store() {
    let f = Fixture::new();
    let s = store::snapshot(&f.0).unwrap();
    assert_eq!(s.account_count, 0);
    assert!(!f.0.exists());
    let (_id, _store) = f.populate();
    let before = fs::read(f.0.join("catalog.json")).unwrap();
    let s = store::snapshot(&f.0).unwrap();
    assert_eq!(s.account_count, 1);
    assert_eq!(before, fs::read(f.0.join("catalog.json")).unwrap());
    assert!(!f.0.join("maintenance").exists());
}
#[test]
fn clean_only_removes_stale_index_and_owned_backups() {
    let f = Fixture::new();
    let (id, _) = f.populate();
    let ghost = uuid::Uuid::new_v4().to_string();
    let mut c = f.catalog();
    let mut a = c.accounts[0].clone();
    a.id = ghost;
    c.accounts.push(a);
    atomic_private(&f.0.join("catalog.json"), &serde_json::to_vec(&c).unwrap()).unwrap();
    private_dir(&f.0.join("maintenance")).unwrap();
    let backups = f.0.join("maintenance/backups");
    private_dir(&backups).unwrap();
    let old = backups.join(format!("{}.json", uuid::Uuid::new_v4()));
    atomic_private(&old, b"{}").unwrap();
    atomic_private(&f.0.join("last-switch.json"), b"keep-current-rollback").unwrap();
    let auth = fs::read(f.0.join("profiles").join(&id).join("auth.json")).unwrap();
    let t = store::preview(&f.0, "clean").unwrap();
    assert_eq!(t.view.count, 2);
    let r = store::apply(&t).unwrap();
    assert_eq!(r.changed, 2);
    assert!(!old.exists());
    assert!(r.backup_path.is_some());
    assert_eq!(f.catalog().accounts.len(), 1);
    assert_eq!(
        auth,
        fs::read(f.0.join("profiles").join(id).join("auth.json")).unwrap()
    );
    assert_eq!(
        fs::read(f.0.join("last-switch.json")).unwrap(),
        b"keep-current-rollback"
    );
}
#[test]
fn rebuild_recovers_profiles_and_preserves_metadata() {
    let f = Fixture::new();
    let (id, store) = f.populate();
    store
        .update(None, |c| {
            c.accounts[0].group = "测试分组".into();
            c.accounts[0].note = "保留备注".into();
            Ok(())
        })
        .unwrap();
    let second = uuid::Uuid::new_v4().to_string();
    store.add(&second,"恢复账号",&json!({"tokens":{"access_token":"synthetic","refresh_token":"synthetic","id_token":"synthetic","account_id":"second-account"}})).unwrap();
    store
        .update(None, |c| {
            c.accounts.retain(|a| a.id != second);
            Ok(())
        })
        .unwrap();
    let t = store::preview(&f.0, "rebuild").unwrap();
    let before = fs::read(f.0.join("catalog.json")).unwrap();
    let result = store::apply(&t).unwrap();
    assert_eq!(result.changed, 2);
    let c = f.catalog();
    let a = c.accounts.iter().find(|a| a.id == id).unwrap();
    assert_eq!(a.name, "保留别名");
    assert_eq!(a.group, "测试分组");
    assert_eq!(a.note, "保留备注");
    assert_eq!(fs::read(result.backup_path.unwrap()).unwrap(), before);
}
#[test]
fn rejects_stale_preview_and_active_profile_lock() {
    let f = Fixture::new();
    let (id, store) = f.populate();
    let t = store::preview(&f.0, "rebuild").unwrap();
    let lock = store.lock(&id).unwrap();
    assert!(store::apply(&t).err().unwrap().contains("刷新"));
    drop(lock);
    store
        .update(None, |c| {
            c.accounts[0].name = "外部变化".into();
            Ok(())
        })
        .unwrap();
    assert!(store::apply(&t).err().unwrap().contains("变化"));
}
#[test]
fn rebuild_empty_profiles_removes_stale_nonempty_catalog() {
    let f = Fixture::new();
    f.populate();
    fs::remove_dir_all(f.0.join("profiles")).unwrap();
    let t = store::preview(&f.0, "rebuild").unwrap();
    assert!(t.view.count > 0);
    let r = store::apply(&t).unwrap();
    assert!(f.catalog().accounts.is_empty());
    assert!(r.backup_path.is_some());
}
#[test]
fn malformed_catalog_can_be_rebuilt_but_not_cleaned() {
    let f = Fixture::new();
    f.populate();
    atomic_private(&f.0.join("catalog.json"), b"{broken").unwrap();
    assert!(store::preview(&f.0, "clean").is_err());
    let t = store::preview(&f.0, "rebuild").unwrap();
    let result = store::apply(&t).unwrap();
    assert_eq!(f.catalog().accounts.len(), 1);
    assert_eq!(fs::read(result.backup_path.unwrap()).unwrap(), b"{broken");
}
#[test]
fn malformed_auth_preserves_existing_account() {
    let f = Fixture::new();
    let (id, _) = f.populate();
    atomic_private(&f.0.join("profiles").join(id).join("auth.json"), b"invalid").unwrap();
    let t = store::preview(&f.0, "rebuild").unwrap();
    let result = store::apply(&t).unwrap();
    assert_eq!(f.catalog().accounts.len(), 1);
    assert_eq!(result.warnings.len(), 1);
}
#[cfg(unix)]
#[test]
fn symlink_profile_never_reaches_external_files() {
    let f = Fixture::new();
    let (id, _) = f.populate();
    let profile = f.0.join("profiles").join(id);
    fs::remove_dir_all(&profile).unwrap();
    let outside = Fixture::new();
    fs::create_dir(&outside.0).unwrap();
    fs::write(outside.0.join("auth.json"), b"external sentinel").unwrap();
    std::os::unix::fs::symlink(&outside.0, &profile).unwrap();
    assert!(store::preview(&f.0, "clean").is_err());
    assert_eq!(
        fs::read(outside.0.join("auth.json")).unwrap(),
        b"external sentinel"
    );
}
struct FakeRunner {
    running: RefCell<bool>,
    events: RefCell<Vec<&'static str>>,
    refuse_quit: bool,
}
impl process::Runner for FakeRunner {
    fn running(&self, _: &Path) -> Result<bool> {
        Ok(*self.running.borrow())
    }
    fn quit(&self, _: &Path) -> Result<()> {
        self.events.borrow_mut().push("quit");
        if !self.refuse_quit {
            *self.running.borrow_mut() = false
        }
        Ok(())
    }
    fn launch(&self, _: &Path) -> Result<()> {
        self.events.borrow_mut().push("launch");
        *self.running.borrow_mut() = true;
        Ok(())
    }
    fn pause(&self) {}
}
#[test]
fn restart_uses_fake_stop_start_and_no_force_kill() {
    let fake = FakeRunner {
        running: RefCell::new(true),
        events: RefCell::new(vec![]),
        refuse_quit: false,
    };
    process::restart(Path::new("/synthetic/Codex.app"), &fake).unwrap();
    assert_eq!(*fake.events.borrow(), vec!["quit", "launch"]);
    let refusal = FakeRunner {
        running: RefCell::new(true),
        events: RefCell::new(vec![]),
        refuse_quit: true,
    };
    assert!(process::restart(Path::new("/synthetic/Codex.app"), &refusal).is_err());
    assert_eq!(*refusal.events.borrow(), vec!["quit"]);
}
