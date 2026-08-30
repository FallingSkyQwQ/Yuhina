//! Persistence for the download domain (contract §5 tables `download_tasks`
//! and `news_cache`).
//!
//! Self-contained over `rusqlite` so the crate does not depend on the
//! (still evolving) `yuhina-db` repository layer. Tables are created with
//! `IF NOT EXISTS` so they coexist with `yuhina-db`'s schema migration.

use std::path::Path;
use std::sync::{Arc, Mutex};

use rusqlite::{Connection, OptionalExtension, params};

use yuhina_api::{DownloadState, DownloadTask, NewsItem, YuhinaError, YuhinaErrorKind};

/// Milliseconds since the UNIX epoch.
pub fn now_ms() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_millis() as u64)
        .unwrap_or(0)
}

/// A persisted `download_tasks` row.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StoredTask {
    pub id: String,
    pub kind: String,
    pub title: String,
    pub instance_id: Option<String>,
    pub url: String,
    pub target_path: String,
    pub total_bytes: u64,
    pub done_bytes: u64,
    pub state: DownloadState,
    pub checksum_sha1: Option<String>,
    pub error: Option<String>,
    pub created_at: u64,
    pub updated_at: u64,
}

impl StoredTask {
    /// The public UI view (contract §2.6).
    pub fn to_public(&self) -> DownloadTask {
        let (can_pause, can_cancel) = match self.state {
            DownloadState::Queued | DownloadState::Running => (true, true),
            DownloadState::Paused => (false, true),
            _ => (false, false),
        };
        DownloadTask {
            id: self.id.clone(),
            title: self.title.clone(),
            state: self.state.clone(),
            total_bytes: self.total_bytes,
            done_bytes: self.done_bytes,
            speed_bps: 0,
            error: self.error.clone(),
            created_at: self.created_at,
            can_pause,
            can_cancel,
        }
    }
}

/// SQLite-backed store for download tasks and the news cache.
#[derive(Clone)]
pub struct Store {
    conn: Arc<Mutex<Connection>>,
}

impl Store {
    /// Opens (creating if needed) the database at `path`.
    pub fn open(path: &Path) -> Result<Self, YuhinaError> {
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).map_err(io_err)?;
        }
        let conn = Connection::open(path).map_err(sql_err)?;
        conn.pragma_update(None, "journal_mode", "WAL").map_err(sql_err)?;
        conn.pragma_update(None, "foreign_keys", "ON").map_err(sql_err)?;
        let store = Self {
            conn: Arc::new(Mutex::new(conn)),
        };
        store.init_schema()?;
        Ok(store)
    }

    /// Opens an in-memory database (tests).
    pub fn in_memory() -> Result<Self, YuhinaError> {
        let conn = Connection::open_in_memory().map_err(sql_err)?;
        conn.pragma_update(None, "foreign_keys", "ON").map_err(sql_err)?;
        let store = Self {
            conn: Arc::new(Mutex::new(conn)),
        };
        store.init_schema()?;
        Ok(store)
    }

    /// Create the download-domain tables if missing (idempotent, matches §5).
    fn init_schema(&self) -> Result<(), YuhinaError> {
        let conn = self.lock();
        conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS download_tasks (
               id TEXT PRIMARY KEY, kind TEXT NOT NULL, title TEXT NOT NULL,
               instance_id TEXT, url TEXT NOT NULL, target_path TEXT NOT NULL,
               total_bytes INTEGER, done_bytes INTEGER, state TEXT NOT NULL,
               checksum_sha1 TEXT, error TEXT, created_at INTEGER, updated_at INTEGER);
             CREATE INDEX IF NOT EXISTS idx_download_tasks_state ON download_tasks(state);
             CREATE TABLE IF NOT EXISTS news_cache (
               id TEXT PRIMARY KEY, title TEXT NOT NULL, url TEXT NOT NULL,
               published TEXT, summary TEXT, fetched_at INTEGER);",
        )
        .map_err(sql_err)?;
        Ok(())
    }

    // ---------------------------------------------------------------------
    // download_tasks
    // ---------------------------------------------------------------------

    /// Inserts or replaces a task row.
    pub fn insert_task(&self, t: &StoredTask) -> Result<(), YuhinaError> {
        self.lock()
            .execute(
                "INSERT OR REPLACE INTO download_tasks
                   (id, kind, title, instance_id, url, target_path, total_bytes,
                    done_bytes, state, checksum_sha1, error, created_at, updated_at)
                 VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9,?10,?11,?12,?13)",
                params![
                    t.id,
                    t.kind,
                    t.title,
                    t.instance_id,
                    t.url,
                    t.target_path,
                    t.total_bytes as i64,
                    t.done_bytes as i64,
                    state_str(&t.state),
                    t.checksum_sha1,
                    t.error,
                    t.created_at as i64,
                    t.updated_at as i64,
                ],
            )
            .map_err(sql_err)?;
        Ok(())
    }

    /// Progress-only update for a running task.
    pub fn update_task(
        &self,
        id: &str,
        state: &DownloadState,
        done_bytes: u64,
        error: Option<&str>,
    ) -> Result<(), YuhinaError> {
        self.lock()
            .execute(
                "UPDATE download_tasks SET state=?1, done_bytes=?2, error=?3, updated_at=?4
                 WHERE id=?5",
                params![
                    state_str(state),
                    done_bytes as i64,
                    error,
                    now_ms() as i64,
                    id,
                ],
            )
            .map_err(sql_err)?;
        Ok(())
    }

    /// Updates the known total size.
    pub fn set_task_total(&self, id: &str, total: u64) -> Result<(), YuhinaError> {
        self.lock()
            .execute(
                "UPDATE download_tasks SET total_bytes=?1, updated_at=?2 WHERE id=?3",
                params![total as i64, now_ms() as i64, id],
            )
            .map_err(sql_err)?;
        Ok(())
    }

    /// Fetches one task row.
    pub fn get_task(&self, id: &str) -> Result<Option<StoredTask>, YuhinaError> {
        self.lock()
            .query_row(
                "SELECT id, kind, title, instance_id, url, target_path, total_bytes,
                        done_bytes, state, checksum_sha1, error, created_at, updated_at
                 FROM download_tasks WHERE id=?1",
                [id],
                |r| row_from(r),
            )
            .optional()
            .map_err(sql_err)
    }

    /// All tasks, oldest first.
    pub fn list_tasks(&self) -> Result<Vec<StoredTask>, YuhinaError> {
        let conn = self.lock();
        let mut stmt = conn
            .prepare(
                "SELECT id, kind, title, instance_id, url, target_path, total_bytes,
                        done_bytes, state, checksum_sha1, error, created_at, updated_at
                 FROM download_tasks ORDER BY created_at ASC",
            )
            .map_err(sql_err)?;
        let rows = stmt
            .query_map([], |r| row_from(r))
            .map_err(sql_err)?;
        collect_rows(rows)
    }

    /// Tasks in a given state.
    pub fn list_tasks_by_state(
        &self,
        state: &DownloadState,
    ) -> Result<Vec<StoredTask>, YuhinaError> {
        let conn = self.lock();
        let mut stmt = conn
            .prepare(
                "SELECT id, kind, title, instance_id, url, target_path, total_bytes,
                        done_bytes, state, checksum_sha1, error, created_at, updated_at
                 FROM download_tasks WHERE state=?1 ORDER BY created_at ASC",
            )
            .map_err(sql_err)?;
        let rows = stmt
            .query_map([state_str(state)], |r| row_from(r))
            .map_err(sql_err)?;
        collect_rows(rows)
    }

    /// Deletes one task row.
    pub fn delete_task(&self, id: &str) -> Result<(), YuhinaError> {
        self.lock()
            .execute("DELETE FROM download_tasks WHERE id=?1", [id])
            .map_err(sql_err)?;
        Ok(())
    }

    /// Removes Done / Failed / Canceled rows.
    pub fn clear_finished_tasks(&self) -> Result<(), YuhinaError> {
        self.lock()
            .execute(
                "DELETE FROM download_tasks WHERE state IN ('done','failed','canceled')",
                [],
            )
            .map_err(sql_err)?;
        Ok(())
    }

    // ---------------------------------------------------------------------
    // news_cache
    // ---------------------------------------------------------------------

    /// Replaces the whole news cache with `items` (all stamped `now`).
    pub fn replace_news(&self, items: &[NewsItem]) -> Result<(), YuhinaError> {
        let conn = self.lock();
        let tx = conn.unchecked_transaction().map_err(sql_err)?;
        tx.execute("DELETE FROM news_cache", []).map_err(sql_err)?;
        {
            let mut stmt = tx
                .prepare(
                    "INSERT OR REPLACE INTO news_cache (id, title, url, published, summary, fetched_at)
                     VALUES (?1,?2,?3,?4,?5,?6)",
                )
                .map_err(sql_err)?;
            for item in items {
                stmt.execute(params![
                    item.url.clone(),
                    item.title,
                    item.url,
                    (!item.published.is_empty()).then_some(item.published.clone()),
                    (!item.summary.is_empty()).then_some(item.summary.clone()),
                    now_ms() as i64,
                ])
                .map_err(sql_err)?;
            }
        }
        tx.commit().map_err(sql_err)?;
        Ok(())
    }

    /// Cached news items (newest published first).
    pub fn list_news(&self) -> Result<Vec<NewsItem>, YuhinaError> {
        let conn = self.lock();
        let mut stmt = conn
            .prepare(
                "SELECT title, url, published, summary FROM news_cache ORDER BY published DESC",
            )
            .map_err(sql_err)?;
        let rows = stmt
            .query_map([], |r| {
                Ok(NewsItem {
                    title: r.get(0)?,
                    url: r.get(1)?,
                    published: r.get::<_, Option<String>>(2)?.unwrap_or_default(),
                    summary: r.get::<_, Option<String>>(3)?.unwrap_or_default(),
                })
            })
            .map_err(sql_err)?;
        rows.collect::<rusqlite::Result<Vec<_>>>().map_err(sql_err)
    }

    /// Newest `fetched_at` in the news cache, if any.
    pub fn latest_news_fetched_at(&self) -> Result<Option<i64>, YuhinaError> {
        self.lock()
            .query_row(
                "SELECT MAX(fetched_at) FROM news_cache",
                [],
                |r| r.get::<_, Option<i64>>(0),
            )
            .map_err(sql_err)
    }

    fn lock(&self) -> std::sync::MutexGuard<'_, Connection> {
        self.conn.lock().unwrap()
    }
}

fn state_str(state: &DownloadState) -> &'static str {
    match state {
        DownloadState::Queued => "queued",
        DownloadState::Running => "running",
        DownloadState::Paused => "paused",
        DownloadState::Done => "done",
        DownloadState::Failed => "failed",
        DownloadState::Canceled => "canceled",
    }
}

fn state_from_str(s: &str) -> Option<DownloadState> {
    match s {
        "queued" => Some(DownloadState::Queued),
        "running" => Some(DownloadState::Running),
        "paused" => Some(DownloadState::Paused),
        "done" => Some(DownloadState::Done),
        "failed" => Some(DownloadState::Failed),
        "canceled" => Some(DownloadState::Canceled),
        _ => None,
    }
}

fn row_from(r: &rusqlite::Row<'_>) -> rusqlite::Result<StoredTask> {
    let state_str: String = r.get(8)?;
    let state = state_from_str(&state_str).ok_or_else(|| {
        rusqlite::Error::FromSqlConversionFailure(
            8,
            rusqlite::types::Type::Text,
            format!("unknown download state '{state_str}'").into(),
        )
    })?;
    Ok(StoredTask {
        id: r.get(0)?,
        kind: r.get(1)?,
        title: r.get(2)?,
        instance_id: r.get(3)?,
        url: r.get(4)?,
        target_path: r.get(5)?,
        total_bytes: r.get::<_, i64>(6)?.max(0) as u64,
        done_bytes: r.get::<_, i64>(7)?.max(0) as u64,
        state,
        checksum_sha1: r.get(9)?,
        error: r.get(10)?,
        created_at: r.get::<_, i64>(11)?.max(0) as u64,
        updated_at: r.get::<_, i64>(12)?.max(0) as u64,
    })
}

fn collect_rows(
    rows: rusqlite::MappedRows<'_, impl FnMut(&rusqlite::Row<'_>) -> rusqlite::Result<StoredTask>>,
) -> Result<Vec<StoredTask>, YuhinaError> {
    rows.collect::<rusqlite::Result<Vec<_>>>().map_err(sql_err)
}

fn sql_err(e: rusqlite::Error) -> YuhinaError {
    YuhinaError::new(YuhinaErrorKind::Io, format!("sqlite: {e}"))
}

fn io_err(e: std::io::Error) -> YuhinaError {
    YuhinaError::new(YuhinaErrorKind::Io, e.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample(id: &str, state: DownloadState) -> StoredTask {
        StoredTask {
            id: id.into(),
            kind: "library".into(),
            title: "t".into(),
            instance_id: None,
            url: "https://example.com/f".into(),
            target_path: "/tmp/f".into(),
            total_bytes: 100,
            done_bytes: 0,
            state,
            checksum_sha1: None,
            error: None,
            created_at: 1,
            updated_at: 1,
        }
    }

    #[test]
    fn task_crud_and_state_filter() {
        let store = Store::in_memory().unwrap();
        store.insert_task(&sample("a", DownloadState::Running)).unwrap();
        store.insert_task(&sample("b", DownloadState::Paused)).unwrap();
        store.update_task("a", &DownloadState::Running, 40, None).unwrap();
        store.set_task_total("a", 200).unwrap();
        let a = store.get_task("a").unwrap().unwrap();
        assert_eq!(a.done_bytes, 40);
        assert_eq!(a.total_bytes, 200);
        assert_eq!(store.list_tasks_by_state(&DownloadState::Running).unwrap().len(), 1);
        assert_eq!(store.list_tasks().unwrap().len(), 2);
        store.update_task("a", &DownloadState::Done, 200, None).unwrap();
        store.clear_finished_tasks().unwrap();
        assert!(store.get_task("a").unwrap().is_none());
        assert!(store.get_task("b").unwrap().is_some());
        assert_eq!(store.list_tasks().unwrap().len(), 1);
    }

    #[test]
    fn public_view_flags() {
        let t = sample("x", DownloadState::Running);
        let p = t.to_public();
        assert!(p.can_pause && p.can_cancel);
        let t = sample("x", DownloadState::Paused).to_public();
        assert!(!t.can_pause && t.can_cancel);
    }

    #[test]
    fn news_replace_and_read() {
        let store = Store::in_memory().unwrap();
        assert_eq!(store.list_news().unwrap().len(), 0);
        assert_eq!(store.latest_news_fetched_at().unwrap(), None);
        store
            .replace_news(&[NewsItem {
                title: "A".into(),
                url: "https://a".into(),
                published: "d".into(),
                summary: "s".into(),
            }])
            .unwrap();
        assert_eq!(store.list_news().unwrap().len(), 1);
        assert!(store.latest_news_fetched_at().unwrap().is_some());
        store.replace_news(&[]).unwrap();
        assert_eq!(store.list_news().unwrap().len(), 0);
    }
}