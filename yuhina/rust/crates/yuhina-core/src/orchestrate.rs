//! Game file download orchestration (task T6).
//!
//! Turns a parsed `VersionManifest` + resolved libraries/assets into a flat
//! list of [`DownloadItem`]s (URL → target path → sha1). Execution is
//! delegated to a [`Downloader`]; existing files are skipped.

use std::path::Path;

use tracing::{info, warn};
use yuhina_api::YuhinaError;

use crate::assets::{dedup_by_hash, plan_assets, AssetIndex};
use crate::config::CorePaths;
use crate::download::{sha1_file, DownloadItem, Downloader};
use crate::libraries::{resolve_libraries, Features, Platform};
use crate::manifest::VersionManifest;

/// The complete plan of files needed to run a version.
#[derive(Debug, Clone)]
pub struct GameFilePlan {
    pub client: DownloadItem,
    pub libraries: Vec<DownloadItem>,
    pub assets: Vec<DownloadItem>,
    pub logging: Option<DownloadItem>,
}

/// Build the file plan for a version (pure — no I/O beyond paths).
pub fn build_game_file_plan(
    manifest: &VersionManifest,
    platform: &Platform,
    paths: &CorePaths,
) -> GameFilePlan {
    let features = Features {
        has_custom_resolution: None,
        is_demo_user: None,
    };
    let resolved = resolve_libraries(&manifest.libraries, platform, &features);

    let client = DownloadItem {
        url: manifest
            .client
            .as_ref()
            .map(|c| c.url.clone())
            .unwrap_or_default(),
        target_path: paths.client_jar(&manifest.id).to_string_lossy().to_string(),
        sha1: manifest.client.as_ref().and_then(|c| c.sha1.clone()),
        size: manifest.client.as_ref().and_then(|c| c.size),
    };

    let libraries = resolved
        .iter()
        .map(|l| l.to_download_item(&paths.libraries_dir))
        .collect();

    let logging = manifest.logging.as_ref().map(|l| DownloadItem {
        url: l.url.clone(),
        target_path: paths
            .logging_config(&manifest.id)
            .to_string_lossy()
            .to_string(),
        sha1: l.sha1.clone(),
        size: l.size,
    });

    GameFilePlan {
        client,
        libraries,
        assets: Vec::new(),
        logging,
    }
}

/// Extend the plan with assets from an already-fetched asset index.
pub fn add_assets(plan: &mut GameFilePlan, index: &AssetIndex, objects_dir: &Path) {
    let files = dedup_by_hash(plan_assets(index));
    plan.assets = files
        .iter()
        .map(|f| f.to_download_item(objects_dir))
        .collect();
}

/// Download all missing files from the plan. Already-present files whose
/// sha1 matches (or which have no sha1) are skipped. Returns the count of
/// files actually downloaded.
///
/// The logging config is best-effort: some mirrors (e.g. BMCLAPI) do not
/// carry it, and the launch command never references it (the engine resolves
/// jvm tokens but has no `${path}` substitution), so a missing config must
/// not block the launch.
pub async fn ensure_downloaded(
    downloader: &dyn Downloader,
    plan: &GameFilePlan,
) -> Result<u32, YuhinaError> {
    let mut downloaded = 0u32;
    let mut items: Vec<&DownloadItem> = plan.libraries.iter().collect();
    items.push(&plan.client);
    items.extend(plan.assets.iter());

    for item in items {
        let path = Path::new(&item.target_path);
        if file_ok(path, item.sha1.as_deref()) {
            continue;
        }
        // Per-file retries: a single flaky connection must not abort the
        // whole install (already-downloaded files are skipped on re-run).
        let mut last_err = None;
        for attempt in 1..=3 {
            match downloader
                .download(&item.url, path, item.sha1.as_deref())
                .await
            {
                Ok(()) => {
                    last_err = None;
                    break;
                }
                Err(e) if attempt < 3 => {
                    last_err = Some(e);
                    warn!(url = %item.url, attempt, "download failed, retrying: {last_err:?}");
                    tokio::time::sleep(std::time::Duration::from_secs(1)).await;
                }
                Err(e) => {
                    last_err = Some(e);
                    break;
                }
            }
        }
        if let Some(e) = last_err {
            return Err(YuhinaError::download_failed(format!(
                "download {}: {e}",
                item.target_path
            )));
        }
        downloaded += 1;
    }

    if let Some(l) = &plan.logging {
        let path = Path::new(&l.target_path);
        if !file_ok(path, l.sha1.as_deref()) {
            match downloader.download(&l.url, path, l.sha1.as_deref()).await {
                Ok(()) => downloaded += 1,
                Err(e) => {
                    warn!(url = %l.url, "logging config unavailable (optional): {e}");
                }
            }
        }
    }
    info!(downloaded, "game files ensured");
    Ok(downloaded)
}

/// True when the file exists and (optionally) matches the expected sha1.
pub fn file_ok(path: &Path, expected_sha1: Option<&str>) -> bool {
    let Ok(actual) = sha1_file(path) else {
        return false;
    };
    match expected_sha1 {
        Some(expected) => actual.eq_ignore_ascii_case(expected),
        None => true,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testutil::{load_fixture, temp_game_root};

    fn paths() -> (CorePaths, std::path::PathBuf) {
        let root = temp_game_root();
        let paths = CorePaths {
            data_dir: root.join("data"),
            game_root: root.join("game"),
            versions_dir: root.join("data/versions"),
            instances_dir: root.join("data/instances"),
            libraries_dir: root.join("game/libraries"),
            assets_dir: root.join("game/assets"),
            assets_objects_dir: root.join("game/assets/objects"),
            logs_dir: root.join("data/logs"),
            db_path: root.join("data/yuhina.db"),
        };
        (paths, root)
    }

    #[test]
    fn plan_real_1204_includes_client_libs_assets() {
        let vj = load_fixture("1.20.4.json");
        let m = VersionManifest::parse(&vj).unwrap();
        let (paths, root) = paths();
        let platform = Platform {
            os: "linux".into(),
            arch: "x86_64".into(),
        };
        let mut plan = build_game_file_plan(&m, &platform, &paths);
        assert_eq!(
            plan.client.target_path,
            paths.client_jar("1.20.4").to_string_lossy()
        );
        assert!(plan.client.url.starts_with("https://"));
        assert!(plan.libraries.len() > 40, "libs {}", plan.libraries.len());
        // assets plan
        let idx_raw = load_fixture("assets_1.20.4.json");
        let idx = AssetIndex::parse("12", &idx_raw).unwrap();
        add_assets(&mut plan, &idx, &paths.assets_objects_dir);
        assert!(plan.assets.len() > 3000, "assets {}", plan.assets.len());
        // logging config
        assert!(plan.logging.is_some());
        assert!(plan.logging.as_ref().unwrap().url.starts_with("https://"));
        let _ = root;
    }

    #[test]
    fn file_ok_logic() {
        let dir = tempfile::tempdir().unwrap();
        let p = dir.path().join("x.bin");
        std::fs::write(&p, b"abc").unwrap();
        let hash = sha1_file(&p).unwrap();
        assert!(file_ok(&p, Some(&hash)));
        assert!(!file_ok(&p, Some("deadbeef")));
        assert!(!file_ok(&dir.path().join("missing"), Some(&hash)));
    }
}
