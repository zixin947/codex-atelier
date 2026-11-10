//! Explicit-path access observations and separately confirmed directory probes.
//!
//! Native evidence: decomp-run_diagnostics_with_mode.c:4339-4503 checks home,
//! sessions, home/codexmate, home/codexmate/relay. CodexPaths_from_home.c:117-129,
//! 258-292 maps the struct offsets. Config readiness is metadata/type/permissions,
//! NOT a trial write. Native :4536-4578 opens an existing source DB RW in normal
//! mode (0x8002), RO in read-only mode (0x8001); neither is performed here because
//! even SQLite opens can affect sidecars. :4581-4795 observes home free space.
//!
//! This adapter deliberately strengthens the native fixed-name write/remove:
//! random exclusive 0600 file, descriptor-relative operations, no symlink
//! traversal, identity checks, and visible cleanup failures. It does not use
//! catalog::publication::stage_atomic: that helper calls create_dir_all, renames
//! destinations and hides Drop cleanup errors. We retain its ownership invariant
//! (device/inode + single regular file) without those publication side effects.
use serde::Serialize;
use std::{
    fs::File,
    io,
    path::{Path, PathBuf},
};

pub const MAX_PROBE_DIRECTORIES: usize = 16;
const PROBE_BYTES: &[u8] = b"atelier-directory-access-probe\n";

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ConfigObservation {
    pub exists: bool,
    pub is_regular_file: bool,
    pub is_symlink: bool,
    pub readonly: bool,
}
#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DirectoryObservation {
    pub path: PathBuf,
    pub exists: bool,
    pub is_directory: bool,
    pub is_symlink: bool,
    pub readonly: bool,
}
#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SpaceObservation {
    pub available_bytes: u64,
    /// Native thresholds: error <10 MiB; warning <1 GiB (100 MiB splits text).
    pub level: String,
}
#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DirectoryProbeResult {
    pub path: PathBuf,
    /// ok, skipped (missing at preparation), or error. Never claims config/DB RW.
    pub status: String,
    pub detail: String,
    /// Present when the probe may remain, including identity-change refusal.
    pub cleanup_error: Option<String>,
    pub probe_path: Option<PathBuf>,
}
#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DirectoryProbeReport {
    pub results: Vec<DirectoryProbeResult>,
}

/// Contains read-only directory handles, not a write authorization. The caller
/// must present its paths and obtain a separate explicit confirmation/ticket.
#[derive(Debug)]
pub struct PreparedDirectoryProbe {
    directories: Vec<DirectoryObservation>,
    #[cfg(unix)]
    handles: Vec<Option<File>>,
}
impl PreparedDirectoryProbe {
    pub fn directories(&self) -> &[DirectoryObservation] {
        &self.directories
    }
}

/// Exact native directory scope; never substitutes Atelier's app-data root.
pub fn native_directories(home: &Path) -> Vec<PathBuf> {
    vec![
        home.to_owned(),
        home.join("sessions"),
        home.join("codexmate"),
        home.join("codexmate/relay"),
    ]
}

/// Observation preserves ordinary ancestor aliases (including macOS /var), but
/// does not follow a final symlink. This is not authorization for a write probe.
fn observation_metadata(path: &Path) -> io::Result<Option<std::fs::Metadata>> {
    match std::fs::symlink_metadata(path) {
        Ok(metadata) => Ok(Some(metadata)),
        Err(error) if error.kind() == io::ErrorKind::NotFound => Ok(None),
        Err(error) => Err(error),
    }
}
pub fn observe_config(path: &Path) -> io::Result<ConfigObservation> {
    let m = observation_metadata(path)?;
    Ok(ConfigObservation {
        exists: m.is_some(),
        is_regular_file: m.as_ref().is_some_and(|m| m.is_file()),
        is_symlink: m.as_ref().is_some_and(|m| m.file_type().is_symlink()),
        readonly: m.as_ref().is_some_and(|m| m.permissions().readonly()),
    })
}
pub fn observe_directories(paths: &[PathBuf]) -> io::Result<Vec<DirectoryObservation>> {
    validate_count(paths)?;
    paths
        .iter()
        .map(|p| {
            let m = observation_metadata(p)?;
            Ok(DirectoryObservation {
                path: p.clone(),
                exists: m.is_some(),
                is_directory: m.as_ref().is_some_and(|m| m.is_dir()),
                is_symlink: m.as_ref().is_some_and(|m| m.file_type().is_symlink()),
                readonly: m.as_ref().is_some_and(|m| m.permissions().readonly()),
            })
        })
        .collect()
}
/// A query failure remains Err, not invented free capacity or a healthy result.
pub fn observe_available_space(path: &Path) -> io::Result<SpaceObservation> {
    let bytes = platform::available_space(path)?;
    Ok(SpaceObservation {
        available_bytes: bytes,
        level: space_level(bytes).into(),
    })
}
/// 原版阈值见常量 0x10130e4e9 的三段文案：1 GB / 100 MB / 10 MB。
/// 四档对应四个 label：正常、偏低、严重偏低、严重不足。
/// 状态映射：>=1GB 为 ok；<1GB 为 warning；<100MB 与 <10MB 为 error。
pub fn space_level(bytes: u64) -> &'static str {
    if bytes < 10 * 1024 * 1024 {
        "error"
    } else if bytes < 100 * 1024 * 1024 {
        "error"
    } else if bytes < 1024 * 1024 * 1024 {
        "warning"
    } else {
        "ok"
    }
}

/// 原版四档磁盘 label；数值以 GB/MB 呈现。
pub fn space_label(bytes: u64) -> String {
    let human = if bytes >= 1024 * 1024 * 1024 {
        format!("{:.1} GB", bytes as f64 / (1024.0 * 1024.0 * 1024.0))
    } else {
        format!("{} MB", bytes / (1024 * 1024))
    };
    if bytes < 10 * 1024 * 1024 {
        format!("磁盘空间严重不足（剩余 {human}）")
    } else if bytes < 100 * 1024 * 1024 {
        format!("磁盘空间严重偏低（剩余 {human}）")
    } else if bytes < 1024 * 1024 * 1024 {
        format!("磁盘空间偏低（剩余 {human}）")
    } else {
        format!("文件系统正常（剩余 {human}）")
    }
}
fn validate_count(paths: &[PathBuf]) -> io::Result<()> {
    if paths.is_empty() || paths.len() > MAX_PROBE_DIRECTORIES {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "directory scope must contain 1..=16 explicit paths",
        ));
    }
    Ok(())
}
fn invalid(message: &str) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidInput, message)
}
fn changed() -> io::Error {
    io::Error::new(
        io::ErrorKind::InvalidData,
        "directory identity changed since preparation; prepare and confirm again",
    )
}

/// Read-only: no create_dir_all, probe creation, SQLite open, or write test.
/// Missing paths are retained as skipped so an absent sessions directory cannot
/// prevent an already-existing home from being tested after confirmation.
pub fn prepare_directory_probe(paths: &[PathBuf]) -> io::Result<PreparedDirectoryProbe> {
    platform::prepare(paths)
}
/// Call only AFTER a distinct, user-confirmed preview. One bounded write/sync/
/// cleanup per existing directory; no worker, process, retry loop or background IO.
/// Preflight identity errors return before any probe is created. Per-directory
/// write and cleanup failures are preserved in the returned report.
pub fn execute_directory_probe(
    prepared: &PreparedDirectoryProbe,
) -> io::Result<DirectoryProbeReport> {
    platform::execute(prepared)
}

#[cfg(unix)]
mod platform {
    use super::*;
    use std::{
        ffi::{CString, OsStr},
        io::Write,
        os::{
            fd::{AsRawFd, FromRawFd},
            unix::ffi::OsStrExt,
        },
        path::Component,
    };

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    struct Identity {
        device: u64,
        inode: u64,
    }
    impl Identity {
        // libc device/inode widths differ between supported Unix targets.
        #[allow(clippy::unnecessary_cast)]
        fn of(m: &libc::stat) -> Self {
            Self {
                device: m.st_dev as u64,
                inode: m.st_ino as u64,
            }
        }
    }
    pub(super) struct Metadata {
        identity: Identity,
    }
    fn cstring(s: &OsStr) -> io::Result<CString> {
        CString::new(s.as_bytes()).map_err(|_| invalid("path contains NUL"))
    }
    fn stat_fd(file: &File) -> io::Result<libc::stat> {
        let mut stat = std::mem::MaybeUninit::<libc::stat>::uninit();
        // SAFETY: valid open fd, writable correctly sized stat allocation.
        if unsafe { libc::fstat(file.as_raw_fd(), stat.as_mut_ptr()) } != 0 {
            return Err(io::Error::last_os_error());
        }
        Ok(unsafe { stat.assume_init() })
    }
    fn stat_at(parent: &File, name: &CString) -> io::Result<Option<libc::stat>> {
        let mut stat = std::mem::MaybeUninit::<libc::stat>::uninit();
        // SAFETY: CString is terminated; parent fd and output storage are valid.
        if unsafe {
            libc::fstatat(
                parent.as_raw_fd(),
                name.as_ptr(),
                stat.as_mut_ptr(),
                libc::AT_SYMLINK_NOFOLLOW,
            )
        } != 0
        {
            let e = io::Error::last_os_error();
            return if e.kind() == io::ErrorKind::NotFound {
                Ok(None)
            } else {
                Err(e)
            };
        }
        Ok(Some(unsafe { stat.assume_init() }))
    }
    fn open_at(parent: &File, name: &CString, flags: i32, mode: libc::mode_t) -> io::Result<File> {
        // SAFETY: valid fd and CString; mode supplied for O_CREAT; ownership transfers once.
        let fd = unsafe {
            libc::openat(
                parent.as_raw_fd(),
                name.as_ptr(),
                flags | libc::O_CLOEXEC | libc::O_NOFOLLOW | libc::O_NONBLOCK,
                mode as libc::c_uint,
            )
        };
        if fd < 0 {
            Err(io::Error::last_os_error())
        } else {
            Ok(unsafe { File::from_raw_fd(fd) })
        }
    }
    fn components(path: &Path) -> io::Result<Vec<CString>> {
        if !path.is_absolute() {
            return Err(invalid("an absolute directory/file path is required"));
        }
        let mut names = vec![];
        for part in path.components() {
            match part {
                Component::RootDir => {}
                Component::Normal(s) => names.push(cstring(s)?),
                _ => {
                    return Err(invalid(
                        "path must not contain parent traversal or a platform prefix",
                    ))
                }
            }
        }
        if names.len() > 128 {
            return Err(invalid("path exceeds 128 components"));
        }
        Ok(names)
    }
    fn root() -> io::Result<File> {
        use std::os::unix::fs::OpenOptionsExt;
        std::fs::OpenOptions::new()
            .read(true)
            .custom_flags(libc::O_DIRECTORY | libc::O_NOFOLLOW | libc::O_CLOEXEC)
            .open("/")
    }
    /// Walk each component without following symlinks, including ancestors.
    fn parent(path: &Path) -> io::Result<Option<(File, CString)>> {
        let mut names = components(path)?;
        let last = names.pop().unwrap_or_else(|| CString::new(".").unwrap());
        let mut dir = root()?;
        for name in names {
            match open_at(&dir, &name, libc::O_RDONLY | libc::O_DIRECTORY, 0) {
                Ok(next) => dir = next,
                Err(e) if e.kind() == io::ErrorKind::NotFound => return Ok(None),
                Err(e) => return Err(e),
            }
        }
        Ok(Some((dir, last)))
    }
    pub(super) fn metadata(path: &Path) -> io::Result<Option<Metadata>> {
        let Some((dir, name)) = parent(path)? else {
            return Ok(None);
        };
        Ok(stat_at(&dir, &name)?.map(|m| Metadata {
            identity: Identity::of(&m),
        }))
    }
    fn open_directory(path: &Path) -> io::Result<File> {
        let (dir, name) = parent(path)?.ok_or_else(|| {
            io::Error::new(io::ErrorKind::NotFound, "directory parent is missing")
        })?;
        open_at(&dir, &name, libc::O_RDONLY | libc::O_DIRECTORY, 0)
    }
    // libc fs counter widths differ between supported Unix targets.
    #[allow(clippy::unnecessary_cast)]
    pub(super) fn available_space(path: &Path) -> io::Result<u64> {
        let file = open_directory(path)?;
        let mut v = std::mem::MaybeUninit::<libc::statvfs>::uninit();
        // SAFETY: valid fd and writable statvfs storage. Does not create anything.
        if unsafe { libc::fstatvfs(file.as_raw_fd(), v.as_mut_ptr()) } != 0 {
            return Err(io::Error::last_os_error());
        }
        let v = unsafe { v.assume_init() };
        (v.f_bavail as u64)
            .checked_mul(v.f_frsize as u64)
            .ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidData, "available capacity overflow")
            })
    }
    pub(super) fn prepare(paths: &[PathBuf]) -> io::Result<PreparedDirectoryProbe> {
        let directories = observe_directories(paths)?;
        let mut handles = Vec::with_capacity(paths.len());
        for observed in &directories {
            // Even absent targets must pass the strict ancestor walk. Read-only
            // observations accepting an alias never weaken probe authorization.
            let strict = metadata(&observed.path)?;
            if strict.is_some() != observed.exists {
                return Err(changed());
            }
            if !observed.exists {
                handles.push(None);
                continue;
            }
            if !observed.is_directory || observed.is_symlink {
                return Err(invalid(
                    "probe target must be an existing non-symlink directory",
                ));
            }
            let handle = open_directory(&observed.path)?;
            let current = metadata(&observed.path)?.ok_or_else(changed)?;
            if current.identity != Identity::of(&stat_fd(&handle)?) {
                return Err(changed());
            }
            handles.push(Some(handle));
        }
        Ok(PreparedDirectoryProbe {
            directories,
            handles,
        })
    }
    fn verify_directory(path: &Path, handle: &File) -> io::Result<()> {
        let fresh = open_directory(path)?;
        if Identity::of(&stat_fd(&fresh)?) != Identity::of(&stat_fd(handle)?) {
            return Err(changed());
        }
        Ok(())
    }
    fn verify_probe(dir: &File, name: &CString, identity: Identity) -> io::Result<()> {
        let m = stat_at(dir, name)?.ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                "probe path disappeared; refusing cleanup",
            )
        })?;
        if m.st_mode & libc::S_IFMT != libc::S_IFREG
            || m.st_nlink != 1
            || Identity::of(&m) != identity
        {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "probe identity changed or is not a single regular file; refusing cleanup",
            ));
        }
        Ok(())
    }
    fn remove_probe(dir: &File, name: &CString, identity: Identity) -> io::Result<()> {
        verify_probe(dir, name, identity)?;
        // SAFETY: fd-relative, single-component name, verified owned regular file.
        if unsafe { libc::unlinkat(dir.as_raw_fd(), name.as_ptr(), 0) } != 0 {
            return Err(io::Error::last_os_error());
        }
        Ok(())
    }
    trait ProbeIo {
        fn name(&mut self) -> String {
            format!(".atelier-access-probe-{}", uuid::Uuid::new_v4().simple())
        }
        fn after_create(&mut self, _: &Path) -> io::Result<()> {
            Ok(())
        }
        fn write_and_sync(&mut self, file: &mut File) -> io::Result<()> {
            file.write_all(PROBE_BYTES)?;
            file.sync_all()
        }
        fn cleanup(&mut self, dir: &File, name: &CString, identity: Identity) -> io::Result<()> {
            remove_probe(dir, name, identity)
        }
    }
    struct SystemIo;
    impl ProbeIo for SystemIo {}
    fn one(path: &Path, dir: &File, io: &mut impl ProbeIo) -> DirectoryProbeResult {
        let mut result = DirectoryProbeResult {
            path: path.to_owned(),
            status: "error".into(),
            detail: String::new(),
            cleanup_error: None,
            probe_path: None,
        };
        if let Err(e) = verify_directory(path, dir) {
            result.detail = e.to_string();
            return result;
        }
        let name = io.name();
        // Also constrain injected names so tests cannot bypass the safety boundary.
        if !name.starts_with(".atelier-access-probe-")
            || name.len() > 100
            || name.contains('/')
            || name.contains('\0')
        {
            result.detail = "invalid probe name".into();
            return result;
        }
        let name = CString::new(name).unwrap();
        let probe_path = path.join(OsStr::from_bytes(name.as_bytes()));
        let mut file = match open_at(
            dir,
            &name,
            libc::O_WRONLY | libc::O_CREAT | libc::O_EXCL,
            0o600,
        ) {
            Ok(f) => f,
            Err(e) => {
                result.detail = format!("exclusive probe creation failed: {e}");
                return result;
            }
        };
        result.probe_path = Some(probe_path.clone());
        let identity = match stat_fd(&file).and_then(|m| {
            if m.st_mode & libc::S_IFMT == libc::S_IFREG && m.st_nlink == 1 {
                Ok(Identity::of(&m))
            } else {
                Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "created handle is not a single regular file",
                ))
            }
        }) {
            Ok(identity) => identity,
            Err(e) => {
                result.detail = e.to_string();
                result.cleanup_error =
                    Some("ownership not established; no unverified path was removed".into());
                return result;
            }
        };
        let write_result = io
            .after_create(&probe_path)
            .and_then(|_| verify_directory(path, dir))
            .and_then(|_| verify_probe(dir, &name, identity))
            .and_then(|_| io.write_and_sync(&mut file));
        // Keep the owned handle live through identity check and cleanup so its inode
        // cannot be recycled. A replaced directory cannot redirect the unlink.
        let cleanup = io.cleanup(dir, &name, identity);
        match cleanup {
            Ok(()) => result.probe_path = None,
            Err(e) => result.cleanup_error = Some(e.to_string()),
        }
        let final_identity = verify_directory(path, dir);
        match write_result.and(final_identity) {
            Ok(()) if result.cleanup_error.is_none() => {
                result.status = "ok".into();
                result.detail = "exclusive probe write, sync and owned-file cleanup succeeded; config and SQLite were not modified".into();
            }
            Ok(()) => {
                result.detail =
                    "probe write/sync succeeded but cleanup failed; writable result is not complete"
                        .into()
            }
            Err(e) => result.detail = format!("probe failed: {e}"),
        }
        result
    }
    fn execute_with(
        prepared: &PreparedDirectoryProbe,
        io: &mut impl ProbeIo,
    ) -> io::Result<DirectoryProbeReport> {
        // Validate every reviewed path before writing anywhere.
        for (observed, handle) in prepared.directories.iter().zip(&prepared.handles) {
            match handle {
                Some(handle) => verify_directory(&observed.path, handle)?,
                None if metadata(&observed.path)?.is_some() => return Err(changed()),
                None => {}
            }
        }
        let results = prepared
            .directories
            .iter()
            .zip(&prepared.handles)
            .map(|(observed, handle)| match handle {
                Some(handle) => one(&observed.path, handle, io),
                None => DirectoryProbeResult {
                    path: observed.path.clone(),
                    status: "ok".into(),
                    detail: "directory did not exist at preparation; not created or probed".into(),
                    cleanup_error: None,
                    probe_path: None,
                },
            })
            .collect();
        Ok(DirectoryProbeReport { results })
    }
    pub(super) fn execute(prepared: &PreparedDirectoryProbe) -> io::Result<DirectoryProbeReport> {
        execute_with(prepared, &mut SystemIo)
    }

    #[cfg(test)]
    mod tests {
        include!("access_checks_tests.rs");
    }
}

#[cfg(not(unix))]
mod platform {
    use super::*;
    fn unsupported<T>() -> io::Result<T> {
        Err(io::Error::new(
            io::ErrorKind::Unsupported,
            "safe no-follow access checks require Unix descriptor operations",
        ))
    }
    pub(super) fn available_space(_: &Path) -> io::Result<u64> {
        unsupported()
    }
    pub(super) fn prepare(_: &[PathBuf]) -> io::Result<PreparedDirectoryProbe> {
        unsupported()
    }
    pub(super) fn execute(_: &PreparedDirectoryProbe) -> io::Result<DirectoryProbeReport> {
        unsupported()
    }
}
