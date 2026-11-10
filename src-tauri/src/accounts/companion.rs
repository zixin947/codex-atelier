//! Sanitized quota handoff to the desktop companion; credentials stay in Accounts.
use super::{
    storage,
    types::{Catalog, CurrentAccount},
};
use crate::gateway::{storage::atomic_private, types::now_ms};
use serde_json::json;
use std::path::{Path, PathBuf};

pub(super) fn path(root: &Path) -> PathBuf {
    root.join("companion-quota.json")
}

pub(super) fn publish(
    root: &Path,
    catalog: &Catalog,
    current: &CurrentAccount,
) -> Result<(), String> {
    let account = if current.state == "matched" {
        catalog
            .accounts
            .iter()
            .find(|a| Some(&a.id) == current.account_id.as_ref())
    } else {
        None
    };
    let fingerprint = match (account, &current.path) {
        (Some(_), Some(file)) => Some(storage::hash(
            &storage::read_external(Path::new(file), 256 * 1024)?
                .ok_or("当前账号文件已变化，请重新检测。")?,
        )),
        _ => None,
    };
    let quota = account.and_then(|a| a.quota.as_ref());
    let now = now_ms();
    let windows: Vec<_> = quota.into_iter().flat_map(|q| [&q.primary, &q.secondary].into_iter().flatten()
        .filter_map(|w| {
            let (used, mins, reset) = (w.used_percent?, w.duration_mins?, w.resets_at?);
            if ![300, 10080].contains(&mins) { return None; }
            Some(json!({"label":if mins == 300 {"5 小时"} else {"周"}, "usedPercent":used,
                "windowDurationMins":mins,"resetsAt":reset,"observedAt":q.checked_at,
                "stale":account.is_some_and(|a| a.status != "ready") || now.saturating_sub(q.checked_at)>900_000 || reset<=now}))
        })).collect();
    let reason = if account.is_none() {
        "请在 Atelier 重新检测当前账号"
    } else if windows.is_empty() {
        "请在 Atelier 刷新当前账号额度"
    } else {
        "来自 Atelier 当前账号的额度查询结果"
    };
    let value = json!({"version":1,"authFingerprint":fingerprint,"publishedAt":now,
        "subscription":{"available":!windows.is_empty(),"windows":windows,
        "observedAt":quota.map(|q|q.checked_at),"reason":reason}});
    atomic_private(
        &path(root),
        &serde_json::to_vec(&value).map_err(|_| "无法编码桌宠额度。")?,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn exports_only_current_quota_and_invalidates_unmatched_account() {
        let root = std::env::temp_dir().join(format!("atelier-companion-{}", uuid::Uuid::new_v4()));
        std::fs::create_dir_all(&root).unwrap();
        let auth = root.join("auth.json");
        std::fs::write(&auth, br#"{"secret":"fixture-credential"}"#).unwrap();
        let now = now_ms();
        let catalog: Catalog = serde_json::from_value(json!({"revision":"fixture","accounts":[{
            "id":"current","name":"private-name","email":"private@example.invalid","plan":"plus",
            "group":"","note":"private-note","status":"ready","error":null,"createdAt":now,
            "quota":{"checkedAt":now,"primary":{"usedPercent":10,"durationMins":300,"resetsAt":now+3600000},
            "secondary":{"usedPercent":20,"durationMins":10080,"resetsAt":now+7200000}}
        }]})).unwrap();
        let mut current = CurrentAccount {
            account_id: Some("current".into()),
            state: "matched".into(),
            path: Some(auth.to_str().unwrap().into()),
            checked_at: Some(now),
            message: String::new(),
        };
        publish(&root, &catalog, &current).unwrap();
        let raw = std::fs::read_to_string(path(&root)).unwrap();
        for private in [
            "fixture-credential",
            "private-name",
            "private@example.invalid",
            "private-note",
        ] {
            assert!(!raw.contains(private));
        }
        let value: serde_json::Value = serde_json::from_str(&raw).unwrap();
        assert_eq!(
            value["subscription"]["windows"].as_array().unwrap().len(),
            2
        );
        current.state = "changed".into();
        publish(&root, &catalog, &current).unwrap();
        let value: serde_json::Value =
            serde_json::from_slice(&std::fs::read(path(&root)).unwrap()).unwrap();
        assert_eq!(value["subscription"]["available"], false);
        assert!(value["authFingerprint"].is_null());
        std::fs::remove_dir_all(root).unwrap();
    }
}
