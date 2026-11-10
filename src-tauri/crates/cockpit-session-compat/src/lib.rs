//! Host adaptations are separate from the hash-verified upstream sources.
mod modules;
pub use modules::codex_session_visibility::*;
use std::path::{Path, PathBuf};
#[derive(Clone)]
struct Host {
    home: PathBuf,
    backups: PathBuf,
    binary: Option<PathBuf>,
}
thread_local! {static HOST:std::cell::RefCell<Option<Host>>=const{std::cell::RefCell::new(None)};}
fn host() -> Result<Host, String> {
    HOST.with(|s| s.borrow().clone())
        .ok_or("缺少显式修复目录；禁止回退到真实用户目录".into())
}
pub fn with_host<T>(
    home: &Path,
    backups: &Path,
    binary: Option<&Path>,
    work: impl FnOnce() -> Result<T, String>,
) -> Result<T, String> {
    struct Reset(Option<Host>);
    impl Drop for Reset {
        fn drop(&mut self) {
            HOST.with(|s| s.replace(self.0.take()));
        }
    }
    let old = HOST.with(|s| {
        s.replace(Some(Host {
            home: home.into(),
            backups: backups.into(),
            binary: binary.map(Path::to_owned),
        }))
    });
    let _reset = Reset(old);
    work()
}
