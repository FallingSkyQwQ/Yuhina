//! Instance management, Modrinth client, mod lifecycle and modpack
//! import/export (Agent C, `yuhina-instance`).
//!
//! `InstanceManager` exposes the api-contract.md §3.4/§3.5 surface for the
//! bridge; the per-domain services live in the sibling modules.

pub mod conflict;
pub mod dependency;
pub mod instance;
pub mod loader;
pub mod modfile;
pub mod modmeta;
pub mod modpack;
pub mod modrinth;

use std::path::{Path, PathBuf};
use std::sync::Arc;

use yuhina_api::{
    CreateInstanceRequest, InstalledMod, InstanceDetail, InstanceSummary, Loader, LoaderKind,
    ModConflict, ModUpdate, ModrinthProject, ModrinthVersion, SearchResult, VersionMeta,
    YuhinaError, YuhinaErrorKind, YuhinaResult,
};
use yuhina_core::download::Downloader;
use yuhina_db::Db;

pub use instance::{InstanceService, DEFAULT_ICON};
pub use loader::{LoaderService, LoaderVersion};
pub use modfile::ModFileService;
pub use modpack::ModpackService;
pub use modrinth::ModrinthClient;

use crate::conflict::{ConflictChecker, ModrinthDepProvider};
use crate::dependency::DependencyResolver;

pub const DEFAULT_MODRINTH_BASE: &str = "https://api.modrinth.com/v2";
/// Repo referenced by the Modrinth-required User-Agent.
pub const USER_AGENT_BASE: &str = "github.com/FallingSkyQwQ/Yuhina";

/// Adapter over the core engine (Agent A) so `yuhina-instance` is decoupled
/// from `YuhinaCore`'s concrete API and testable against a fake. `YuhinaCore`
/// implements this trait.
#[async_trait::async_trait]
pub trait CoreAdapter: Send + Sync {
    /// Cached version list (empty if never fetched).
    fn get_version_list(&self) -> Vec<VersionMeta>;
    /// Ensure game files (client jar/libraries/assets) are present.
    async fn ensure_version_files(&self, mc: &str) -> YuhinaResult<u32>;
    /// Available loader versions for `mc` + `kind` (UI picker data).
    async fn resolve_loader_versions(
        &self,
        mc: &str,
        kind: LoaderKind,
    ) -> YuhinaResult<Vec<String>>;
    /// Install a loader for an instance (downloads + runs the installer).
    async fn install_loader(&self, instance_id: &str, loader: &Loader) -> YuhinaResult<Loader>;
}

#[async_trait::async_trait]
impl CoreAdapter for yuhina_core::YuhinaCore {
    fn get_version_list(&self) -> Vec<VersionMeta> {
        self.get_version_list()
    }
    async fn ensure_version_files(&self, mc: &str) -> YuhinaResult<u32> {
        self.ensure_version_files(mc).await
    }
    async fn resolve_loader_versions(
        &self,
        mc: &str,
        kind: LoaderKind,
    ) -> YuhinaResult<Vec<String>> {
        self.resolve_loader_versions(mc, kind).await
    }
    async fn install_loader(&self, instance_id: &str, loader: &Loader) -> YuhinaResult<Loader> {
        self.install_loader_for_instance(instance_id, loader).await
    }
}

/// Facade over the instance/mod ecosystem. Compose one per launcher process.
#[derive(Clone)]
pub struct InstanceManager {
    db: Db,
    modrinth: ModrinthClient,
    instance_svc: InstanceService,
    modfile_svc: ModFileService,
    loader_svc: LoaderService,
    modpack_svc: ModpackService,
    resolver: DependencyResolver,
}

impl InstanceManager {
    /// Build the facade. `game_root` is the instance directory root
    /// (`LauncherConfig.game_root`).
    pub fn new(
        db: Db,
        core: Arc<dyn CoreAdapter>,
        downloader: Arc<dyn Downloader>,
        game_root: PathBuf,
    ) -> Self {
        let modrinth = ModrinthClient::new();
        Self::with_modrinth(db, core, downloader, game_root, modrinth)
    }

    /// Builder with a custom Modrinth client (mock servers in tests).
    pub fn with_modrinth(
        db: Db,
        core: Arc<dyn CoreAdapter>,
        downloader: Arc<dyn Downloader>,
        game_root: PathBuf,
        modrinth: ModrinthClient,
    ) -> Self {
        let instance_svc = InstanceService::new(db.clone(), Arc::clone(&core), game_root.clone());
        let modfile_svc = ModFileService::new(db.clone(), Arc::clone(&downloader));
        let loader_svc = LoaderService::new(Arc::clone(&core));
        let modpack_svc = ModpackService::new(
            db.clone(),
            Arc::clone(&core),
            Arc::clone(&downloader),
            modrinth.clone(),
            game_root.clone(),
        );
        let resolver = DependencyResolver::new(modrinth.clone());
        Self {
            db,
            modrinth,
            instance_svc,
            modfile_svc,
            loader_svc,
            modpack_svc,
            resolver,
        }
    }

    // -----------------------------------------------------------------
    // §3.4 instances
    // -----------------------------------------------------------------

    pub async fn list_instances(&self) -> Vec<InstanceSummary> {
        self.instance_svc.list_instances()
    }

    pub async fn get_instance(&self, id: String) -> YuhinaResult<InstanceDetail> {
        self.instance_svc.get_instance(&id)
    }

    pub async fn create_instance(
        &self,
        req: CreateInstanceRequest,
    ) -> YuhinaResult<InstanceSummary> {
        self.instance_svc.create_instance(req)
    }

    pub async fn rename_instance(&self, id: String, name: String) -> YuhinaResult<()> {
        self.instance_svc.rename_instance(&id, name)
    }

    pub async fn set_instance_icon(&self, id: String, icon: String) -> YuhinaResult<()> {
        self.instance_svc.set_instance_icon(&id, icon)
    }

    pub async fn clone_instance(
        &self,
        id: String,
        new_name: String,
    ) -> YuhinaResult<InstanceSummary> {
        self.instance_svc.clone_instance(&id, new_name)
    }

    pub async fn delete_instance(&self, id: String, delete_files: bool) -> YuhinaResult<()> {
        self.instance_svc.delete_instance(&id, delete_files)
    }

    pub async fn install_instance_loader(&self, id: String, loader: Loader) -> YuhinaResult<()> {
        let detail = self.instance_svc.get_instance(&id)?;
        crate::loader::validate_loader_for_mc(&detail.summary.mc_version, loader.kind)?;
        let snapshot = detail.summary.loader.clone();
        let snapshot_installed = detail.summary.is_installed;
        let game_dir = PathBuf::from(&detail.game_dir);
        let db = self.db.clone();
        let id_for_restore = id.clone();
        let restore = move || -> YuhinaResult<()> {
            db.instance_repo()
                .update(
                    &id_for_restore,
                    None,
                    None,
                    Some(snapshot.as_ref()),
                    None,
                    None,
                    None,
                    Some(snapshot_installed),
                    None,
                )
                .map_err(|e| YuhinaError::internal(e.to_string()))
        };
        self.loader_svc
            .install(&id, &loader, &game_dir, restore)
            .await?;
        Ok(())
    }

    /// Download game files + install the loader (used before launch).
    pub async fn ensure_installed(&self, id: String) -> YuhinaResult<()> {
        self.instance_svc.ensure_installed(&id).await
    }

    /// Loader versions for the UI picker.
    pub async fn available_loader_versions(
        &self,
        mc: String,
        kind: LoaderKind,
    ) -> YuhinaResult<Vec<LoaderVersion>> {
        self.loader_svc.available_loader_versions(&mc, kind).await
    }

    // -----------------------------------------------------------------
    // §3.5 mods
    // -----------------------------------------------------------------

    pub async fn list_mods(&self, instance_id: String) -> Vec<InstalledMod> {
        self.modfile_svc.list_mods(&instance_id)
    }

    pub async fn set_mod_enabled(
        &self,
        instance_id: String,
        mod_id: String,
        enabled: bool,
    ) -> YuhinaResult<()> {
        self.modfile_svc
            .set_mod_enabled(&instance_id, &mod_id, enabled)
    }

    pub async fn delete_mod(&self, instance_id: String, mod_id: String) -> YuhinaResult<()> {
        self.modfile_svc.delete_mod(&instance_id, &mod_id)
    }

    pub async fn rescan_mods(&self, instance_id: String) -> YuhinaResult<()> {
        self.modfile_svc.rescan(&instance_id)
    }

    pub async fn search_mods(
        &self,
        query: String,
        loaders: Vec<String>,
        game_versions: Vec<String>,
        index: u32,
        limit: u32,
    ) -> YuhinaResult<SearchResult> {
        self.modrinth
            .search(&query, &loaders, &game_versions, index, limit)
            .await
    }

    pub async fn get_mod_project(&self, project_id: String) -> YuhinaResult<ModrinthProject> {
        self.modrinth.get_project(&project_id).await
    }

    pub async fn list_mod_versions(
        &self,
        project_id: String,
        loaders: Vec<String>,
        game_versions: Vec<String>,
    ) -> Vec<ModrinthVersion> {
        self.modrinth
            .get_project_versions(&project_id, &loaders, &game_versions)
            .await
            .unwrap_or_default()
    }

    /// Install a mod from Modrinth. `version_id: None` auto-selects the latest
    /// compatible version; required dependencies are resolved transitively.
    pub async fn install_mod(
        &self,
        instance_id: String,
        project_id: String,
        version_id: Option<String>,
    ) -> YuhinaResult<InstalledMod> {
        let detail = self.instance_svc.get_instance(&instance_id)?;
        let summary = detail.summary.clone();
        let version = match &version_id {
            Some(vid) => self.modrinth.get_version(vid).await?,
            None => self
                .resolver
                .latest_compatible(&project_id, &summary)
                .await?
                .ok_or_else(|| {
                    YuhinaError::new(
                        YuhinaErrorKind::VersionNotFound,
                        format!(
                            "no compatible version of {project_id} for {}",
                            summary.mc_version
                        ),
                    )
                })?,
        };
        let installed_before = self.modfile_svc.list_mods(&instance_id);
        let file = version.files.first().ok_or_else(|| {
            YuhinaError::new(
                YuhinaErrorKind::Internal,
                format!("version {} has no files", version.version_id),
            )
        })?;
        let m = self
            .modfile_svc
            .install_version_file(&instance_id, file, &project_id, &version.version_id)
            .await?;

        // Transitive required dependencies.
        let resolved = self
            .resolver
            .resolve_required(&version, Some(&project_id), &summary, &installed_before)
            .await?;
        for dep in resolved.to_install {
            let already = self
                .modfile_svc
                .list_mods(&instance_id)
                .iter()
                .any(|x| x.project_id.as_deref() == Some(dep.project_id.as_str()));
            if already {
                continue;
            }
            let _ = self
                .modfile_svc
                .install_version_file(
                    &instance_id,
                    &dep.file,
                    &dep.project_id,
                    &dep.version.version_id,
                )
                .await?;
        }
        Ok(m)
    }

    /// Install a local mod file into an instance.
    pub async fn install_mod_file(
        &self,
        instance_id: String,
        path: String,
    ) -> YuhinaResult<InstalledMod> {
        self.modfile_svc
            .install_mod_file(&instance_id, Path::new(&path))
    }

    pub async fn check_mod_updates(&self, instance_id: String) -> YuhinaResult<Vec<ModUpdate>> {
        let detail = self.instance_svc.get_instance(&instance_id)?;
        let mods = self.modfile_svc.list_mods(&instance_id);
        self.resolver.check_updates(&mods, &detail.summary).await
    }

    /// Replace an installed mod with a newer Modrinth version.
    pub async fn update_mod(
        &self,
        instance_id: String,
        mod_id: String,
        to_version_id: String,
    ) -> YuhinaResult<InstalledMod> {
        let mods = self.modfile_svc.list_mods(&instance_id);
        let old = mods
            .iter()
            .find(|m| m.id == mod_id)
            .cloned()
            .ok_or_else(|| {
                YuhinaError::invalid_instance(format!("mod {mod_id} is not installed"))
            })?;
        let (project_id, version) = self
            .modrinth
            .get_version_with_project(&to_version_id)
            .await?;
        let file = version.files.first().ok_or_else(|| {
            YuhinaError::new(
                YuhinaErrorKind::Internal,
                format!("version {to_version_id} has no files"),
            )
        })?;
        let new_m = self
            .modfile_svc
            .install_version_file(&instance_id, file, &project_id, &to_version_id)
            .await?;
        // Remove the old file + row.
        self.modfile_svc.delete_mod(&instance_id, &old.sha1)?;
        Ok(new_m)
    }

    pub async fn check_mod_conflicts(&self, instance_id: String) -> YuhinaResult<Vec<ModConflict>> {
        let detail = self.instance_svc.get_instance(&instance_id)?;
        let mods = self.modfile_svc.list_mods(&instance_id);
        let provider = ModrinthDepProvider {
            client: self.modrinth.clone(),
        };
        Ok(ConflictChecker
            .check(&detail.summary, &mods, &provider)
            .await)
    }

    // -----------------------------------------------------------------
    // modpacks
    // -----------------------------------------------------------------

    pub async fn export_modpack(
        &self,
        instance_id: String,
        dest_path: String,
    ) -> YuhinaResult<String> {
        self.modpack_svc
            .export_modpack(&instance_id, &dest_path)
            .await
    }

    pub async fn import_modpack(
        &self,
        mrpack_path: String,
        name: String,
    ) -> YuhinaResult<InstanceSummary> {
        self.modpack_svc.import_modpack(&mrpack_path, &name).await
    }

    pub async fn download_modpack_from_modrinth(
        &self,
        project_id: String,
        version_id: String,
    ) -> YuhinaResult<InstanceSummary> {
        self.modpack_svc
            .download_modpack_from_modrinth(&project_id, &version_id)
            .await
    }
}

/// sha1 (lowercase hex) of a file's contents.
pub fn sha1_hex_file(path: &std::path::Path) -> YuhinaResult<String> {
    yuhina_download::checksum::sha1_hex_file(path)
        .map_err(|e| YuhinaError::io(format!("sha1 {}: {e}", path.display())))
}

#[cfg(test)]
pub(crate) mod testutil {
    use super::*;
    use yuhina_api::{JavaSelection, LaunchArgs, Source};

    /// In-memory DB (schema migrated).
    pub(crate) fn db() -> Db {
        Db::in_memory().unwrap()
    }

    pub(crate) fn temp_game_root() -> (tempfile::TempDir, std::path::PathBuf) {
        let dir = tempfile::tempdir().unwrap();
        let game = dir.path().join("game");
        (dir, game)
    }

    pub(crate) fn launcher_config(root: &std::path::Path) -> yuhina_api::LauncherConfig {
        yuhina_api::LauncherConfig {
            data_dir: root.join("data").to_string_lossy().to_string(),
            game_root: root.join("game").to_string_lossy().to_string(),
            download_source: Source::Official,
            custom_source_host: None,
            launch_args: LaunchArgs::default(),
            locale: "zh-CN".into(),
            theme_seed: 0,
            auto_update: false,
        }
    }

    pub(crate) fn version(id: &str) -> VersionMeta {
        VersionMeta {
            id: id.into(),
            version_type: "release".into(),
            release_time: String::new(),
            url: String::new(),
            is_latest_release: false,
            is_latest_snapshot: false,
        }
    }

    pub(crate) fn create_request(name: &str, mc: &str) -> CreateInstanceRequest {
        CreateInstanceRequest {
            name: name.into(),
            icon: String::new(),
            mc_version: mc.into(),
            loader: None,
            java: JavaSelection::Auto(21),
            dir_name: None,
        }
    }

    /// Core adapter stub: accepts any version (empty version list).
    pub(crate) struct AnyVersionCore;

    #[async_trait::async_trait]
    impl CoreAdapter for AnyVersionCore {
        fn get_version_list(&self) -> Vec<VersionMeta> {
            Vec::new()
        }
        async fn ensure_version_files(&self, _mc: &str) -> YuhinaResult<u32> {
            Ok(0)
        }
        async fn resolve_loader_versions(
            &self,
            _mc: &str,
            _k: LoaderKind,
        ) -> YuhinaResult<Vec<String>> {
            Ok(vec!["0.16.0".into()])
        }
        async fn install_loader(&self, _id: &str, l: &Loader) -> YuhinaResult<Loader> {
            Ok(l.clone())
        }
    }

    /// Core adapter stub that rejects every loader install (rollback tests).
    pub(crate) struct FailingLoaderCore;

    #[async_trait::async_trait]
    impl CoreAdapter for FailingLoaderCore {
        fn get_version_list(&self) -> Vec<VersionMeta> {
            Vec::new()
        }
        async fn ensure_version_files(&self, _mc: &str) -> YuhinaResult<u32> {
            Ok(0)
        }
        async fn resolve_loader_versions(
            &self,
            _mc: &str,
            _k: LoaderKind,
        ) -> YuhinaResult<Vec<String>> {
            Ok(vec!["0.16.0".into()])
        }
        async fn install_loader(&self, _id: &str, l: &Loader) -> YuhinaResult<Loader> {
            Err(YuhinaError::loader_not_installed(format!(
                "installer failed for {}",
                l.version
            )))
        }
    }

    /// Downloader stub that never downloads (only used in file-move tests).
    pub(crate) struct NoopDownloader;

    #[async_trait::async_trait]
    impl Downloader for NoopDownloader {
        fn rewrite(&self, url: &str) -> String {
            url.to_string()
        }
        async fn download(
            &self,
            _url: &str,
            _dest: &std::path::Path,
            _sha1: Option<&str>,
        ) -> YuhinaResult<()> {
            Err(YuhinaError::network("noop downloader"))
        }
        async fn fetch_bytes(&self, _url: &str) -> YuhinaResult<Vec<u8>> {
            Err(YuhinaError::network("noop downloader"))
        }
    }

    pub(crate) struct DummyCore;

    #[async_trait::async_trait]
    impl CoreAdapter for DummyCore {
        fn get_version_list(&self) -> Vec<VersionMeta> {
            vec![version("1.20.4"), version("1.12.2")]
        }
        async fn ensure_version_files(&self, _mc: &str) -> YuhinaResult<u32> {
            Ok(0)
        }
        async fn resolve_loader_versions(
            &self,
            _mc: &str,
            _k: LoaderKind,
        ) -> YuhinaResult<Vec<String>> {
            Ok(vec!["0.16.0".into()])
        }
        async fn install_loader(&self, _id: &str, l: &Loader) -> YuhinaResult<Loader> {
            Ok(l.clone())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testutil;

    #[tokio::test]
    async fn manager_constructs() {
        let (dir, game) = testutil::temp_game_root();
        let cfg = testutil::launcher_config(dir.path());
        let db = testutil::db();
        let core: Arc<dyn CoreAdapter> = Arc::new(testutil::DummyCore);
        let dl: Arc<dyn Downloader> = Arc::new(testutil::NoopDownloader);
        let m = InstanceManager::new(db, core, dl, game);
        assert!(m.list_instances().await.is_empty());
        let _ = cfg;
    }
}
