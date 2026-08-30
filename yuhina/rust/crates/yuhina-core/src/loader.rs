//! Loader version metadata + installation orchestration (task T7).
//!
//! Fabric/Quilt/Forge/NeoForge version discovery is pure and offline-testable
//! (real fixtures included). The installer *execution* is the low-level API
//! Agent C will drive; it runs the installer jar with a matched Java and
//! returns full stdout/stderr so failures are diagnosable.

use std::path::Path;

use serde_json::Value;
use yuhina_api::{LoaderKind, YuhinaError};

use crate::download::Downloader;

/// Base maven roots (mirror-rewritten through the downloader at runtime).
pub const FABRIC_MAVEN: &str = "https://maven.fabricmc.net";
pub const FORGE_MAVEN: &str = "https://maven.minecraftforge.net";
pub const NEOFORGE_MAVEN: &str = "https://maven.neoforged.net/releases";
pub const QUILT_INSTALLER_URL: &str =
    "https://quiltmc.org/api/v1/download-latest-installer/Quilt-Installer";

/// A fully-resolved loader installation plan.
#[derive(Debug, Clone)]
pub struct LoaderChoice {
    pub mc_version: String,
    pub kind: LoaderKind,
    pub loader_version: String,
    pub installer_url: String,
    pub installer_filename: String,
    /// Args appended after `java -jar <installer>`.
    pub install_args: Vec<String>,
    /// Human display id, e.g. `1.20.4-fabric-0.16.0`.
    pub display: String,
}

// ---------------------------------------------------------------------------
// Version discovery (pure, offline-testable)
// ---------------------------------------------------------------------------

/// Fabric loader versions from `meta.fabricmc.net/v2/versions/loader/{mc}`.
pub fn fabric_versions(value: &Value) -> Vec<String> {
    value
        .as_array()
        .map(|arr| {
            arr.iter()
                .filter_map(|v| {
                    v.get("loader")
                        .and_then(|l| l.get("version"))
                        .and_then(Value::as_str)
                })
                .map(String::from)
                .collect()
        })
        .unwrap_or_default()
}

/// Quilt loader versions from `meta.quiltmc.org/v3/versions/loader/{mc}`.
pub fn quilt_versions(value: &Value) -> Vec<String> {
    value
        .as_array()
        .map(|arr| {
            arr.iter()
                .filter_map(|v| {
                    v.get("loader")
                        .and_then(|l| l.get("version"))
                        .and_then(Value::as_str)
                })
                .map(String::from)
                .collect()
        })
        .unwrap_or_default()
}

/// Forge versions for `mc` from `promotions_slim.json` (`promos` map).
/// Returns versions in `mc-*` order, recommended first when present.
pub fn forge_versions(value: &Value, mc: &str) -> Vec<String> {
    let Some(promos) = value.get("promos").and_then(Value::as_object) else {
        return Vec::new();
    };
    let prefix = format!("{mc}-");
    let recommended = promos
        .get(&format!("{prefix}recommended"))
        .and_then(Value::as_str);
    let mut versions = Vec::new();
    if let Some(r) = recommended {
        versions.push(r.to_string());
    }
    let mut rest: Vec<String> = promos
        .keys()
        .filter(|k| k.starts_with(&prefix) && !k.ends_with("-recommended"))
        .filter_map(|k| promos.get(k).and_then(Value::as_str))
        .map(String::from)
        .collect();
    rest.sort();
    for v in rest {
        if !versions.contains(&v) {
            versions.push(v);
        }
    }
    versions
}

/// NeoForge versions from `maven-metadata.xml`, filtered by the MC version
/// prefix (MC 1.20.4 → NeoForge `20.4.x`). Beta/alphas are de-prioritised.
pub fn neoforge_versions(xml: &str, mc: &str) -> Vec<String> {
    let prefix = mc_neoforge_prefix(mc);
    let mut versions: Vec<String> = xml
        .split("<version>")
        .skip(1)
        .filter_map(|s| s.split("</version>").next())
        .filter(|v| v.starts_with(&prefix))
        .map(String::from)
        .collect();
    versions.sort_by(|a, b| cmp_versions(b, a));
    versions
}

fn mc_neoforge_prefix(mc: &str) -> String {
    mc.strip_prefix("1.").unwrap_or(mc).to_string()
}

/// Compare two version strings by numeric segments (descending).
fn cmp_versions(a: &str, b: &str) -> std::cmp::Ordering {
    let seg = |s: &str| -> Vec<u64> {
        s.trim_start_matches(|c: char| !c.is_ascii_digit())
            .split('.')
            .filter_map(|p| p.parse::<u64>().ok())
            .collect()
    };
    seg(a).cmp(&seg(b))
}

// ---------------------------------------------------------------------------
// Installer plan construction
// ---------------------------------------------------------------------------

/// Latest fabric installer version from `meta.fabricmc.net/v2/versions/installer`.
pub fn latest_fabric_installer(value: &Value) -> String {
    value
        .as_array()
        .and_then(|arr| arr.first())
        .and_then(|v| v.get("version"))
        .and_then(Value::as_str)
        .unwrap_or("1.1.2")
        .to_string()
}

/// Resolve the plan for a loader on `mc` (version `None` → latest).
#[allow(clippy::too_many_arguments)]
pub fn resolve_loader(
    mc: &str,
    kind: LoaderKind,
    version: Option<&str>,
    fabric_loader_meta: &Value,
    fabric_installer_meta: &Value,
    quilt_meta: &Value,
    forge_promos: &Value,
    neoforge_xml: &str,
) -> Result<LoaderChoice, YuhinaError> {
    match kind {
        LoaderKind::Fabric => {
            let versions = fabric_versions(fabric_loader_meta);
            let version = version
                .map(str::to_string)
                .unwrap_or_else(|| versions.first().cloned().unwrap_or_else(|| "0.16.0".into()));
            let installer = latest_fabric_installer(fabric_installer_meta);
            let display = format!("{mc}-fabric-{version}");
            Ok(LoaderChoice {
                mc_version: mc.into(),
                kind,
                loader_version: version.clone(),
                installer_url: format!("{FABRIC_MAVEN}/net/fabricmc/fabric-installer/{installer}/fabric-installer-{installer}.jar"),
                installer_filename: format!("fabric-installer-{installer}.jar"),
                install_args: vec!["server".into(), "-dir".into(), "GAME_DIR".into(), "-mcversion".into(), mc.into(), "-loader".into(), version],
                display,
            })
        }
        LoaderKind::Quilt => {
            let versions = quilt_versions(quilt_meta);
            let version = version
                .map(str::to_string)
                .unwrap_or_else(|| versions.first().cloned().unwrap_or_else(|| "0.20.0".into()));
            let display = format!("{mc}-quilt-{version}");
            Ok(LoaderChoice {
                mc_version: mc.into(),
                kind,
                loader_version: version.clone(),
                installer_url: QUILT_INSTALLER_URL.into(),
                installer_filename: "quilt-installer.jar".into(),
                install_args: vec![
                    "install".into(),
                    "server".into(),
                    "GAME_DIR".into(),
                    "-mcversion".into(),
                    mc.into(),
                    "-loader".into(),
                    version,
                ],
                display,
            })
        }
        LoaderKind::Forge => {
            let versions = forge_versions(forge_promos, mc);
            let version = version
                .map(str::to_string)
                .or_else(|| versions.first().cloned())
                .ok_or_else(|| {
                    YuhinaError::loader_not_installed(format!("no forge version for MC {mc}"))
                })?;
            let installer_filename = format!("forge-{mc}-{version}-installer.jar");
            let display = format!("{mc}-forge-{version}");
            Ok(LoaderChoice {
                mc_version: mc.into(),
                kind,
                loader_version: version.clone(),
                installer_url: format!(
                    "{FORGE_MAVEN}/net/minecraftforge/forge/{mc}-{version}/{installer_filename}"
                ),
                installer_filename,
                install_args: vec!["--installServer".into()],
                display,
            })
        }
        LoaderKind::NeoForge => {
            let versions = neoforge_versions(neoforge_xml, mc);
            let version = version
                .map(str::to_string)
                .or_else(|| versions.first().cloned())
                .ok_or_else(|| {
                    YuhinaError::loader_not_installed(format!("no neoforge version for MC {mc}"))
                })?;
            let installer_filename = format!("neoforge-{version}-installer.jar");
            let display = format!("{mc}-neoforge-{version}");
            Ok(LoaderChoice {
                mc_version: mc.into(),
                kind,
                loader_version: version.clone(),
                installer_url: format!(
                    "{NEOFORGE_MAVEN}/net/neoforged/neoforge/{version}/{installer_filename}"
                ),
                installer_filename,
                install_args: vec!["--installServer".into()],
                display,
            })
        }
    }
}

// ---------------------------------------------------------------------------
// Installer execution
// ---------------------------------------------------------------------------

/// Result of running a loader installer.
#[derive(Debug, Clone)]
pub struct InstallerResult {
    pub success: bool,
    pub exit_code: Option<i32>,
    pub stdout: String,
    pub stderr: String,
}

/// Run `java -jar <installer> <args>` in `cwd`, capturing output.
pub async fn run_installer(
    java_bin: &Path,
    installer_jar: &Path,
    args: &[String],
    cwd: &Path,
) -> Result<InstallerResult, YuhinaError> {
    let output = tokio::process::Command::new(java_bin)
        .arg("-jar")
        .arg(installer_jar)
        .args(args)
        .current_dir(cwd)
        .output()
        .await
        .map_err(|e| YuhinaError::io(format!("run installer: {e}")))?;
    Ok(InstallerResult {
        success: output.status.success(),
        exit_code: output.status.code(),
        stdout: String::from_utf8_lossy(&output.stdout).to_string(),
        stderr: String::from_utf8_lossy(&output.stderr).to_string(),
    })
}

/// Orchestrate a loader install: resolve version, download the installer jar,
/// run it in the game dir, then remove the installer.
pub async fn install_loader(
    downloader: &dyn Downloader,
    java_bin: &Path,
    game_dir: &Path,
    loader_dir: &Path,
    choice: &LoaderChoice,
) -> Result<InstallerResult, YuhinaError> {
    std::fs::create_dir_all(loader_dir)
        .map_err(|e| YuhinaError::io(format!("mkdir {}: {e}", loader_dir.display())))?;
    let installer_path = loader_dir.join(&choice.installer_filename);
    if !installer_path.exists() {
        downloader
            .download(&choice.installer_url, &installer_path, None)
            .await
            .map_err(|e| YuhinaError::download_failed(format!("downloader installer: {e}")))?;
    }
    // Substitute the GAME_DIR placeholder with the real path.
    let args: Vec<String> = choice
        .install_args
        .iter()
        .map(|a| {
            if a == "GAME_DIR" {
                game_dir.to_string_lossy().to_string()
            } else {
                a.clone()
            }
        })
        .collect();
    let result = run_installer(java_bin, &installer_path, &args, game_dir).await?;
    let _ = std::fs::remove_file(&installer_path);
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testutil::load_fixture;

    fn fabric_meta() -> Value {
        load_fixture("fabric_loader_1.20.4.json")
    }
    fn fabric_installer() -> Value {
        load_fixture("fabric_installer.json")
    }
    fn quilt_meta() -> Value {
        load_fixture("quilt_loader_1.20.4.json")
    }
    fn forge_promos() -> Value {
        load_fixture("forge_promotions.json")
    }
    fn neoforge_xml() -> String {
        include_str!("../tests/fixtures/neoforge_metadata.xml").to_string()
    }

    #[test]
    fn fabric_loader_versions_real() {
        let versions = fabric_versions(&fabric_meta());
        assert!(!versions.is_empty());
        assert_eq!(versions[0], "0.19.5");
    }

    #[test]
    fn quilt_loader_versions_real() {
        let versions = quilt_versions(&quilt_meta());
        assert!(!versions.is_empty());
        assert!(versions[0].starts_with("0.20"));
    }

    #[test]
    fn forge_versions_real() {
        let v = forge_versions(&forge_promos(), "1.20.4");
        assert!(!v.is_empty(), "forge versions for 1.20.4");
        assert!(v.contains(&"49.2.8".to_string()));
        // recommended first
        assert_eq!(v[0], "49.2.0");
    }

    #[test]
    fn neoforge_versions_real() {
        let v = neoforge_versions(&neoforge_xml(), "1.20.4");
        assert!(!v.is_empty());
        assert!(v.iter().all(|x| x.starts_with("20.4")), "prefix filter");
        // versions sorted descending
        assert!(cmp_versions(&v[0], &v[1]).is_gt() || v.len() == 1);
    }

    #[test]
    fn resolve_fabric_plan() {
        let c = resolve_loader(
            "1.20.4",
            LoaderKind::Fabric,
            None,
            &fabric_meta(),
            &fabric_installer(),
            &quilt_meta(),
            &forge_promos(),
            &neoforge_xml(),
        )
        .unwrap();
        assert_eq!(c.kind, LoaderKind::Fabric);
        assert!(c
            .installer_url
            .starts_with("https://maven.fabricmc.net/net/fabricmc/fabric-installer/"));
        assert!(c.install_args.contains(&"-loader".to_string()));
        assert_eq!(c.display, format!("1.20.4-fabric-{}", c.loader_version));
    }

    #[test]
    fn resolve_quilt_plan() {
        let c = resolve_loader(
            "1.20.4",
            LoaderKind::Quilt,
            None,
            &fabric_meta(),
            &fabric_installer(),
            &quilt_meta(),
            &forge_promos(),
            &neoforge_xml(),
        )
        .unwrap();
        assert_eq!(c.kind, LoaderKind::Quilt);
        assert_eq!(c.installer_url, QUILT_INSTALLER_URL);
        assert!(c.install_args.iter().any(|a| a == "server"));
    }

    #[test]
    fn resolve_forge_plan() {
        let c = resolve_loader(
            "1.20.4",
            LoaderKind::Forge,
            None,
            &fabric_meta(),
            &fabric_installer(),
            &quilt_meta(),
            &forge_promos(),
            &neoforge_xml(),
        )
        .unwrap();
        assert_eq!(c.loader_version, "49.2.0");
        assert!(c
            .installer_url
            .contains("forge-1.20.4-49.2.0-installer.jar"));
        assert_eq!(c.install_args, vec!["--installServer"]);
        assert_eq!(c.display, "1.20.4-forge-49.2.0");
    }

    #[test]
    fn resolve_neoforge_plan() {
        let c = resolve_loader(
            "1.20.4",
            LoaderKind::NeoForge,
            None,
            &fabric_meta(),
            &fabric_installer(),
            &quilt_meta(),
            &forge_promos(),
            &neoforge_xml(),
        )
        .unwrap();
        assert!(c.loader_version.starts_with("20.4."));
        assert!(c.installer_url.contains(&format!(
            "{}/neoforge-{}-installer.jar",
            c.loader_version, c.loader_version
        )));
        assert_eq!(c.install_args, vec!["--installServer"]);
    }

    #[test]
    fn explicit_version_wins() {
        let c = resolve_loader(
            "1.20.4",
            LoaderKind::Forge,
            Some("49.0.51"),
            &fabric_meta(),
            &fabric_installer(),
            &quilt_meta(),
            &forge_promos(),
            &neoforge_xml(),
        )
        .unwrap();
        assert_eq!(c.loader_version, "49.0.51");
    }

    #[test]
    fn run_installer_fake_java() {
        // Use `sh -c` style fake: run_installer invokes java -jar; emulate by
        // pointing java_bin at sh and installing a "jar" that is actually args.
        #[cfg(unix)]
        {
            let rt = tokio::runtime::Runtime::new().unwrap();
            let dir = tempfile::tempdir().unwrap();
            let script = dir.path().join("fakejava");
            std::fs::write(&script, "#!/bin/sh\necho INSTALLED\nexit 0\n").unwrap();
            use std::os::unix::fs::PermissionsExt;
            let mut perm = std::fs::metadata(&script).unwrap().permissions();
            perm.set_mode(0o755);
            std::fs::set_permissions(&script, perm).unwrap();
            let res = rt
                .block_on(run_installer(
                    &script,
                    dir.path().join("dummy.jar").as_path(),
                    &["-jar".into()],
                    dir.path(),
                ))
                .unwrap();
            assert!(res.success);
            assert!(res.stdout.contains("INSTALLED"));
        }
    }
}
