#[path = "../src/auth/mod.rs"]
mod auth;
use auth::*;
use std::fs;
use tempfile::TempDir;
fn ctx() -> (TempDir, AuthContext) {
    let d = tempfile::tempdir().unwrap();
    let c = AuthContext::new(d.path(), RouterMode::Off);
    (d, c)
}
fn write(c: &AuthContext, t: &str) {
    fs::write(c.auth_path(), t).unwrap();
}
#[test]
fn missing_empty_invalid_and_unreadable() {
    let (_d, c) = ctx();
    assert_eq!(check_auth_integrity(&c).reason, AuthReason::Missing);
    write(&c, " \n");
    assert_eq!(check_auth_integrity(&c).reason, AuthReason::Empty);
    write(&c, "{");
    assert_eq!(check_auth_integrity(&c).reason, AuthReason::InvalidJson);
    fs::remove_file(c.auth_path()).unwrap();
    fs::create_dir(c.auth_path()).unwrap();
    assert_eq!(check_auth_integrity(&c).reason, AuthReason::Unreadable);
}
#[test]
fn token_precedes_api_and_is_not_trimmed() {
    assert_eq!(
        check_auth_text(r#"{"tokens":{"access_token":"placeholder"},"OPENAI_API_KEY":"sk-real"}"#)
            .reason,
        AuthReason::PlaceholderToken
    );
    assert_eq!(
        check_auth_text(r#"{"OPENAI_API_KEY":" sk-real"}"#).reason,
        AuthReason::MissingToken
    );
    assert_eq!(
        check_auth_text(r#"{"OPENAI_API_KEY":"sk-"}"#).level,
        Level::Ok
    );
    assert_eq!(
        check_auth_text(r#"{"tokens":{"access_token":"   sufficientlylong   "}}"#).level,
        Level::Warning
    );
}
#[test]
fn native_jwt_check_is_syntactic_only() {
    assert!(is_plausible_jwt("e30.e30.not-a-signature"));
    assert!(!is_plausible_jwt("e30.e30"));
    assert!(!is_plausible_jwt("e30!.e30.signature"));
    assert!(is_plausible_jwt("..unchecked"));
    assert!(is_known_placeholder_token("123456789"));
    assert!(is_known_placeholder_token("PLACEHOLDER"));
    assert!(!is_known_placeholder_token("long_realish_token"));
}
#[test]
fn managed_virtual_bypasses_placeholder() {
    assert_eq!(
        check_auth_text(r#"{"aimami_router_unlock_auth":true,"tokens":{"access_token":"test"}}"#)
            .reason,
        AuthReason::ManagedVirtual
    );
    assert_eq!(
        check_auth_text(r#"{"OPENAI_API_KEY":"sk-aimami-relay-virtual-abc"}"#).reason,
        AuthReason::ManagedVirtual
    );
    assert_eq!(
        check_auth_text(
            r#"{"OPENAI_API_KEY":"sk-aimami-relay-virtual-abc","tokens":{"access_token":"test"}}"#
        )
        .reason,
        AuthReason::PlaceholderToken
    );
}
#[test]
fn scan_filters_and_restores_bytes() {
    let (_d, c) = ctx();
    fs::create_dir_all(c.auth_backup_dir()).unwrap();
    let root = c.auth_backup_dir();
    fs::write(root.join("valid.JSON"), r#"{"OPENAI_API_KEY":"sk-upper"}"#).unwrap();
    fs::create_dir(root.join("directory.json")).unwrap();
    fs::write(root.join("bad.json"), "bad").unwrap();
    fs::write(
        root.join("user.json"),
        "{\n \"OPENAI_API_KEY\": \"sk-synthetic\"\n}\n",
    )
    .unwrap();
    let p = plan_auth_integrity(&c).unwrap();
    assert_eq!(p.selected_backup.as_ref().unwrap(), &root.join("user.json"));
    assert!(!c.auth_path().exists());
    assert_eq!(apply_auth_integrity(&c, &p).unwrap().level, Level::Ok);
    assert_eq!(
        fs::read(c.auth_path()).unwrap(),
        fs::read(root.join("user.json")).unwrap()
    );
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        assert_eq!(
            fs::metadata(c.auth_path()).unwrap().permissions().mode() & 0o777,
            0o600
        );
    }
}
#[test]
fn selected_malformed_token_stops_recovery() {
    let (_d, c) = ctx();
    fs::create_dir_all(c.auth_backup_dir()).unwrap();
    let a = c.auth_backup_dir().join("older.json");
    let b = c.auth_backup_dir().join("newer.json");
    fs::write(&a, r#"{"OPENAI_API_KEY":"sk-good"}"#).unwrap();
    fs::write(&b, r#"{"tokens":{"access_token":"long_but_not_jwt"}}"#).unwrap();
    let epoch = std::time::UNIX_EPOCH;
    fs::File::open(&a)
        .unwrap()
        .set_modified(epoch + std::time::Duration::from_secs(100))
        .unwrap();
    fs::File::open(&b)
        .unwrap()
        .set_modified(epoch + std::time::Duration::from_secs(200))
        .unwrap();
    let p = plan_auth_integrity(&c).unwrap();
    assert_eq!(p.selected_backup, Some(b));
    assert_eq!(apply_auth_integrity(&c, &p).unwrap().level, Level::Warning);
}
#[test]
fn enabled_route_blocks_auth_restore() {
    let (_d, mut c) = ctx();
    c.router_mode = RouterMode::Native;
    assert!(matches!(
        plan_auth_integrity(&c),
        Err(AuthError::RouterEnabled)
    ));
    c.router_mode = RouterMode::NoAccount;
    assert!(matches!(
        plan_auth_integrity(&c),
        Err(AuthError::RouterEnabled)
    ));
}
#[test]
fn preview_snapshot_is_rechecked() {
    let (_d, c) = ctx();
    fs::create_dir_all(c.auth_backup_dir()).unwrap();
    fs::write(
        c.auth_backup_dir().join("x.json"),
        r#"{"OPENAI_API_KEY":"sk-synthetic"}"#,
    )
    .unwrap();
    let p = plan_auth_integrity(&c).unwrap();
    write(&c, "new state");
    assert!(matches!(
        apply_auth_integrity(&c, &p),
        Err(AuthError::ChangedSincePreview(_))
    ));
    assert_eq!(fs::read_to_string(c.auth_path()).unwrap(), "new state");
}
#[test]
fn live_state_and_residue_include_virtual_without_sidecar() {
    use auth::unlock::*;
    let (_d, mut c) = ctx();
    assert_eq!(live_auth_state(&c), LiveAuthState::Missing);
    write(
        &c,
        r#"{"auth_mode":" APIKEY ","OPENAI_API_KEY":" sk-aimami-relay-virtual-x ","tokens":[],"aimami_router_unlock_auth":true}"#,
    );
    assert_eq!(live_auth_state(&c), LiveAuthState::VirtualApiKey);
    for m in [RouterMode::Off, RouterMode::Native, RouterMode::NoAccount] {
        c.router_mode = m;
        assert_eq!(check_router_unlock_auth_residue(&c), Level::Error);
    }
    write(&c, r#"{"tokens":{"access_token":"real"}}"#);
    assert_eq!(live_auth_state(&c), LiveAuthState::User);
    assert!(!has_residue(&c));
    write(&c, "bad");
    assert_eq!(live_auth_state(&c), LiveAuthState::Unreadable);
}
#[test]
fn takeover_backup_presence_and_gate() {
    use auth::takeover::*;
    let (_d, mut c) = ctx();
    assert!(!backup_exists(&c));
    fs::create_dir_all(c.relay_dir()).unwrap();
    fs::write(
        c.relay_dir().join("codex_router_top_level_backup.json"),
        "bad",
    )
    .unwrap();
    assert_eq!(check_takeover_backup_orphan(&c), Level::Warning);
    c.router_mode = RouterMode::Native;
    assert_eq!(check_takeover_backup_orphan(&c), Level::Ok);
}
fn sidecars(c: &AuthContext, marker: Option<&str>, backup: Option<&str>) {
    fs::create_dir_all(c.relay_dir()).unwrap();
    if let Some(m) = marker {
        fs::write(auth::unlock::marker_path(c), m).unwrap();
    }
    if let Some(b) = backup {
        fs::write(auth::unlock::backup_path(c), b).unwrap();
    }
}
#[test]
fn cleanup_backup_accepts_any_nonempty_user_key() {
    use auth::unlock::*;
    let (d, _c) = ctx();
    let p = d.path().join("backup.json");
    for text in [
        r#"{"OPENAI_API_KEY":" user key "}"#,
        r#"{"tokens":{"access_token":"one.two.three"}}"#,
        r#"{"tokens":{"id_token":"one.two.three.four"}}"#,
    ] {
        fs::write(&p, text).unwrap();
        assert_eq!(
            read_cleanup_backup(&p).unwrap(),
            Some(text.as_bytes().to_vec())
        );
    }
    for text in [
        "[]",
        "bad",
        r#"{"tokens":{"access_token":"__one.two.three"}}"#,
        r#"{"tokens":{"access_token":"only.two"}}"#,
    ] {
        fs::write(&p, text).unwrap();
        assert!(read_cleanup_backup(&p).is_err());
    }
}
#[test]
fn cleanup_discards_managed_backup_but_rejects_unsafe_user_backup() {
    use auth::unlock::*;
    let (d, _c) = ctx();
    let p = d.path().join("backup.json");
    for text in [
        r#"{"aimami_router_unlock_auth":true}"#,
        r#"{"auth_mode":"apikey","OPENAI_API_KEY":" sk-aimami-relay-virtual-x "}"#,
    ] {
        fs::write(&p, text).unwrap();
        assert!(read_cleanup_backup(&p).unwrap().is_none());
    }
    fs::write(&p,r#"{"OPENAI_API_KEY":" sk-aimami-relay-virtual-x ","tokens":{"access_token":"not-a-token"}}"#).unwrap();
    assert!(read_cleanup_backup(&p).is_err());
}
#[test]
fn orphan_unlock_restores_user_backup_and_removes_sidecars() {
    use auth::unlock::*;
    let (_d, c) = ctx();
    write(&c, r#"{"aimami_router_unlock_auth":true}"#);
    sidecars(&c, None, Some(r#"{"OPENAI_API_KEY":"synthetic user key"}"#));
    let p = plan_router_unlock_auth_residue(&c).unwrap();
    assert_eq!(p.changes.len(), 2);
    assert_eq!(apply_router_unlock_auth_residue(&c, &p).unwrap(), Level::Ok);
    assert_eq!(
        fs::read_to_string(c.auth_path()).unwrap(),
        r#"{"OPENAI_API_KEY":"synthetic user key"}"#
    );
    assert!(!backup_path(&c).exists());
}
#[test]
fn orphan_virtual_backup_is_removed_not_restored() {
    use auth::unlock::*;
    let (_d, c) = ctx();
    write(&c, r#"{"aimami_router_unlock_auth":true}"#);
    sidecars(
        &c,
        Some("broken marker"),
        Some(r#"{"aimami_router_unlock_auth":true}"#),
    );
    let p = plan_router_unlock_auth_residue(&c).unwrap();
    assert_eq!(apply_router_unlock_auth_residue(&c, &p).unwrap(), Level::Ok);
    assert!(!c.auth_path().exists());
    assert!(!marker_path(&c).exists());
}
#[test]
fn cleanup_preserves_user_auth_but_removes_sidecars() {
    use auth::unlock::*;
    let (_d, c) = ctx();
    write(&c, r#"{"OPENAI_API_KEY":"sk-user"}"#);
    sidecars(&c, Some("broken marker"), Some("unparseable backup"));
    let before = fs::read(c.auth_path()).unwrap();
    let p = plan_router_unlock_auth_residue(&c).unwrap();
    assert_eq!(apply_router_unlock_auth_residue(&c, &p).unwrap(), Level::Ok);
    assert_eq!(fs::read(c.auth_path()).unwrap(), before);
}
#[test]
fn unreadable_auth_preserves_sidecars_and_remains_error() {
    use auth::unlock::*;
    let (_d, c) = ctx();
    write(&c, "invalid JSON");
    sidecars(&c, Some("broken marker"), Some("unparseable backup"));
    let p = plan_router_unlock_auth_residue(&c).unwrap();
    assert!(p.changes.is_empty());
    assert_eq!(
        apply_router_unlock_auth_residue(&c, &p).unwrap(),
        Level::Error
    );
    assert!(marker_path(&c).exists());
    assert!(backup_path(&c).exists());
}
#[test]
fn unsafe_backup_produces_no_plan_or_mutation() {
    use auth::unlock::*;
    let (_d, c) = ctx();
    write(&c, r#"{"aimami_router_unlock_auth":true}"#);
    sidecars(&c, None, Some("unparseable backup"));
    assert!(plan_router_unlock_auth_residue(&c).is_err());
    assert!(backup_path(&c).exists());
    assert!(c.auth_path().exists());
}
#[test]
fn no_account_mode_blocks_cleanup() {
    use auth::unlock::*;
    let (_d, mut c) = ctx();
    c.router_mode = RouterMode::NoAccount;
    assert!(plan_router_unlock_auth_residue(&c).is_err());
}
#[test]
fn typed_timestamp_native_coercions() {
    let a: AuthFile = serde_json::from_str(r#"{"last_refresh":"  not-a-date  "}"#).unwrap();
    assert_eq!(a.last_refresh.as_deref(), Some("not-a-date"));
    let a: AuthFile = serde_json::from_str(r#"{"last_refresh":1000}"#).unwrap();
    assert_eq!(a.last_refresh.as_deref(), Some("1970-01-01T00:16:40+00:00"));
    let a: AuthFile = serde_json::from_str(r#"{"last_refresh":" NaN "}"#).unwrap();
    assert_eq!(a.last_refresh.as_deref(), Some("1970-01-01T00:00:00+00:00"));
    assert!(serde_json::from_str::<AuthFile>(r#"{"last_refresh":1e300}"#).is_err());
    assert!(serde_json::from_str::<AuthFile>(r#"{"last_refresh":true}"#).is_err());
}
#[test]
fn marker_controls_whether_backup_is_restored() {
    use auth::unlock::*;
    for flag in [false, true] {
        let (_d, c) = ctx();
        write(&c, r#"{"aimami_router_unlock_auth":true}"#);
        let marker = format!(r#"{{"createdAt":-1,"backedUpPreviousAuth":{flag}}}"#);
        sidecars(
            &c,
            Some(&marker),
            Some(r#"{"OPENAI_API_KEY":"sk-synthetic-user"}"#),
        );
        let p = plan_router_unlock_auth_residue(&c).unwrap();
        apply_router_unlock_auth_residue(&c, &p).unwrap();
        assert_eq!(c.auth_path().exists(), flag);
        assert!(!backup_path(&c).exists());
    }
}
#[test]
fn malformed_marker_uses_orphan_recovery() {
    use auth::unlock::*;
    let (_d, c) = ctx();
    sidecars(
        &c,
        Some(r#"{"createdAt":"invalid type","backedUpPreviousAuth":false}"#),
        Some(r#"{"OPENAI_API_KEY":"sk-synthetic-user"}"#),
    );
    let p = plan_router_unlock_auth_residue(&c).unwrap();
    apply_router_unlock_auth_residue(&c, &p).unwrap();
    assert!(c.auth_path().exists());
}
#[test]
fn legacy_restores_matching_virtual_and_preserves_other_user() {
    use auth::unlock::*;
    for matching in [true, false] {
        let (_d, c) = ctx();
        write(
            &c,
            if matching {
                r#"{"OPENAI_API_KEY":"sk-old-virtual","tokens":null}"#
            } else {
                r#"{"OPENAI_API_KEY":"sk-current-user"}"#
            },
        );
        sidecars(&c, None, None);
        fs::write(
            legacy_marker_path(&c),
            r#"{"apiKey":"sk-old-virtual","backedUpRealAuth":true}"#,
        )
        .unwrap();
        fs::write(
            legacy_backup_path(&c),
            r#"{"OPENAI_API_KEY":"synthetic saved key"}"#,
        )
        .unwrap();
        let p = plan_router_unlock_auth_residue(&c).unwrap();
        apply_router_unlock_auth_residue(&c, &p).unwrap();
        assert_eq!(
            fs::read_to_string(c.auth_path()).unwrap(),
            if matching {
                r#"{"OPENAI_API_KEY":"synthetic saved key"}"#
            } else {
                r#"{"OPENAI_API_KEY":"sk-current-user"}"#
            }
        );
        assert!(!legacy_marker_path(&c).exists());
        assert!(!legacy_backup_path(&c).exists());
    }
}
#[test]
fn legacy_invalid_marker_and_managed_backup_preserved() {
    use auth::unlock::*;
    let (_d, c) = ctx();
    sidecars(&c, None, None);
    fs::write(
        legacy_marker_path(&c),
        r#"{"apiKey":"virtual","backedUpRealAuth":true}"#,
    )
    .unwrap();
    fs::write(
        legacy_backup_path(&c),
        r#"{"aimami_router_unlock_auth":true}"#,
    )
    .unwrap();
    assert!(plan_router_unlock_auth_residue(&c).is_err());
    assert!(legacy_backup_path(&c).exists());
    fs::write(legacy_marker_path(&c), "bad").unwrap();
    assert!(plan_router_unlock_auth_residue(&c).is_err());
    assert!(legacy_marker_path(&c).exists());
}
#[test]
fn legacy_does_not_treat_empty_object_tokens_as_virtual() {
    use auth::unlock::*;
    let (_d, c) = ctx();
    write(&c, r#"{"OPENAI_API_KEY":"virtual","tokens":{}}"#);
    sidecars(&c, None, None);
    fs::write(legacy_marker_path(&c), r#"{"apiKey":"virtual"}"#).unwrap();
    let p = plan_router_unlock_auth_residue(&c).unwrap();
    apply_router_unlock_auth_residue(&c, &p).unwrap();
    assert!(c.auth_path().exists());
}
#[test]
fn native_oauth_shape_only_and_legacy_api_mode() {
    use auth::unlock::*;
    let (d, _c) = ctx();
    let p = d.path().join("backup");
    fs::write(&p, r#"{"auth_mode":"ApiKey"}"#).unwrap();
    assert!(read_legacy_user_backup(&p).is_ok());
    assert!(read_cleanup_backup(&p).is_err());
    fs::write(
        &p,
        r#"{"tokens":{"access_token":"one.two.three","access_token":"four.five.six"}}"#,
    )
    .unwrap();
    assert!(read_cleanup_backup(&p).is_err());
}
#[test]
fn cleanup_rechecks_unchanged_auth_dependency() {
    use auth::unlock::*;
    let (_d, c) = ctx();
    write(&c, r#"{"OPENAI_API_KEY":"sk-user"}"#);
    sidecars(&c, None, Some("broken backup"));
    let p = plan_router_unlock_auth_residue(&c).unwrap();
    write(&c, r#"{"aimami_router_unlock_auth":true}"#);
    assert!(matches!(
        apply_router_unlock_auth_residue(&c, &p),
        Err(AuthError::ChangedSincePreview(_))
    ));
    assert!(backup_path(&c).exists());
}
#[test]
fn live_auth_lock_is_exclusive_until_file_closes() {
    use auth::unlock::*;
    let (_d, c) = ctx();
    let guard = acquire_live_auth_lock(&c).unwrap();
    let second = fs::OpenOptions::new()
        .read(true)
        .write(true)
        .open(c.codex_home.join("accounts/.live-auth.lock"))
        .unwrap();
    assert!(matches!(
        second.try_lock(),
        Err(std::fs::TryLockError::WouldBlock)
    ));
    drop(guard);
    assert!(second.try_lock().is_ok());
}
fn takeover_backup(c: &AuthContext, top: Vec<&str>, sections: serde_json::Value) {
    fs::create_dir_all(c.relay_dir()).unwrap();
    let b = serde_json::json!({"version":1,"createdAtMs":0,"topLevelLines":top,"sectionBlocks":sections,"originalConfig":"never blindly restore this"});
    fs::write(
        c.relay_dir().join("router-takeover-backup.json"),
        serde_json::to_vec(&b).unwrap(),
    )
    .unwrap();
}
#[test]
fn takeover_merges_keys_preserving_current_and_filters_owned() {
    use auth::takeover::*;
    let (_d, c) = ctx();
    fs::write(c.codex_home.join("config.toml"), "model = 'current-user'\n").unwrap();
    takeover_backup(
        &c,
        vec![
            "model = 'backup-user'",
            "theme = 'dark'",
            "theme='duplicate'",
            "profile='aimai1'",
            "model_provider='aimami_relay_x'",
        ],
        serde_json::json!([]),
    );
    let p = plan_takeover_backup_orphan(&c).unwrap();
    assert_eq!(p.report.restored_top_level_keys, vec!["theme"]);
    assert_eq!(p.report.skipped_top_level_keys, vec!["model", "theme"]);
    assert_eq!(apply_takeover_backup_orphan(&c, &p).unwrap(), Level::Ok);
    let t = fs::read_to_string(c.codex_home.join("config.toml")).unwrap();
    assert_eq!(t, "theme = 'dark'\n\nmodel = 'current-user'\n");
    assert!(!backup_exists(&c));
}
#[test]
fn takeover_sections_overlap_and_owned_nested_headers_skip_whole_block() {
    use auth::takeover::*;
    let (_d, c) = ctx();
    fs::write(c.codex_home.join("config.toml"), "[existing.child]\nx=1\n").unwrap();
    takeover_backup(
        &c,
        vec![],
        serde_json::json!([{"key":"existing","lines":["[existing]","value=1"]},{"key":"new","lines":["[new]","x=2"]},{"key":"new.child","lines":["[new.child]","x=3"]},{"key":"foreign","lines":["[foreign]","x=4","[profiles.aimai1]","model='owned'"]}]),
    );
    let p = plan_takeover_backup_orphan(&c).unwrap();
    assert_eq!(p.report.restored_sections, vec!["new"]);
    assert_eq!(p.report.skipped_sections, vec!["existing", "new.child"]);
    apply_takeover_backup_orphan(&c, &p).unwrap();
    assert!(fs::read_to_string(c.codex_home.join("config.toml"))
        .unwrap()
        .contains("[new]\nx=2"));
}
#[test]
fn takeover_inserts_sections_before_general_managed_block() {
    use auth::takeover::*;
    let (_d, c) = ctx();
    let text="# >>> aimami-relay managed start (DO NOT EDIT MANUALLY)\nmodel='aimami_relay'\n# <<< aimami-relay managed end\n";
    fs::write(c.codex_home.join("config.toml"), text).unwrap();
    takeover_backup(
        &c,
        vec!["model='user-model'"],
        serde_json::json!([{"key":"user","lines":["[user]","value=1"]}]),
    );
    let p = plan_takeover_backup_orphan(&c).unwrap();
    assert_eq!(p.report.restored_sections, vec!["user"]);
    let changed = p
        .files
        .changes
        .iter()
        .find(|x| x.path.ends_with("config.toml"))
        .unwrap();
    let t = std::str::from_utf8(changed.after.as_ref().unwrap()).unwrap();
    assert!(t.find("[user]").unwrap() < t.find("# >>>").unwrap());
    assert!(t.contains("model='aimami_relay'"));
    assert!(apply_takeover_backup_orphan(&c, &p).is_ok());
}
#[test]
fn takeover_bad_toml_candidate_keeps_usable_backups() {
    use auth::takeover::*;
    let (_d, c) = ctx();
    takeover_backup(
        &c,
        vec!["theme='first'\ntheme='second'"],
        serde_json::json!([]),
    );
    assert!(matches!(
        plan_takeover_backup_orphan(&c),
        Err(AuthError::InvalidConfig(_))
    ));
    assert!(backup_exists(&c));
    assert!(!c.codex_home.join("config.toml").exists());
}
#[test]
fn takeover_schema_failure_is_preserved_and_not_quarantined() {
    use auth::takeover::*;
    let (_d, c) = ctx();
    fs::create_dir_all(c.relay_dir()).unwrap();
    for b in [
        r#"{"version":2,"createdAtMs":0,"topLevelLines":[],"originalConfig":""}"#,
        r#"{"version":1,"createdAtMs":0,"topLevelLines":[],"originalConfig":"","unknown":1}"#,
        r#"{"topLevelLines":[]}"#,
    ] {
        fs::write(c.relay_dir().join("router-takeover-backup.json"), b).unwrap();
        assert!(plan_takeover_backup_orphan(&c).is_err());
        assert_eq!(fs::read_dir(c.relay_dir()).unwrap().count(), 1);
    }
}
#[test]
fn takeover_damaged_bytes_quarantine_before_removing_active() {
    use auth::takeover::*;
    let (_d, c) = ctx();
    fs::create_dir_all(c.relay_dir()).unwrap();
    let source = c.relay_dir().join("router-takeover-backup.json");
    let bytes = b"{ damaged synthetic data \xff";
    fs::write(&source, bytes).unwrap();
    let p = plan_takeover_backup_orphan(&c).unwrap();
    assert_eq!(p.report.quarantined_files.len(), 1);
    assert!(source.exists());
    assert!(!p.report.quarantined_files[0].exists());
    assert_eq!(apply_takeover_backup_orphan(&c, &p).unwrap(), Level::Ok);
    assert_eq!(fs::read(&p.report.quarantined_files[0]).unwrap(), bytes);
    assert!(!source.exists());
}
#[test]
fn takeover_legacy_default_version_and_empty_backup_postcheck() {
    use auth::takeover::*;
    let (_d, c) = ctx();
    fs::create_dir_all(c.relay_dir()).unwrap();
    let pth = c.relay_dir().join("codex_router_top_level_backup.json");
    fs::write(&pth, r#"{"lines":[]}"#).unwrap();
    let p = plan_takeover_backup_orphan(&c).unwrap();
    assert!(p.files.changes.is_empty());
    assert_eq!(
        apply_takeover_backup_orphan(&c, &p).unwrap(),
        Level::Warning
    );
    fs::write(&pth, r#"{"lines":["theme='user'"]}"#).unwrap();
    let p = plan_takeover_backup_orphan(&c).unwrap();
    assert_eq!(apply_takeover_backup_orphan(&c, &p).unwrap(), Level::Ok);
    assert_eq!(
        fs::read_to_string(c.codex_home.join("config.toml")).unwrap(),
        "theme='user'\n"
    );
}
#[cfg(unix)]
#[test]
fn takeover_rejects_symlink_backup() {
    use auth::takeover::*;
    use std::os::unix::fs::symlink;
    let (d, c) = ctx();
    fs::create_dir_all(c.relay_dir()).unwrap();
    fs::write(d.path().join("target"), "damaged").unwrap();
    symlink(
        d.path().join("target"),
        c.relay_dir().join("router-takeover-backup.json"),
    )
    .unwrap();
    assert!(plan_takeover_backup_orphan(&c).is_err());
    assert_eq!(
        fs::read_to_string(d.path().join("target")).unwrap(),
        "damaged"
    );
}
#[test]
fn takeover_enabled_route_blocks_preview_and_apply() {
    use auth::takeover::*;
    let (_d, mut c) = ctx();
    takeover_backup(&c, vec!["theme='user'"], serde_json::json!([]));
    let p = plan_takeover_backup_orphan(&c).unwrap();
    c.router_mode = RouterMode::Native;
    assert!(matches!(
        plan_takeover_backup_orphan(&c),
        Err(AuthError::RouterEnabled)
    ));
    assert!(matches!(
        apply_takeover_backup_orphan(&c, &p),
        Err(AuthError::RouterEnabled)
    ));
    assert!(backup_exists(&c));
}
#[test]
fn takeover_ownership_and_naive_section_normalization() {
    use auth::takeover::*;
    let (_d, c) = ctx();
    assert!(owned_model_section("profiles.\"aimai1\".tools"));
    assert!(!owned_model_section("profiles.'aimai1'"));
    assert!(owned_top_level_line(&c, "model='aimami_relay_x'"));
    assert!(!owned_top_level_line(&c, "model='aimai1'"));
    assert!(owned_top_level_line(
        &c,
        &format!(
            "model_catalog_json='{}'",
            c.relay_dir().join("codex_router_catalog.json").display()
        )
    ));
    assert_eq!(normalize_section_key(" foo . \"bar\" "), "foo.bar");
    assert!(section_overlaps("x", "x.y"));
    assert!(!section_overlaps("x.y", "x.z"));
}
#[test]
fn managed_claims_use_id_fallback_and_fixed_account() {
    use base64::Engine;
    let claims = serde_json::json!({"https://api.openai.com/auth":{"chatgpt_user_id":"synthetic-unlock","chatgpt_account_id":"00000000-0000-4000-8000-000000000001"}});
    let payload = base64::engine::general_purpose::URL_SAFE_NO_PAD
        .encode(serde_json::to_vec(&claims).unwrap());
    let text =
        serde_json::json!({"tokens":{"id_token":format!("e30.{payload}.synthetic")}}).to_string();
    assert_eq!(check_auth_text(&text).reason, AuthReason::ManagedVirtual);
    let mut value: serde_json::Value = serde_json::from_str(&text).unwrap();
    value["tokens"]["account_id"] = serde_json::json!("different-user-account");
    assert_eq!(
        check_auth_text(&value.to_string()).reason,
        AuthReason::MissingToken
    );
}
#[test]
fn cleanup_user_key_precedes_virtual_claims_but_legacy_rejects_them() {
    use auth::unlock::*;
    use base64::Engine;
    let (d, _c) = ctx();
    let p = d.path().join("backup.json");
    let claims = serde_json::json!({"https://api.openai.com/auth":{"chatgpt_user_id":"synthetic-unlock","chatgpt_account_id":"00000000-0000-4000-8000-000000000001"}});
    let payload = base64::engine::general_purpose::URL_SAFE_NO_PAD
        .encode(serde_json::to_vec(&claims).unwrap());
    let text = serde_json::json!({"OPENAI_API_KEY":"synthetic-user-key","tokens":{"id_token":format!("e30.{payload}.synthetic")}}).to_string();
    fs::write(&p, &text).unwrap();
    assert_eq!(read_cleanup_backup(&p).unwrap(), Some(text.into_bytes()));
    assert!(read_legacy_user_backup(&p).is_err());
}
