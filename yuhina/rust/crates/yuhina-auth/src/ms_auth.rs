//! Microsoft OAuth authorization-code + PKCE (S256) login
//! (04-agent-auth.md T3).
//!
//! Flow: `begin_login` starts a loopback callback server on
//! `127.0.0.1:<high random port>/callback`, opens the system browser, and
//! returns a `MicrosoftLoginHandle`. `poll_login` is non-blocking: `Ok(None)`
//! while waiting, `Ok(Some(completed))` on success, `Err` with a readable
//! `Auth` reason on failure. `cancel_login` tears the flow down.
//!
//! After the code is captured, the chain is:
//!   code → MS token → XBL → XSTS → login_with_xbox → minecraft/profile.
//!
//! Configuration (env overrides, used by CI/mock tests):
//!   YUHINA_MS_CLIENT_ID        client id (default below)
//!   YUHINA_MS_TOKEN_URL        token endpoint
//!   YUHINA_XBL_URL             XBL authenticate endpoint
//!   YUHINA_XSTS_URL            XSTS authorize endpoint
//!   YUHINA_MC_LOGIN_URL        login_with_xbox endpoint
//!   YUHINA_MC_PROFILE_URL      minecraft profile endpoint
//!   YUHINA_MS_AUTHORIZE_URL    authorize page
//!   YUHINA_MS_NO_BROWSER=1     skip opening the browser (tests)

use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{mpsc, Arc};
use std::time::Duration;

use base64::Engine;
use sha2::{Digest, Sha256};

use yuhina_api::{Account, AccountKind, MicrosoftLoginHandle, Result, YuhinaError};

pub const DEFAULT_CLIENT_ID: &str = "ff0aea8c-fc13-40b7-9f40-1c29fa20979b";
const SCOPE: &str = "XboxLive.signin%20offline_access%20openid%20profile%20email";

#[derive(Debug, Clone)]
pub struct MsAuth {
    http: reqwest::Client,
    logins: Arc<tokio::sync::Mutex<HashMap<String, LoginHandle>>>,
}

/// In-flight login state for one handle.
#[derive(Debug)]
struct LoginHandle {
    verifier: String,
    state: String,
    redirect_uri: String,
    client_id: String,
    rx: mpsc::Receiver<CallbackResult>,
    server: CallbackServer,
}

impl MsAuth {
    pub fn new(http: reqwest::Client) -> Self {
        MsAuth {
            http,
            logins: Arc::new(tokio::sync::Mutex::new(HashMap::new())),
        }
    }

    /// Start the loopback callback server + browser, return a poll handle.
    pub async fn begin_login(&self) -> Result<MicrosoftLoginHandle> {
        Ok(self.begin_login_details().await?.0)
    }

    /// Like `begin_login`, additionally returning the redirect URI and full
    /// authorize URL (used by integration tests to simulate the browser).
    pub async fn begin_login_details(&self) -> Result<(MicrosoftLoginHandle, MsLoginDetails)> {
        let (server, redirect_uri, rx) = CallbackServer::start()?;
        let verifier = random_verifier()?;
        let state = random_token()?;
        let client_id = client_id();

        let challenge = code_challenge(&verifier)?;
        let authorize_url = format!(
            "{authorize}?client_id={client_id}&response_type=code&redirect_uri={redirect_uri}\
             &scope={SCOPE}&code_challenge={challenge}&code_challenge_method=S256\
             &state={state}&prompt=select_account",
            authorize = authorize_url_env(),
            redirect_uri = urlencode(&redirect_uri),
        );

        if std::env::var("YUHINA_MS_NO_BROWSER").as_deref() != Ok("1") {
            if let Err(e) = open::that(&authorize_url) {
                tracing::warn!("failed to open browser: {e}");
            }
        }

        let handle_id = uuid::Uuid::new_v4().to_string();
        let handle = LoginHandle {
            verifier,
            state,
            redirect_uri: redirect_uri.clone(),
            client_id,
            rx,
            server,
        };
        self.logins.lock().await.insert(handle_id.clone(), handle);
        Ok((
            MicrosoftLoginHandle { handle_id },
            MsLoginDetails {
                redirect_uri,
                authorize_url,
            },
        ))
    }

    /// Non-blocking poll. `Ok(None)` = still waiting.
    /// `Ok(Some(MsCompleted))` = full chain done (not yet persisted).
    pub async fn poll_login(&self, handle_id: &str) -> Result<Option<MsCompleted>> {
        let mut logins = self.logins.lock().await;
        let handle = logins
            .get_mut(handle_id)
            .ok_or_else(|| YuhinaError::auth("unknown login handle; begin a new login."))?;
        let result = match handle.rx.try_recv() {
            Ok(r) => r,
            Err(mpsc::TryRecvError::Empty) => return Ok(None),
            Err(mpsc::TryRecvError::Disconnected) => {
                return Err(YuhinaError::auth("callback server stopped unexpectedly."))
            }
        };
        let code = match result {
            CallbackResult::Code { code, state } => {
                if state != handle.state {
                    return Err(YuhinaError::auth("OAuth state mismatch (CSRF)."));
                }
                code
            }
            CallbackResult::Error(msg) => return Err(YuhinaError::auth(msg)),
        };

        let session = self
            .exchange_code(&handle.client_id, &code, &handle.verifier, &handle.redirect_uri)
            .await?;
        let account = build_ms_account(&session, false);

        let mut login = logins.remove(handle_id).unwrap();
        login.server.stop();
        drop(logins);

        Ok(Some(MsCompleted { account, session }))
    }

    /// Cancel an in-flight login, stopping the callback server.
    pub async fn cancel_login(&self, handle_id: &str) -> Result<()> {
        let mut logins = self.logins.lock().await;
        if let Some(mut handle) = logins.remove(handle_id) {
            handle.server.stop();
        }
        Ok(())
    }

    /// Refresh an existing Microsoft session using its refresh token.
    pub async fn refresh_session(&self, client_id: &str, refresh_token: &str) -> Result<MsSession> {
        let token_resp = self
            .http
            .post(token_url_env())
            .form(&[
                ("client_id", client_id.to_string()),
                ("grant_type", "refresh_token".to_string()),
                ("refresh_token", refresh_token.to_string()),
                ("scope", SCOPE.replace("%20", " ")),
            ])
            .send()
            .await
            .map_err(|e| YuhinaError::network(e.to_string()))?;
        let status = token_resp.status();
        let body: TokenResponse = token_resp
            .json()
            .await
            .map_err(|e| YuhinaError::network(e.to_string()))?;
        if !status.is_success() {
            return Err(map_token_error(status.as_u16(), &body));
        }
        let ms_access = body
            .access_token
            .ok_or_else(|| YuhinaError::auth("token refresh returned no access token."))?;
        let ms_refresh = body
            .refresh_token
            .unwrap_or_else(|| refresh_token.to_string());
        self.xbox_chain(ms_access, ms_refresh).await
    }

    /// Full 5-step chain from an authorization code. Public for mock tests.
    pub async fn exchange_code(
        &self,
        client_id: &str,
        code: &str,
        verifier: &str,
        redirect_uri: &str,
    ) -> Result<MsSession> {
        let token_resp = self
            .http
            .post(token_url_env())
            .form(&[
                ("client_id", client_id.to_string()),
                ("grant_type", "authorization_code".to_string()),
                ("code", code.to_string()),
                ("redirect_uri", redirect_uri.to_string()),
                ("code_verifier", verifier.to_string()),
            ])
            .send()
            .await
            .map_err(|e| YuhinaError::network(e.to_string()))?;
        let status = token_resp.status();
        let body: TokenResponse = token_resp
            .json()
            .await
            .map_err(|e| YuhinaError::network(e.to_string()))?;
        if !status.is_success() {
            return Err(map_token_error(status.as_u16(), &body));
        }
        let ms_access = body
            .access_token
            .ok_or_else(|| YuhinaError::auth("token exchange returned no access token."))?;
        let ms_refresh = body
            .refresh_token
            .ok_or_else(|| YuhinaError::auth("token exchange returned no refresh token."))?;
        self.xbox_chain(ms_access, ms_refresh).await
    }

    /// XBL → XSTS → login_with_xbox → profile.
    async fn xbox_chain(
        &self,
        ms_access_token: String,
        ms_refresh_token: String,
    ) -> Result<MsSession> {
        // 2. XBL user token.
        let xbl_body = serde_json::json!({
            "Properties": {
                "AuthMethod": "RPS",
                "SiteName": "user.auth.xboxlive.com",
                "RpsTicket": format!("d={ms_access_token}"),
            },
            "RelyingParty": "http://auth.xboxlive.com",
            "TokenType": "JWT",
        });
        let xbl: XblResponse = self
            .post_json_with_contract(xbl_url_env(), &xbl_body, "XBL authenticate")
            .await?;
        let uhs = xbl
            .display_claims
            .xui
            .first()
            .ok_or_else(|| YuhinaError::auth("XBL returned no user hash."))?
            .uhs
            .clone();
        let xbl_token = xbl.token;

        // 3. XSTS token.
        let xsts_body = serde_json::json!({
            "Properties": {
                "SandboxId": "RETAIL",
                "UserTokens": [xbl_token],
            },
            "RelyingParty": "rp://api.minecraftservices.com/",
            "TokenType": "JWT",
        });
        let xsts: XblResponse = self
            .post_json_with_contract(xsts_url_env(), &xsts_body, "XSTS authorize")
            .await?;
        let xsts_token = xsts.token;
        // XSTS uhs must equal XBL uhs; prefer the fresh one from the response.
        let uhs = xsts
            .display_claims
            .xui
            .first()
            .map(|x| x.uhs.clone())
            .unwrap_or(uhs);

        // 4. Minecraft login_with_xbox.
        let mc_login_body = serde_json::json!({
            "identityToken": format!("XBL3.0 x={uhs};{xsts_token}"),
        });
        let mc_resp = self
            .http
            .post(mc_login_url_env())
            .json(&mc_login_body)
            .send()
            .await
            .map_err(|e| YuhinaError::network(e.to_string()))?;
        let status = mc_resp.status();
        let text = mc_resp
            .text()
            .await
            .map_err(|e| YuhinaError::network(e.to_string()))?;
        if !status.is_success() {
            return Err(YuhinaError::auth(format!(
                "Minecraft login_with_xbox failed with HTTP {status}: {text}"
            )));
        }
        let mc_login: McLoginResponse = serde_json::from_str(&text)
            .map_err(|e| YuhinaError::auth(format!("bad login_with_xbox response: {e}")))?;
        let mc_access_token = mc_login
            .access_token
            .ok_or_else(|| YuhinaError::auth("login_with_xbox returned no access token."))?;
        let expires_in = mc_login.expires_in.unwrap_or(86400);

        // 5. Minecraft profile.
        let profile_resp = self
            .http
            .get(profile_url_env())
            .bearer_auth(&mc_access_token)
            .send()
            .await
            .map_err(|e| YuhinaError::network(e.to_string()))?;
        let status = profile_resp.status();
        let text = profile_resp
            .text()
            .await
            .map_err(|e| YuhinaError::network(e.to_string()))?;
        if !status.is_success() {
            return Err(YuhinaError::auth(format!(
                "Minecraft profile lookup failed with HTTP {status}: {text}"
            )));
        }
        let profile: McProfile = serde_json::from_str(&text)
            .map_err(|e| YuhinaError::auth(format!("bad profile response: {e}")))?;
        if profile.id.is_empty() || profile.name.is_empty() {
            return Err(YuhinaError::auth(
                "Minecraft profile is empty; the account likely owns no Minecraft copy.",
            ));
        }
        let skin_url = profile
            .skins
            .into_iter()
            .rev()
            .find(|s| s.kind == "SKIN")
            .and_then(|s| s.url);

        let expires_at = now_ms() + (expires_in as u64) * 1000;
        Ok(MsSession {
            mc_access_token,
            ms_refresh_token,
            profile: MsProfile {
                id: profile.id,
                name: profile.name,
                skin_url,
            },
            expires_at,
        })
    }

    async fn post_json_with_contract(
        &self,
        url: String,
        body: &serde_json::Value,
        label: &str,
    ) -> Result<XblResponse> {
        let resp = self
            .http
            .post(url)
            .header("x-xbl-contract-version", "1")
            .json(body)
            .send()
            .await
            .map_err(|e| YuhinaError::network(e.to_string()))?;
        let status = resp.status();
        let text = resp
            .text()
            .await
            .map_err(|e| YuhinaError::network(e.to_string()))?;
        if !status.is_success() {
            return Err(YuhinaError::auth(format!(
                "{label} failed with HTTP {status}: {text}"
            )));
        }
        serde_json::from_str(&text)
            .map_err(|e| YuhinaError::auth(format!("bad {label} response: {e}")))
    }
}

fn client_id() -> String {
    std::env::var("YUHINA_MS_CLIENT_ID").unwrap_or_else(|_| DEFAULT_CLIENT_ID.to_string())
}

fn token_url_env() -> String {
    std::env::var("YUHINA_MS_TOKEN_URL")
        .unwrap_or_else(|_| "https://login.microsoftonline.com/consumers/oauth2/v2.0/token".into())
}

fn xbl_url_env() -> String {
    std::env::var("YUHINA_XBL_URL")
        .unwrap_or_else(|_| "https://user.auth.xboxlive.com/user/authenticate".into())
}

fn xsts_url_env() -> String {
    std::env::var("YUHINA_XSTS_URL")
        .unwrap_or_else(|_| "https://xsts.auth.xboxlive.com/xsts/authorize".into())
}

fn mc_login_url_env() -> String {
    std::env::var("YUHINA_MC_LOGIN_URL")
        .unwrap_or_else(|_| "https://api.minecraftservices.com/authentication/login_with_xbox".into())
}

fn profile_url_env() -> String {
    std::env::var("YUHINA_MC_PROFILE_URL")
        .unwrap_or_else(|_| "https://api.minecraftservices.com/minecraft/profile".into())
}

fn authorize_url_env() -> String {
    std::env::var("YUHINA_MS_AUTHORIZE_URL")
        .unwrap_or_else(|_| "https://login.microsoftonline.com/consumers/oauth2/v2.0/authorize".into())
}

/// SHA-256 based PKCE challenge (S256).
fn code_challenge(verifier: &str) -> Result<String> {
    let mut hasher = Sha256::new();
    hasher.update(verifier.as_bytes());
    let digest = hasher.finalize();
    Ok(base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(digest))
}

fn random_verifier() -> Result<String> {
    // 64 random bytes → 86-char base64url string (within the 43..=128 limit).
    let mut buf = [0u8; 64];
    getrandom::getrandom(&mut buf).map_err(|e| YuhinaError::io(e.to_string()))?;
    Ok(base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(buf))
}

fn random_token() -> Result<String> {
    let mut buf = [0u8; 32];
    getrandom::getrandom(&mut buf).map_err(|e| YuhinaError::io(e.to_string()))?;
    Ok(base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(buf))
}

fn urlencode(s: &str) -> String {
    let mut out = String::new();
    for b in s.bytes() {
        match b {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                out.push(b as char)
            }
            _ => out.push_str(&format!("%{b:02X}")),
        }
    }
    out
}

fn map_token_error(status: u16, body: &TokenResponse) -> YuhinaError {
    let error = body.error.clone().unwrap_or_default();
    let description = body.error_description.clone().unwrap_or_default();
    let is_refresh_issue = matches!(
        error.as_str(),
        "invalid_grant" | "interaction_required" | "invalid_request"
    ) || description.contains("AADSTS70008")
        || description.contains("AADSTS700082")
        || description.contains("AADSTS7000215");
    if status == 400 && is_refresh_issue {
        YuhinaError::auth_expired(format!(
            "Microsoft token expired or revoked: {error} {description}"
        ))
    } else {
        YuhinaError::auth(format!(
            "Microsoft token endpoint failed (HTTP {status}): {error} {description}"
        ))
    }
}

/// Result of a completed poll (session + account, not yet persisted).
#[derive(Debug, Clone)]
pub struct MsCompleted {
    pub account: Account,
    pub session: MsSession,
}

/// Details of an in-flight login (integration tests / diagnostics).
#[derive(Debug, Clone)]
pub struct MsLoginDetails {
    pub redirect_uri: String,
    pub authorize_url: String,
}

#[derive(Debug, Clone)]
pub struct MsSession {
    pub mc_access_token: String,
    pub ms_refresh_token: String,
    pub profile: MsProfile,
    pub expires_at: u64,
}

#[derive(Debug, Clone)]
pub struct MsProfile {
    pub id: String,
    pub name: String,
    pub skin_url: Option<String>,
}

pub fn build_ms_account(session: &MsSession, is_active: bool) -> Account {
    Account {
        id: uuid::Uuid::new_v4().to_string(),
        kind: AccountKind::Microsoft,
        username: session.profile.name.clone(),
        uuid: session.profile.id.clone(),
        yggdrasil_server: None,
        skin_url: session.profile.skin_url.clone(),
        is_active,
        expires_at: Some(session.expires_at),
    }
}

// ---------------------------------------------------------------------------
// Loopback callback server
// ---------------------------------------------------------------------------

enum CallbackResult {
    Code { code: String, state: String },
    Error(String),
}

struct CallbackServer {
    server: Arc<tiny_http::Server>,
    stop: Arc<AtomicBool>,
    thread: Option<std::thread::JoinHandle<()>>,
}

impl std::fmt::Debug for CallbackServer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CallbackServer")
            .field("server", &"..")
            .field("stop", &self.stop.load(Ordering::Relaxed))
            .field("thread_alive", &self.thread.is_some())
            .finish()
    }
}

impl CallbackServer {
    /// Bind `127.0.0.1:<random high port>` and serve `/callback` until stopped.
    fn start() -> Result<(Self, String, mpsc::Receiver<CallbackResult>)> {
        let mut last_err = None;
        for _ in 0..20 {
            let port = random_port();
            let addr = format!("127.0.0.1:{port}");
            match tiny_http::Server::http(&addr) {
                Ok(server) => {
                    let redirect_uri = format!("http://{addr}/callback");
                    return Ok(Self::spawn(server, redirect_uri));
                }
                Err(e) => last_err = Some(e),
            }
        }
        Err(YuhinaError::io(format!(
            "could not bind loopback callback server: {last_err:?}"
        )))
    }

    fn spawn(server: tiny_http::Server, redirect_uri: String) -> (Self, String, mpsc::Receiver<CallbackResult>) {
        let server = Arc::new(server);
        let stop = Arc::new(AtomicBool::new(false));
        let (tx, rx) = mpsc::channel();
        let thread = {
            let server = Arc::clone(&server);
            let stop = Arc::clone(&stop);
            let tx = tx.clone();
            std::thread::Builder::new()
                .name("yuhina-ms-callback".into())
                .spawn(move || {
                    while !stop.load(Ordering::Relaxed) {
                        match server.recv_timeout(Duration::from_millis(200)) {
                            Ok(Some(request)) => {
                                let url = request.url().to_string();
                                let response = if url.starts_with("/callback") {
                                    let params = parse_query(url.split('?').nth(1).unwrap_or(""));
                                    let result = match (params.get("code"), params.get("state")) {
                                        (Some(code), Some(state)) => CallbackResult::Code {
                                            code: code.clone(),
                                            state: state.clone(),
                                        },
                                        (Some(_), None) => {
                                            CallbackResult::Error("missing state parameter".into())
                                        }
                                        _ => CallbackResult::Error(
                                            "callback received no authorization code".into(),
                                        ),
                                    };
                                    let _ = tx.send(result);
                                    let html = "<!doctype html><html><head><meta charset=\"utf-8\"><title>Yuhina</title></head><body style=\"font-family:sans-serif;text-align:center;padding-top:4rem\"><h2>登录成功</h2><p>你现在可以关闭此窗口并返回启动器。</p></body></html>";
                                    let content_type = "Content-Type: text/html; charset=utf-8"
                                        .parse::<tiny_http::Header>()
                                        .unwrap();
                                    tiny_http::Response::from_string(html)
                                        .with_status_code(200)
                                        .with_header(content_type)
                                } else {
                                    tiny_http::Response::from_string("Not Found")
                                        .with_status_code(404)
                                };
                                let _ = request.respond(response);
                            }
                            Ok(None) => {}
                            Err(_) => {}
                        }
                    }
                })
                .expect("spawn callback thread")
        };
        (
            CallbackServer {
                server,
                stop,
                thread: Some(thread),
            },
            redirect_uri,
            rx,
        )
    }

    fn stop(&mut self) {
        self.stop.store(true, Ordering::Relaxed);
        self.server.unblock();
        if let Some(thread) = self.thread.take() {
            let _ = thread.join();
        }
    }
}

fn parse_query(query: &str) -> HashMap<String, String> {
    let mut map = HashMap::new();
    for pair in query.split('&') {
        let mut it = pair.splitn(2, '=');
        if let (Some(k), Some(v)) = (it.next(), it.next()) {
            map.insert(percent_decode(k), percent_decode(v));
        }
    }
    map
}

fn percent_decode(s: &str) -> String {
    let bytes = s.as_bytes();
    let mut out = Vec::with_capacity(bytes.len());
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'%' && i + 2 < bytes.len() {
            if let (Some(h), Some(l)) = (hex_val(bytes[i + 1]), hex_val(bytes[i + 2])) {
                out.push((h << 4) | l);
                i += 3;
                continue;
            }
        }
        if bytes[i] == b'+' {
            out.push(b' ');
        } else {
            out.push(bytes[i]);
        }
        i += 1;
    }
    String::from_utf8_lossy(&out).into_owned()
}

fn hex_val(b: u8) -> Option<u8> {
    match b {
        b'0'..=b'9' => Some(b - b'0'),
        b'a'..=b'f' => Some(b - b'a' + 10),
        b'A'..=b'F' => Some(b - b'A' + 10),
        _ => None,
    }
}

fn random_port() -> u16 {
    let mut buf = [0u8; 2];
    if getrandom::getrandom(&mut buf).is_err() {
        buf = [0x5a, 0x00];
    }
    let n = u16::from_le_bytes(buf);
    20000 + (n % 45000)
}

// ---------------------------------------------------------------------------
// Serialization DTOs
// ---------------------------------------------------------------------------

#[derive(Debug, serde::Deserialize)]
struct TokenResponse {
    access_token: Option<String>,
    refresh_token: Option<String>,
    error: Option<String>,
    error_description: Option<String>,
}

#[derive(Debug, serde::Deserialize)]
struct XblResponse {
    #[serde(rename = "Token")]
    token: String,
    #[serde(rename = "DisplayClaims")]
    display_claims: DisplayClaims,
}

#[derive(Debug, serde::Deserialize)]
struct DisplayClaims {
    #[serde(default)]
    xui: Vec<Xui>,
}

#[derive(Debug, serde::Deserialize)]
struct Xui {
    uhs: String,
}

#[derive(Debug, serde::Deserialize)]
struct McLoginResponse {
    access_token: Option<String>,
    expires_in: Option<u64>,
}

#[derive(Debug, serde::Deserialize)]
struct McProfile {
    id: String,
    name: String,
    #[serde(default)]
    skins: Vec<McSkin>,
}

#[derive(Debug, serde::Deserialize)]
struct McSkin {
    #[serde(default)]
    kind: String,
    url: Option<String>,
}

fn now_ms() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_millis() as u64)
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_client_id_is_product_value() {
        // Env not set in tests; ensure the hardcoded default is used.
        unsafe {
            std::env::remove_var("YUHINA_MS_CLIENT_ID");
        }
        assert_eq!(client_id(), DEFAULT_CLIENT_ID);
        assert_eq!(
            DEFAULT_CLIENT_ID,
            "ff0aea8c-fc13-40b7-9f40-1c29fa20979b"
        );
    }

    #[test]
    fn pkce_challenge_is_stable_s256() {
        let verifier = "dBjftJeZ4CVP-mB92K27uhbUJU1p1r_wW1gFWFOEjXk";
        let challenge = code_challenge(verifier).unwrap();
        // RFC 7636 S256 example.
        assert_eq!(
            challenge,
            "E9Melhoa2OwvFrEMTJguCHaoeK1t8URWbuGJSstw-cM"
        );
    }

    #[test]
    fn percent_decode_handles_encoding() {
        assert_eq!(percent_decode("abc"), "abc");
        assert_eq!(percent_decode("a+b"), "a b");
        assert_eq!(percent_decode("code%3Dx%2Fy"), "code=x/y");
    }

    #[test]
    fn parse_query_extracts_code_and_state() {
        let params = parse_query("code=abc123&state=s1&x=y%20z");
        assert_eq!(params.get("code").map(String::as_str), Some("abc123"));
        assert_eq!(params.get("state").map(String::as_str), Some("s1"));
        assert_eq!(params.get("x").map(String::as_str), Some("y z"));
    }

    #[test]
    fn account_builder_uses_profile() {
        let session = MsSession {
            mc_access_token: "at".into(),
            ms_refresh_token: "rt".into(),
            profile: MsProfile {
                id: "player-uuid".into(),
                name: "Notch".into(),
                skin_url: Some("https://skins/s.png".into()),
            },
            expires_at: 42,
        };
        let acc = build_ms_account(&session, true);
        assert_eq!(acc.kind, AccountKind::Microsoft);
        assert_eq!(acc.username, "Notch");
        assert_eq!(acc.uuid, "player-uuid");
        assert_eq!(acc.expires_at, Some(42));
        assert!(acc.is_active);
        assert_eq!(acc.skin_url.as_deref(), Some("https://skins/s.png"));
    }
}