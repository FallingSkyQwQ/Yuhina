//! `YuhinaService`: the FFI facade aggregating core / download / instance /
//! auth into the api-contract.md §3 surface. Implemented method-for-method.
//!
//! Lifecycle: `YuhinaService::new(config)` is the launcher singleton. Event
//! streams (`watch_events` / `watch_progress` / `watch_game_output`) are wired
//! from the domain broadcast channels into FRB `StreamSink`s (Dart `Stream`).

use std::sync::{Arc, RwLock};

use crate::frb_generated::StreamSink;
use tokio::sync::broadcast;

use yuhina_api::{
    Account, AppEvent, CreateInstanceRequest, DownloadProgressEvent, DownloadState, DownloadTask,
    GameLogEntry, GameOutput, GameSession, InstalledMod, InstanceDetail, InstanceSummary,
    JavaRuntime, LauncherConfig, Loader, MicrosoftLoginHandle, ModConflict, ModUpdate,
    ModrinthProject, ModrinthVersion, NewsItem, Result, SearchResult, VersionMeta, YuhinaError,
};
use yuhina_auth::AuthService;
use yuhina_core::YuhinaCore;
use yuhina_download::{DownloadManager, ManagerConfig, NewsService, Store};
use yuhina_instance::{CoreAdapter, InstanceManager};

/// Aggregated launcher service exposed to Flutter.
pub struct YuhinaService {
    config: Arc<RwLock<LauncherConfig>>,
    core: YuhinaCore,
    download: Arc<DownloadManager>,
    instances: InstanceManager,
    auth: AuthService,
    news: NewsService,
    events_tx: broadcast::Sender<AppEvent>,
}

impl YuhinaService {
    // -----------------------------------------------------------------
    // §3.1 配置与初始化
    // -----------------------------------------------------------------

    /// Construct every domain service over `<data_dir>/yuhina.db` and wire the
    /// global event bus. Call once at app startup.
    pub async fn new(config: LauncherConfig) -> Result<Self> {
        let core =
            YuhinaCore::new(config.clone()).map_err(|e| YuhinaError::internal(e.to_string()))?;
        let paths = core.paths().clone();

        // Shared download store + manager (resumes unfinished persisted tasks).
        let store = Store::open(&paths.db_path)?;
        let download = Arc::new(DownloadManager::start(
            store.clone(),
            ManagerConfig::default(),
        ));
        for t in store.list_tasks()? {
            if matches!(
                t.state,
                DownloadState::Queued | DownloadState::Running | DownloadState::Paused
            ) {
                let _ = download.restore(&t);
            }
        }

        // Instance manager over the same SQLite file + core adapter.
        let db = yuhina_db::Db::new(&paths.db_path).map_err(|e| YuhinaError::io(e.to_string()))?;
        let core_adapter: Arc<dyn CoreAdapter> = Arc::new(core.clone());
        let downloader: Arc<dyn yuhina_core::download::Downloader> = core.downloader();
        let instances = InstanceManager::new(db, core_adapter, downloader, paths.game_root.clone());

        let auth = AuthService::new(&paths.data_dir)?;
        let news = NewsService::new(store, reqwest::Client::new());

        // Merged event bus: forward core + auth broadcasts.
        let (events_tx, _) = broadcast::channel(256);
        {
            let tx = events_tx.clone();
            let mut rx = core.subscribe_events();
            tokio::spawn(async move {
                while let Ok(e) = rx.recv().await {
                    let _ = tx.send(e);
                }
            });
        }
        {
            let tx = events_tx.clone();
            let mut rx = auth.subscribe_events();
            tokio::spawn(async move {
                while let Ok(e) = rx.recv().await {
                    let _ = tx.send(e);
                }
            });
        }

        Ok(Self {
            config: Arc::new(RwLock::new(config)),
            core,
            download,
            instances,
            auth,
            news,
            events_tx,
        })
    }

    pub async fn get_config(&self) -> LauncherConfig {
        self.config.read().expect("config lock").clone()
    }

    /// Persist the config; swaps the core downloader for the new mirror source
    /// and emits `ConfigChanged` on the global event stream.
    pub async fn set_config(&self, config: LauncherConfig) -> Result<()> {
        {
            let mut cfg = self.config.write().expect("config lock");
            *cfg = config.clone();
        }
        self.core.set_config(config);
        Ok(())
    }

    /// Global event stream (Config/Accounts/Instances/Tasks/Java/Versions changed).
    pub async fn watch_events(&self, sink: StreamSink<AppEvent>) {
        let mut rx = self.events_tx.subscribe();
        tokio::spawn(async move {
            while let Ok(e) = rx.recv().await {
                if sink.add(e).is_err() {
                    break;
                }
            }
        });
    }

    /// Resolved absolute data / game-root directories.
    pub async fn resolve_data_paths(&self) -> Result<(String, String)> {
        let p = self.core.paths();
        Ok((
            p.data_dir.to_string_lossy().to_string(),
            p.game_root.to_string_lossy().to_string(),
        ))
    }

    // -----------------------------------------------------------------
    // §3.2 账号
    // -----------------------------------------------------------------

    pub async fn list_accounts(&self) -> Vec<Account> {
        self.auth.list_accounts().await
    }

    pub async fn set_active_account(&self, id: String) -> Result<()> {
        self.auth.set_active_account(id).await
    }

    pub async fn add_offline_account(&self, username: String) -> Result<Account> {
        self.auth.add_offline_account(username).await
    }

    pub async fn begin_microsoft_login(&self) -> Result<MicrosoftLoginHandle> {
        self.auth.begin_microsoft_login().await
    }

    pub async fn poll_microsoft_login(
        &self,
        handle: MicrosoftLoginHandle,
    ) -> Result<Option<Account>> {
        self.auth.poll_microsoft_login(handle).await
    }

    pub async fn cancel_microsoft_login(&self, handle: MicrosoftLoginHandle) -> Result<()> {
        self.auth.cancel_microsoft_login(handle).await
    }

    pub async fn add_yggdrasil_account(
        &self,
        server_url: String,
        username: String,
        password: String,
    ) -> Result<Account> {
        self.auth
            .add_yggdrasil_account(server_url, username, password)
            .await
    }

    pub async fn refresh_account(&self, id: String) -> Result<Account> {
        self.auth.refresh_account(id).await
    }

    pub async fn remove_account(&self, id: String) -> Result<()> {
        self.auth.remove_account(id).await
    }

    /// Fails with `Auth(NotLoggedIn)` when no active account exists.
    pub async fn get_active_account(&self) -> Result<Account> {
        self.auth.get_active_account().await
    }

    // -----------------------------------------------------------------
    // §3.3 版本 / Java
    // -----------------------------------------------------------------

    pub async fn fetch_version_list(&self) -> Result<Vec<VersionMeta>> {
        self.core.fetch_version_list().await
    }

    pub async fn get_version_list(&self) -> Vec<VersionMeta> {
        self.core.get_version_list()
    }

    pub async fn list_java_runtimes(&self) -> Vec<JavaRuntime> {
        self.core.list_java_runtimes()
    }

    pub async fn scan_system_java(&self) -> Result<()> {
        self.core.scan_system_java()
    }

    pub async fn add_manual_java(&self, path: String) -> Result<JavaRuntime> {
        self.core.add_manual_java(path)
    }

    /// Progress is surfaced through `watch_progress` (the core enqueues a task).
    pub async fn install_java(&self, major: u32) -> Result<JavaRuntime> {
        self.core.install_java(major).await
    }

    pub async fn remove_java(&self, id: String) -> Result<()> {
        self.core.remove_java(&id)
    }

    // -----------------------------------------------------------------
    // §3.4 实例
    // -----------------------------------------------------------------

    pub async fn list_instances(&self) -> Vec<InstanceSummary> {
        self.instances.list_instances().await
    }

    pub async fn get_instance(&self, id: String) -> Result<InstanceDetail> {
        self.instances.get_instance(id).await
    }

    pub async fn create_instance(&self, req: CreateInstanceRequest) -> Result<InstanceSummary> {
        let r = self.instances.create_instance(req).await;
        if r.is_ok() {
            let _ = self.events_tx.send(AppEvent::InstancesChanged);
        }
        r
    }

    pub async fn rename_instance(&self, id: String, name: String) -> Result<()> {
        let r = self.instances.rename_instance(id, name).await;
        if r.is_ok() {
            let _ = self.events_tx.send(AppEvent::InstancesChanged);
        }
        r
    }

    pub async fn set_instance_icon(&self, id: String, icon: String) -> Result<()> {
        let r = self.instances.set_instance_icon(id, icon).await;
        if r.is_ok() {
            let _ = self.events_tx.send(AppEvent::InstancesChanged);
        }
        r
    }

    pub async fn clone_instance(&self, id: String, new_name: String) -> Result<InstanceSummary> {
        let r = self.instances.clone_instance(id, new_name).await;
        if r.is_ok() {
            let _ = self.events_tx.send(AppEvent::InstancesChanged);
        }
        r
    }

    pub async fn delete_instance(&self, id: String, delete_files: bool) -> Result<()> {
        let r = self.instances.delete_instance(id, delete_files).await;
        if r.is_ok() {
            let _ = self.events_tx.send(AppEvent::InstancesChanged);
        }
        r
    }

    pub async fn install_instance_loader(&self, id: String, loader: Loader) -> Result<()> {
        let r = self.instances.install_instance_loader(id, loader).await;
        if r.is_ok() {
            let _ = self.events_tx.send(AppEvent::InstancesChanged);
        }
        r
    }

    // -----------------------------------------------------------------
    // §3.5 Mod / 整合包
    // -----------------------------------------------------------------

    pub async fn list_mods(&self, instance_id: String) -> Vec<InstalledMod> {
        self.instances.list_mods(instance_id).await
    }

    pub async fn set_mod_enabled(
        &self,
        instance_id: String,
        mod_id: String,
        enabled: bool,
    ) -> Result<()> {
        let r = self
            .instances
            .set_mod_enabled(instance_id, mod_id, enabled)
            .await;
        if r.is_ok() {
            let _ = self.events_tx.send(AppEvent::InstancesChanged);
        }
        r
    }

    pub async fn delete_mod(&self, instance_id: String, mod_id: String) -> Result<()> {
        let r = self.instances.delete_mod(instance_id, mod_id).await;
        if r.is_ok() {
            let _ = self.events_tx.send(AppEvent::InstancesChanged);
        }
        r
    }

    pub async fn search_mods(
        &self,
        query: String,
        loaders: Vec<String>,
        game_versions: Vec<String>,
        index: u32,
        limit: u32,
    ) -> Result<SearchResult> {
        self.instances
            .search_mods(query, loaders, game_versions, index, limit)
            .await
    }

    pub async fn get_mod_project(&self, project_id: String) -> Result<ModrinthProject> {
        self.instances.get_mod_project(project_id).await
    }

    pub async fn list_mod_versions(
        &self,
        project_id: String,
        loaders: Vec<String>,
        game_versions: Vec<String>,
    ) -> Vec<ModrinthVersion> {
        self.instances
            .list_mod_versions(project_id, loaders, game_versions)
            .await
    }

    /// `version_id: None` auto-selects the latest compatible version.
    pub async fn install_mod(
        &self,
        instance_id: String,
        project_id: String,
        version_id: Option<String>,
    ) -> Result<InstalledMod> {
        let r = self
            .instances
            .install_mod(instance_id, project_id, version_id)
            .await;
        if r.is_ok() {
            let _ = self.events_tx.send(AppEvent::InstancesChanged);
        }
        r
    }

    pub async fn install_mod_file(
        &self,
        instance_id: String,
        path: String,
    ) -> Result<InstalledMod> {
        let r = self.instances.install_mod_file(instance_id, path).await;
        if r.is_ok() {
            let _ = self.events_tx.send(AppEvent::InstancesChanged);
        }
        r
    }

    pub async fn check_mod_updates(&self, instance_id: String) -> Result<Vec<ModUpdate>> {
        self.instances.check_mod_updates(instance_id).await
    }

    pub async fn update_mod(
        &self,
        instance_id: String,
        mod_id: String,
        to_version_id: String,
    ) -> Result<InstalledMod> {
        let r = self
            .instances
            .update_mod(instance_id, mod_id, to_version_id)
            .await;
        if r.is_ok() {
            let _ = self.events_tx.send(AppEvent::InstancesChanged);
        }
        r
    }

    pub async fn check_mod_conflicts(&self, instance_id: String) -> Result<Vec<ModConflict>> {
        self.instances.check_mod_conflicts(instance_id).await
    }

    pub async fn export_modpack(&self, instance_id: String, dest_path: String) -> Result<String> {
        self.instances.export_modpack(instance_id, dest_path).await
    }

    pub async fn import_modpack(
        &self,
        mrpack_path: String,
        name: String,
    ) -> Result<InstanceSummary> {
        let r = self.instances.import_modpack(mrpack_path, name).await;
        if r.is_ok() {
            let _ = self.events_tx.send(AppEvent::InstancesChanged);
        }
        r
    }

    pub async fn download_modpack_from_modrinth(
        &self,
        project_id: String,
        version_id: String,
    ) -> Result<InstanceSummary> {
        let r = self
            .instances
            .download_modpack_from_modrinth(project_id, version_id)
            .await;
        if r.is_ok() {
            let _ = self.events_tx.send(AppEvent::InstancesChanged);
        }
        r
    }

    // -----------------------------------------------------------------
    // §3.6 下载中心
    // -----------------------------------------------------------------

    pub async fn list_download_tasks(&self) -> Vec<DownloadTask> {
        self.download.list_tasks().unwrap_or_default()
    }

    pub async fn pause_task(&self, id: String) -> Result<()> {
        self.download.pause_task(&id).await
    }

    pub async fn resume_task(&self, id: String) -> Result<()> {
        self.download.resume_task(&id).await
    }

    pub async fn cancel_task(&self, id: String) -> Result<()> {
        self.download.cancel_task(&id).await
    }

    pub async fn clear_finished_tasks(&self) -> Result<()> {
        self.download.clear_finished()
    }

    /// Global download progress stream (throttled to 100ms by the manager).
    pub async fn watch_progress(&self, sink: StreamSink<DownloadProgressEvent>) {
        let mut rx = self.download.subscribe();
        tokio::spawn(async move {
            while let Ok(e) = rx.recv().await {
                if sink.add(e).is_err() {
                    break;
                }
            }
        });
    }

    // -----------------------------------------------------------------
    // §3.7 启动 / 进程
    // -----------------------------------------------------------------

    pub async fn launch_instance(&self, instance_id: String) -> Result<GameSession> {
        self.core.launch_instance(&instance_id).await
    }

    pub async fn stop_game(&self, session_id: String) -> Result<()> {
        self.core.stop_game(&session_id).await
    }

    pub async fn get_game_session(&self, session_id: String) -> Result<GameSession> {
        self.core.get_game_session(&session_id).await
    }

    pub async fn list_game_sessions(&self) -> Vec<GameSession> {
        self.core.list_game_sessions().await
    }

    /// Real-time stdout/stderr stream for one session (line-delimited).
    pub async fn watch_game_output(
        &self,
        session_id: String,
        sink: StreamSink<GameOutput>,
    ) -> Result<()> {
        let rx = self
            .core
            .subscribe_game_output(&session_id)
            .ok_or_else(|| {
                YuhinaError::invalid_instance(format!("no game session {session_id}"))
            })?;
        tokio::spawn(async move {
            let mut rx = rx;
            while let Ok(e) = rx.recv().await {
                if sink.add(e).is_err() {
                    break;
                }
            }
        });
        Ok(())
    }

    pub async fn get_game_logs(&self, session_id: String, after_index: u64) -> Vec<GameLogEntry> {
        self.core.get_game_logs(&session_id, after_index)
    }

    pub async fn open_game_dir(&self, instance_id: String) -> Result<()> {
        self.core.open_game_dir(&instance_id)
    }

    // -----------------------------------------------------------------
    // §3.8 资讯 / 更新
    // -----------------------------------------------------------------

    pub async fn fetch_news(&self) -> Result<Vec<NewsItem>> {
        self.news.fetch_news().await
    }

    pub async fn get_news(&self) -> Vec<NewsItem> {
        self.news.get_news()
    }

    /// Latest launcher version tag, or `None` when up to date / unavailable.
    pub async fn check_launcher_update(&self) -> Result<Option<String>> {
        let client = reqwest::Client::new();
        yuhina_download::check_launcher_update(
            &client,
            env!("CARGO_PKG_VERSION"),
            yuhina_download::DEFAULT_UPDATE_API_URL,
        )
        .await
    }
}
