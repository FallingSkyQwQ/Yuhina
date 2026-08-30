//! Mojang version JSON parsing (`<data_dir>/versions/<id>/<id>.json`).
//!
//! Produces a typed view with resolved libraries, arguments and download
//! entries ready for orchestration (task T3).

use serde_json::Value;
use yuhina_api::YuhinaError;

use crate::arguments::{build_arguments, build_legacy_game_args, ArgTokens, ArgumentMode};
use crate::libraries::{resolve_libraries, Features, Library, Platform};

/// Reference to a download artifact inside the version json.
#[derive(Debug, Clone)]
pub struct VersionDownload {
    pub sha1: Option<String>,
    pub size: Option<u64>,
    pub url: String,
}

#[derive(Debug, Clone)]
pub struct AssetIndexRef {
    pub id: String,
    pub sha1: Option<String>,
    pub size: Option<u64>,
    pub url: String,
}

#[derive(Debug, Clone)]
pub struct LoggingConfig {
    pub argument: String,
    pub id: String,
    pub sha1: Option<String>,
    pub size: Option<u64>,
    pub url: String,
}

/// Fully-parsed version metadata for a single Minecraft version.
#[derive(Debug, Clone)]
pub struct VersionManifest {
    pub id: String,
    pub version_type: String,
    pub main_class: String,
    pub assets: String,
    pub asset_index: AssetIndexRef,
    pub client: Option<VersionDownload>,
    pub server: Option<VersionDownload>,
    pub java_major: u32,
    pub libraries: Vec<Library>,
    pub arguments: Option<Value>,
    pub minecraft_arguments: Option<String>,
    pub logging: Option<LoggingConfig>,
    pub release_time: String,
    pub time: String,
    pub raw: Value,
}

impl VersionManifest {
    pub fn parse(raw: &Value) -> Result<Self, YuhinaError> {
        let id = raw
            .get("id")
            .and_then(Value::as_str)
            .ok_or_else(|| YuhinaError::internal("version json missing id"))?
            .to_string();
        let version_type = raw
            .get("type")
            .and_then(Value::as_str)
            .unwrap_or("release")
            .to_string();
        let main_class = raw
            .get("mainClass")
            .and_then(Value::as_str)
            .unwrap_or("net.minecraft.client.main.Main")
            .to_string();
        let assets = raw
            .get("assets")
            .and_then(Value::as_str)
            .unwrap_or(&id)
            .to_string();
        let asset_index = raw
            .get("assetIndex")
            .map(|ai| AssetIndexRef {
                id: ai
                    .get("id")
                    .and_then(Value::as_str)
                    .unwrap_or(&assets)
                    .to_string(),
                sha1: ai.get("sha1").and_then(Value::as_str).map(String::from),
                size: ai.get("size").and_then(Value::as_u64),
                url: ai
                    .get("url")
                    .and_then(Value::as_str)
                    .unwrap_or("")
                    .to_string(),
            })
            .unwrap_or(AssetIndexRef {
                id: assets.clone(),
                sha1: None,
                size: None,
                url: String::new(),
            });
        let client = raw
            .get("downloads")
            .and_then(|d| d.get("client"))
            .map(parse_download);
        let server = raw
            .get("downloads")
            .and_then(|d| d.get("server"))
            .map(parse_download);
        let java_major = raw
            .get("javaVersion")
            .and_then(|j| j.get("majorVersion"))
            .and_then(Value::as_u64)
            .map(|v| v as u32)
            .unwrap_or(8);
        let libraries: Vec<Library> = raw
            .get("libraries")
            .and_then(Value::as_array)
            .map(|arr| {
                arr.iter()
                    .filter_map(|l| serde_json::from_value(l.clone()).ok())
                    .collect()
            })
            .unwrap_or_default();
        let arguments = raw.get("arguments").cloned().filter(|v| !v.is_null());
        let minecraft_arguments = raw
            .get("minecraftArguments")
            .and_then(Value::as_str)
            .map(String::from);
        let logging = raw
            .get("logging")
            .and_then(|l| l.get("client"))
            .map(|c| LoggingConfig {
                argument: c
                    .get("argument")
                    .and_then(Value::as_str)
                    .unwrap_or("")
                    .to_string(),
                id: c
                    .get("file")
                    .and_then(|f| f.get("id"))
                    .and_then(Value::as_str)
                    .unwrap_or("client-1.12.xml")
                    .to_string(),
                sha1: c
                    .get("file")
                    .and_then(|f| f.get("sha1"))
                    .and_then(Value::as_str)
                    .map(String::from),
                size: c
                    .get("file")
                    .and_then(|f| f.get("size"))
                    .and_then(Value::as_u64),
                url: c
                    .get("file")
                    .and_then(|f| f.get("url"))
                    .and_then(Value::as_str)
                    .unwrap_or("")
                    .to_string(),
            });
        let release_time = raw
            .get("releaseTime")
            .and_then(Value::as_str)
            .unwrap_or("")
            .to_string();
        let time = raw
            .get("time")
            .and_then(Value::as_str)
            .unwrap_or("")
            .to_string();

        Ok(Self {
            id,
            version_type,
            main_class,
            assets,
            asset_index,
            client,
            server,
            java_major,
            libraries,
            arguments,
            minecraft_arguments,
            logging,
            release_time,
            time,
            raw: raw.clone(),
        })
    }

    /// True when this version uses the modern `arguments` object.
    pub fn uses_modern_arguments(&self) -> bool {
        self.arguments.is_some()
    }

    /// Resolved client libraries for `platform`.
    pub fn resolved_libraries(
        &self,
        platform: &Platform,
    ) -> Vec<crate::libraries::ResolvedLibrary> {
        let features = Features {
            has_custom_resolution: None,
            is_demo_user: None,
        };
        resolve_libraries(&self.libraries, platform, &features)
    }

    /// Game arguments (modern or legacy), token-substituted.
    pub fn game_arguments(&self, platform: &Platform, tokens: &ArgTokens) -> Vec<String> {
        if let Some(args) = &self.arguments {
            let features = Features {
                has_custom_resolution: Some(tokens.resolution_width.is_some()),
                is_demo_user: None,
            };
            build_arguments(args, platform, &features, ArgumentMode::Game, tokens)
        } else {
            build_legacy_game_args(self.minecraft_arguments.as_deref().unwrap_or(""), tokens)
        }
    }

    /// JVM arguments from the modern object (empty for legacy versions).
    pub fn jvm_arguments(&self, platform: &Platform, tokens: &ArgTokens) -> Vec<String> {
        if let Some(args) = &self.arguments {
            let features = Features {
                has_custom_resolution: Some(tokens.resolution_width.is_some()),
                is_demo_user: None,
            };
            build_arguments(args, platform, &features, ArgumentMode::Jvm, tokens)
        } else {
            Vec::new()
        }
    }
}

fn parse_download(v: &Value) -> VersionDownload {
    VersionDownload {
        sha1: v.get("sha1").and_then(Value::as_str).map(String::from),
        size: v.get("size").and_then(Value::as_u64),
        url: v
            .get("url")
            .and_then(Value::as_str)
            .unwrap_or("")
            .to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testutil::load_fixture;

    #[test]
    fn parse_real_1204() {
        let vj = load_fixture("1.20.4.json");
        let m = VersionManifest::parse(&vj).unwrap();
        assert_eq!(m.id, "1.20.4");
        assert_eq!(m.main_class, "net.minecraft.client.main.Main");
        assert_eq!(m.assets, "12");
        assert_eq!(m.java_major, 17);
        assert!(m.uses_modern_arguments());
        assert!(m.minecraft_arguments.is_none());
        assert!(m.client.is_some());
        assert!(m.logging.is_some());
        assert_eq!(m.asset_index.id, "12");
        assert!(m.asset_index.url.starts_with("https://"));
    }

    #[test]
    fn parse_real_1122_legacy() {
        let vj = load_fixture("1.12.2.json");
        let m = VersionManifest::parse(&vj).unwrap();
        assert_eq!(m.id, "1.12.2");
        assert_eq!(m.java_major, 8);
        assert!(!m.uses_modern_arguments());
        assert!(m.minecraft_arguments.is_some());
        // legacy game args include the username token
        assert!(m
            .minecraft_arguments
            .as_deref()
            .unwrap()
            .contains("${auth_player_name}"));
        // 1.12.2 has a log4j client config too
        assert!(m.logging.is_some());
    }

    #[test]
    fn game_arguments_modern_with_tokens() {
        let vj = load_fixture("1.20.4.json");
        let m = VersionManifest::parse(&vj).unwrap();
        let platform = Platform {
            os: "linux".into(),
            arch: "x86_64".into(),
        };
        let tokens = ArgTokens {
            auth_player_name: "Steve".into(),
            version_name: "1.20.4".into(),
            game_directory: "/g".into(),
            assets_root: "/g/assets".into(),
            assets_index_name: "12".into(),
            auth_uuid: "u".into(),
            auth_access_token: "t".into(),
            user_type: "msa".into(),
            version_type: "release".into(),
            natives_directory: "/g/natives".into(),
            launcher_name: "yuhina".into(),
            launcher_version: "0.1.0".into(),
            classpath: "/cp".into(),
            library_directory: "/g/libraries".into(),
            resolution_width: None,
            resolution_height: None,
            game_assets: "/g/assets/virtual/legacy".into(),
        };
        let game = m.game_arguments(&platform, &tokens);
        assert!(game.iter().any(|a| a == "--username"));
        assert!(game.iter().any(|a| a == "Steve"));
        assert!(game.iter().any(|a| a == "--version"));
        // resolution args absent (no custom resolution)
        assert!(!game.iter().any(|a| a == "--width"));
    }

    #[test]
    fn game_arguments_legacy() {
        let vj = load_fixture("1.12.2.json");
        let m = VersionManifest::parse(&vj).unwrap();
        let platform = Platform {
            os: "linux".into(),
            arch: "x86_64".into(),
        };
        let tokens = ArgTokens {
            auth_player_name: "Steve".into(),
            version_name: "1.12.2".into(),
            game_directory: "/g".into(),
            assets_root: "/g/assets".into(),
            assets_index_name: "1.12".into(),
            auth_uuid: "u".into(),
            auth_access_token: "t".into(),
            user_type: "legacy".into(),
            version_type: "release".into(),
            natives_directory: "/g/natives".into(),
            launcher_name: "yuhina".into(),
            launcher_version: "0.1.0".into(),
            classpath: "/cp".into(),
            library_directory: "/g/libraries".into(),
            resolution_width: None,
            resolution_height: None,
            game_assets: "/g/assets/virtual/legacy".into(),
        };
        let game = m.game_arguments(&platform, &tokens);
        assert!(game.iter().any(|a| a == "--username"));
        assert!(game.iter().any(|a| a == "Steve"));
    }

    #[test]
    fn jvm_arguments_modern() {
        let vj = load_fixture("1.20.4.json");
        let m = VersionManifest::parse(&vj).unwrap();
        let platform = Platform {
            os: "linux".into(),
            arch: "x86_64".into(),
        };
        let tokens = ArgTokens {
            auth_player_name: "Steve".into(),
            version_name: "1.20.4".into(),
            game_directory: "/g".into(),
            assets_root: "/g/assets".into(),
            assets_index_name: "12".into(),
            auth_uuid: "u".into(),
            auth_access_token: "t".into(),
            user_type: "msa".into(),
            version_type: "release".into(),
            natives_directory: "/g/natives".into(),
            launcher_name: "yuhina".into(),
            launcher_version: "0.1.0".into(),
            classpath: "/cp".into(),
            library_directory: "/g/libraries".into(),
            resolution_width: None,
            resolution_height: None,
            game_assets: "/g/assets/virtual/legacy".into(),
        };
        let jvm = m.jvm_arguments(&platform, &tokens);
        // jvm args should include the classpath token
        assert!(jvm
            .iter()
            .any(|a| a.contains("${classpath}") || a.contains("/cp")));
        assert!(jvm
            .iter()
            .any(|a| a.contains("natives_directory") || a.contains("java.library.path")));
    }
}
