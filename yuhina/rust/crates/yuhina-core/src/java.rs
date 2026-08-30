//! Java runtime discovery, manual registration and Adoptium download (T4).

use std::path::{Path, PathBuf};

use serde_json::Value;
use tracing::warn;
use yuhina_api::{JavaRuntime, JavaSource, YuhinaError};

use crate::download::Downloader;
use crate::libraries::Platform;

/// Extracted info about a java installation.
#[derive(Debug, Clone)]
pub struct JavaInfo {
    pub major: u32,
    pub vendor: String,
    pub version: String,
    pub arch: String,
}

/// Whether `path` looks like a runnable `java` executable.
pub fn is_java_executable(path: &Path) -> bool {
    let name = path
        .file_name()
        .map(|n| n.to_string_lossy().to_string())
        .unwrap_or_default();
    let base_ok = name == "java" || name == "java.exe";
    if !base_ok {
        return false;
    }
    if cfg!(windows) {
        path.exists()
    } else {
        // must be executable (unix-only permission bit)
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            path.exists()
                && path
                    .metadata()
                    .map(|m| m.permissions().mode() & 0o111 != 0)
                    .unwrap_or(false)
        }
        #[cfg(not(unix))]
        {
            path.exists()
        }
    }
}

/// Resolve the java binary inside a JRE/JDK home dir (`<home>/bin/java`).
pub fn java_bin_in_home(home: &Path) -> Option<PathBuf> {
    let bin = if cfg!(windows) {
        home.join("bin/java.exe")
    } else {
        home.join("bin/java")
    };
    if is_java_executable(&bin) {
        Some(bin)
    } else {
        None
    }
}

/// Parse the major version from a java version string like
/// `21.0.2`, `17.0.9`, or legacy `1.8.0_292`.
pub fn parse_major(version: &str) -> u32 {
    let v = version.trim();
    if let Some(rest) = v.strip_prefix("1.") {
        // legacy: 1.8.x → 8
        return rest
            .split('.')
            .next()
            .and_then(|s| s.parse().ok())
            .unwrap_or(0);
    }
    v.split(['.', '-', '+', '_'])
        .next()
        .and_then(|s| s.parse().ok())
        .unwrap_or(0)
}

/// Detect java info by invoking `java -XshowSettings:properties -version`.
pub fn detect_java(bin: &Path) -> Result<JavaInfo, YuhinaError> {
    let output = std::process::Command::new(bin)
        .arg("-XshowSettings:properties")
        .arg("-version")
        .output()
        .map_err(|e| YuhinaError::java_not_found(format!("cannot run {}: {e}", bin.display())))?;
    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();
    let combined = format!("{stdout}\n{stderr}");

    let mut version = String::new();
    let mut vendor = String::new();
    let mut arch = String::new();
    for line in combined.lines() {
        let line = line.trim();
        if line.contains("os.arch") {
            arch = line.split('=').nth(1).unwrap_or("").trim().to_string();
        }
        if line.contains("java.version") && version.is_empty() {
            version = line.split('=').nth(1).unwrap_or("").trim().to_string();
        }
        if line.contains("java.vendor") && vendor.is_empty() {
            vendor = line.split('=').nth(1).unwrap_or("").trim().to_string();
        }
        // fallback: parse the plain `java -version` banner
        if version.is_empty()
            && (line.starts_with("openjdk version") || line.starts_with("java version"))
        {
            if let Some(q) = line.find('"') {
                if let Some(q2) = line[q + 1..].find('"') {
                    version = line[q + 1..q + 1 + q2].to_string();
                }
            }
        }
    }
    if version.is_empty() {
        return Err(YuhinaError::java_not_found(format!(
            "could not determine version of {}",
            bin.display()
        )));
    }
    if arch.is_empty() {
        arch = std::env::consts::ARCH.to_string();
    }
    let major = parse_major(&version);
    Ok(JavaInfo {
        major,
        vendor,
        version,
        arch,
    })
}

/// Candidate system locations for java (Linux/Windows).
pub fn system_search_paths() -> Vec<PathBuf> {
    let mut paths = Vec::new();
    if let Some(home) = std::env::var_os("JAVA_HOME") {
        paths.push(PathBuf::from(home));
    }
    if let Some(path) = std::env::var_os("PATH") {
        for p in std::env::split_paths(&path) {
            paths.push(p);
        }
    }
    if cfg!(target_os = "linux") {
        if let Ok(entries) = std::fs::read_dir("/usr/lib/jvm") {
            for e in entries.flatten() {
                paths.push(e.path());
            }
        }
        paths.push(PathBuf::from("/usr/lib/jvm"));
        paths.push(PathBuf::from("/opt/java"));
        paths.push(PathBuf::from("/usr/java"));
    }
    if cfg!(windows) {
        let pf = std::env::var_os("ProgramFiles").map(PathBuf::from);
        if let Some(pf) = pf {
            for vendor_dir in [
                "Java",
                "Eclipse Adoptium",
                "Microsoft",
                "Zulu",
                "Amazon Corretto",
            ] {
                if let Ok(entries) = std::fs::read_dir(pf.join(vendor_dir)) {
                    for e in entries.flatten() {
                        paths.push(e.path());
                    }
                }
            }
        }
    }
    paths
}

/// Scan the system for java installations, deduplicated by resolved path.
pub fn scan_system() -> Vec<JavaRuntime> {
    let mut seen = std::collections::HashSet::new();
    let mut out = Vec::new();
    let mut candidates: Vec<PathBuf> = Vec::new();
    for p in system_search_paths() {
        let is_binary = p
            .file_name()
            .is_some_and(|n| n == "java" || n == "java.exe");
        if let Some(bin) = java_bin_in_home(&p) {
            candidates.push(bin);
        } else if is_binary {
            candidates.push(p);
        }
    }
    // Also glob common linux layouts directly for `bin/java`.
    for c in candidates {
        let canonical = std::fs::canonicalize(&c).unwrap_or_else(|_| c.clone());
        let key = canonical.to_string_lossy().to_string();
        if !seen.insert(key) {
            continue;
        }
        match detect_java(&canonical) {
            Ok(info) => {
                out.push(JavaRuntime {
                    id: uuid::Uuid::new_v4().to_string(),
                    path: canonical.to_string_lossy().to_string(),
                    major: info.major,
                    vendor: info.vendor,
                    version: info.version,
                    arch: info.arch,
                    source: JavaSource::System,
                });
            }
            Err(e) => warn!(path = %c.display(), "skip java candidate: {e}"),
        }
    }
    out
}

/// Build the Adoptium latest-assets URL for `major`.
pub fn adoptium_url(major: u32, platform: &Platform, image_type: &str) -> String {
    let os = match platform.os.as_str() {
        "windows" => "windows",
        "osx" => "mac",
        _ => "linux",
    };
    let arch = match platform.arch.as_str() {
        "x86" => "x86",
        "arm" => "arm",
        _ => platform.arch.as_str(),
    };
    format!(
        "https://api.adoptium.net/v3/assets/latest/{major}/hotspot?os={os}&architecture={arch}&image_type={image_type}&vendor=adoptium"
    )
}

/// Adoptium asset metadata for the first matching binary.
#[derive(Debug, Clone)]
pub struct AdoptiumBinary {
    pub name: String,
    pub link: String,
    pub size: u64,
    pub checksum: Option<String>,
    pub release_name: String,
}

pub async fn query_adoptium(
    downloader: &dyn Downloader,
    major: u32,
    platform: &Platform,
    image_type: &str,
) -> Result<AdoptiumBinary, YuhinaError> {
    let url = adoptium_url(major, platform, image_type);
    let bytes = downloader.fetch_bytes(&url).await?;
    let value: Value = serde_json::from_slice(&bytes)
        .map_err(|e| YuhinaError::internal(format!("parse adoptium response: {e}")))?;
    let arr = value.as_array().ok_or_else(|| {
        YuhinaError::download_failed(format!("adoptium returned non-array for {url}"))
    })?;
    let asset = arr.first().ok_or_else(|| {
        YuhinaError::download_failed(format!("no adoptium asset for java {major}"))
    })?;
    let binary = asset
        .get("binary")
        .ok_or_else(|| YuhinaError::internal("adoptium asset missing binary"))?;
    let pkg = binary
        .get("package")
        .ok_or_else(|| YuhinaError::internal("adoptium binary missing package"))?;
    Ok(AdoptiumBinary {
        name: pkg
            .get("name")
            .and_then(Value::as_str)
            .unwrap_or("")
            .to_string(),
        link: pkg
            .get("link")
            .and_then(Value::as_str)
            .unwrap_or("")
            .to_string(),
        size: pkg.get("size").and_then(Value::as_u64).unwrap_or(0),
        checksum: pkg
            .get("checksum")
            .and_then(Value::as_str)
            .map(String::from),
        release_name: binary
            .get("release_name")
            .and_then(Value::as_str)
            .unwrap_or("")
            .to_string(),
    })
}

/// Download + unpack an Adoptium archive into `dest_dir`, returning the java
/// binary path and detected info.
pub async fn install_java_from_adoptium(
    downloader: &dyn Downloader,
    major: u32,
    dest_dir: &Path,
    platform: &Platform,
) -> Result<(PathBuf, JavaInfo), YuhinaError> {
    std::fs::create_dir_all(dest_dir)
        .map_err(|e| YuhinaError::io(format!("mkdir {}: {e}", dest_dir.display())))?;
    let bin = query_adoptium(downloader, major, platform, "jre")
        .await
        .map_err(|e| YuhinaError::download_failed(format!("query adoptium: {e}")))?;
    let archive = dest_dir.join(&bin.name);
    downloader
        .download(&bin.link, &archive, None)
        .await
        .map_err(|e| YuhinaError::download_failed(format!("download java {major}: {e}")))?;

    let unpack_dir = dest_dir.join(format!("jre-{major}"));
    unpack_archive(&archive, &unpack_dir)?;

    let bin_path = find_java_recursive(&unpack_dir, 4).ok_or_else(|| {
        YuhinaError::download_failed(format!("no java binary found after unpacking {}", bin.name))
    })?;
    let info = detect_java(&bin_path)?;
    let _ = std::fs::remove_file(&archive);
    Ok((bin_path, info))
}

/// Recursively search for `bin/java` under `root` up to `depth` levels.
pub fn find_java_recursive(root: &Path, depth: usize) -> Option<PathBuf> {
    if depth == 0 {
        return None;
    }
    if let Some(bin) = java_bin_in_home(root) {
        return Some(bin);
    }
    let entries = std::fs::read_dir(root).ok()?;
    for e in entries.flatten() {
        let p = e.path();
        if p.is_dir() {
            if let Some(found) = find_java_recursive(&p, depth - 1) {
                return Some(found);
            }
        }
    }
    None
}

/// Unpack a `.tar.gz`/`.tgz` (or `.zip`) archive into `dest`.
pub fn unpack_archive(archive: &Path, dest: &Path) -> Result<(), YuhinaError> {
    std::fs::create_dir_all(dest)
        .map_err(|e| YuhinaError::io(format!("mkdir {}: {e}", dest.display())))?;
    let name = archive
        .file_name()
        .map(|n| n.to_string_lossy().to_string())
        .unwrap_or_default();
    if name.ends_with(".zip") {
        unpack_zip(archive, dest)
    } else {
        unpack_tar_gz(archive, dest)
    }
}

fn unpack_tar_gz(archive: &Path, dest: &Path) -> Result<(), YuhinaError> {
    let file = std::fs::File::open(archive)
        .map_err(|e| YuhinaError::io(format!("open {}: {e}", archive.display())))?;
    let gz = flate2::read::GzDecoder::new(file);
    let mut tar = tar::Archive::new(gz);
    tar.set_preserve_permissions(true);
    tar.unpack(dest)
        .map_err(|e| YuhinaError::io(format!("unpack {}: {e}", archive.display())))?;
    Ok(())
}

fn unpack_zip(archive: &Path, dest: &Path) -> Result<(), YuhinaError> {
    let file = std::fs::File::open(archive)
        .map_err(|e| YuhinaError::io(format!("open {}: {e}", archive.display())))?;
    let mut zip = zip::ZipArchive::new(file)
        .map_err(|e| YuhinaError::io(format!("open zip {}: {e}", archive.display())))?;
    for i in 0..zip.len() {
        let mut entry = zip
            .by_index(i)
            .map_err(|e| YuhinaError::io(format!("zip entry {i}: {e}")))?;
        let name = entry.name().to_string();
        let out = dest.join(name);
        if entry.is_dir() {
            std::fs::create_dir_all(&out)
                .map_err(|e| YuhinaError::io(format!("mkdir {}: {e}", out.display())))?;
            continue;
        }
        if let Some(parent) = out.parent() {
            std::fs::create_dir_all(parent)
                .map_err(|e| YuhinaError::io(format!("mkdir {}: {e}", parent.display())))?;
        }
        let mut f = std::fs::File::create(&out)
            .map_err(|e| YuhinaError::io(format!("create {}: {e}", out.display())))?;
        std::io::copy(&mut entry, &mut f)
            .map_err(|e| YuhinaError::io(format!("extract {}: {e}", out.display())))?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_major_versions() {
        assert_eq!(parse_major("21.0.2"), 21);
        assert_eq!(parse_major("17.0.9"), 17);
        assert_eq!(parse_major("1.8.0_292"), 8);
        assert_eq!(parse_major("11.0.20"), 11);
        assert_eq!(parse_major("26"), 26);
        assert_eq!(parse_major("garbage"), 0);
    }

    #[test]
    fn adoptium_url_builder() {
        let linux = Platform {
            os: "linux".into(),
            arch: "x86_64".into(),
        };
        let url = adoptium_url(21, &linux, "jre");
        assert!(url.contains("/v3/assets/latest/21/hotspot"));
        assert!(url.contains("os=linux"));
        assert!(url.contains("architecture=x86_64"));
        assert!(url.contains("image_type=jre"));
        let win = Platform {
            os: "windows".into(),
            arch: "arm64".into(),
        };
        let url = adoptium_url(17, &win, "jdk");
        assert!(url.contains("os=windows"));
        assert!(url.contains("architecture=arm64"));
        assert!(url.contains("image_type=jdk"));
    }

    #[test]
    fn java_bin_path_detection() {
        let home = tempfile::tempdir().unwrap();
        std::fs::create_dir_all(home.path().join("bin")).unwrap();
        let bin = if cfg!(windows) {
            home.path().join("bin/java.exe")
        } else {
            home.path().join("bin/java")
        };
        std::fs::write(&bin, "#!/bin/sh\nexit 0\n").unwrap();
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mut perm = std::fs::metadata(&bin).unwrap().permissions();
            perm.set_mode(0o755);
            std::fs::set_permissions(&bin, perm).unwrap();
        }
        assert!(is_java_executable(&bin));
        assert_eq!(java_bin_in_home(home.path()).unwrap(), bin);
    }
}
