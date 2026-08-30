//! Dependency resolution integration tests against a mock Modrinth (task T5):
//! cycle (闭环), missing dependency, incompatible dependency and the
//! "newest published compatible version" selection strategy.

mod instance_common;

use instance_common::*;
use yuhina_api::{InstalledMod, InstanceSummary, Loader, LoaderKind};
use yuhina_instance::dependency::{version_compatible, DependencyResolver};
use yuhina_instance::ModrinthClient;

fn fabric_instance() -> InstanceSummary {
    InstanceSummary {
        id: "i".into(),
        name: "n".into(),
        icon: "".into(),
        mc_version: "1.20.4".into(),
        loader: Some(Loader {
            kind: LoaderKind::Fabric,
            version: "0.16.0".into(),
        }),
        is_installed: true,
        last_launched_at: None,
        mod_count: 0,
        total_size_bytes: 0,
        created_at: 0,
        updated_at: 0,
    }
}

fn resolver(mock: &MockModrinth) -> DependencyResolver {
    DependencyResolver::new(ModrinthClient::new_with_base(mock.api_base()))
}

#[tokio::test]
async fn latest_compatible_picks_newest() {
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
            "2024-06-01T00:00:00Z",
            &["1.20.4"],
            &["fabric"],
            &[],
        ),
    );
    mock.add_version(
        "AAA",
        version_json(
            "v3",
            "AAA",
            "2024-06-01T00:00:00Z",
            &["1.21.1"],
            &["fabric"],
            &[],
        ),
    );
    let r = resolver(&mock);
    let latest = r
        .latest_compatible("AAA", &fabric_instance())
        .await
        .unwrap()
        .unwrap();
    assert_eq!(latest.version_id, "v2", "newest compatible published wins");
}

#[tokio::test]
async fn compatible_filter_respects_loader_and_mc() {
    let mock = MockModrinth::start();
    mock.add_version(
        "AAA",
        version_json(
            "v1",
            "AAA",
            "2024-01-01T00:00:00Z",
            &["1.20.4"],
            &["forge"],
            &[],
        ),
    );
    let r = resolver(&mock);
    let latest = r
        .latest_compatible("AAA", &fabric_instance())
        .await
        .unwrap();
    assert!(
        latest.is_none(),
        "forge version is not compatible with a fabric instance"
    );
}

#[tokio::test]
async fn required_dependency_cycle_terminates() {
    let mock = MockModrinth::start();
    // A → B → A (cycle)
    mock.add_version(
        "A",
        version_json(
            "a1",
            "A",
            "2024-01-01T00:00:00Z",
            &["1.20.4"],
            &["fabric"],
            &[dep("B", "required")],
        ),
    );
    mock.add_version(
        "B",
        version_with_file(
            "b1",
            "B",
            "2024-01-01T00:00:00Z",
            &["1.20.4"],
            &["fabric"],
            &[dep("A", "required")],
            &mock.url("/files/b.jar"),
            "b1sha1",
        ),
    );
    let client = ModrinthClient::new_with_base(mock.api_base());
    let root = client.get_version("a1").await.unwrap();
    let resolved = resolver(&mock)
        .resolve_required(&root, Some("A"), &fabric_instance(), &[])
        .await
        .unwrap();
    assert!(resolved.missing.is_empty());
    assert_eq!(
        resolved.to_install.len(),
        1,
        "cycle collapses to a single dependency"
    );
    assert_eq!(resolved.to_install[0].project_id, "B");
}

#[tokio::test]
async fn missing_required_dependency_reported() {
    let mock = MockModrinth::start();
    mock.add_version(
        "A",
        version_json(
            "a1",
            "A",
            "2024-01-01T00:00:00Z",
            &["1.20.4"],
            &["fabric"],
            &[dep("GHOST", "required")],
        ),
    );
    let client = ModrinthClient::new_with_base(mock.api_base());
    let root = client.get_version("a1").await.unwrap();
    let resolved = resolver(&mock)
        .resolve_required(&root, Some("A"), &fabric_instance(), &[])
        .await
        .unwrap();
    assert!(resolved.to_install.is_empty());
    assert_eq!(resolved.missing.len(), 1);
    assert_eq!(resolved.missing[0].project_id.as_deref(), Some("GHOST"));
}

#[tokio::test]
async fn incompatible_dependency_on_installed_mod_reported() {
    let mock = MockModrinth::start();
    mock.add_version(
        "A",
        version_json(
            "a1",
            "A",
            "2024-01-01T00:00:00Z",
            &["1.20.4"],
            &["fabric"],
            &[dep("B", "incompatible")],
        ),
    );
    let client = ModrinthClient::new_with_base(mock.api_base());
    let root = client.get_version("a1").await.unwrap();
    let installed = vec![InstalledMod {
        id: "x".into(),
        file_name: "b.jar".into(),
        file_size: 1,
        sha1: "x".into(),
        name: "B".into(),
        modid: "b".into(),
        description: String::new(),
        loaders: vec![],
        mc_versions: vec![],
        project_id: Some("B".into()),
        version_id: Some("b1".into()),
        enabled: true,
        installed_at: 0,
    }];
    let resolved = resolver(&mock)
        .resolve_required(&root, Some("A"), &fabric_instance(), &installed)
        .await
        .unwrap();
    assert_eq!(resolved.incompatible.len(), 1);
    assert_eq!(resolved.incompatible[0].project_id.as_deref(), Some("B"));
    assert_eq!(resolved.incompatible[0].dep_type, "incompatible");
}

#[tokio::test]
async fn already_installed_required_dependency_skipped() {
    let mock = MockModrinth::start();
    mock.add_version(
        "A",
        version_json(
            "a1",
            "A",
            "2024-01-01T00:00:00Z",
            &["1.20.4"],
            &["fabric"],
            &[dep("B", "required")],
        ),
    );
    mock.add_version(
        "B",
        version_json(
            "b1",
            "B",
            "2024-01-01T00:00:00Z",
            &["1.20.4"],
            &["fabric"],
            &[],
        ),
    );
    let client = ModrinthClient::new_with_base(mock.api_base());
    let root = client.get_version("a1").await.unwrap();
    let installed = vec![InstalledMod {
        id: "x".into(),
        file_name: "b.jar".into(),
        file_size: 1,
        sha1: "x".into(),
        name: "B".into(),
        modid: "b".into(),
        description: String::new(),
        loaders: vec![],
        mc_versions: vec![],
        project_id: Some("B".into()),
        version_id: Some("b1".into()),
        enabled: true,
        installed_at: 0,
    }];
    let resolved = resolver(&mock)
        .resolve_required(&root, Some("A"), &fabric_instance(), &installed)
        .await
        .unwrap();
    assert!(
        resolved.to_install.is_empty(),
        "already-installed dep not re-queued"
    );
    assert!(resolved.missing.is_empty());
}

#[test]
fn version_compatible_logic() {
    let inst = fabric_instance();
    let v = contract_version(&version_json(
        "v",
        "P",
        "2024-01-01T00:00:00Z",
        &["1.20.4"],
        &["fabric"],
        &[],
    ));
    assert!(version_compatible(&v, &inst));
    let forge = contract_version(&version_json(
        "v",
        "P",
        "2024-01-01T00:00:00Z",
        &["1.20.4"],
        &["forge"],
        &[],
    ));
    assert!(!version_compatible(&forge, &inst));
    let wrong_mc = contract_version(&version_json(
        "v",
        "P",
        "2024-01-01T00:00:00Z",
        &["1.21.1"],
        &["fabric"],
        &[],
    ));
    assert!(!version_compatible(&wrong_mc, &inst));
}

/// Map a mock version document onto the contract `ModrinthVersion`.
fn contract_version(json: &serde_json::Value) -> yuhina_api::ModrinthVersion {
    yuhina_api::ModrinthVersion {
        version_id: json["id"].as_str().unwrap_or("").to_string(),
        name: json["name"].as_str().unwrap_or("").to_string(),
        version_number: json["version_number"].as_str().unwrap_or("").to_string(),
        game_versions: json["game_versions"]
            .as_array()
            .unwrap()
            .iter()
            .filter_map(|x| x.as_str().map(String::from))
            .collect(),
        loaders: json["loaders"]
            .as_array()
            .unwrap()
            .iter()
            .filter_map(|x| x.as_str().map(String::from))
            .collect(),
        files: Vec::new(),
        dependencies: Vec::new(),
        published: json["date_published"].as_str().unwrap_or("").to_string(),
    }
}
