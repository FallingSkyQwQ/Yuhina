//! Shared helpers for unit tests (only compiled in test builds).

use std::path::PathBuf;

pub fn load_fixture(name: &str) -> serde_json::Value {
    let json = match name {
        "1.20.4.json" => include_str!("../tests/fixtures/1.20.4.json"),
        "1.12.2.json" => include_str!("../tests/fixtures/1.12.2.json"),
        "assets_1.20.4.json" => include_str!("../tests/fixtures/assets_1.20.4.json"),
        "version_manifest_v2.json" => include_str!("../tests/fixtures/version_manifest_v2.json"),
        "fabric_loader_1.20.4.json" => include_str!("../tests/fixtures/fabric_loader_1.20.4.json"),
        "fabric_installer.json" => include_str!("../tests/fixtures/fabric_installer.json"),
        "quilt_loader_1.20.4.json" => include_str!("../tests/fixtures/quilt_loader_1.20.4.json"),
        "forge_promotions.json" => include_str!("../tests/fixtures/forge_promotions.json"),
        other => panic!("unknown fixture {other}"),
    };
    serde_json::from_str(json).expect("fixture is valid json")
}

/// A throwaway root dir that lives for the whole process (avoids TempDir drop
/// deleting paths that the golden tests only reference as strings).
pub fn temp_game_root() -> PathBuf {
    let dir = tempfile::tempdir().expect("create tempdir");
    let path = dir.path().to_path_buf();
    std::mem::forget(dir);
    path
}
