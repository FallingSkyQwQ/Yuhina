//! Modrinth client integration tests against a mock server (task T3):
//! search pagination mapping, project detail, version filter semantics and
//! the required `User-Agent` header.

mod instance_common;

use instance_common::*;
use yuhina_instance::ModrinthClient;

#[tokio::test]
async fn search_maps_hits_total_and_sends_ua() {
    let mock = MockModrinth::start();
    mock.set_search_hits(vec![
        serde_json::json!({
            "project_id": "AAA", "slug": "a", "title": "Mod A",
            "description": "d", "icon_url": null, "downloads": 100, "follows": 10,
            "loaders": ["fabric"], "game_versions": ["1.20.4"], "categories": ["utility"],
            "versions": ["v1"]
        }),
        serde_json::json!({
            "project_id": "BBB", "slug": "b", "title": "Mod B",
            "description": "d", "icon_url": null, "downloads": 5, "follows": 1,
            "loaders": ["forge"], "game_versions": ["1.20.4"], "categories": ["decoration"],
            "versions": ["v1"]
        }),
    ]);
    let client = ModrinthClient::new_with_base(mock.api_base());

    let res = client
        .search("mod", &["fabric".into()], &["1.20.4".into()], 0, 20)
        .await
        .unwrap();
    assert_eq!(res.total, 2);
    assert_eq!(res.offset, 0);
    assert_eq!(res.hits.len(), 2);
    assert_eq!(res.hits[0].project_id, "AAA");
    assert_eq!(res.hits[0].title, "Mod A");
    assert_eq!(res.hits[0].loaders, vec!["fabric".to_string()]);
    assert_eq!(res.hits[1].follows, 1);

    // UA required by Modrinth.
    let uas = mock.uas_for("/v2/search");
    assert_eq!(uas.len(), 1);
    assert!(
        uas[0].starts_with("yuhina/"),
        "expected yuhina UA, got {:?}",
        uas[0]
    );
    assert!(uas[0].contains("github.com"));
}

#[tokio::test]
async fn project_detail_maps_fields() {
    let mock = MockModrinth::start();
    mock.set_project(
        "AAA",
        serde_json::json!({
            "id": "AAA", "slug": "a", "title": "Mod A", "description": "desc",
            "icon_url": "https://cdn/i.png", "downloads": 1000, "followers": 42,
            "loaders": ["fabric", "quilt"], "game_versions": ["1.20.4", "1.21.1"],
            "categories": ["utility"], "versions": ["v1", "v2"]
        }),
    );
    let client = ModrinthClient::new_with_base(mock.api_base());
    let p = client.get_project("AAA").await.unwrap();
    assert_eq!(p.project_id, "AAA");
    assert_eq!(p.follows, 42);
    assert_eq!(p.downloads, 1000);
    assert_eq!(p.loaders.len(), 2);
    assert_eq!(
        p.game_versions,
        vec!["1.20.4".to_string(), "1.21.1".to_string()]
    );
}

#[tokio::test]
async fn version_filter_applies_loaders_and_game_versions() {
    let mock = MockModrinth::start();
    mock.add_version(
        "AAA",
        version_json(
            "v1",
            "AAA",
            "2024-01-01T00:00:00Z",
            &["1.20.4"],
            &["fabric"],
            &[],
        ),
    );
    mock.add_version(
        "AAA",
        version_json(
            "v2",
            "AAA",
            "2024-02-01T00:00:00Z",
            &["1.20.4"],
            &["forge"],
            &[],
        ),
    );
    mock.add_version(
        "AAA",
        version_json(
            "v3",
            "AAA",
            "2024-03-01T00:00:00Z",
            &["1.21.1"],
            &["fabric"],
            &[],
        ),
    );
    let client = ModrinthClient::new_with_base(mock.api_base());

    let versions = client
        .get_project_versions("AAA", &["fabric".into()], &["1.20.4".into()])
        .await
        .unwrap();
    let ids: Vec<&str> = versions.iter().map(|v| v.version_id.as_str()).collect();
    assert_eq!(ids, vec!["v1"], "only fabric + 1.20.4 version matches");

    // Unfiltered returns everything.
    let all = client.get_project_versions("AAA", &[], &[]).await.unwrap();
    assert_eq!(all.len(), 3);
}

#[tokio::test]
async fn version_detail_maps_dependencies_and_files() {
    let mock = MockModrinth::start();
    mock.add_version(
        "AAA",
        serde_json::json!({
            "id": "v1", "project_id": "AAA", "name": "n", "version_number": "1.0",
            "game_versions": ["1.20.4"], "loaders": ["fabric"],
            "files": [{
                "name": "a-1.0.jar", "size": 10, "url": "http://cdn/a-1.0.jar",
                "hashes": {"sha1": "deadbeef"}
            }],
            "dependencies": [
                {"project_id": "BBB", "dependency_type": "required"},
                {"project_id": "CCC", "dependency_type": "incompatible"}
            ],
            "date_published": "2024-01-01T00:00:00Z"
        }),
    );
    let client = ModrinthClient::new_with_base(mock.api_base());
    let (pid, v) = client.get_version_with_project("v1").await.unwrap();
    assert_eq!(pid, "AAA");
    assert_eq!(v.version_id, "v1");
    assert_eq!(v.files[0].name, "a-1.0.jar");
    assert_eq!(v.files[0].sha1, "deadbeef");
    assert_eq!(v.dependencies.len(), 2);
    assert_eq!(v.dependencies[0].dep_type, "required");
    assert_eq!(v.dependencies[0].project_id.as_deref(), Some("BBB"));
    assert_eq!(v.dependencies[1].dep_type, "incompatible");
}

#[tokio::test]
async fn http_error_surfaces_http_kind() {
    let mock = MockModrinth::start();
    let client = ModrinthClient::new_with_base(mock.api_base());
    let err = client.get_project("GHOST").await.unwrap_err();
    assert_eq!(
        err.kind,
        yuhina_api::YuhinaErrorKind::Http(404, mock.api_base() + "/project/GHOST")
    );
}
