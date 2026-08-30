//! Integration tests exercising `yuhina-core` through its public API.
//! All tests run offline against real Mojang fixtures.

use std::path::{Path, PathBuf};

use yuhina_api::{Account, AccountKind, LaunchArgs, LauncherConfig};
use yuhina_core::{
    build_classpath_for, build_launch_command, launch::LaunchInput, libraries::resolve_libraries,
    CorePaths, GameManager, GameState, Platform, VersionManifest,
};

fn fixture(name: &str) -> serde_json::Value {
    let json = match name {
        "1.20.4.json" => include_str!("fixtures/1.20.4.json"),
        other => panic!("unknown fixture {other}"),
    };
    serde_json::from_str(json).unwrap()
}

fn test_root() -> PathBuf {
    let dir = tempfile::tempdir().expect("tempdir");
    let path = dir.path().to_path_buf();
    std::mem::forget(dir);
    path
}

fn core_config(root: &Path) -> LauncherConfig {
    LauncherConfig {
        data_dir: root.join("data").to_string_lossy().to_string(),
        game_root: root.join("game").to_string_lossy().to_string(),
        download_source: yuhina_api::Source::Official,
        custom_source_host: None,
        launch_args: LaunchArgs::default(),
        locale: "zh-CN".into(),
        theme_seed: 0,
        auto_update: false,
    }
}

/// Golden launch command built through the public API surface.
#[test]
fn integration_launch_command_golden() {
    let manifest = VersionManifest::parse(&fixture("1.20.4.json")).unwrap();
    let root = test_root();
    let config = core_config(&root);
    let paths = CorePaths::from_config(&config);
    let platform = Platform::detect();
    let resolved = resolve_libraries(
        &manifest.libraries,
        &platform,
        &yuhina_core::libraries::Features {
            has_custom_resolution: None,
            is_demo_user: None,
        },
    );
    assert!(resolved.iter().any(|l| l.is_native), "natives resolved");

    let game_dir = root.join("instances/vanilla");
    let classpath = build_classpath_for(&resolved, &paths, &paths.client_jar("1.20.4"));
    let account = Account {
        id: "acc".into(),
        kind: AccountKind::Offline,
        username: "Steve".into(),
        uuid: "00000000-0000-0000-0000-000000000000".into(),
        yggdrasil_server: None,
        skin_url: None,
        is_active: true,
        expires_at: None,
    };
    let input = LaunchInput {
        java_bin: std::path::Path::new("/opt/jdk-21/bin/java"),
        game_dir: &game_dir,
        paths: &paths,
        assets_index: manifest.asset_index.id.clone(),
        natives_dir: &paths.natives_dir("sess"),
        classpath,
        version_name: "1.20.4".into(),
        version_type: manifest.version_type.clone(),
        main_class: manifest.main_class.clone(),
        launch_args: &LaunchArgs::default(),
        account: &account,
        manifest: &manifest,
        launcher_name: "yuhina".into(),
        launcher_version: "0.1.0".into(),
        platform,
    };
    let argv = build_launch_command(&input).full_argv();
    assert_eq!(argv[0], "/opt/jdk-21/bin/java");
    assert!(argv.iter().any(|a| a == "--username"));
    assert!(argv.iter().any(|a| a == "Steve"));
    assert!(argv.iter().any(|a| a == "--gameDir"));
    assert!(argv.iter().any(|a| a == "net.minecraft.client.main.Main"));
}

/// A fake game subprocess: stream lines, reach Stopped(0), persist log.
#[tokio::test]
async fn integration_fake_game_process() {
    let dir = tempfile::tempdir().unwrap();
    let log_path = dir.path().join("logs/sess1/game.log");
    let mgr = GameManager::new();
    let cmd = yuhina_core::LaunchCommand {
        java_bin: if cfg!(windows) {
            "cmd".into()
        } else {
            "sh".into()
        },
        args: if cfg!(windows) {
            vec!["/C".into(), "echo hello & exit /b 0".into()]
        } else {
            vec!["-c".into(), "echo hello-from-game; exit 0".into()]
        },
        cwd: dir.path().to_path_buf(),
    };
    let session = mgr.spawn(cmd, "inst", &log_path, dir.path()).await.unwrap();
    assert!(session.pid > 0);
    let marker = if cfg!(windows) {
        "hello"
    } else {
        "hello-from-game"
    };
    let mut rx = mgr.subscribe(&session.session_id).expect("subscribe");
    let mut got = false;
    loop {
        match tokio::time::timeout(std::time::Duration::from_secs(5), rx.recv()).await {
            Ok(Ok(o)) if o.text.contains(marker) => got = true,
            _ => {}
        }
        let s = mgr.get(&session.session_id).await.unwrap();
        if matches!(s.state, GameState::Stopped(0)) {
            break;
        }
    }
    assert!(got, "saw game output");
    let entries = yuhina_core::process::read_game_log(&log_path, 0).unwrap();
    assert!(entries.iter().any(|e| e.text.contains(marker)));
}

/// Core construction wires paths + db + events.
#[test]
fn integration_core_construction() {
    let root = test_root();
    let core = yuhina_core::YuhinaCore::new(core_config(&root)).unwrap();
    let mut rx = core.subscribe_events();
    assert!(core.list_java_runtimes().is_empty());
    // set_config triggers ConfigChanged
    core.set_config(core_config(&root));
    match rx.try_recv() {
        Ok(yuhina_api::AppEvent::ConfigChanged) => {}
        other => panic!("expected ConfigChanged, got {other:?}"),
    }
}
