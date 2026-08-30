//! Startup recovery (task T3): persisted `Running` tasks → `Queued` so they
//! resume from their `.part` files after a launcher restart.

use yuhina_api::DownloadState;

use crate::manager::DownloadManager;
use crate::store::StoredTask;
use crate::YuhinaResult;

/// Re-enqueues every persisted `Running` task.
///
/// - `Running` → set to `Queued` and pushed onto the worker queue; the
///   download resumes from the `.part` offset (done_bytes are kept).
/// - `Paused` stays paused; `Failed` is kept for the user to retry manually.
pub fn resume_after_restart(manager: &DownloadManager) -> YuhinaResult<Vec<String>> {
    let rows: Vec<StoredTask> = manager
        .store()
        .list_tasks_by_state(&DownloadState::Running)
        .unwrap_or_default();
    let mut ids = Vec::new();
    for row in rows {
        if let Ok(id) = manager.restore(&row) {
            ids.push(id);
        }
    }
    Ok(ids)
}