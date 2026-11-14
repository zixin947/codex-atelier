use serde_json::{json, Value};
use std::{path::PathBuf, process::Stdio};
use tauri::{Emitter, Manager};
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::process::Command;

const PET_STOP_SCRIPT: &str = r#"
import { runningService, serviceRequest } from './runtime/process.mjs';
if (await runningService()) {
  const result = await serviceRequest('/api/stop', { method: 'POST', body: {} });
  if (result.ok !== true) throw new Error('桌宠拒绝了停止请求');
  const deadline = Date.now() + 10000;
  while (await runningService()) {
    if (Date.now() >= deadline) throw new Error('桌宠未在期限内退出');
    await new Promise(resolve => setTimeout(resolve, 250));
  }
}
console.log(JSON.stringify({ ok: true, stopped: true }));
"#;

fn node() -> Result<PathBuf, String> {
    let mut candidates: Vec<PathBuf> = std::env::var_os("PATH")
        .map(|paths| {
            std::env::split_paths(&paths)
                .map(|p| p.join("node"))
                .collect()
        })
        .unwrap_or_default();
    candidates.extend([
        PathBuf::from("/opt/homebrew/bin/node"),
        PathBuf::from("/usr/local/bin/node"),
    ]);
    if let Some(home) = std::env::var_os("HOME") {
        let versions = PathBuf::from(home).join(".nvm/versions/node");
        if versions.is_dir() {
            let mut installed: Vec<_> = std::fs::read_dir(versions)
                .map_err(|_| "无法读取 Node.js 安装目录。")?
                .filter_map(Result::ok)
                .map(|entry| entry.path().join("bin/node"))
                .collect();
            installed.sort();
            candidates.extend(installed.into_iter().rev());
        }
    }
    candidates
        .into_iter()
        .find(|path| path.is_file())
        .ok_or_else(|| "请先安装 Node.js 24+（含 npm）。".into())
}

#[tauri::command]
pub async fn desktop_pet_action(app: tauri::AppHandle, action: String) -> Result<Value, String> {
    if !cfg!(target_os = "macos") {
        return Err("当前桌宠接入仅支持 macOS。".into());
    }
    if ![
        "status",
        "refresh-quota",
        "install",
        "desktop",
        "follow",
        "open",
        "stop",
    ]
    .contains(&action.as_str())
    {
        return Err("不支持的桌宠操作。".into());
    }
    let quota_file = crate::accounts::companion_quota(&app, action == "refresh-quota").await?;
    let action = if action == "refresh-quota" {
        "status".to_owned()
    } else {
        action
    };
    let binary = node()?;
    let version = Command::new(&binary)
        .arg("--version")
        .env_remove("NODE_OPTIONS")
        .output()
        .await
        .map_err(|_| "无法运行 Node.js。")?;
    let version = String::from_utf8_lossy(&version.stdout);
    let major = version
        .trim()
        .trim_start_matches('v')
        .split('.')
        .next()
        .and_then(|s| s.parse::<u32>().ok());
    if !version.starts_with('v') || major.is_none_or(|n| n < 24) {
        return Err("桌宠需要 Node.js 24 或更新版本。".into());
    }
    let bundled = if cfg!(debug_assertions) {
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../vendor/gpt-dragon-girl")
    } else {
        app.path()
            .resource_dir()
            .map_err(|_| "无法取得桌宠资源目录。")?
            .join("gpt-dragon-girl")
    };
    let runtime = app
        .path()
        .app_data_dir()
        .map_err(|_| "无法取得桌宠数据目录。")?
        .join("desktop-pets/gpt-dragon-girl-0.3.0-atelier3");
    if action == "install" && !runtime.exists() {
        let mut pending = vec![(bundled.clone(), runtime.clone())];
        while let Some((source, destination)) = pending.pop() {
            std::fs::create_dir_all(&destination).map_err(|_| "无法创建桌宠运行目录。")?;
            for entry in std::fs::read_dir(source).map_err(|_| "无法读取内置桌宠资源。")?
            {
                let entry = entry.map_err(|_| "无法读取桌宠资源。")?;
                let target = destination.join(entry.file_name());
                if entry.path().is_dir() {
                    pending.push((entry.path(), target));
                } else {
                    std::fs::copy(entry.path(), target).map_err(|_| "无法保存桌宠资源。")?;
                }
            }
        }
    }
    let root = if runtime.exists() { runtime } else { bundled };
    let mut command = Command::new(&binary);
    command
        .env("ATELIER_QUOTA_FILE", &quota_file)
        .env_remove("NODE_OPTIONS")
        .env_remove("NODE_PATH")
        .current_dir(&root)
        .stdin(Stdio::null())
        .kill_on_drop(true);
    if let Some(parent) = binary.parent() {
        let mut paths = vec![
            parent.to_path_buf(),
            PathBuf::from("/usr/bin"),
            PathBuf::from("/bin"),
            PathBuf::from("/opt/homebrew/bin"),
        ];
        if let Some(existing) = std::env::var_os("PATH") {
            paths.extend(std::env::split_paths(&existing));
        }
        command.env(
            "PATH",
            std::env::join_paths(paths).map_err(|_| "Node.js 路径无效。")?,
        );
    }
    if action == "status" {
        command.args(["--input-type=module", "-e", r#"
import { runningService, supervisorStatus } from './runtime/process.mjs';
import { VERSION } from './runtime/paths.mjs';
const service = await runningService();
const supervisor = supervisorStatus();
console.log(JSON.stringify({ version: VERSION, installed: supervisor.installed, running: !!service }));
"#]);
    } else if action == "stop" {
        command.args(["--input-type=module", "-e", PET_STOP_SCRIPT]);
    } else if action == "install" {
        command.arg("scripts/install-macos.mjs");
    } else {
        command.args(["scripts/control.mjs", &action]);
    }
    if action == "install" {
        let mut child = command
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .spawn()
            .map_err(|_| "无法启动桌宠安装器。")?;
        let mut lines = BufReader::new(child.stdout.take().unwrap()).lines();
        let mut stage = "正在准备安装";
        let result = tokio::time::timeout(std::time::Duration::from_secs(600), async {
            while let Some(line) = lines.next_line().await.map_err(|_| "无法读取安装进度。")?
            {
                if let Some(key) = line.strip_prefix("ATELIER_PET_STAGE ") {
                    stage = match key {
                        "dependencies" => "正在安装 npm 依赖（最长 2 分钟）",
                        "download" => "正在下载并校验 Electron（最长 2 分钟）",
                        "extract" => "正在解压 Electron",
                        "sign" => "正在签名桌面组件",
                        "probe" => "正在编译窗口跟随组件",
                        "service" => "正在启动桌宠后台服务",
                        _ => continue,
                    };
                    let _ = app.emit("desktop-pet-install-progress", stage);
                }
            }
            child.wait().await.map_err(|_| "无法读取安装结果。")
        })
        .await
        .map_err(|_| "安装超时，请刷新状态后重试。")??;
        if !result.success() {
            return Err(format!(
                "桌宠安装失败：{stage}。请重试；已有配置与账本已保留。"
            ));
        }
        return Ok(json!({ "ok": true }));
    }
    let timeout = 30;
    let output = tokio::time::timeout(std::time::Duration::from_secs(timeout), command.output())
        .await
        .map_err(|_| "操作超时，请刷新状态后检查桌宠。")?
        .map_err(|_| "无法执行桌宠操作。")?;
    if !output.status.success() {
        // Upstream install logs can contain private backup paths; never return raw output.
        return Err(if action == "stop" {
            "桌宠尚未确认退出，请刷新状态后重试。"
        } else if action == "install" {
            "桌宠安装未完成。请检查网络、npm 和 Xcode Command Line Tools 后重试。"
        } else {
            "桌宠操作失败，请检查是否已安装，并保持 Codex 桌面应用打开。"
        }
        .into());
    }
    if action == "status" {
        serde_json::from_slice(&output.stdout).map_err(|_| "桌宠状态响应无效。".into())
    } else {
        Ok(json!({ "ok": true }))
    }
}
