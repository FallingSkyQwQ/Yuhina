//! Version manifest (list) fetching, caching and per-version json retrieval.

use serde_json::Value;
use yuhina_api::{VersionMeta, YuhinaError};

use crate::download::Downloader;

/// Official version manifest URL.
pub const VERSION_MANIFEST_URL: &str =
    "https://piston-meta.mojang.com/mc/game/version_manifest_v2.json";

/// Raw entry of the version list.
#[derive(Debug, Clone, serde::Deserialize)]
pub struct ManifestEntry {
    pub id: String,
    #[serde(rename = "type")]
    pub version_type: String,
    pub url: String,
    #[serde(default)]
    pub time: String,
    #[serde(default)]
    pub release_time: String,
}

/// Parsed `version_manifest_v2.json`.
#[derive(Debug, Clone, serde::Deserialize)]
pub struct VersionManifestList {
    pub latest: LatestVersions,
    pub versions: Vec<ManifestEntry>,
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct LatestVersions {
    pub release: String,
    pub snapshot: String,
}

impl VersionManifestList {
    pub fn parse(raw: &Value) -> Result<Self, YuhinaError> {
        serde_json::from_value(raw.clone())
            .map_err(|e| YuhinaError::internal(format!("parse version manifest: {e}")))
    }

    /// Convert raw entries into contract `VersionMeta`, marking latest.
    pub fn to_meta(&self) -> Vec<VersionMeta> {
        self.versions
            .iter()
            .map(|e| VersionMeta {
                id: e.id.clone(),
                version_type: e.version_type.clone(),
                release_time: e.release_time.clone(),
                url: e.url.clone(),
                is_latest_release: e.id == self.latest.release,
                is_latest_snapshot: e.id == self.latest.snapshot,
            })
            .collect()
    }
}

/// Fetch the manifest through a `Downloader` (mirror-aware).
pub async fn fetch_version_list(downloader: &dyn Downloader) -> Result<VersionManifestList, YuhinaError> {
    let bytes = downloader.fetch_bytes(VERSION_MANIFEST_URL).await?;
    let value: Value = serde_json::from_slice(&bytes)
        .map_err(|e| YuhinaError::internal(format!("parse manifest json: {e}")))?;
    VersionManifestList::parse(&value)
}

/// Get a cached version json `Value` for `id`, or fetch + return it.
pub async fn get_version_json(
    downloader: &dyn Downloader,
    cache: &dyn VersionJsonCache,
    id: &str,
) -> Result<Value, YuhinaError> {
    if let Some(v) = cache.get_version_json(id) {
        return Ok(v);
    }
    let url = cache
        .get_version_url(id)
        .ok_or_else(|| YuhinaError::not_found_version(id))?;
    let bytes = downloader.fetch_bytes(&url).await?;
    let value: Value = serde_json::from_slice(&bytes)
        .map_err(|e| YuhinaError::internal(format!("parse version json {id}: {e}")))?;
    cache.put_version_json(id, value.clone());
    Ok(value)
}

/// Abstraction over the version json cache (implemented by `YuhinaCore`).
pub trait VersionJsonCache: Send + Sync {
    fn get_version_json(&self, id: &str) -> Option<Value>;
    fn put_version_json(&self, id: &str, value: Value);
    fn get_version_url(&self, id: &str) -> Option<String>;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testutil::load_fixture;

    #[test]
    fn parse_manifest_fixture() {
        let raw = load_fixture("version_manifest_v2.json");
        let list = VersionManifestList::parse(&raw).unwrap();
        assert!(list.versions.len() > 500, "count {}", list.versions.len());
        assert_eq!(list.latest.release, raw["latest"]["release"].as_str().unwrap());
        let meta = list.to_meta();
        assert_eq!(meta.len(), list.versions.len());
        assert!(meta.iter().any(|m| m.is_latest_release));
        assert!(meta.iter().any(|m| m.is_latest_snapshot));
        let m = meta.iter().find(|m| m.id == "1.20.4").unwrap();
        assert_eq!(m.version_type, "release");
        assert!(m.url.starts_with("https://"));
    }
}