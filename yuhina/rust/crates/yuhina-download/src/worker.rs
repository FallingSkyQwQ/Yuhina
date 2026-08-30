//! Download worker: single-file HTTP download with Range resume, exponential
//! backoff retries, sha1 verification and pause/cancel cooperation.

use std::sync::Arc;

use reqwest::StatusCode;
use tokio::fs::OpenOptions;
use tokio::io::AsyncWriteExt;
use tokio_stream::StreamExt;

use yuhina_api::DownloadState;

use crate::checksum::verify_file;
use crate::manager::{Inner, Progress, TaskHandle, finish, set_state};
use crate::task::part_path;

/// Outcome of a single download job.
pub enum JobOutcome {
    Done,
    Paused,
    Canceled,
    Failed { error: String },
}

/// A worker task: pops jobs from the shared queue until shutdown.
pub(crate) async fn worker_loop(inner: Arc<Inner>) {
    loop {
        let Some(id) = inner.queue.pop(&inner.shutdown).await else {
            break;
        };
        let Some(handle) = inner.tasks.lock().unwrap().get(&id).cloned() else {
            continue;
        };

        // Gate checks: a job may have been paused/cancelled while queued.
        if handle.cancel.load(std::sync::atomic::Ordering::Relaxed) {
            finish(&inner, &handle, DownloadState::Canceled, None).await;
            continue;
        }
        if handle.pause.load(std::sync::atomic::Ordering::Relaxed)
            || handle.state.lock().unwrap().clone() == DownloadState::Paused
        {
            finish(&inner, &handle, DownloadState::Paused, None).await;
            continue;
        }

        set_state(&inner, &handle, DownloadState::Running, None).await;
        let outcome = run_download(&inner, &handle).await;
        match outcome {
            JobOutcome::Done => finish(&inner, &handle, DownloadState::Done, None).await,
            JobOutcome::Paused => {
                // .part is preserved; done_bytes were persisted on the way out.
                set_state(&inner, &handle, DownloadState::Paused, None).await
            }
            JobOutcome::Canceled => {
                let _ = tokio::fs::remove_file(part_path(&handle.req.dest)).await;
                finish(&inner, &handle, DownloadState::Canceled, None).await;
            }
            JobOutcome::Failed { error } => {
                finish(&inner, &handle, DownloadState::Failed, Some(error)).await;
            }
        }
    }
}

/// Downloads one file with retry + resume. Partial data is kept in `.part`
/// across network failures and pause/resume; on checksum mismatch the `.part`
/// is deleted and the download restarts from scratch (a corrupted partial can
/// never be repaired by appending).
async fn run_download(inner: &Arc<Inner>, handle: &Arc<TaskHandle>) -> JobOutcome {
    let cfg = &inner.config;
    let req = &handle.req;
    let part = part_path(&req.dest);

    let mut network_attempts: u32 = 0;
    let mut checksum_attempts: u32 = 0;

    'outer: loop {
        if handle.cancel.load(std::sync::atomic::Ordering::Relaxed) {
            return JobOutcome::Canceled;
        }
        if handle.pause.load(std::sync::atomic::Ordering::Relaxed) {
            return JobOutcome::Paused;
        }

        let start = existing_part_len(&part).await;

        // Only ask for a Range when we already have partial data.
        let mut req_builder = inner.client.get(&req.url);
        if start > 0 {
            req_builder = req_builder.header("Range", format!("bytes={start}-"));
        }
        let response = match req_builder.send().await {
            Ok(r) => r,
            Err(e) => {
                if network_attempts < cfg.retry_max {
                    network_attempts += 1;
                    backoff(cfg, network_attempts).await;
                    continue;
                }
                return JobOutcome::Failed {
                    error: format!("request failed: {e}"),
                };
            }
        };

        let status = response.status();

        // Retryable server errors (5xx) get the backoff treatment.
        if status.is_server_error() {
            drop(response);
            if network_attempts < cfg.retry_max {
                network_attempts += 1;
                backoff(cfg, network_attempts).await;
                continue;
            }
            return JobOutcome::Failed {
                error: format!("HTTP {}", status.as_u16()),
            };
        }

        // 416: the requested range starts beyond the end of the resource, so
        // the `.part` file already holds everything. Go verify + finalize.
        if status == StatusCode::RANGE_NOT_SATISFIABLE {
            drop(response);
            if let Some(outcome) = verify_and_finalize(handle, &part).await {
                return outcome;
            }
            // Verification failed and retries remain → loop (fresh download).
            if checksum_attempts < cfg.retry_max {
                checksum_attempts += 1;
                network_attempts = 0;
                continue;
            }
            return JobOutcome::Failed {
                error: format!(
                    "sha1 mismatch, expected {}",
                    req.sha1.as_deref().unwrap_or("<none>")
                ),
            };
        }

        // Non-retryable client errors (404 etc.) fail immediately.
        if !status.is_success() && status != StatusCode::PARTIAL_CONTENT {
            return JobOutcome::Failed {
                error: format!("HTTP {}", status.as_u16()),
            };
        }

        let resumed = status == StatusCode::PARTIAL_CONTENT;
        let start_actual = if resumed { start } else { 0 };
        if let Some(total) = response_total(&response, start_actual, resumed) {
            *handle.progress.lock().unwrap() = Progress {
                done: start_actual,
                total,
            };
            let _ = inner.store.set_task_total(&handle.id, total);
        }

        // Open the `.part` file: truncate for a fresh download, append for a
        // resumed one.
        let mut opts = OpenOptions::new();
        opts.write(true).create(true);
        if start_actual == 0 {
            opts.truncate(true);
        } else {
            opts.append(true);
        }
        let mut file = match opts.open(&part).await {
            Ok(f) => f,
            Err(e) => return JobOutcome::Failed { error: format!("open part file: {e}") },
        };

        let mut done = start_actual;
        let mut stream = response.bytes_stream();
        while let Some(chunk) = stream.next().await {
            if handle.cancel.load(std::sync::atomic::Ordering::Relaxed) {
                return JobOutcome::Canceled; // keep .part for nothing: canceled
            }
            if handle.pause.load(std::sync::atomic::Ordering::Relaxed) {
                return JobOutcome::Paused; // keep .part for resume
            }
            match chunk {
                Ok(bytes) => {
                    if let Err(e) = file.write_all(&bytes).await {
                        return JobOutcome::Failed { error: format!("write part file: {e}") };
                    }
                    done += bytes.len() as u64;
                    let mut p = handle.progress.lock().unwrap();
                    p.done = done;
                    if p.total == 0 {
                        p.total = done;
                    }
                }
                Err(e) => {
                    // Mid-stream network failure: resume from the partial file.
                    if network_attempts < cfg.retry_max {
                        network_attempts += 1;
                        let _ = file.flush().await;
                        drop(file);
                        drop(stream);
                        backoff(cfg, network_attempts).await;
                        continue 'outer;
                    }
                    return JobOutcome::Failed {
                        error: format!("stream error: {e}"),
                    };
                }
            }
        }
        drop(file);
        drop(stream);

        if handle.cancel.load(std::sync::atomic::Ordering::Relaxed) {
            return JobOutcome::Canceled;
        }
        if handle.pause.load(std::sync::atomic::Ordering::Relaxed) {
            return JobOutcome::Paused;
        }

        match verify_and_finalize(handle, &part).await {
            Some(outcome) => return outcome, // Done | Failed (final or exhausted)
            None => {
                // Checksum mismatch with retries remaining → fresh download.
                if checksum_attempts < cfg.retry_max {
                    checksum_attempts += 1;
                    network_attempts = 0;
                    continue;
                }
                return JobOutcome::Failed {
                    error: format!(
                        "sha1 mismatch, expected {}",
                        req.sha1.as_deref().unwrap_or("<none>")
                    ),
                };
            }
        }
    }
}

/// Verifies the `.part` checksum (when one is expected) and atomically
/// renames it to the destination.
///
/// Returns:
/// - `Some(JobOutcome)` when the job is finished (Done, or a terminal failure
///   such as a checksum/IO error);
/// - `None` when verification failed and a fresh retry should be attempted.
async fn verify_and_finalize(
    handle: &Arc<TaskHandle>,
    part: &std::path::Path,
) -> Option<JobOutcome> {
    let req = &handle.req;
    if let Some(expected) = &req.sha1 {
        match verify_file(part, expected) {
            Ok(true) => {}
            Ok(false) => {
                let _ = tokio::fs::remove_file(part).await;
                return None; // corrupted partial → retry from scratch
            }
            Err(e) => {
                return Some(JobOutcome::Failed {
                    error: format!("checksum error: {e}"),
                })
            }
        }
    }

    match tokio::fs::rename(part, &req.dest).await {
        Ok(()) => Some(JobOutcome::Done),
        Err(e) => Some(JobOutcome::Failed {
            error: format!("finalize rename: {e}"),
        }),
    }
}

/// Length of the existing `.part` file (0 when absent). This is the resume
/// offset.
async fn existing_part_len(part: &std::path::Path) -> u64 {
    match tokio::fs::metadata(part).await {
        Ok(m) => m.len(),
        Err(_) => 0,
    }
}

/// Total resource size from the response (Content-Range total for 206,
/// Content-Length for 200). `None` when unknown.
fn response_total(response: &reqwest::Response, start: u64, resumed: bool) -> Option<u64> {
    if resumed {
        if let Some(cr) = response.headers().get("content-range") {
            if let Ok(s) = cr.to_str() {
                if let Some(total) = s.rsplit('/').next() {
                    if let Ok(t) = total.trim().parse::<u64>() {
                        return Some(t);
                    }
                }
            }
        }
        response.content_length().map(|len| start + len)
    } else {
        response.content_length()
    }
}

/// Exponential backoff: base·2^(attempt-1), capped at `backoff_cap_ms`.
async fn backoff(cfg: &crate::manager::ManagerConfig, attempt: u32) {
    let exp = 1u64 << (attempt.saturating_sub(1).min(6));
    let ms = cfg
        .backoff_base_ms
        .saturating_mul(exp)
        .min(cfg.backoff_cap_ms.max(1));
    tokio::time::sleep(std::time::Duration::from_millis(ms)).await;
}