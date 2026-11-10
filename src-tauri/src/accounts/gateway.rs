//! Exposes managed Codex accounts to the gateway without duplicating credentials.
use super::{storage, Accounts, Rpc};
use crate::gateway::types::{now_ms, GatewayModel, Profile, ProfileKind};
use serde_json::{json, Value};

impl Accounts {
    pub(crate) fn gateway_profiles(&self) -> Result<Vec<Profile>, String> {
        self.store
            .read()?
            .accounts
            .into_iter()
            .map(|a| {
                let ready = self
                    .store
                    .auth(&a.id)
                    .and_then(|b| {
                        let v: Value =
                            serde_json::from_slice(&b).map_err(|_| "账号凭据格式错误。")?;
                        storage::normalize(&v)
                    })
                    .is_ok();
                let models = storage::read_external(
                    &self.store.home(&a.id)?.join("models_cache.json"),
                    8 * 1024 * 1024,
                )?
                .and_then(|b| serde_json::from_slice::<Value>(&b).ok())
                .map(|v| cached_models(&v))
                .unwrap_or_default();
                Ok(Profile {
                    id: a.id,
                    kind: ProfileKind::Codex,
                    name: a.name,
                    email: a.email,
                    enabled: true,
                    priority: 100,
                    base_url: "https://chatgpt.com/backend-api/codex".into(),
                    auth_status: if ready { "ready" } else { "needsLogin" }.into(),
                    models,
                    cooldown_until: None,
                    last_error: None,
                    requests: 0,
                    input_tokens: 0,
                    output_tokens: 0,
                    created_at: a.created_at,
                })
            })
            .collect()
    }

    pub(crate) async fn gateway_credentials(&self, id: &str, force: bool) -> Result<Value, String> {
        let _lock = self.store.lock(id)?;
        let account = self
            .store
            .read()?
            .accounts
            .into_iter()
            .find(|a| a.id == id)
            .ok_or("账号已移除。")?;
        let read = || -> Result<Value, String> {
            let v: Value =
                serde_json::from_slice(&self.store.auth(id)?).map_err(|_| "账号凭据格式错误。")?;
            storage::normalize(&v)
        };
        let mut auth = read()?;
        let expires = storage::decode_hint(auth["tokens"]["access_token"].as_str().unwrap())["exp"]
            .as_u64()
            .unwrap_or(0);
        if force || expires <= now_ms() / 1000 + 60 {
            let mut rpc = Rpc::start(&self.store.prepare(id)?).await?;
            let result = rpc.call("account/read", json!({"refreshToken":true})).await;
            rpc.close().await;
            if result?["account"]["type"] != "chatgpt" {
                return Err("请重新登录此 Codex 账号。".into());
            }
            auth = read()?;
        }
        if storage::hints(&auth).0 != account.identity {
            return Err("账号身份已变化，请重新授权。".into());
        }
        Ok(json!({"token":auth["tokens"]["access_token"],"accountID":auth["tokens"]["account_id"]}))
    }

    pub(crate) async fn gateway_models(&self, id: &str) -> Result<Vec<GatewayModel>, String> {
        let _lock = self.store.lock(id)?;
        if !self.store.read()?.accounts.iter().any(|a| a.id == id) {
            return Err("账号已移除。".into());
        }
        let mut rpc = Rpc::start(&self.store.prepare(id)?).await?;
        let result = rpc.call("model/list", json!({"includeHidden":false})).await;
        rpc.close().await;
        let v = result?;
        let rows = v["data"].as_array().ok_or("Codex 模型目录格式不兼容。")?;
        Ok(rows
            .iter()
            .filter_map(|m| {
                let id = m["model"].as_str()?;
                Some(GatewayModel {
                    id: id.into(),
                    name: m["displayName"].as_str().unwrap_or(id).into(),
                })
            })
            .take(100)
            .collect())
    }
}

fn cached_models(v: &Value) -> Vec<GatewayModel> {
    v["models"]
        .as_array()
        .into_iter()
        .flatten()
        .filter(|m| m["visibility"] == "list")
        .filter_map(|m| {
            Some(GatewayModel {
                id: m["slug"].as_str()?.into(),
                name: m["display_name"].as_str().or(m["slug"].as_str())?.into(),
            })
        })
        .take(100)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::super::{background, network, Store};
    use super::*;
    use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};
    use std::{
        collections::HashMap,
        sync::{Arc, Mutex},
    };

    #[tokio::test]
    async fn managed_login_is_selectable_without_copying_credentials_and_tracks_removal() {
        let root =
            std::env::temp_dir().join(format!("atelier-account-bridge-{}", uuid::Uuid::new_v4()));
        let store = Store::new(root.join("accounts")).unwrap();
        let id = uuid::Uuid::new_v4().to_string();
        let token = format!(
            "fixture.{}.signature",
            URL_SAFE_NO_PAD.encode(br#"{"sub":"fixture","exp":4102444800}"#)
        );
        store.add(&id, "Logged in account", &json!({"tokens":{"access_token":token,"id_token":token,"refresh_token":"fixture-refresh","account_id":"fixture-account"}})).unwrap();
        let accounts = Accounts {
            background: background::Controller::open(store.root.join("auto-refresh.json")),
            network: network::Network::new(&store.root),
            store,
            attempts: Mutex::new(HashMap::new()),
            tickets: Mutex::new(HashMap::new()),
            login_gate: tokio::sync::Mutex::new(()),
        };
        let gateway = crate::gateway::storage::Store::new(root.join("gateway")).unwrap();
        gateway
            .synchronize_accounts(accounts.gateway_profiles().unwrap())
            .unwrap();
        let catalog = gateway.read().unwrap();
        assert_eq!(catalog.profiles[0].kind, ProfileKind::Codex);
        assert_eq!(catalog.profiles[0].auth_status, "ready");
        assert!(gateway.credential(&id).is_err());
        assert_eq!(
            accounts.gateway_credentials(&id, false).await.unwrap()["token"],
            token
        );
        gateway
            .update(None, |c| {
                c.settings.preferred_profile_id = Some(id.clone());
                c.profiles[0].priority = 7;
                Ok(())
            })
            .unwrap();
        accounts
            .store
            .update(None, |c| {
                c.accounts[0].name = "Renamed".into();
                Ok(())
            })
            .unwrap();
        gateway
            .synchronize_accounts(accounts.gateway_profiles().unwrap())
            .unwrap();
        let catalog = gateway.read().unwrap();
        assert_eq!(catalog.profiles[0].name, "Renamed");
        assert_eq!(catalog.profiles[0].priority, 7);
        assert_eq!(
            catalog.settings.preferred_profile_id.as_deref(),
            Some(id.as_str())
        );
        accounts
            .store
            .update(None, |c| {
                c.accounts.clear();
                Ok(())
            })
            .unwrap();
        gateway
            .synchronize_accounts(accounts.gateway_profiles().unwrap())
            .unwrap();
        assert!(gateway.read().unwrap().profiles.is_empty());
        assert!(gateway
            .read()
            .unwrap()
            .settings
            .preferred_profile_id
            .is_none());
        assert!(accounts.gateway_credentials(&id, false).await.is_err());
        std::fs::remove_dir_all(root).unwrap();
    }
}
