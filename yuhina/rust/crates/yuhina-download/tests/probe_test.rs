mod common;
use common::{fast_config, DropServer};
use std::time::Duration;
use yuhina_api::DownloadState;
use yuhina_download::{DownloadManager, FileReq, Priority, TaskKind};

#[tokio::test]
async fn manager_drop_probe() {
    let data: Vec<u8> = (0..4096u32).map(|i| (i % 251) as u8).collect();
    let server = DropServer::start(data.clone(), 1024);
    let mgr = DownloadManager::start(yuhina_download::Store::in_memory().unwrap(), fast_config(1));
    let dest = std::env::temp_dir().join(format!("drop-{}.bin", uuid::Uuid::new_v4()));
    let id = mgr
        .enqueue(FileReq {
            id: None,
            title: "t".into(),
            url: server.url("/d.bin"),
            dest: dest.clone(),
            sha1: None,
            priority: Priority::Library,
            kind: TaskKind::Library,
            instance_id: None,
        })
        .unwrap();
    let start = std::time::Instant::now();
    loop {
        for t in mgr.list_tasks().unwrap() {
            if t.id == id {
                eprintln!(
                    "state={:?} done={} total={}",
                    t.state, t.done_bytes, t.total_bytes
                );
            }
        }
        if start.elapsed() > Duration::from_secs(6) {
            break;
        }
        if mgr.list_tasks().unwrap().iter().any(|t| {
            t.id == id
                && matches!(
                    t.state,
                    DownloadState::Done | DownloadState::Failed | DownloadState::Canceled
                )
        }) {
            break;
        }
        tokio::time::sleep(Duration::from_millis(100)).await;
    }
    eprintln!("hit_count={}", server.hit_count());
    eprintln!("part={}", yuhina_download::task::part_path(&dest).exists());
    eprintln!("final={}", dest.exists());
    let _ = std::fs::remove_file(&dest);
    mgr.shutdown();
}
