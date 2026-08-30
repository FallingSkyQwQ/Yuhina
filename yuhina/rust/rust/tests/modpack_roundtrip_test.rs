//! `.mrpack` round-trip integration tests (task T7/T8): export → re-import
//! yields an identical file set (including a local non-Modrinth mod and the
//! `env.client.optional` → `.disabled` path), plus Modrinth pack install.

mod instance_common;

use std::path::PathBuf;
use std::sync::Arc;

use instance_common::*;
use yuhina_api::{CreateInstanceRequest, JavaSelection, Loader, LoaderKind};
use yuhina_core::download::Downloader;
use yuhina_db::Db;
use yuhina_instance::{InstanceManager, ModrinthClient};

fn manager(mock: &MockModrinth, game_root: PathBuf) -> (InstanceManager, Db) {
    let db = Db::in_memory().unwrap();
    let core: Arc<dyn yuhina_instance::CoreAdapter> = Arc::new(StubCore);
    let dl: Arc<dyn Downloader> = Arc::new(MockHttpDownloader);
    let m = InstanceManager::with_modrinth(
        db.clone(),
        core,
        dl,
        game_root,
        ModrinthClient::new_with_base(mock.api_base()),
    );
    (m, db)
}

async fn create_pack_instance(m: &InstanceManager) -> yuhina_api::InstanceSummary {
    m.create_instance(CreateInstanceRequest {
        name: "packbase".into(),
        icon: String::new(),
        mc_version: "1.20.4".into(),
        loader: Some(Loader {
            kind: LoaderKind::Fabric,
            version: "0.16.0".into(),
        }),
        java: JavaSelection::Auto(21),
        dir_name: None,
    })
    .await
    .unwrap()
}

#[tokio::test]
async fn roundtrip_preserves_mods_and_overrides() {
    let mock = MockModrinth::start();
    let dir = tempfile::tempdir().unwrap();
    let game_root = dir.path().join("game");

    // Modrinth-linked mod: a real fabric jar served by the mock CDN.
    let jar_bytes = std::fs::read(fabric_jar(dir.path(), "linked.jar", "linkedmod")).unwrap();
    let sha1 = yuhina_download::checksum::sha1_hex(&jar_bytes);
    mock.add_file("/files/linked.jar", jar_bytes.clone());
    mock.add_version(
        "LINKED",
        serde_json::json!({
            "id": "lv1", "project_id": "LINKED", "name": "Linked", "version_number": "1.0",
            "game_versions": ["1.20.4"], "loaders": ["fabric"],
            "files": [{
                "name": "linked.jar", "size": jar_bytes.len(),
                "url": mock.url("/files/linked.jar"), "hashes": {"sha1": sha1}
            }],
            "dependencies": [], "date_published": "2024-01-01T00:00:00Z"
        }),
    );

    let (m, _db) = manager(&mock, game_root.clone());
    let s = create_pack_instance(&m).await;

    // 1. install the Modrinth-linked mod (auto-selects lv1)
    let linked = m
        .install_mod(s.id.clone(), "LINKED".into(), None)
        .await
        .unwrap();
    assert_eq!(linked.project_id.as_deref(), Some("LINKED"));
    assert_eq!(linked.version_id.as_deref(), Some("lv1"));

    // 2. install a local (non-Modrinth) mod, then disable it → optional in pack
    let local_jar = fabric_jar(dir.path(), "local.jar", "localmod");
    let local = m
        .install_mod_file(s.id.clone(), local_jar.to_string_lossy().into())
        .await
        .unwrap();
    m.set_mod_enabled(s.id.clone(), local.id.clone(), false)
        .await
        .unwrap();

    // 3. an override file (config/menu.json)
    let game_dir = PathBuf::from(m.get_instance(s.id.clone()).await.unwrap().game_dir);
    std::fs::create_dir_all(game_dir.join("config")).unwrap();
    std::fs::write(game_dir.join("config/menu.json"), b"{}").unwrap();

    // export
    let dest = dir.path().join("pack.mrpack");
    let out = m
        .export_modpack(s.id.clone(), dest.to_string_lossy().into())
        .await
        .unwrap();
    assert!(PathBuf::from(&out).exists());

    // inspect the archive
    let file = std::fs::File::open(&out).unwrap();
    let mut archive = zip::ZipArchive::new(file).unwrap();
    let mut index_json = String::new();
    {
        let mut entry = archive.by_name("index.json").unwrap();
        std::io::Read::read_to_string(&mut entry, &mut index_json).unwrap();
    }
    let index: serde_json::Value = serde_json::from_str(&index_json).unwrap();
    assert_eq!(index["formatVersion"], 1);
    assert_eq!(index["versionId"], "1.20.4");
    assert_eq!(index["modloaders"][0]["id"], "fabric-0.16.0");
    let paths: Vec<&str> = index["files"]
        .as_array()
        .unwrap()
        .iter()
        .map(|f| f["path"].as_str().unwrap())
        .collect();
    assert!(paths.contains(&"mods/linked.jar"));
    assert!(paths.contains(&"mods/local.jar"));
    let linked_entry = index["files"]
        .as_array()
        .unwrap()
        .iter()
        .find(|f| f["path"] == "mods/linked.jar")
        .unwrap();
    assert!(
        !linked_entry["downloads"].as_array().unwrap().is_empty(),
        "linked mod has a download url"
    );
    assert_eq!(linked_entry["env"]["client"], "required");
    let local_entry = index["files"]
        .as_array()
        .unwrap()
        .iter()
        .find(|f| f["path"] == "mods/local.jar")
        .unwrap();
    assert!(
        local_entry["downloads"].as_array().unwrap().is_empty(),
        "local mod has no url"
    );
    assert_eq!(local_entry["env"]["client"], "optional");
    // local mod content also shipped in overrides
    let names: Vec<String> = (0..archive.len())
        .map(|i| archive.by_index(i).unwrap().name().to_string())
        .collect();
    assert!(
        names.iter().any(|n| n == "overrides/mods/local.jar"),
        "local mod in overrides"
    );
    assert!(
        names.iter().any(|n| n == "overrides/config/menu.json"),
        "override file packed"
    );

    // import into a fresh instance
    let imported = m.import_modpack(out, "Imported".into()).await.unwrap();
    assert_eq!(imported.mc_version, "1.20.4");
    assert_eq!(imported.loader.as_ref().unwrap().kind, LoaderKind::Fabric);
    let mods = m.list_mods(imported.id.clone()).await;
    assert_eq!(mods.len(), 2);
    let imported_dir = PathBuf::from(m.get_instance(imported.id.clone()).await.unwrap().game_dir);
    assert!(imported_dir.join("mods/linked.jar").exists());
    assert!(
        imported_dir.join("mods/.disabled/local.jar").exists(),
        "optional mod lands disabled"
    );
    assert!(
        imported_dir.join("config/menu.json").exists(),
        "override applied"
    );
    let local_imported = mods.iter().find(|x| x.file_name == "local.jar").unwrap();
    assert!(!local_imported.enabled);
    let linked_imported = mods.iter().find(|x| x.file_name == "linked.jar").unwrap();
    assert!(linked_imported.enabled);

    // byte-identical file set: linked.jar sha1 must match the source
    assert_eq!(
        yuhina_download::checksum::sha1_hex_file(&imported_dir.join("mods/linked.jar")).unwrap(),
        sha1
    );

    let _ = dir;
}

#[tokio::test]
async fn download_modpack_from_modrinth_imports() {
    let mock = MockModrinth::start();
    let dir = tempfile::tempdir().unwrap();
    let game_root = dir.path().join("game");
    let (m, _db) = manager(&mock, game_root.clone());

    // Build a real mrpack via export, then serve it as a Modrinth version file.
    let s = create_pack_instance(&m).await;
    let jar_bytes = std::fs::read(fabric_jar(dir.path(), "c.jar", "cm")).unwrap();
    let sha1 = yuhina_download::checksum::sha1_hex(&jar_bytes);
    mock.add_file("/files/c.jar", jar_bytes.clone());
    mock.add_version(
        "C",
        serde_json::json!({
            "id": "cv1", "project_id": "C", "name": "C", "version_number": "1",
            "game_versions": ["1.20.4"], "loaders": ["fabric"],
            "files": [{"name": "c.jar", "size": jar_bytes.len(), "url": mock.url("/files/c.jar"),
                        "hashes": {"sha1": sha1}}],
            "dependencies": [], "date_published": "2024-01-01T00:00:00Z"
        }),
    );
    m.install_mod(s.id.clone(), "C".into(), None).await.unwrap();

    let pack_path = dir.path().join("p.mrpack");
    let out = m
        .export_modpack(s.id.clone(), pack_path.to_string_lossy().into())
        .await
        .unwrap();
    let pack_bytes = std::fs::read(&out).unwrap();
    let pack_sha1 = yuhina_download::checksum::sha1_hex(&pack_bytes);
    let pack_size = pack_bytes.len();
    mock.add_file("/files/pack.mrpack", pack_bytes);
    mock.set_project(
        "PACK",
        serde_json::json!({
            "id": "PACK", "slug": "pack", "title": "The Pack", "description": "d",
            "icon_url": null, "downloads": 1, "followers": 1, "loaders": ["fabric"],
            "game_versions": ["1.20.4"], "categories": [], "versions": ["pv1"]
        }),
    );
    mock.add_version(
        "PACK",
        serde_json::json!({
            "id": "pv1", "project_id": "PACK", "name": "Pack v1", "version_number": "1.0",
            "game_versions": ["1.20.4"], "loaders": ["fabric"],
            "files": [{"name": "pack.mrpack", "size": pack_size,
                        "url": mock.url("/files/pack.mrpack"), "hashes": {"sha1": pack_sha1}}],
            "dependencies": [], "date_published": "2024-02-01T00:00:00Z"
        }),
    );

    let imported = m
        .download_modpack_from_modrinth("PACK".into(), "pv1".into())
        .await
        .unwrap();
    assert_eq!(imported.name, "The Pack");
    let mods = m.list_mods(imported.id.clone()).await;
    assert_eq!(mods.len(), 1);
    let game_dir = PathBuf::from(m.get_instance(imported.id.clone()).await.unwrap().game_dir);
    assert!(game_dir.join("mods/c.jar").exists());

    let _ = dir;
}
