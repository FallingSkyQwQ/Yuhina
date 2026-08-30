//! Download manager integration tests (concurrency, resume, retry, checksum,
//! pause/resume/cancel, progress throttling, priority).

mod common;

use std::time::Duration;

use common::{MockConfig, MockServer, fast_config, wait_for};
use yuhina_api::{DownloadState, Source};
use yuhina_download::{DownloadManager, FileReq, ManagerConfig, Priority, TaskKind, rewrite_url};

fn sha1_hex(data: &[u8]) -> String {
    use sha1::Digest;
    let mut h = sha1::Sha1::new();
    h.update(data);
    let out = h.finalize();
    out.iter().map(|b| format!("{b:02x}")).collect()
}

fn temp_db() -> yuhina_download::Store {
    yuhina_download::Store::in_memory().unwrap()
}

fn req(url: String, dest: std::path::PathBuf, sha1: Option<String>) -> FileReq {
    FileReq {
        id: None,
        title: "test".into(),
        url,
        dest,
        sha1,
        priority: Priority::Library,
        kind: TaskKind::Library,
        instance_id: None,
    }
}

/// Mirrors a mock-server URL through the Bmclapi source (host is unknown →
/// unchanged), just to exercise the mirror path in the manager pipeline.
fn mirror_url(base: &str, path: &str) -> String {
    rewrite_url(&format!("{base}{path}"), &Source::Bmclapi)
}

#[tokio::test]
async fn downloads_and_verifies_sha1() {
    let data = b"hello yuhina download".to_vec();
    let server = MockServer::start(MockConfig {
        data: data.clone(),
        ..Default::default()
    });
    let db = temp_db();
    let mgr = DownloadManager::start(db, fast_config(2));
    let dest = std::env::temp_dir().join(format!("yuhina-test-{}.bin", uuid()));

    let id = mgr
        .enqueue(req(mirror_url(&server.base_url, "/file.bin"), dest.clone(), Some(sha1_hex(&data))))
        .unwrap();

    assert!(
        wait_for(
            || mgr.list_tasks().unwrap().iter().any(|t| t.id == id && t.state == DownloadState::Done),
            Duration::from_secs(10)
        )
        .await
    );
    assert_eq!(std::fs::read(&dest).unwrap(), data);
    assert!(!dest.with_extension("bin.part").exists());
    assert_eq!(server.hit_count(), 1);
    mgr.shutdown();
    let _ = std::fs::remove_file(&dest);
}

#[tokio::test]
async fn resumes_from_partial_file_with_range() {
    let data: Vec<u8> = (0..4096u32).map(|i| (i % 251) as u8).collect();
    let server = MockServer::start(MockConfig {
        data: data.clone(),
        ..Default::default()
    });
    let db = temp_db();
    let mgr = DownloadManager::start(db, fast_config(2));
    let dest = std::env::temp_dir().join(format!("yuhina-resume-{}.jar", uuid()));
    let part = yuhina_download::task::part_path(&dest);

    // Simulate an interrupted download: pre-write the first half as `.part`.
    std::fs::create_dir_all(dest.parent().unwrap()).unwrap();
    std::fs::write(&part, &data[..2048]).unwrap();

    let id = mgr
        .enqueue(req(mirror_url(&server.base_url, "/resume.jar"), dest.clone(), None))
        .unwrap();

    assert!(
        wait_for(
            || mgr.list_tasks().unwrap().iter().any(|t| t.id == id && t.state == DownloadState::Done),
            Duration::from_secs(10)
        )
        .await
    );
    assert_eq!(std::fs::read(&dest).unwrap(), data);
    assert!(!part.exists());

    // The server must have received a Range request resuming from byte 2048.
    let ranged = server.requests().iter().any(|r| {
        r.range.as_deref() == Some("bytes=2048-")
    });
    assert!(ranged, "expected a Range request resuming from 2048, got {:?}", server.requests());
    mgr.shutdown();
    let _ = std::fs::remove_file(&dest);
}

#[tokio::test]
async fn retries_then_succeeds_on_server_error() {
    let data = b"retry me".to_vec();
    let server = MockServer::start(MockConfig {
        data: data.clone(),
        fail_count: 2,
        ..Default::default()
    });
    let db = temp_db();
    let mgr = DownloadManager::start(db, fast_config(2));
    let dest = std::env::temp_dir().join(format!("yuhina-retry-{}.bin", uuid()));

    let id = mgr.enqueue(req(server.url("/retry.bin"), dest.clone(), None)).unwrap();
    assert!(
        wait_for(
            || mgr.list_tasks().unwrap().iter().any(|t| t.id == id && t.state == DownloadState::Done),
            Duration::from_secs(10)
        )
        .await
    );
    assert_eq!(std::fs::read(&dest).unwrap(), data);
    // 2 failures + 1 success.
    assert_eq!(server.hit_count(), 3);
    mgr.shutdown();
    let _ = std::fs::remove_file(&dest);
}

#[tokio::test]
async fn retries_after_mid_stream_drop_and_resumes() {
    let data: Vec<u8> = (0..4096u32).map(|i| (i % 251) as u8).collect();
    let server = common::DropServer::start(data.clone(), 1024);
    let db = temp_db();
    let mgr = DownloadManager::start(db, fast_config(2));
    let dest = std::env::temp_dir().join(format!("yuhina-drop-{}.bin", uuid()));

    let id = mgr.enqueue(req(server.url("/drop.bin"), dest.clone(), None)).unwrap();
    assert!(
        wait_for(
            || mgr.list_tasks().unwrap().iter().any(|t| t.id == id && t.state == DownloadState::Done),
            Duration::from_secs(10)
        )
        .await
    );
    assert_eq!(std::fs::read(&dest).unwrap(), data);
    // First connection was dropped; the retry resumed from the `.part` offset.
    assert!(server.hit_count() >= 2);
    mgr.shutdown();
    let _ = std::fs::remove_file(&dest);
}

#[tokio::test]
async fn checksum_mismatch_fails_and_cleans_part() {
    let data = b"some content".to_vec();
    let server = MockServer::start(MockConfig {
        data: data.clone(),
        ..Default::default()
    });
    let db = temp_db();
    let config = ManagerConfig {
        retry_max: 1,
        ..fast_config(2)
    };
    let mgr = DownloadManager::start(db, config);
    let dest = std::env::temp_dir().join(format!("yuhina-bad-{}.bin", uuid()));

    let id = mgr
        .enqueue(req(server.url("/bad.bin"), dest.clone(), Some("deadbeef".into())))
        .unwrap();

    assert!(
        wait_for(
            || mgr
                .list_tasks()
                .unwrap()
                .iter()
                .any(|t| t.id == id && t.state == DownloadState::Failed),
            Duration::from_secs(10)
        )
        .await
    );
    let task = mgr
        .list_tasks()
        .unwrap()
        .into_iter()
        .find(|t| t.id == id)
        .unwrap();
    assert!(task.error.as_deref().unwrap_or("").contains("sha1"));
    assert!(!dest.exists());
    assert!(!dest.with_extension("bin.part").exists());
    mgr.shutdown();
}

#[tokio::test]
async fn respects_concurrency_limit() {
    let data = vec![0u8; 64 * 1024];
    let server = MockServer::start(MockConfig {
        data,
        delay: Some(Duration::from_millis(15)),
        chunk: 16 * 1024,
        ..Default::default()
    });
    let db = temp_db();
    let mgr = DownloadManager::start(db, fast_config(3));
    let mut ids = Vec::new();
    for i in 0..9 {
        let dest = std::env::temp_dir().join(format!("yuhina-conc-{}-{}.bin", uuid(), i));
        let id = mgr.enqueue(req(server.url("/c.bin"), dest, None)).unwrap();
        ids.push(id);
    }

    assert!(
        wait_for(
            || mgr
                .list_tasks()
                .unwrap()
                .iter()
                .filter(|t| ids.contains(&t.id) && t.state == DownloadState::Done)
                .count()
                == ids.len(),
            Duration::from_secs(20)
        )
        .await
    );
    let max = server.max_active();
    assert!(max <= 3, "concurrency exceeded: {max}");
    assert!(max >= 2, "no concurrency observed: {max}");
    mgr.shutdown();
}

#[tokio::test]
async fn pause_resume_cancel() {
    let data = vec![7u8; 64 * 1024];
    let server = MockServer::start(MockConfig {
        data: data.clone(),
        delay: Some(Duration::from_millis(8)),
        chunk: 4 * 1024,
        ..Default::default()
    });
    let db = temp_db();
    let mgr = DownloadManager::start(db, fast_config(2));

    // Task 1: pause mid-way, then resume.
    let dest1 = std::env::temp_dir().join(format!("yuhina-pause-{}.bin", uuid()));
    let id1 = mgr.enqueue(req(server.url("/p.bin"), dest1.clone(), None)).unwrap();
    // Task 2: cancel after starting.
    let dest2 = std::env::temp_dir().join(format!("yuhina-cancel-{}.bin", uuid()));
    let id2 = mgr.enqueue(req(server.url("/p.bin"), dest2.clone(), None)).unwrap();

    // Wait for both to be Running, then pause #1 and cancel #2.
    assert!(
        wait_for(
            || mgr
                .list_tasks()
                .unwrap()
                .iter()
                .filter(|t| matches!(t.state, DownloadState::Running))
                .count()
                >= 2,
            Duration::from_secs(10)
        )
        .await
    );
    mgr.pause_task(&id1).await.unwrap();
    mgr.cancel_task(&id2).await.unwrap();

    assert!(
        wait_for(
            || mgr
                .list_tasks()
                .unwrap()
                .iter()
                .any(|t| t.id == id2 && t.state == DownloadState::Canceled),
            Duration::from_secs(10)
        )
        .await
    );
    assert!(!dest2.exists());
    assert!(!yuhina_download::task::part_path(&dest2).exists());

    assert!(
        wait_for(
            || mgr
                .list_tasks()
                .unwrap()
                .iter()
                .any(|t| t.id == id1 && t.state == DownloadState::Paused),
            Duration::from_secs(10)
        )
        .await
    );
    // Partial data must be kept for resume.
    let part1 = yuhina_download::task::part_path(&dest1);
    assert!(part1.exists());

    mgr.resume_task(&id1).await.unwrap();
    assert!(
        wait_for(
            || mgr
                .list_tasks()
                .unwrap()
                .iter()
                .any(|t| t.id == id1 && t.state == DownloadState::Done),
            Duration::from_secs(10)
        )
        .await
    );
    assert_eq!(std::fs::read(&dest1).unwrap(), data);
    assert!(!part1.exists());
    mgr.shutdown();
    let _ = std::fs::remove_file(&dest1);
}

#[tokio::test]
async fn progress_events_are_throttled() {
    let data = vec![1u8; 32 * 1024];
    let server = MockServer::start(MockConfig {
        data: data.clone(),
        delay: Some(Duration::from_millis(4)),
        chunk: 1 * 1024,
        ..Default::default()
    });
    let db = temp_db();
    let config = ManagerConfig {
        progress_interval_ms: 40,
        persist_interval_ms: 80,
        ..fast_config(1)
    };
    let mgr = DownloadManager::start(db, config);
    let mut rx = mgr.subscribe();
    let dest = std::env::temp_dir().join(format!("yuhina-throttle-{}.bin", uuid()));

    let id = mgr.enqueue(req(server.url("/t.bin"), dest.clone(), None)).unwrap();

    let mut running = 0usize;
    let mut saw_done = false;
    let deadline = std::time::Instant::now() + Duration::from_secs(10);
    loop {
        let ev = tokio::time::timeout(Duration::from_millis(200), rx.recv())
            .await
            .expect("progress event within timeout")
            .expect("channel open");
        if ev.task_id != id {
            continue;
        }
        match ev.state {
            DownloadState::Running => running += 1,
            DownloadState::Done => {
                saw_done = true;
                break;
            }
            _ => {}
        }
        if std::time::Instant::now() > deadline {
            break;
        }
    }
    assert!(saw_done, "never saw Done event");
    // 32 chunks × 4ms ≈ 130ms. With 40ms throttle expect ≤ ~6 running events.
    assert!(running >= 1, "expected at least one progress event");
    assert!(
        running <= 8,
        "progress events not throttled: got {running} in ~130ms download"
    );
    mgr.shutdown();
    let _ = std::fs::remove_file(&dest);
}

#[tokio::test]
async fn high_priority_starts_first() {
    let data = vec![2u8; 16 * 1024];
    let server = MockServer::start(MockConfig {
        data: data.clone(),
        delay: Some(Duration::from_millis(10)),
        chunk: 16 * 1024,
        ..Default::default()
    });
    let db = temp_db();
    let mgr = DownloadManager::start(db, fast_config(1));

    let dest_lo = std::env::temp_dir().join(format!("yuhina-lo-{}.bin", uuid()));
    let dest_hi = std::env::temp_dir().join(format!("yuhina-hi-{}.bin", uuid()));

    let id_lo = mgr
        .enqueue(FileReq {
            priority: Priority::Asset,
            ..req(server.url("/p.bin"), dest_lo, None)
        })
        .unwrap();
    let id_hi = mgr
        .enqueue(FileReq {
            priority: Priority::Launch,
            ..req(server.url("/p.bin"), dest_hi, None)
        })
        .unwrap();

    // Concurrency 1: the high-priority job must finish first.
    assert!(
        wait_for(
            || mgr
                .list_tasks()
                .unwrap()
                .iter()
                .any(|t| t.id == id_hi && t.state == DownloadState::Done),
            Duration::from_secs(10)
        )
        .await
    );
    let lo_state = mgr
        .list_tasks()
        .unwrap()
        .into_iter()
        .find(|t| t.id == id_lo)
        .unwrap()
        .state;
    assert_eq!(lo_state, DownloadState::Running);
    mgr.shutdown();
}

fn uuid() -> String {
    uuid::Uuid::new_v4().to_string()
}
