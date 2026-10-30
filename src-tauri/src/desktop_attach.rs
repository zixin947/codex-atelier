//! Read-only process/socket identity checks for an already running macOS Desktop.
//! No environment, credentials, conversation files or socket payloads are inspected here.
use std::collections::HashMap;

// Match Desktop's client endpoint to a daemon-owned accepted socket bound to the
// exact control path. A daemon process or an unrelated Desktop socket is insufficient.
pub(crate) fn linked(output: &str, desktop: u32, engine: u32, socket: &str) -> bool {
    let mut pid = 0;
    let mut local = None;
    let mut kind = String::new();
    let mut sockets: HashMap<u32, Vec<(u64, &str)>> = HashMap::new();
    for line in output.lines() {
        let Some((field, value)) = line.split_at_checked(1) else {
            continue;
        };
        match field {
            "p" => {
                pid = value.parse().unwrap_or(0);
                local = None;
                kind.clear();
            }
            "f" => {
                local = None;
                kind.clear();
            }
            "t" => kind = value.into(),
            "d" => local = hex(value),
            "n" if kind == "unix" => {
                if let Some(local) = local {
                    sockets.entry(pid).or_default().push((local, value));
                }
            }
            _ => {}
        }
    }
    desktop > 0
        && engine > 0
        && desktop != engine
        && sockets.get(&desktop).is_some_and(|clients| {
            sockets.get(&engine).is_some_and(|servers| {
                clients.iter().any(|(_, peer)| {
                    peer.strip_prefix("->").and_then(hex).is_some_and(|target| {
                        servers
                            .iter()
                            .any(|(local, name)| *local == target && *name == socket)
                    })
                })
            })
        })
}
fn hex(value: &str) -> Option<u64> {
    let value = value.strip_prefix("0x")?;
    let number = u64::from_str_radix(value, 16).ok()?;
    (number != 0).then_some(number)
}

#[cfg(target_os = "macos")]
mod mac {
    use super::linked;
    use std::{
        io::Read,
        path::{Path, PathBuf},
        process::{Command, Stdio},
        sync::mpsc,
        time::{Duration, Instant},
    };

    #[link(name = "proc")]
    unsafe extern "C" {
        fn proc_pidpath(pid: libc::c_int, buffer: *mut libc::c_void, size: u32) -> libc::c_int;
    }
    fn executable(pid: u32) -> Option<PathBuf> {
        let mut bytes = [0u8; 4096];
        let len = unsafe {
            proc_pidpath(
                pid.try_into().ok()?,
                bytes.as_mut_ptr().cast(),
                bytes.len() as u32,
            )
        };
        if len <= 0 {
            return None;
        }
        let end = bytes.iter().position(|b| *b == 0)?;
        Some(PathBuf::from(std::str::from_utf8(&bytes[..end]).ok()?))
    }
    fn desktop_path(path: &Path) -> bool {
        crate::desktop_runtime::applications()
            .into_iter()
            .any(|app| {
                ["ChatGPT", "Codex"]
                    .iter()
                    .any(|name| app.join("Contents/MacOS").join(name) == path)
            })
    }
    fn capture(mut command: Command) -> Result<String, String> {
        let mut child = command
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .spawn()
            .map_err(|_| "无法检查 Desktop 进程连接。")?;
        let stdout = child.stdout.take().ok_or("进程检查输出不可用。")?;
        let (tx, rx) = mpsc::channel();
        std::thread::spawn(move || {
            let mut data = Vec::new();
            let result = stdout
                .take(1024 * 1024 + 1)
                .read_to_end(&mut data)
                .map(|_| data);
            let _ = tx.send(result);
        });
        let deadline = Instant::now() + Duration::from_secs(3);
        loop {
            match child.try_wait() {
                Ok(Some(status)) => {
                    if !status.success() {
                        return Err("无法证明 Desktop 与该宿主的连接，未接管。".into());
                    }
                    let data = rx
                        .recv_timeout(Duration::from_secs(1))
                        .map_err(|_| "进程检查输出超时。")?
                        .map_err(|_| "进程检查读取失败。")?;
                    if data.len() > 1024 * 1024 {
                        return Err("进程检查超过上限，未接管。".into());
                    }
                    return String::from_utf8(data).map_err(|_| "进程检查格式不兼容。".into());
                }
                Ok(None) if Instant::now() < deadline => {
                    std::thread::sleep(Duration::from_millis(20))
                }
                _ => {
                    let _ = child.kill();
                    let _ = child.wait();
                    return Err("进程检查超时，未接管。".into());
                }
            }
        }
    }
    pub fn desktops() -> Result<Vec<u32>, String> {
        let mut command = Command::new("/usr/bin/pgrep");
        command.args([
            "-u",
            &unsafe { libc::getuid() }.to_string(),
            "-x",
            "ChatGPT|Codex",
        ]);
        let output = capture(command)?;
        let pids: Vec<_> = output
            .lines()
            .filter_map(|s| s.parse::<u32>().ok())
            .filter(|pid| executable(*pid).is_some_and(|path| desktop_path(&path)))
            .take(9)
            .collect();
        if pids.is_empty() || pids.len() > 8 {
            return Err("未找到可确认身份的已运行 Desktop。".into());
        }
        Ok(pids)
    }
    pub fn verify(desktop: u32, engine: u32, home: &Path) -> Result<(), String> {
        if !executable(desktop).is_some_and(|p| desktop_path(&p)) {
            return Err("Desktop 进程已变化，连接失效。".into());
        }
        let engine_path = executable(engine).ok_or("无法确认 Desktop 引擎身份。")?;
        let canonical_home = home.canonicalize().map_err(|_| "Codex 目录不可用。")?;
        let managed = canonical_home.join("packages/app-server-daemon");
        let legacy = canonical_home.join("packages/standalone");
        if !crate::desktop_runtime::owns_binary(&engine_path)
            && !(engine_path.file_name().is_some_and(|p| p == "codex")
                && (engine_path.starts_with(managed) || engine_path.starts_with(legacy)))
        {
            return Err("控制入口不属于受支持的 Codex 引擎，未接管。".into());
        }
        let mut command = Command::new("/usr/sbin/lsof");
        command.args([
            "-n",
            "-a",
            "-p",
            &format!("{desktop},{engine}"),
            "-U",
            "-F",
            "pftdn",
        ]);
        let socket = home
            .join("app-server-control/app-server-control.sock")
            .canonicalize()
            .map_err(|_| "控制入口已变化。")?;
        if !linked(
            &capture(command)?,
            desktop,
            engine,
            socket.to_str().ok_or("控制入口路径不兼容。")?,
        ) {
            return Err(
                "未证明当前 Desktop 正在使用该共享宿主。独占 stdio 宿主不能无重启接管。".into(),
            );
        }
        Ok(())
    }
}
#[cfg(target_os = "macos")]
pub(crate) use mac::{desktops, verify};

#[cfg(test)]
mod tests {
    use super::*;
    const PAIR: &str = "p10\nf21\ntunix\nd0xabcdef11\nn->0xabcdef22\np20\nf8\ntunix\nd0xabcdef22\nn/fixture/control.sock\n";
    #[test]
    fn requires_a_desktop_client_connected_to_the_exact_control_listener() {
        assert!(linked(PAIR, 10, 20, "/fixture/control.sock"));
        assert!(!linked(PAIR, 10, 10, "/fixture/control.sock"));
        assert!(!linked(PAIR, 11, 20, "/fixture/control.sock"));
        assert!(!linked(PAIR, 10, 20, "/other/control.sock"));
        assert!(!linked(
            &PAIR.replace("d0xabcdef22", "d0xffffffff"),
            10,
            20,
            "/fixture/control.sock"
        ));
    }
    #[test]
    fn rejects_unconnected_paths_zero_and_other_fd_types() {
        assert!(!linked(
            &PAIR.replace("tunix", "tREG"),
            10,
            20,
            "/fixture/control.sock"
        ));
        assert!(!linked(
            &PAIR.replace("n->0xabcdef22", "n/fixture/control.sock"),
            10,
            20,
            "/fixture/control.sock"
        ));
        assert!(!linked(
            &PAIR
                .replace("0xabcdef11", "0x0")
                .replace("0xabcdef22", "0x0"),
            10,
            20,
            "/fixture/control.sock"
        ));
    }
    #[test]
    fn resets_endpoint_identity_between_file_descriptors() {
        assert!(!linked(
            &PAIR.replace("n->0xabcdef22", "f22\ntunix\nn->0xabcdef22"),
            10,
            20,
            "/fixture/control.sock"
        ));
    }
}
