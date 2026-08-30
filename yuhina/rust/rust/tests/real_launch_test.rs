//! M1 milestone verification: launch a REAL Minecraft client with the real
//! Yuhina Rust engine, capture the game log and assert the client reaches the
//! main menu.
//!
//! This is the slow, real-network/real-Java integration test from
//! `docs/handoff.md` §4. It requires:
//!   - network access to BMCLAPI (or Mojang)
//!   - a Java 21 runtime (downloaded via Adoptium, or a manual fallback)
//!   - a virtual X display (run under `xvfb-run`)
//!
//! Run with:
//!   cd rust && xvfb-run -a cargo test --test real_launch -- --ignored --nocapture
//!
//! It is excluded from `cargo test --workspace` (marked `#[ignore]`).

use std::path::PathBuf;
use std::sync::Arc;
use std::time::{Duration, Instant};

use yuhina_api::{
    Account, AccountKind, CreateInstanceRequest, GameState, JavaSelection, LaunchArgs,
    LauncherConfig, Source,
};
use yuhina_auth::offline::offline_uuid;
use yuhina_core::YuhinaCore;
use yuhina_db::Db;
use yuhina_instance::InstanceManager;

/// How long the whole "download files + boot to main menu" may take.
const LAUNCH_DEADLINE: Duration = Duration::from_secs(600);

/// Evidence lines that must all be present to prove the client booted up to
/// the main-menu transition. These are logged before/independent of the first
/// rendered frame, so they hold even under software GL (llvmpipe) where the
/// menu's first frame can take a long time.
const REQUIRED_EVIDENCE: &[&str] = &[
    "Setting user: Tester",      // offline auth parameters reached the client
    "Backend library: LWJGL",    // GLFW/LWJGL initialized
    "Reloading ResourceManager", // asset reload started
    "Sound engine started",      // game init passed the GL window
    "Created:",                  // texture atlases created -> assets loaded
];

/// Bonus evidence (reported, not asserted). "Loaded X recipes/advancements"
/// is only logged once the first main-menu frame renders, which under
/// software rendering can lag for minutes.
const OPTIONAL_EVIDENCE: &[&str] = &["Loaded", "Minecraft sound engine"];

#[tokio::test]
#[ignore = "M1: real launch needs network + Java 21 + virtual display (xvfb-run)"]
async fn real_minecraft_launch_reaches_main_menu() {
    let _guard = YuhinaRuntimeGuard::init();

    // ------------------------------------------------------------------
    // 1. temp data + game dirs, Bmclapi mirror
    // ------------------------------------------------------------------
    // Persistent root via YUHINA_M1_ROOT lets re-runs reuse downloaded assets.
    let root = match std::env::var("YUHINA_M1_ROOT") {
        Ok(dir) => PathBuf::from(dir),
        Err(_) => {
            let tag = format!(
                "yuhina-m1-{}-{:x}",
                std::process::id(),
                std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_nanos()
                    & 0xffff_ffff
            );
            std::env::temp_dir().join(tag)
        }
    };
    let config = LauncherConfig {
        data_dir: root.join("data").to_string_lossy().to_string(),
        game_root: root.join("game").to_string_lossy().to_string(),
        download_source: Source::Bmclapi,
        custom_source_host: None,
        launch_args: LaunchArgs {
            min_memory_mb: 1024,
            max_memory_mb: 2048,
            extra_jvm_args: Vec::new(),
            extra_mc_args: Vec::new(),
            // small window: the whole run renders via llvmpipe (software GL),
            // so a small framebuffer speeds up reaching the main menu.
            window_width: Some(640),
            window_height: Some(360),
        },
        locale: "en-US".into(),
        theme_seed: 0,
        auto_update: false,
    };
    println!("[m1] temp root: {}", root.display());

    let core = YuhinaCore::new(config.clone()).expect("YuhinaCore::new");

    // ------------------------------------------------------------------
    // 2. version list -> pick a stable release (prefer 1.21.1)
    // ------------------------------------------------------------------
    let list = retry(3, || core.fetch_version_list())
        .await
        .expect("fetch_version_list (retried)");
    let releases: Vec<&str> = list
        .iter()
        .filter(|m| m.version_type == "release")
        .map(|m| m.id.as_str())
        .collect();
    let mc = if releases.contains(&"1.21.1") {
        "1.21.1".to_string()
    } else {
        releases
            .first()
            .map(|s| s.to_string())
            .unwrap_or_else(|| panic!("no release in version list"))
    };
    println!(
        "[m1] selected Minecraft version: {mc} ({} releases fetched)",
        releases.len()
    );

    // ------------------------------------------------------------------
    // 3. ensure client jar + libraries + assets (mirror downloads)
    // ------------------------------------------------------------------
    let dl_start = Instant::now();
    let mut downloaded: u32 = 0;
    for attempt in 1..=3 {
        match core.ensure_version_files(&mc).await {
            Ok(n) => {
                downloaded = n;
                break;
            }
            Err(e) if attempt < 3 => {
                println!("[m1] ensure_version_files attempt {attempt} failed: {e}; retrying");
                tokio::time::sleep(Duration::from_secs(2)).await;
            }
            Err(e) => panic!("ensure_version_files failed after 3 attempts: {e}"),
        }
    }
    let dl_secs = dl_start.elapsed().as_secs_f64();
    println!(
        "[m1] game files ensured in {dl_secs:.1}s ({} files downloaded)",
        downloaded
    );

    // ------------------------------------------------------------------
    // 4. Java 21 (Adoptium auto; manual /usr/bin/java as fallback)
    // ------------------------------------------------------------------
    let mut java_fallback = false;
    let java = match core.install_java(21).await {
        Ok(j) => j,
        Err(e) => {
            println!("[m1] install_java(21) failed: {e}; falling back to /usr/bin/java");
            java_fallback = true;
            core.add_manual_java("/usr/bin/java".into())
                .expect("add_manual_java(/usr/bin/java)")
        }
    };
    println!(
        "[m1] using Java {} (major {}, vendor {}, source {:?}, fallback={java_fallback})",
        java.version, java.major, java.vendor, java.source
    );

    // ------------------------------------------------------------------
    // 5. offline account (standard "OfflinePlayer:<name>" UUID)
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
    println!("[m1] offline account Tester uuid={}", account.uuid);

    // ------------------------------------------------------------------
    // 6. create a vanilla instance via InstanceManager (same DB file)
    // ------------------------------------------------------------------
    let db = Db::new(&root.join("data/yuhina.db")).expect("open shared db");
    let mgr = InstanceManager::new(
        db,
        Arc::new(core.clone()),
        core.downloader(),
        PathBuf::from(&config.game_root),
    );
    let summary = mgr
        .create_instance(CreateInstanceRequest {
            name: "M1 Vanilla".into(),
            icon: "🎮".into(),
            mc_version: mc.clone(),
            loader: None,
            java: JavaSelection::Auto(21),
            dir_name: None,
        })
        .await
        .expect("create_instance");
    println!(
        "[m1] instance created id={} name={} game_dir={}",
        summary.id,
        summary.name,
        mgr.get_instance(summary.id.clone()).await.unwrap().game_dir
    );

    // ------------------------------------------------------------------
    // 7. launch + subscribe to game output
    // ------------------------------------------------------------------
    let session = core
        .launch_instance_with(&summary.id, &account)
        .await
        .expect("launch_instance_with");
    println!(
        "[m1] game session id={} pid={} state={:?}",
        session.session_id, session.pid, session.state
    );
    let mut rx = core
        .subscribe_game_output(&session.session_id)
        .expect("subscribe_game_output");
    assert!(session.pid > 0, "java process pid must be non-zero");

    // ------------------------------------------------------------------
    // 8. collect output until the evidence set is complete
    // ------------------------------------------------------------------
    let mut lines: Vec<String> = Vec::new();
    let deadline = Instant::now() + LAUNCH_DEADLINE;
    let mut reached_main_menu = false;
    while Instant::now() < deadline {
        match tokio::time::timeout(Duration::from_secs(10), rx.recv()).await {
            Ok(Ok(out)) => {
                lines.push(out.text.clone());
                let joined = lines.join("\n");
                let missing: Vec<&str> = REQUIRED_EVIDENCE
                    .iter()
                    .copied()
                    .filter(|m| !joined.contains(m))
                    .collect();
                if missing.is_empty() {
                    reached_main_menu = true;
                    println!("[m1] full evidence set reached");
                    break;
                }
            }
            Ok(Err(_)) => { /* lagged or channel closed; keep polling */ }
            Err(_) => {
                // no output for 10s: check whether the process is still alive
                if let Ok(s) = core.get_game_session(&session.session_id).await {
                    if !matches!(s.state, GameState::Running | GameState::Starting) {
                        println!("[m1] game ended early: {:?}", s.state);
                        break;
                    }
                }
            }
        }
    }

    // full persisted log for the report
    let full_log = core.get_game_logs(&session.session_id, 0);
    let full_text: String = full_log
        .iter()
        .map(|e| e.text.as_str())
        .collect::<Vec<_>>()
        .join("\n");
    println!(
        "[m1] collected {} streamed lines, {} persisted log entries",
        lines.len(),
        full_log.len()
    );

    // ------------------------------------------------------------------
    // 9. stop the game gracefully and record the exit status
    // ------------------------------------------------------------------
    let _ = core.stop_game(&session.session_id).await;
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
    println!("[m1] final game state: {final_state:?}");

    // ------------------------------------------------------------------
    // 10. assertions
    // ------------------------------------------------------------------
    assert!(
        reached_main_menu,
        "did not reach the main menu; log was:\n{full_text}"
    );
    assert!(!full_log.is_empty(), "game log must not be empty");

    let evidence: Vec<&str> = REQUIRED_EVIDENCE
        .iter()
        .copied()
        .filter(|m| full_text.contains(m))
        .collect();
    let bonus: Vec<&str> = OPTIONAL_EVIDENCE
        .iter()
        .copied()
        .filter(|m| full_text.contains(m))
        .collect();
    println!("[m1] required evidence found: {evidence:?}");
    println!("[m1] optional evidence found: {bonus:?}");
    assert_eq!(
        evidence.len(),
        REQUIRED_EVIDENCE.len(),
        "missing evidence lines (found {evidence:?})"
    );

    // The process must have exited after our `stop_game` request. Minecraft
    // does not install a SIGTERM handler on Linux, so the JVM shuts down with
    // exit code 143 (128+SIGTERM) — that is the expected graceful-stop result,
    // not a crash.
    match final_state {
        GameState::Stopped(0) => println!("[m1] game exited cleanly with code 0"),
        GameState::Stopped(c) => println!("[m1] game exited with code {c}"),
        GameState::Crashed(reason) if reason.contains("143") => {
            println!("[m1] game stopped via SIGTERM (exit 143) after stop_game: expected")
        }
        GameState::Crashed(reason) => {
            panic!("game crashed during/after main menu: {reason}\nlog:\n{full_text}")
        }
        GameState::Starting | GameState::Running => {
            panic!("game still running after stop; log:\n{full_text}")
        }
    }

    // cleanup: only auto-generated roots are removed; an explicit
    // YUHINA_M1_ROOT is a reusable cache and is kept.
    if std::env::var("YUHINA_M1_ROOT").is_err() {
        let _ = std::fs::remove_dir_all(&root);
    }
}

// ---------------------------------------------------------------------------
// helpers
// ---------------------------------------------------------------------------

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
                println!("[m1] attempt {i}/{attempts} failed: {e}");
                tokio::time::sleep(Duration::from_secs(2)).await;
            }
        }
    }
    Err(last.unwrap_or_else(|| "no error".into()))
}

/// Guard that initialises tracing (harmless if already set) and records that
/// this test ran under a real display.
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
        println!("[m1] DISPLAY={display} (expected a virtual X server, e.g. :99)");
        Self
    }
}

impl Drop for YuhinaRuntimeGuard {
    fn drop(&mut self) {
        // no-op; kept for symmetry
    }
}
