//! Loader version queries + install orchestration (task T2).
//!
//! Version lists are aggregated from the four meta sources through Agent A's
//! `resolve_loader_versions` (mirror-aware). Installation delegates to Agent
//! A's `install_loader_for_instance` via the [`CoreAdapter`] trait; on failure
//! the instance's loader fields are rolled back and freshly-created installer
//! artefacts in the game dir are removed.

use std::path::{Path, PathBuf};

use yuhina_api::{Loader, LoaderKind, YuhinaError, YuhinaResult};

use crate::dependency::loader_tag;

/// A loader version offered to the UI picker.
#[derive(Debug, Clone)]
pub struct LoaderVersion {
    pub kind: LoaderKind,
    pub version: String,
    /// Human id like `1.20.4-fabric-0.16.0`.
    pub display: String,
    pub recommended: bool,
}

/// Loader orchestration over the core adapter.
#[derive(Clone)]
pub struct LoaderService {
    core: std::sync::Arc<dyn crate::CoreAdapter>,
}

impl LoaderService {
    pub fn new(core: std::sync::Arc<dyn crate::CoreAdapter>) -> Self {
        Self { core }
    }

    /// Query the available loader versions for `mc` + `kind` (for the UI).
    pub async fn available_loader_versions(
        &self,
        mc: &str,
        kind: LoaderKind,
    ) -> YuhinaResult<Vec<LoaderVersion>> {
        validate_loader_for_mc(mc, kind)?;
        let versions = self.core.resolve_loader_versions(mc, kind).await?;
        Ok(versions
            .into_iter()
            .enumerate()
            .map(|(i, v)| LoaderVersion {
                kind,
                version: v.clone(),
                display: format!("{mc}-{}-{v}", loader_tag(kind)),
                recommended: i == 0,
            })
            .collect())
    }

    /// Install a loader for an instance. Returns the installed loader.
    pub async fn install(
        &self,
        instance_id: &str,
        loader: &Loader,
        game_dir: &Path,
        restore: impl FnOnce() -> YuhinaResult<()>,
    ) -> YuhinaResult<Loader> {
        let pre_existing = known_loader_dirs_existing(game_dir);
        let result = self.core.install_loader(instance_id, loader).await;
        if result.is_ok() {
            return result;
        }
        // Rollback: restore the instance row + remove newly created dirs.
        let _ = restore();
        rollback_loader_dirs(game_dir, &pre_existing);
        result
    }
}

/// Validate that a loader kind supports the requested MC version (heuristic).
pub fn validate_loader_for_mc(mc: &str, kind: LoaderKind) -> YuhinaResult<()> {
    let min = match kind {
        LoaderKind::Fabric => (1, 14, 0),
        LoaderKind::Quilt => (1, 18, 2),
        LoaderKind::Forge => (1, 13, 0),
        LoaderKind::NeoForge => (1, 20, 1),
    };
    if mc_tuple(mc) < min {
        return Err(YuhinaError::loader_not_installed(format!(
            "loader {} is not supported for Minecraft {mc} (minimum {}.{}.{})",
            loader_tag(kind),
            min.0,
            min.1,
            min.2
        )));
    }
    Ok(())
}

fn mc_tuple(mc: &str) -> (u32, u32, u32) {
    let parts: Vec<u32> = mc
        .split(['.', '-'])
        .filter_map(|s| s.parse().ok())
        .collect();
    (
        parts.first().copied().unwrap_or(0),
        parts.get(1).copied().unwrap_or(0),
        parts.get(2).copied().unwrap_or(0),
    )
}

/// Loader installer output directories inside a game dir.
pub fn known_loader_dirs() -> &'static [&'static str] {
    &["libraries", "versions"]
}

fn known_loader_dirs_existing(game_dir: &Path) -> Vec<PathBuf> {
    known_loader_dirs()
        .iter()
        .filter(|d| game_dir.join(d).exists())
        .map(|d| game_dir.join(d))
        .collect()
}

/// Remove loader install artefacts created since `pre_existing` was recorded.
pub fn rollback_loader_dirs(game_dir: &Path, pre_existing: &[PathBuf]) {
    for name in known_loader_dirs() {
        let dir = game_dir.join(name);
        if dir.exists() && !pre_existing.contains(&dir) {
            let _ = std::fs::remove_dir_all(&dir);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validate_supported_combinations() {
        assert!(validate_loader_for_mc("1.20.4", LoaderKind::Fabric).is_ok());
        assert!(validate_loader_for_mc("1.20.4", LoaderKind::Quilt).is_ok());
        assert!(validate_loader_for_mc("1.20.4", LoaderKind::Forge).is_ok());
        assert!(validate_loader_for_mc("1.20.4", LoaderKind::NeoForge).is_ok());
        assert!(validate_loader_for_mc("1.21.1", LoaderKind::NeoForge).is_ok());
    }

    #[tokio::test]
    async fn available_versions_map_to_picker_rows() {
        use crate::testutil;
        let core: std::sync::Arc<dyn crate::CoreAdapter> = std::sync::Arc::new(testutil::DummyCore);
        let svc = LoaderService::new(core);
        let rows = svc.available_loader_versions("1.20.4", LoaderKind::Fabric).await.unwrap();
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].kind, LoaderKind::Fabric);
        assert_eq!(rows[0].version, "0.16.0");
        assert_eq!(rows[0].display, "1.20.4-fabric-0.16.0");
        assert!(rows[0].recommended, "first row marked recommended");
    }

    #[test]
    fn validate_unsupported_combinations() {
        assert!(validate_loader_for_mc("1.12.2", LoaderKind::Fabric).is_err());
        assert!(validate_loader_for_mc("1.16.5", LoaderKind::Quilt).is_err());
        assert!(validate_loader_for_mc("1.7.10", LoaderKind::Forge).is_err());
        assert!(validate_loader_for_mc("1.19.2", LoaderKind::NeoForge).is_err());
    }

    #[test]
    fn rollback_removes_created_dirs() {
        let dir = tempfile::tempdir().unwrap();
        let game = dir.path().join("game");
        std::fs::create_dir_all(game.join("mods")).unwrap();
        // pre-existing: mods only. installer creates libraries + versions.
        let pre = known_loader_dirs_existing(&game);
        std::fs::create_dir_all(game.join("libraries")).unwrap();
        std::fs::create_dir_all(game.join("versions")).unwrap();
        std::fs::write(game.join("libraries/x.jar"), b"x").unwrap();
        rollback_loader_dirs(&game, &pre);
        assert!(!game.join("libraries").exists());
        assert!(!game.join("versions").exists());
        assert!(game.join("mods").exists());
    }

    #[tokio::test]
    async fn install_failure_rolls_back_db_and_files() {
        use std::path::PathBuf;
        use std::sync::atomic::{AtomicBool, Ordering};
        use yuhina_api::{Loader, LoaderKind, VersionMeta};
        use yuhina_db::Db;

        use crate::instance::InstanceService;
        use crate::testutil;

        /// Failing core that *creates* a loader dir before erroring, so the
        /// rollback can demonstrate artefact removal.
        struct CreatingFailingCore {
            game_dir: PathBuf,
        }

        #[async_trait::async_trait]
        impl crate::CoreAdapter for CreatingFailingCore {
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
                let _ = std::fs::create_dir_all(self.game_dir.join("libraries"));
                Err(YuhinaError::loader_not_installed(format!(
                    "installer failed for {}",
                    l.version
                )))
            }
        }

        let (dir, game_root) = testutil::temp_game_root();
        let db = Db::in_memory().unwrap();
        let any_core: std::sync::Arc<dyn crate::CoreAdapter> =
            std::sync::Arc::new(testutil::AnyVersionCore);
        let inst = InstanceService::new(db.clone(), any_core, game_root);
        let mut req = testutil::create_request("t", "1.20.4");
        req.loader = Some(Loader {
            kind: LoaderKind::Fabric,
            version: "0.16.0".into(),
        });
        let s = inst.create_instance(req).unwrap();
        let detail = inst.get_instance(&s.id).unwrap();
        let game = std::path::PathBuf::from(&detail.game_dir);

        let core: std::sync::Arc<dyn crate::CoreAdapter> =
            std::sync::Arc::new(CreatingFailingCore {
                game_dir: game.clone(),
            });
        let restored = std::sync::Arc::new(AtomicBool::new(false));
        let restored_clone = restored.clone();
        let svc = LoaderService::new(core);
        let result = svc
            .install(
                &s.id,
                &Loader {
                    kind: LoaderKind::Fabric,
                    version: "0.16.0".into(),
                },
                &game,
                move || {
                    restored_clone.store(true, Ordering::SeqCst);
                    Ok(())
                },
            )
            .await;
        assert!(result.is_err());
        assert!(restored.load(Ordering::SeqCst), "restore callback ran");
        assert!(
            !game.join("libraries").exists(),
            "created loader dir removed"
        );
        let _ = dir;
    }
}
