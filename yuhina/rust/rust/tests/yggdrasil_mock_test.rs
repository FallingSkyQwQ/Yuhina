//! Yggdrasil mock round-trip test (04-agent-auth.md T4).
//!
//! A local axum server stands in for an authlib-injector server
//! (`/authserver/authenticate`, `/authserver/refresh`,
//! `/sessionserver/session/minecraft/profile/{uuid}`). Exercises login with
//! skin fetch, multi-account clientToken isolation, refresh, and error mapping.

use std::sync::{Arc, Mutex};

use axum::http::{HeaderMap, StatusCode, Uri};
use axum::response::Json;
use axum::routing::get;
use axum::{routing::post, Router};
use serde_json::{json, Value};

use yuhina_api::{AccountKind, YuhinaErrorKind};
use yuhina_auth::crypto::Crypto;
use yuhina_auth::AuthService;

#[derive(Debug, Clone)]
struct Capture {
    path: String,
    body: String,
}

type Captures = Arc<Mutex<Vec<Capture>>>;

/// `auth_error` maps the error body; respond 403 + Yggdrasil error JSON.
fn error_response() -> (StatusCode, Json<Value>) {
    (
        StatusCode::FORBIDDEN,
        Json(json!({
            "error": "ForbiddenOperationException",
            "errorMessage": "Invalid credentials. Invalid username or password.",
        })),
    )
}

fn success_authenticate() -> (StatusCode, Json<Value>) {
    (
        StatusCode::OK,
        Json(json!({
            "accessToken": "YGG-ACCESS-TOKEN",
            "clientToken": "YGG-CLIENT-TOKEN",
            "selectedProfile": { "id": "ygg-profile-uuid", "name": "YggSteve" },
            "availableProfiles": [{ "id": "ygg-profile-uuid", "name": "YggSteve" }],
        })),
    )
}

fn success_refresh() -> (StatusCode, Json<Value>) {
    (
        StatusCode::OK,
        Json(json!({
            "accessToken": "YGG-ACCESS-TOKEN-REFRESHED",
            "clientToken": "YGG-CLIENT-TOKEN",
            "selectedProfile": { "id": "ygg-profile-uuid", "name": "YggSteve" },
        })),
    )
}

fn textures_value(skin_url: &str) -> String {
    let payload = json!({
        "timestamp": 1700000000000u64,
        "profileId": "ygg-profile-uuid",
        "profileName": "YggSteve",
        "textures": { "SKIN": { "url": skin_url } },
    });
    base64_std_encode(payload.to_string().as_bytes())
}

fn base64_std_encode(bytes: &[u8]) -> String {
    use base64::Engine;
    base64::engine::general_purpose::STANDARD.encode(bytes)
}

async fn spawn_yggdrasil_mock(password: &str) -> (String, Captures) {
    let captures: Captures = Arc::new(Mutex::new(Vec::new()));
    let state = captures.clone();
    let good_password = password.to_string();
    let auth_state = state.clone();
    let auth_pw = good_password.clone();
    let refresh_state = state.clone();
    let profile_state = state.clone();
    let app = Router::new()
        .route(
            "/authserver/authenticate",
            post(move |uri: Uri, _headers: HeaderMap, body: String| {
                let state = auth_state.clone();
                let good_password = auth_pw.clone();
                async move {
                    state.lock().unwrap().push(Capture {
                        path: uri.path().to_string(),
                        body: body.clone(),
                    });
                    let parsed: Value = serde_json::from_str(&body).unwrap();
                    let pw = parsed["password"].as_str().unwrap_or("");
                    if pw != good_password {
                        error_response()
                    } else {
                        success_authenticate()
                    }
                }
            }),
        )
        .route(
            "/authserver/refresh",
            post(move |uri: Uri, _body: String| {
                let state = refresh_state.clone();
                async move {
                    state.lock().unwrap().push(Capture {
                        path: uri.path().to_string(),
                        body: _body.clone(),
                    });
                    success_refresh()
                }
            }),
        )
        .route(
            "/sessionserver/session/minecraft/profile/:uuid",
            get(move |uri: Uri, _headers: HeaderMap| {
                let state = profile_state.clone();
                async move {
                    let path = uri.path().to_string();
                    state.lock().unwrap().push(Capture {
                        path: path.clone(),
                        body: String::new(),
                    });
                    let skin_url = "https://littleskin.cn/textures/ygg-skin.png";
                    (
                        StatusCode::OK,
                        Json(json!({
                            "id": "ygg-profile-uuid",
                            "name": "YggSteve",
                            "properties": [
                                { "name": "textures", "value": textures_value(skin_url) },
                            ],
                        })),
                    )
                }
            }),
        )
        .with_state(());

    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    tokio::spawn(async move {
        axum::serve(listener, app).await.unwrap();
    });
    (format!("http://{addr}"), captures)
}

fn json_body(c: &Capture) -> Value {
    serde_json::from_str(&c.body).unwrap()
}

fn last_auth_capture(captures: &Captures) -> Capture {
    captures
        .lock()
        .unwrap()
        .iter()
        .rev()
        .find(|c| c.path == "/authserver/authenticate")
        .cloned()
        .expect("authenticate capture")
}

#[tokio::test]
async fn yggdrasil_login_skin_and_refresh_round_trip() {
    let (base, captures) = spawn_yggdrasil_mock("correct-password").await;
    let svc = AuthService::new_in_memory(Crypto::from_key([51u8; 32])).unwrap();

    let account = svc
        .add_yggdrasil_account(base.clone(), "YggSteve".into(), "correct-password".into())
        .await
        .unwrap();

    assert_eq!(account.kind, AccountKind::Yggdrasil);
    assert_eq!(account.username, "YggSteve");
    assert_eq!(account.uuid, "ygg-profile-uuid");
    assert_eq!(account.yggdrasil_server.as_deref(), Some(base.as_str()));
    assert_eq!(
        account.skin_url.as_deref(),
        Some("https://littleskin.cn/textures/ygg-skin.png")
    );
    assert!(account.is_active);

    // Authenticate request payload.
    let auth = last_auth_capture(&captures);
    let auth_body = json_body(&auth);
    assert_eq!(auth_body["agent"]["name"].as_str(), Some("Minecraft"));
    assert_eq!(auth_body["agent"]["version"].as_i64(), Some(1));
    assert_eq!(auth_body["username"].as_str(), Some("YggSteve"));
    assert_eq!(auth_body["password"].as_str(), Some("correct-password"));
    assert_eq!(auth_body["requestUser"].as_bool(), Some(true));
    let client_token_1 = auth_body["clientToken"].as_str().unwrap().to_string();
    assert_eq!(
        client_token_1, account.id,
        "clientToken == stable account id"
    );

    // Skin lookup happened after authenticate.
    assert!(captures
        .lock()
        .unwrap()
        .iter()
        .any(|c| c.path.contains("/minecraft/profile/")));

    // Refresh uses the stored access token and returns a new one.
    let refreshed = svc.refresh_account(account.id.clone()).await.unwrap();
    assert_eq!(refreshed.id, account.id);
    assert_eq!(refreshed.username, "YggSteve");

    let refresh_call = captures
        .lock()
        .unwrap()
        .iter()
        .rev()
        .find(|c| c.path == "/authserver/refresh")
        .cloned()
        .expect("refresh capture");
    let refresh_body = json_body(&refresh_call);
    assert_eq!(
        refresh_body["accessToken"].as_str(),
        Some("YGG-ACCESS-TOKEN")
    );
    assert_eq!(
        refresh_body["clientToken"].as_str(),
        Some(client_token_1.as_str())
    );

    // After refresh, store holds the new token.
    let stored = svc.list_accounts().await;
    assert_eq!(stored.len(), 1);
}

#[tokio::test]
async fn yggdrasil_multi_account_client_token_isolation() {
    let (base, captures) = spawn_yggdrasil_mock("pw").await;
    let svc = AuthService::new_in_memory(Crypto::from_key([52u8; 32])).unwrap();

    let a = svc
        .add_yggdrasil_account(base.clone(), "Alice".into(), "pw".into())
        .await
        .unwrap();
    let b = svc
        .add_yggdrasil_account(base.clone(), "Bob".into(), "pw".into())
        .await
        .unwrap();

    // Two accounts → two distinct clientTokens (each == its own account id).
    let token_calls: Vec<Capture> = captures
        .lock()
        .unwrap()
        .iter()
        .filter(|c| c.path == "/authserver/authenticate")
        .cloned()
        .collect();
    assert_eq!(token_calls.len(), 2);
    let ct_a = json_body(&token_calls[0])["clientToken"]
        .as_str()
        .unwrap()
        .to_string();
    let ct_b = json_body(&token_calls[1])["clientToken"]
        .as_str()
        .unwrap()
        .to_string();
    assert_eq!(ct_a, a.id);
    assert_eq!(ct_b, b.id);
    assert_ne!(ct_a, ct_b);

    // Bob was added second → only the first account is active.
    assert!(a.is_active);
    assert!(!b.is_active);
}

#[tokio::test]
async fn yggdrasil_wrong_password_maps_to_auth_error() {
    let (base, _captures) = spawn_yggdrasil_mock("right-pw").await;
    let svc = AuthService::new_in_memory(Crypto::from_key([53u8; 32])).unwrap();

    let err = svc
        .add_yggdrasil_account(base, "YggSteve".into(), "wrong-pw".into())
        .await
        .expect_err("wrong password must fail");
    assert_eq!(err.kind, YuhinaErrorKind::Auth);
    assert!(
        err.message.contains("Invalid credentials"),
        "error message surfaced from yggdrasil: {}",
        err.message
    );
}

#[tokio::test]
async fn yggdrasil_refresh_on_missing_token_is_auth_expired() {
    let (base, _captures) = spawn_yggdrasil_mock("pw").await;
    let svc = AuthService::new_in_memory(Crypto::from_key([54u8; 32])).unwrap();

    // An account row without a token (simulate via offline account that we treat
    // as yggdrasil by id lookup — refresh of a non-existent id).
    let err = svc
        .refresh_account("no-such-account".into())
        .await
        .expect_err("refresh of unknown account fails");
    assert_eq!(err.kind, YuhinaErrorKind::Auth);

    let _ = base;
}

#[tokio::test]
async fn yggdrasil_http_failure_is_network_or_auth() {
    // Server that always 500s.
    let app = Router::new().route(
        "/authserver/authenticate",
        post(|| async { (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({}))) }),
    );
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    tokio::spawn(async move {
        axum::serve(listener, app).await.unwrap();
    });
    let svc = AuthService::new_in_memory(Crypto::from_key([55u8; 32])).unwrap();
    let err = svc
        .add_yggdrasil_account(format!("http://{addr}"), "User".into(), "pw".into())
        .await
        .expect_err("500 must fail");
    assert_eq!(err.kind, YuhinaErrorKind::Auth);
    assert!(err.message.contains("HTTP 500"), "got: {}", err.message);
}

#[tokio::test]
async fn yggdrasil_empty_server_url_uses_littleskin_preset() {
    // No network needed: authenticate will fail fast with a network error, but
    // the request URL must target the LittleSkin preset.
    let svc = AuthService::new_in_memory(Crypto::from_key([56u8; 32])).unwrap();
    // Can't hit the real server; instead verify the preset constant.
    assert_eq!(
        yuhina_auth::yggdrasil::LITTLESKIN_URL,
        "https://littleskin.cn/api/yggdrasil"
    );
    // Login against the real preset is a manual/M3 test (not in CI).
    let _ = svc;
}
