use super::types::*;
use fs2::FileExt;
use sha2::{Digest, Sha256};
use std::{
    fs::{self, File, OpenOptions},
    io::Write,
    path::{Path, PathBuf},
    sync::Mutex,
};
use uuid::Uuid;

pub struct Store {
    root: PathBuf,
    gate: Mutex<()>,
}
impl Store {
    pub fn new(root: PathBuf) -> Result<Self, String> {
        private_dir(&root)?;
        private_dir(&root.join("credentials"))?;
        private_dir(&root.join("locks"))?;
        let store = Self {
            root,
            gate: Mutex::new(()),
        };
        {
            let _guard = store.gate.lock().map_err(|_| "账号存储锁异常。")?;
            let _file = store.file_lock()?;
            if !store.path().exists() {
                store.write(&Catalog {
                    revision: String::new(),
                    host_id: format!("urn:uuid:{}", Uuid::new_v4()),
                    profiles: vec![],
                    settings: Settings::default(),
                })?;
            }
        }
        Ok(store)
    }
    fn path(&self) -> PathBuf {
        self.root.join("catalog.json")
    }
    fn file_lock(&self) -> Result<File, String> {
        let f = open_private(&self.root.join("catalog.lock"))?;
        f.try_lock_exclusive()
            .map_err(|_| "另一 Atelier 正在修改账号资料，请稍后重试。")?;
        Ok(f)
    }
    pub fn credential_lock(&self, id: &str) -> Result<File, String> {
        valid_id(id)?;
        let f = open_private(&self.root.join("locks").join(format!("{id}.lock")))?;
        f.try_lock_exclusive()
            .map_err(|_| "该账号正在刷新或更新凭据，请稍后重试。")?;
        Ok(f)
    }
    fn read_inner(&self) -> Result<Catalog, String> {
        let bytes = read_private(&self.path(), 4 * 1024 * 1024)?;
        let mut c: Catalog =
            serde_json::from_slice(&bytes).map_err(|_| "账号目录格式错误；未覆盖原文件。")?;
        if c.profiles.len() > 64 || c.host_id.is_empty() {
            return Err("账号目录超出支持范围。".into());
        }
        c.revision = format!("{:x}", Sha256::digest(&bytes));
        Ok(c)
    }
    pub fn read(&self) -> Result<Catalog, String> {
        let _guard = self.gate.lock().map_err(|_| "账号存储锁异常。")?;
        let _file = self.file_lock()?;
        self.read_inner()
    }
    fn write(&self, c: &Catalog) -> Result<Catalog, String> {
        let mut c = c.clone();
        c.revision = Uuid::new_v4().to_string();
        let bytes = serde_json::to_vec_pretty(&c).map_err(|_| "账号目录编码失败。")?;
        atomic_private(&self.path(), &bytes)?;
        c.revision = format!("{:x}", Sha256::digest(&bytes));
        Ok(c)
    }
    pub fn update<F>(&self, expected: Option<&str>, change: F) -> Result<Catalog, String>
    where
        F: FnOnce(&mut Catalog) -> Result<(), String>,
    {
        let _guard = self.gate.lock().map_err(|_| "账号存储锁异常。")?;
        let _file = self.file_lock()?;
        let mut c = self.read_inner()?;
        if expected.is_some_and(|v| v != c.revision) {
            return Err("账号配置已变化，请刷新后重新操作。".into());
        }
        change(&mut c)?;
        self.write(&c)
    }
    pub fn credential(&self, id: &str) -> Result<Credential, String> {
        valid_id(id)?;
        serde_json::from_slice(&read_private(&self.secret_path(id), 128 * 1024)?)
            .map_err(|_| "账号凭据不可用，请重新授权或配置。".into())
    }
    fn secret_path(&self, id: &str) -> PathBuf {
        self.root.join("credentials").join(format!("{id}.json"))
    }
    pub fn save_credential(&self, id: &str, c: &Credential) -> Result<(), String> {
        valid_id(id)?;
        atomic_private(
            &self.secret_path(id),
            &serde_json::to_vec(c).map_err(|_| "凭据编码失败。")?,
        )
    }
    pub fn upsert(
        &self,
        p: Profile,
        secret: Credential,
        expected: Option<&str>,
    ) -> Result<(), String> {
        valid_id(&p.id)?;
        self.update(expected, |c| {
            if !c.profiles.iter().any(|v| v.id == p.id) && c.profiles.len() >= 64 {
                return Err("最多保存 64 个账号档案。".into());
            }
            self.save_credential(&p.id, &secret)?;
            if let Some(old) = c.profiles.iter_mut().find(|v| v.id == p.id) {
                *old = p;
            } else {
                c.profiles.push(p);
            }
            Ok(())
        })?;
        Ok(())
    }
    pub fn remove(&self, id: &str, revision: &str) -> Result<(), String> {
        valid_id(id)?;
        let _credential = self.credential_lock(id)?;
        self.update(Some(revision), |c| {
            if !c.profiles.iter().any(|p| p.id == id) {
                return Err("账号不存在。".into());
            }
            // Remove secret first: on partial failure the profile remains visibly unusable.
            match fs::remove_file(self.secret_path(id)) {
                Ok(()) => {}
                Err(e) if e.kind() == std::io::ErrorKind::NotFound => {}
                Err(_) => return Err("无法移除本地凭据。".into()),
            }
            c.profiles.retain(|p| p.id != id);
            if c.settings.preferred_profile_id.as_deref() == Some(id) {
                c.settings.preferred_profile_id = None;
            }
            Ok(())
        })?;
        Ok(())
    }
}
fn valid_id(id: &str) -> Result<(), String> {
    Uuid::parse_str(id)
        .map(|_| ())
        .map_err(|_| "账号标识无效。".into())
}
pub(crate) fn private_dir(path: &Path) -> Result<(), String> {
    fs::create_dir_all(path).map_err(|_| "无法创建 Atelier 数据目录。")?;
    let m = fs::symlink_metadata(path).map_err(|_| "无法检查数据目录。")?;
    if !m.is_dir() {
        return Err("数据目录类型不安全。".into());
    }
    #[cfg(unix)]
    {
        use std::os::unix::fs::{MetadataExt, PermissionsExt};
        if m.uid() != unsafe { libc::getuid() } {
            return Err("数据目录不属于当前用户。".into());
        }
        fs::set_permissions(path, fs::Permissions::from_mode(0o700))
            .map_err(|_| "无法保护数据目录。")?;
    }
    #[cfg(not(unix))]
    {
        return Err("凭据存储当前仅支持 Unix 用户权限；此平台尚未启用。".into());
    }
    Ok(())
}
pub(crate) fn open_private(path: &Path) -> Result<File, String> {
    let mut o = OpenOptions::new();
    o.read(true).write(true).create(true).truncate(false);
    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt;
        o.mode(0o600).custom_flags(libc::O_NOFOLLOW);
    }
    let f = o.open(path).map_err(|_| "无法打开受保护的存储文件。")?;
    check_file(&f, 4 * 1024 * 1024)?;
    Ok(f)
}
fn check_file(file: &File, max: u64) -> Result<(), String> {
    let m = file.metadata().map_err(|_| "无法检查存储文件。")?;
    if !m.is_file() || m.len() > max {
        return Err("存储文件类型或大小不受支持。".into());
    }
    #[cfg(unix)]
    {
        use std::os::unix::fs::MetadataExt;
        if m.uid() != unsafe { libc::getuid() } || m.mode() & 0o077 != 0 {
            return Err("凭据文件权限不安全，已拒绝读取。".into());
        }
    }
    Ok(())
}
pub(crate) fn read_private(path: &Path, max: u64) -> Result<Vec<u8>, String> {
    use std::io::Read;
    let mut opts = OpenOptions::new();
    opts.read(true);
    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt;
        opts.custom_flags(libc::O_NOFOLLOW);
    }
    let file = opts
        .open(path)
        .map_err(|_| "本地账号资料不存在或不可读。")?;
    check_file(&file, max)?;
    let mut b = Vec::new();
    file.take(max + 1)
        .read_to_end(&mut b)
        .map_err(|_| "读取账号资料失败。")?;
    if b.len() as u64 > max {
        return Err("账号资料超过大小上限。".into());
    }
    Ok(b)
}
pub(crate) fn atomic_private(path: &Path, bytes: &[u8]) -> Result<(), String> {
    let tmp = path.with_extension(format!("{}.tmp", Uuid::new_v4()));
    let mut o = OpenOptions::new();
    o.create_new(true).write(true);
    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt;
        o.mode(0o600).custom_flags(libc::O_NOFOLLOW);
    }
    let result = (|| {
        let mut f = o.open(&tmp).map_err(|_| "无法准备账号写入。")?;
        f.write_all(bytes)
            .and_then(|_| f.sync_all())
            .map_err(|_| "账号资料写入失败。")?;
        fs::rename(&tmp, path).map_err(|_| "账号资料原子替换失败。")?;
        Ok(())
    })();
    if result.is_err() {
        let _ = fs::remove_file(&tmp);
    }
    result
}
