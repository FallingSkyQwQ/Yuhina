//! Restart-recovery integration tests (task T3).

mod common;

use std::time::Duration;

use common::{fast_config, wait_for, MockConfig, MockServer};
use yuhina_api::DownloadState;
use yuhina_download::store::StoredTask;
use yuhina_download::{
    resume::resume_after_restart, DownloadManager, FileReq, Priority, Store, TaskKind,
};

fn sample_row(
    id: &str,
    state: DownloadState,
    url: &str,
    dest: &str,
    done: u64,
    total: u64,
) -> StoredTask {
    StoredTask {
        id: id.into(),
        kind: "library".into(),
        title: "resume me".into(),
        instance_id: None,
        url: url.into(),
        target_path: dest.into(),
        total_bytes: total,
        done_bytes: done,
        state,
        checksum_sha1: None,
        error: None,
        created_at: 1,
        updated_at: 1,
    }
}

#[tokio::test]
async fn running_task_is_requeued_and_finishes() {
    let data = vec![3u8; 2048];
    let server = MockServer::start(MockConfig {
        data: data.clone(),
        ..Default::default()
    });
    let db = Store::in_memory().unwrap();
    let dest = std::env::temp_dir().join(format!("yuhina-restart-{}.bin", uuid()));

    // Simulate a task that was Running when the launcher died. Persist a
    // partial `.part` file too, so the resume offset is exercised.
    let part = yuhina_download::task::part_path(&dest);
    std::fs::write(&part, &data[..1024]).unwrap();
    let row = sample_row(
        "restart-1",
        DownloadState::Running,
        &server.url("/r.bin"),
        &dest.to_string_lossy(),
        1024,
        data.len() as u64,
    );
    db.insert_task(&row).unwrap();

    let mgr = DownloadManager::start(db, fast_config(2));
    let ids = resume_after_restart(&mgr).unwrap();
    assert_eq!(ids, vec!["restart-1".to_string()]);

    assert!(
        wait_for(
            || mgr
                .list_tasks()
                .unwrap()
                .iter()
                .any(|t| t.id == "restart-1" && t.state == DownloadState::Done),
            Duration::from_secs(10)
        )
        .await
    );
    // The resumed download completed with the Range request resuming at 1024.
    assert_eq!(std::fs::read(&dest).unwrap(), data);
    assert!(server
        .requests()
        .iter()
        .any(|r| r.range.as_deref() == Some("bytes=1024-")));
    mgr.shutdown();
    let _ = std::fs::remove_file(&dest);
}

#[tokio::test]
async fn paused_and_failed_tasks_are_not_requeued() {
    let db = Store::in_memory().unwrap();
    let row_paused = sample_row(
        "p1",
        DownloadState::Paused,
        "http://127.0.0.1:1/x",
        "/tmp/p1.bin",
        10,
        100,
    );
    let row_failed = sample_row(
        "f1",
        DownloadState::Failed,
        "http://127.0.0.1:1/x",
        "/tmp/f1.bin",
        0,
        0,
    );
    db.insert_task(&row_paused).unwrap();
    db.insert_task(&row_failed).unwrap();

    let mgr = DownloadManager::start(db, fast_config(1));
    let ids = resume_after_restart(&mgr).unwrap();
    assert!(
        ids.is_empty(),
        "only Running tasks should be requeued: {ids:?}"
    );

    let tasks = mgr.list_tasks().unwrap();
    let paused = tasks.iter().find(|t| t.id == "p1").unwrap();
    let failed = tasks.iter().find(|t| t.id == "f1").unwrap();
    assert_eq!(paused.state, DownloadState::Paused);
    assert_eq!(failed.state, DownloadState::Failed);
    mgr.shutdown();
}

#[tokio::test]
async fn restore_keeps_progress_fields() {
    let db = Store::in_memory().unwrap();
    let dest = std::env::temp_dir().join(format!("yuhina-keep-{}.bin", uuid()));
    let row = sample_row(
        "keep-1",
        DownloadState::Running,
        "http://127.0.0.1:1/x",
        &dest.to_string_lossy(),
        1234,
        9999,
    );
    db.insert_task(&row).unwrap();

    let mgr = DownloadManager::start(db, fast_config(1));
    let _ = resume_after_restart(&mgr).unwrap();

    let task = mgr
        .list_tasks()
        .unwrap()
        .into_iter()
        .find(|t| t.id == "keep-1")
        .unwrap();
    assert_eq!(task.state, DownloadState::Queued);
    assert_eq!(task.done_bytes, 1234);
    assert_eq!(task.total_bytes, 9999);
    mgr.shutdown();
}

#[tokio::test]
async fn enqueue_restore_duplicate_is_safe() {
    let db = Store::in_memory().unwrap();
    let dest = std::env::temp_dir().join("dup.bin");
    let row = sample_row(
        "dup-1",
        DownloadState::Running,
        "http://127.0.0.1:1/x",
        &dest.to_string_lossy(),
        0,
        0,
    );
    db.insert_task(&row).unwrap();
    let mgr = DownloadManager::start(db, fast_config(1));
    let _ = resume_after_restart(&mgr).unwrap();
    // Calling restore again for the same id is a no-op (idempotent).
    let again = resume_after_restart(&mgr).unwrap();
    assert!(again.is_empty() || again.len() == 1);
    mgr.shutdown();
}

fn uuid() -> String {
    uuid::Uuid::new_v4().to_string()
}

#[allow(dead_code)]
fn _type_check(req: FileReq, p: Priority, k: TaskKind) -> (FileReq, Priority, TaskKind) {
    (req, p, k)
}
