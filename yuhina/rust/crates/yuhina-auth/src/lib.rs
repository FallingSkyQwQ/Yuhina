//! Authentication: Microsoft OAuth, Yggdrasil, offline accounts
//! (api-contract.md §3.2, 04-agent-auth.md).

pub mod crypto;
pub mod ms_auth;
pub mod offline;
pub mod store;
pub mod yggdrasil;

use std::path::Path;
use std::sync::Arc;

use tokio::sync::broadcast;

use yuhina_api::{
    Account, AccountAuth, AccountKind, AppEvent, MicrosoftLoginHandle, Result, YuhinaError,
};

use crate::crypto::Crypto;
use crate::ms_auth::MsAuth;
use crate::store::{AccountTokens, Store};
use crate::yggdrasil::YggdrasilClient;

/// High-level auth facade implementing contract §3.2.
#[derive(Clone)]
pub struct AuthService {
    store: Arc<Store>,
    ms: MsAuth,
    events: broadcast::Sender<AppEvent>,
}

impl AuthService {
    /// Open the service over `<data_dir>/yuhina.db` + encrypted token store.
    pub fn new(data_dir: &Path) -> Result<Self> {
        let store = Arc::new(Store::open(data_dir)?);
        Ok(Self::from_store(store))
    }

    /// Build over an in-memory store (tests).
    pub fn new_in_memory(crypto: Crypto) -> Result<Self> {
        let store = Arc::new(Store::open_in_memory(crypto)?);
        Ok(Self::from_store(store))
    }

    fn from_store(store: Arc<Store>) -> Self {
        let (events, _) = broadcast::channel(32);
        AuthService {
            store,
            ms: MsAuth::new(reqwest::Client::new()),
            events,
        }
    }

    /// Subscribe to service events (wire into `watch_events` at the bridge).
    pub fn subscribe_events(&self) -> broadcast::Receiver<AppEvent> {
        self.events.subscribe()
    }

    fn notify(&self) {
        let _ = self.events.send(AppEvent::AccountsChanged);
    }

    fn should_activate(&self) -> Result<bool> {
        Ok(!self.store.has_active()?)
    }

    // ---- contract §3.2 ---------------------------------------------------

    pub async fn list_accounts(&self) -> Vec<Account> {
        match self.store.list_accounts() {
            Ok(accs) => accs.into_iter().map(|s| s.account).collect(),
            Err(e) => {
                tracing::error!("list_accounts failed: {e}");
                Vec::new()
            }
        }
    }

    pub async fn set_active_account(&self, id: String) -> Result<()> {
        if self.store.load_account(&id)?.is_none() {
            return Err(YuhinaError::auth(format!("account not found: {id}")));
        }
        self.store.set_active(&id)?;
        self.notify();
        Ok(())
    }

    pub async fn add_offline_account(&self, username: String) -> Result<Account> {
        let activate = self.should_activate()?;
        let account = offline::build_offline_account(&username, activate)?;
        self.store
            .save_account(&account, &AccountTokens::default())?;
        self.notify();
        Ok(account)
    }

    pub async fn begin_microsoft_login(&self) -> Result<MicrosoftLoginHandle> {
        Ok(self.begin_microsoft_login_with_details().await?.0)
    }

    /// Like `begin_microsoft_login`, also returning the loopback redirect URI
    /// and authorize URL (integration tests / diagnostics).
    pub async fn begin_microsoft_login_with_details(
        &self,
    ) -> Result<(MicrosoftLoginHandle, crate::ms_auth::MsLoginDetails)> {
        self.ms.begin_login_details().await
    }

    pub async fn poll_microsoft_login(
        &self,
        handle: MicrosoftLoginHandle,
    ) -> Result<Option<Account>> {
        match self.ms.poll_login(&handle.handle_id).await? {
            None => Ok(None),
            Some(completed) => {
                let activate = self.should_activate()?;
                let account = {
                    let mut acc = completed.account;
                    acc.is_active = activate;
                    acc
                };
                let tokens = AccountTokens {
                    access_token: Some(completed.session.mc_access_token.clone()),
                    refresh_token: Some(completed.session.ms_refresh_token.clone()),
                };
                self.store.save_account(&account, &tokens)?;
                self.notify();
                Ok(Some(account))
            }
        }
    }

    pub async fn cancel_microsoft_login(&self, handle: MicrosoftLoginHandle) -> Result<()> {
        self.ms.cancel_login(&handle.handle_id).await
    }

    pub async fn add_yggdrasil_account(
        &self,
        server_url: String,
        username: String,
        password: String,
    ) -> Result<Account> {
        if username.trim().is_empty() {
            return Err(YuhinaError::auth("Username must not be empty."));
        }
        if password.is_empty() {
            return Err(YuhinaError::auth("Password must not be empty."));
        }
        let client = YggdrasilClient::new(server_url, reqwest::Client::new());
        // clientToken is derived from a stable per-account UUID so refresh works
        // across sessions while remaining unique per account (multi-account isolation).
        let account_id = uuid::Uuid::new_v4().to_string();
        let session = client
            .authenticate(&username, &password, &account_id)
            .await?;
        let activate = self.should_activate()?;
        let skin_url = client.fetch_skin(&session.profile.id).await;
        let account = crate::yggdrasil::build_yggdrasil_account(
            client.server_url(),
            &session,
            skin_url,
            activate,
        );
        // Keep the clientToken stable for refresh: it equals the account id.
        let account = Account {
            id: account_id,
            ..account
        };
        let tokens = AccountTokens {
            access_token: Some(session.access_token.clone()),
            refresh_token: None,
        };
        self.store.save_account(&account, &tokens)?;
        self.notify();
        Ok(account)
    }

    pub async fn refresh_account(&self, id: String) -> Result<Account> {
        let stored = self
            .store
            .load_account(&id)?
            .ok_or_else(|| YuhinaError::auth(format!("account not found: {id}")))?;
        let mut account = stored.account;

        match account.kind {
            AccountKind::Offline => Ok(account),
            AccountKind::Microsoft => {
                let refresh = stored.tokens.refresh_token.clone().ok_or_else(|| {
                    YuhinaError::auth_expired("no refresh token; please log in again.")
                })?;
                let session = self
                    .ms
                    .refresh_session(ms_auth::DEFAULT_CLIENT_ID, &refresh)
                    .await?;
                account.username = session.profile.name.clone();
                account.uuid = session.profile.id.clone();
                account.skin_url = session.profile.skin_url.clone();
                account.expires_at = Some(session.expires_at);
                let tokens = AccountTokens {
                    access_token: Some(session.mc_access_token.clone()),
                    refresh_token: Some(session.ms_refresh_token.clone()),
                };
                self.store.save_account(&account, &tokens)?;
                self.notify();
                Ok(account)
            }
            AccountKind::Yggdrasil => {
                let access = stored.tokens.access_token.clone().ok_or_else(|| {
                    YuhinaError::auth_expired("no yggdrasil token; please log in again.")
                })?;
                let server = account
                    .yggdrasil_server
                    .clone()
                    .unwrap_or_else(|| yggdrasil::LITTLESKIN_URL.to_string());
                let client = YggdrasilClient::new(server, reqwest::Client::new());
                let session = client.refresh(&access, &id).await?;
                account.username = session.profile.name.clone();
                account.uuid = session.profile.id.clone();
                account.skin_url = client.fetch_skin(&session.profile.id).await;
                let tokens = AccountTokens {
                    access_token: Some(session.access_token.clone()),
                    refresh_token: None,
                };
                self.store.save_account(&account, &tokens)?;
                self.notify();
                Ok(account)
            }
        }
    }

    pub async fn remove_account(&self, id: String) -> Result<()> {
        let was_active = self.store.load_account(&id)?.map(|s| s.account.is_active);
        self.store.remove_account(&id)?;
        if was_active == Some(true) {
            self.store.clear_active()?;
        }
        self.notify();
        Ok(())
    }

    pub async fn get_active_account(&self) -> Result<Account> {
        match self.store.get_active()? {
            Some(s) => Ok(s.account),
            None => Err(YuhinaError::not_logged_in()),
        }
    }

    // ---- extras for Agent A / bridge ------------------------------------

    /// Launch credentials for the active account (Agent A consumption).
    /// Offline: `access_token = "0"`, `user_type = "legacy"`.
    pub async fn get_active_account_auth(&self) -> Result<AccountAuth> {
        let stored = self
            .store
            .get_active()?
            .ok_or_else(YuhinaError::not_logged_in)?;
        let (access_token, user_type) = match stored.account.kind {
            AccountKind::Offline => ("0".to_string(), "legacy".to_string()),
            AccountKind::Microsoft => (
                stored
                    .tokens
                    .access_token
                    .clone()
                    .ok_or_else(|| YuhinaError::auth_expired("token missing; please refresh."))?,
                "msa".to_string(),
            ),
            AccountKind::Yggdrasil => (
                stored
                    .tokens
                    .access_token
                    .clone()
                    .ok_or_else(|| YuhinaError::auth_expired("token missing; please refresh."))?,
                "mojang".to_string(),
            ),
        };
        Ok(AccountAuth {
            username: stored.account.username,
            uuid: stored.account.uuid,
            access_token,
            user_type,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn service() -> AuthService {
        AuthService::new_in_memory(Crypto::from_key([21u8; 32])).unwrap()
    }

    #[tokio::test]
    async fn add_and_list_offline_account() {
        let svc = service();
        let acc = svc.add_offline_account("Steve".into()).await.unwrap();
        assert_eq!(acc.kind, AccountKind::Offline);
        assert!(acc.is_active); // first account becomes active

        let list = svc.list_accounts().await;
        assert_eq!(list.len(), 1);
        assert_eq!(list[0].username, "Steve");
    }

    #[tokio::test]
    async fn get_active_fails_when_no_account() {
        let svc = service();
        let err = svc.get_active_account().await.unwrap_err();
        assert_eq!(err.kind, yuhina_api::YuhinaErrorKind::NotLoggedIn);
        assert!(svc.get_active_account_auth().await.is_err());
    }

    #[tokio::test]
    async fn set_active_switches_unique() {
        let svc = service();
        let a = svc.add_offline_account("Alice".into()).await.unwrap();
        let b = svc.add_offline_account("Bob".into()).await.unwrap();
        assert!(a.is_active);
        assert!(!b.is_active);

        svc.set_active_account(b.id.clone()).await.unwrap();
        let active = svc.get_active_account().await.unwrap();
        assert_eq!(active.id, b.id);
        let actives = svc
            .list_accounts()
            .await
            .iter()
            .filter(|c| c.is_active)
            .count();
        assert_eq!(actives, 1);
    }

    #[tokio::test]
    async fn set_active_unknown_id_fails() {
        let svc = service();
        assert!(svc
            .set_active_account("does-not-exist".into())
            .await
            .is_err());
    }

    #[tokio::test]
    async fn offline_auth_uses_zero_token_and_legacy() {
        let svc = service();
        svc.add_offline_account("Steve".into()).await.unwrap();
        let auth = svc.get_active_account_auth().await.unwrap();
        assert_eq!(auth.username, "Steve");
        assert_eq!(auth.access_token, "0");
        assert_eq!(auth.user_type, "legacy");
        assert_eq!(auth.uuid, offline::offline_uuid("Steve"));
    }

    #[tokio::test]
    async fn remove_active_clears_active() {
        let svc = service();
        let a = svc.add_offline_account("Alice".into()).await.unwrap();
        svc.remove_account(a.id.clone()).await.unwrap();
        assert!(svc.get_active_account().await.is_err());
        assert!(svc.list_accounts().await.is_empty());
    }

    #[tokio::test]
    async fn refresh_offline_is_identity() {
        let svc = service();
        let a = svc.add_offline_account("Steve".into()).await.unwrap();
        let refreshed = svc.refresh_account(a.id.clone()).await.unwrap();
        assert_eq!(refreshed.id, a.id);
    }

    #[tokio::test]
    async fn events_broadcast_accounts_changed() {
        let svc = service();
        let mut rx = svc.subscribe_events();
        svc.add_offline_account("Steve".into()).await.unwrap();
        let ev = tokio::time::timeout(std::time::Duration::from_secs(1), rx.recv())
            .await
            .unwrap()
            .unwrap();
        assert_eq!(ev, AppEvent::AccountsChanged);
    }

    #[tokio::test]
    async fn begin_poll_cancel_flow() {
        unsafe {
            std::env::set_var("YUHINA_MS_NO_BROWSER", "1");
        }
        let svc = service();
        let handle = svc.begin_microsoft_login().await.unwrap();
        // No code yet → waiting.
        assert!(svc
            .poll_microsoft_login(handle.clone())
            .await
            .unwrap()
            .is_none());
        svc.cancel_microsoft_login(handle).await.unwrap();
        // After cancel, handle is gone → error.
        let h = MicrosoftLoginHandle {
            handle_id: "missing".into(),
        };
        assert!(svc.poll_microsoft_login(h).await.is_err());
    }

    #[tokio::test]
    async fn yggdrasil_requires_credentials() {
        let svc = service();
        assert!(svc
            .add_yggdrasil_account("".into(), "".into(), "".into())
            .await
            .is_err());
        assert!(svc
            .add_yggdrasil_account("".into(), "User".into(), "".into())
            .await
            .is_err());
    }
}