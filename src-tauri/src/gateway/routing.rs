//! Atelier routing decisions and their in-memory request trace.
use super::types::{now_ms, Catalog, Profile, RuntimeView};
use serde::Serialize;

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Candidate {
    pub profile_id: String,
    pub name: String,
    pub eligible: bool,
    pub reason: String,
}

pub fn plan(c: &Catalog, preferred: &Profile, model: &str, pinned: Option<&str>) -> Vec<Candidate> {
    let mut profiles: Vec<_> = c.profiles.iter().collect();
    profiles.sort_by_key(|p| (p.id != preferred.id, p.priority));
    profiles
        .into_iter()
        .map(|p| {
            let excluded = if p.kind != preferred.kind {
                Some("计费类型不同，不跨账号池")
            } else if !p.enabled {
                Some("账号已停用")
            } else if !matches!(p.auth_status.as_str(), "ready" | "unknown")
                || (p.kind == super::types::ProfileKind::Chatgpt && p.auth_status != "ready")
            {
                Some("需要重新授权")
            } else if p.cooldown_until.is_some_and(|t| t > now_ms()) {
                Some("限额冷却中")
            } else if model.is_empty() || !p.models.iter().any(|m| m.id == model) {
                Some("不支持请求模型")
            } else if pinned.is_some_and(|id| id != p.id) {
                Some("上下文已绑定其他账号")
            } else if !c.settings.auto_switch && p.id != preferred.id {
                Some("自动切换已关闭")
            } else {
                None
            };
            Candidate {
                profile_id: p.id.clone(),
                name: p.name.clone(),
                eligible: excluded.is_none(),
                reason: excluded.map(String::from).unwrap_or_else(|| {
                    if pinned.is_some() {
                        "上下文绑定账号".into()
                    } else if p.id == preferred.id {
                        "当前首选账号".into()
                    } else {
                        format!("备用账号 · 优先级 {}", p.priority)
                    }
                }),
            }
        })
        .collect()
}

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Attempt {
    pub profile_id: String,
    pub started_at: u64,
    pub duration_ms: Option<u64>,
    pub http_status: Option<u16>,
    pub code: String,
}

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Route {
    pub id: String,
    pub at: u64,
    pub model: String,
    pub endpoint: String,
    pub status: String,
    pub code: String,
    pub duration_ms: Option<u64>,
    pub candidates: Vec<Candidate>,
    pub attempts: Vec<Attempt>,
}
impl Route {
    pub fn new(endpoint: &str) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            at: now_ms(),
            model: String::new(),
            endpoint: endpoint.into(),
            status: "routing".into(),
            code: String::new(),
            duration_ms: None,
            candidates: vec![],
            attempts: vec![],
        }
    }
    pub fn finish_attempt(&mut self, code: &str) {
        if let Some(a) = self.attempts.last_mut().filter(|a| a.duration_ms.is_none()) {
            a.duration_ms = Some(now_ms().saturating_sub(a.started_at));
            a.code = code.into();
        }
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Snapshot {
    pub runtime: RuntimeView,
    pub rerouted_requests: u64,
    pub routes: Vec<Route>,
}

pub fn native_snapshot(trace: &serde_json::Value, mut runtime: RuntimeView) -> Snapshot {
    let rows = trace["routes"].as_array().map(Vec::as_slice).unwrap_or(&[]);
    runtime.active_requests = rows.iter().filter(|r| r["done"] != true).count();
    runtime.failed_requests = trace["totals"]["errors"].as_u64().unwrap_or(0);
    runtime.completed_requests = trace["totals"]["requests"]
        .as_u64()
        .unwrap_or(0)
        .saturating_sub(runtime.failed_requests);
    let rerouted_requests = trace["totals"]["rerouted"].as_u64().unwrap_or(0);
    let routes = rows
        .iter()
        .rev()
        .map(|r| {
            let at = r["time"]
                .as_str()
                .and_then(|s| {
                    time::OffsetDateTime::parse(s, &time::format_description::well_known::Rfc3339)
                        .ok()
                })
                .map(|t| (t.unix_timestamp_nanos() / 1_000_000) as u64)
                .unwrap_or(0);
            let candidates = r["order"]
                .as_array()
                .into_iter()
                .flatten()
                .map(|c| Candidate {
                    profile_id: c["id"].as_str().unwrap_or("").into(),
                    name: c["who"]
                        .as_str()
                        .or_else(|| c["name"].as_str())
                        .unwrap_or("账号")
                        .into(),
                    eligible: c["rest"].is_null(),
                    reason: format!(
                        "{} · {}",
                        c["routing"]
                            .as_str()
                            .filter(|s| !s.is_empty())
                            .unwrap_or("smart"),
                        if c["known"] == true {
                            format!("配额已用 {}%", c["used"])
                        } else {
                            "配额未知".into()
                        }
                    ),
                })
                .collect();
            let attempts = r["tries"]
                .as_array()
                .into_iter()
                .flatten()
                .map(|a| Attempt {
                    profile_id: a["id"].as_str().unwrap_or("").into(),
                    started_at: at,
                    duration_ms: if a["done"] == true {
                        Some(a["ms"].as_u64().unwrap_or(0))
                    } else {
                        None
                    },
                    http_status: a["status"].as_u64().map(|n| n as u16),
                    code: a["fail"]
                        .as_str()
                        .filter(|s| !s.is_empty())
                        .map(String::from)
                        .unwrap_or_else(|| {
                            if a["done"] == true && a["status"].as_u64().is_some_and(|s| s < 400) {
                                "response.completed".into()
                            } else {
                                String::new()
                            }
                        }),
                })
                .collect();
            let done = r["done"] == true;
            let failed = r["status"].as_u64().is_some_and(|s| s >= 400)
                || r["canceled"] == true
                || r["error"].as_str().is_some_and(|s| !s.is_empty())
                || r["fail"].as_str().is_some_and(|s| !s.is_empty());
            Route {
                id: r["id"].to_string(),
                at,
                model: r["model"].as_str().unwrap_or("").into(),
                endpoint: "native routing core".into(),
                status: if !done {
                    "forwarding"
                } else if failed {
                    "failed"
                } else {
                    "completed"
                }
                .into(),
                code: r["fail"]
                    .as_str()
                    .or_else(|| r["error"].as_str())
                    .unwrap_or(if done && !failed {
                        "response.completed"
                    } else {
                        ""
                    })
                    .into(),
                duration_ms: if done {
                    Some(r["ms"].as_u64().unwrap_or(0))
                } else {
                    None
                },
                candidates,
                attempts,
            }
        })
        .collect();
    Snapshot {
        runtime,
        rerouted_requests,
        routes,
    }
}
