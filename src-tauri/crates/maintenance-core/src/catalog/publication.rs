//! Native catalog preparation/publication, recovered from AiMaMi arm64.
//! Every path is explicit. No application is launched and no user home is discovered.
use std::{
    fs::{self, File, OpenOptions},
    io::{self, Read, Write},
    path::{Path, PathBuf},
    time::SystemTime,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CliMetadata {
    pub length: u64,
    pub modified: Option<SystemTime>,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CliSignature {
    Unavailable,
    Located {
        path: PathBuf,
        metadata: Option<CliMetadata>,
    },
}
/// Native priority: nested CodexCLI.app executable, then the legacy Resources/codex.
/// The platform adapter supplies the bundle path; this function performs the actual file checks.
pub fn codex_bundled_cli_path(bundle: &Path) -> Option<PathBuf> {
    [
        "Contents/Resources/codex-cli/CodexCLI.app/Contents/MacOS/codex",
        "Contents/Resources/codex",
    ]
    .iter()
    .map(|relative| bundle.join(relative))
    .find(|p| p.is_file())
}
pub fn current_bundled_cli_signature(bundle: Option<&Path>) -> CliSignature {
    let Some(path) = bundle.and_then(codex_bundled_cli_path) else {
        return CliSignature::Unavailable;
    };
    cli_signature_for_path(&path)
}
/// A path resolved immediately before this call may disappear; native retains that path
/// and records absent metadata rather than confusing it with an unresolved application.
pub fn cli_signature_for_path(path: &Path) -> CliSignature {
    let metadata = fs::metadata(path).ok().map(|m| CliMetadata {
        length: m.len(),
        modified: m.modified().ok(),
    });
    CliSignature::Located {
        path: path.to_owned(),
        metadata,
    }
}

#[derive(Debug)]
pub struct Publication {
    pub destination: PathBuf,
    /// Rename has already succeeded when this is Some. It is a durability warning.
    pub directory_sync_error: Option<io::Error>,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AcknowledgedPublication {
    pub destination: PathBuf,
    pub directory_sync_error: Option<io::ErrorKind>,
}
impl Publication {
    /// Native acknowledge logs the directory-sync error kind and consumes it; publication
    /// remains successful. The returned receipt exposes that same distinction to the caller.
    pub fn acknowledge(self) -> AcknowledgedPublication {
        let error = self.directory_sync_error.map(|e| e.kind());
        if let Some(kind) = error {
            eprintln!("atomic_write: publication directory sync unconfirmed: {kind:?}");
        }
        AcknowledgedPublication {
            destination: self.destination,
            directory_sync_error: error,
        }
    }
    pub fn require_sync(self) -> io::Result<PathBuf> {
        if let Some(e) = self.directory_sync_error {
            return Err(io::Error::new(e.kind(), "ATOMIC_WRITE_PUBLISHED_SYNC_UNCONFIRMED: replacement exists; retain recovery record\n"));
        }
        Ok(self.destination)
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct FileIdentity {
    device: u64,
    inode: u64,
}
#[cfg(unix)]
impl FileIdentity {
    pub(crate) fn read(file: &File) -> io::Result<Self> {
        use std::os::unix::fs::MetadataExt;
        let m = file.metadata()?;
        if !m.is_file() || m.nlink() != 1 {
            return Err(unowned());
        }
        Ok(Self {
            device: m.dev(),
            inode: m.ino(),
        })
    }
    pub(crate) fn verify_owned(self, path: &Path) -> io::Result<()> {
        use std::os::unix::fs::MetadataExt;
        let m = fs::symlink_metadata(path)?;
        if !m.is_file() || m.nlink() != 1 || m.dev() != self.device || m.ino() != self.inode {
            return Err(unowned());
        }
        Ok(())
    }
}
#[cfg(not(unix))]
impl FileIdentity {
    pub(crate) fn read(_: &File) -> io::Result<Self> {
        Err(io::Error::new(
            io::ErrorKind::Unsupported,
            "native staging identity requires Unix metadata",
        ))
    }
    pub(crate) fn verify_owned(self, _: &Path) -> io::Result<()> {
        Err(io::Error::new(
            io::ErrorKind::Unsupported,
            "native staging identity requires Unix metadata",
        ))
    }
}
fn unowned() -> io::Error {
    io::Error::new(
        io::ErrorKind::InvalidData,
        "file identity changed or is not a single regular file",
    )
}
/// Retains ownership identity so a replaced staging path is neither published nor deleted.
#[derive(Debug)]
pub struct StagedWrite {
    destination: PathBuf,
    staging: PathBuf,
    identity: Option<FileIdentity>,
}
impl StagedWrite {
    pub fn staging_path(&self) -> &Path {
        &self.staging
    }
    pub fn publish(mut self) -> io::Result<Publication> {
        self.identity
            .ok_or_else(unowned)?
            .verify_owned(&self.staging)?;
        let result = replace_staged_file_report(&self.staging, &self.destination)?;
        self.identity = None;
        Ok(result)
    }
}
impl Drop for StagedWrite {
    fn drop(&mut self) {
        if let Some(identity) = self.identity {
            if identity.verify_owned(&self.staging).is_ok() {
                let _ = fs::remove_file(&self.staging);
            }
        }
    }
}
pub fn temp_file_stem(path: &Path) -> String {
    let stem: String = path
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("aimami")
        .chars()
        .take(48)
        .collect();
    if stem.is_empty() {
        "aimami".into()
    } else {
        stem
    }
}
pub(crate) fn random_uuid_simple() -> io::Result<String> {
    let mut bytes = [0u8; 16];
    File::open("/dev/urandom")?.read_exact(&mut bytes)?;
    bytes[6] = (bytes[6] & 0x0f) | 0x40;
    bytes[8] = (bytes[8] & 0x3f) | 0x80;
    Ok(bytes.iter().map(|b| format!("{b:02x}")).collect())
}
pub fn stage_atomic(path: &Path, content: &[u8]) -> io::Result<StagedWrite> {
    stage_atomic_with_mode(path, content, None)
}
fn stage_atomic_with_mode(
    path: &Path,
    content: &[u8],
    private_mode: Option<u32>,
) -> io::Result<StagedWrite> {
    let parent = path.parent().ok_or_else(|| {
        io::Error::new(
            io::ErrorKind::InvalidInput,
            "atomic destination has no parent",
        )
    })?;
    fs::create_dir_all(parent)?;
    let staging = parent.join(format!(
        ".{}.tmp.{}.{}",
        temp_file_stem(path),
        std::process::id(),
        random_uuid_simple()?
    ));
    if staging.components().eq(path.components()) || staging.parent() != path.parent() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "invalid atomic staging path",
        ));
    }
    let mut options = OpenOptions::new();
    options.write(true).create_new(true);
    #[cfg(unix)]
    {
        use std::os::unix::fs::{MetadataExt, OpenOptionsExt};
        options.mode(private_mode.unwrap_or_else(|| {
            fs::metadata(path)
                .map(|m| m.mode() & 0o777)
                .unwrap_or(0o666)
        }));
    }
    let mut file = options.open(&staging)?;
    // Native cleanup is armed only after the created handle passed FileIdentity::read.
    let identity = FileIdentity::read(&file)?;
    let staged = StagedWrite {
        destination: path.to_owned(),
        staging,
        identity: Some(identity),
    };
    file.write_all(content)?;
    file.sync_all()?;
    drop(file);
    Ok(staged)
}
pub fn replace_staged_file_report(staging: &Path, destination: &Path) -> io::Result<Publication> {
    let parent = destination.parent().ok_or_else(|| {
        io::Error::new(
            io::ErrorKind::InvalidInput,
            "atomic destination has no parent",
        )
    })?;
    fs::rename(staging, destination)?;
    let directory_sync_error = File::open(parent).and_then(|file| file.sync_all()).err();
    Ok(Publication {
        destination: destination.to_owned(),
        directory_sync_error,
    })
}
pub fn write_atomic_report(path: &Path, content: &[u8]) -> io::Result<Publication> {
    stage_atomic(path, content)?.publish()
}
/// Native private publication always opens the replacement with mode 0600, independently
/// of a preexisting destination's permissions. All staging ownership checks still apply.
pub fn write_atomic_private_report(path: &Path, content: &[u8]) -> io::Result<Publication> {
    stage_atomic_with_mode(path, content, Some(0o600))?.publish()
}
/// Preserves the exact original bytes before changing provider identifiers, then verifies
/// the published backup by reading it back. Root is an explicit application-owned directory.
pub fn preserve_state_before_provider_id_migration(
    root: &Path,
    original: &[u8],
) -> io::Result<PathBuf> {
    let directory = root.join("state-migration-backups");
    fs::create_dir_all(&directory)?;
    let millis = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .map(|d| d.as_millis())
        .unwrap_or(0);
    for attempt in 0..100u8 {
        let destination = directory.join(format!(
            "state-before-provider-id-migration-{millis}-{attempt}.json.bak"
        ));
        if fs::metadata(&destination).is_ok() {
            continue;
        }
        write_atomic_private_report(&destination, original)?.acknowledge();
        if fs::read(&destination)? != original {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                "relay state provider-id migration backup verification failed",
            ));
        }
        return Ok(destination);
    }
    Err(io::Error::new(
        io::ErrorKind::Other,
        "cannot allocate relay state provider-id migration backup path",
    ))
}

#[derive(Debug, Clone)]
pub struct PreparedCatalog {
    pub document: serde_json::Value,
    pub bytes: Vec<u8>,
    pub cli_signature: CliSignature,
    pub native_account: bool,
}
pub fn prepare_catalog(
    state: &super::state::RelayStateDocument,
    bundle: Option<&Path>,
) -> Result<PreparedCatalog, super::state::StateError> {
    let inputs = state.catalog_inputs()?;
    let document = crate::catalog::builder::prepare_router_catalog(
        &inputs.providers,
        &inputs.active_provider_ids,
        !inputs.no_account_mode,
        &inputs.aliases,
    )
    .map_err(|e| super::state::StateError(e.to_string()))?;
    let bytes = crate::catalog::builder::encode_prepared_catalog(&document)
        .map_err(|e| super::state::StateError(e.to_string()))?;
    Ok(PreparedCatalog {
        document,
        bytes,
        cli_signature: current_bundled_cli_signature(bundle),
        native_account: !inputs.no_account_mode,
    })
}
pub fn write_prepared_catalog(
    root: &Path,
    prepared: &PreparedCatalog,
) -> io::Result<AcknowledgedPublication> {
    Ok(
        write_atomic_report(&root.join(crate::catalog::CATALOG_FILE), &prepared.bytes)?
            .acknowledge(),
    )
}
