use super::*;
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LiveAuthState {
    Missing,
    VirtualApiKey,
    FlaggedUnlock,
    User,
    Unreadable,
}
pub fn live_auth_state(ctx: &AuthContext) -> LiveAuthState {
    let p = ctx.auth_path();
    if fs::metadata(&p).is_err() {
        return LiveAuthState::Missing;
    }
    let Ok(t) = fs::read_to_string(p) else {
        return LiveAuthState::Unreadable;
    };
    let Ok(v) = serde_json::from_str::<Value>(&t) else {
        return LiveAuthState::Unreadable;
    };
    if v.get("auth_mode")
        .and_then(Value::as_str)
        .is_some_and(|s| s.trim().eq_ignore_ascii_case("apikey"))
        && v.get("OPENAI_API_KEY")
            .and_then(Value::as_str)
            .is_some_and(|s| s.trim().starts_with(VIRTUAL_KEY_PREFIX))
        && v.get("tokens")
            .and_then(Value::as_object)
            .is_none_or(|o| o.is_empty())
    {
        return LiveAuthState::VirtualApiKey;
    }
    if v.get("aimami_router_unlock_auth").and_then(Value::as_bool) == Some(true) {
        LiveAuthState::FlaggedUnlock
    } else {
        LiveAuthState::User
    }
}
pub fn marker_path(ctx: &AuthContext) -> PathBuf {
    ctx.relay_dir().join("router-unlock-auth-marker.json")
}
pub fn backup_path(ctx: &AuthContext) -> PathBuf {
    ctx.relay_dir().join("router-unlock-auth-backup.json")
}
pub fn legacy_marker_path(ctx: &AuthContext) -> PathBuf {
    ctx.relay_dir().join("virtual-auth-marker.json")
}
pub fn has_residue(ctx: &AuthContext) -> bool {
    fs::metadata(marker_path(ctx)).is_ok()
        || fs::metadata(backup_path(ctx)).is_ok()
        || matches!(
            live_auth_state(ctx),
            LiveAuthState::VirtualApiKey | LiveAuthState::FlaggedUnlock
        )
}
pub fn check_router_unlock_auth_residue(ctx: &AuthContext) -> Level {
    if has_residue(ctx) || fs::metadata(legacy_marker_path(ctx)).is_ok() {
        Level::Error
    } else {
        Level::Ok
    }
}

/// None denotes a managed virtual backup that native cleanup deliberately discards.
/// Unsafe user data produces an error and leaves all sidecars untouched.
pub fn read_cleanup_backup(path: &Path) -> Result<Option<Vec<u8>>, AuthError> {
    let bytes = fs::read(path).map_err(|e| ioerr(path, e))?;
    let v: Value = serde_json::from_slice(&bytes).map_err(|_| {
        AuthError::UnsafeBackup("router unlock auth backup is not valid JSON".into())
    })?;
    if !v.is_object() {
        return Err(AuthError::UnsafeBackup(
            "auth backup root is not an object".into(),
        ));
    }
    let typed = serde_json::from_slice::<AuthFile>(&bytes).ok();
    let virtual_key = v
        .get("OPENAI_API_KEY")
        .and_then(Value::as_str)
        .is_some_and(|s| s.trim().starts_with(VIRTUAL_KEY_PREFIX));
    let managed = v.get("aimami_router_unlock_auth").and_then(Value::as_bool) == Some(true)
        || (virtual_key
            && v.get("tokens")
                .and_then(Value::as_object)
                .is_none_or(|o| o.is_empty()))
        || typed.as_ref().is_some_and(is_managed_virtual_auth);
    if v.get("aimami_router_unlock_auth").and_then(Value::as_bool) == Some(true) {
        return Ok(None);
    }
    if let Some(k) = v
        .get("OPENAI_API_KEY")
        .and_then(Value::as_str)
        .map(str::trim)
        .filter(|s| !s.is_empty())
    {
        if !k.starts_with(VIRTUAL_KEY_PREFIX) {
            return Ok(Some(bytes));
        }
    }
    if managed {
        return Ok(None);
    }
    if typed.as_ref().is_some_and(is_switchable_oauth_auth) {
        return Ok(Some(bytes));
    }
    Err(AuthError::UnsafeBackup(
        "backup contains neither user OAuth auth nor user API-key auth".into(),
    ))
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct RouterUnlockMarker {
    #[serde(default, rename = "apiKey")]
    _api_key: Option<String>,
    #[serde(default, rename = "email")]
    _email: Option<String>,
    #[serde(default, rename = "accountKey")]
    _account_key: Option<String>,
    #[serde(default, rename = "createdAt")]
    _created_at: i64,
    #[serde(default)]
    backed_up_previous_auth: bool,
}
/// Stages native unlock cleanup. Legacy cleanup is folded into this plan when present.
pub fn plan_router_unlock_auth_residue(ctx: &AuthContext) -> Result<AuthPlan, AuthError> {
    if ctx.router_mode == RouterMode::NoAccount {
        return Err(AuthError::Unsupported(
            "无账号路由启用时不能执行认证残留修复".into(),
        ));
    }
    let mut plan = AuthPlan {
        changes: Vec::new(),
        selected_backup: None,
        observations: Vec::new(),
    };
    if !has_residue(ctx) && fs::metadata(legacy_marker_path(ctx)).is_err() {
        return Ok(plan);
    }
    for path in [
        ctx.auth_path(),
        marker_path(ctx),
        backup_path(ctx),
        legacy_marker_path(ctx),
    ] {
        plan.observations.push(FileObservation {
            bytes: snapshot(&path)?,
            path,
        });
    }
    let marker = fs::read_to_string(marker_path(ctx))
        .ok()
        .and_then(|t| serde_json::from_str::<RouterUnlockMarker>(&t).ok());
    let live = live_auth_state(ctx);
    if live == LiveAuthState::Unreadable {
        append_legacy_cleanup(ctx, &mut plan)?;
        return Ok(plan);
    }
    let backup = backup_path(ctx);
    let should_restore =
        fs::metadata(&backup).is_ok() && marker.as_ref().is_none_or(|m| m.backed_up_previous_auth);
    match live {
        LiveAuthState::Missing => {
            if should_restore {
                if let Some(bytes) = read_cleanup_backup(&backup)? {
                    plan.changes.push(change(ctx.auth_path(), Some(bytes))?)
                }
            }
        }
        LiveAuthState::VirtualApiKey | LiveAuthState::FlaggedUnlock => {
            let after = if should_restore {
                read_cleanup_backup(&backup)?
            } else {
                None
            };
            plan.changes.push(change(ctx.auth_path(), after)?);
        }
        LiveAuthState::User => {}
        LiveAuthState::Unreadable => unreachable!(),
    }
    for p in [backup, marker_path(ctx)] {
        if fs::metadata(&p).is_ok() {
            plan.changes.push(change(p, None)?);
        }
    }
    append_legacy_cleanup(ctx, &mut plan)?;
    Ok(plan)
}
pub fn apply_router_unlock_auth_residue(
    ctx: &AuthContext,
    plan: &AuthPlan,
) -> Result<Level, AuthError> {
    if ctx.router_mode == RouterMode::NoAccount {
        return Err(AuthError::Unsupported(
            "无账号路由启用时不能执行认证残留修复".into(),
        ));
    }
    let _lock = if plan.changes.is_empty() {
        None
    } else {
        Some(acquire_live_auth_lock(ctx)?)
    };
    apply_plan(plan)?;
    Ok(check_router_unlock_auth_residue(ctx))
}

/// Same coordination file, 20-second deadline, 25-ms retry, and exclusive advisory lock
/// as account_coordination::FileLock::acquire @100785384.
pub fn acquire_live_auth_lock(ctx: &AuthContext) -> Result<fs::File, AuthError> {
    #[cfg(unix)]
    use std::os::unix::fs::OpenOptionsExt;
    let parent = ctx.codex_home.join("accounts");
    fs::create_dir_all(&parent).map_err(|e| ioerr(&parent, e))?;
    let path = parent.join(".live-auth.lock");
    let mut options = fs::OpenOptions::new();
    options.read(true).write(true).create(true);
    #[cfg(unix)]
    options.mode(0o600);
    let file = options.open(&path).map_err(|e| ioerr(&path, e))?;
    let deadline = std::time::Instant::now() + std::time::Duration::from_secs(20);
    loop {
        match file.try_lock() {
            Ok(()) => return Ok(file),
            Err(std::fs::TryLockError::WouldBlock) => {
                if std::time::Instant::now() >= deadline {
                    return Err(ioerr(
                        &path,
                        io::Error::new(
                            io::ErrorKind::TimedOut,
                            "timed out waiting for live auth lock",
                        ),
                    ));
                }
                std::thread::sleep(std::time::Duration::from_millis(25));
            }
            Err(std::fs::TryLockError::Error(e)) if e.kind() == io::ErrorKind::Interrupted => {}
            Err(std::fs::TryLockError::Error(e)) => return Err(ioerr(&path, e)),
        }
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct LegacyMarker {
    api_key: String,
    #[serde(default)]
    backed_up_real_auth: bool,
}
pub fn legacy_backup_path(ctx: &AuthContext) -> PathBuf {
    ctx.relay_dir().join("api-login-auth-backup.json")
}
pub fn read_legacy_user_backup(path: &Path) -> Result<Vec<u8>, AuthError> {
    let bytes = fs::read(path).map_err(|e| ioerr(path, e))?;
    let a: AuthFile = serde_json::from_slice(&bytes).map_err(|_| {
        AuthError::UnsafeBackup("legacy auth backup is invalid; backup was preserved".into())
    })?;
    let api = a
        .auth_mode
        .as_deref()
        .is_some_and(|s| s.eq_ignore_ascii_case("apikey"))
        || a.api_key.as_deref().is_some_and(|s| !s.trim().is_empty());
    if !is_managed_virtual_auth(&a)
        && !a
            .api_key
            .as_deref()
            .is_some_and(|s| s.starts_with(VIRTUAL_KEY_PREFIX))
        && (api || is_switchable_oauth_auth(&a))
    {
        Ok(bytes)
    } else {
        Err(AuthError::UnsafeBackup(
            "legacy auth backup is not user-owned auth; backup was preserved".into(),
        ))
    }
}
fn merge_change(plan: &mut AuthPlan, c: FileChange) {
    if let Some(old) = plan.changes.iter_mut().find(|old| old.path == c.path) {
        old.after = c.after
    } else {
        plan.changes.push(c)
    }
}
fn append_legacy_cleanup(ctx: &AuthContext, plan: &mut AuthPlan) -> Result<(), AuthError> {
    let path = legacy_marker_path(ctx);
    let text = match fs::read_to_string(&path) {
        Ok(t) => t,
        Err(e) if e.kind() == io::ErrorKind::NotFound => return Ok(()),
        Err(e) => return Err(ioerr(&path, e)),
    };
    let marker: LegacyMarker = serde_json::from_str(&text)
        .map_err(|_| AuthError::UnsafeBackup("legacy virtual auth marker is invalid".into()))?;
    let auth = ctx.auth_path();
    let staged = plan
        .changes
        .iter()
        .find(|c| c.path == auth)
        .map(|c| c.after.clone());
    let live = match staged {
        Some(b) => b,
        None => snapshot(&auth)?,
    };
    let live_value = if let Some(b) = &live {
        match std::str::from_utf8(b)
            .ok()
            .and_then(|s| serde_json::from_str::<Value>(s).ok())
        {
            Some(v) => Some(v),
            None => return Ok(()),
        }
    } else {
        None
    };
    let is_legacy = live_value.as_ref().is_some_and(|v| {
        v.get("OPENAI_API_KEY").and_then(Value::as_str) == Some(marker.api_key.as_str())
            && v.get("tokens").is_none_or(Value::is_null)
    });
    let backup = legacy_backup_path(ctx);
    plan.observations.push(FileObservation {
        bytes: snapshot(&backup)?,
        path: backup.clone(),
    });
    if live.is_none() || is_legacy {
        if marker.backed_up_real_auth && fs::metadata(&backup).is_ok() {
            merge_change(plan, change(auth, Some(read_legacy_user_backup(&backup)?))?)
        } else if is_legacy {
            merge_change(plan, change(auth, None)?)
        }
    }
    for p in [backup, path] {
        if fs::metadata(&p).is_ok() {
            merge_change(plan, change(p, None)?);
        }
    }
    Ok(())
}
