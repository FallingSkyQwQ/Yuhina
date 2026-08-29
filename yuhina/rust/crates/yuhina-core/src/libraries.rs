//! Minecraft library (dependency) parsing, OS/arch rules filtering and
//! natives classifier expansion (task T3).

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::download::DownloadItem;

/// Current platform, derived from `std::env::consts`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Platform {
    pub os: String,
    pub arch: String,
}

impl Platform {
    pub fn detect() -> Self {
        let os = match std::env::consts::OS {
            "linux" => "linux",
            "macos" => "osx",
            "windows" => "windows",
            other => other,
        };
        let arch = match std::env::consts::ARCH {
            "x86" => "x86",
            "x86_64" => "x86_64",
            "aarch64" => "arm64",
            "arm" => "arm",
            other => other,
        };
        Self {
            os: os.to_string(),
            arch: arch.to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OsRule {
    pub name: Option<String>,
    pub arch: Option<String>,
    pub version: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Features {
    #[serde(rename = "has_custom_resolution")]
    pub has_custom_resolution: Option<bool>,
    #[serde(rename = "is_demo_user")]
    pub is_demo_user: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Rule {
    pub action: RuleAction,
    pub os: Option<OsRule>,
    pub features: Option<Features>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RuleAction {
    Allow,
    Disallow,
}

/// Whether a rule set permits loading on the given platform + features.
/// An absent `rules` array means "always allow".
pub fn matches_rules(
    rules: &[Rule],
    platform: &Platform,
    features: &Features,
) -> bool {
    if rules.is_empty() {
        return true;
    }
    let mut allowed = false;
    for rule in rules {
        let os_match = rule
            .os
            .as_ref()
            .map(|os| {
                let name_ok = os
                    .name
                    .as_deref()
                    .map(|n| n == platform.os)
                    .unwrap_or(true);
                let arch_ok = os
                    .arch
                    .as_deref()
                    .map(|a| a == platform.arch)
                    .unwrap_or(true);
                name_ok && arch_ok
            })
            .unwrap_or(true);
        let features_ok = match &rule.features {
            None => true,
            Some(f) => {
                let res_ok = match f.has_custom_resolution {
                    Some(v) => v == features.has_custom_resolution.unwrap_or(false),
                    None => true,
                };
                let demo_ok = match f.is_demo_user {
                    Some(v) => v == features.is_demo_user.unwrap_or(false),
                    None => true,
                };
                res_ok && demo_ok
            }
        };
        if os_match && features_ok {
            allowed = rule.action == RuleAction::Allow;
        }
    }
    allowed
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Artifact {
    pub path: String,
    pub sha1: Option<String>,
    pub size: Option<u64>,
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LibraryDownloads {
    pub artifact: Option<Artifact>,
    pub classifiers: Option<HashMap<String, Artifact>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtractRule {
    pub exclude: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Library {
    pub name: String,
    #[serde(default)]
    pub rules: Vec<Rule>,
    pub downloads: Option<LibraryDownloads>,
    pub natives: Option<HashMap<String, String>>,
    pub extract: Option<ExtractRule>,
    pub url: Option<String>,
    /// `serverreq`/`clientreq` legacy flags (default true for client).
    #[serde(default = "default_true")]
    pub clientreq: bool,
    #[serde(default = "default_true")]
    pub serverreq: bool,
}

fn default_true() -> bool {
    true
}

impl Library {
    /// Maven coordinates as (group, artifact, version).
    pub fn coords(&self) -> (String, String, String) {
        // name like "com.mojang:patchy:2.2.10" or "group:artifact:version:classifier"
        let parts: Vec<&str> = self.name.split(':').collect();
        match parts.as_slice() {
            [g, a, v] => (g.to_string(), a.to_string(), v.to_string()),
            [g, a, v, _classifier] => (g.to_string(), a.to_string(), v.to_string()),
            _ => (self.name.clone(), String::new(), String::new()),
        }
    }

    /// Whether this library has a natives classifier for `platform`.
    pub fn native_classifier(&self, platform: &Platform) -> Option<String> {
        self.natives
            .as_ref()
            .and_then(|n| n.get(&platform.os).cloned())
    }

    /// True when this library is a native library for `platform`.
    ///
    /// Two shapes exist across MC versions:
    /// - legacy: a `natives` map with a classifier key (e.g. `natives-linux`);
    /// - modern: a dedicated entry whose maven name has a `natives-<os>`
    ///   classifier as its 4th segment (gated by os rules), artifact path
    ///   already containing the classifier.
    pub fn is_native_for(&self, platform: &Platform) -> bool {
        if self.natives.is_some() {
            return true;
        }
        if self.name.split(':').nth(3).is_some_and(|c| c.starts_with("natives-")) {
            return true;
        }
        if let Some(artifact) = self.downloads.as_ref().and_then(|d| d.artifact.as_ref()) {
            return artifact
                .path
                .contains(&format!("natives-{}", platform.os));
        }
        false
    }

    /// Pick the artifact to fetch for a client launch on `platform`.
    /// Native libs resolve through `classifiers`, others through `artifact`.
    pub fn artifact_for(&self, platform: &Platform) -> Option<Artifact> {
        if let Some(classifier) = self.native_classifier(platform) {
            if let Some(classifiers) = self.downloads.as_ref().and_then(|d| d.classifiers.as_ref()) {
                return classifiers.get(&classifier).cloned();
            }
            // Legacy: build classifier path from maven coords.
            let (g, a, v) = self.coords();
            let rel = format!(
                "{}/{}/{}/{}-{}-{}.jar",
                g.replace('.', "/"),
                a,
                v,
                a,
                v,
                classifier
            );
            let url = self
                .url
                .as_deref()
                .unwrap_or("https://libraries.minecraft.net/");
            let url = format!("{}/{}", url.trim_end_matches('/'), rel);
            return Some(Artifact {
                path: rel,
                sha1: None,
                size: None,
                url,
            });
        }
        if self.natives.is_some() {
            return None;
        }
        self.downloads.as_ref().and_then(|d| d.artifact.clone()).or_else(|| {
            // Legacy library without downloads: derive path from name + url.
            let (g, a, v) = self.coords();
            let rel = format!("{}/{}/{}/{}-{}.jar", g.replace('.', "/"), a, v, a, v);
            let url = self
                .url
                .as_deref()
                .unwrap_or("https://libraries.minecraft.net/");
            let url = format!("{}/{}", url.trim_end_matches('/'), rel);
            Some(Artifact {
                path: rel,
                sha1: None,
                size: None,
                url,
            })
        })
    }
}

/// A resolved library ready for the classpath / download orchestration.
#[derive(Debug, Clone)]
pub struct ResolvedLibrary {
    /// Maven name (for dedup + classpath ordering).
    pub name: String,
    /// Relative path under the libraries dir.
    pub path: String,
    pub sha1: Option<String>,
    pub size: Option<u64>,
    pub url: String,
    /// True when this is a natives jar that must be extracted before launch.
    pub is_native: bool,
    pub extract_exclude: Vec<String>,
}

impl ResolvedLibrary {
    pub fn to_download_item(&self, libraries_dir: &std::path::Path) -> DownloadItem {
        DownloadItem {
            url: self.url.clone(),
            target_path: libraries_dir.join(&self.path).to_string_lossy().to_string(),
            sha1: self.sha1.clone(),
            size: self.size,
        }
    }
}

/// Resolve all libraries for a client launch on `platform`, applying rules,
/// filtering non-client artifacts and expanding natives.
pub fn resolve_libraries(
    libraries: &[Library],
    platform: &Platform,
    features: &Features,
) -> Vec<ResolvedLibrary> {
    let mut out: Vec<ResolvedLibrary> = Vec::new();
    for lib in libraries {
        if !lib.clientreq {
            continue;
        }
        if !matches_rules(&lib.rules, platform, features) {
            continue;
        }
        let is_native = lib.is_native_for(platform);
        if let Some(artifact) = lib.artifact_for(platform) {
            out.push(ResolvedLibrary {
                name: lib.name.clone(),
                path: artifact.path,
                sha1: artifact.sha1,
                size: artifact.size,
                url: artifact.url,
                is_native,
                extract_exclude: lib
                    .extract
                    .as_ref()
                    .map(|e| e.exclude.clone())
                    .unwrap_or_default(),
            });
        }
    }
    out
}

/// Build the classpath string (libraries separated by platform separator),
/// appending `client_jar` last.
pub fn build_classpath(libraries: &[ResolvedLibrary], libraries_dir: &std::path::Path, client_jar: &std::path::Path) -> String {
    let sep = std::path::MAIN_SEPARATOR.to_string();
    let mut entries: Vec<String> = Vec::new();
    for lib in libraries {
        if lib.is_native {
            continue;
        }
        entries.push(libraries_dir.join(&lib.path).to_string_lossy().to_string());
    }
    entries.push(client_jar.to_string_lossy().to_string());
    entries.join(&sep)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testutil::load_fixture;

    fn linux() -> Platform {
        Platform {
            os: "linux".into(),
            arch: "x86_64".into(),
        }
    }

    fn no_features() -> Features {
        Features {
            has_custom_resolution: Some(false),
            is_demo_user: Some(false),
        }
    }

    #[test]
    fn rules_matrix() {
        let allow_linux = Rule {
            action: RuleAction::Allow,
            os: Some(OsRule {
                name: Some("linux".into()),
                arch: None,
                version: None,
            }),
            features: None,
        };
        let disallow_windows = Rule {
            action: RuleAction::Disallow,
            os: Some(OsRule {
                name: Some("windows".into()),
                arch: None,
                version: None,
            }),
            features: None,
        };
        // linux: allow_linux matches (true) then disallow_windows doesn't match → allowed
        assert!(matches_rules(&[allow_linux.clone(), disallow_windows.clone()], &linux(), &no_features()));
        // windows: allow_linux no match (stays false), disallow_windows matches → disallowed
        let windows = Platform { os: "windows".into(), arch: "x86_64".into() };
        assert!(!matches_rules(&[allow_linux, disallow_windows], &windows, &no_features()));
        // empty rules → allow
        assert!(matches_rules(&[], &linux(), &no_features()));
    }

    #[test]
    fn arch_rules() {
        let arm = Rule {
            action: RuleAction::Allow,
            os: Some(OsRule {
                name: Some("osx".into()),
                arch: Some("arm64".into()),
                version: None,
            }),
            features: None,
        };
        let mac_arm = Platform { os: "osx".into(), arch: "arm64".into() };
        let mac_x64 = Platform { os: "osx".into(), arch: "x86_64".into() };
        let rules = [arm];
        assert!(matches_rules(&rules, &mac_arm, &no_features()));
        assert!(!matches_rules(&rules, &mac_x64, &no_features()));
    }

    #[test]
    fn feature_rules() {
        let rule = Rule {
            action: RuleAction::Allow,
            os: None,
            features: Some(Features {
                has_custom_resolution: Some(true),
                is_demo_user: None,
            }),
        };
        let rules = [rule];
        assert!(matches_rules(&rules, &linux(), &Features { has_custom_resolution: Some(true), is_demo_user: None }));
        assert!(!matches_rules(&rules, &linux(), &no_features()));
    }

    #[test]
    fn native_classifier_selection() {
        let lib: Library = serde_json::from_str(
            r#"{
                "name": "org.lwjgl:lwjgl:3.3.1",
                "downloads": {
                    "artifact": {"path": "org/lwjgl/lwjgl/3.3.1/lwjgl-3.3.1.jar", "url": "https://libraries.minecraft.net/org/lwjgl/lwjgl/3.3.1/lwjgl-3.3.1.jar"},
                    "classifiers": {
                        "natives-linux": {"path": "org/lwjgl/lwjgl/3.3.1/lwjgl-3.3.1-natives-linux.jar", "url": "https://libraries.minecraft.net/org/lwjgl/lwjgl/3.3.1/lwjgl-3.3.1-natives-linux.jar"},
                        "natives-windows": {"path": "org/lwjgl/lwjgl/3.3.1/lwjgl-3.3.1-natives-windows.jar", "url": "https://libraries.minecraft.net/org/lwjgl/lwjgl/3.3.1/lwjgl-3.3.1-natives-windows.jar"}
                    }
                },
                "natives": {"linux": "natives-linux", "windows": "natives-windows"}
            }"#,
        )
        .unwrap();
        let artifact = lib.artifact_for(&linux()).unwrap();
        assert!(artifact.path.ends_with("natives-linux.jar"));
        let resolved = resolve_libraries(&[lib], &linux(), &no_features());
        assert_eq!(resolved.len(), 1);
        assert!(resolved[0].is_native);
    }

    #[test]
    fn legacy_library_without_downloads() {
        let lib: Library = serde_json::from_str(
            r#"{"name": "com.paulscode:codecjorbis:20101023", "url": "https://libraries.minecraft.net/"}"#,
        )
        .unwrap();
        let a = lib.artifact_for(&linux()).unwrap();
        assert_eq!(
            a.path,
            "com/paulscode/codecjorbis/20101023/codecjorbis-20101023.jar"
        );
        assert!(a.url.starts_with("https://libraries.minecraft.net/com/paulscode"));
    }

    #[test]
    fn resolve_real_1204_libraries() {
        let vj = load_fixture("1.20.4.json");
        let libs: Vec<Library> = serde_json::from_value(vj["libraries"].clone()).unwrap();
        let resolved = resolve_libraries(&libs, &linux(), &no_features());
        // 1.20.4 has 88 raw libraries; resolved (client, linux x64) should be fewer.
        assert!(resolved.len() > 40 && resolved.len() < 88, "resolved {}", resolved.len());
        assert!(resolved.iter().any(|l| l.is_native));
        // classpath must contain client jar appended
        let cp = build_classpath(&resolved, std::path::Path::new("/g/libs"), std::path::Path::new("/g/v/1.20.4.jar"));
        assert!(cp.ends_with("/g/v/1.20.4.jar"));
        assert!(cp.contains("org/lwjgl/lwjgl/3.3.2/lwjgl-3.3.2.jar"));
        // natives jars are excluded from classpath
        assert!(!cp.contains("natives-linux"));
    }

    #[test]
    fn resolve_real_1122_libraries() {
        let vj = load_fixture("1.12.2.json");
        let libs: Vec<Library> = serde_json::from_value(vj["libraries"].clone()).unwrap();
        let resolved = resolve_libraries(&libs, &linux(), &no_features());
        assert!(resolved.len() > 15, "resolved {}", resolved.len());
        // 1.12.2 uses legacy urls; artifact paths derived from maven coords
        assert!(resolved.iter().all(|l| !l.path.is_empty()));
    }
}