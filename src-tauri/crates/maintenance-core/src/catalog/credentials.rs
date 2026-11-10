//! Native relay credential wire schema and migration storage. Paths and namespaces are explicit.
use super::state::{RelayStateDocument, StateError};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    fs, io,
    path::{Path, PathBuf},
};

#[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct CredentialRevision(String);
impl CredentialRevision {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}
impl std::fmt::Debug for CredentialRevision {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("CredentialRevision").field(&self.0).finish()
    }
}
impl TryFrom<String> for CredentialRevision {
    type Error = StateError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.len() == 32
            && value
                .bytes()
                .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
        {
            Ok(Self(value))
        } else {
            Err(StateError("invalid relay credential revision".into()))
        }
    }
}
impl From<CredentialRevision> for String {
    fn from(v: CredentialRevision) -> Self {
        v.0
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "lowercase")]
pub enum RelayCredential {
    None,
    #[default]
    Legacy,
    Revision {
        id: CredentialRevision,
    },
}
#[derive(Debug, Clone, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct ProviderCredential {
    pub id: String,
    pub api_key_stored: bool,
    pub credential: RelayCredential,
}
impl Default for ProviderCredential {
    fn default() -> Self {
        Self {
            id: String::new(),
            api_key_stored: false,
            credential: RelayCredential::Legacy,
        }
    }
}
pub fn provider_credentials(
    state: &RelayStateDocument,
) -> Result<Vec<ProviderCredential>, StateError> {
    Ok(state
        .as_value()
        .get("providers")
        .map(|v| serde_json::from_value(v.clone()))
        .transpose()?
        .unwrap_or_default())
}
pub fn validate_credentials(state: &RelayStateDocument) -> Result<(), StateError> {
    let version = state.schema_version()?;
    for provider in provider_credentials(state)? {
        match provider.credential {
            RelayCredential::Legacy => continue,
            RelayCredential::None if version >= 16 && !provider.api_key_stored => continue,
            RelayCredential::Revision { .. } if version >= 16 => {
                crate::catalog::builder::validate_relay_provider_id(&provider.id)
                    .map_err(|e| StateError(e.to_string()))?;
                if provider.api_key_stored {
                    continue;
                }
            }
            _ => {}
        }
        return Err(StateError(
            "relay credential reference conflicts with state schema or presence flag".into(),
        ));
    }
    Ok(())
}

/// Native legacy macOS backend only reads/deletes generic passwords. Current writes use
/// application-owned private files. Service/account arguments always come from explicit scope.
pub trait LegacyKeychain {
    fn get(&mut self, service: &str, account: &str) -> io::Result<Option<Vec<u8>>>;
    fn delete(&mut self, service: &str, account: &str) -> io::Result<()>;
}
/// All fields are supplied by the owning application; no native service, environment,
/// home directory or hostname is discovered implicitly.
#[derive(Clone)]
pub struct CredentialNamespace {
    pub legacy_service: String,
    pub obfuscation_domain: String,
    pub hostname: String,
    pub username: String,
}
pub fn legacy_id_is_safe_identifier(id: &str) -> bool {
    !id.is_empty()
        && !matches!(id, "." | "..")
        && !id
            .chars()
            .any(|c| c < ' ' || ('\u{7f}'..='\u{9f}').contains(&c) || matches!(c, '/' | '\\'))
}
pub struct CredentialStore<K> {
    root: PathBuf,
    namespace: CredentialNamespace,
    pub keychain: K,
}
static KEY_STORE_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());
fn lock_store() -> io::Result<std::sync::MutexGuard<'static, ()>> {
    KEY_STORE_LOCK
        .lock()
        .map_err(|_| io::Error::other("relay key store lock was poisoned"))
}
impl<K: LegacyKeychain> CredentialStore<K> {
    pub fn new(root: PathBuf, namespace: CredentialNamespace, keychain: K) -> Self {
        Self {
            root,
            namespace,
            keychain,
        }
    }
    pub fn root(&self) -> &Path {
        &self.root
    }
    fn read_key_file(&self, id: &str) -> io::Result<Option<String>> {
        match fs::read_to_string(self.root.join("keys").join(id)) {
            Ok(value) => Ok(Some(value.trim().into())),
            Err(e) if e.kind() == io::ErrorKind::NotFound => Ok(None),
            Err(e) => Err(e),
        }
    }
    fn write_key_file(&self, id: &str, key: &str) -> io::Result<()> {
        let directory = self.root.join("keys");
        fs::create_dir_all(&directory)?;
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let _ = fs::set_permissions(&directory, fs::Permissions::from_mode(0o700));
        }
        super::publication::write_atomic_private_report(&directory.join(id), key.as_bytes())?
            .acknowledge();
        Ok(())
    }
    fn legacy_map(&self) -> io::Result<HashMap<String, String>> {
        match fs::read(self.root.join("secrets.json")) {
            Ok(bytes) => serde_json::from_slice(&bytes).map_err(|_| {
                io::Error::new(
                    io::ErrorKind::InvalidData,
                    "invalid legacy relay key storage",
                )
            }),
            Err(e) if e.kind() == io::ErrorKind::NotFound => Ok(HashMap::new()),
            Err(e) => Err(e),
        }
    }
    fn read_legacy_secret(&self, id: &str) -> io::Result<Option<String>> {
        use base64::Engine;
        use sha2::{Digest, Sha256};
        let map = self.legacy_map()?;
        let Some(encoded) = map.get(id) else {
            return Ok(None);
        };
        let bytes = base64::engine::general_purpose::STANDARD
            .decode(encoded)
            .map_err(|_| {
                io::Error::new(
                    io::ErrorKind::InvalidData,
                    "invalid legacy relay key encoding",
                )
            })?;
        let salt = format!(
            "{}::{}::{}",
            self.namespace.obfuscation_domain, self.namespace.hostname, self.namespace.username
        );
        let mask = Sha256::digest(salt.as_bytes());
        let decoded: Vec<u8> = bytes
            .iter()
            .enumerate()
            .map(|(i, b)| b ^ mask[i % 32])
            .collect();
        String::from_utf8(decoded).map(Some).map_err(|_| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                "invalid legacy relay key encoding",
            )
        })
    }
    fn read_keychain(&mut self, id: &str) -> io::Result<Option<String>> {
        self.keychain
            .get(&self.namespace.legacy_service, id)?
            .map(|v| {
                String::from_utf8(v).map_err(|_| {
                    io::Error::new(
                        io::ErrorKind::InvalidData,
                        "legacy relay keychain read failed",
                    )
                })
            })
            .transpose()
    }
    pub fn set_api_key(&mut self, id: &str, key: &str) -> io::Result<()> {
        crate::catalog::builder::validate_relay_provider_id(id)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e.to_string()))?;
        let _guard = lock_store()?;
        self.write_key_file(id, key)
    }
    /// Native delete writes an empty private-file barrier; cleanup is a separate later step.
    pub fn delete_api_key(&mut self, id: &str) -> io::Result<()> {
        crate::catalog::builder::validate_relay_provider_id(id)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e.to_string()))?;
        let _guard = lock_store()?;
        self.write_key_file(id, "")
    }
    pub fn cleanup_deleted_api_key(&mut self, id: &str) -> io::Result<Vec<io::ErrorKind>> {
        crate::catalog::builder::validate_relay_provider_id(id)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e.to_string()))?;
        let _guard = lock_store()?;
        if self.read_key_file(id)?.as_deref() != Some("") {
            return Err(io::Error::other(
                "relay key deletion barrier changed; legacy cleanup skipped",
            ));
        }
        Ok(self.cleanup_legacy_stores(id))
    }
    /// Read-only lookup with the same precedence and tombstone semantics as `get_api_key`.
    /// The caller explicitly supplies the keychain scope; no migration or cleanup is performed.
    pub fn observe_api_key(&mut self, id: &str) -> io::Result<Option<String>> {
        crate::catalog::builder::validate_relay_provider_id(id)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e.to_string()))?;
        let _guard = lock_store()?;
        if let Some(value) = self.read_key_file(id)? {
            return Ok((!value.is_empty()).then_some(value));
        }
        let value = match self.read_legacy_secret(id)? {
            Some(v) if !v.is_empty() => Some(v),
            _ => self.read_keychain(id)?,
        };
        Ok(value.filter(|v| !v.is_empty()))
    }
    pub fn get_api_key(&mut self, id: &str) -> io::Result<Option<String>> {
        crate::catalog::builder::validate_relay_provider_id(id)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e.to_string()))?;
        let _guard = lock_store()?;
        if let Some(value) = self.read_key_file(id)? {
            return Ok((!value.is_empty()).then_some(value));
        }
        let value = match self.read_legacy_secret(id)? {
            Some(v) if !v.is_empty() => Some(v),
            _ => self.read_keychain(id)?,
        };
        if let Some(value) = value.filter(|v| !v.is_empty()) {
            self.write_key_file(id, &value)?;
            Ok(Some(value))
        } else {
            Ok(None)
        }
    }
    /// Returns actual copied/present status from destination/source storage; no caller status input.
    pub fn copy_api_key_from_legacy_id(&mut self, old: &str, new: &str) -> io::Result<bool> {
        crate::catalog::builder::validate_relay_provider_id(new)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e.to_string()))?;
        let _guard = lock_store()?;
        if let Some(value) = self.read_key_file(new)? {
            return Ok(!value.is_empty());
        }
        if !legacy_id_is_safe_identifier(old) {
            return Ok(false);
        }
        let value = match self.read_key_file(old)? {
            Some(v) => Some(v),
            None => match self.read_legacy_secret(old)? {
                Some(v) => Some(v),
                None => self.read_keychain(old)?,
            },
        };
        if let Some(value) = value.filter(|v| !v.is_empty()) {
            self.write_key_file(new, &value)?;
            Ok(true)
        } else {
            Ok(false)
        }
    }
    /// Native cleanup is post-publication and best effort. Tombstone prevents legacy fallback.
    pub fn cleanup_legacy_provider_key(&mut self, id: &str) -> Vec<io::ErrorKind> {
        if !legacy_id_is_safe_identifier(id) {
            return vec![];
        }
        let _guard = match lock_store() {
            Ok(g) => g,
            Err(e) => return vec![e.kind()],
        };
        if let Err(e) = self.write_key_file(id, "") {
            return vec![e.kind()];
        }
        self.cleanup_legacy_stores(id)
    }
    fn cleanup_legacy_stores(&mut self, id: &str) -> Vec<io::ErrorKind> {
        let mut errors = Vec::new();
        match self.legacy_map() {
            Ok(mut map) => {
                if map.remove(id).is_some() {
                    match serde_json::to_vec(&map)
                        .map_err(io::Error::other)
                        .and_then(|bytes| {
                            super::publication::write_atomic_private_report(
                                &self.root.join("secrets.json"),
                                &bytes,
                            )
                        }) {
                        Ok(receipt) => {
                            receipt.acknowledge();
                        }
                        Err(e) => errors.push(e.kind()),
                    }
                }
            }
            Err(e) => errors.push(e.kind()),
        }
        if let Err(e) = self.keychain.delete(&self.namespace.legacy_service, id) {
            errors.push(e.kind());
        }
        errors
    }
}

#[derive(Debug)]
pub struct StateMigrationPublication {
    pub destination: PathBuf,
    pub provider_id_backup: Option<PathBuf>,
    pub provider_id_migrations: Vec<super::state::ProviderIdMigration>,
    pub imported_plaintext_keys: usize,
    pub cleanup_errors: Vec<io::ErrorKind>,
    pub directory_sync_error: Option<io::ErrorKind>,
}
/// Native load's key migration order: preserve original bytes, copy keys, transform state,
/// publish state, then clean only legacy IDs whose actual copy succeeded. Copy lookup/write
/// failures degrade that provider to apiKeyStored=false with native lastError and still rename it.
/// A state write failure retains the caller's document and every old key store.
pub fn migrate_and_publish_state<K: LegacyKeychain>(
    state: &mut RelayStateDocument,
    original_bytes: &[u8],
    destination: &Path,
    store: &mut CredentialStore<K>,
) -> Result<StateMigrationPublication, StateError> {
    let mut initial = state.clone();
    initial.validate_schema_version()?;
    initial.normalize_legacy_active_codex();
    initial.catalog_inputs()?;
    initial.proxy_status()?;
    initial.claude_web_search_compat()?;
    validate_credentials(&initial)?;
    let source_version = if state.as_value().get("schemaVersion").is_none() {
        0
    } else {
        state.schema_version()?
    };
    let plan = state.plan_provider_id_migrations()?;
    let backup = if plan.is_empty() {
        None
    } else {
        Some(
            super::publication::preserve_state_before_provider_id_migration(
                store.root(),
                original_bytes,
            )?,
        )
    };
    let mut raw = initial.as_value().clone();
    let mut copied = Vec::new();
    for migration in &plan {
        let provider = &mut raw["providers"][migration.provider_index];
        if provider
            .get("apiKeyStored")
            .and_then(serde_json::Value::as_bool)
            .unwrap_or(false)
        {
            match store.copy_api_key_from_legacy_id(&migration.old_id, &migration.new_id) {
                Ok(true) => copied.push(migration.old_id.clone()),
                Ok(false) => {
                    provider["apiKeyStored"] = serde_json::Value::Bool(false);
                    provider["lastError"] = serde_json::Value::String(
                        "旧 Provider ID 已安全迁移；原 API Key 无法自动定位，请重新输入".into(),
                    );
                }
                Err(error) => {
                    provider["apiKeyStored"] = serde_json::Value::Bool(false);
                    provider["lastError"] = serde_json::Value::String(format!(
                        "旧 Provider ID 已安全迁移；API Key 迁移失败，请重新输入：{error}"
                    ));
                }
            }
        }
        provider["id"] = serde_json::Value::String(migration.new_id.clone());
    }
    let mapping: HashMap<_, _> = plan
        .iter()
        .map(|p| (p.old_id.as_str(), p.new_id.as_str()))
        .collect();
    if let Some(active) = raw
        .pointer_mut("/activeByIde/codex")
        .and_then(serde_json::Value::as_array_mut)
    {
        let mut seen = std::collections::HashSet::new();
        let mut transformed = Vec::new();
        for id in active.iter() {
            let id = id
                .as_str()
                .ok_or_else(|| StateError("active codex provider ID must be a string".into()))?;
            let id = mapping.get(id).copied().unwrap_or(id);
            if seen.insert(id.to_owned()) {
                transformed.push(serde_json::Value::String(id.into()));
            }
        }
        *active = transformed;
    }
    if let Some(slots) = raw
        .get_mut("codexNoAccountSlots")
        .and_then(serde_json::Value::as_array_mut)
    {
        for slot in slots {
            if let Some(id) = slot
                .get("providerId")
                .and_then(serde_json::Value::as_str)
                .and_then(|id| mapping.get(id))
            {
                slot["providerId"] = serde_json::Value::String((*id).into());
            }
        }
    }
    let mut imported = 0;
    if let Some(providers) = raw
        .get_mut("providers")
        .and_then(serde_json::Value::as_array_mut)
    {
        for provider in providers {
            if source_version <= 1 {
                if let Some(key) = provider
                    .get("apiKey")
                    .and_then(serde_json::Value::as_str)
                    .filter(|v| !v.is_empty())
                {
                    let id = provider
                        .get("id")
                        .and_then(serde_json::Value::as_str)
                        .unwrap_or("");
                    if !id.is_empty() {
                        store.set_api_key(id, key)?;
                        provider["apiKeyStored"] = serde_json::Value::Bool(true);
                        imported += 1;
                    }
                }
            }
            // Native RelayProvider Deserialize never retains this old plaintext wire field.
            if let Some(provider) = provider.as_object_mut() {
                provider.remove("apiKey");
            }
        }
    }
    let mut candidate = RelayStateDocument::from_value(raw)?;
    candidate.normalize_legacy_active_codex();
    candidate.migrate_legacy_official_protocols()?;
    candidate.migrate_legacy_no_account_slots()?;
    let mut raw = candidate.as_value().clone();
    raw["schemaVersion"] = serde_json::Value::from(16);
    let candidate = RelayStateDocument::from_value(raw)?;
    validate_credentials(&candidate)?;
    let receipt =
        super::publication::write_atomic_report(destination, &candidate.to_pretty_json()?)?
            .acknowledge();
    *state = candidate;
    let mut cleanup_errors = Vec::new();
    for old in copied {
        cleanup_errors.extend(store.cleanup_legacy_provider_key(&old));
    }
    Ok(StateMigrationPublication {
        destination: receipt.destination,
        provider_id_backup: backup,
        provider_id_migrations: plan,
        imported_plaintext_keys: imported,
        cleanup_errors,
        directory_sync_error: receipt.directory_sync_error,
    })
}

const REVISION_FILE_LIMIT: u64 = 0x600400;
const KEY_LIMIT: usize = 0x100000;
const KEY_FORMAT: &str = "aimami-relay-key-v1";
#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
struct StoredKey {
    format: String,
    provider_id: String,
    revision: CredentialRevision,
    api_key: String,
}
pub struct PreparedCredential {
    path: PathBuf,
    provider_id: String,
    revision: CredentialRevision,
    identity: super::publication::FileIdentity,
    published: bool,
}
impl PreparedCredential {
    pub fn path(&self) -> &Path {
        &self.path
    }
    pub fn provider_id(&self) -> &str {
        &self.provider_id
    }
    pub fn reference(&self) -> RelayCredential {
        RelayCredential::Revision {
            id: self.revision.clone(),
        }
    }
}
impl Drop for PreparedCredential {
    fn drop(&mut self) {
        if !self.published && self.identity.verify_owned(&self.path).is_ok() {
            let _ = fs::remove_file(&self.path);
        }
    }
}
pub fn revision_key_path(
    root: &Path,
    provider_id: &str,
    revision: &CredentialRevision,
) -> Result<PathBuf, StateError> {
    crate::catalog::builder::validate_relay_provider_id(provider_id)
        .map_err(|e| StateError(e.to_string()))?;
    Ok(root
        .join("key-revisions")
        .join(format!("{provider_id}.{}", revision.as_str())))
}
pub fn prepare_credential_revision(
    root: &Path,
    provider_id: &str,
    key: &str,
) -> Result<PreparedCredential, StateError> {
    use std::io::Write;
    if key.len() > KEY_LIMIT || key.trim().is_empty() {
        return Err(StateError(
            "relay key is empty or exceeds the storage limit".into(),
        ));
    }
    let revision = CredentialRevision::try_from(super::publication::random_uuid_simple()?)?;
    let path = revision_key_path(root, provider_id, &revision)?;
    let directory = root.join("key-revisions");
    fs::create_dir_all(&directory)?;
    if !fs::symlink_metadata(&directory)?.file_type().is_dir() {
        return Err(StateError("invalid relay credential directory".into()));
    }
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(&directory, fs::Permissions::from_mode(0o700))?;
    }
    let mut options = fs::OpenOptions::new();
    options.write(true).create_new(true);
    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt;
        options.mode(0o600);
    }
    let mut file = options.open(&path)?;
    let identity = super::publication::FileIdentity::read(&file)?;
    let prepared = PreparedCredential {
        path,
        provider_id: provider_id.into(),
        revision: revision.clone(),
        identity,
        published: false,
    };
    let stored = StoredKey {
        format: KEY_FORMAT.into(),
        provider_id: provider_id.into(),
        revision,
        api_key: key.into(),
    };
    file.write_all(&serde_json::to_vec(&stored)?)?;
    file.sync_all()?;
    drop(file);
    fs::File::open(&directory)?.sync_all()?;
    fs::File::open(root)?.sync_all()?;
    Ok(prepared)
}
pub fn read_credential_revision(
    root: &Path,
    provider_id: &str,
    revision: &CredentialRevision,
) -> Result<String, StateError> {
    let path = revision_key_path(root, provider_id, revision)?;
    if !fs::symlink_metadata(root.join("key-revisions"))?
        .file_type()
        .is_dir()
    {
        return Err(StateError("invalid relay credential directory".into()));
    }
    let (stored, _) = read_stored_revision(&path)?;
    if stored.provider_id != provider_id || stored.revision != *revision {
        return Err(StateError(
            "relay credential revision ownership mismatch".into(),
        ));
    }
    Ok(stored.api_key)
}
fn read_stored_revision(
    path: &Path,
) -> Result<(StoredKey, super::publication::FileIdentity), StateError> {
    use std::io::Read;
    let before = fs::symlink_metadata(path)?;
    #[cfg(unix)]
    {
        use std::os::unix::fs::MetadataExt;
        if !before.is_file() || before.nlink() != 1 {
            return Err(StateError(
                "relay credential revision is missing or invalid".into(),
            ));
        }
    }
    let file = fs::File::open(path)?;
    let identity = super::publication::FileIdentity::read(&file)?;
    #[cfg(unix)]
    {
        use std::os::unix::fs::MetadataExt;
        let after = file.metadata()?;
        if before.dev() != after.dev() || before.ino() != after.ino() {
            return Err(StateError(
                "relay credential revision changed while opening".into(),
            ));
        }
    }
    identity.verify_owned(path)?;
    let mut bytes = Vec::new();
    file.take(REVISION_FILE_LIMIT + 1).read_to_end(&mut bytes)?;
    if bytes.len() as u64 > REVISION_FILE_LIMIT {
        return Err(StateError(
            "relay credential file exceeds the storage limit".into(),
        ));
    }
    let stored: StoredKey = serde_json::from_slice(&bytes)
        .map_err(|_| StateError("invalid relay credential revision".into()))?;
    if stored.format != KEY_FORMAT
        || stored.api_key.len() > KEY_LIMIT
        || stored.api_key.trim().is_empty()
    {
        return Err(StateError("invalid relay credential revision".into()));
    }
    crate::catalog::builder::validate_relay_provider_id(&stored.provider_id)
        .map_err(|e| StateError(e.to_string()))?;
    Ok((stored, identity))
}
/// Native sweep checks the saved state's provider ID/credential sequence before scanning,
/// and its original bytes plus directory identity before every deletion. Unknown files stay.
pub fn cleanup_unreferenced_revisions(
    root: &Path,
    state_path: &Path,
    expected: &RelayStateDocument,
) -> Result<(), StateError> {
    let directory = root.join("key-revisions");
    match fs::symlink_metadata(&directory) {
        Err(e) if e.kind() == io::ErrorKind::NotFound => return Ok(()),
        Err(_) => {
            return Err(StateError(
                "cannot inspect relay credential directory".into(),
            ))
        }
        Ok(_) => {}
    }
    let directory_identity = revision_directory_identity(&directory)?;
    let original = fs::read(state_path)
        .map_err(|_| StateError("cannot verify state before credential cleanup".into()))?;
    let saved = RelayStateDocument::from_value(
        serde_json::from_slice(&original)
            .map_err(|_| StateError("invalid state before credential cleanup".into()))?,
    )?;
    let saved_providers = provider_credentials(&saved)?;
    let expected_providers = provider_credentials(expected)?;
    if saved.schema_version()? != expected.schema_version()?
        || saved_providers.len() != expected_providers.len()
        || saved_providers
            .iter()
            .zip(&expected_providers)
            .any(|(a, b)| a.id != b.id || a.credential != b.credential)
    {
        return Err(StateError(
            "relay state changed before credential cleanup".into(),
        ));
    }
    let referenced: std::collections::HashSet<PathBuf> = expected_providers
        .iter()
        .filter_map(|p| match &p.credential {
            RelayCredential::Revision { id } => Some(revision_key_path(root, &p.id, id)),
            _ => None,
        })
        .collect::<Result<_, _>>()?;
    let entries = fs::read_dir(&directory)
        .map_err(|_| StateError("cannot enumerate relay credential revisions".into()))?;
    let mut unrecognized = false;
    for (index, entry) in entries.enumerate() {
        if index >= 4096 {
            return Err(StateError(
                "relay credential cleanup budget exceeded; remaining files retained".into(),
            ));
        }
        let entry =
            entry.map_err(|_| StateError("cannot inspect relay credential revision".into()))?;
        let path = entry.path();
        if referenced.contains(&path) {
            continue;
        }
        let (stored, identity) = match read_stored_revision(&path) {
            Ok(v) => v,
            Err(_) => {
                unrecognized = true;
                continue;
            }
        };
        if revision_key_path(root, &stored.provider_id, &stored.revision)? != path {
            unrecognized = true;
            continue;
        }
        if revision_directory_identity(&directory)? != directory_identity {
            return Err(StateError("relay credential directory changed".into()));
        }
        let current = fs::read(state_path)
            .map_err(|_| StateError("cannot verify state before credential cleanup".into()))?;
        if current != original {
            return Err(StateError(
                "relay state changed before credential cleanup".into(),
            ));
        }
        let removed = match identity.verify_owned(&path) {
            Ok(()) => fs::remove_file(&path),
            // Native remove_owned treats metadata NotFound as an already removed file.
            Err(e) if e.kind() == io::ErrorKind::NotFound => Ok(()),
            Err(e) => Err(e),
        };
        removed.map_err(|_| StateError("unreferenced relay credential cleanup failed".into()))?;
    }
    if unrecognized {
        Err(StateError(
            "unrecognized relay credential files retained".into(),
        ))
    } else {
        Ok(())
    }
}
fn revision_directory_identity(path: &Path) -> Result<(u64, u64), StateError> {
    #[cfg(unix)]
    {
        use std::os::unix::fs::MetadataExt;
        let meta = fs::symlink_metadata(path)
            .map_err(|_| StateError("invalid relay credential directory".into()))?;
        if !meta.is_dir() {
            return Err(StateError("invalid relay credential directory".into()));
        }
        Ok((meta.dev(), meta.ino()))
    }
    #[cfg(not(unix))]
    {
        let _ = path;
        Err(StateError(
            "revision directory identity is unsupported on this platform".into(),
        ))
    }
}
/// Prepared files already exist durably. State publication commits their references; if it
/// fails, ownership-aware Drop removes only those newly prepared files.
pub fn publish_credential_revisions(
    state: &mut RelayStateDocument,
    candidate: RelayStateDocument,
    mut prepared: Vec<PreparedCredential>,
    destination: &Path,
) -> Result<super::publication::AcknowledgedPublication, StateError> {
    candidate.validate_schema_version()?;
    validate_credentials(&candidate)?;
    let providers = provider_credentials(&candidate)?;
    for key in &prepared {
        let reference = key.reference();
        if !providers
            .iter()
            .any(|p| p.id == key.provider_id && p.api_key_stored && p.credential == reference)
        {
            return Err(StateError(
                "prepared credential is not referenced by candidate state".into(),
            ));
        }
    }
    let receipt =
        super::publication::write_atomic_report(destination, &candidate.to_pretty_json()?)?
            .acknowledge();
    for key in &mut prepared {
        key.published = true;
    }
    *state = candidate;
    Ok(receipt)
}
#[derive(Default)]
pub struct MemoryKeychain {
    entries: HashMap<(String, String), Vec<u8>>,
    pub operations: Vec<(String, String, String)>,
    pub fail_next_get: Option<io::ErrorKind>,
    pub fail_next_delete: Option<io::ErrorKind>,
}
impl MemoryKeychain {
    pub fn insert(&mut self, service: &str, account: &str, secret: &[u8]) {
        self.entries
            .insert((service.into(), account.into()), secret.into());
    }
}
impl LegacyKeychain for MemoryKeychain {
    fn get(&mut self, service: &str, account: &str) -> io::Result<Option<Vec<u8>>> {
        self.operations
            .push(("get".into(), service.into(), account.into()));
        if let Some(kind) = self.fail_next_get.take() {
            return Err(kind.into());
        }
        Ok(self.entries.get(&(service.into(), account.into())).cloned())
    }
    fn delete(&mut self, service: &str, account: &str) -> io::Result<()> {
        self.operations
            .push(("delete".into(), service.into(), account.into()));
        if let Some(kind) = self.fail_next_delete.take() {
            return Err(kind.into());
        }
        self.entries.remove(&(service.into(), account.into()));
        Ok(())
    }
}

#[cfg(target_os = "macos")]
mod macos {
    use super::{io, LegacyKeychain};
    use std::{ffi::c_void, ptr};
    type Ref = *const c_void;
    #[link(name = "CoreFoundation", kind = "framework")]
    extern "C" {
        fn CFStringCreateWithBytes(
            allocator: Ref,
            bytes: *const u8,
            len: isize,
            encoding: u32,
            external: u8,
        ) -> Ref;
        fn CFDictionaryCreate(
            allocator: Ref,
            keys: *const Ref,
            values: *const Ref,
            len: isize,
            key_callbacks: Ref,
            value_callbacks: Ref,
        ) -> Ref;
        fn CFRelease(value: Ref);
        fn CFGetTypeID(value: Ref) -> usize;
        fn CFDataGetTypeID() -> usize;
        fn CFDataGetLength(value: Ref) -> isize;
        fn CFDataGetBytePtr(value: Ref) -> *const u8;
        static kCFBooleanTrue: Ref;
    }
    #[link(name = "Security", kind = "framework")]
    extern "C" {
        fn SecItemCopyMatching(query: Ref, result: *mut Ref) -> i32;
        fn SecItemDelete(query: Ref) -> i32;
        static kSecClass: Ref;
        static kSecClassGenericPassword: Ref;
        static kSecAttrService: Ref;
        static kSecAttrAccount: Ref;
        static kSecUseDataProtectionKeychain: Ref;
        static kSecReturnData: Ref;
        static kSecMatchLimit: Ref;
        static kSecMatchLimitOne: Ref;
    }
    struct Owned(Ref);
    impl Owned {
        fn string(value: &str) -> io::Result<Self> {
            let raw = unsafe {
                CFStringCreateWithBytes(
                    ptr::null(),
                    value.as_ptr(),
                    value.len() as isize,
                    0x08000100,
                    0,
                )
            };
            if raw.is_null() {
                Err(io::Error::new(
                    io::ErrorKind::Other,
                    "cannot allocate keychain query string",
                ))
            } else {
                Ok(Self(raw))
            }
        }
    }
    impl Drop for Owned {
        fn drop(&mut self) {
            if !self.0.is_null() {
                unsafe { CFRelease(self.0) }
            }
        }
    }
    fn query(service: &str, account: &str, read: bool) -> io::Result<(Owned, Owned, Owned)> {
        let service = Owned::string(service)?;
        let account = Owned::string(account)?;
        let mut keys = unsafe {
            vec![
                kSecClass,
                kSecAttrService,
                kSecAttrAccount,
                kSecUseDataProtectionKeychain,
            ]
        };
        let mut values = unsafe {
            vec![
                kSecClassGenericPassword,
                service.0,
                account.0,
                kCFBooleanTrue,
            ]
        };
        if read {
            unsafe {
                keys.extend([kSecReturnData, kSecMatchLimit]);
                values.extend([kCFBooleanTrue, kSecMatchLimitOne]);
            }
        }
        // Null callbacks borrow the string references; the returned tuple keeps them alive.
        let raw = unsafe {
            CFDictionaryCreate(
                ptr::null(),
                keys.as_ptr(),
                values.as_ptr(),
                keys.len() as isize,
                ptr::null(),
                ptr::null(),
            )
        };
        if raw.is_null() {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                "cannot allocate keychain query",
            ));
        }
        Ok((Owned(raw), service, account))
    }
    fn error(status: i32) -> io::Error {
        io::Error::new(io::ErrorKind::Other, format!("keychain OSStatus {status}"))
    }
    /// Constructing this adapter does no I/O. Calls require explicit service/account inputs.
    pub struct MacKeychain;
    impl LegacyKeychain for MacKeychain {
        fn get(&mut self, service: &str, account: &str) -> io::Result<Option<Vec<u8>>> {
            let (query, _service, _account) = query(service, account, true)?;
            let mut result = ptr::null();
            let status = unsafe { SecItemCopyMatching(query.0, &mut result) };
            let result = Owned(result);
            if status == -25300 {
                return Ok(None);
            }
            if status != 0 {
                return Err(error(status));
            }
            if result.0.is_null() || unsafe { CFGetTypeID(result.0) != CFDataGetTypeID() } {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "keychain returned a non-data result",
                ));
            }
            let len = unsafe { CFDataGetLength(result.0) };
            if len < 0 {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "keychain returned invalid data length",
                ));
            }
            let bytes = if len == 0 {
                Vec::new()
            } else {
                unsafe { std::slice::from_raw_parts(CFDataGetBytePtr(result.0), len as usize) }
                    .to_vec()
            };
            std::str::from_utf8(&bytes).map_err(|_| {
                io::Error::new(io::ErrorKind::InvalidData, "keychain password is not UTF-8")
            })?;
            Ok(Some(bytes))
        }
        fn delete(&mut self, service: &str, account: &str) -> io::Result<()> {
            let (query, _service, _account) = query(service, account, false)?;
            let status = unsafe { SecItemDelete(query.0) };
            if status == 0 || status == -25300 {
                Ok(())
            } else {
                Err(error(status))
            }
        }
    }
}
#[cfg(target_os = "macos")]
pub use macos::MacKeychain;
