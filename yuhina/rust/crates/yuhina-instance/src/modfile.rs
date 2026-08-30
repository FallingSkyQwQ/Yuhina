//! Mod file lifecycle: install / enable / disable / delete (task T4).
//!
//! Enabled jars live in `<game_dir>/mods/<name>.jar`; disabled jars are moved
//! to `<game_dir>/mods/.disabled/<name>.jar` (atomic rename). The instance's
//! `installed_mods` rows are kept in sync; the public `InstalledMod.id` is the
//! file's sha1.

use std::path::{Path, PathBuf};
use std::sync::Arc;

use yuhina_api::{InstalledMod, ModrinthFile, YuhinaError, YuhinaResult};
use yuhina_core::download::Downloader;
use yuhina_db::Db;

use crate::modmeta;

pub const MODS_DIR: &str = "mods";
pub const DISABLED_DIR: &str = ".disabled";

/// `<mods_dir>/<name>`, suffixed with `-N` when an existing file of a
/// different content (sha1) already holds that name.
fn unique_mod_dest(mods_dir: &Path, file_name: &str, sha1: &str) -> YuhinaResult<PathBuf> {
    let candidate = mods_dir.join(file_name);
    if !candidate.exists() {
        return Ok(candidate);
    }
    if crate::sha1_hex_file(&candidate).ok().as_deref() == Some(sha1) {
        return Ok(candidate);
    }
    let stem = Path::new(file_name)
        .file_stem()
        .map(|s| s.to_string_lossy().to_string())
        .unwrap_or_else(|| "mod".into());
    let ext = Path::new(file_name)
        .extension()
        .map(|e| format!(".{}", e.to_string_lossy()))
        .unwrap_or_else(|| ".jar".into());
    let mut n = 2;
    loop {
        let name = format!("{stem}-{n}{ext}");
        let candidate = mods_dir.join(&name);
        if !candidate.exists() || crate::sha1_hex_file(&candidate).ok().as_deref() == Some(sha1) {
            return Ok(candidate);
        }
        n += 1;
    }
}

/// File-management service over an instance's mods directory.
#[derive(Clone)]
pub struct ModFileService {
    db: Db,
    downloader: Arc<dyn Downloader>,
}

impl ModFileService {
    pub fn new(db: Db, downloader: Arc<dyn Downloader>) -> Self {
        Self { db, downloader }
    }

    /// Absolute path of an instance's game directory.
    pub fn game_dir(&self, instance_id: &str) -> YuhinaResult<PathBuf> {
        self.db
            .instance_repo()
            .get_detail(instance_id)
            .map_err(|e| YuhinaError::internal(e.to_string()))?
            .map(|d| PathBuf::from(d.game_dir))
            .ok_or_else(|| {
                YuhinaError::invalid_instance(format!("instance {instance_id} not found"))
            })
    }

    pub fn list_mods(&self, instance_id: &str) -> Vec<InstalledMod> {
        self.db
            .installed_mod_repo()
            .list(instance_id)
            .unwrap_or_default()
    }

    /// Register a mod file that already exists at `src` (copied into the mods
    /// dir if not already there). Idempotent by sha1.
    pub fn install_mod_file(&self, instance_id: &str, src: &Path) -> YuhinaResult<InstalledMod> {
        let sha1 = crate::sha1_hex_file(src)?;
        let repo = self.db.installed_mod_repo();
        if let Some(existing) = repo.get_by_sha1(instance_id, &sha1)? {
            return Ok(existing);
        }
        let meta = modmeta::parse_mod_metadata(src);
        let file_name = src
            .file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_else(|| format!("{}.jar", meta.modid));

        let mods_dir = self.game_dir(instance_id)?.join(MODS_DIR);
        std::fs::create_dir_all(&mods_dir)
            .map_err(|e| YuhinaError::io(format!("mkdir {}: {e}", mods_dir.display())))?;
        // A different mod may already occupy this file name → suffix it so the
        // older row's file on disk is never clobbered by different content.
        let dest = unique_mod_dest(&mods_dir, &file_name, &sha1)?;
        if dest != src {
            std::fs::copy(src, &dest).map_err(|e| {
                YuhinaError::io(format!("copy {} -> {}: {e}", src.display(), dest.display()))
            })?;
        }
        let file_name = dest
            .file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or(file_name);
        let size = std::fs::metadata(&dest)
            .map_err(|e| YuhinaError::io(format!("stat {}: {e}", dest.display())))?
            .len();
        let m = InstalledMod {
            id: sha1.clone(),
            file_name,
            file_size: size,
            sha1,
            name: meta.name,
            modid: meta.modid,
            description: meta.description,
            loaders: meta.loaders,
            mc_versions: meta.mc_versions,
            project_id: None,
            version_id: None,
            enabled: true,
            installed_at: yuhina_db::now_millis() as u64,
        };
        repo.insert(instance_id, &m)?;
        Ok(m)
    }

    /// Download a Modrinth version file into the instance and register it,
    /// attaching the project/version linkage.
    pub async fn install_version_file(
        &self,
        instance_id: &str,
        file: &ModrinthFile,
        project_id: &str,
        version_id: &str,
    ) -> YuhinaResult<InstalledMod> {
        let mods_dir = self.game_dir(instance_id)?.join(MODS_DIR);
        std::fs::create_dir_all(&mods_dir)
            .map_err(|e| YuhinaError::io(format!("mkdir {}: {e}", mods_dir.display())))?;
        let dest = mods_dir.join(&file.name);
        self.downloader
            .download(&file.url, &dest, Some(&file.sha1))
            .await?;
        let mut m = self.install_mod_file(instance_id, &dest)?;
        let sha1 = m.sha1.clone();
        self.db.installed_mod_repo().set_modrinth(
            instance_id,
            &sha1,
            Some(project_id),
            Some(version_id),
        )?;
        m.project_id = Some(project_id.to_string());
        m.version_id = Some(version_id.to_string());
        Ok(m)
    }

    /// Move a mod between `mods/` and `mods/.disabled/` and update the row.
    pub fn set_mod_enabled(
        &self,
        instance_id: &str,
        mod_id: &str,
        enabled: bool,
    ) -> YuhinaResult<()> {
        let repo = self.db.installed_mod_repo();
        let m = repo.get(instance_id, mod_id)?.ok_or_else(|| {
            YuhinaError::invalid_instance(format!("mod {mod_id} is not installed"))
        })?;
        let game_dir = self.game_dir(instance_id)?;
        let mods_dir = game_dir.join(MODS_DIR);
        let disabled_dir = mods_dir.join(DISABLED_DIR);
        let src = if m.enabled {
            mods_dir.join(&m.file_name)
        } else {
            disabled_dir.join(&m.file_name)
        };
        let dst = if enabled {
            mods_dir.join(&m.file_name)
        } else {
            disabled_dir.join(&m.file_name)
        };
        if src.exists() {
            if dst.exists() {
                std::fs::remove_file(&dst)
                    .map_err(|e| YuhinaError::io(format!("remove {}: {e}", dst.display())))?;
            }
            if let Some(parent) = dst.parent() {
                std::fs::create_dir_all(parent)
                    .map_err(|e| YuhinaError::io(format!("mkdir {}: {e}", parent.display())))?;
            }
            std::fs::rename(&src, &dst).map_err(|e| {
                YuhinaError::io(format!("move {} -> {}: {e}", src.display(), dst.display()))
            })?;
        }
        repo.set_enabled(instance_id, mod_id, enabled)?;
        Ok(())
    }

    /// Remove the file (from either location) and the DB row.
    pub fn delete_mod(&self, instance_id: &str, mod_id: &str) -> YuhinaResult<()> {
        let repo = self.db.installed_mod_repo();
        if let Some(m) = repo.get(instance_id, mod_id)? {
            let game_dir = self.game_dir(instance_id)?;
            let mods_dir = game_dir.join(MODS_DIR);
            for path in [
                mods_dir.join(&m.file_name),
                mods_dir.join(DISABLED_DIR).join(&m.file_name),
            ] {
                if path.exists() {
                    std::fs::remove_file(&path)
                        .map_err(|e| YuhinaError::io(format!("remove {}: {e}", path.display())))?;
                }
            }
            repo.delete(instance_id, mod_id)?;
        }
        Ok(())
    }

    /// Scan the mods directory and register jars the DB does not know about
    /// (e.g. user-dropped files). Marks `.disabled/` files as disabled.
    pub fn rescan(&self, instance_id: &str) -> YuhinaResult<()> {
        let game_dir = self.game_dir(instance_id)?;
        let mods_dir = game_dir.join(MODS_DIR);
        let disabled_dir = mods_dir.join(DISABLED_DIR);
        let repo = self.db.installed_mod_repo();
        for (dir, enabled) in [(mods_dir, true), (disabled_dir, false)] {
            if !dir.exists() {
                continue;
            }
            for entry in std::fs::read_dir(&dir)
                .map_err(|e| YuhinaError::io(format!("read dir {}: {e}", dir.display())))?
            {
                let path = entry.map_err(|e| YuhinaError::io(e.to_string()))?.path();
                let is_jar = path
                    .extension()
                    .map(|e| e.to_string_lossy().eq_ignore_ascii_case("jar"))
                    .unwrap_or(false);
                if !is_jar {
                    continue;
                }
                let sha1 = crate::sha1_hex_file(&path)?;
                if repo.get_by_sha1(instance_id, &sha1)?.is_some() {
                    continue;
                }
                let meta = modmeta::parse_mod_metadata(&path);
                let file_name = path
                    .file_name()
                    .map(|n| n.to_string_lossy().to_string())
                    .unwrap_or_default();
                let size = std::fs::metadata(&path)
                    .map_err(|e| YuhinaError::io(e.to_string()))?
                    .len();
                let m = InstalledMod {
                    id: sha1.clone(),
                    file_name,
                    file_size: size,
                    sha1,
                    name: meta.name,
                    modid: meta.modid,
                    description: meta.description,
                    loaders: meta.loaders,
                    mc_versions: meta.mc_versions,
                    project_id: None,
                    version_id: None,
                    enabled,
                    installed_at: yuhina_db::now_millis() as u64,
                };
                repo.insert(instance_id, &m)?;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;

    use crate::instance::InstanceService;
    use crate::testutil;

    /// Build a tiny jar whose fabric.mod.json marks it as a fabric mod.
    fn make_jar(dir: &Path, name: &str, modid: &str) -> PathBuf {
        let path = dir.join(name);
        let file = File::create(&path).unwrap();
        let mut zip = zip::ZipWriter::new(file);
        zip.start_file("fabric.mod.json", zip::write::SimpleFileOptions::default())
            .unwrap();
        std::io::Write::write_all(
            &mut zip,
            format!(r#"{{"id":"{modid}","name":"{modid}","depends":{{"minecraft":["1.20.4"]}}}}"#)
                .as_bytes(),
        )
        .unwrap();
        zip.finish().unwrap();
        path
    }

    fn setup() -> (ModFileService, String, tempfile::TempDir) {
        let (dir, game_root) = testutil::temp_game_root();
        let db = testutil::db();
        let core: std::sync::Arc<dyn crate::CoreAdapter> =
            std::sync::Arc::new(testutil::AnyVersionCore);
        let inst = InstanceService::new(db.clone(), core, game_root);
        let s = inst
            .create_instance(testutil::create_request("t", "1.20.4"))
            .unwrap();
        let dl: std::sync::Arc<dyn Downloader> = std::sync::Arc::new(testutil::NoopDownloader);
        (ModFileService::new(db, dl), s.id, dir)
    }

    #[test]
    fn install_and_toggle() {
        let (svc, instance_id, dir) = setup();
        let jar = make_jar(dir.path(), "mymod.jar", "mymod");
        let m = svc.install_mod_file(&instance_id, &jar).unwrap();
        assert_eq!(m.modid, "mymod");
        assert_eq!(m.name, "mymod");
        assert_eq!(m.loaders, vec!["fabric"]);

        let game = svc.game_dir(&instance_id).unwrap();
        assert!(game.join("mods/mymod.jar").exists());

        // disable → moves into .disabled
        svc.set_mod_enabled(&instance_id, &m.id, false).unwrap();
        assert!(!game.join("mods/mymod.jar").exists());
        assert!(game.join("mods/.disabled/mymod.jar").exists());
        assert!(!svc.list_mods(&instance_id)[0].enabled);

        // enable → moves back
        svc.set_mod_enabled(&instance_id, &m.id, true).unwrap();
        assert!(game.join("mods/mymod.jar").exists());
        assert!(!game.join("mods/.disabled/mymod.jar").exists());
        assert!(svc.list_mods(&instance_id)[0].enabled);

        // idempotent install returns the same row
        let again = svc.install_mod_file(&instance_id, &jar).unwrap();
        assert_eq!(again.id, m.id);
        assert_eq!(svc.list_mods(&instance_id).len(), 1);
    }

    #[test]
    fn delete_removes_file_and_row() {
        let (svc, instance_id, dir) = setup();
        let jar = make_jar(dir.path(), "bye.jar", "bye");
        let m = svc.install_mod_file(&instance_id, &jar).unwrap();
        let game = svc.game_dir(&instance_id).unwrap();
        svc.delete_mod(&instance_id, &m.id).unwrap();
        assert!(!game.join("mods/bye.jar").exists());
        assert!(svc.list_mods(&instance_id).is_empty());
    }

    #[test]
    fn rescan_picks_up_dropped_jars() {
        let (svc, instance_id, dir) = setup();
        let game = svc.game_dir(&instance_id).unwrap();
        let mods = game.join("mods");
        std::fs::create_dir_all(&mods).unwrap();
        make_jar(&mods, "dropped.jar", "dropped");
        std::fs::create_dir_all(mods.join(".disabled")).unwrap();
        make_jar(&mods.join(".disabled"), "off.jar", "off");

        svc.rescan(&instance_id).unwrap();
        let rows = svc.list_mods(&instance_id);
        assert_eq!(rows.len(), 2);
        let on = rows.iter().find(|m| m.file_name == "dropped.jar").unwrap();
        let off = rows.iter().find(|m| m.file_name == "off.jar").unwrap();
        assert!(on.enabled);
        assert!(!off.enabled);
        let _ = dir;
    }

    #[test]
    fn different_content_same_name_gets_suffix() {
        let (svc, instance_id, dir) = setup();
        let first = make_jar(dir.path(), "conflict.jar", "first");
        let m1 = svc.install_mod_file(&instance_id, &first).unwrap();
        assert_eq!(m1.file_name, "conflict.jar");

        // A second mod with a different modid but the same file name.
        let second = make_jar(dir.path(), "conflict.jar", "second");
        let m2 = svc.install_mod_file(&instance_id, &second).unwrap();
        assert_ne!(m2.id, m1.id);
        assert_eq!(m2.file_name, "conflict-2.jar", "suffixed to avoid clobber");

        let game = svc.game_dir(&instance_id).unwrap();
        assert!(game.join("mods/conflict.jar").exists());
        assert!(game.join("mods/conflict-2.jar").exists());
        assert_eq!(svc.list_mods(&instance_id).len(), 2);
        let _ = dir;
    }
}
