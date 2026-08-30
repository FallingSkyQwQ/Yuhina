//! Microsoft OAuth end-to-end mock test (04-agent-auth.md T3).
//!
//! Simulates the full browser login: an axum server stands in for the
//! Microsoft token endpoint, XBL, XSTS, `login_with_xbox` and
//! `minecraft/profile`. We drive the real `begin → poll` flow and assert
//! every request body/header along the chain, plus callback-port recycling
//! and the cancel path.

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use axum::extract::State;
use axum::http::{HeaderMap, StatusCode, Uri};
use axum::response::Json;
use axum::{routing::get, Router};
use serde_json::{json, Value};

use yuhina_api::{AccountKind, YuhinaErrorKind};
use yuhina_auth::crypto::Crypto;
use yuhina_auth::AuthService;

/// Serializes tests that mutate process-wide env vars.
static ENV_LOCK: Mutex<()> = Mutex::new(());

fn env_guard() -> std::sync::MutexGuard<'static, ()> {
    ENV_LOCK.lock().unwrap_or_else(|e| e.into_inner())
}

#[derive(Debug, Clone)]
struct Capture {
    path: String,
    headers: HashMap<String, String>,
    body: String,
}

type Captures = Arc<Mutex<Vec<Capture>>>;

async fn spawn_mock() -> (String, Captures) {
    let captures: Captures = Arc::new(Mutex::new(Vec::new()));
    let state = captures.clone();
    let app = Router::new()
        .route("/token", get(handler).post(handler))
        .route("/xbl", get(handler).post(handler))
        .route("/xsts", get(handler).post(handler))
        .route("/login", get(handler).post(handler))
        .route("/profile", get(handler).post(handler))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    tokio::spawn(async move {
        axum::serve(listener, app).await.unwrap();
    });
    (format!("http://{addr}"), captures)
}

async fn handler(
    State(captures): State<Captures>,
    uri: Uri,
    headers: HeaderMap,
    body: String,
) -> (StatusCode, Json<Value>) {
    let headers = headers
        .iter()
        .map(|(k, v)| (k.to_string(), v.to_str().unwrap_or("").to_string()))
        .collect();
    captures.lock().unwrap().push(Capture {
        path: uri.path().to_string(),
        headers,
        body: body.clone(),
    });
    let resp = match uri.path() {
        "/token" => json!({
            "access_token": "MS-ACCESS-MOCK",
            "refresh_token": "MS-REFRESH-MOCK",
            "expires_in": 3600,
        }),
        "/xbl" => json!({
            "Token": "XBL-TOKEN-MOCK",
            "DisplayClaims": { "xui": [ { "uhs": "XBL-UHS-MOCK" } ] },
        }),
        "/xsts" => json!({
            "Token": "XSTS-TOKEN-MOCK",
            "DisplayClaims": { "xui": [ { "uhs": "XBL-UHS-MOCK" } ] },
        }),
        "/login" => json!({
            "access_token": "MC-ACCESS-MOCK",
            "expires_in": 86400,
        }),
        "/profile" => json!({
            "id": "mc-profile-uuid-mock",
            "name": "MockSteve",
            "skins": [{
                "id": "s1", "state": "ACTIVE", "url": "https://mock.example/skin.png",
                "variant": "CLASSIC", "kind": "SKIN",
            }],
        }),
        _ => unreachable!(),
    };
    (StatusCode::OK, Json(resp))
}

fn capture_by_path(captures: &Captures, path: &str) -> Capture {
    captures
        .lock()
        .unwrap()
        .iter()
        .find(|c| c.path == path)
        .cloned()
        .unwrap_or_else(|| panic!("no request captured for {path}"))
}

fn form_params(body: &str) -> HashMap<String, String> {
    url::form_urlencoded::parse(body.as_bytes())
        .into_owned()
        .collect()
}

fn json_body(body: &str) -> Value {
    serde_json::from_str(body).unwrap_or_else(|_| panic!("invalid json: {body}"))
}

async fn poll_until_done(
    svc: &AuthService,
    handle: &yuhina_api::MicrosoftLoginHandle,
) -> yuhina_api::Account {
    for _ in 0..200 {
        if let Some(acc) = svc
            .poll_microsoft_login(handle.clone())
            .await
            .expect("poll should not error in happy path")
        {
            return acc;
        }
        tokio::time::sleep(Duration::from_millis(50)).await;
    }
    panic!("login did not complete in time")
}

async fn simulate_browser_redirect(details: &yuhina_auth::ms_auth::MsLoginDetails, code: &str) {
    let url = url::Url::parse(&details.authorize_url).unwrap();
    let state = url
        .query_pairs()
        .find(|(k, _)| k == "state")
        .map(|(_, v)| v.to_string())
        .unwrap();
    let client = reqwest::Client::new();
    let res = client
        .get(format!(
            "{}?code={code}&state={state}",
            details.redirect_uri
        ))
        .send()
        .await
        .unwrap();
    assert!(res.status().is_success());
}

#[tokio::test]
#[allow(clippy::await_holding_lock)]
async fn ms_login_full_chain_mock() {
    let _lock = env_guard();
    unsafe {
        std::env::set_var("YUHINA_MS_NO_BROWSER", "1");
        std::env::set_var("YUHINA_MS_CLIENT_ID", "mock-client-id");
    }
    let (base, captures) = spawn_mock().await;
    unsafe {
        std::env::set_var("YUHINA_MS_TOKEN_URL", format!("{base}/token"));
        std::env::set_var("YUHINA_XBL_URL", format!("{base}/xbl"));
        std::env::set_var("YUHINA_XSTS_URL", format!("{base}/xsts"));
        std::env::set_var("YUHINA_MC_LOGIN_URL", format!("{base}/login"));
        std::env::set_var("YUHINA_MC_PROFILE_URL", format!("{base}/profile"));
        std::env::set_var("YUHINA_MS_AUTHORIZE_URL", format!("{base}/authorize"));
    }

    let svc = AuthService::new_in_memory(Crypto::from_key([42u8; 32])).unwrap();
    let (handle, details) = svc.begin_microsoft_login_with_details().await.unwrap();

    // Waiting before the callback → Ok(None).
    assert!(svc
        .poll_microsoft_login(handle.clone())
        .await
        .unwrap()
        .is_none());

    simulate_browser_redirect(&details, "auth-code-mock").await;
    let account = poll_until_done(&svc, &handle).await;

    assert_eq!(account.kind, AccountKind::Microsoft);
    assert_eq!(account.username, "MockSteve");
    assert_eq!(account.uuid, "mc-profile-uuid-mock");
    assert!(account.is_active, "first account becomes active");
    assert!(account.expires_at.is_some());
    assert_eq!(
        account.skin_url.as_deref(),
        Some("https://mock.example/skin.png")
    );

    // ---- assert the 5-step chain requests -------------------------------
    let token = capture_by_path(&captures, "/token");
    let params = form_params(&token.body);
    assert_eq!(
        params.get("client_id").map(String::as_str),
        Some("mock-client-id")
    );
    assert_eq!(
        params.get("grant_type").map(String::as_str),
        Some("authorization_code")
    );
    assert_eq!(
        params.get("code").map(String::as_str),
        Some("auth-code-mock")
    );
    assert_eq!(
        params.get("redirect_uri").map(String::as_str),
        Some(details.redirect_uri.as_str())
    );
    let verifier = params.get("code_verifier").expect("code_verifier present");
    assert!(verifier.len() >= 43 && verifier.len() <= 128);

    let xbl = capture_by_path(&captures, "/xbl");
    let xbl_body = json_body(&xbl.body);
    assert_eq!(
        xbl.headers
            .get("x-xbl-contract-version")
            .map(String::as_str),
        Some("1")
    );
    assert_eq!(
        xbl_body["Properties"]["RpsTicket"].as_str(),
        Some("d=MS-ACCESS-MOCK")
    );
    assert_eq!(
        xbl_body["Properties"]["SiteName"].as_str(),
        Some("user.auth.xboxlive.com")
    );
    assert_eq!(
        xbl_body["RelyingParty"].as_str(),
        Some("http://auth.xboxlive.com")
    );

    let xsts = capture_by_path(&captures, "/xsts");
    let xsts_body = json_body(&xsts.body);
    assert_eq!(
        xsts.headers
            .get("x-xbl-contract-version")
            .map(String::as_str),
        Some("1")
    );
    assert_eq!(
        xsts_body["Properties"]["SandboxId"].as_str(),
        Some("RETAIL")
    );
    assert_eq!(
        xsts_body["Properties"]["UserTokens"][0].as_str(),
        Some("XBL-TOKEN-MOCK")
    );
    assert_eq!(
        xsts_body["RelyingParty"].as_str(),
        Some("rp://api.minecraftservices.com/")
    );

    let login = capture_by_path(&captures, "/login");
    let login_body = json_body(&login.body);
    assert_eq!(
        login_body["identityToken"].as_str(),
        Some("XBL3.0 x=XBL-UHS-MOCK;XSTS-TOKEN-MOCK")
    );

    let profile = capture_by_path(&captures, "/profile");
    assert_eq!(
        profile.headers.get("authorization").map(String::as_str),
        Some("Bearer MC-ACCESS-MOCK")
    );
}

#[tokio::test]
#[allow(clippy::await_holding_lock)]
async fn ms_refresh_account_uses_refresh_token() {
    let _lock = env_guard();
    unsafe {
        std::env::set_var("YUHINA_MS_NO_BROWSER", "1");
        std::env::set_var("YUHINA_MS_CLIENT_ID", "mock-client-id");
    }
    let (base, captures) = spawn_mock().await;
    unsafe {
        std::env::set_var("YUHINA_MS_TOKEN_URL", format!("{base}/token"));
        std::env::set_var("YUHINA_XBL_URL", format!("{base}/xbl"));
        std::env::set_var("YUHINA_XSTS_URL", format!("{base}/xsts"));
        std::env::set_var("YUHINA_MC_LOGIN_URL", format!("{base}/login"));
        std::env::set_var("YUHINA_MC_PROFILE_URL", format!("{base}/profile"));
    }

    let svc = AuthService::new_in_memory(Crypto::from_key([43u8; 32])).unwrap();
    let (handle, details) = svc.begin_microsoft_login_with_details().await.unwrap();
    simulate_browser_redirect(&details, "auth-code-mock").await;
    let account = poll_until_done(&svc, &handle).await;

    let refreshed = svc.refresh_account(account.id.clone()).await.unwrap();
    assert_eq!(refreshed.id, account.id);
    assert_eq!(refreshed.kind, AccountKind::Microsoft);
    assert!(refreshed.expires_at.is_some());

    // Last token request must be grant_type=refresh_token with the stored refresh token.
    let token_calls: Vec<_> = captures
        .lock()
        .unwrap()
        .iter()
        .filter(|c| c.path == "/token")
        .cloned()
        .collect();
    assert!(token_calls.len() >= 2);
    let last = form_params(&token_calls.last().unwrap().body);
    assert_eq!(
        last.get("grant_type").map(String::as_str),
        Some("refresh_token")
    );
    assert_eq!(
        last.get("refresh_token").map(String::as_str),
        Some("MS-REFRESH-MOCK")
    );
    // The 5-step chain runs again after refresh.
    assert!(
        captures
            .lock()
            .unwrap()
            .iter()
            .filter(|c| c.path == "/profile")
            .count()
            >= 2
    );
}

#[tokio::test]
#[allow(clippy::await_holding_lock)]
async fn ms_callback_port_is_released_on_cancel() {
    let _lock = env_guard();
    unsafe {
        std::env::set_var("YUHINA_MS_NO_BROWSER", "1");
        std::env::set_var("YUHINA_MS_CLIENT_ID", "mock-client-id");
    }
    let svc = AuthService::new_in_memory(Crypto::from_key([44u8; 32])).unwrap();
    let (handle, details) = svc.begin_microsoft_login_with_details().await.unwrap();
    let port = details
        .redirect_uri
        .split(':')
        .nth(2)
        .unwrap()
        .split('/')
        .next()
        .unwrap()
        .to_string();

    svc.cancel_microsoft_login(handle).await.unwrap();

    // tiny_http closes the listening socket asynchronously on drop (the accept
    // thread unblocks and exits within a few hundred ms) → poll until free.
    let deadline = std::time::Instant::now() + Duration::from_secs(3);
    let mut released = false;
    while std::time::Instant::now() < deadline {
        if tokio::net::TcpListener::bind(format!("127.0.0.1:{port}"))
            .await
            .is_ok()
        {
            released = true;
            break;
        }
        tokio::time::sleep(Duration::from_millis(50)).await;
    }
    assert!(released, "callback port {port} should be free after cancel");

    // Polling a cancelled handle errors cleanly.
    let err = svc
        .poll_microsoft_login(yuhina_api::MicrosoftLoginHandle {
            handle_id: "gone".into(),
        })
        .await
        .unwrap_err();
    assert_eq!(err.kind, YuhinaErrorKind::Auth);
}

#[tokio::test]
#[allow(clippy::await_holding_lock)]
async fn ms_poll_returns_error_on_bad_state() {
    let _lock = env_guard();
    unsafe {
        std::env::set_var("YUHINA_MS_NO_BROWSER", "1");
        std::env::set_var("YUHINA_MS_CLIENT_ID", "mock-client-id");
    }
    let (base, _captures) = spawn_mock().await;
    unsafe {
        std::env::set_var("YUHINA_MS_TOKEN_URL", format!("{base}/token"));
        std::env::set_var("YUHINA_XBL_URL", format!("{base}/xbl"));
        std::env::set_var("YUHINA_XSTS_URL", format!("{base}/xsts"));
        std::env::set_var("YUHINA_MC_LOGIN_URL", format!("{base}/login"));
        std::env::set_var("YUHINA_MC_PROFILE_URL", format!("{base}/profile"));
    }

    let svc = AuthService::new_in_memory(Crypto::from_key([45u8; 32])).unwrap();
    let (handle, details) = svc.begin_microsoft_login_with_details().await.unwrap();

    // Wrong state → CSRF rejection.
    let client = reqwest::Client::new();
    let res = client
        .get(format!("{}?code=evil&state=WRONG", details.redirect_uri))
        .send()
        .await
        .unwrap();
    assert!(res.status().is_success());

    let err = svc
        .poll_microsoft_login(handle.clone())
        .await
        .expect_err("state mismatch must fail");
    assert_eq!(err.kind, YuhinaErrorKind::Auth);
    assert!(
        err.message.contains("state"),
        "message explains the cause: {}",
        err.message
    );

    svc.cancel_microsoft_login(handle).await.unwrap();
}
