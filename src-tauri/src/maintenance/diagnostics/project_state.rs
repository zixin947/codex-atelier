//! Rebuild missing project navigation from actual thread working directories.
//! Preserve user-defined fields, roots and projectless exclusions.
use super::*;
use serde_json::{json, Value};
use std::collections::{BTreeMap, BTreeSet};

pub(super) struct ProjectPlan {
    pub issues: usize,
    pub before: Vec<u8>,
    pub after: Option<Vec<u8>>,
    pub detail: String,
}
#[derive(Clone, Serialize)]
pub(super) struct CwdUpdate {
    pub id: String,
    pub before: String,
    pub after: String,
    pub path: String,
}
fn generated_projectless(raw: &str) -> bool {
    let parts = raw
        .split(['/', '\\'])
        .filter(|p| !p.is_empty())
        .collect::<Vec<_>>();
    parts.windows(3).any(|p| {
        let date = p[2].as_bytes();
        p[0].eq_ignore_ascii_case("documents")
            && p[1].eq_ignore_ascii_case("codex")
            && date.len() == 10
            && date[4] == b'-'
            && date[7] == b'-'
            && date
                .iter()
                .enumerate()
                .all(|(i, b)| matches!(i, 4 | 7) || b.is_ascii_digit())
    })
}
fn real_root(raw: &str) -> Option<String> {
    let raw = raw.trim();
    if raw.is_empty() || raw.chars().any(char::is_control) || generated_projectless(raw) {
        return None;
    }
    let win = raw.as_bytes().get(1) == Some(&b':')
        && raw
            .as_bytes()
            .get(2)
            .is_some_and(|b| matches!(b, b'/' | b'\\'));
    if !Path::new(raw).is_absolute() && !win {
        return None;
    }
    Some(raw.trim_end_matches(['/', '\\']).to_string()).filter(|s| !s.is_empty())
}
fn strings(v: Option<&Value>) -> Vec<String> {
    v.and_then(Value::as_array)
        .into_iter()
        .flatten()
        .filter_map(Value::as_str)
        .map(str::to_owned)
        .collect()
}
fn rebuild(before: &[u8], rows: &[(String, String)]) -> Result<ProjectPlan> {
    let mut state: Value =
        serde_json::from_slice(before).map_err(|_| "项目状态 JSON 无法解析，未丢弃未知配置")?;
    let map = state.as_object_mut().ok_or("项目状态根节点不是对象")?;
    let excluded = strings(map.get("projectless-thread-ids"))
        .into_iter()
        .collect::<BTreeSet<_>>();
    let mut roots = Vec::<String>::new();
    let mut hints = BTreeMap::<String, String>::new();
    for (id, cwd) in rows {
        if excluded.contains(id) {
            continue;
        }
        if let Some(root) = real_root(cwd) {
            if !roots.contains(&root) {
                roots.push(root.clone());
            }
            hints.insert(id.clone(), root);
        }
    }
    let mut issues = 0;
    if !roots.is_empty() {
        for key in ["electron-saved-workspace-roots", "project-order"] {
            if strings(map.get(key)).is_empty() {
                issues += 1;
                map.insert(key.into(), json!(roots));
            }
        }
        if map
            .get("thread-workspace-root-hints")
            .and_then(Value::as_object)
            .is_none_or(|m| m.is_empty())
        {
            issues += 1;
            map.insert("thread-workspace-root-hints".into(), json!(hints));
        }
        if issues > 0 && strings(map.get("active-workspace-roots")).is_empty() {
            map.insert("active-workspace-roots".into(), json!([roots[0]]));
        }
    }
    let after = if issues > 0 {
        Some(serde_json::to_vec_pretty(&state).map_err(err)?)
    } else {
        None
    };
    Ok(ProjectPlan {
        issues,
        before: before.to_vec(),
        after,
        detail: if roots.is_empty() {
            "线程库中没有可用于重建的真实项目 cwd；保留当前项目状态，不写入虚构路径".into()
        } else {
            format!("核对 {} 个真实项目与 {} 条线程映射；{issues} 项项目入口或映射为空。保留已有项目、无项目排除和未知字段。", roots.len(), hints.len())
        },
    })
}
pub(super) fn inspect_existing(ctx: &Context, db: &Path) -> Result<ProjectPlan> {
    let before = read_bounded(&ctx.home.join(".codex-global-state.json"), 8 * 1024 * 1024)?;
    let mut rows = super::snapshot::with_connection(db, |conn| {
        let mut st = conn.prepare("PRAGMA table_info(threads)").map_err(err)?;
        let cols = st
            .query_map([], |r| r.get::<_, String>(1))
            .map_err(err)?
            .collect::<std::result::Result<BTreeSet<_>, _>>()
            .map_err(err)?;
        if !cols.contains("cwd") {
            return Err("此索引没有 cwd 字段，无法核对项目入口".into());
        }
        let archived = if cols.contains("archived") {
            "COALESCE(archived,0)=0"
        } else {
            "1=1"
        };
        let source = if cols.contains("source") {
            "AND COALESCE(source,'') NOT LIKE '%subagent%'"
        } else {
            ""
        };
        let order = if cols.contains("updated_at_ms") {
            "updated_at_ms DESC"
        } else if cols.contains("updated_at") {
            "updated_at DESC"
        } else {
            "id"
        };
        let mut st = conn.prepare(&format!("SELECT id,COALESCE(cwd,'') FROM threads WHERE {archived} {source} ORDER BY {order} LIMIT 1001")).map_err(err)?;
        let rows = st
            .query_map([], |r| Ok((r.get::<_, String>(0)?, r.get::<_, String>(1)?)))
            .map_err(err)?
            .collect::<std::result::Result<Vec<_>, _>>()
            .map_err(err)?;
        if rows.len() > 1000 {
            return Err("项目索引超过本次检查上限，未自动重建".into());
        }
        Ok(rows)
    })?;
    let updates = cwd_repairs(ctx, db)?;
    for update in &updates {
        if let Some(row) = rows.iter_mut().find(|(id, _)| id == &update.id) {
            row.1 = update.after.clone();
        }
    }
    let mut plan = rebuild(&before, &rows)?;
    plan.issues += updates.len();
    if !updates.is_empty() {
        plan.detail.push_str(&format!(
            "另有 {} 条空 cwd 可从身份匹配的会话元数据回填。",
            updates.len()
        ));
    }
    Ok(plan)
}

pub(super) fn cwd_repairs(ctx: &Context, db: &Path) -> Result<Vec<CwdUpdate>> {
    let project = read_bounded(&ctx.home.join(".codex-global-state.json"), 8 * 1024 * 1024)?;
    let state: Value = serde_json::from_slice(&project).map_err(|_| "项目状态无法解析")?;
    let excluded = strings(state.get("projectless-thread-ids"))
        .into_iter()
        .collect::<BTreeSet<_>>();
    let candidates = super::snapshot::with_connection(db, |conn| {
        let mut st = conn.prepare("PRAGMA table_info(threads)").map_err(err)?;
        let cols = st
            .query_map([], |r| r.get::<_, String>(1))
            .map_err(err)?
            .collect::<std::result::Result<BTreeSet<_>, _>>()
            .map_err(err)?;
        if !cols.contains("cwd") || !cols.contains("rollout_path") {
            return Ok(vec![]);
        }
        let archived = if cols.contains("archived") {
            "AND COALESCE(archived,0)=0"
        } else {
            ""
        };
        let source = if cols.contains("source") {
            "AND COALESCE(source,'') NOT LIKE '%subagent%'"
        } else {
            ""
        };
        let mut st=conn.prepare(&format!("SELECT id,COALESCE(cwd,''),rollout_path FROM threads WHERE COALESCE(cwd,'')='' {archived} {source} LIMIT 1001")).map_err(err)?;
        let rows = st
            .query_map([], |r| {
                Ok((
                    r.get::<_, String>(0)?,
                    r.get::<_, String>(1)?,
                    r.get::<_, String>(2)?,
                ))
            })
            .map_err(err)?
            .collect::<std::result::Result<Vec<_>, _>>()
            .map_err(err)?;
        if rows.len() > 1000 {
            return Err("待回填项目超过检查上限".into());
        }
        Ok(rows)
    })?;
    let mut updates = vec![];
    let mut total = 0;
    for (id, before, path) in candidates {
        if excluded.contains(&id) {
            continue;
        }
        let Ok(resolved) = super::inspect::resolve(ctx, &path) else {
            continue;
        };
        let Ok(bytes) = super::inspect::rollout(&resolved) else {
            continue;
        };
        total += bytes.len();
        if total > 128 * 1024 * 1024 {
            return Err("项目回填记录超过128MiB检查上限".into());
        }
        let Ok(meta) = super::inspect::meta(&bytes) else {
            continue;
        };
        if meta["id"].as_str() != Some(id.as_str()) {
            continue;
        }
        if let Some(after) = meta["cwd"].as_str().and_then(real_root) {
            updates.push(CwdUpdate {
                id,
                before,
                after,
                path,
            });
        }
    }
    Ok(updates)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn fills_empty_navigation_from_real_cwd_and_preserves_unknown_fields() {
        let raw=br#"{"theme":"keep","projectless-thread-ids":["excluded"],"electron-saved-workspace-roots":[],"thread-workspace-root-hints":{}}"#;
        let p = rebuild(
            raw,
            &[
                ("a".into(), "/projects/alpha".into()),
                ("b".into(), "/projects/alpha".into()),
                ("excluded".into(), "/secret/excluded".into()),
                (
                    "scratch".into(),
                    "/Users/example/Documents/Codex/2026-11-07".into(),
                ),
            ],
        )
        .unwrap();
        assert_eq!(p.issues, 3);
        let after: Value = serde_json::from_slice(p.after.as_ref().unwrap()).unwrap();
        assert_eq!(after["theme"], "keep");
        assert_eq!(
            after["electron-saved-workspace-roots"],
            json!(["/projects/alpha"])
        );
        assert_eq!(
            after["thread-workspace-root-hints"]
                .as_object()
                .unwrap()
                .len(),
            2
        );
        assert_eq!(
            rebuild(
                p.after.as_ref().unwrap(),
                &[("a".into(), "/projects/alpha".into())]
            )
            .unwrap()
            .issues,
            0
        );
    }
    #[test]
    fn existing_navigation_and_non_projects_are_not_overwritten() {
        let raw=br#"{"electron-saved-workspace-roots":["/pinned"],"project-order":["/pinned"],"thread-workspace-root-hints":{"old":"/pinned"}}"#;
        assert!(rebuild(raw, &[("a".into(), "/another".into())])
            .unwrap()
            .after
            .is_none());
        assert!(rebuild(b"{}", &[("a".into(), "relative".into())])
            .unwrap()
            .after
            .is_none());
        assert!(rebuild(b"broken", &[]).is_err());
    }
    #[test]
    fn generated_projectless_paths_require_the_document_date_layout() {
        assert!(generated_projectless(
            "C:\\Users\\demo\\Documents\\Codex\\2026-11-07"
        ));
        assert!(!generated_projectless("/projects/Codex/2026-11-07"));
        assert!(!generated_projectless(
            "/Users/demo/Documents/Codex/Atelier"
        ));
    }
}
