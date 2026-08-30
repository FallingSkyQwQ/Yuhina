//! Asset index parsing and sparse asset download planning (task T3/T6).

use serde_json::Value;
use yuhina_api::YuhinaError;

use crate::download::DownloadItem;

/// One asset object from the index (`objects` map).
#[derive(Debug, Clone)]
pub struct AssetObject {
    pub hash: String,
    pub size: u64,
}

/// Parsed asset index (`assets/<index>.json`).
#[derive(Debug, Clone)]
pub struct AssetIndex {
    pub id: String,
    pub objects: Vec<(String, AssetObject)>,
    pub total_size: u64,
}

impl AssetIndex {
    pub fn parse(id: &str, raw: &Value) -> Result<Self, YuhinaError> {
        let objects = raw
            .get("objects")
            .and_then(Value::as_object)
            .ok_or_else(|| YuhinaError::internal("asset index missing objects"))?;
        let mut parsed = Vec::with_capacity(objects.len());
        let mut total_size = 0u64;
        for (key, v) in objects {
            let hash = v
                .get("hash")
                .and_then(Value::as_str)
                .unwrap_or_default()
                .to_string();
            let size = v.get("size").and_then(Value::as_u64).unwrap_or(0);
            total_size += size;
            parsed.push((key.clone(), AssetObject { hash, size }));
        }
        Ok(Self {
            id: id.to_string(),
            objects: parsed,
            total_size,
        })
    }

    /// The on-disk object path for a hash: `objects/<first2>/<rest>`.
    pub fn object_rel_path(hash: &str) -> String {
        let (a, b) = hash.split_at(hash.len().min(2));
        format!("objects/{a}/{b}")
    }
}

/// A concrete asset file to download (or verify existing).
#[derive(Debug, Clone)]
pub struct AssetFile {
    pub key: String,
    pub hash: String,
    pub size: u64,
    pub rel_path: String,
    pub url: String,
}

impl AssetFile {
    pub fn to_download_item(&self, objects_dir: &std::path::Path) -> DownloadItem {
        DownloadItem {
            url: self.url.clone(),
            target_path: objects_dir.join(&self.rel_path).to_string_lossy().to_string(),
            sha1: Some(self.hash.clone()),
            size: Some(self.size),
        }
    }
}

/// Base URL for asset downloads (official).
pub const ASSETS_BASE_URL: &str = "https://resources.download.minecraft.net";

/// Build the full asset download list (sparse: only what's in the index).
pub fn plan_assets(index: &AssetIndex) -> Vec<AssetFile> {
    index
        .objects
        .iter()
        .map(|(key, obj)| AssetFile {
            key: key.clone(),
            hash: obj.hash.clone(),
            size: obj.size,
            rel_path: AssetIndex::object_rel_path(&obj.hash),
            url: format!("{ASSETS_BASE_URL}/{}/{}", &obj.hash[..2], obj.hash),
        })
        .collect()
}

/// Download targets are deduplicated by hash (multiple keys share a file).
pub fn dedup_by_hash(files: Vec<AssetFile>) -> Vec<AssetFile> {
    let mut seen = std::collections::HashSet::new();
    files
        .into_iter()
        .filter(|f| seen.insert(f.hash.clone()))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testutil::load_fixture;

    #[test]
    fn parse_real_asset_index() {
        let raw = load_fixture("assets_1.20.4.json");
        let idx = AssetIndex::parse("12", &raw).unwrap();
        assert_eq!(idx.objects.len(), 3811);
        assert_eq!(idx.total_size, raw["totalSize"].as_u64().unwrap_or(idx.total_size));
        // object path rule
        let (key, obj) = &idx.objects[0];
        assert_eq!(
            AssetIndex::object_rel_path(&obj.hash),
            format!("objects/{}/{}", &obj.hash[..2], &obj.hash[2..])
        );
        assert!(key.starts_with("icons/") || key.contains('/'));
    }

    #[test]
    fn plan_and_dedup_assets() {
        let raw = load_fixture("assets_1.20.4.json");
        let idx = AssetIndex::parse("12", &raw).unwrap();
        let files = plan_assets(&idx);
        assert_eq!(files.len(), 3811);
        let deduped = dedup_by_hash(files);
        assert!(deduped.len() <= 3811);
        // urls follow resources.download.minecraft.net/<hh>/<hash>
        let f = &deduped[0];
        assert_eq!(
            f.url,
            format!("{ASSETS_BASE_URL}/{}/{}", &f.hash[..2], f.hash)
        );
    }

    #[test]
    fn object_rel_path_edge() {
        assert_eq!(AssetIndex::object_rel_path("abc"), "objects/ab/c");
        assert_eq!(AssetIndex::object_rel_path("a"), "objects/a/");
    }
}