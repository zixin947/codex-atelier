//! Desktop identity follows Cockpit's macOS main-process and profile matching.
use std::{path::PathBuf, process::Command};
fn normalize(value: &str) -> String {
    let p = PathBuf::from(value.trim());
    p.canonicalize()
        .unwrap_or(p)
        .to_string_lossy()
        .trim_end_matches('/')
        .to_owned()
}
fn desktop(exe: &str) -> bool {
    exe.ends_with("/Codex.app/Contents/MacOS/Codex")
        || exe.ends_with("/ChatGPT.app/Contents/MacOS/ChatGPT")
}
fn alive(pid: u32) -> bool {
    Command::new("ps")
        .args(["-p", &pid.to_string(), "-o", "stat="])
        .output()
        .map(|o| {
            o.status.success() && {
                let s = String::from_utf8_lossy(&o.stdout);
                !s.trim().is_empty() && !s.trim().starts_with('Z')
            }
        })
        .unwrap_or(false)
}
fn profile_matches(target: Option<&str>, home: Option<&str>) -> bool {
    let default = std::env::var_os("HOME")
        .map(PathBuf::from)
        .map(|p| p.join(".codex"));
    let default = default.as_ref().map(|p| normalize(&p.to_string_lossy()));
    let target = target
        .map(normalize)
        .filter(|s| !s.is_empty())
        .or_else(|| default.clone());
    let home = home.map(normalize).filter(|s| !s.is_empty()).or(default);
    target == home
}
// ps renders environment values without quoting. Preserve spaces up to the next KEY=.
fn codex_home(line: &str) -> Option<String> {
    let words: Vec<_> = line.split_whitespace().collect();
    let start = words.iter().position(|s| s.starts_with("CODEX_HOME="))?;
    let mut value = words[start].strip_prefix("CODEX_HOME=")?.to_owned();
    for word in &words[start + 1..] {
        if word.split_once('=').is_some_and(|(k, _)| {
            !k.is_empty() && k.chars().all(|c| c.is_ascii_alphanumeric() || c == '_')
        }) {
            break;
        }
        value.push(' ');
        value.push_str(word);
    }
    Some(value)
}
pub fn collect_codex_process_entries() -> Vec<(u32, Option<String>)> {
    collect_codex_launch_maintenance_process_entries().unwrap_or_else(|_| vec![(u32::MAX, None)])
}
pub fn collect_codex_launch_maintenance_process_entries(
) -> Result<Vec<(u32, Option<String>)>, String> {
    #[cfg(test)]
    {
        return Ok(vec![]);
    }
    #[cfg(not(test))]
    {
        let output = Command::new("ps")
            .args(["-A", "-o", "pid=,comm="])
            .output()
            .map_err(|e| e.to_string())?;
        if !output.status.success() {
            return Err("无法核对 Desktop 进程".into());
        }
        let mut entries = Vec::new();
        for line in String::from_utf8_lossy(&output.stdout).lines() {
            let Some((pid, exe)) = line.trim().split_once(char::is_whitespace) else {
                continue;
            };
            let Ok(pid) = pid.parse::<u32>() else {
                continue;
            };
            if !desktop(exe.trim()) || !alive(pid) {
                continue;
            }
            let env = Command::new("ps")
                .args(["-Eww", "-p", &pid.to_string(), "-o", "command="])
                .output()
                .map_err(|e| e.to_string())?;
            if !env.status.success() {
                if alive(pid) {
                    return Err(format!("无法识别 Desktop PID {pid} 的实例目录"));
                }
                continue;
            }
            entries.push((pid, codex_home(&String::from_utf8_lossy(&env.stdout))));
        }
        Ok(entries)
    }
}
pub fn resolve_codex_pid_from_entries(
    last: Option<u32>,
    home: Option<&str>,
    entries: &[(u32, Option<String>)],
) -> Option<u32> {
    if entries.iter().any(|e| e.0 == u32::MAX) {
        return Some(u32::MAX);
    }
    let matching: Vec<_> = entries
        .iter()
        .filter(|(pid, h)| profile_matches(home, h.as_deref()) && alive(*pid))
        .map(|e| e.0)
        .collect();
    last.filter(|id| matching.contains(id))
        .or_else(|| matching.first().copied())
}
pub fn detect_codex_exec_path() -> Option<PathBuf> {
    crate::host().ok().and_then(|h| h.binary)
}
pub fn apply_managed_proxy_env_to_command(_: &mut Command) {}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn unrelated_instance_does_not_block() {
        let pid = std::process::id();
        assert_eq!(
            resolve_codex_pid_from_entries(
                Some(pid),
                Some("/tmp/target"),
                &[(pid, Some("/tmp/other".into()))]
            ),
            None
        );
        assert_eq!(
            resolve_codex_pid_from_entries(
                None,
                Some("/tmp/target"),
                &[(pid, Some("/tmp/target".into()))]
            ),
            Some(pid)
        );
    }
    #[test]
    fn exited_process_does_not_block() {
        let mut child = Command::new("/usr/bin/true").spawn().unwrap();
        let pid = child.id();
        child.wait().unwrap();
        assert_eq!(
            resolve_codex_pid_from_entries(
                Some(pid),
                Some("/tmp/target"),
                &[(pid, Some("/tmp/target".into()))]
            ),
            None
        );
    }
    #[test]
    fn background_engine_is_not_desktop() {
        assert!(desktop("/Applications/ChatGPT.app/Contents/MacOS/ChatGPT"));
        assert!(!desktop("/Applications/ChatGPT.app/Contents/Resources/codex-cli/CodexCLI.app/Contents/MacOS/codex"));
    }
    #[test]
    fn profile_paths_preserve_spaces_and_default() {
        assert_eq!(
            codex_home("app CODEX_HOME=/tmp/my profile NEXT=value"),
            Some("/tmp/my profile".into())
        );
        assert!(profile_matches(None, None));
        assert!(!profile_matches(Some("/tmp/isolated"), None));
    }
}
