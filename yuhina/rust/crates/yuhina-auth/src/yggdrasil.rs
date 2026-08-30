//! Yggdrasil-compatible accounts (authlib-injector protocol).
//!
//! Built-in preset: LittleSkin `https://littleskin.cn/api/yggdrasil`.
//! Custom servers are supported by passing any authlib-injector base URL.
//! Multiple accounts are isolated by per-account `clientToken`
//! (04-agent-auth.md T4).

use yuhina_api::{Account, AccountKind, Result, YuhinaError};
use base64::Engine;

pub const LITTLESKIN_URL: &str = "https://littleskin.cn/api/yggdrasil";

#[derive(Debug, Clone)]
pub struct YggdrasilClient {
    server_url: String,
    http: reqwest::Client,
}

impl YggdrasilClient {
    pub fn new(server_url: impl Into<String>, http: reqwest::Client) -> Self {
        let server_url = normalize_server_url(server_url.into());
        YggdrasilClient { server_url, http }
    }

    pub fn server_url(&self) -> &str {
        &self.server_url
    }

    /// `POST {server}/authserver/authenticate`.
    pub async fn authenticate(
        &self,
        username: &str,
        password: &str,
        client_token: &str,
    ) -> Result<YggdrasilSession> {
        let body = serde_json::json!({
            "agent": { "name": "Minecraft", "version": 1 },
            "username": username,
            "password": password,
            "clientToken": client_token,
            "requestUser": true,
        });
        let resp = self
            .http
            .post(format!("{}/authserver/authenticate", self.server_url))
            .json(&body)
            .send()
            .await
            .map_err(|e| YuhinaError::network(e.to_string()))?;
        let status = resp.status();
        let text = resp
            .text()
            .await
            .map_err(|e| YuhinaError::network(e.to_string()))?;
        if !status.is_success() {
            return Err(map_yggdrasil_error(status.as_u16(), &text, "authenticate"));
        }
        let parsed: AuthenticateResponse = serde_json::from_str(&text)
            .map_err(|e| YuhinaError::auth(format!("bad authenticate response: {e}")))?;
        let profile = parsed
            .selected_profile
            .or_else(|| parsed.available_profiles.into_iter().next())
            .ok_or_else(|| YuhinaError::auth("no Minecraft profile on this account."))?;
        let access_token = parsed
            .access_token
            .ok_or_else(|| YuhinaError::auth("authenticate returned no access token."))?;
        Ok(YggdrasilSession {
            access_token,
            client_token: parsed.client_token.unwrap_or_else(|| client_token.to_string()),
            profile,
        })
    }

    /// `POST {server}/authserver/refresh` with the previous session.
    pub async fn refresh(
        &self,
        access_token: &str,
        client_token: &str,
    ) -> Result<YggdrasilSession> {
        let body = serde_json::json!({
            "accessToken": access_token,
            "clientToken": client_token,
            "requestUser": true,
        });
        let resp = self
            .http
            .post(format!("{}/authserver/refresh", self.server_url))
            .json(&body)
            .send()
            .await
            .map_err(|e| YuhinaError::network(e.to_string()))?;
        let status = resp.status();
        let text = resp
            .text()
            .await
            .map_err(|e| YuhinaError::network(e.to_string()))?;
        if !status.is_success() {
            return Err(map_yggdrasil_error(status.as_u16(), &text, "refresh"));
        }
        let parsed: AuthenticateResponse = serde_json::from_str(&text)
            .map_err(|e| YuhinaError::auth(format!("bad refresh response: {e}")))?;
        let access_token = parsed
            .access_token
            .ok_or_else(|| YuhinaError::auth("refresh returned no access token."))?;
        let profile = parsed
            .selected_profile
            .ok_or_else(|| YuhinaError::auth("refresh returned no profile."))?;
        Ok(YggdrasilSession {
            access_token,
            client_token: parsed.client_token.unwrap_or_else(|| client_token.to_string()),
            profile,
        })
    }

    /// `GET {server}/sessionserver/session/minecraft/profile/{uuid}` → skin URL.
    /// Skin lookup is best-effort: `None` when missing/unparseable.
    pub async fn fetch_skin(&self, uuid: &str) -> Option<String> {
        let url = format!(
            "{}/sessionserver/session/minecraft/profile/{uuid}?unsigned=false",
            self.server_url
        );
        let resp = self.http.get(&url).send().await.ok()?;
        if !resp.status().is_success() {
            return None;
        }
        let parsed: ProfileResponse = resp.json().await.ok()?;
        for prop in parsed.properties {
            if prop.name != "textures" {
                continue;
            }
            let decoded = base64::engine::general_purpose::STANDARD
                .decode(&prop.value)
                .ok()?;
            let textures: serde_json::Value = serde_json::from_slice(&decoded).ok()?;
            let skin_url = textures
                .get("textures")?
                .get("SKIN")?
                .get("url")?
                .as_str()?
                .to_string();
            return Some(skin_url);
        }
        None
    }
}

fn normalize_server_url(url: String) -> String {
    let url = url.trim().trim_end_matches('/');
    if url.is_empty() {
        LITTLESKIN_URL.to_string()
    } else {
        url.to_string()
    }
}

fn map_yggdrasil_error(status: u16, body: &str, op: &str) -> YuhinaError {
    let message = serde_json::from_str::<serde_json::Value>(body)
        .ok()
        .and_then(|v| {
            v.get("errorMessage")
                .and_then(|m| m.as_str())
                .map(|s| s.to_string())
        })
        .unwrap_or_else(|| format!("{op} failed with HTTP {status}"));
    YuhinaError::auth(format!("Yggdrasil {op} failed: {message}"))
}

/// An authenticated Yggdrasil session.
#[derive(Debug, Clone)]
pub struct YggdrasilSession {
    pub access_token: String,
    pub client_token: String,
    pub profile: YggdrasilProfile,
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct YggdrasilProfile {
    pub id: String,
    pub name: String,
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct AuthenticateResponse {
    access_token: Option<String>,
    client_token: Option<String>,
    selected_profile: Option<YggdrasilProfile>,
    #[serde(default)]
    available_profiles: Vec<YggdrasilProfile>,
}

#[derive(Debug, serde::Deserialize)]
struct ProfileResponse {
    #[serde(default)]
    properties: Vec<TextureProperty>,
}

#[derive(Debug, serde::Deserialize)]
struct TextureProperty {
    name: String,
    value: String,
}

/// Build a Yggdrasil `Account` from a session. `make_active` mirrors the
/// offline semantics (first account becomes active).
pub fn build_yggdrasil_account(
    server_url: &str,
    session: &YggdrasilSession,
    skin_url: Option<String>,
    make_active: bool,
) -> Account {
    Account {
        id: uuid::Uuid::new_v4().to_string(),
        kind: AccountKind::Yggdrasil,
        username: session.profile.name.clone(),
        uuid: session.profile.id.clone(),
        yggdrasil_server: Some(normalize_server_url(server_url.to_string())),
        skin_url,
        is_active: make_active,
        expires_at: None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn preset_url_constants() {
        assert_eq!(LITTLESKIN_URL, "https://littleskin.cn/api/yggdrasil");
    }

    #[test]
    fn empty_server_url_falls_back_to_littleskin() {
        let c = YggdrasilClient::new("", reqwest::Client::new());
        assert_eq!(c.server_url(), LITTLESKIN_URL);
        let c = YggdrasilClient::new("https://example.com/api/yggdrasil/", reqwest::Client::new());
        assert_eq!(c.server_url(), "https://example.com/api/yggdrasil");
    }

    #[test]
    fn account_builder_maps_session() {
        let session = YggdrasilSession {
            access_token: "at".into(),
            client_token: "ct".into(),
            profile: YggdrasilProfile {
                id: "deadbeef".into(),
                name: "Player".into(),
            },
        };
        let acc = build_yggdrasil_account(
            "https://example.com/api/yggdrasil",
            &session,
            Some("https://example.com/skin.png".into()),
            true,
        );
        assert_eq!(acc.kind, AccountKind::Yggdrasil);
        assert_eq!(acc.username, "Player");
        assert_eq!(acc.uuid, "deadbeef");
        assert_eq!(
            acc.yggdrasil_server.as_deref(),
            Some("https://example.com/api/yggdrasil")
        );
        assert!(acc.is_active);
    }
}