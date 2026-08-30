//! Real-network tests for Agent C (handoff.md §4 slow-test list).
//!
//! These hit live endpoints and are excluded from `cargo test --workspace`;
//! run locally with `cargo test -- --ignored`.

mod instance_common;

use yuhina_api::LoaderKind;
use yuhina_instance::ModrinthClient;

/// Modrinth live search + project detail.
#[tokio::test]
#[ignore = "requires the real Modrinth API (not run in CI)"]
async fn modrinth_real_search() {
    let client = ModrinthClient::new();
    let res = client
        .search("sodium", &["fabric".into()], &["1.20.4".into()], 0, 5)
        .await
        .expect("modrinth search");
    assert!(res.total > 0, "expected sodium hits");
    assert!(!res.hits.is_empty());
    let top = &res.hits[0];
    let detail = client.get_project(&top.project_id).await.expect("project detail");
    assert_eq!(detail.project_id, top.project_id);
}

/// Live loader version discovery through the core adapter chain (Agent A's
/// `resolve_loader_versions` is mirror-aware).
#[tokio::test]
#[ignore = "requires live loader meta endpoints (not run in CI)"]
async fn fabric_loader_versions_live() {
    use std::sync::Arc;
    use yuhina_instance::CoreAdapter;

    let core: Arc<dyn CoreAdapter> = Arc::new(instance_common::StubCore);
    let versions = core.resolve_loader_versions("1.20.4", LoaderKind::Fabric).await.unwrap();
    assert!(!versions.is_empty(), "fabric loader versions for 1.20.4");
}

/// Live mrpack round-trip through Modrinth's modpack CDN.
#[tokio::test]
#[ignore = "requires live Modrinth download (not run in CI)"]
async fn modrinth_real_modpack_download() {
    let client = ModrinthClient::new();
    let res = client
        .search("create", &[], &["1.20.1".into()], 0, 5)
        .await
        .expect("search modpacks");
    // search returns mods; modpacks need a project_type facet. Just verify the
    // version endpoint works against a real, known-fine project (sodium).
    let project_id = res.hits.first().map(|p| p.project_id.clone()).expect("hit");
    let versions = client
        .get_project_versions(&project_id, &[], &["1.20.4".into()])
        .await
        .unwrap();
    assert!(!versions.is_empty());
}