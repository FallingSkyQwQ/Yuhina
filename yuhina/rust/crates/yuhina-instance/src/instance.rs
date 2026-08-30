//! Instance CRUD / clone / delete / directory management (task T1).
//!
//! `game_dir` defaults to `<game_root>/<dir_name>`; `dir_name` may be
//! supplied and gets a numeric suffix on collision. Creating an instance
//! validates the MC version (against the cached version list when available)
//! and the loader/MC combination.

use std::path::{Path, PathBuf};
use std::sync::Arc;

use yuhina_api::{
    CreateInstanceRequest, InstanceDetail, InstanceSummary, JavaSelection, YuhinaError,
    YuhinaResult,
};
use yuhina_db::{Db, InstanceRecord};

use crate::loader::{validate_loader_for_mc, LoaderService};

pub const DEFAULT_ICON: &str = "🎮";

/// Standard sub-directories created for a fresh instance.
const INSTANCE_SUBDIRS: &[&str] = &[
    "mods",
    "config",
    "saves",
    "resourcepacks",
    "shaderpacks",
    "datapacks",
];

/// Instance lifecycle management.
#[derive(Clone)]
pub struct InstanceService {
    db: Db,
    core: Arc<dyn crate::CoreAdapter>,
    game_root: PathBuf,
}

impl InstanceService {
    pub fn new(db: Db, core: Arc<dyn crate::CoreAdapter>, game_root: PathBuf) -> Self {
        Self {
            db,
            core,
            game_root,
        }
    }

    pub fn list_instances(&self) -> Vec<InstanceSummary> {
        self.db.instance_repo().list().unwrap_or_default()
    }

    pub fn get_instance(&self, id: &str) -> YuhinaResult<InstanceDetail> {
        self.db
            .instance_repo()
            .get_detail(id)
            .map_err(|e| YuhinaError::internal(e.to_string()))?
            .ok_or_else(|| YuhinaError::invalid_instance(format!("instance {id} not found")))
    }

    pub fn create_instance(&self, req: CreateInstanceRequest) -> YuhinaResult<InstanceSummary> {
        self.validate_mc_version(&req.mc_version)?;
        if let Some(l) = &req.loader {
            validate_loader_for_mc(&req.mc_version, l.kind)?;
        }
        self.create_instance_unchecked(req)
    }

    /// Create an instance without version/loader validation. Used by mrpack
    /// import where the pack's version may not be in the local cache yet.
    pub(crate) fn create_instance_unchecked(
        &self,
        req: CreateInstanceRequest,
    ) -> YuhinaResult<InstanceSummary> {
        let id = uuid::Uuid::new_v4().to_string();
        let dir_name = resolve_dir_name(&req);
        let game_dir = unique_dir(&self.game_root, &dir_name);
        create_instance_dirs(&game_dir)?;
        self.insert_row(&id, &req, &game_dir, &req.name)
    }

    pub fn rename_instance(&self, id: &str, name: String) -> YuhinaResult<()> {
        if self.get_instance(id).is_err() {
            return Err(YuhinaError::invalid_instance(format!(
                "instance {id} not found"
            )));
        }
        self.db
            .instance_repo()
            .update(id, Some(name), None, None, None, None, None, None, None)
            .map_err(|e| YuhinaError::internal(e.to_string()))
    }

    pub fn set_instance_icon(&self, id: &str, icon: String) -> YuhinaResult<()> {
        if self.get_instance(id).is_err() {
            return Err(YuhinaError::invalid_instance(format!(
                "instance {id} not found"
            )));
        }
        self.db
            .instance_repo()
            .update(id, None, Some(icon), None, None, None, None, None, None)
            .map_err(|e| YuhinaError::internal(e.to_string()))
    }

    /// Deep-copy the game directory + mod records into a new instance.
    pub fn clone_instance(&self, id: &str, new_name: String) -> YuhinaResult<InstanceSummary> {
        let source = self.get_instance(id)?;
        let new_id = uuid::Uuid::new_v4().to_string();
        let dir_name = sanitize_dir_name(&new_name);
        let game_dir = unique_dir(&self.game_root, &dir_name);
        copy_dir_contents(Path::new(&source.game_dir), &game_dir)?;

        let summary = self.insert_row_with(
            &new_id,
            &new_name,
            source.summary.icon.clone(),
            &source.summary.mc_version,
            source.summary.loader.clone(),
            &source.java,
            source.notes.clone(),
            source.summary.is_installed,
            &game_dir,
        )?;

        // Carry over installed mod records (files were copied with the dir).
        let mods = self.db.installed_mod_repo().list(id)?;
        for m in mods {
            self.db.installed_mod_repo().insert(&new_id, &m)?;
        }
        Ok(summary)
    }

    /// Delete an instance; optionally remove its files on disk.
    pub fn delete_instance(&self, id: &str, delete_files: bool) -> YuhinaResult<()> {
        let detail = self.get_instance(id)?;
        self.db
            .instance_repo()
            .delete(id)
            .map_err(|e| YuhinaError::internal(e.to_string()))?;
        if delete_files {
            let dir = Path::new(&detail.game_dir);
            if dir.exists() {
                std::fs::remove_dir_all(dir)
                    .map_err(|e| YuhinaError::io(format!("remove {}: {e}", dir.display())))?;
            }
        }
        Ok(())
    }

    /// Validate the MC version against the cached list when available.
    /// An empty cache (version list never fetched) is allowed so offline
    /// creation still works; the version is re-validated at install time.
    pub fn validate_mc_version(&self, mc: &str) -> YuhinaResult<()> {
        let versions = self.core.get_version_list();
        if versions.is_empty() {
            return Ok(());
        }
        if versions.iter().any(|v| v.id == mc) {
            Ok(())
        } else {
            Err(YuhinaError::version_not_found(format!(
                "Minecraft version '{mc}' is not in the version list"
            )))
        }
    }

    /// Ensure the version files (and loader, if any) are installed.
    pub async fn ensure_installed(&self, id: &str) -> YuhinaResult<()> {
        let detail = self.get_instance(id)?;
        if detail.summary.is_installed {
            return Ok(());
        }
        self.core
            .ensure_version_files(&detail.summary.mc_version)
            .await?;
        if let Some(loader) = detail.summary.loader.clone() {
            let loader_svc = LoaderService::new(Arc::clone(&self.core));
            let game_dir = PathBuf::from(&detail.game_dir);
            let snapshot = detail.summary.loader.clone();
            let snapshot_installed = detail.summary.is_installed;
            let db = self.db.clone();
            let id_owned = id.to_string();
            let restore = move || -> YuhinaResult<()> {
                db.instance_repo()
                    .update(
                        &id_owned,
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
            loader_svc.install(id, &loader, &game_dir, restore).await?;
        } else {
            self.db
                .instance_repo()
                .update(id, None, None, None, None, None, None, Some(true), None)
                .map_err(|e| YuhinaError::internal(e.to_string()))?;
        }
        Ok(())
    }

    fn insert_row(
        &self,
        id: &str,
        req: &CreateInstanceRequest,
        game_dir: &Path,
        name: &str,
    ) -> YuhinaResult<InstanceSummary> {
        let icon = if req.icon.is_empty() {
            DEFAULT_ICON.to_string()
        } else {
            req.icon.clone()
        };
        self.insert_row_with(
            id,
            name,
            icon,
            &req.mc_version,
            req.loader.clone(),
            &req.java,
            String::new(),
            false,
            game_dir,
        )
    }

    #[allow(clippy::too_many_arguments)]
    fn insert_row_with(
        &self,
        id: &str,
        name: &str,
        icon: String,
        mc_version: &str,
        loader: Option<yuhina_api::Loader>,
        java: &JavaSelection,
        notes: String,
        is_installed: bool,
        game_dir: &Path,
    ) -> YuhinaResult<InstanceSummary> {
        let now = yuhina_db::now_millis() as u64;
        let summary = InstanceSummary {
            id: id.to_string(),
            name: name.to_string(),
            icon,
            mc_version: mc_version.to_string(),
            loader,
            is_installed,
            last_launched_at: None,
            mod_count: 0,
            total_size_bytes: 0,
            created_at: now,
            updated_at: now,
        };
        let record = InstanceRecord {
            summary: summary.clone(),
            game_dir: game_dir.to_string_lossy().to_string(),
            java: java.clone(),
            notes,
        };
        self.db
            .instance_repo()
            .create(&record)
            .map_err(|e| YuhinaError::internal(e.to_string()))?;
        Ok(summary)
    }
}

/// The directory name for a request: explicit `dir_name` or a sanitized name.
pub fn resolve_dir_name(req: &CreateInstanceRequest) -> String {
    match &req.dir_name {
        Some(d) if !d.trim().is_empty() => sanitize_dir_name(d),
        _ => sanitize_dir_name(&req.name),
    }
}

/// Replace characters that are invalid or dangerous in a directory name.
pub fn sanitize_dir_name(name: &str) -> String {
    let cleaned: String = name
        .chars()
        .map(|c| match c {
            '/' | '\\' | ':' | '*' | '?' | '"' | '<' | '>' | '|' | '\0' => '_',
            c if c.is_control() => '_',
            c => c,
        })
        .collect();
    let trimmed = cleaned.trim().trim_matches('.').trim();
    if trimmed.is_empty() {
        "instance".to_string()
    } else {
        trimmed.to_string()
    }
}

/// First non-existing `<root>/<name>` path, appending `-2`, `-3`, ... on
/// collisions.
pub fn unique_dir(root: &Path, name: &str) -> PathBuf {
    let mut candidate = root.join(name);
    let mut n = 2;
    while candidate.exists() {
        candidate = root.join(format!("{name}-{n}"));
        n += 1;
    }
    candidate
}

pub fn create_instance_dirs(game_dir: &Path) -> YuhinaResult<()> {
    std::fs::create_dir_all(game_dir)
        .map_err(|e| YuhinaError::io(format!("mkdir {}: {e}", game_dir.display())))?;
    for sub in INSTANCE_SUBDIRS {
        std::fs::create_dir_all(game_dir.join(sub))
            .map_err(|e| YuhinaError::io(format!("mkdir {}/{}: {e}", game_dir.display(), sub)))?;
    }
    Ok(())
}

/// Recursively copy a directory tree.
pub fn copy_dir_contents(src: &Path, dst: &Path) -> YuhinaResult<()> {
    if !src.exists() {
        std::fs::create_dir_all(dst)
            .map_err(|e| YuhinaError::io(format!("mkdir {}: {e}", dst.display())))?;
        return Ok(());
    }
    std::fs::create_dir_all(dst)
        .map_err(|e| YuhinaError::io(format!("mkdir {}: {e}", dst.display())))?;
    for entry in std::fs::read_dir(src)
        .map_err(|e| YuhinaError::io(format!("read {}: {e}", src.display())))?
    {
        let entry = entry.map_err(|e| YuhinaError::io(e.to_string()))?;
        let from = entry.path();
        let to = dst.join(entry.file_name());
        if from.is_dir() {
            copy_dir_contents(&from, &to)?;
        } else if from.is_file() {
            std::fs::copy(&from, &to).map_err(|e| {
                YuhinaError::io(format!("copy {} -> {}: {e}", from.display(), to.display()))
            })?;
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use yuhina_api::{Loader, LoaderKind};

    use crate::testutil;

    fn svc() -> (InstanceService, tempfile::TempDir) {
        let (dir, game_root) = testutil::temp_game_root();
        let db = testutil::db();
        let core: Arc<dyn crate::CoreAdapter> = Arc::new(testutil::DummyCore);
        (InstanceService::new(db, core, game_root), dir)
    }

    fn req(name: &str, mc: &str, loader: Option<Loader>) -> CreateInstanceRequest {
        CreateInstanceRequest {
            name: name.into(),
            icon: String::new(),
            mc_version: mc.into(),
            loader,
            java: JavaSelection::Auto(21),
            dir_name: None,
        }
    }

    #[test]
    fn create_and_get() {
        let (svc, dir) = svc();
        let s = svc
            .create_instance(req("My Instance", "1.20.4", None))
            .unwrap();
        assert_eq!(s.name, "My Instance");
        assert!(!s.is_installed);
        let detail = svc.get_instance(&s.id).unwrap();
        assert_eq!(detail.summary.mc_version, "1.20.4");
        assert!(Path::new(&detail.game_dir).join("mods").exists());
        assert!(Path::new(&detail.game_dir).join("config").exists());
        let _ = dir;
    }

    #[test]
    fn dir_name_collision_gets_suffix() {
        let (svc, dir) = svc();
        let a = svc.create_instance(req("dup", "1.20.4", None)).unwrap();
        let b = svc.create_instance(req("dup", "1.20.4", None)).unwrap();
        let da = PathBuf::from(svc.get_instance(&a.id).unwrap().game_dir);
        let db_ = PathBuf::from(svc.get_instance(&b.id).unwrap().game_dir);
        assert_ne!(da, db_);
        assert!(
            db_.to_string_lossy().contains("dup-2"),
            "suffix expected, got {db_:?}"
        );
        let _ = dir;
    }

    #[test]
    fn explicit_dir_name() {
        let (svc, dir) = svc();
        let mut r = req("named", "1.20.4", None);
        r.dir_name = Some("custom_dir".into());
        let s = svc.create_instance(r).unwrap();
        let d = PathBuf::from(svc.get_instance(&s.id).unwrap().game_dir);
        assert!(d.to_string_lossy().ends_with("custom_dir"));
        let _ = dir;
    }

    #[test]
    fn invalid_mc_version_rejected() {
        let (svc, dir) = svc();
        let err = svc.create_instance(req("bad", "9.9.9", None)).unwrap_err();
        assert_eq!(err.kind, yuhina_api::YuhinaErrorKind::VersionNotFound);
        let _ = dir;
    }

    #[test]
    fn invalid_loader_combination_rejected() {
        let (svc, dir) = svc();
        let r = req(
            "bad",
            "1.12.2",
            Some(Loader {
                kind: LoaderKind::Fabric,
                version: "x".into(),
            }),
        );
        let err = svc.create_instance(r).unwrap_err();
        assert_eq!(err.kind, yuhina_api::YuhinaErrorKind::LoaderNotInstalled);
        let _ = dir;
    }

    #[test]
    fn rename_and_icon() {
        let (svc, dir) = svc();
        let s = svc.create_instance(req("r", "1.20.4", None)).unwrap();
        svc.rename_instance(&s.id, "Renamed".into()).unwrap();
        svc.set_instance_icon(&s.id, "🧱".into()).unwrap();
        let d = svc.get_instance(&s.id).unwrap();
        assert_eq!(d.summary.name, "Renamed");
        assert_eq!(d.summary.icon, "🧱");
        let _ = dir;
    }

    #[test]
    fn clone_copies_dir_and_mods() {
        let (svc, dir) = svc();
        let a = svc.create_instance(req("orig", "1.20.4", None)).unwrap();
        let ga = PathBuf::from(svc.get_instance(&a.id).unwrap().game_dir);
        std::fs::write(ga.join("mods/test.jar"), b"jar").unwrap();

        let cloned = svc.clone_instance(&a.id, "clone".to_string()).unwrap();
        let gc = PathBuf::from(svc.get_instance(&cloned.id).unwrap().game_dir);
        assert_ne!(ga, gc);
        assert!(gc.join("mods/test.jar").exists(), "dir deep-copied");
        assert_eq!(svc.list_instances().len(), 2);
        let _ = dir;
    }

    #[test]
    fn delete_removes_files_optionally() {
        let (svc, dir) = svc();
        let a = svc.create_instance(req("del", "1.20.4", None)).unwrap();
        let ga = PathBuf::from(svc.get_instance(&a.id).unwrap().game_dir);

        let b = svc.create_instance(req("keep", "1.20.4", None)).unwrap();
        let gb = PathBuf::from(svc.get_instance(&b.id).unwrap().game_dir);

        svc.delete_instance(&a.id, true).unwrap();
        assert!(!ga.exists());
        assert!(svc.get_instance(&a.id).is_err());

        svc.delete_instance(&b.id, false).unwrap();
        assert!(gb.exists(), "files kept when delete_files=false");
        assert_eq!(svc.list_instances().len(), 0);
        let _ = dir;
    }
}
