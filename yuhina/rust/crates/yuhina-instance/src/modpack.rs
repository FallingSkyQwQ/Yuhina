//! `.mrpack` import/export + Modrinth pack install (task T7/T8).
//!
//! Export writes `index.json` (formatVersion 1) plus an `overrides/` tree.
//! Modrinth-linked mods are referenced by sha1 + download URL; local mods get
//! a local hash and empty downloads (their content travels in overrides).
//! Import recreates the instance, downloads+verifies files and applies
//! `env.client.force/optional/unsupported`.

use std::path::{Path, PathBuf};
use std::sync::Arc;

use serde::{Deserialize, Serialize};
use yuhina_api::{
    CreateInstanceRequest, InstanceSummary, JavaSelection, Loader, LoaderKind, YuhinaError,
    YuhinaResult,
};
use yuhina_core::download::Downloader;
use yuhina_db::Db;

use crate::instance::InstanceService;
use crate::modfile::ModFileService;
use crate::modrinth::ModrinthClient;

/// `.mrpack` import/export service.
#[derive(Clone)]
pub struct ModpackService {
    db: Db,
    core: Arc<dyn crate::CoreAdapter>,
    downloader: Arc<dyn Downloader>,
    modrinth: ModrinthClient,
    game_root: PathBuf,
}

impl ModpackService {
    pub fn new(
        db: Db,
        core: Arc<dyn crate::CoreAdapter>,
        downloader: Arc<dyn Downloader>,
        modrinth: ModrinthClient,
        game_root: PathBuf,
    ) -> Self {
        Self {
            db,
            core,
            downloader,
            modrinth,
            game_root,
        }
    }

    // -----------------------------------------------------------------
    // Export
    // -----------------------------------------------------------------

    /// Write a `.mrpack` for `instance_id` to `dest_path`. Returns the path.
    pub async fn export_modpack(&self, instance_id: &str, dest_path: &str) -> YuhinaResult<String> {
        let detail = self
            .db
            .instance_repo()
            .get_detail(instance_id)
            .map_err(|e| YuhinaError::internal(e.to_string()))?
            .ok_or_else(|| {
                YuhinaError::invalid_instance(format!("instance {instance_id} not found"))
            })?;
        let summary = &detail.summary;
        let game_dir = PathBuf::from(&detail.game_dir);
        let mods_dir = game_dir.join("mods");
        let disabled_dir = mods_dir.join(".disabled");
        let installed = self.db.installed_mod_repo().list(instance_id)?;

        let mut files: Vec<PackFile> = Vec::new();
        // Local-only mods (no download url) must travel inside overrides so a
        // re-import can recover their content.
        let mut local_mod_overrides: Vec<(PathBuf, String)> = Vec::new();
        for (dir, optional) in [(mods_dir, false), (disabled_dir, true)] {
            if !dir.exists() {
                continue;
            }
            for entry in std::fs::read_dir(&dir)
                .map_err(|e| YuhinaError::io(format!("read dir {}: {e}", dir.display())))?
            {
                let path = entry.map_err(|e| YuhinaError::io(e.to_string()))?.path();
                if path.is_dir() {
                    continue;
                }
                let Some(file) = self.pack_file_for(&path, &installed, optional).await? else {
                    continue;
                };
                if file.downloads.is_empty() {
                    if let Some(name) = path.file_name().map(|n| n.to_string_lossy().to_string()) {
                        local_mod_overrides.push((path, format!("overrides/mods/{name}")));
                    }
                }
                files.push(file);
            }
        }

        let modloaders = summary
            .loader
            .as_ref()
            .map(|l| {
                vec![PackModLoader {
                    id: format!("{}-{}", loader_tag(l.kind), l.version),
                }]
            })
            .unwrap_or_default();

        let index = PackIndex {
            format_version: 1,
            game: "minecraft".into(),
            version_id: summary.mc_version.clone(),
            name: summary.name.clone(),
            files,
            modloaders,
            overrides: "overrides".into(),
        };

        let file = std::fs::File::create(dest_path)
            .map_err(|e| YuhinaError::io(format!("create {}: {e}", dest_path)))?;
        let mut zip = zip::ZipWriter::new(file);
        let opts = zip::write::SimpleFileOptions::default()
            .compression_method(zip::CompressionMethod::Deflated);
        let index_json = serde_json::to_string_pretty(&index)
            .map_err(|e| YuhinaError::internal(e.to_string()))?;
        zip.start_file("index.json", opts)
            .map_err(|e| YuhinaError::io(e.to_string()))?;
        std::io::Write::write_all(&mut zip, index_json.as_bytes())
            .map_err(|e| YuhinaError::io(e.to_string()))?;

        self.write_overrides(&mut zip, &game_dir, "overrides")?;
        let opts = zip::write::SimpleFileOptions::default()
            .compression_method(zip::CompressionMethod::Deflated);
        for (src, zip_path) in local_mod_overrides {
            zip.start_file(&zip_path, opts)
                .map_err(|e| YuhinaError::io(e.to_string()))?;
            let bytes = std::fs::read(&src)
                .map_err(|e| YuhinaError::io(format!("read {}: {e}", src.display())))?;
            std::io::Write::write_all(&mut zip, &bytes)
                .map_err(|e| YuhinaError::io(e.to_string()))?;
        }
        zip.finish().map_err(|e| YuhinaError::io(e.to_string()))?;
        Ok(dest_path.to_string())
    }

    async fn pack_file_for(
        &self,
        path: &Path,
        installed: &[yuhina_api::InstalledMod],
        optional: bool,
    ) -> YuhinaResult<Option<PackFile>> {
        let Some(name) = path.file_name().map(|n| n.to_string_lossy().to_string()) else {
            return Ok(None);
        };
        let local_sha1 = crate::sha1_hex_file(path)?;
        let size = std::fs::metadata(path)
            .map_err(|e| YuhinaError::io(e.to_string()))?
            .len();

        // Prefer the Modrinth download URL when this file has a linkage.
        let mut downloads: Vec<String> = Vec::new();
        let mut sha1 = local_sha1.clone();
        if let Some(m) = installed.iter().find(|m| m.sha1 == local_sha1) {
            if let (Some(_pid), Some(vid)) = (&m.project_id, &m.version_id) {
                if let Ok((_, version)) = self.modrinth.get_version_with_project(vid).await {
                    if let Some(file) = version.files.first() {
                        downloads = vec![file.url.clone()];
                        if !file.sha1.is_empty() {
                            sha1 = file.sha1.clone();
                        }
                    }
                }
            }
        }

        let env = PackEnv {
            client: Some(if optional {
                "optional".into()
            } else {
                "required".into()
            }),
            server: None,
        };
        Ok(Some(PackFile {
            path: format!("mods/{name}"),
            hashes: PackHashes { sha1 },
            env: Some(env),
            downloads,
            file_size: Some(size),
        }))
    }

    fn write_overrides(
        &self,
        zip: &mut zip::ZipWriter<std::fs::File>,
        game_dir: &Path,
        prefix: &str,
    ) -> YuhinaResult<()> {
        let opts = zip::write::SimpleFileOptions::default()
            .compression_method(zip::CompressionMethod::Deflated);
        let mut queue: Vec<(PathBuf, String)> = vec![(game_dir.to_path_buf(), prefix.to_string())];
        while let Some((dir, zip_prefix)) = queue.pop() {
            for entry in std::fs::read_dir(&dir)
                .map_err(|e| YuhinaError::io(format!("read {}: {e}", dir.display())))?
            {
                let entry = entry.map_err(|e| YuhinaError::io(e.to_string()))?;
                let name = entry.file_name().to_string_lossy().to_string();
                let src = entry.path();
                if src.is_dir() {
                    if name == "mods" {
                        continue; // mods live in index.json files[]
                    }
                    queue.push((src, format!("{zip_prefix}/{name}")));
                } else {
                    let zip_path = format!("{zip_prefix}/{name}");
                    zip.start_file(&zip_path, opts)
                        .map_err(|e| YuhinaError::io(e.to_string()))?;
                    let bytes = std::fs::read(&src)
                        .map_err(|e| YuhinaError::io(format!("read {}: {e}", src.display())))?;
                    std::io::Write::write_all(zip, &bytes)
                        .map_err(|e| YuhinaError::io(e.to_string()))?;
                }
            }
        }
        Ok(())
    }

    // -----------------------------------------------------------------
    // Import
    // -----------------------------------------------------------------

    /// Import a `.mrpack`: unpack, create the instance, download/verify mods,
    /// apply overrides and `env.client` semantics.
    pub async fn import_modpack(
        &self,
        mrpack_path: &str,
        name: &str,
    ) -> YuhinaResult<InstanceSummary> {
        let file = std::fs::File::open(mrpack_path)
            .map_err(|e| YuhinaError::modpack_invalid(format!("open {mrpack_path}: {e}")))?;
        let mut archive = zip::ZipArchive::new(file)
            .map_err(|e| YuhinaError::modpack_invalid(format!("unzip {mrpack_path}: {e}")))?;

        let mut index_json = String::new();
        {
            let mut entry = archive
                .by_name("index.json")
                .map_err(|_| YuhinaError::modpack_invalid("mrpack missing index.json"))?;
            std::io::Read::read_to_string(&mut entry, &mut index_json)
                .map_err(|e| YuhinaError::io(e.to_string()))?;
        }
        let index: PackIndex = serde_json::from_str(&index_json)
            .map_err(|e| YuhinaError::modpack_invalid(format!("bad index.json: {e}")))?;
        if index.format_version != 1 {
            return Err(YuhinaError::modpack_invalid(format!(
                "unsupported mrpack formatVersion {}",
                index.format_version
            )));
        }

        let loader = index
            .modloaders
            .first()
            .and_then(|m| parse_modloader(&m.id));
        let inst_svc = InstanceService::new(
            self.db.clone(),
            Arc::clone(&self.core),
            self.game_root.clone(),
        );
        let req = CreateInstanceRequest {
            name: name.to_string(),
            icon: String::new(),
            mc_version: index.version_id.clone(),
            loader,
            java: JavaSelection::Auto(21),
            dir_name: None,
        };
        let summary = inst_svc.create_instance_unchecked(req)?;

        let modfile = ModFileService::new(self.db.clone(), Arc::clone(&self.downloader));
        let game_dir = modfile.game_dir(&summary.id)?;

        // Apply overrides first so local-only files (empty downloads) are
        // already present when the files[] loop runs.
        self.apply_overrides(&mut archive, &index.overrides, &game_dir)?;

        // Download mod files into the instance.
        let mut warnings: Vec<String> = Vec::new();
        for pf in &index.files {
            if !pf.path.starts_with("mods/") {
                continue;
            }
            let client = pf.env.as_ref().and_then(|e| e.client.as_deref());
            if client == Some("unsupported") {
                continue; // server-only
            }
            let rel = pf.path.trim_start_matches("mods/");
            let dest = safe_join(&game_dir, &pf.path)?;
            let Some(parent) = dest.parent() else {
                continue;
            };
            std::fs::create_dir_all(parent)
                .map_err(|e| YuhinaError::io(format!("mkdir {}: {e}", parent.display())))?;
            if let Some(url) = pf.downloads.first() {
                if let Err(e) = self
                    .downloader
                    .download(url, &dest, Some(&pf.hashes.sha1))
                    .await
                {
                    warnings.push(format!("skip {}: {e}", pf.path));
                    let _ = std::fs::remove_file(&dest);
                    continue;
                }
            } else if !dest.exists() {
                // Local-only file: content must come from overrides.
                warnings.push(format!("local file {} has no download url", pf.path));
                continue;
            }
            // env.client.optional → park in .disabled
            if client == Some("optional") {
                let disabled = game_dir.join("mods").join(".disabled").join(rel);
                if let Some(p) = disabled.parent() {
                    std::fs::create_dir_all(p)
                        .map_err(|e| YuhinaError::io(format!("mkdir {}: {e}", p.display())))?;
                }
                if dest.exists() {
                    std::fs::rename(&dest, &disabled).map_err(|e| {
                        YuhinaError::io(format!(
                            "move {} -> {}: {e}",
                            dest.display(),
                            disabled.display()
                        ))
                    })?;
                }
            }
            let _ = rel;
        }

        // Register jars from mods/ + mods/.disabled/.
        modfile.rescan(&summary.id)?;

        if !warnings.is_empty() {
            tracing::warn!(instance_id = %summary.id, "mrpack import warnings: {:?}", warnings);
        }
        Ok(summary)
    }

    /// Download a Modrinth modpack version and import it.
    pub async fn download_modpack_from_modrinth(
        &self,
        project_id: &str,
        version_id: &str,
    ) -> YuhinaResult<InstanceSummary> {
        let (_, version) = self.modrinth.get_version_with_project(version_id).await?;
        let file = version
            .files
            .iter()
            .find(|f| f.name.to_lowercase().ends_with(".mrpack"))
            .or_else(|| version.files.first())
            .ok_or_else(|| {
                YuhinaError::modpack_invalid(format!(
                    "version {version_id} has no downloadable file"
                ))
            })?
            .clone();

        let project = self.modrinth.get_project(project_id).await?;
        let tmp = std::env::temp_dir().join(format!("yuhina-pack-{version_id}.mrpack"));
        self.downloader
            .download(&file.url, &tmp, Some(&file.sha1))
            .await?;
        let result = self
            .import_modpack(tmp.to_string_lossy().as_ref(), &project.title)
            .await;
        let _ = std::fs::remove_file(&tmp);
        result
    }

    fn apply_overrides(
        &self,
        archive: &mut zip::ZipArchive<std::fs::File>,
        overrides: &str,
        game_dir: &Path,
    ) -> YuhinaResult<()> {
        let prefix = overrides.trim_matches('/');
        for i in 0..archive.len() {
            let mut entry = match archive.by_index(i) {
                Ok(e) => e,
                Err(_) => continue,
            };
            let name = entry.name().to_string();
            if entry.is_dir() || !name.starts_with(&format!("{prefix}/")) {
                continue;
            }
            let rel = name.trim_start_matches(&format!("{prefix}/"));
            let dest = safe_join(game_dir, rel)?;
            if let Some(parent) = dest.parent() {
                std::fs::create_dir_all(parent)
                    .map_err(|e| YuhinaError::io(format!("mkdir {}: {e}", parent.display())))?;
            }
            let mut out = std::fs::File::create(&dest)
                .map_err(|e| YuhinaError::io(format!("create {}: {e}", dest.display())))?;
            std::io::copy(&mut entry, &mut out)
                .map_err(|e| YuhinaError::io(format!("extract {name}: {e}")))?;
        }
        Ok(())
    }
}

// ---------------------------------------------------------------------------
// mrpack index.json model
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
struct PackIndex {
    #[serde(rename = "formatVersion", default)]
    format_version: u32,
    #[serde(default = "default_game")]
    game: String,
    #[serde(rename = "versionId")]
    version_id: String,
    name: String,
    #[serde(default)]
    files: Vec<PackFile>,
    #[serde(default)]
    modloaders: Vec<PackModLoader>,
    #[serde(default = "default_overrides")]
    overrides: String,
}

fn default_game() -> String {
    "minecraft".into()
}

fn default_overrides() -> String {
    "overrides".into()
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct PackFile {
    path: String,
    hashes: PackHashes,
    #[serde(default)]
    env: Option<PackEnv>,
    #[serde(default)]
    downloads: Vec<String>,
    #[serde(rename = "fileSize", default)]
    file_size: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct PackHashes {
    sha1: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct PackEnv {
    client: Option<String>,
    server: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct PackModLoader {
    id: String,
}

fn loader_tag(kind: LoaderKind) -> &'static str {
    crate::dependency::loader_tag(kind)
}

/// Parse a modloader id like `fabric-0.16.0` / `neoforge-20.4.237`.
fn parse_modloader(id: &str) -> Option<Loader> {
    let (kind, version) = id.split_once('-')?;
    let kind = match kind {
        "fabric" => LoaderKind::Fabric,
        "quilt" => LoaderKind::Quilt,
        "forge" => LoaderKind::Forge,
        "neoforge" => LoaderKind::NeoForge,
        _ => return None,
    };
    Some(Loader {
        kind,
        version: version.to_string(),
    })
}

/// Join `base` + a relative path, rejecting traversal / absolute escapes.
fn safe_join(base: &Path, rel: &str) -> YuhinaResult<PathBuf> {
    let rel = Path::new(rel);
    if rel.is_absolute()
        || rel
            .components()
            .any(|c| matches!(c, std::path::Component::ParentDir))
    {
        return Err(YuhinaError::modpack_invalid(format!(
            "unsafe path '{}' in mrpack",
            rel.display()
        )));
    }
    Ok(base.join(rel))
}
