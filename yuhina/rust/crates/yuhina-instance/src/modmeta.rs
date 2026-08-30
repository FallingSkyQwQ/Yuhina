//! Mod jar metadata parsing (task T4).
//!
//! Reads `fabric.mod.json`, `quilt.mod.json`, `META-INF/neoforge.mods.toml`
//! and `META-INF/mods.toml` from a jar/zip and extracts
//! name/modid/description/loaders/mc_versions. Files with no recognisable
//! metadata fall back to an `Unknown` entry that can still be enabled or
//! disabled.

use std::fs::File;
use std::path::Path;

use serde_json::Value;

/// Metadata parsed from a mod jar. All fields are best-effort.
#[derive(Debug, Clone, Default)]
pub struct ModMeta {
    pub name: String,
    pub modid: String,
    pub description: String,
    pub loaders: Vec<String>,
    pub mc_versions: Vec<String>,
}

/// Parse a mod jar. Never fails: unreadable/unparseable jars yield an
/// `Unknown` entry keyed by the file stem.
pub fn parse_mod_metadata(jar_path: &Path) -> ModMeta {
    let fallback_name = jar_path
        .file_stem()
        .map(|s| s.to_string_lossy().to_string())
        .unwrap_or_else(|| "Unknown".into());
    let file = match File::open(jar_path) {
        Ok(f) => f,
        Err(_) => return unknown(fallback_name),
    };
    let mut archive = match zip::ZipArchive::new(file) {
        Ok(a) => a,
        Err(_) => return unknown(fallback_name),
    };

    // Priority: quilt > fabric > neoforge > forge (mixed jars resolve to the
    // more specific loader).
    let candidates = [
        ("quilt.mod.json", MetaFormat::Json, "quilt"),
        ("fabric.mod.json", MetaFormat::Json, "fabric"),
        ("META-INF/neoforge.mods.toml", MetaFormat::Toml, "neoforge"),
        ("META-INF/mods.toml", MetaFormat::Toml, "forge"),
    ];
    for (name, format, loader_tag) in candidates {
        let mut meta = match read_entry(&mut archive, name, format, loader_tag) {
            Some(m) => m,
            None => continue,
        };
        if meta.modid.is_empty() {
            meta.modid = meta.name.clone();
        }
        if meta.name.is_empty() {
            meta.name = fallback_name.clone();
        }
        meta.loaders.push(loader_tag.to_string());
        return meta;
    }
    unknown(fallback_name)
}

fn unknown(name: String) -> ModMeta {
    ModMeta {
        name,
        ..Default::default()
    }
}

enum MetaFormat {
    Json,
    Toml,
}

fn read_entry(
    archive: &mut zip::ZipArchive<File>,
    name: &str,
    format: MetaFormat,
    loader_tag: &str,
) -> Option<ModMeta> {
    let mut entry = archive.by_name(name).ok()?;
    if entry.is_dir() {
        return None;
    }
    let mut bytes = Vec::new();
    std::io::Read::read_to_end(&mut entry, &mut bytes).ok()?;
    match format {
        MetaFormat::Json => parse_fabric_json(&bytes),
        MetaFormat::Toml => parse_mods_toml(&bytes, loader_tag),
    }
}

fn parse_fabric_json(bytes: &[u8]) -> Option<ModMeta> {
    let v: Value = serde_json::from_slice(bytes).ok()?;
    let modid = v.get("id").and_then(Value::as_str).unwrap_or_default();
    let name = v
        .get("name")
        .and_then(Value::as_str)
        .unwrap_or(modid)
        .to_string();
    let description = v
        .get("description")
        .and_then(Value::as_str)
        .unwrap_or_default()
        .to_string();
    let mc_versions = v
        .get("depends")
        .and_then(|d| d.get("minecraft"))
        .map(extract_mc_versions)
        .unwrap_or_default();
    Some(ModMeta {
        name,
        modid: modid.to_string(),
        description,
        loaders: Vec::new(),
        mc_versions,
    })
}

fn parse_mods_toml(bytes: &[u8], _loader_tag: &str) -> Option<ModMeta> {
    let v: toml::Value = toml::from_slice(bytes).ok()?;
    let mods = v.get("mods")?.as_array()?;
    let first = mods.first()?;
    let modid = first
        .get("modId")
        .and_then(|m| m.as_str())
        .unwrap_or_default();
    let name = first
        .get("displayName")
        .and_then(|m| m.as_str())
        .unwrap_or(modid)
        .to_string();
    let description = first
        .get("description")
        .and_then(|m| m.as_str())
        .unwrap_or_default()
        .to_string();
    let mut mc_versions: Vec<String> = Vec::new();
    // `[[dependencies.<modid>]]` blocks live at the top level of the toml.
    if let Some(deps) = v.get("dependencies").and_then(|d| d.as_table()) {
        for dep_arr in deps.values() {
            let Some(arr) = dep_arr.as_array() else {
                continue;
            };
            for dep in arr {
                if dep.get("modId").and_then(|d| d.as_str()) != Some("minecraft") {
                    continue;
                }
                if let Some(range) = dep.get("versionRange").and_then(|d| d.as_str()) {
                    for tok in version_tokens(range) {
                        if !mc_versions.contains(&tok) {
                            mc_versions.push(tok);
                        }
                    }
                }
            }
        }
    }
    Some(ModMeta {
        name,
        modid: modid.to_string(),
        description,
        loaders: Vec::new(),
        mc_versions,
    })
}

/// Extract concrete MC version strings from a `depends.minecraft` value.
fn extract_mc_versions(value: &Value) -> Vec<String> {
    match value {
        Value::String(s) => version_tokens(s),
        Value::Array(arr) => arr
            .iter()
            .filter_map(Value::as_str)
            .flat_map(version_tokens)
            .collect(),
        _ => Vec::new(),
    }
}

/// Split a range/expression like `[1.20.4,1.21)` or `>=1.20.4` into the
/// version-like tokens it mentions.
fn version_tokens(s: &str) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();
    for piece in s.split(|c: char| !(c.is_ascii_alphanumeric() || c == '.')) {
        let p = piece.trim_matches('.');
        if is_versionish(p) && !out.iter().any(|o| o == p) {
            out.push(p.to_string());
        }
    }
    out
}

fn is_versionish(s: &str) -> bool {
    if s.is_empty() || !s.starts_with(|c: char| c.is_ascii_digit()) {
        return false;
    }
    s.contains('.')
        && s.chars()
            .all(|c| c.is_ascii_digit() || c == '.' || c.is_ascii_alphabetic())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn write_jar(dir: &std::path::Path, entries: &[(&str, &[u8])]) -> std::path::PathBuf {
        let path = dir.join("mod.jar");
        let file = File::create(&path).unwrap();
        let mut zip = zip::ZipWriter::new(file);
        for (name, data) in entries {
            zip.start_file(*name, zip::write::SimpleFileOptions::default())
                .unwrap();
            std::io::Write::write_all(&mut zip, data).unwrap();
        }
        zip.finish().unwrap();
        path
    }

    #[test]
    fn fabric_mod_json() {
        let dir = tempfile::tempdir().unwrap();
        let jar = write_jar(
            dir.path(),
            &[(
                "fabric.mod.json",
                br#"{"id":"myfabmod","name":"My Fab Mod","description":"desc","depends":{"minecraft":["1.20.4","1.20.1"],"fabricloader":">=0.15"}}"#,
            )],
        );
        let meta = parse_mod_metadata(&jar);
        assert_eq!(meta.modid, "myfabmod");
        assert_eq!(meta.name, "My Fab Mod");
        assert_eq!(meta.loaders, vec!["fabric"]);
        assert!(meta.mc_versions.contains(&"1.20.4".to_string()));
        assert!(meta.mc_versions.contains(&"1.20.1".to_string()));
    }

    #[test]
    fn quilt_mod_json() {
        let dir = tempfile::tempdir().unwrap();
        let jar = write_jar(
            dir.path(),
            &[(
                "quilt.mod.json",
                br#"{"id":"myquilmod","name":"Q Mod","depends":{"minecraft":"~1.20.4"}}"#,
            )],
        );
        let meta = parse_mod_metadata(&jar);
        assert_eq!(meta.modid, "myquilmod");
        assert_eq!(meta.loaders, vec!["quilt"]);
        assert!(meta.mc_versions.contains(&"1.20.4".to_string()));
    }

    #[test]
    fn forge_mods_toml() {
        let dir = tempfile::tempdir().unwrap();
        let toml = br#"
modLoader="javafml"
loaderVersion="[1,)"
[[mods]]
modId="myforgemod"
displayName="Forge Mod"
description="forge desc"
[[dependencies.myforgemod]]
modId="minecraft"
versionRange="[1.20.4,1.21)"
mandatory=true
"#;
        let jar = write_jar(dir.path(), &[("META-INF/mods.toml", toml)]);
        let meta = parse_mod_metadata(&jar);
        assert_eq!(meta.modid, "myforgemod");
        assert_eq!(meta.name, "Forge Mod");
        assert_eq!(meta.loaders, vec!["forge"]);
        assert!(meta.mc_versions.contains(&"1.20.4".to_string()));
    }

    #[test]
    fn neoforge_toml_wins_over_forge() {
        let dir = tempfile::tempdir().unwrap();
        let neoforge = br#"
modLoader="javafml"
[[mods]]
modId="myneomod"
displayName="Neo Mod"
"#;
        let forge = br#"
modLoader="javafml"
[[mods]]
modId="legacyforge"
displayName="Legacy"
"#;
        let jar = write_jar(
            dir.path(),
            &[
                ("META-INF/neoforge.mods.toml", neoforge),
                ("META-INF/mods.toml", forge),
            ],
        );
        let meta = parse_mod_metadata(&jar);
        assert_eq!(meta.modid, "myneomod");
        assert_eq!(meta.loaders, vec!["neoforge"]);
    }

    #[test]
    fn unknown_jar_falls_back() {
        let dir = tempfile::tempdir().unwrap();
        let jar = write_jar(dir.path(), &[("random.txt", b"hello")]);
        let meta = parse_mod_metadata(&jar);
        assert_eq!(meta.name, "mod");
        assert!(meta.modid.is_empty());
        assert!(meta.loaders.is_empty());
    }
}
