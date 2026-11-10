use super::*;
use std::{
    fs,
    os::unix::fs::{symlink, PermissionsExt},
};

struct Fixture(PathBuf);
impl Fixture {
    fn new() -> Self {
        // macOS /var and /tmp are symlinked: use the canonical isolated fixture
        // root explicitly rather than relaxing production ancestor checks.
        let base = fs::canonicalize(std::env::temp_dir()).unwrap();
        let path = base.join(format!("atelier-access-test-{}", uuid::Uuid::new_v4()));
        fs::create_dir(&path).unwrap();
        Self(path)
    }
    fn path(&self) -> &Path {
        &self.0
    }
}
impl Drop for Fixture {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}
fn tree(path: &Path) -> Vec<(PathBuf, Vec<u8>)> {
    let mut out = vec![];
    for entry in fs::read_dir(path).unwrap() {
        let p = entry.unwrap().path();
        if p.is_dir() {
            out.extend(tree(&p));
        } else {
            out.push((p.clone(), fs::read(p).unwrap()));
        }
    }
    out.sort();
    out
}
fn prepare(root: &Fixture) -> PreparedDirectoryProbe {
    prepare_directory_probe(std::slice::from_ref(&root.0)).unwrap()
}

#[test]
fn scope_is_native_four_directories_not_app_root_or_archive() {
    let paths = native_directories(Path::new("/explicit/codex"));
    assert_eq!(
        paths,
        [
            "/explicit/codex",
            "/explicit/codex/sessions",
            "/explicit/codex/codexmate",
            "/explicit/codex/codexmate/relay"
        ]
        .map(PathBuf::from)
    );
}
#[test]
fn config_readiness_only_observes_metadata_even_for_unreadable_bytes() {
    let f = Fixture::new();
    let config = f.path().join("config.toml");
    fs::write(&config, [0xff, 0x00, 0xfe]).unwrap();
    let before = tree(f.path());
    let observed = observe_config(&config).unwrap();
    assert!(observed.exists && observed.is_regular_file && !observed.is_symlink);
    fs::set_permissions(&config, fs::Permissions::from_mode(0o400)).unwrap();
    assert!(observe_config(&config).unwrap().readonly);
    assert_eq!(tree(f.path()), before);
    assert!(!observe_config(&f.path().join("absent")).unwrap().exists);
}
#[test]
fn metadata_errors_are_not_swallowed_or_reclassified_as_missing() {
    let f = Fixture::new();
    fs::write(f.path().join("notdir"), b"existing").unwrap();
    let path = f.path().join("notdir/config.toml");
    assert!(observe_config(&path).is_err());
    assert!(observe_directories(std::slice::from_ref(&path)).is_err());
    assert!(prepare_directory_probe(&[path]).is_err());
    let bad_nul = PathBuf::from(OsStr::from_bytes(b"/bad\0path"));
    assert!(observe_config(&bad_nul).is_err());
}
#[test]
fn prepare_is_read_only_and_missing_directories_do_not_block_existing_home() {
    let f = Fixture::new();
    fs::write(f.path().join("config.toml"), b"original").unwrap();
    fs::write(f.path().join("state_5.sqlite"), b"not opened as sqlite").unwrap();
    let before = tree(f.path());
    let paths = native_directories(f.path());
    let prepared = prepare_directory_probe(&paths).unwrap();
    assert_eq!(prepared.directories().len(), 4);
    assert_eq!(tree(f.path()), before);
    let report = execute_directory_probe(&prepared).unwrap();
    assert_eq!(
        report
            .results
            .iter()
            .map(|r| r.status.as_str())
            .collect::<Vec<_>>(),
        ["ok", "ok", "ok", "ok"]
    );
    assert_eq!(tree(f.path()), before);
    assert!(!f.path().join("sessions").exists());
}
#[test]
fn success_writes_only_random_probe_and_leaves_existing_fixed_probe_intact() {
    let f = Fixture::new();
    fs::write(f.path().join(".aimami_probe"), b"user data").unwrap();
    let before = tree(f.path());
    let prepared = prepare(&f);
    let report = execute_directory_probe(&prepared).unwrap();
    assert_eq!(report.results[0].status, "ok");
    assert!(report.results[0].cleanup_error.is_none());
    assert!(report.results[0].probe_path.is_none());
    assert_eq!(tree(f.path()), before);
}
#[test]
fn probe_symlink_targets_and_ancestors_are_not_followed() {
    let f = Fixture::new();
    let target = f.path().join("real");
    fs::create_dir(&target).unwrap();
    fs::write(target.join("config.toml"), b"target untouched").unwrap();
    let link = f.path().join("link");
    symlink(&target, &link).unwrap();
    let observed = observe_directories(std::slice::from_ref(&link)).unwrap();
    assert!(observed[0].is_symlink && !observed[0].is_directory);
    assert!(prepare_directory_probe(std::slice::from_ref(&link)).is_err());
    assert!(
        observe_config(&link.join("config.toml"))
            .unwrap()
            .is_regular_file
    );
    assert!(prepare_directory_probe(&[link.join("config.toml")]).is_err());
    assert!(prepare_directory_probe(&[link.join("missing")]).is_err());
    assert_eq!(
        fs::read(target.join("config.toml")).unwrap(),
        b"target untouched"
    );
}
#[test]
fn readonly_observations_accept_ancestor_aliases_but_preserve_final_symlinks() {
    let f = Fixture::new();
    let real = f.path().join("real");
    fs::create_dir(&real).unwrap();
    fs::create_dir(real.join("sessions")).unwrap();
    fs::write(real.join("config.toml"), b"unchanged config").unwrap();
    let alias = f.path().join("alias");
    symlink(&real, &alias).unwrap();
    symlink("config.toml", real.join("config-link")).unwrap();
    let before = fs::read(real.join("config.toml")).unwrap();
    let config = observe_config(&alias.join("config.toml")).unwrap();
    assert!(config.exists && config.is_regular_file && !config.is_symlink && !config.readonly);
    let endpoint = observe_config(&alias.join("config-link")).unwrap();
    assert!(endpoint.exists && endpoint.is_symlink && !endpoint.is_regular_file);
    let paths = [alias.join("sessions"), alias.join("missing")];
    let observations = observe_directories(&paths).unwrap();
    assert!(observations[0].exists && observations[0].is_directory && !observations[0].is_symlink);
    assert_eq!(observations[0].path, paths[0]);
    assert!(!observations[1].exists);
    // Read-only acceptance does not authorize writes through an alias, even
    // where its final component is absent at preparation.
    assert!(prepare_directory_probe(std::slice::from_ref(&paths[0])).is_err());
    assert!(prepare_directory_probe(std::slice::from_ref(&paths[1])).is_err());
    assert_eq!(fs::read(real.join("config.toml")).unwrap(), before);
    assert!(!real.join("missing").exists());
}
#[cfg(target_os = "macos")]
#[test]
fn readonly_observations_accept_macos_var_alias_without_canonicalizing_input() {
    let f = Fixture::new();
    let tail = f.path().strip_prefix("/private/var").unwrap();
    let alias = Path::new("/var").join(tail);
    fs::write(f.path().join("config.toml"), b"macOS alias fixture").unwrap();
    assert!(
        observe_config(&alias.join("config.toml"))
            .unwrap()
            .is_regular_file
    );
    let observed = observe_directories(std::slice::from_ref(&alias)).unwrap();
    assert!(observed[0].is_directory);
    assert_eq!(observed[0].path, alias);
    assert!(prepare_directory_probe(std::slice::from_ref(&alias)).is_err());
}
#[test]
fn replaced_directory_rejects_entire_scope_before_any_write() {
    let f = Fixture::new();
    let other = f.path().join("other");
    fs::create_dir(&other).unwrap();
    let prepared = prepare_directory_probe(&[f.0.clone(), other.clone()]).unwrap();
    fs::rename(&other, f.path().join("old-other")).unwrap();
    fs::create_dir(&other).unwrap();
    let before = tree(f.path());
    assert!(execute_directory_probe(&prepared).is_err());
    assert_eq!(tree(f.path()), before);
}
#[test]
fn a_missing_path_appearing_after_preview_requires_new_confirmation() {
    let f = Fixture::new();
    let absent = f.path().join("sessions");
    let prepared = prepare_directory_probe(&[f.0.clone(), absent.clone()]).unwrap();
    fs::create_dir(&absent).unwrap();
    assert!(execute_directory_probe(&prepared).is_err());
    assert_eq!(fs::read_dir(&absent).unwrap().count(), 0);
}
#[derive(Default)]
struct Fault {
    write_error: bool,
    sync_error: bool,
    cleanup_error: bool,
    replace: bool,
    hardlink: bool,
    replace_directory: bool,
    name: Option<String>,
}
impl ProbeIo for Fault {
    fn name(&mut self) -> String {
        self.name
            .clone()
            .unwrap_or_else(|| format!(".atelier-access-probe-{}", uuid::Uuid::new_v4().simple()))
    }
    fn after_create(&mut self, path: &Path) -> io::Result<()> {
        assert_eq!(fs::metadata(path)?.permissions().mode() & 0o777, 0o600);
        if self.replace {
            fs::remove_file(path)?;
            fs::write(path, b"replacement user data")?;
        }
        if self.hardlink {
            fs::hard_link(path, path.parent().unwrap().join("second-link"))?;
        }
        if self.replace_directory {
            let parent = path.parent().unwrap();
            fs::rename(parent, parent.with_extension("moved"))?;
            fs::create_dir(parent)?;
        }
        Ok(())
    }
    fn write_and_sync(&mut self, file: &mut File) -> io::Result<()> {
        if self.write_error {
            return Err(io::Error::from_raw_os_error(libc::ENOSPC));
        }
        file.write_all(PROBE_BYTES)?;
        if self.sync_error {
            return Err(io::Error::from_raw_os_error(libc::EIO));
        }
        file.sync_all()
    }
    fn cleanup(&mut self, dir: &File, name: &CString, identity: Identity) -> io::Result<()> {
        if self.cleanup_error {
            return Err(io::Error::from_raw_os_error(libc::EACCES));
        }
        remove_probe(dir, name, identity)
    }
}
#[test]
fn enospc_and_sync_failures_clean_owned_file_and_report_failure() {
    for sync in [false, true] {
        let f = Fixture::new();
        let prepared = prepare(&f);
        let mut fault = Fault {
            write_error: !sync,
            sync_error: sync,
            ..Default::default()
        };
        let report = execute_with(&prepared, &mut fault).unwrap();
        let r = &report.results[0];
        assert_eq!(r.status, "error");
        assert!(r.cleanup_error.is_none());
        assert!(r.probe_path.is_none());
        assert!(r.detail.contains("probe failed"));
        assert_eq!(fs::read_dir(f.path()).unwrap().count(), 0);
    }
}
#[test]
fn cleanup_failure_is_visible_with_exact_residual_path() {
    let f = Fixture::new();
    let prepared = prepare(&f);
    let report = execute_with(
        &prepared,
        &mut Fault {
            cleanup_error: true,
            ..Default::default()
        },
    )
    .unwrap();
    let r = &report.results[0];
    assert_eq!(r.status, "error");
    assert!(r.cleanup_error.is_some());
    let leftover = r.probe_path.as_ref().unwrap();
    assert_eq!(fs::read(leftover).unwrap(), PROBE_BYTES);
    assert!(r.detail.contains("cleanup failed"));
}
#[test]
fn replaced_probe_is_neither_written_nor_removed() {
    let f = Fixture::new();
    let prepared = prepare(&f);
    let report = execute_with(
        &prepared,
        &mut Fault {
            replace: true,
            ..Default::default()
        },
    )
    .unwrap();
    let r = &report.results[0];
    assert_eq!(r.status, "error");
    assert!(r.cleanup_error.is_some());
    assert_eq!(
        fs::read(r.probe_path.as_ref().unwrap()).unwrap(),
        b"replacement user data"
    );
}
#[test]
fn multiply_linked_probe_is_not_written_or_removed() {
    let f = Fixture::new();
    let prepared = prepare(&f);
    let report = execute_with(
        &prepared,
        &mut Fault {
            hardlink: true,
            ..Default::default()
        },
    )
    .unwrap();
    let r = &report.results[0];
    assert_eq!(r.status, "error");
    assert!(r.cleanup_error.is_some());
    assert_eq!(fs::read(f.path().join("second-link")).unwrap(), b"");
    assert!(r.probe_path.as_ref().unwrap().exists());
}
#[test]
fn exclusive_name_collision_never_truncates_or_cleans_existing_file() {
    let f = Fixture::new();
    let name = ".atelier-access-probe-collision";
    fs::write(f.path().join(name), b"existing file").unwrap();
    let report = execute_with(
        &prepare(&f),
        &mut Fault {
            name: Some(name.into()),
            ..Default::default()
        },
    )
    .unwrap();
    assert_eq!(report.results[0].status, "error");
    assert_eq!(fs::read(f.path().join(name)).unwrap(), b"existing file");
}
#[test]
fn symlink_probe_collision_does_not_follow_or_delete_link() {
    let f = Fixture::new();
    let name = ".atelier-access-probe-collision";
    let target = f.path().join("config.toml");
    fs::write(&target, b"config").unwrap();
    symlink(&target, f.path().join(name)).unwrap();
    let report = execute_with(
        &prepare(&f),
        &mut Fault {
            name: Some(name.into()),
            ..Default::default()
        },
    )
    .unwrap();
    assert_eq!(report.results[0].status, "error");
    assert!(fs::symlink_metadata(f.path().join(name))
        .unwrap()
        .file_type()
        .is_symlink());
    assert_eq!(fs::read(target).unwrap(), b"config");
}
#[test]
fn swapped_directory_during_probe_uses_owned_fd_cleanup_and_reports_error() {
    let f = Fixture::new();
    let directory = f.path().join("scoped");
    fs::create_dir(&directory).unwrap();
    let prepared = prepare_directory_probe(std::slice::from_ref(&directory)).unwrap();
    let report = execute_with(
        &prepared,
        &mut Fault {
            replace_directory: true,
            ..Default::default()
        },
    )
    .unwrap();
    assert_eq!(report.results[0].status, "error");
    assert!(report.results[0].cleanup_error.is_none());
    assert_eq!(fs::read_dir(&directory).unwrap().count(), 0);
    assert_eq!(
        fs::read_dir(directory.with_extension("moved"))
            .unwrap()
            .count(),
        0
    );
}
#[test]
fn space_thresholds_match_native_boundaries_and_query_is_read_only() {
    // 原版三段阈值文案（1 GB / 100 MB / 10 MB）对应四档状态。
    assert_eq!(space_level(10 * 1024 * 1024 - 1), "error");
    assert_eq!(space_level(10 * 1024 * 1024), "error");
    assert_eq!(space_level(100 * 1024 * 1024 - 1), "error");
    assert_eq!(space_level(100 * 1024 * 1024), "warning");
    assert_eq!(space_level(1024 * 1024 * 1024 - 1), "warning");
    assert_eq!(space_level(1024 * 1024 * 1024), "ok");
    // 四档 label 与原文一致。
    assert_eq!(
        space_label(5 * 1024 * 1024),
        "磁盘空间严重不足（剩余 5 MB）"
    );
    assert_eq!(
        space_label(50 * 1024 * 1024),
        "磁盘空间严重偏低（剩余 50 MB）"
    );
    assert_eq!(
        space_label(500 * 1024 * 1024),
        "磁盘空间偏低（剩余 500 MB）"
    );
    assert_eq!(
        space_label(8 * 1024 * 1024 * 1024),
        "文件系统正常（剩余 8.0 GB）"
    );
    let f = Fixture::new();
    let before = tree(f.path());
    let space = observe_available_space(f.path()).unwrap();
    assert!(["ok", "warning", "error"].contains(&space.level.as_str()));
    assert!(observe_available_space(&f.path().join("absent")).is_err());
    assert_eq!(tree(f.path()), before);
}
#[test]
fn unbounded_relative_parent_traversal_scopes_are_rejected() {
    let f = Fixture::new();
    assert!(prepare_directory_probe(&[]).is_err());
    assert!(prepare_directory_probe(&vec![f.0.clone(); 17]).is_err());
    assert!(prepare_directory_probe(&[PathBuf::from("relative")]).is_err());
    assert!(prepare_directory_probe(&[f.path().join("../escape")]).is_err());
}
