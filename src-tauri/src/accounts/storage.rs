use super::types::*;
use crate::gateway::{
    storage::{atomic_private, open_private, private_dir, read_private},
    types::now_ms,
};
use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};
use fs2::FileExt;
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    fs::{self, File},
    path::{Path, PathBuf},
    sync::Mutex,
};
use uuid::Uuid;
pub struct Store {
    pub root: PathBuf,
    gate: Mutex<()>,
}
pub fn hash(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}
pub fn id_ok(id: &str) -> Result<(), String> {
    Uuid::parse_str(id)
        .map(|_| ())
        .map_err(|_| "账号标识无效。".into())
}
impl Store {
    pub fn new(root: PathBuf) -> Result<Self, String> {
        private_dir(&root)?;
        private_dir(&root.join("profiles"))?;
        private_dir(&root.join("locks"))?;
        let store = Self {
            root,
            gate: Mutex::new(()),
        };
        let _guard = store.gate.lock().map_err(|_| "账号目录锁异常。")?;
        let _file = store.lock("catalog")?;
        if !store.path().exists() {
            atomic_private(
                &store.path(),
                &serde_json::to_vec(&Catalog::default()).unwrap(),
            )?;
        }
        drop(_guard);
        Ok(store)
    }
    fn path(&self) -> PathBuf {
        self.root.join("catalog.json")
    }
    pub fn lock(&self, id: &str) -> Result<File, String> {
        if id != "catalog" && id != "target" {
            id_ok(id)?;
        }
        let file = open_private(&self.root.join("locks").join(format!("{id}.lock")))?;
        file.try_lock_exclusive()
            .map_err(|_| "账号正在被另一操作使用，请稍后重试。")?;
        Ok(file)
    }
    fn read_inner(&self) -> Result<Catalog, String> {
        let bytes = read_private(&self.path(), 4 * 1024 * 1024)?;
        let mut c: Catalog =
            serde_json::from_slice(&bytes).map_err(|_| "账号目录损坏，未覆盖。")?;
        if c.accounts.len() > 64 {
            return Err("账号数量超过支持上限。".into());
        }
        c.revision = hash(&bytes);
        Ok(c)
    }
    pub fn read(&self) -> Result<Catalog, String> {
        let _g = self.gate.lock().map_err(|_| "账号目录锁异常。")?;
        let _f = self.lock("catalog")?;
        self.read_inner()
    }
    pub fn update<F>(&self, expected: Option<&str>, f: F) -> Result<Catalog, String>
    where
        F: FnOnce(&mut Catalog) -> Result<(), String>,
    {
        let _g = self.gate.lock().map_err(|_| "账号目录锁异常。")?;
        let _f = self.lock("catalog")?;
        let mut c = self.read_inner()?;
        if expected.is_some_and(|v| v != c.revision) {
            return Err("账号列表已变化，请刷新后重试。".into());
        }
        f(&mut c)?;
        c.revision = Uuid::new_v4().to_string();
        let bytes = serde_json::to_vec_pretty(&c).map_err(|_| "账号编码失败。")?;
        atomic_private(&self.path(), &bytes)?;
        c.revision = hash(&bytes);
        Ok(c)
    }
    pub fn home(&self, id: &str) -> Result<PathBuf, String> {
        id_ok(id)?;
        Ok(self.root.join("profiles").join(id))
    }
    pub fn prepare(&self, id: &str) -> Result<PathBuf, String> {
        let home = self.home(id)?;
        private_dir(&home)?;
        // All managed helper processes use only this owned profile and file credentials.
        atomic_private(&home.join("config.toml"),b"cli_auth_credentials_store = \"file\"\nmodel_provider = \"openai\"\n[analytics]\nenabled = false\n")?;
        Ok(home)
    }
    pub fn auth(&self, id: &str) -> Result<Vec<u8>, String> {
        read_private(&self.home(id)?.join("auth.json"), 256 * 1024)
    }
    pub fn add(&self, id: &str, name: &str, raw: &Value) -> Result<Account, String> {
        let auth = normalize(raw)?;
        let (identity, email, plan) = hints(&auth);
        let account = Account {
            id: id.into(),
            name: name.chars().take(80).collect(),
            email,
            plan,
            group: raw["group"]
                .as_str()
                .unwrap_or("")
                .chars()
                .take(40)
                .collect(),
            note: raw["note"]
                .as_str()
                .unwrap_or("")
                .chars()
                .take(500)
                .collect(),
            status: "imported".into(),
            quota: None,
            error: None,
            created_at: now_ms(),
            identity,
            metadata: super::metadata::from_auth(&auth),
        };
        self.update(None, |c| {
            if c.accounts.len() >= 64 {
                return Err("最多保存 64 个账号。".into());
            }
            if c.accounts
                .iter()
                .any(|a| !a.identity.is_empty() && a.identity == account.identity)
            {
                return Err("此账号已存在，未覆盖原凭据。".into());
            }
            let home = self.prepare(id)?;
            atomic_private(
                &home.join("auth.json"),
                &serde_json::to_vec(&auth).map_err(|_| "凭据编码失败。")?,
            )?;
            c.accounts.push(account.clone());
            Ok(())
        })?;
        Ok(account)
    }
    pub fn reauthorize(&self, id: &str, raw: &Value, revision: &str) -> Result<(), String> {
        let _lock = self.lock(id)?;
        let auth = normalize(raw)?;
        let (identity, email, _) = hints(&auth);
        self.update(Some(revision), |c| {
            let p = c
                .accounts
                .iter_mut()
                .find(|p| p.id == id)
                .ok_or("原账号已移除。")?;
            if p.identity != identity {
                return Err("登录身份与原账号不一致，原凭据未替换。".into());
            }
            atomic_private(
                &self.home(id)?.join("auth.json"),
                &serde_json::to_vec(&auth).map_err(|_| "凭据编码失败。")?,
            )?;
            p.email = email;
            p.metadata = super::metadata::merge(&p.metadata, &super::metadata::from_auth(&auth));
            p.status = "imported".into();
            p.error = None;
            Ok(())
        })?;
        Ok(())
    }
    pub fn remove(&self, id: &str, revision: &str) -> Result<(), String> {
        let _lock = self.lock(id)?;
        self.update(Some(revision), |c| {
            if super::local::current_view(c).account_id.as_deref() == Some(id) {
                return Err("不能删除当前文件账号；请先切换到其他账号并重新核对。".into());
            }
            if !c.accounts.iter().any(|a| a.id == id) {
                return Err("账号不存在。".into());
            }
            let home = self.home(id)?;
            if home.exists() {
                let metadata = fs::symlink_metadata(&home).map_err(|_| "无法检查账号目录。")?;
                if !metadata.is_dir() {
                    return Err("账号目录不安全。".into());
                }
                fs::remove_dir_all(home).map_err(|_| "无法移除本地账号资料。")?;
            }
            c.accounts.retain(|a| a.id != id);
            if c.last_selected_id.as_deref() == Some(id) {
                c.last_selected_id = None;
            }
            Ok(())
        })?;
        Ok(())
    }
}
pub fn normalize(raw: &Value) -> Result<Value, String> {
    let raw = raw.get("auth").unwrap_or(raw);
    let tokens = raw.get("tokens").unwrap_or(raw);
    let mut out = serde_json::Map::new();
    for key in ["access_token", "refresh_token", "id_token"] {
        let value=tokens[key].as_str().filter(|s|!s.is_empty()&&s.len()<65536).ok_or("需要包含 access_token、refresh_token、id_token 的 Codex auth.json；不接受单独刷新令牌。")?;
        out.insert(key.into(), json!(value));
    }
    let account = tokens["account_id"]
        .as_str()
        .map(str::to_owned)
        .or_else(|| {
            let claims = decode_hint(tokens["id_token"].as_str()?);
            claims["https://api.openai.com/auth"]["chatgpt_account_id"]
                .as_str()
                .map(str::to_owned)
        })
        .filter(|s| !s.is_empty() && s.len() < 256)
        .ok_or("凭据缺少 Codex account_id。")?;
    out.insert("account_id".into(), json!(account));
    let mut auth = json!({"auth_mode":"chatgpt","OPENAI_API_KEY":null,"tokens":out});
    if let Some(refresh) = raw["last_refresh"].as_str() {
        if refresh.len() < 100 {
            auth["last_refresh"] = json!(refresh);
        }
    }
    Ok(auth)
}
fn decode_hint(token: &str) -> Value {
    token
        .split('.')
        .nth(1)
        .and_then(|p| URL_SAFE_NO_PAD.decode(p).ok())
        .and_then(|b| serde_json::from_slice(&b).ok())
        .unwrap_or(Value::Null)
}
pub fn hints(auth: &Value) -> (String, Option<String>, Option<String>) {
    let claims = decode_hint(auth["tokens"]["id_token"].as_str().unwrap_or(""));
    let account = auth["tokens"]["account_id"].as_str().unwrap_or("");
    let subject = claims["sub"].as_str().unwrap_or("");
    let identity = hash(format!("{account}:{subject}").as_bytes());
    let email = claims["email"]
        .as_str()
        .map(|s| s.chars().take(200).collect());
    // Unverified import metadata is only a hint. Plan is populated after an official RPC read.
    (identity, email, None)
}
pub fn read_external(path: &Path, max: u64) -> Result<Option<Vec<u8>>, String> {
    use std::io::Read;
    let mut opts = fs::OpenOptions::new();
    opts.read(true);
    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt;
        opts.custom_flags(libc::O_NOFOLLOW);
    }
    let file = match opts.open(path) {
        Ok(f) => f,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(None),
        Err(_) => return Err("无法安全读取目标文件。".into()),
    };
    let m = file.metadata().map_err(|_| "无法检查目标文件。")?;
    if !m.is_file() || m.len() > max {
        return Err("目标文件类型或大小不受支持。".into());
    }
    #[cfg(unix)]
    {
        use std::os::unix::fs::MetadataExt;
        if m.uid() != unsafe { libc::getuid() } {
            return Err("目标文件不属于当前用户。".into());
        }
    }
    let mut b = Vec::new();
    file.take(max + 1)
        .read_to_end(&mut b)
        .map_err(|_| "读取目标文件失败。")?;
    if b.len() as u64 > max {
        return Err("目标文件过大。".into());
    }
    Ok(Some(b))
}
