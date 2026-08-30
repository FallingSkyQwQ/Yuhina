//! Real-network auth protocol tests (handoff.md §2 M3 账号·协议层).
//!
//! These hit live endpoints and are excluded from `cargo test --workspace`;
//! run locally with:
//!   `cargo test --test real_auth_test -- --ignored --nocapture`
//!
//! Covered here:
//!   - Microsoft loopback chain (server binds 127.0.0.1:<high port>, callback
//!     handling, fake-code → readable `Auth` error from the real token
//!     endpoint, no panic/hang, cancel releases the port).
//!   - LittleSkin real endpoints: wrong-password → structured `Auth` error with
//!     the server's errorMessage; unknown-uuid profile lookup → error path.
//!
//! NOT covered (human E2E only): completing a real Microsoft browser
//! authorization, and a successful LittleSkin login with real credentials.

use std::sync::Mutex;
use std::time::Duration;

use yuhina_api::{MicrosoftLoginHandle, YuhinaErrorKind};
use yuhina_auth::crypto::Crypto;
use yuhina_auth::yggdrasil::{YggdrasilClient, LITTLESKIN_URL};
use yuhina_auth::AuthService;

/// Serializes tests that mutate process-wide env vars.
static ENV_LOCK: Mutex<()> = Mutex::new(());

fn env_guard() -> std::sync::MutexGuard<'static, ()> {
    ENV_LOCK.lock().unwrap_or_else(|e| e.into_inner())
}

fn service() -> AuthService {
    AuthService::new_in_memory(Crypto::from_key([99u8; 32])).unwrap()
}

fn parse_state(authorize_url: &str) -> String {
    url::Url::parse(authorize_url)
        .unwrap()
        .query_pairs()
        .find(|(k, _)| k == "state")
        .map(|(_, v)| v.to_string())
        .expect("state present in authorize url")
}

fn parse_port(redirect_uri: &str) -> String {
    redirect_uri
        .split(':')
        .nth(2)
        .unwrap()
        .split('/')
        .next()
        .unwrap()
        .to_string()
}

async fn port_released(port: &str) -> bool {
    let deadline = std::time::Instant::now() + Duration::from_secs(3);
    while std::time::Instant::now() < deadline {
        if tokio::net::TcpListener::bind(format!("127.0.0.1:{port}"))
            .await
            .is_ok()
        {
            return true;
        }
        tokio::time::sleep(Duration::from_millis(50)).await;
    }
    false
}

/// Full Microsoft loopback chain against the real endpoints, stopping right
/// before the human-authorization step: we inject a fake code with the correct
/// OAuth `state` and expect the real token endpoint to reject it as a readable
/// `Auth` error — never a panic, never a hang.
#[tokio::test]
#[allow(clippy::await_holding_lock)]
#[ignore = "requires the real Microsoft OAuth endpoints (not run in CI)"]
async fn ms_loopback_listens_and_rejects_fake_code() {
    let _lock = env_guard();
    unsafe {
        std::env::set_var("YUHINA_MS_NO_BROWSER", "1");
    }
    let svc = service();
    let (handle, details) = svc
        .begin_microsoft_login_with_details()
        .await
        .expect("begin_microsoft_login must start the loopback server");
    let port = parse_port(&details.redirect_uri);
    let state = parse_state(&details.authorize_url);
    let port_num: u16 = port.parse().unwrap();

    // The redirect URI is a high loopback port (client_id injected at begin).
    assert!(
        details.redirect_uri.starts_with("http://127.0.0.1:"),
        "redirect uri is loopback: {}",
        details.redirect_uri
    );
    assert!(port_num >= 20000, "high random port, got {port}");
    assert!(
        details
            .authorize_url
            .contains("ff0aea8c-fc13-40b7-9f40-1c29fa20979b"),
        "real client_id injected into authorize url"
    );

    let client = reqwest::Client::new();
    // Non-callback path is routed by the embedded tiny_http server → 404.
    let r = client
        .get(format!("http://127.0.0.1:{port}/not-a-callback"))
        .send()
        .await
        .expect("loopback server must be listening");
    assert_eq!(r.status().as_u16(), 404, "non-callback path 404s");

    // Inject a fake code with the correct OAuth state (simulated browser step).
    let cb = client
        .get(format!(
            "{}?code=fakecode&state={state}",
            details.redirect_uri
        ))
        .send()
        .await
        .expect("callback request must reach the loopback server");
    assert!(cb.status().is_success(), "callback HTTP {}", cb.status());

    // Poll until the real Microsoft token endpoint answers: fake code → error.
    let err = tokio::time::timeout(Duration::from_secs(60), async {
        for _ in 0..40 {
            match svc.poll_microsoft_login(handle.clone()).await {
                Ok(None) => tokio::time::sleep(Duration::from_millis(100)).await,
                Ok(Some(_)) => panic!("a fake code must never log in"),
                Err(e) => return e,
            }
        }
        panic!("poll never surfaced the token rejection")
    })
    .await
    .expect("poll must not hang");

    assert!(
        matches!(
            err.kind,
            YuhinaErrorKind::Auth | YuhinaErrorKind::AuthExpired
        ),
        "unexpected kind {:?}: {}",
        err.kind,
        err.message
    );
    assert!(
        err.message.contains("invalid_grant") || err.message.contains("AADSTS"),
        "server rejection text must be surfaced: {}",
        err.message
    );
    eprintln!(
        "[ms] fake-code rejection (real Microsoft endpoint): kind={:?} message={}",
        err.kind, err.message
    );

    // Cancel must tear the flow down and release the loopback port.
    svc.cancel_microsoft_login(handle).await.expect("cancel");
    assert!(port_released(&port).await, "loopback port {port} released");
}

/// Cancel cleans the in-flight handle and frees the listening socket.
#[tokio::test]
#[allow(clippy::await_holding_lock)]
#[ignore = "loopback server lifecycle (kept manual, no external network)"]
async fn ms_cancel_releases_loopback_port_cleanly() {
    let _lock = env_guard();
    unsafe {
        std::env::set_var("YUHINA_MS_NO_BROWSER", "1");
    }
    let svc = service();
    let (handle, details) = svc
        .begin_microsoft_login_with_details()
        .await
        .expect("begin login");
    let port = parse_port(&details.redirect_uri);

    // Server is up before cancel.
    let before = tokio::net::TcpStream::connect(format!("127.0.0.1:{port}"))
        .await
        .is_ok();
    assert!(before, "server listening before cancel");

    svc.cancel_microsoft_login(handle).await.expect("cancel");
    assert!(port_released(&port).await, "port {port} free after cancel");

    // Polling a cancelled/unknown handle errors cleanly (no panic).
    let err = svc
        .poll_microsoft_login(MicrosoftLoginHandle {
            handle_id: "gone".into(),
        })
        .await
        .expect_err("poll of unknown handle fails");
    assert_eq!(err.kind, YuhinaErrorKind::Auth);
}

/// Real LittleSkin: a wrong password must be rejected by the server and mapped
/// to a structured `YuhinaError` carrying the server's own errorMessage —
/// proving the protocol and the `/authserver/authenticate` URL rule.
#[tokio::test]
#[ignore = "requires the real LittleSkin endpoint (not run in CI)"]
async fn yggdrasil_real_littleskin_wrong_password() {
    let svc = service();

    let err = tokio::time::timeout(Duration::from_secs(60), async {
        for attempt in 0..3 {
            match svc
                .add_yggdrasil_account(
                    LITTLESKIN_URL.to_string(),
                    "yuhina_probe".into(),
                    "wrongpass".into(),
                )
                .await
            {
                Ok(_) => panic!("a wrong password must never log in"),
                Err(e) if e.kind == YuhinaErrorKind::Auth => return e,
                Err(e) => {
                    eprintln!(
                        "[yggdrasil] attempt {attempt} not an auth rejection: {}; retrying",
                        e.message
                    );
                    tokio::time::sleep(Duration::from_secs(2)).await;
                }
            }
        }
        panic!("LittleSkin not reachable after retries")
    })
    .await
    .expect("must not hang");

    assert_eq!(err.kind, YuhinaErrorKind::Auth, "message: {}", err.message);
    assert!(
        err.message.contains("用户名或密码错误")
            || err.message.contains("Incorrect username or password"),
        "server errorMessage must be surfaced: {}",
        err.message
    );
    eprintln!(
        "[yggdrasil] real LittleSkin rejection: kind={:?} message={}",
        err.kind, err.message
    );

    // A failed login must not persist an account.
    assert!(svc.list_accounts().await.is_empty());
}

/// Real LittleSkin sessionserver URL rule: an unknown-uuid profile lookup must
/// be served by the server as an error (not a network failure), and the
/// best-effort `fetch_skin` must return `None` without panicking.
#[tokio::test]
#[ignore = "requires the real LittleSkin endpoint (not run in CI)"]
async fn yggdrasil_real_profile_unknown_uuid_error_path() {
    let unknown_uuid = "069a79f4-44e9-4726-a5be-fca90e38aaf5";

    let client = reqwest::Client::new();
    // Base URL answers (server is up and the `/api/yggdrasil` prefix is routed).
    let base = client
        .get(LITTLESKIN_URL)
        .send()
        .await
        .expect("LittleSkin base reachable");
    assert!(
        base.status().is_success(),
        "LittleSkin base HTTP {}",
        base.status()
    );

    // `/sessionserver/session/minecraft/profile/{uuid}` route is served; an
    // unknown uuid must be a 4xx from the server, not a transport error.
    let res = client
        .get(format!(
            "{LITTLESKIN_URL}/sessionserver/session/minecraft/profile/{unknown_uuid}?unsigned=false"
        ))
        .send()
        .await
        .expect("profile endpoint reachable");
    assert!(
        res.status().is_client_error(),
        "unknown uuid must 4xx, got {}",
        res.status()
    );
    eprintln!(
        "[yggdrasil] profile lookup of unknown uuid → HTTP {}",
        res.status()
    );

    // Best-effort skin fetch degrades to None (never panics).
    let ygg = YggdrasilClient::new(LITTLESKIN_URL, reqwest::Client::new());
    assert!(ygg.fetch_skin(unknown_uuid).await.is_none());
}
