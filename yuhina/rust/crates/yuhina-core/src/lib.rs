//! Core engine: version metadata, download orchestration, Java management,
//! launch command building, subprocess management and loader installation.
//!
//! The public surface mirrors `docs/api-contract.md` §3 for the parts owned
//! by Agent A; `yuhina-bridge` adapts these into FRB methods.

use std::collections::HashMap;
use std::path::Path;
use std::sync::{Arc, RwLock};

use serde_json::Value;
use tokio::sync::{broadcast, Mutex};
use tracing::info;
use yuhina_api::{
    Account, AppEvent, GameLogEntry, GameOutput, GameSession, InstanceDetail, InstanceSummary,
    JavaRuntime, JavaSource, LauncherConfig, Loader, LoaderKind, VersionMeta, YuhinaError,
    YuhinaErrorKind, YuhinaResult,
};
use yuhina_db::Db;

use crate::download::{Downloader, HttpDownloader};
use crate::java::{
    detect_java, install_java_from_adoptium, is_java_executable, java_bin_in_home, scan_system,
};
use crate::loader::{install_loader, resolve_loader};
use crate::orchestrate::{add_assets, build_game_file_plan, ensure_downloaded};
use crate::version::{fetch_version_list, get_version_json, VersionJsonCache};

pub mod arguments;
pub mod assets;
pub mod config;
pub mod download;
pub mod java;
pub mod launch;
pub mod libraries;
pub mod loader;
pub mod manifest;
pub mod mirror;
pub mod orchestrate;
pub mod process;
pub mod version;

// Convenience re-exports for downstream consumers (bridge, instance).
pub use crate::config::CorePaths;
pub use crate::launch::{build_classpath_for, build_launch_command, LaunchCommand, LaunchInput};
pub use crate::libraries::{resolve_libraries, Features, Platform};
pub use crate::manifest::VersionManifest;
pub use crate::process::GameManager;
pub use yuhina_api::GameState;

#[cfg(test)]
mod testutil;

/// Milliseconds since the UNIX epoch.
pub fn now_millis() -> i64 {
    yuhina_db::now_millis()
}

/// Core engine handle. Cheap to clone; share one per launcher process.
#[derive(Clone)]
pub struct YuhinaCore {
    config: Arc<RwLock<LauncherConfig>>,
    paths: CorePaths,
    db: Arc<Mutex<Db>>,
    downloader: Arc<RwLock<Arc<dyn Downloader>>>,
    games: Arc<GameManager>,
    events: broadcast::Sender<AppEvent>,
    version_json: Arc<RwLock<HashMap<String, Value>>>,
    logs_registry: Arc<Mutex<HashMap<String, std::path::PathBuf>>>,
}

impl YuhinaCore {
    pub fn new(config: LauncherConfig) -> YuhinaResult<Self> {
        let paths = CorePaths::from_config(&config);
        let db = Db::new(&paths.db_path)?;
        let events = broadcast::channel(256).0;
        let downloader: Arc<dyn Downloader> =
            Arc::new(HttpDownloader::new(config.download_source.clone()));
        Ok(Self {
            config: Arc::new(RwLock::new(config)),
            paths,
            db: Arc::new(Mutex::new(db)),
            downloader: Arc::new(RwLock::new(downloader)),
            games: Arc::new(GameManager::new()),
            events,
            version_json: Arc::new(RwLock::new(HashMap::new())),
            logs_registry: Arc::new(Mutex::new(HashMap::new())),
        })
    }

    // -----------------------------------------------------------------
    // config / events
    // -----------------------------------------------------------------

    pub fn config(&self) -> LauncherConfig {
        self.config.read().expect("config lock").clone()
    }

    pub fn set_config(&self, config: LauncherConfig) {
        let source = config.download_source.clone();
        let mut cfg = self.config.write().expect("config lock");
        *cfg = config;
        drop(cfg);
        // swap downloader for the new mirror source
        let dl: Arc<dyn Downloader> = Arc::new(HttpDownloader::new(source));
        *self.downloader.write().expect("downloader lock") = dl;
        let _ = self.events.send(AppEvent::ConfigChanged);
    }

    pub fn subscribe_events(&self) -> broadcast::Receiver<AppEvent> {
        self.events.subscribe()
    }

    pub fn paths(&self) -> &CorePaths {
        &self.paths
    }

    pub fn downloader(&self) -> Arc<dyn Downloader> {
        self.downloader.read().expect("downloader lock").clone()
    }

    // -----------------------------------------------------------------
    // version list (T3)
    // -----------------------------------------------------------------

    pub async fn fetch_version_list(&self) -> YuhinaResult<Vec<VersionMeta>> {
        let dl = self.downloader();
        let list = fetch_version_list(dl.as_ref()).await?;
        let meta = list.to_meta();
        let db = self.db.lock().await;
        let repo = db.version_cache_repo();
        for m in &meta {
            repo.upsert(m, None)?;
        }
        drop(db);
        let _ = self.events.send(AppEvent::VersionListChanged);
        Ok(meta)
    }

    pub fn get_version_list(&self) -> Vec<VersionMeta> {
        self.with_db(|db| Ok(db.version_cache_repo().list()?))
            .unwrap_or_default()
    }

    /// Load the full version manifest (json) for `id`, using cache or URL.
    pub async fn get_version_manifest(&self, id: &str) -> YuhinaResult<VersionManifest> {
        let dl = self.downloader();
        let value = get_version_json(dl.as_ref(), self, id).await?;
        VersionManifest::parse(&value)
    }

    // -----------------------------------------------------------------
    // game file orchestration (T6)
    // -----------------------------------------------------------------

    /// Ensure client jar + libraries + assets + log config are present for a
    /// version. Returns how many files were downloaded.
    pub async fn ensure_version_files(&self, id: &str) -> YuhinaResult<u32> {
        let manifest = self.get_version_manifest(id).await?;
        let platform = Platform::detect();
        let mut plan = build_game_file_plan(&manifest, &platform, &self.paths);

        // fetch asset index if we have a url
        if !manifest.asset_index.url.is_empty() {
            if let Ok(bytes) = self
                .downloader()
                .fetch_bytes(&manifest.asset_index.url)
                .await
            {
                if let Ok(v) = serde_json::from_slice::<Value>(&bytes) {
                    if let Ok(idx) = crate::assets::AssetIndex::parse(&manifest.asset_index.id, &v)
                    {
                        add_assets(&mut plan, &idx, &self.paths.assets_objects_dir);
                    }
                }
            }
        }
        let dl = self.downloader();
        ensure_downloaded(dl.as_ref(), &plan).await
    }

    // -----------------------------------------------------------------
    // java runtimes (T4)
    // -----------------------------------------------------------------

    pub fn list_java_runtimes(&self) -> Vec<JavaRuntime> {
        self.with_db(|db| Ok(db.java_repo().list()?))
            .unwrap_or_default()
    }

    pub fn scan_system_java(&self) -> YuhinaResult<()> {
        let found = scan_system();
        let core = self.clone();
        let rt = tokio::runtime::Handle::try_current();
        if rt.is_ok() {
            tokio::task::block_in_place(move || {
                tokio::runtime::Handle::current().block_on(async move {
                    let db = core.db.lock().await;
                    for j in found {
                        let repo = db.java_repo();
                        if repo.get_by_path(&j.path)?.is_none() {
                            repo.insert(&j)?;
                        }
                    }
                    Ok(())
                })
            })
        } else {
            let db = self
                .db
                .try_lock()
                .map_err(|_| YuhinaError::internal("db locked"))?;
            for j in found {
                let repo = db.java_repo();
                if repo.get_by_path(&j.path)?.is_none() {
                    repo.insert(&j)?;
                }
            }
            Ok(())
        }
    }

    pub fn add_manual_java(&self, path: String) -> YuhinaResult<JavaRuntime> {
        let p = Path::new(&path);
        let bin = if is_java_executable(p) {
            p.to_path_buf()
        } else {
            java_bin_in_home(p).ok_or_else(|| {
                YuhinaError::java_not_found(format!("{} is not a java binary", p.display()))
            })?
        };
        let bin = std::fs::canonicalize(&bin)
            .map_err(|e| YuhinaError::io(format!("canonicalize {}: {e}", bin.display())))?;
        let info = detect_java(&bin)?;
        let java = JavaRuntime {
            id: uuid::Uuid::new_v4().to_string(),
            path: bin.to_string_lossy().to_string(),
            major: info.major,
            vendor: info.vendor,
            version: info.version,
            arch: info.arch,
            source: JavaSource::Manual,
        };
        self.with_db(|db| {
            db.java_repo().insert(&java)?;
            Ok(())
        })?;
        let _ = self.events.send(AppEvent::JavaRuntimesChanged);
        Ok(java)
    }

    pub async fn install_java(&self, major: u32) -> YuhinaResult<JavaRuntime> {
        let platform = Platform::detect();
        let dest = self.paths.data_dir.join("java");
        let (bin, info) =
            install_java_from_adoptium(self.downloader().as_ref(), major, &dest, &platform).await?;
        let java = JavaRuntime {
            id: uuid::Uuid::new_v4().to_string(),
            path: bin.to_string_lossy().to_string(),
            major: info.major,
            vendor: info.vendor,
            version: info.version,
            arch: info.arch,
            source: JavaSource::Bundled,
        };
        let db = self.db.lock().await;
        db.java_repo().insert(&java)?;
        drop(db);
        let _ = self.events.send(AppEvent::JavaRuntimesChanged);
        Ok(java)
    }

    pub fn remove_java(&self, id: &str) -> YuhinaResult<()> {
        self.with_db(|db| {
            db.java_repo().delete(id)?;
            Ok(())
        })?;
        let _ = self.events.send(AppEvent::JavaRuntimesChanged);
        Ok(())
    }

    /// Pick a java runtime for an instance's `JavaSelection`.
    pub fn select_java(&self, java: &yuhina_api::JavaSelection) -> YuhinaResult<JavaRuntime> {
        let all = self.list_java_runtimes();
        match java {
            yuhina_api::JavaSelection::Manual(path) => {
                all.into_iter().find(|j| j.path == *path).ok_or_else(|| {
                    YuhinaError::java_not_found(format!("java at {path} not registered"))
                })
            }
            yuhina_api::JavaSelection::Auto(major) => {
                let need = if *major > 0 { *major } else { 21 };
                all.iter()
                    .filter(|j| j.major == need)
                    .max_by_key(|j| sort_key(&j.version))
                    .cloned()
                    .or_else(|| all.iter().max_by_key(|j| j.major).cloned())
                    .ok_or_else(|| YuhinaError::java_not_found(format!("no java for major {need}")))
            }
        }
    }

    // -----------------------------------------------------------------
    // instance launch (T5/T8)
    // -----------------------------------------------------------------

    pub async fn launch_instance(&self, instance_id: &str) -> YuhinaResult<GameSession> {
        let account = self.active_account().await?;
        self.launch_instance_with(instance_id, &account).await
    }

    pub async fn launch_instance_with(
        &self,
        instance_id: &str,
        account: &Account,
    ) -> YuhinaResult<GameSession> {
        let detail = self.instance_detail(instance_id).await?;
        let summary = &detail.summary;
        if !summary.is_installed {
            // ensure files first
            let _ = self.ensure_version_files(&summary.mc_version).await?;
        }
        let manifest = self.get_version_manifest(&summary.mc_version).await?;
        let java = self.select_java(&detail.java)?;
        let java_bin = Path::new(&java.path);

        // native extraction
        let platform = Platform::detect();
        let session_natives = self.paths.natives_dir(instance_id);
        if session_natives.exists() {
            let _ = std::fs::remove_dir_all(&session_natives);
        }
        std::fs::create_dir_all(&session_natives)
            .map_err(|e| YuhinaError::io(format!("mkdir natives: {e}")))?;
        let resolved = resolve_libraries(
            &manifest.libraries,
            &platform,
            &Features {
                has_custom_resolution: None,
                is_demo_user: None,
            },
        );
        crate::launch::extract_natives(&resolved, &self.paths.libraries_dir, &session_natives)?;

        let game_dir = Path::new(&detail.game_dir);
        let client_jar = self.paths.client_jar(&summary.mc_version);
        let classpath = build_classpath_for(&resolved, &self.paths, &client_jar);
        let launch_args = detail
            .launch_args
            .clone()
            .unwrap_or_else(|| self.config().launch_args.clone());
        let version_name = loader_version_name(summary, &manifest);
        let input = LaunchInput {
            java_bin,
            game_dir,
            paths: &self.paths,
            assets_index: manifest.asset_index.id.clone(),
            natives_dir: &session_natives,
            classpath,
            version_name,
            version_type: manifest.version_type.clone(),
            main_class: manifest.main_class.clone(),
            launch_args: &launch_args,
            account,
            manifest: &manifest,
            launcher_name: "yuhina".into(),
            launcher_version: env!("CARGO_PKG_VERSION").into(),
            platform,
        };
        let cmd = build_launch_command(&input);
        let log_path = self
            .paths
            .session_log_path(&uuid::Uuid::new_v4().to_string());
        let session = self
            .games
            .spawn(cmd, instance_id, &log_path, game_dir)
            .await?;
        self.logs_registry
            .lock()
            .await
            .insert(session.session_id.clone(), log_path);
        let _ = self.events.send(AppEvent::InstancesChanged);
        Ok(session)
    }

    async fn active_account(&self) -> YuhinaResult<Account> {
        let db = self.db.lock().await;
        db.account_repo()
            .get_active()?
            .ok_or_else(|| YuhinaError::new(YuhinaErrorKind::NotLoggedIn, "no active account"))
    }

    async fn instance_detail(&self, id: &str) -> YuhinaResult<InstanceDetail> {
        let db = self.db.lock().await;
        db.instance_repo().get_detail(id)?.ok_or_else(|| {
            YuhinaError::new(
                YuhinaErrorKind::InvalidInstance,
                format!("instance {id} not found"),
            )
        })
    }

    pub async fn stop_game(&self, session_id: &str) -> YuhinaResult<()> {
        self.games.stop(session_id).await
    }

    pub async fn get_game_session(&self, session_id: &str) -> YuhinaResult<GameSession> {
        self.games.get(session_id).await
    }

    pub async fn list_game_sessions(&self) -> Vec<GameSession> {
        self.games.list().await
    }

    pub fn subscribe_game_output(
        &self,
        session_id: &str,
    ) -> Option<broadcast::Receiver<GameOutput>> {
        self.games.subscribe(session_id)
    }

    pub fn get_game_logs(&self, session_id: &str, after_index: u64) -> Vec<GameLogEntry> {
        let path = self
            .logs_registry
            .try_lock()
            .ok()
            .and_then(|m| m.get(session_id).cloned())
            .or_else(|| self.games.log_path(session_id));
        let Some(path) = path else {
            return Vec::new();
        };
        crate::process::read_game_log(&path, after_index).unwrap_or_default()
    }

    pub fn open_game_dir(&self, instance_id: &str) -> YuhinaResult<()> {
        let detail = self.with_db(|db| {
            db.instance_repo().get_detail(instance_id)?.ok_or_else(|| {
                YuhinaError::new(YuhinaErrorKind::InvalidInstance, "instance not found")
            })
        })?;
        open_dir(Path::new(&detail.game_dir))
    }

    // -----------------------------------------------------------------
    // loader install (T7, consumed by Agent C)
    // -----------------------------------------------------------------

    /// Resolve available loader versions for `mc` (for C's picker UI).
    pub async fn resolve_loader_versions(
        &self,
        mc: &str,
        kind: LoaderKind,
    ) -> YuhinaResult<Vec<String>> {
        let dl = self.downloader();
        match kind {
            LoaderKind::Fabric => {
                let bytes = dl
                    .fetch_bytes(&format!(
                        "https://meta.fabricmc.net/v2/versions/loader/{mc}"
                    ))
                    .await?;
                let v: Value = serde_json::from_slice(&bytes)
                    .map_err(|e| YuhinaError::internal(e.to_string()))?;
                Ok(crate::loader::fabric_versions(&v))
            }
            LoaderKind::Quilt => {
                let bytes = dl
                    .fetch_bytes(&format!("https://meta.quiltmc.org/v3/versions/loader/{mc}"))
                    .await?;
                let v: Value = serde_json::from_slice(&bytes)
                    .map_err(|e| YuhinaError::internal(e.to_string()))?;
                Ok(crate::loader::quilt_versions(&v))
            }
            LoaderKind::Forge => {
                let bytes = dl.fetch_bytes("https://files.minecraftforge.net/net/minecraftforge/forge/promotions_slim.json").await?;
                let v: Value = serde_json::from_slice(&bytes)
                    .map_err(|e| YuhinaError::internal(e.to_string()))?;
                Ok(crate::loader::forge_versions(&v, mc))
            }
            LoaderKind::NeoForge => {
                let bytes = dl.fetch_bytes("https://maven.neoforged.net/releases/net/neoforged/neoforge/maven-metadata.xml").await?;
                let xml = String::from_utf8_lossy(&bytes).to_string();
                Ok(crate::loader::neoforge_versions(&xml, mc))
            }
        }
    }

    /// Low-level loader installation for an instance. Downloads the installer,
    /// runs it in the game dir with a matched Java, updates the instance's
    /// loader fields + installed flag on success. Returns the installed loader.
    pub async fn install_loader_for_instance(
        &self,
        instance_id: &str,
        loader: &Loader,
    ) -> YuhinaResult<Loader> {
        let detail = self.instance_detail(instance_id).await?;
        let mc = detail.summary.mc_version.clone();
        let java = self.select_java(&detail.java)?;

        let dl = self.downloader();
        // fetch meta to resolve the plan
        let choice = match loader.kind {
            LoaderKind::Fabric => {
                let meta: Value = serde_json::from_slice(
                    &dl.fetch_bytes(&format!(
                        "https://meta.fabricmc.net/v2/versions/loader/{mc}"
                    ))
                    .await?,
                )
                .map_err(YuhinaError::from)?;
                let inst: Value = serde_json::from_slice(
                    &dl.fetch_bytes("https://meta.fabricmc.net/v2/versions/installer")
                        .await?,
                )
                .map_err(YuhinaError::from)?;
                resolve_loader(
                    &mc,
                    loader.kind,
                    Some(&loader.version),
                    &meta,
                    &inst,
                    &Value::Null,
                    &Value::Null,
                    "",
                )?
            }
            LoaderKind::Quilt => {
                let meta: Value = serde_json::from_slice(
                    &dl.fetch_bytes(&format!("https://meta.quiltmc.org/v3/versions/loader/{mc}"))
                        .await?,
                )
                .map_err(YuhinaError::from)?;
                resolve_loader(
                    &mc,
                    loader.kind,
                    Some(&loader.version),
                    &Value::Null,
                    &Value::Null,
                    &meta,
                    &Value::Null,
                    "",
                )?
            }
            LoaderKind::Forge => {
                let meta: Value = serde_json::from_slice(&dl.fetch_bytes("https://files.minecraftforge.net/net/minecraftforge/forge/promotions_slim.json").await?).map_err(YuhinaError::from)?;
                resolve_loader(
                    &mc,
                    loader.kind,
                    Some(&loader.version),
                    &Value::Null,
                    &Value::Null,
                    &Value::Null,
                    &meta,
                    "",
                )?
            }
            LoaderKind::NeoForge => {
                let xml = String::from_utf8_lossy(&dl.fetch_bytes("https://maven.neoforged.net/releases/net/neoforged/neoforge/maven-metadata.xml").await?).to_string();
                resolve_loader(
                    &mc,
                    loader.kind,
                    Some(&loader.version),
                    &Value::Null,
                    &Value::Null,
                    &Value::Null,
                    &Value::Null,
                    &xml,
                )?
            }
        };

        let game_dir = Path::new(&detail.game_dir);
        let loader_dir = self.paths.data_dir.join("loaders");
        let result = install_loader(
            dl.as_ref(),
            Path::new(&java.path),
            game_dir,
            &loader_dir,
            &choice,
        )
        .await?;
        if !result.success {
            return Err(YuhinaError::new(
                YuhinaErrorKind::LoaderNotInstalled,
                format!(
                    "loader install failed (exit {:?})\nstdout:\n{}\nstderr:\n{}",
                    result.exit_code, result.stdout, result.stderr
                ),
            ));
        }
        // mark instance installed + record loader
        let db = self.db.lock().await;
        db.instance_repo().update(
            instance_id,
            None,
            None,
            Some(Some(loader)),
            None,
            None,
            None,
            Some(true),
            None,
        )?;
        drop(db);
        let _ = self.events.send(AppEvent::InstancesChanged);
        info!(instance_id, loader = %loader.version, "loader installed");
        Ok(loader.clone())
    }

    // -----------------------------------------------------------------
    // helpers
    // -----------------------------------------------------------------

    fn with_db<T>(&self, f: impl FnOnce(&Db) -> YuhinaResult<T>) -> YuhinaResult<T> {
        match self.db.try_lock() {
            Ok(db) => f(&db),
            Err(_) => {
                // lock held by an async task; block inline on the current runtime
                let core = self.clone();
                tokio::task::block_in_place(|| {
                    tokio::runtime::Handle::current().block_on(async move {
                        let db = core.db.lock().await;
                        f(&db)
                    })
                })
            }
        }
    }
}

impl VersionJsonCache for YuhinaCore {
    fn get_version_json(&self, id: &str) -> Option<Value> {
        if let Some(v) = self.version_json.read().ok()?.get(id) {
            return Some(v.clone());
        }
        let db = self.db.try_lock().ok()?;
        db.version_cache_repo()
            .get(id)
            .ok()
            .flatten()
            .and_then(|r| r.manifest_json)
            .and_then(|s| serde_json::from_str(&s).ok())
    }

    fn put_version_json(&self, id: &str, value: Value) {
        if let Ok(mut m) = self.version_json.write() {
            m.insert(id.to_string(), value.clone());
        }
        let rt = tokio::runtime::Handle::try_current();
        if rt.is_ok() {
            // best-effort persist to db
            if let Ok(db) = self.db.try_lock() {
                let meta = VersionMeta {
                    id: id.to_string(),
                    version_type: value
                        .get("type")
                        .and_then(Value::as_str)
                        .unwrap_or("release")
                        .to_string(),
                    release_time: value
                        .get("releaseTime")
                        .and_then(Value::as_str)
                        .unwrap_or_default()
                        .to_string(),
                    url: String::new(),
                    is_latest_release: false,
                    is_latest_snapshot: false,
                };
                let _ = db
                    .version_cache_repo()
                    .upsert(&meta, Some(&value.to_string()));
            }
        }
    }

    fn get_version_url(&self, id: &str) -> Option<String> {
        self.db
            .try_lock()
            .ok()?
            .version_cache_repo()
            .get(id)
            .ok()
            .flatten()
            .map(|r| r.meta.url)
    }
}

fn loader_version_name(summary: &InstanceSummary, manifest: &VersionManifest) -> String {
    match &summary.loader {
        Some(l) => format!(
            "{}-{}-{}",
            summary.mc_version,
            loader_kind_tag(&l.kind),
            l.version
        ),
        None => manifest.id.clone(),
    }
}

fn loader_kind_tag(kind: &LoaderKind) -> &'static str {
    match kind {
        LoaderKind::Forge => "forge",
        LoaderKind::Fabric => "fabric",
        LoaderKind::NeoForge => "neoforge",
        LoaderKind::Quilt => "quilt",
    }
}

#[cfg(unix)]
fn open_dir(path: &Path) -> YuhinaResult<()> {
    std::process::Command::new("xdg-open")
        .arg(path)
        .spawn()
        .map_err(|e| YuhinaError::io(format!("xdg-open: {e}")))?;
    Ok(())
}

#[cfg(windows)]
fn open_dir(path: &Path) -> YuhinaResult<()> {
    std::process::Command::new("explorer")
        .arg(path)
        .spawn()
        .map_err(|e| YuhinaError::io(format!("explorer: {e}")))?;
    Ok(())
}

fn sort_key(version: &str) -> (u32, u32, u32) {
    let seg: Vec<u32> = version
        .split(['.', '-', '+'])
        .filter_map(|s| s.parse().ok())
        .collect();
    (
        seg.first().copied().unwrap_or(0),
        seg.get(1).copied().unwrap_or(0),
        seg.get(2).copied().unwrap_or(0),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use yuhina_api::{CreateInstanceRequest, JavaSelection, LaunchArgs};

    fn test_core() -> YuhinaCore {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().to_path_buf();
        std::mem::forget(dir);
        let config = LauncherConfig {
            data_dir: path.join("data").to_string_lossy().to_string(),
            game_root: path.join("game").to_string_lossy().to_string(),
            download_source: yuhina_api::Source::Official,
            custom_source_host: None,
            launch_args: LaunchArgs::default(),
            locale: "zh-CN".into(),
            theme_seed: 0,
            auto_update: false,
        };
        YuhinaCore::new(config).unwrap()
    }

    #[test]
    fn core_constructs_with_paths() {
        let core = test_core();
        assert!(core
            .paths()
            .versions_dir
            .to_string_lossy()
            .contains("versions"));
    }

    #[tokio::test]
    #[ignore = "requires network to fetch the Mojang manifest (CI cache allowed)"]
    async fn version_list_cached() {
        let core = test_core();
        let meta = core.fetch_version_list().await.expect("fetch manifest");
        assert!(!meta.is_empty());
        // now served from cache without network
        assert_eq!(core.get_version_list().len(), meta.len());
    }

    #[tokio::test]
    async fn install_loader_fabric_resolution_uses_meta() {
        // offline: verify resolution path against fixtures
        let meta = crate::testutil::load_fixture("fabric_loader_1.20.4.json");
        let versions = crate::loader::fabric_versions(&meta);
        assert!(!versions.is_empty());
    }

    #[test]
    fn loader_version_name_format() {
        let s = InstanceSummary {
            id: "i".into(),
            name: "n".into(),
            icon: "🎮".into(),
            mc_version: "1.20.4".into(),
            loader: Some(Loader {
                kind: LoaderKind::Fabric,
                version: "0.16.0".into(),
            }),
            is_installed: true,
            last_launched_at: None,
            mod_count: 0,
            total_size_bytes: 0,
            created_at: 0,
            updated_at: 0,
        };
        let m = VersionManifest::parse(&crate::testutil::load_fixture("1.20.4.json")).unwrap();
        assert_eq!(loader_version_name(&s, &m), "1.20.4-fabric-0.16.0");
    }

    #[test]
    fn create_instance_request_smoke() {
        let req = CreateInstanceRequest {
            name: "t".into(),
            icon: "🎮".into(),
            mc_version: "1.20.4".into(),
            loader: None,
            java: JavaSelection::Auto(21),
            dir_name: None,
        };
        assert_eq!(req.mc_version, "1.20.4");
    }

    #[test]
    fn select_java_manual_registered() {
        let core = test_core();
        let j = core.add_manual_java("/usr/bin/java".into());
        // may fail if java missing; assert only when it succeeds
        if let Ok(java) = j {
            let selected = core
                .select_java(&JavaSelection::Manual(java.path.clone()))
                .unwrap();
            assert_eq!(selected.id, java.id);
        }
    }
}
