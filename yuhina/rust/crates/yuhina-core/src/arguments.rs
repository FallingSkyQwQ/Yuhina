//! Minecraft launch arguments parsing and token substitution.
//!
//! Handles the modern `arguments` object (game/jvm arrays of string-or-rule
//! objects) and the legacy `minecraftArguments` string (task T3).

use serde_json::Value;
use yuhina_api::LogLevel;

use crate::libraries::{matches_rules, Features, Platform};

/// A single argument entry: either a literal string or a rule-gated value.
#[derive(Debug, Clone)]
enum ArgEntry {
    Literal(String),
    Ruled {
        rules: Vec<crate::libraries::Rule>,
        value: Vec<String>,
    },
}

fn parse_arg_value(v: &Value) -> Vec<String> {
    match v {
        Value::String(s) => vec![s.clone()],
        Value::Array(items) => items
            .iter()
            .filter_map(|i| i.as_str().map(|s| s.to_string()))
            .collect(),
        _ => Vec::new(),
    }
}

fn parse_entries(arr: &Value) -> Vec<ArgEntry> {
    let Some(items) = arr.as_array() else {
        return Vec::new();
    };
    let mut out = Vec::new();
    for item in items {
        match item {
            Value::String(s) => out.push(ArgEntry::Literal(s.clone())),
            Value::Object(_) => {
                let rules = item
                    .get("rules")
                    .and_then(|r| serde_json::from_value(r.clone()).ok())
                    .unwrap_or_default();
                let value = item.get("value").map(parse_arg_value).unwrap_or_default();
                out.push(ArgEntry::Ruled { rules, value });
            }
            _ => {}
        }
    }
    out
}

/// Split the legacy space-separated `minecraftArguments` string.
/// Handles simple quoting for values containing spaces.
pub fn split_minecraft_arguments(s: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut current = String::new();
    let mut in_quotes = false;
    for c in s.chars() {
        match c {
            '"' => in_quotes = !in_quotes,
            ' ' if !in_quotes => {
                if !current.is_empty() {
                    out.push(std::mem::take(&mut current));
                }
            }
            _ => current.push(c),
        }
    }
    if !current.is_empty() {
        out.push(current);
    }
    out
}

/// Values used for token substitution during command building.
#[derive(Debug, Clone)]
pub struct ArgTokens {
    pub auth_player_name: String,
    pub version_name: String,
    pub game_directory: String,
    pub assets_root: String,
    pub assets_index_name: String,
    pub auth_uuid: String,
    pub auth_access_token: String,
    pub user_type: String,
    pub version_type: String,
    pub natives_directory: String,
    pub launcher_name: String,
    pub launcher_version: String,
    pub classpath: String,
    pub library_directory: String,
    pub resolution_width: Option<String>,
    pub resolution_height: Option<String>,
    /// Legacy `--gameAssets` path (older versions).
    pub game_assets: String,
}

impl ArgTokens {
    pub fn resolve(&self, token: &str) -> Option<String> {
        Some(match token {
            "auth_player_name" => self.auth_player_name.clone(),
            "version_name" => self.version_name.clone(),
            "game_directory" => self.game_directory.clone(),
            "assets_root" => self.assets_root.clone(),
            "assets_index_name" => self.assets_index_name.clone(),
            "auth_uuid" => self.auth_uuid.clone(),
            "auth_access_token" => self.auth_access_token.clone(),
            "user_type" => self.user_type.clone(),
            "version_type" => self.version_type.clone(),
            "natives_directory" => self.natives_directory.clone(),
            "launcher_name" => self.launcher_name.clone(),
            "launcher_version" => self.launcher_version.clone(),
            "classpath" => self.classpath.clone(),
            "library_directory" => self.library_directory.clone(),
            "resolution_width" => self
                .resolution_width
                .clone()
                .unwrap_or_else(|| "854".into()),
            "resolution_height" => self
                .resolution_height
                .clone()
                .unwrap_or_else(|| "480".into()),
            "game_assets" => self.game_assets.clone(),
            _ => return None,
        })
    }

    pub fn substitute(&self, arg: &str) -> String {
        let mut out = String::with_capacity(arg.len());
        let mut rest = arg;
        while let Some(start) = rest.find("${") {
            out.push_str(&rest[..start]);
            let tail = &rest[start + 2..];
            match tail.find('}') {
                Some(end) => {
                    let token = &tail[..end];
                    let value = self
                        .resolve(token)
                        .unwrap_or_else(|| format!("${{{token}}}"));
                    out.push_str(&value);
                    rest = &tail[end + 1..];
                }
                None => {
                    out.push_str("${");
                    rest = tail;
                }
            }
        }
        out.push_str(rest);
        out
    }
}

/// Collect the final argument list from modern `arguments.game`/`.jvm`.
/// Returns (jvm_args, game_args). `mode` selects which array to expand.
pub fn build_arguments(
    arguments: &Value,
    platform: &Platform,
    features: &Features,
    mode: ArgumentMode,
    tokens: &ArgTokens,
) -> Vec<String> {
    let key = match mode {
        ArgumentMode::Game => "game",
        ArgumentMode::Jvm => "jvm",
    };
    let arr = arguments.get(key).cloned().unwrap_or(Value::Null);
    let entries = parse_entries(&arr);
    let mut out = Vec::new();
    for entry in entries {
        match entry {
            ArgEntry::Literal(s) => out.push(tokens.substitute(&s)),
            ArgEntry::Ruled { rules, value } => {
                if matches_rules(&rules, platform, features) {
                    for v in value {
                        out.push(tokens.substitute(&v));
                    }
                }
            }
        }
    }
    out
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArgumentMode {
    Game,
    Jvm,
}

/// Legacy game args (`minecraftArguments`), split and substituted.
pub fn build_legacy_game_args(raw: &str, tokens: &ArgTokens) -> Vec<String> {
    split_minecraft_arguments(raw)
        .into_iter()
        .map(|a| tokens.substitute(&a))
        .collect()
}

/// Detect a warn/error level from a Minecraft log line for classification.
pub fn classify_level(line: &str) -> LogLevel {
    let l = line.to_lowercase();
    if l.contains("fatal")
        || l.contains("exception")
        || l.contains("error")
        || l.contains("[error]")
        || l.starts_with("error")
    {
        LogLevel::Error
    } else if l.contains("warn") || l.contains("[warn]") {
        LogLevel::Warn
    } else if l.contains("debug") || l.contains("[debug]") {
        LogLevel::Debug
    } else {
        LogLevel::Info
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tokens() -> ArgTokens {
        ArgTokens {
            auth_player_name: "Steve".into(),
            version_name: "1.20.4".into(),
            game_directory: "/game".into(),
            assets_root: "/game/assets".into(),
            assets_index_name: "12".into(),
            auth_uuid: "uuid-1".into(),
            auth_access_token: "token-1".into(),
            user_type: "msa".into(),
            version_type: "release".into(),
            natives_directory: "/tmp/natives".into(),
            launcher_name: "yuhina".into(),
            launcher_version: "0.1.0".into(),
            classpath: "/cp".into(),
            library_directory: "/game/libraries".into(),
            resolution_width: Some("1920".into()),
            resolution_height: Some("1080".into()),
            game_assets: "/game/assets/virtual/legacy".into(),
        }
    }

    #[test]
    fn substitution_basic_and_unknown() {
        let t = tokens();
        assert_eq!(
            t.substitute("--username ${auth_player_name}"),
            "--username Steve"
        );
        assert_eq!(
            t.substitute("--version ${version_name}"),
            "--version 1.20.4"
        );
        // unknown token left intact
        assert_eq!(t.substitute("${unknown}"), "${unknown}");
        // empty
        assert_eq!(t.substitute(""), "");
        assert_eq!(t.substitute("no tokens"), "no tokens");
    }

    #[test]
    fn legacy_argument_split_with_quotes() {
        let args = split_minecraft_arguments(
            "--username Steve --version 1.12.2 \"--with space\" --uuid x",
        );
        assert_eq!(
            args,
            vec![
                "--username",
                "Steve",
                "--version",
                "1.12.2",
                "--with space",
                "--uuid",
                "x"
            ]
        );
    }

    #[test]
    fn modern_arguments_with_rules() {
        let json = r#"{
          "game": [
            "--username",
            "${auth_player_name}",
            "--version",
            "${version_name}",
            {"rules": [{"action": "allow", "features": {"has_custom_resolution": true}}],
             "value": ["--width", "${resolution_width}", "--height", "${resolution_height}"]}
          ],
          "jvm": [
            "-Xmx2G",
            {"rules": [{"action": "allow", "os": {"name": "osx"}}],
             "value": "-XstartOnFirstThread"}
          ]
        }"#;
        let v: Value = serde_json::from_str(json).unwrap();
        let linux = Platform {
            os: "linux".into(),
            arch: "x86_64".into(),
        };
        let f_res = Features {
            has_custom_resolution: Some(true),
            is_demo_user: None,
        };
        let game = build_arguments(&v, &linux, &f_res, ArgumentMode::Game, &tokens());
        assert_eq!(
            game,
            vec![
                "--username",
                "Steve",
                "--version",
                "1.20.4",
                "--width",
                "1920",
                "--height",
                "1080"
            ]
        );
        let f_nores = Features {
            has_custom_resolution: Some(false),
            is_demo_user: None,
        };
        let game2 = build_arguments(&v, &linux, &f_nores, ArgumentMode::Game, &tokens());
        assert_eq!(game2, vec!["--username", "Steve", "--version", "1.20.4"]);
        // jvm on linux: no osx rule
        let jvm = build_arguments(&v, &linux, &f_nores, ArgumentMode::Jvm, &tokens());
        assert_eq!(jvm, vec!["-Xmx2G"]);
    }

    #[test]
    fn classify_level_detection() {
        assert_eq!(classify_level("ERROR: crash"), LogLevel::Error);
        assert_eq!(classify_level("Exception in thread"), LogLevel::Error);
        assert_eq!(classify_level("WARN: something"), LogLevel::Warn);
        assert_eq!(classify_level("DEBUG detail"), LogLevel::Debug);
        assert_eq!(classify_level("Loading Minecraft 1.20.4"), LogLevel::Info);
    }
}
