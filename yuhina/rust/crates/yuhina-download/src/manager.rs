//! Concurrent download manager: priority queue, workers, pause/resume/cancel,
//! throttled progress broadcast and persistence.

use std::collections::{BinaryHeap, HashMap, HashSet};
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::{Arc, Mutex};
use std::time::Duration;

use tokio::sync::{broadcast, Notify};
use tokio::task::JoinHandle;

use crate::store::{now_ms, Store, StoredTask};
use crate::task::{part_path, req_from_row, row_from_req, FileReq, Priority};
use crate::worker;
use crate::YuhinaResult;
use yuhina_api::{
    DownloadProgressEvent, DownloadState, DownloadTask, YuhinaError, YuhinaErrorKind,
};

/// Default number of concurrent downloads.
pub const DEFAULT_CONCURRENCY: usize = 8;
pub const DEFAULT_RETRY_MAX: u32 = 3;
pub const DEFAULT_BACKOFF_BASE_MS: u64 = 1_000;
pub const DEFAULT_BACKOFF_CAP_MS: u64 = 30_000;
pub const DEFAULT_PROGRESS_INTERVAL_MS: u64 = 100;
pub const DEFAULT_PERSIST_INTERVAL_MS: u64 = 1_000;

/// Tunable manager behaviour (exposed for tests / later settings UI).
#[derive(Debug, Clone)]
pub struct ManagerConfig {
    pub concurrency: usize,
    pub retry_max: u32,
    pub backoff_base_ms: u64,
    pub backoff_cap_ms: u64,
    /// Throttle interval for progress events (contract: 100ms).
    pub progress_interval_ms: u64,
    /// How often progress is persisted while running.
    pub persist_interval_ms: u64,
    pub connect_timeout: Duration,
}

impl Default for ManagerConfig {
    fn default() -> Self {
        Self {
            concurrency: DEFAULT_CONCURRENCY,
            retry_max: DEFAULT_RETRY_MAX,
            backoff_base_ms: DEFAULT_BACKOFF_BASE_MS,
            backoff_cap_ms: DEFAULT_BACKOFF_CAP_MS,
            progress_interval_ms: DEFAULT_PROGRESS_INTERVAL_MS,
            persist_interval_ms: DEFAULT_PERSIST_INTERVAL_MS,
            connect_timeout: Duration::from_secs(10),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct Progress {
    pub done: u64,
    pub total: u64,
}

/// Runtime handle for one task, shared by workers and control methods.
pub(crate) struct TaskHandle {
    pub id: String,
    pub req: FileReq,
    pub priority: Priority,
    pub pause: AtomicBool,
    pub cancel: AtomicBool,
    pub state: Mutex<DownloadState>,
    pub progress: Mutex<Progress>,
}

/// A queued job reference (id + priority + FIFO sequence).
pub(crate) struct QueueItem {
    id: String,
    priority: Priority,
    seq: u64,
}

impl PartialEq for QueueItem {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.seq == other.seq
    }
}
impl Eq for QueueItem {}
impl PartialOrd for QueueItem {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for QueueItem {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // BinaryHeap is a max-heap: higher priority first, then lower seq.
        self.priority
            .cmp(&other.priority)
            .then_with(|| other.seq.cmp(&self.seq))
    }
}

/// Async priority queue: workers block on `pop` until a job is available.
pub(crate) struct AsyncPriorityQueue {
    heap: Mutex<BinaryHeap<QueueItem>>,
    notify: Notify,
}

impl AsyncPriorityQueue {
    fn new() -> Self {
        Self {
            heap: Mutex::new(BinaryHeap::new()),
            notify: Notify::new(),
        }
    }

    pub(crate) fn push(&self, item: QueueItem) {
        self.heap.lock().unwrap().push(item);
        self.notify.notify_one();
    }

    /// Returns the next job id, or `None` when the queue is empty and the
    /// manager has been shut down (workers exit).
    pub(crate) async fn pop(&self, shutdown: &AtomicBool) -> Option<String> {
        loop {
            if let Some(item) = self.heap.lock().unwrap().pop() {
                return Some(item.id);
            }
            if shutdown.load(Ordering::Relaxed) {
                return None;
            }
            self.notify.notified().await;
        }
    }
}

pub(crate) struct Inner {
    pub store: Store,
    pub config: ManagerConfig,
    pub client: reqwest::Client,
    pub tasks: Mutex<HashMap<String, Arc<TaskHandle>>>,
    pub queue: AsyncPriorityQueue,
    pub tx: broadcast::Sender<DownloadProgressEvent>,
    pub seq: AtomicU64,
    pub shutdown: AtomicBool,
}

/// The download manager. Spawns `concurrency` workers plus a progress
/// broadcaster on construction.
pub struct DownloadManager {
    inner: Arc<Inner>,
    _workers: Vec<JoinHandle<()>>,
    _broadcaster: JoinHandle<()>,
}

impl DownloadManager {
    /// Starts a manager over an existing store and configuration.
    pub fn start(store: Store, config: ManagerConfig) -> Self {
        let client = reqwest::Client::builder()
            .connect_timeout(config.connect_timeout)
            .build()
            .expect("build reqwest client");
        Self::with_client(store, config, client)
    }

    /// Starts a manager with a caller-supplied HTTP client.
    pub fn with_client(store: Store, config: ManagerConfig, client: reqwest::Client) -> Self {
        let (tx, _rx) = broadcast::channel(256);
        let inner = Arc::new(Inner {
            store,
            config: config.clone(),
            client,
            tasks: Mutex::new(HashMap::new()),
            queue: AsyncPriorityQueue::new(),
            tx,
            seq: AtomicU64::new(0),
            shutdown: AtomicBool::new(false),
        });

        let n = config.concurrency.max(1);
        let mut workers = Vec::with_capacity(n);
        for _ in 0..n {
            let inner = Arc::clone(&inner);
            workers.push(tokio::spawn(worker::worker_loop(inner)));
        }
        let inner_bc = Arc::clone(&inner);
        let broadcaster = tokio::spawn(broadcaster_loop(inner_bc));

        Self {
            inner,
            _workers: workers,
            _broadcaster: broadcaster,
        }
    }

    /// Global progress stream. Callers may subscribe any number of times.
    pub fn subscribe(&self) -> broadcast::Receiver<DownloadProgressEvent> {
        self.inner.tx.subscribe()
    }

    /// Access to the persistence store (used by recovery queries).
    pub fn store(&self) -> &Store {
        &self.inner.store
    }

    /// Enqueues a new download. Returns the task id.
    pub fn enqueue(&self, req: FileReq) -> YuhinaResult<String> {
        let inner = &self.inner;
        let id = req
            .id
            .clone()
            .unwrap_or_else(|| uuid::Uuid::new_v4().to_string());
        if inner.tasks.lock().unwrap().contains_key(&id) {
            return Err(YuhinaError::new(
                YuhinaErrorKind::DownloadFailed,
                format!("download task {id} already exists"),
            ));
        }
        let created = now_ms();
        let priority = req.priority;
        inner
            .store
            .insert_task(&row_from_req(
                id.clone(),
                &req,
                DownloadState::Queued,
                created,
            ))
            .map_err(internal)?;
        let handle = Arc::new(TaskHandle {
            id: id.clone(),
            req,
            priority,
            pause: AtomicBool::new(false),
            cancel: AtomicBool::new(false),
            state: Mutex::new(DownloadState::Queued),
            progress: Mutex::new(Progress { done: 0, total: 0 }),
        });
        inner.tasks.lock().unwrap().insert(id.clone(), handle);
        enqueue_job(inner, &id, &priority);
        Ok(id)
    }

    /// Pauses a queued or running task. Partial data in `.part` is kept.
    pub async fn pause_task(&self, id: &str) -> YuhinaResult<()> {
        let inner = &self.inner;
        let handle = self.find_handle(id)?;
        let state = *handle.state.lock().unwrap();
        match state {
            DownloadState::Queued | DownloadState::Running => {
                handle.pause.store(true, Ordering::Relaxed);
                if state == DownloadState::Queued {
                    // Reflect immediately; the worker confirms when popped.
                    set_state(inner, &handle, DownloadState::Paused, None).await;
                }
                Ok(())
            }
            DownloadState::Paused => Ok(()),
            _ => Err(YuhinaError::new(
                YuhinaErrorKind::DownloadFailed,
                format!("task {id} cannot be paused in state {state:?}"),
            )),
        }
    }

    /// Resumes a paused (or failed) task. `.part` data is reused.
    pub async fn resume_task(&self, id: &str) -> YuhinaResult<()> {
        let inner = &self.inner;
        let handle = self.find_handle(id)?;
        let state = *handle.state.lock().unwrap();
        match state {
            DownloadState::Paused => {
                handle.pause.store(false, Ordering::Relaxed);
                handle.cancel.store(false, Ordering::Relaxed);
                set_state(inner, &handle, DownloadState::Queued, None).await;
                enqueue_job(inner, &handle.id, &handle.priority);
                Ok(())
            }
            DownloadState::Failed => {
                // A failed download (e.g. bad checksum) must start fresh.
                let _ = tokio::fs::remove_file(part_path(&handle.req.dest)).await;
                handle.pause.store(false, Ordering::Relaxed);
                handle.cancel.store(false, Ordering::Relaxed);
                set_state(inner, &handle, DownloadState::Queued, None).await;
                enqueue_job(inner, &handle.id, &handle.priority);
                Ok(())
            }
            DownloadState::Queued => Ok(()),
            _ => Err(YuhinaError::new(
                YuhinaErrorKind::DownloadFailed,
                format!("task {id} cannot be resumed in state {state:?}"),
            )),
        }
    }

    /// Cancels a task and discards its `.part` file.
    pub async fn cancel_task(&self, id: &str) -> YuhinaResult<()> {
        let inner = &self.inner;
        let handle = self.find_handle(id)?;
        let state = *handle.state.lock().unwrap();
        match state {
            DownloadState::Running | DownloadState::Queued => {
                // The worker observes the flag at the next chunk boundary and
                // performs the cleanup.
                handle.cancel.store(true, Ordering::Relaxed);
                handle.pause.store(false, Ordering::Relaxed);
                Ok(())
            }
            DownloadState::Paused | DownloadState::Failed => {
                // No worker is processing these; clean up synchronously.
                handle.cancel.store(true, Ordering::Relaxed);
                let _ = tokio::fs::remove_file(part_path(&handle.req.dest)).await;
                finish(inner, &handle, DownloadState::Canceled, None).await;
                Ok(())
            }
            DownloadState::Canceled => Ok(()),
            DownloadState::Done => {
                remove_handle(inner, &handle);
                Ok(())
            }
        }
    }

    /// Restores a persisted task back into the queue (restart / resume path).
    /// Keeps `done_bytes`/`total_bytes`; the actual resume offset comes from
    /// the `.part` file on disk.
    pub fn restore(&self, row: &StoredTask) -> YuhinaResult<String> {
        let inner = &self.inner;
        let id = row.id.clone();
        if inner.tasks.lock().unwrap().contains_key(&id) {
            return Ok(id);
        }
        let req = req_from_row(row, Priority::Library);
        inner
            .store
            .update_task(&id, &DownloadState::Queued, row.done_bytes, None)
            .map_err(internal)?;
        if row.total_bytes > 0 {
            inner
                .store
                .set_task_total(&id, row.total_bytes)
                .map_err(internal)?;
        }
        let handle = Arc::new(TaskHandle {
            id: id.clone(),
            req,
            priority: Priority::Library,
            pause: AtomicBool::new(false),
            cancel: AtomicBool::new(false),
            state: Mutex::new(DownloadState::Queued),
            progress: Mutex::new(Progress {
                done: row.done_bytes,
                total: row.total_bytes,
            }),
        });
        inner
            .tasks
            .lock()
            .unwrap()
            .insert(id.clone(), handle.clone());
        enqueue_job(inner, &id, &Priority::Library);
        Ok(id)
    }

    /// All persisted tasks (oldest first) as the public UI view.
    pub fn list_tasks(&self) -> YuhinaResult<Vec<DownloadTask>> {
        let tasks = self.inner.store.list_tasks().map_err(internal)?;
        Ok(tasks
            .into_iter()
            .map(|t| t.to_public())
            .map(fix_flags)
            .collect())
    }

    /// Removes finished tasks (Done / Failed / Canceled) from db and memory.
    pub fn clear_finished(&self) -> YuhinaResult<()> {
        self.inner.store.clear_finished_tasks().map_err(internal)?;
        let mut tasks = self.inner.tasks.lock().unwrap();
        tasks.retain(|_, h| {
            !matches!(
                *h.state.lock().unwrap(),
                DownloadState::Done | DownloadState::Canceled | DownloadState::Failed
            )
        });
        Ok(())
    }

    /// Stops workers and the broadcaster (idempotent).
    pub fn shutdown(&self) {
        let inner = &self.inner;
        if inner.shutdown.swap(true, Ordering::SeqCst) {
            return;
        }
        inner.queue.notify.notify_waiters();
    }

    pub(crate) fn find_handle(&self, id: &str) -> YuhinaResult<Arc<TaskHandle>> {
        self.inner
            .tasks
            .lock()
            .unwrap()
            .get(id)
            .cloned()
            .ok_or_else(|| {
                YuhinaError::new(
                    YuhinaErrorKind::DownloadFailed,
                    format!("download task {id} not found"),
                )
            })
    }
}

impl Drop for DownloadManager {
    fn drop(&mut self) {
        self.shutdown();
    }
}

fn enqueue_job(inner: &Inner, id: &str, priority: &Priority) {
    inner.queue.push(QueueItem {
        id: id.to_string(),
        priority: *priority,
        seq: inner.seq.fetch_add(1, Ordering::SeqCst),
    });
}

fn remove_handle(inner: &Inner, handle: &TaskHandle) {
    inner.tasks.lock().unwrap().remove(&handle.id);
}

/// Applies the runtime pause/cancel flags to a persisted public view.
fn fix_flags(mut t: DownloadTask) -> DownloadTask {
    t.can_pause = matches!(t.state, DownloadState::Queued | DownloadState::Running);
    t.can_cancel = matches!(
        t.state,
        DownloadState::Queued | DownloadState::Running | DownloadState::Paused
    );
    t
}

pub(crate) fn internal(e: impl std::fmt::Display) -> YuhinaError {
    YuhinaError::new(YuhinaErrorKind::Internal, e.to_string())
}

/// Persists a state transition + error, broadcasts it, then evicts terminal
/// tasks from memory.
pub(crate) async fn finish(
    inner: &Arc<Inner>,
    handle: &Arc<TaskHandle>,
    state: DownloadState,
    error: Option<String>,
) {
    set_state(inner, handle, state, error).await;
    broadcast_event(inner, handle, &state, 0);
    if matches!(state, DownloadState::Done | DownloadState::Canceled) {
        remove_handle(inner, handle);
    }
}

/// Persists the current state/progress (+ optional error) and updates the
/// runtime state.
pub(crate) async fn set_state(
    inner: &Arc<Inner>,
    handle: &Arc<TaskHandle>,
    state: DownloadState,
    error: Option<String>,
) {
    let prog = *handle.progress.lock().unwrap();
    *handle.state.lock().unwrap() = state;
    let _ = inner
        .store
        .update_task(&handle.id, &state, prog.done, error.as_deref());
    if prog.total > 0 {
        let _ = inner.store.set_task_total(&handle.id, prog.total);
    }
}

pub(crate) fn broadcast_event(
    inner: &Inner,
    handle: &TaskHandle,
    state: &DownloadState,
    speed_bps: u64,
) {
    let prog = *handle.progress.lock().unwrap();
    let _ = inner.tx.send(DownloadProgressEvent {
        task_id: handle.id.clone(),
        state: *state,
        done_bytes: prog.done,
        total_bytes: prog.total,
        speed_bps,
    });
}

/// Emits a progress event for one running task (used by the broadcaster).
fn emit_running(inner: &Inner, id: &str, done: u64, total: u64, speed: u64) {
    let _ = inner.tx.send(DownloadProgressEvent {
        task_id: id.to_string(),
        state: DownloadState::Running,
        done_bytes: done,
        total_bytes: total,
        speed_bps: speed,
    });
}

/// Periodic task: broadcasts progress every `progress_interval_ms` and
/// persists running-task progress every `persist_interval_ms`.
async fn broadcaster_loop(inner: Arc<Inner>) {
    let interval = inner.config.progress_interval_ms.max(1);
    let persist_every = (inner.config.persist_interval_ms / interval).max(1);
    let mut last: HashMap<String, (u128, u64)> = HashMap::new();
    let mut tick: u64 = 0;
    loop {
        tokio::time::sleep(Duration::from_millis(interval)).await;
        if inner.shutdown.load(Ordering::Relaxed) {
            break;
        }
        tick += 1;
        let now_ms = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_millis())
            .unwrap_or(0);

        // Snapshot running tasks.
        let running: Vec<(String, u64, u64)> = {
            let tasks = inner.tasks.lock().unwrap();
            tasks
                .values()
                .filter_map(|h| {
                    if *h.state.lock().unwrap() != DownloadState::Running {
                        return None;
                    }
                    let p = *h.progress.lock().unwrap();
                    Some((h.id.clone(), p.done, p.total))
                })
                .collect()
        };

        for (id, done, total) in &running {
            let speed = match last.get(id) {
                Some((pt, pd)) if *pt < now_ms && done >= pd => {
                    ((done - pd) * 1000) / (now_ms - pt) as u64
                }
                _ => 0,
            };
            last.insert(id.clone(), (now_ms, *done));
            emit_running(&inner, id, *done, *total, speed);
        }
        let running_ids: HashSet<String> = running.iter().map(|(id, _, _)| id.clone()).collect();
        last.retain(|id, _| running_ids.contains(id));

        if tick.is_multiple_of(persist_every) {
            for (id, done, total) in &running {
                let _ = inner
                    .store
                    .update_task(id, &DownloadState::Running, *done, None);
                if *total > 0 {
                    let _ = inner.store.set_task_total(id, *total);
                }
            }
        }
    }
}
