//! Path resolution and launch configuration helpers.

use std::path::{Path, PathBuf};

use yuhina_api::{LauncherConfig, LaunchArgs};

/// Resolved absolute paths derived from `LauncherConfig`.
#[derive(Debug, Clone)]
pub struct CorePaths {
    pub data_dir: PathBuf,
    pub game_root: PathBuf,
    pub versions_dir: PathBuf,
    pub instances_dir: PathBuf,
    pub libraries_dir: PathBuf,
    pub assets_dir: PathBuf,
    pub assets_objects_dir: PathBuf,
    pub logs_dir: PathBuf,
    pub db_path: PathBuf,
}

impl CorePaths {
    pub fn from_config(config: &LauncherConfig) -> Self {
        let data_dir = expand(&config.data_dir);
        let game_root = expand(&config.game_root);
        Self {
            versions_dir: data_dir.join("versions"),
            instances_dir: data_dir.join("instances"),
            libraries_dir: game_root.join("libraries"),
            assets_dir: game_root.join("assets"),
            assets_objects_dir: game_root.join("assets/objects"),
            logs_dir: data_dir.join("logs"),
            db_path: data_dir.join("yuhina.db"),
            data_dir,
            game_root,
        }
    }

    /// Directory for a given version's own files (client jar, json, log config).
    pub fn version_dir(&self, version_id: &str) -> PathBuf {
        self.versions_dir.join(version_id)
    }

    /// Path of the client jar for `version_id`.
    pub fn client_jar(&self, version_id: &str) -> PathBuf {
        self.version_dir(version_id).join(format!("{version_id}.jar"))
    }

    /// Path of the version json for `version_id`.
    pub fn version_json(&self, version_id: &str) -> PathBuf {
        self.version_dir(version_id).join(format!("{version_id}.json"))
    }

    /// Path of the log4j config for `version_id`.
    pub fn logging_config(&self, version_id: &str) -> PathBuf {
        self.version_dir(version_id).join(format!("{version_id}.log"))
    }

    /// Full path of a library artifact given its maven-like relative path.
    pub fn library_path(&self, rel_path: &str) -> PathBuf {
        self.libraries_dir.join(rel_path)
    }

    /// Directory for a session's natives (extracted native libraries).
    pub fn natives_dir(&self, session_id: &str) -> PathBuf {
        self.data_dir.join("natives").join(session_id)
    }

    pub fn session_log_path(&self, session_id: &str) -> PathBuf {
        self.logs_dir.join(session_id).join("game.log")
    }
}

fn expand(p: &str) -> PathBuf {
    let p = shellexpand(p);
    let path = PathBuf::from(p);
    if path.is_absolute() {
        path
    } else {
        std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")).join(path)
    }
}

fn shellexpand(p: &str) -> String {
    if let Some(home) = std::env::var_os("HOME") {
        if let Some(rest) = p.strip_prefix("~/") {
            return Path::new(&home).join(rest).to_string_lossy().to_string();
        }
    }
    p.to_string()
}

/// Determine Java major version required by a Minecraft release.
///
/// Mapping table (task T4):
/// - `< 1.17`        → 8
/// - `1.17 .. 1.20.4` → 17
/// - `>= 1.20.5`     → 21
pub fn java_major_for_mc(mc_version: &str) -> u32 {
    // Snapshot id "1.21.x-..." style handled via prefix parse below.
    let trimmed = mc_version
        .split('-')
        .next()
        .unwrap_or(mc_version)
        .to_string();
    let parse = |s: &str| -> Option<(u32, u32)> {
        let mut it = s.split('.');
        let major = it.next()?.parse().ok()?;
        let minor: u32 = it.next().and_then(|v| v.parse().ok()).unwrap_or(0);
        Some((major, minor))
    };
    let (major, minor) = match parse(&trimmed) {
        Some(v) => v,
        None => return 21,
    };
    if major == 0 {
        return 21;
    }
    match major {
        1 => match minor {
            0..=16 => 8,
            17..=20 => {
                if minor == 20 {
                    // 1.20.x: 1.20.4 uses 17, 1.20.5+ uses 21.
                    if let Some(patch) = trimmed.split('.').nth(2).and_then(|p| p.parse::<u32>().ok()) {
                        if patch >= 5 {
                            return 21;
                        }
                    }
                    17
                } else {
                    17
                }
            }
            _ => 21,
        },
        _ => 21,
    }
}

/// Apply a `LaunchArgs` (global or per-instance) to produce JVM memory flags.
pub fn memory_flags(args: &LaunchArgs) -> Vec<String> {
    let min = args.min_memory_mb.max(256);
    let max = args.max_memory_mb.max(min);
    vec![
        format!("-Xms{min}M"),
        format!("-Xmx{max}M"),
        "-XX:+UseG1GC".to_string(),
        "-XX:+UnlockExperimentalVMOptions".to_string(),
        "-XX:MaxGCPauseMillis=50".to_string(),
        "-XX:G1HeapRegionSize=16M".to_string(),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn java_major_mapping() {
        assert_eq!(java_major_for_mc("1.12.2"), 8);
        assert_eq!(java_major_for_mc("1.8.9"), 8);
        assert_eq!(java_major_for_mc("1.16.5"), 8);
        assert_eq!(java_major_for_mc("1.17.1"), 17);
        assert_eq!(java_major_for_mc("1.18.2"), 17);
        assert_eq!(java_major_for_mc("1.20.4"), 17);
        assert_eq!(java_major_for_mc("1.20.5"), 21);
        assert_eq!(java_major_for_mc("1.20.6"), 21);
        assert_eq!(java_major_for_mc("1.21.1"), 21);
        assert_eq!(java_major_for_mc("26.2"), 21);
        assert_eq!(java_major_for_mc("1.21.5-pre1"), 21);
    }

    #[test]
    fn memory_flags_respect_floor() {
        let args = LaunchArgs {
            min_memory_mb: 128,
            max_memory_mb: 128,
            ..Default::default()
        };
        let flags = memory_flags(&args);
        assert!(flags.iter().any(|f| f == "-Xms256M"));
        assert!(flags.iter().any(|f| f == "-Xmx256M"));
    }
}