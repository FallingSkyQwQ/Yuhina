//! SQLite persistence layer for Yuhina.
//!
//! - Schema & migrations via `PRAGMA user_version` (`schema` module).
//! - Typed repositories over a single `Connection` (`repo` module).
//!
//! # Threading
//! `Db` owns a `rusqlite::Connection` which is `!Sync`. The service layer
//! should share it behind `Arc<Mutex<Db>>`/`Arc<RwLock<Db>>` when used from
//! tokio tasks. Repos borrow the connection for the duration of a call.

use std::path::Path;

use anyhow::{Context, Result};
use rusqlite::Connection;

pub mod repo;
pub mod schema;

use repo::{
    AccountRepo, DownloadTaskRepo, InstalledModRepo, InstanceRepo, JavaRepo, NewsCacheRepo,
    SettingsRepo, VersionCacheRepo,
};

/// Milliseconds since UNIX epoch (used for all timestamps).
pub fn now_millis() -> i64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_millis() as i64)
        .unwrap_or(0)
}

pub struct Db {
    conn: Connection,
}

impl Db {
    /// Opens (creating if needed) the database at `path`, enables WAL +
    /// foreign keys and applies pending migrations. Parent directories are
    /// created automatically.
    pub fn new(path: impl AsRef<Path>) -> Result<Self> {
        let path = path.as_ref();
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)
                .with_context(|| format!("create db parent dir {}", parent.display()))?;
        }
        let conn = Connection::open(path)
            .with_context(|| format!("open sqlite db {}", path.display()))?;
        conn.pragma_update(None, "journal_mode", "WAL")?;
        conn.pragma_update(None, "foreign_keys", "ON")?;
        conn.pragma_update(None, "busy_timeout", 5000)?;
        let db = Self { conn };
        db.migrate()?;
        Ok(db)
    }

    /// Opens an in-memory database (tests, temp caches).
    pub fn in_memory() -> Result<Self> {
        let conn = Connection::open_in_memory()?;
        conn.pragma_update(None, "foreign_keys", "ON")?;
        let db = Self { conn };
        db.migrate()?;
        Ok(db)
    }

    pub fn migrate(&self) -> Result<()> {
        schema::migrate(&self.conn)
    }

    pub fn conn(&self) -> &Connection {
        &self.conn
    }

    pub fn settings_repo(&self) -> SettingsRepo<'_> {
        SettingsRepo::new(&self.conn)
    }

    pub fn account_repo(&self) -> AccountRepo<'_> {
        AccountRepo::new(&self.conn)
    }

    pub fn instance_repo(&self) -> InstanceRepo<'_> {
        InstanceRepo::new(&self.conn)
    }

    pub fn installed_mod_repo(&self) -> InstalledModRepo<'_> {
        InstalledModRepo::new(&self.conn)
    }

    pub fn download_task_repo(&self) -> DownloadTaskRepo<'_> {
        DownloadTaskRepo::new(&self.conn)
    }

    pub fn java_repo(&self) -> JavaRepo<'_> {
        JavaRepo::new(&self.conn)
    }

    pub fn version_cache_repo(&self) -> VersionCacheRepo<'_> {
        VersionCacheRepo::new(&self.conn)
    }

    pub fn news_cache_repo(&self) -> NewsCacheRepo<'_> {
        NewsCacheRepo::new(&self.conn)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn db_opens_and_migrates() {
        let dir = tempfile::tempdir().unwrap();
        let db = Db::new(dir.path().join("sub/dir/yuhina.db")).unwrap();
        let v: i64 = db
            .conn()
            .query_row("PRAGMA user_version", [], |r| r.get(0))
            .unwrap();
        assert_eq!(v, schema::SCHEMA_VERSION);
        // WAL enabled
        let mode: String = db
            .conn()
            .query_row("PRAGMA journal_mode", [], |r| r.get(0))
            .unwrap();
        assert_eq!(mode.to_lowercase(), "wal");
    }

    #[test]
    fn db_in_memory() {
        let db = Db::in_memory().unwrap();
        assert!(db.conn().is_autocommit());
    }

    #[test]
    fn all_tables_exist() {
        let db = Db::in_memory().unwrap();
        let mut stmt = db
            .conn()
            .prepare("SELECT name FROM sqlite_master WHERE type='table'")
            .unwrap();
        let tables: Vec<String> = stmt
            .query_map([], |r| r.get(0))
            .unwrap()
            .collect::<rusqlite::Result<_>>()
            .unwrap();
        for t in [
            "settings",
            "accounts",
            "instances",
            "installed_mods",
            "download_tasks",
            "java_runtimes",
            "version_cache",
            "news_cache",
        ] {
            assert!(tables.iter().any(|x| x == t), "missing table {t}");
        }
        // indexes
        let mut stmt = db
            .conn()
            .prepare("SELECT name FROM sqlite_master WHERE type='index' AND name LIKE 'idx_%'")
            .unwrap();
        let idxs: Vec<String> = stmt
            .query_map([], |r| r.get(0))
            .unwrap()
            .collect::<rusqlite::Result<_>>()
            .unwrap();
        assert!(idxs.iter().any(|x| x == "idx_installed_mods_instance"));
        assert!(idxs.iter().any(|x| x == "idx_download_tasks_state"));
    }
}