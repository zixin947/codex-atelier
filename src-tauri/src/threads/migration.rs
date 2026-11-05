//! Provider migration preserves every non-metadata byte. Journals allow explicit recovery.
use super::*;
use crate::gateway::storage::{atomic_private, private_dir};
use sha2::{Digest, Sha256};
use std::io::Read;
#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct RolloutChange {
    pub path: PathBuf,
    pub asset: String,
    pub before_hash: String,
    pub after_hash: String,
    pub reverse: bool,
}
fn hash(b: &[u8]) -> String {
    format!("{:x}", Sha256::digest(b))
}
fn bounded(path: &PathBuf) -> Result<Vec<u8>> {
    let mut v = vec![];
    fs::File::open(path)
        .map_err(err)?
        .take(32 * 1024 * 1024 + 1)
        .read_to_end(&mut v)
        .map_err(err)?;
    if v.len() > 32 * 1024 * 1024 {
        return Err("完整迁移仅支持单文件不超过 32 MiB；未修改任何文件".into());
    }
    Ok(v)
}
fn approved(s: &Store, path: &PathBuf) -> Result<PathBuf> {
    if fs::symlink_metadata(path)
        .map_err(err)?
        .file_type()
        .is_symlink()
    {
        return Err("迁移不支持符号链接记录文件".into());
    }
    let p = path.canonicalize().map_err(err)?;
    if !["sessions", "archived_sessions"].iter().any(|name| {
        s.home
            .join(name)
            .canonicalize()
            .is_ok_and(|r| p.starts_with(r))
    }) {
        return Err("迁移路径不在会话目录内".into());
    }
    Ok(p)
}
fn rewrite(bytes: &[u8], id: &str, target: &str) -> Result<Vec<u8>> {
    let mut out = vec![];
    let mut found = false;
    for line in bytes.split_inclusive(|b| *b == b'\n') {
        let v: Value =
            serde_json::from_slice(line).map_err(|_| "记录文件包含不完整或无效 JSON，停止迁移")?;
        if v["type"] == "session_meta" {
            if v["payload"]["id"].as_str() != Some(id) {
                return Err("会话文件 ID 与索引不一致，停止迁移".into());
            }
            let mut v = v;
            let obj = v["payload"].as_object_mut().ok_or("会话元数据格式无效")?;
            obj.insert("model_provider".into(), json!(target));
            out.extend(serde_json::to_vec(&v).map_err(err)?);
            if line.ends_with(b"\r\n") {
                out.extend(b"\r\n")
            } else if line.ends_with(b"\n") {
                out.push(b'\n')
            }
            found = true;
        } else {
            out.extend(line)
        }
    }
    if !found {
        return Err("会话文件缺少 session_meta，停止迁移".into());
    }
    Ok(out)
}
pub(super) fn prepare(s: &Store, rows: &[Thread], target: &str) -> Result<Vec<RolloutChange>> {
    private_dir(&s.backups)?;
    let mut plans = vec![];
    let mut total = 0;
    for row in rows {
        let p = PathBuf::from(&row.rollout_path);
        let p = if p.is_absolute() { p } else { s.home.join(p) };
        let path = approved(s, &p)?;
        let before = bounded(&path)?;
        total += before.len();
        if total > 128 * 1024 * 1024 {
            return Err("本批迁移超过 128 MiB，请缩小选择范围".into());
        }
        let after = rewrite(&before, &row.id, target)?;
        if before == after {
            continue;
        }
        let asset = uuid::Uuid::new_v4().to_string();
        atomic_private(&s.backups.join(format!("{asset}.before")), &before)?;
        atomic_private(&s.backups.join(format!("{asset}.after")), &after)?;
        plans.push(RolloutChange {
            path,
            asset,
            before_hash: hash(&before),
            after_hash: hash(&after),
            reverse: false,
        })
    }
    Ok(plans)
}
pub(super) fn verify(s: &Store, c: &RolloutChange) -> Result<()> {
    let path = approved(s, &c.path)?;
    let actual = hash(&bounded(&path)?);
    if actual != c.before_hash {
        return Err("会话文件在预览后发生变化，请重新检查".into());
    }
    Ok(())
}
pub(super) fn write(s: &Store, c: &RolloutChange, undo: bool) -> Result<()> {
    uuid::Uuid::parse_str(&c.asset).map_err(err)?;
    let reverse = c.reverse ^ undo;
    let data = crate::gateway::storage::read_private(
        &s.backups.join(format!(
            "{}.{}",
            c.asset,
            if reverse { "before" } else { "after" }
        )),
        32 * 1024 * 1024,
    )?;
    let expected = if undo { &c.before_hash } else { &c.after_hash };
    if hash(&data) != *expected {
        return Err("迁移备份校验失败".into());
    }
    let p = approved(s, &c.path)?;
    let current = hash(&bounded(&p)?);
    let expected_current = if undo { &c.after_hash } else { &c.before_hash };
    if current != *expected_current {
        return Err("写入前文件已变化，未覆盖；请使用备份人工核对".into());
    }
    atomic_private(&p, &data)
}
pub(super) fn inverse(s: &Store, old: &[RolloutChange]) -> Result<Vec<RolloutChange>> {
    let mut result = vec![];
    for c in old {
        let path = approved(s, &c.path)?;
        let current = hash(&bounded(&path)?);
        if current == c.before_hash {
            continue;
        }
        if current != c.after_hash {
            return Err("历史文件已被外部修改，拒绝覆盖回滚".into());
        }
        result.push(RolloutChange {
            path: c.path.clone(),
            asset: c.asset.clone(),
            before_hash: c.after_hash.clone(),
            after_hash: c.before_hash.clone(),
            reverse: !c.reverse,
        })
    }
    Ok(result)
}
pub(super) fn require_stopped() -> Result<()> {
    #[cfg(not(test))]
    {
        #[cfg(unix)]
        {
            let result = std::process::Command::new("ps")
                .args(["-A", "-o", "comm="])
                .output()
                .map_err(err)?;
            if !result.status.success() {
                return Err("无法检查 Codex 进程，未执行迁移".into());
            }
            let text = String::from_utf8_lossy(&result.stdout);
            if text.lines().any(|l| {
                matches!(
                    l.trim().rsplit('/').next().unwrap_or(""),
                    "Codex" | "codex" | "ChatGPT"
                )
            }) {
                return Err("完整历史迁移需要先退出 Codex / ChatGPT 及 codex CLI。Atelier 不会替你关闭客户端。".into());
            }
        }
        #[cfg(not(unix))]
        {
            return Err("此平台尚未实现可靠的进程退出检查，完整迁移未执行".into());
        }
    }
    Ok(())
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn preserves_non_metadata_and_rejects_wrong_id() {
        let bytes=b"{\"type\":\"session_meta\",\"payload\":{\"id\":\"a\",\"model_provider\":\"old\"}}\r\n{ \"type\": \"response_item\", \"payload\": {\"encrypted_content\":\"keep\"} }\n";
        let out = rewrite(bytes, "a", "new").unwrap();
        assert!(String::from_utf8_lossy(&out).contains("new"));
        assert!(out.ends_with(
            b"{ \"type\": \"response_item\", \"payload\": {\"encrypted_content\":\"keep\"} }\n"
        ));
        assert!(rewrite(bytes, "wrong", "new").is_err());
    }
}
