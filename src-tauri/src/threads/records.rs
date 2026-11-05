use super::*;
use std::{
    collections::BTreeMap,
    io::{BufRead, BufReader, Read},
};
const MAX_BYTES: u64 = 32 * 1024 * 1024;
const MAX_LINE: u64 = 1024 * 1024;
fn text(v: &Value) -> String {
    if let Some(s) = v.as_str() {
        return s.into();
    }
    v.as_array()
        .map(|a| {
            a.iter()
                .filter_map(|v| v.get("text").and_then(Value::as_str))
                .collect::<Vec<_>>()
                .join("\n")
        })
        .unwrap_or_default()
}
pub(super) fn detail(s: &Store, r: &Thread) -> Result<Value> {
    let p = PathBuf::from(&r.rollout_path);
    let p = if p.is_absolute() { p } else { s.home.join(p) };
    let p = match p.canonicalize() {
        Ok(path) => path,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
            let compressed = PathBuf::from(format!("{}.zst", p.display()));
            compressed
                .canonicalize()
                .map_err(|error| match error.kind() {
                    std::io::ErrorKind::NotFound => {
                        "索引对应的会话文件及 .zst 压缩文件均未找到".to_string()
                    }
                    std::io::ErrorKind::PermissionDenied => "无权访问会话压缩文件".to_string(),
                    _ => format!("解析会话压缩文件路径失败：{error}"),
                })?
        }
        Err(e) => return Err(format!("无法访问会话文件：{e}")),
    };
    let allowed = [s.home.join("sessions"), s.home.join("archived_sessions")]
        .iter()
        .filter_map(|p| p.canonicalize().ok())
        .any(|root| p.starts_with(root));
    if !allowed {
        return Err("会话文件不在 Codex sessions / archived_sessions 目录内".into());
    }
    let f = fs::File::open(&p).map_err(err)?;
    let before = f.metadata().map_err(err)?;
    let size = before.len();
    let compressed = p.extension().is_some_and(|ext| ext == "zst");
    let stream: Box<dyn Read> = if compressed {
        let mut decoder = zstd::stream::read::Decoder::new(f)
            .map_err(|e| format!("无法打开 Zstandard 会话记录：{e}"))?;
        decoder.window_log_max(27).map_err(err)?;
        Box::new(decoder)
    } else {
        Box::new(f)
    };
    // Bound decoded bytes, not compressed bytes: a small archive can expand substantially.
    let mut reader = BufReader::new(stream.take(MAX_BYTES + 1));
    let mut decoded_bytes = 0u64;
    let mut truncated = false;
    let mut usage: Option<Value> = None;
    let mut history = super::usage_history::UsageHistory::default();
    let mut tools = BTreeMap::<String, u64>::new();
    let mut models = std::collections::BTreeSet::new();
    let mut messages = vec![];
    let mut turns = 0;
    let mut warnings = vec![];
    let mut encrypted = false;
    let mut malformed = 0;
    loop {
        let mut line = vec![];
        let n = reader
            .by_ref()
            .take(MAX_LINE + 1)
            .read_until(b'\n', &mut line)
            .map_err(|e| {
                format!(
                    "读取{}会话记录失败（文件可能损坏或不完整）：{e}",
                    if compressed { "压缩" } else { "" }
                )
            })?;
        decoded_bytes += n as u64;
        if decoded_bytes > MAX_BYTES {
            truncated = true;
            break;
        }
        if n == 0 {
            break;
        }
        if n as u64 > MAX_LINE {
            warnings.push("单条记录超过 1 MiB，停止解析；统计不完整".to_string());
            break;
        }
        let v: Value = match serde_json::from_slice(&line) {
            Ok(v) => v,
            Err(_) => {
                malformed += 1;
                continue;
            }
        };
        let payload = &v["payload"];
        let kind = v["type"].as_str().unwrap_or("");
        if payload.get("encrypted_content").is_some() {
            encrypted = true
        }
        if kind == "session_meta"
            && (payload.get("forked_from_id").is_some()
                || payload.get("parent_thread_id").is_some())
        {
            warnings.push("存在父会话引用；当前统计仅覆盖本文件，不递归合并父会话".into())
        }
        if kind == "turn_context" {
            if let Some(m) = payload["model"].as_str() {
                models.insert(m.to_string());
                history.context(m);
            }
        }
        if kind == "event_msg" && payload["type"] == "user_message" {
            turns += 1;
        }
        if kind == "event_msg" && payload["type"] == "token_count" {
            let u = &payload["info"]["total_token_usage"];
            if u.is_object() {
                let num = |k: &str| u[k].as_u64().unwrap_or(0);
                if let Some(total) = u["total_tokens"].as_u64() {
                    history.record(total, v["timestamp"].as_str());
                }
                usage = Some(
                    json!({"input":num("input_tokens"),"cached":num("cached_input_tokens"),"output":num("output_tokens"),"reasoning":num("reasoning_output_tokens"),"total":num("total_tokens")}),
                );
            }
        }
        if kind == "response_item" {
            match payload["type"].as_str().unwrap_or("") {
                "function_call" | "custom_tool_call" => {
                    *tools
                        .entry(payload["name"].as_str().unwrap_or("unknown").into())
                        .or_default() += 1
                }
                "message" => {
                    if messages.len() < 400 {
                        let t = text(&payload["content"]);
                        if !t.is_empty() {
                            if t.chars().count() > 16000 {
                                warnings.push("消息正文超过预览长度，导出包含截断内容".into())
                            }
                            messages.push(json!({"role":payload["role"].as_str().unwrap_or("unknown"),"text":t.chars().take(16000).collect::<String>()}));
                        }
                    }
                }
                _ => {}
            }
        }
    }
    if truncated {
        warnings.push("解码内容超过 32 MiB，本次仅解析前 32 MiB；统计与导出不完整".into())
    }
    if messages.len() >= 400 {
        warnings.push("正文预览最多 400 条，每条最多 16000 字符；导出使用同一预览范围".into())
    }
    if malformed > 0 {
        warnings.push(format!("{malformed} 条记录未能解析（包括尚未写完的末行）"))
    }
    if usage.is_none() {
        warnings.push("没有找到累计 Token 记录，用量保持未知".into())
    }
    if let Ok(after) = fs::metadata(&p) {
        if after.len() != before.len() || after.modified().ok() != before.modified().ok() {
            warnings.push("读取期间文件发生变化，请刷新后重试".into())
        }
    }
    warnings.extend(history.warnings.clone());
    warnings.sort();
    warnings.dedup();
    let complete = warnings.is_empty();
    Ok(
        json!({"id":r.id,"bytes":size,"compressed":compressed,"resolvedPath":p.display().to_string(),"turns":turns,"usage":usage,"tools":tools.into_iter().map(|(name,count)|json!({"name":name,"count":count})).collect::<Vec<_>>(),"models":models,"timeline":history.timeline(),"modelUsage":history.model_usage(),"messages":messages,"warnings":warnings,"encrypted":encrypted,"complete":complete}),
    )
}
