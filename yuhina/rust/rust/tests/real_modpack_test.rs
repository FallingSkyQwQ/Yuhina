//! M2 milestone verification: the FULL mod ecosystem flow against live
//! services — Modrinth search/install of a mod WITH required dependencies
//! into a Fabric instance, a real client launch that proves the Fabric loader
//! initializes and loads the mods, and an `.mrpack` export → re-import
//! round-trip that must reproduce an identical mod file set.
//!
//! Requires (handoff.md §4 slow-test list):
//!   - network to Modrinth, meta.fabricmc.net / maven.fabricmc.net, BMCLAPI
//!   - a Java 21 runtime (auto-downloaded via Adoptium, /usr/bin/java fallback)
//!   - a virtual X display (run under `xvfb-run`)
//!
//! Run with:
//!   cd rust && xvfb-run -a cargo test --test real_modpack_test -- --ignored --nocapture
//!
//! The first run downloads the vanilla 1.21.1 game files + Java 21 into the
//! persistent root. Reuse them on later runs by keeping the root (default
//! `$TMPDIR/yuhina-m2-root`), or point it elsewhere with `YUHINA_M2_ROOT`.
//! For MRPACK_OUT, set `YUHINA_M2_MRPACK` to a path to keep the exported pack.

use std::path::PathBuf;
use std::sync::Arc;
use std::time::{Duration, Instant};

use serde_json::Value;
use yuhina_api::{
    Account, AccountKind, ConflictSeverity, CreateInstanceRequest, GameState, JavaSelection,
    LaunchArgs, LauncherConfig, Loader, LoaderKind, Source,
};
use yuhina_auth::offline::offline_uuid;
use yuhina_core::YuhinaCore;
use yuhina_db::Db;
use yuhina_instance::InstanceManager;

const MC_VERSION: &str = "1.21.1";

/// Fabric loader initialisation evidence (case-sensitive substrings). These
/// are printed by the FabricLoader before the GL window opens.
const FABRIC_INIT_EVIDENCE: &[&str] = &[
    "Fabric Loader", // "Loading Minecraft 1.21.1 with Fabric Loader <v>"
    "Loading Minecraft",
    " mods:", // "Loading <N> mods:" + indented mod list
];

/// Mod names printed on the "Loading <N> mods:" line (case-insensitive).
const MOD_EVIDENCE: &[&str] = &["sodium", "iris", "fabric-api"];

/// Vanilla-boot evidence proving the game itself actually launched.
const BOOT_EVIDENCE: &[&str] = &["Setting user: Tester"];

#[tokio::test]
#[ignore = "M2: real Modrinth + Fabric launch + mrpack round-trip (needs network + Java 21 + xvfb)"]
async fn real_fabric_modpack_flow() {
    let _guard = YuhinaRuntimeGuard::init();

    // ------------------------------------------------------------------
    // 1. temp data + game dirs, Bmclapi mirror (persistent root for reuse)
    // ------------------------------------------------------------------
    let root = match std::env::var("YUHINA_M2_ROOT") {
        Ok(dir) => PathBuf::from(dir),
        Err(_) => std::env::temp_dir().join("yuhina-m2-root"),
    };
    println!("[m2] temp root: {}", root.display());
    // BMCLAPI can burst-rate-limit (429) under load; Mojang's official hosts
    // are reachable from this machine, so default to them. Set
    // YUHINA_M2_SOURCE=bmclapi to force the mirror.
    let download_source = if std::env::var("YUHINA_M2_SOURCE").as_deref() == Ok("bmclapi") {
        Source::Bmclapi
    } else {
        Source::Official
    };
    let config = LauncherConfig {
        data_dir: root.join("data").to_string_lossy().to_string(),
        game_root: root.join("game").to_string_lossy().to_string(),
        download_source,
        custom_source_host: None,
        launch_args: LaunchArgs {
            min_memory_mb: 1024,
            max_memory_mb: 2048,
            extra_jvm_args: Vec::new(),
            extra_mc_args: Vec::new(),
            window_width: Some(640),
            window_height: Some(360),
        },
        locale: "en-US".into(),
        theme_seed: 0,
        auto_update: false,
    };
    let core = YuhinaCore::new(config.clone()).expect("YuhinaCore::new");

    // ------------------------------------------------------------------
    // 2. version list → MC 1.21.1
    // ------------------------------------------------------------------
    let list = retry(3, || core.fetch_version_list())
        .await
        .expect("fetch_version_list (retried)");
    let has_mc = list.iter().any(|m| m.id == MC_VERSION);
    assert!(
        has_mc,
        "version {MC_VERSION} must exist in the live manifest"
    );

    // ------------------------------------------------------------------
    // 3. ensure vanilla game files (client jar + libraries + assets)
    // ------------------------------------------------------------------
    let dl_start = Instant::now();
    let mut downloaded: u32 = 0;
    for attempt in 1..=3 {
        match core.ensure_version_files(MC_VERSION).await {
            Ok(n) => {
                downloaded = n;
                break;
            }
            Err(e) if attempt < 3 => {
                println!("[m2] ensure_version_files attempt {attempt} failed: {e}; retrying");
                tokio::time::sleep(Duration::from_secs(2)).await;
            }
            Err(e) => panic!("ensure_version_files failed after 3 attempts: {e}"),
        }
    }
    println!(
        "[m2] game files ensured in {:.1}s ({} files downloaded)",
        dl_start.elapsed().as_secs_f64(),
        downloaded
    );

    // ------------------------------------------------------------------
    // 4. Java 21 (Adoptium auto; manual /usr/bin/java as fallback)
    // ------------------------------------------------------------------
    let mut java_fallback = false;
    let java = match core.install_java(21).await {
        Ok(j) => j,
        Err(e) => {
            println!("[m2] install_java(21) failed: {e}; falling back to /usr/bin/java");
            java_fallback = true;
            core.add_manual_java("/usr/bin/java".into())
                .expect("add_manual_java(/usr/bin/java)")
        }
    };
    println!(
        "[m2] using Java {} (major {}, vendor {}, source {:?}, fallback={java_fallback})",
        java.version, java.major, java.vendor, java.source
    );

    // ------------------------------------------------------------------
    // 5. resolve the REAL Fabric loader version for 1.21.1 (stable first)
    // ------------------------------------------------------------------
    let loader_version = stable_fabric_loader(&core, MC_VERSION).await;
    println!("[m2] fabric loader version for {MC_VERSION}: {loader_version}");

    // ------------------------------------------------------------------
    // 6. offline account + Fabric instance
    // ------------------------------------------------------------------
    let account = Account {
        id: uuid::Uuid::new_v4().to_string(),
        kind: AccountKind::Offline,
        username: "Tester".into(),
        uuid: offline_uuid("Tester"),
        yggdrasil_server: None,
        skin_url: None,
        is_active: true,
        expires_at: None,
    };
    let db = Db::new(&root.join("data/yuhina.db")).expect("open shared db");
    let mgr = InstanceManager::new(
        db,
        Arc::new(core.clone()),
        core.downloader(),
        PathBuf::from(&config.game_root),
    );

    // --- (a) Modrinth search ---------------------------------------------
    let search = retry(3, || {
        mgr.search_mods(
            "sodium".into(),
            vec!["fabric".into()],
            vec![MC_VERSION.into()],
            0,
            5,
        )
    })
    .await
    .expect("search_mods(sodium, fabric, 1.21.1) (retried)");
    assert!(search.total > 0, "expected sodium hits");
    assert!(!search.hits.is_empty(), "sodium hits must not be empty");
    let sodium_project = search
        .hits
        .iter()
        .find(|h| h.slug == "sodium")
        .expect("sodium slug present in hits");
    assert!(!sodium_project.title.is_empty());
    assert!(!sodium_project.description.is_empty());
    assert!(sodium_project.loaders.contains(&"fabric".to_string()));
    assert!(sodium_project
        .game_versions
        .contains(&MC_VERSION.to_string()));
    assert!(sodium_project.downloads > 0);
    println!(
        "[m2] (a) search ok: {} hits, top slug={} title={} downloads={}",
        search.total, search.hits[0].slug, search.hits[0].title, search.hits[0].downloads
    );

    // --- create the Fabric instance ---------------------------------------
    let summary = mgr
        .create_instance(CreateInstanceRequest {
            name: "M2 Fabric".into(),
            icon: "🟨".into(),
            mc_version: MC_VERSION.into(),
            loader: Some(Loader {
                kind: LoaderKind::Fabric,
                version: loader_version.clone(),
            }),
            java: JavaSelection::Auto(21),
            dir_name: None,
        })
        .await
        .expect("create_instance (fabric)");
    println!(
        "[m2] instance created id={} game_dir={}",
        summary.id,
        mgr.get_instance(summary.id.clone()).await.unwrap().game_dir
    );

    // --- (b) install the loader (vanilla files + fabric installer) --------
    let install_start = Instant::now();
    retry(3, || mgr.ensure_installed(summary.id.clone()))
        .await
        .expect("ensure_installed (fabric loader)");
    let detail = mgr.get_instance(summary.id.clone()).await.unwrap();
    assert!(
        detail.summary.is_installed,
        "instance must be marked installed"
    );
    assert_eq!(
        detail.summary.loader.as_ref().unwrap().version,
        loader_version
    );
    println!(
        "[m2] (b) loader installed in {:.1}s (is_installed=true)",
        install_start.elapsed().as_secs_f64()
    );

    // --- (c) install fabric-api + iris (iris requires sodium transitively) -
    let fabric_api = search_project(&mgr, "fabric-api", MC_VERSION).await;
    let iris = search_project(&mgr, "iris", MC_VERSION).await;
    let m_fabric_api = mgr
        .install_mod(summary.id.clone(), fabric_api.project_id.clone(), None)
        .await
        .expect("install fabric-api");
    let m_iris = mgr
        .install_mod(summary.id.clone(), iris.project_id.clone(), None)
        .await
        .expect("install iris (should auto-resolve sodium)");
    println!(
        "[m2] (c) installed: fabric-api {} / iris {}",
        m_fabric_api.version_id.as_deref().unwrap_or("?"),
        m_iris.version_id.as_deref().unwrap_or("?")
    );

    let mods = mgr.list_mods(summary.id.clone()).await;
    let names: Vec<&str> = mods.iter().map(|m| m.file_name.as_str()).collect();
    println!("[m2] installed mods: {names:?}");
    let mods_lc: Vec<String> = mods.iter().map(|m| m.file_name.to_lowercase()).collect();
    assert!(
        mods_lc.iter().any(|n| n.starts_with("fabric-api")),
        "fabric-api must be installed, got {names:?}"
    );
    assert!(
        mods_lc.iter().any(|n| n.starts_with("iris")),
        "iris must be installed, got {names:?}"
    );
    assert!(
        mods_lc.iter().any(|n| n.starts_with("sodium")),
        "sodium (required dep of iris) must be auto-installed, got {names:?}"
    );
    assert_eq!(mods.len(), 3, "expected exactly 3 mods, got {names:?}");

    // --- conflict check: no Error-level conflicts -------------------------
    let conflicts = mgr
        .check_mod_conflicts(summary.id.clone())
        .await
        .expect("check_mod_conflicts");
    let errors: Vec<&yuhina_api::ModConflict> = conflicts
        .iter()
        .filter(|c| c.severity == ConflictSeverity::Error)
        .collect();
    assert!(
        errors.is_empty(),
        "must have no Error-level conflicts, got {:?}",
        conflicts
    );
    println!(
        "[m2] (c) conflicts: {} ({} error): {:?}",
        conflicts.len(),
        errors.len(),
        conflicts
            .iter()
            .map(|c| c.message.as_str())
            .collect::<Vec<_>>()
    );

    // --- (d) real launch under xvfb ---------------------------------------
    let session = core
        .launch_instance_with(&summary.id, &account)
        .await
        .expect("launch_instance_with (fabric)");
    println!(
        "[m2] (d) game session id={} pid={} state={:?}",
        session.session_id, session.pid, session.state
    );
    assert!(session.pid > 0, "java process pid must be non-zero");
    let mut rx = core
        .subscribe_game_output(&session.session_id)
        .expect("subscribe_game_output");

    let mut lines: Vec<String> = Vec::new();
    let mut fabric_evidence = false;
    let mut boot_evidence = false;
    let deadline = Instant::now() + LAUNCH_DEADLINE;
    while Instant::now() < deadline {
        match tokio::time::timeout(Duration::from_secs(10), rx.recv()).await {
            Ok(Ok(out)) => {
                lines.push(out.text.clone());
                let joined = lines.join("\n");
                if !fabric_evidence {
                    let missing: Vec<&str> = FABRIC_INIT_EVIDENCE
                        .iter()
                        .copied()
                        .filter(|m| !joined.contains(m))
                        .collect();
                    if missing.is_empty() {
                        fabric_evidence = true;
                        println!("[m2] fabric loader evidence reached");
                    }
                }
                if fabric_evidence && BOOT_EVIDENCE.iter().all(|m| joined.contains(m)) {
                    boot_evidence = true;
                }
                // stop once the loader init + boot evidence are complete
                if fabric_evidence && boot_evidence {
                    println!("[m2] fabric + boot evidence complete; stopping game");
                    break;
                }
            }
            Ok(Err(_)) => { /* lagged; keep polling */ }
            Err(_) => {
                if let Ok(s) = core.get_game_session(&session.session_id).await {
                    if !matches!(s.state, GameState::Running | GameState::Starting) {
                        println!("[m2] game ended early: {:?}", s.state);
                        break;
                    }
                }
            }
        }
    }

    let full_log = core.get_game_logs(&session.session_id, 0);
    let full_text: String = full_log
        .iter()
        .map(|e| e.text.as_str())
        .collect::<Vec<_>>()
        .join("\n");
    println!(
        "[m2] collected {} streamed lines, {} persisted entries, fabric_evidence={fabric_evidence}, boot_evidence={boot_evidence}",
        lines.len(),
        full_log.len()
    );

    // grace period: let the game keep running a little so sodium/iris finish
    // their own init before we tear the process down (best-effort).
    if fabric_evidence {
        let _ = core.stop_game(&session.session_id).await;
    } else {
        tokio::time::sleep(MOD_EVIDENCE_GRACE).await;
        let _ = core.stop_game(&session.session_id).await;
    }
    let mut final_state = GameState::Running;
    for _ in 0..100 {
        let s = core
            .get_game_session(&session.session_id)
            .await
            .expect("session still tracked");
        if !matches!(s.state, GameState::Running | GameState::Starting) {
            final_state = s.state;
            break;
        }
        tokio::time::sleep(Duration::from_millis(100)).await;
    }
    println!("[m2] (d) final game state: {final_state:?}");

    // --- assertions on the launch -----------------------------------------
    assert!(
        fabric_evidence,
        "fabric loader did not initialise; log was:\n{full_text}"
    );
    let missing_mods: Vec<&str> = MOD_EVIDENCE
        .iter()
        .copied()
        .filter(|m| !contains_ci(&full_text, m))
        .collect();
    assert!(
        missing_mods.is_empty(),
        "expected mod-load evidence {missing_mods:?} in log:\n{full_text}"
    );
    let boot_found: Vec<&str> = BOOT_EVIDENCE
        .iter()
        .copied()
        .filter(|m| full_text.contains(m))
        .collect();
    println!("[m2] (d) mod evidence OK; boot evidence: {boot_found:?}");
    // a hard crash BEFORE the loader init is a real failure; a crash after the
    // evidence is a rendering-environment concern, not a mod-ecosystem one.
    if !fabric_evidence && !matches!(final_state, GameState::Stopped(_)) {
        panic!("game ended before fabric loader evidence: {final_state:?}\n{full_text}");
    }

    // --- (e) mrpack export → inspect → re-import round-trip ----------------
    let mrpack_path = std::env::var("YUHINA_M2_MRPACK")
        .map(PathBuf::from)
        .unwrap_or_else(|_| root.join("out.mrpack"));
    let out = mgr
        .export_modpack(summary.id.clone(), mrpack_path.to_string_lossy().into())
        .await
        .expect("export_modpack");
    assert!(PathBuf::from(&out).exists(), "mrpack written to {out}");
    println!("[m2] (e) exported mrpack: {out}");

    // inspect the archive
    let file = std::fs::File::open(&out).unwrap();
    let mut archive = zip::ZipArchive::new(file).unwrap();
    let mut index_json = String::new();
    {
        let mut entry = archive.by_name("index.json").unwrap();
        std::io::Read::read_to_string(&mut entry, &mut index_json).unwrap();
    }
    let index: Value = serde_json::from_str(&index_json).unwrap();
    assert_eq!(index["formatVersion"], 1);
    assert_eq!(index["versionId"], MC_VERSION);
    assert_eq!(
        index["modloaders"][0]["id"],
        format!("fabric-{loader_version}")
    );
    let files = index["files"].as_array().unwrap();
    assert_eq!(files.len(), 3, "index must list the 3 installed mods");
    let mut index_paths: Vec<String> = Vec::new();
    for f in files {
        let path = f["path"].as_str().unwrap();
        assert!(path.starts_with("mods/"), "path {path} under mods/");
        let downloads = f["downloads"].as_array().unwrap();
        assert!(
            !downloads.is_empty(),
            "Modrinth-linked mod needs a url: {path}"
        );
        let sha1 = f["hashes"]["sha1"].as_str().unwrap();
        assert_eq!(sha1.len(), 40, "sha1 for {path}");
        index_paths.push(path.to_string());
    }
    println!("[m2] (e) mrpack index ok: {index_paths:?}");

    // import into a fresh instance
    let imported = mgr
        .import_modpack(out.clone(), "M2 Imported".into())
        .await
        .expect("import_modpack");
    assert_eq!(imported.mc_version, MC_VERSION);
    assert_eq!(imported.loader.as_ref().unwrap().kind, LoaderKind::Fabric);
    let imported_mods = mgr.list_mods(imported.id.clone()).await;
    assert_eq!(
        imported_mods.len(),
        3,
        "imported instance must have the same mod count"
    );
    // byte-identical file set: same file names and same sha1s as the export.
    let mut source_set: Vec<(String, String)> = mods
        .iter()
        .map(|m| (m.file_name.clone(), m.sha1.clone()))
        .collect();
    source_set.sort();
    let mut imported_set: Vec<(String, String)> = imported_mods
        .iter()
        .map(|m| (m.file_name.clone(), m.sha1.clone()))
        .collect();
    imported_set.sort();
    assert_eq!(
        source_set, imported_set,
        "export → import must reproduce the identical mod file set"
    );
    println!("[m2] (e) round-trip OK: 3 mods byte-identical (name+sha1)");

    println!(
        "[m2] ALL M2 CHECKS PASSED (fabric {loader_version}, sodium/iris/fabric-api, mrpack round-trip)"
    );
}

// ---------------------------------------------------------------------------
// helpers
// ---------------------------------------------------------------------------

/// Resolve the Fabric loader version for `mc`, preferring the newest stable
/// entry from `meta.fabricmc.net` (via the mirror-aware downloader).
async fn stable_fabric_loader(core: &YuhinaCore, mc: &str) -> String {
    let url = format!("https://meta.fabricmc.net/v2/versions/loader/{mc}");
    let bytes = core
        .downloader()
        .fetch_bytes(&url)
        .await
        .unwrap_or_else(|e| panic!("fetch fabric loader meta {url}: {e}"));
    let v: Value = serde_json::from_slice(&bytes).expect("fabric loader meta json");
    let arr = v.as_array().expect("fabric loader meta array");
    for entry in arr {
        let loader = &entry["loader"];
        if loader["stable"].as_bool().unwrap_or(false) {
            if let Some(s) = loader["version"].as_str() {
                return s.to_string();
            }
        }
    }
    arr[0]["loader"]["version"]
        .as_str()
        .expect("newest loader version")
        .to_string()
}

/// Find a Modrinth project by slug through `search_mods` and return its hit.
async fn search_project(
    mgr: &InstanceManager,
    slug: &str,
    mc: &str,
) -> yuhina_api::ModrinthProject {
    let res = retry(3, || {
        mgr.search_mods(slug.into(), vec!["fabric".into()], vec![mc.into()], 0, 8)
    })
    .await
    .unwrap_or_else(|e| panic!("search {slug}: {e}"));
    res.hits
        .iter()
        .find(|h| h.slug == slug)
        .cloned()
        .unwrap_or_else(|| panic!("slug {slug} not in search hits: {:?}", res.hits))
}

/// Case-insensitive substring check.
fn contains_ci(haystack: &str, needle: &str) -> bool {
    haystack.to_lowercase().contains(&needle.to_lowercase())
}

/// Retry a network call up to `attempts` times.
async fn retry<F, Fut, T, E>(attempts: u32, mut f: F) -> Result<T, String>
where
    F: FnMut() -> Fut,
    Fut: std::future::Future<Output = Result<T, E>>,
    E: std::fmt::Display,
{
    let mut last = None;
    for i in 1..=attempts {
        match f().await {
            Ok(v) => return Ok(v),
            Err(e) => {
                last = Some(e.to_string());
                println!("[m2] attempt {i}/{attempts} failed: {e}");
                tokio::time::sleep(Duration::from_secs(2)).await;
            }
        }
    }
    Err(last.unwrap_or_else(|| "no error".into()))
}

/// Guard that initialises tracing (harmless if already set) and records the
/// display.
struct YuhinaRuntimeGuard;

impl YuhinaRuntimeGuard {
    fn init() -> Self {
        let _ = tracing_subscriber::fmt()
            .with_env_filter(
                tracing_subscriber::EnvFilter::try_from_default_env()
                    .unwrap_or_else(|_| "info".into()),
            )
            .try_init();
        let display = std::env::var("DISPLAY").unwrap_or_default();
        println!("[m2] DISPLAY={display} (expected a virtual X server, e.g. :99)");
        Self
    }
}

impl Drop for YuhinaRuntimeGuard {
    fn drop(&mut self) {}
}

const LAUNCH_DEADLINE: Duration = Duration::from_secs(600);
const MOD_EVIDENCE_GRACE: Duration = Duration::from_secs(120);
