use super::*;
pub fn backup_exists(ctx: &AuthContext) -> bool {
    fs::metadata(ctx.relay_dir().join("router-takeover-backup.json")).is_ok()
        || fs::metadata(ctx.relay_dir().join("codex_router_top_level_backup.json")).is_ok()
}
pub fn check_takeover_backup_orphan(ctx: &AuthContext) -> Level {
    if !ctx.router_mode.enabled() && backup_exists(ctx) {
        Level::Warning
    } else {
        Level::Ok
    }
}

use super::lexical::Lexical;
use std::collections::HashSet;
const BEGIN: [&str; 3] = [
    "# >>> aimami-relay managed start (DO NOT EDIT MANUALLY)",
    "# >>> aimami-relay managed start (top, DO NOT EDIT MANUALLY)",
    "# >>> aimami-relay codex-router top start (DO NOT EDIT MANUALLY)",
];
const END: [&str; 3] = [
    "# <<< aimami-relay managed end",
    "# <<< aimami-relay managed end (top)",
    "# <<< aimami-relay codex-router top end",
];
#[derive(Clone, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct TakeoverBackup {
    pub version: u32,
    pub created_at_ms: i64,
    pub top_level_lines: Vec<String>,
    #[serde(default)]
    pub section_blocks: Vec<TakeoverSection>,
    pub original_config: String,
}
#[derive(Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TakeoverSection {
    pub key: String,
    pub lines: Vec<String>,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct LegacyBackup {
    #[serde(default = "version_one")]
    version: u32,
    lines: Vec<String>,
}
fn version_one() -> u32 {
    1
}
#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct RestoreReport {
    pub restored_top_level_keys: Vec<String>,
    pub restored_sections: Vec<String>,
    pub skipped_top_level_keys: Vec<String>,
    pub skipped_sections: Vec<String>,
    pub quarantined_files: Vec<PathBuf>,
}
pub struct TakeoverPlan {
    pub files: AuthPlan,
    pub report: RestoreReport,
    pub quarantine_pairs: Vec<(PathBuf, PathBuf)>,
}
fn owned_provider(s: &str) -> bool {
    s == "aimami_relay"
        || s.starts_with("aimami_relay_")
        || s == "aimai1"
        || s == "aimami_codex_router"
}
fn first_section_part(s: &str) -> String {
    let mut out = String::new();
    let mut quoted = false;
    for c in s.chars() {
        if c == '"' {
            quoted = !quoted
        } else if c == '.' && !quoted {
            break;
        } else {
            out.push(c)
        }
    }
    out.trim().to_owned()
}
pub fn owned_model_section(s: &str) -> bool {
    s.strip_prefix("model_providers.")
        .or_else(|| s.strip_prefix("profiles."))
        .is_some_and(|s| owned_provider(&first_section_part(s)))
}
pub fn normalize_section_key(s: &str) -> String {
    s.split('.')
        .map(|p| p.trim().trim_matches('"'))
        .collect::<Vec<_>>()
        .join(".")
}
pub fn section_overlaps(a: &str, b: &str) -> bool {
    a.split('.').zip(b.split('.')).all(|(a, b)| a == b)
}
fn assignment_key(s: &str) -> Option<&str> {
    let s = s.trim();
    if s.starts_with('#') {
        return None;
    }
    let (key, _) = s.split_once('=')?;
    let key = key.trim();
    (!key.is_empty()
        && key
            .bytes()
            .all(|b| b.is_ascii_alphanumeric() || b == b'_' || b == b'-'))
    .then_some(key)
}
fn normalize_path(s: &str) -> String {
    s.trim().replace('\\', "/").trim_end_matches('/').to_owned()
}
pub fn owned_top_level_line(ctx: &AuthContext, line: &str) -> bool {
    let Some(key) = assignment_key(line) else {
        return false;
    };
    let Ok(v) = toml::from_str::<toml::Value>(line) else {
        return false;
    };
    let Some(s) = v.get(key).and_then(toml::Value::as_str) else {
        return false;
    };
    match key {
        "model" => s == "aimami_relay" || s.starts_with("aimami_relay_"),
        "profile" | "model_provider" => owned_provider(s),
        "model_catalog_json" => {
            normalize_path(s)
                == normalize_path(
                    &ctx.relay_dir()
                        .join("codex_router_catalog.json")
                        .to_string_lossy(),
                )
        }
        "openai_base_url" => {
            let Ok(u) = url::Url::parse(s.trim()) else {
                return false;
            };
            u.scheme() == "http"
                && u.host_str() == Some("127.0.0.1")
                && u.username().is_empty()
                && u.password().is_none()
                && u.path().trim_end_matches('/') == "/codex/router/v1"
                && u.query().is_none()
                && u.fragment().is_none()
                && u.port().is_some()
        }
        _ => false,
    }
}
fn current_keys(text: &str) -> (HashSet<String>, Vec<String>) {
    let mut keys = HashSet::new();
    let mut sections = Vec::new();
    let mut lex = Lexical(2);
    let mut managed = false;
    let mut section = false;
    for raw in text.lines() {
        let structural = lex.line_is_structural(raw);
        let s = raw.trim();
        if structural && BEGIN.contains(&s) {
            managed = true;
            continue;
        }
        if structural && END.contains(&s) {
            managed = false;
            continue;
        }
        if managed {
            continue;
        }
        if !structural {
            continue;
        }
        if s.starts_with('[') && s.ends_with(']') {
            section = true;
            sections.push(normalize_section_key(&s[1..s.len() - 1]));
            continue;
        }
        if !section {
            if let Some(k) = assignment_key(s) {
                keys.insert(k.to_owned());
            }
        }
    }
    (keys, sections)
}
fn tidy_blank_lines(s: &str) -> String {
    let mut out = String::new();
    let mut blank = 0;
    for line in s.lines() {
        if line.trim().is_empty() {
            if !out.is_empty() {
                blank += 1
            }
        } else {
            for _ in 0..blank {
                out.push('\n')
            }
            blank = 0;
            out.push_str(line);
            out.push('\n')
        }
    }
    out
}
/// Preserves current user keys and overlapping sections; never restores AiMaMi-owned values.
pub fn restore_config_text(
    ctx: &AuthContext,
    current: &str,
    backup: Option<&TakeoverBackup>,
    legacy_lines: &[String],
) -> Result<(String, RestoreReport), AuthError> {
    let (keys, mut sections) = current_keys(current);
    let mut restored_lines: Vec<String> = Vec::new();
    let mut blocks = Vec::new();
    let mut report = RestoreReport::default();
    let mut push_line = |line: &String, report: &mut RestoreReport| {
        if owned_top_level_line(ctx, line) {
            return;
        }
        let Some(key) = assignment_key(line) else {
            return;
        };
        if keys.contains(key)
            || restored_lines
                .iter()
                .any(|s| assignment_key(s) == Some(key))
        {
            report.skipped_top_level_keys.push(key.to_owned())
        } else {
            report.restored_top_level_keys.push(key.to_owned());
            restored_lines.push(line.clone())
        }
    };
    if let Some(b) = backup {
        for line in &b.top_level_lines {
            push_line(line, &mut report)
        }
        for block in &b.section_blocks {
            if owned_model_section(&block.key) {
                continue;
            }
            let mut lex = Lexical(2);
            if block.lines.iter().any(|line| {
                let structural = lex.line_is_structural(line);
                let s = line.trim();
                structural
                    && s.starts_with('[')
                    && s.ends_with(']')
                    && owned_model_section(&s[1..s.len() - 1])
            }) {
                continue;
            }
            if sections.iter().any(|s| section_overlaps(s, &block.key)) {
                report.skipped_sections.push(block.key.clone());
            } else {
                report.restored_sections.push(block.key.clone());
                sections.push(block.key.clone());
                blocks.push(block.clone());
            }
        }
    }
    for line in legacy_lines {
        push_line(line, &mut report)
    }
    let candidate = if restored_lines.is_empty() && blocks.is_empty() {
        current.to_owned()
    } else {
        let mut out = String::new();
        for line in &restored_lines {
            out.push_str(line);
            out.push('\n')
        }
        let section_text = blocks
            .iter()
            .map(|b| b.lines.iter().map(|s| format!("{s}\n")).collect::<String>())
            .collect::<Vec<_>>()
            .join("\n");
        let mut lex = Lexical(2);
        let mut offset = 0;
        let mut insertion = None;
        for raw in current.split_inclusive('\n') {
            let line = raw.strip_suffix('\n').unwrap_or(raw);
            if lex.line_is_structural(line) && line.trim() == BEGIN[0] {
                insertion = Some(offset);
                break;
            }
            offset += raw.len();
        }
        if !out.is_empty() && !current.trim().is_empty() {
            out.push('\n')
        }
        if !section_text.is_empty() {
            if let Some(at) = insertion {
                out.push_str(&current[..at]);
                if !out.is_empty() && !out.ends_with("\n\n") {
                    out.push('\n')
                }
                out.push_str(&section_text);
                out.push('\n');
                out.push_str(&current[at..]);
            } else {
                out.push_str(current);
                if !out.is_empty() {
                    if !out.ends_with('\n') {
                        out.push('\n')
                    }
                    if !out.ends_with("\n\n") {
                        out.push('\n')
                    }
                }
                out.push_str(&section_text);
            }
        } else {
            out.push_str(current)
        }
        tidy_blank_lines(&out)
    };
    if !candidate.trim().is_empty() {
        toml::from_str::<toml::Value>(&candidate)
            .map_err(|_| AuthError::InvalidConfig("restored config.toml is invalid TOML".into()))?;
    }
    Ok((candidate, report))
}

fn read_regular_backup(path: &Path) -> Result<Option<Vec<u8>>, AuthError> {
    let metadata = match fs::symlink_metadata(path) {
        Ok(m) => m,
        Err(e) if e.kind() == io::ErrorKind::NotFound => return Ok(None),
        Err(e) => return Err(ioerr(path, e)),
    };
    if !metadata.is_file() {
        return Err(AuthError::Unsupported(format!(
            "{} is not a regular recovery file; original was preserved",
            path.display()
        )));
    }
    fs::read(path).map(Some).map_err(|e| ioerr(path, e))
}
fn decode_backup<T: for<'de> Deserialize<'de>>(
    path: &Path,
    bytes: &[u8],
) -> Result<Option<T>, AuthError> {
    match serde_json::from_slice::<T>(bytes) {
        Ok(b) => Ok(Some(b)),
        Err(_) => {
            if serde_json::from_slice::<Value>(bytes).is_ok() {
                Err(AuthError::Unsupported(format!(
                    "{} has an unsupported recovery schema; original was preserved",
                    path.display()
                )))
            } else {
                Ok(None)
            }
        }
    }
}
fn append_quarantine(
    plan: &mut TakeoverPlan,
    path: &Path,
    bytes: &[u8],
    stamp: u128,
) -> Result<(), AuthError> {
    let name = path
        .file_name()
        .ok_or_else(|| AuthError::Unsupported("recovery path has no name".into()))?
        .to_string_lossy();
    let parent = path
        .parent()
        .ok_or_else(|| AuthError::Unsupported("recovery path has no parent".into()))?;
    for attempt in 0..100 {
        let q = parent.join(format!("{name}.damaged-{stamp}-{attempt}.bak"));
        if fs::metadata(&q).is_ok() {
            continue;
        }
        plan.files
            .changes
            .push(change(q.clone(), Some(bytes.to_vec()))?);
        plan.files.changes.push(change(path.to_path_buf(), None)?);
        plan.report.quarantined_files.push(q.clone());
        plan.quarantine_pairs.push((path.to_path_buf(), q));
        return Ok(());
    }
    Err(AuthError::Unsupported(format!(
        "{} cannot allocate quarantine path; original was preserved",
        path.display()
    )))
}
/// Recovers both backup formats, including damaged-file quarantine. Routing must be Off.
pub fn plan_takeover_backup_orphan(ctx: &AuthContext) -> Result<TakeoverPlan, AuthError> {
    if ctx.router_mode.enabled() {
        return Err(AuthError::RouterEnabled);
    }
    let current_path = ctx.relay_dir().join("router-takeover-backup.json");
    let legacy_path = ctx.relay_dir().join("codex_router_top_level_backup.json");
    let current_bytes = read_regular_backup(&current_path)?;
    let legacy_bytes = read_regular_backup(&legacy_path)?;
    let backup = match &current_bytes {
        Some(b) => decode_backup::<TakeoverBackup>(&current_path, b)?,
        None => None,
    };
    let legacy = match &legacy_bytes {
        Some(b) => decode_backup::<LegacyBackup>(&legacy_path, b)?,
        None => None,
    };
    if backup.as_ref().is_some_and(|b| b.version != 1)
        || legacy.as_ref().is_some_and(|b| b.version != 1)
    {
        return Err(AuthError::Unsupported(
            "unsupported takeover backup version".into(),
        ));
    }
    let mut plan = TakeoverPlan {
        files: AuthPlan {
            changes: Vec::new(),
            selected_backup: None,
            observations: vec![
                FileObservation {
                    path: current_path.clone(),
                    bytes: current_bytes.clone(),
                },
                FileObservation {
                    path: legacy_path.clone(),
                    bytes: legacy_bytes.clone(),
                },
            ],
        },
        report: RestoreReport::default(),
        quarantine_pairs: Vec::new(),
    };
    let stamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis();
    if backup.is_none() {
        if let Some(bytes) = &current_bytes {
            append_quarantine(&mut plan, &current_path, bytes, stamp)?
        }
    }
    if legacy.is_none() {
        if let Some(bytes) = &legacy_bytes {
            append_quarantine(&mut plan, &legacy_path, bytes, stamp)?
        }
    }
    let legacy_lines = legacy.as_ref().map(|b| b.lines.as_slice()).unwrap_or(&[]);
    // The native early return leaves an empty legacy backup active, so postcheck can fail.
    if backup.is_none() && legacy_lines.is_empty() {
        return Ok(plan);
    }
    let config_path = ctx.codex_home.join("config.toml");
    let before = snapshot(&config_path)?;
    let text = match &before {
        Some(b) => std::str::from_utf8(b)
            .map_err(|_| AuthError::InvalidConfig("config.toml is not UTF-8".into()))?,
        None => "",
    };
    let (candidate, report) = restore_config_text(ctx, text, backup.as_ref(), legacy_lines)?;
    plan.report.restored_top_level_keys = report.restored_top_level_keys;
    plan.report.restored_sections = report.restored_sections;
    plan.report.skipped_top_level_keys = report.skipped_top_level_keys;
    plan.report.skipped_sections = report.skipped_sections;
    plan.files.observations.push(FileObservation {
        path: config_path.clone(),
        bytes: before.clone(),
    });
    if candidate.as_bytes() != text.as_bytes() {
        plan.files.changes.push(FileChange {
            path: config_path,
            before,
            after: Some(candidate.into_bytes()),
        });
    }
    for (path, bytes, usable) in [
        (current_path, current_bytes, backup.is_some()),
        (legacy_path, legacy_bytes, legacy.is_some()),
    ] {
        if usable && bytes.is_some() {
            plan.files.changes.push(FileChange {
                path,
                before: bytes,
                after: None,
            });
        }
    }
    Ok(plan)
}
/// Copy+sync+readback precede deletion of damaged backup files. Candidate config
/// publication (or confirmation of existing bytes) precedes deletion of usable backups.
pub fn apply_takeover_backup_orphan(
    ctx: &AuthContext,
    plan: &TakeoverPlan,
) -> Result<Level, AuthError> {
    if ctx.router_mode.enabled() {
        return Err(AuthError::RouterEnabled);
    }
    verify_plan(&plan.files)?;
    for source in [
        ctx.relay_dir().join("router-takeover-backup.json"),
        ctx.relay_dir().join("codex_router_top_level_backup.json"),
    ] {
        if fs::symlink_metadata(&source).is_ok() {
            read_regular_backup(&source)?;
        }
    }
    for c in &plan.files.changes {
        if let Some(bytes) = &c.after {
            write_private(&c.path, bytes)?;
        } else {
            if let Some((_, q)) = plan
                .quarantine_pairs
                .iter()
                .find(|(source, _)| source == &c.path)
            {
                let copy = snapshot(q)?;
                if copy != c.before {
                    return Err(AuthError::UnsafeBackup(
                        "damaged backup quarantine verification failed".into(),
                    ));
                }
                if snapshot(&c.path)? != c.before {
                    return Err(AuthError::ChangedSincePreview(c.path.clone()));
                }
            } else {
                let config = ctx.codex_home.join("config.toml");
                if fs::metadata(&config).is_ok() {
                    fs::File::open(&config)
                        .and_then(|f| f.sync_all())
                        .map_err(|e| ioerr(&config, e))?;
                    if let Some(parent) = config.parent() {
                        fs::File::open(parent)
                            .and_then(|f| f.sync_all())
                            .map_err(|e| ioerr(parent, e))?;
                    }
                }
            }
            match fs::remove_file(&c.path) {
                Ok(()) => {}
                Err(e) if e.kind() == io::ErrorKind::NotFound => {}
                Err(e) => return Err(ioerr(&c.path, e)),
            }
        }
    }
    Ok(check_takeover_backup_orphan(ctx))
}

/// Native `config_takeover::validate_recovery_backups` @1001fb3c8. This is a
/// read-only transition guard; damaged JSON is recoverable by quarantine, whereas
/// a valid JSON document with an unsupported shape/version blocks the transition.
/// The modern backup is checked before the legacy backup, preserving error order.
pub fn validate_recovery_backups(relay_root: &Path) -> Result<(), AuthError> {
    let current = relay_root.join("router-takeover-backup.json");
    if let Some(bytes) = read_regular_backup(&current)? {
        if decode_backup::<TakeoverBackup>(&current, &bytes)?
            .is_some_and(|backup| backup.version != 1)
        {
            return Err(AuthError::Unsupported(
                "unsupported takeover backup version".into(),
            ));
        }
    }
    let legacy = relay_root.join("codex_router_top_level_backup.json");
    if let Some(bytes) = read_regular_backup(&legacy)? {
        if decode_backup::<LegacyBackup>(&legacy, &bytes)?.is_some_and(|backup| backup.version != 1)
        {
            return Err(AuthError::Unsupported(
                "unsupported takeover backup version".into(),
            ));
        }
    }
    Ok(())
}
