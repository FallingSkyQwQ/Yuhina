//! SQLite persistence layer for Yuhina.
//! Schema follows api-contract.md §5 (frozen). Repos are returned by
//! `Db::<repo>_repo()` and each holds a clone of the shared connection.

pub mod repos;

use std::path::Path;
use std::sync::{Arc, Mutex};

use rusqlite::Connection;

pub use repos::*;

/// Wrapped SQLite database with WAL enabled and schema migrations applied.
#[derive(Clone)]
pub struct Db {
    conn: Arc<Mutex<Connection>>,
}

impl Db {
    /// Open (or create) the database at `path`, apply migrations, enable WAL.
    pub fn new(path: &Path) -> anyhow::Result<Self> {
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let conn = Connection::open(path)?;
        conn.pragma_update(None, "journal_mode", "WAL")?;
        conn.pragma_update(None, "foreign_keys", "ON")?;
        let db = Db {
            conn: Arc::new(Mutex::new(conn)),
        };
        db.migrate()?;
        Ok(db)
    }

    /// Open an in-memory database (tests).
    pub fn in_memory() -> anyhow::Result<Self> {
        let conn = Connection::open_in_memory()?;
        conn.pragma_update(None, "foreign_keys", "ON")?;
        let db = Db {
            conn: Arc::new(Mutex::new(conn)),
        };
        db.migrate()?;
        Ok(db)
    }

    pub(crate) fn conn(&self) -> Arc<Mutex<Connection>> {
        Arc::clone(&self.conn)
    }

    pub fn account_repo(&self) -> AccountRepo {
        AccountRepo { conn: self.conn() }
    }

    pub fn java_repo(&self) -> JavaRepo {
        JavaRepo { conn: self.conn() }
    }

    pub fn version_cache_repo(&self) -> VersionCacheRepo {
        VersionCacheRepo { conn: self.conn() }
    }

    pub fn instance_repo(&self) -> InstanceRepo {
        InstanceRepo { conn: self.conn() }
    }

    pub fn installed_mod_repo(&self) -> InstalledModRepo {
        InstalledModRepo { conn: self.conn() }
    }

    fn migrate(&self) -> anyhow::Result<()> {
        let mut conn = self.conn.lock().unwrap();
        let version: i64 = conn.query_row("PRAGMA user_version", [], |r| r.get(0))?;
        if version < 1 {
            let tx = conn.transaction()?;
            schema::create_all(&tx)?;
            tx.pragma_update(None, "user_version", 1)?;
            tx.commit()?;
        }
        Ok(())
    }
}

/// Current wall-clock time in integer milliseconds since the Unix epoch.
pub fn now_millis() -> i64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_millis() as i64)
        .unwrap_or(0)
}

pub mod schema {
    use rusqlite::Connection;

    /// Create every table defined by the frozen contract §5.
    pub fn create_all(conn: &Connection) -> rusqlite::Result<()> {
        conn.execute_batch(
            r#"
            CREATE TABLE settings (
                key   TEXT PRIMARY KEY,
                value TEXT NOT NULL
            );

            CREATE TABLE accounts (
                id                TEXT PRIMARY KEY,
                kind              TEXT NOT NULL,
                username          TEXT NOT NULL,
                uuid              TEXT NOT NULL,
                yggdrasil_server  TEXT,
                skin_url          TEXT,
                access_token_enc  TEXT,
                refresh_token_enc TEXT,
                expires_at        INTEGER,
                is_active         INTEGER DEFAULT 0,
                created_at        INTEGER,
                updated_at        INTEGER
            );

            CREATE TABLE instances (
                id                TEXT PRIMARY KEY,
                name              TEXT NOT NULL,
                icon              TEXT NOT NULL DEFAULT '🎮',
                mc_version        TEXT NOT NULL,
                loader_kind       TEXT,
                loader_version    TEXT,
                game_dir          TEXT NOT NULL,
                java_auto_major   INTEGER,
                java_manual_path  TEXT,
                launch_args_json  TEXT,
                notes             TEXT DEFAULT '',
                is_installed      INTEGER DEFAULT 0,
                last_launched_at  INTEGER,
                created_at        INTEGER,
                updated_at        INTEGER
            );

            CREATE TABLE installed_mods (
                id           TEXT PRIMARY KEY,
                instance_id  TEXT NOT NULL REFERENCES instances(id) ON DELETE CASCADE,
                file_name    TEXT NOT NULL,
                file_size    INTEGER,
                sha1         TEXT,
                name         TEXT,
                modid        TEXT,
                description  TEXT,
                loaders_json TEXT,
                mc_versions_json TEXT,
                project_id   TEXT,
                version_id   TEXT,
                enabled      INTEGER DEFAULT 1,
                installed_at INTEGER
            );
            CREATE INDEX idx_installed_mods_instance ON installed_mods(instance_id);

            CREATE TABLE download_tasks (
                id             TEXT PRIMARY KEY,
                kind           TEXT NOT NULL,
                title          TEXT NOT NULL,
                instance_id    TEXT,
                url            TEXT NOT NULL,
                target_path    TEXT NOT NULL,
                total_bytes    INTEGER,
                done_bytes     INTEGER,
                state          TEXT NOT NULL,
                checksum_sha1  TEXT,
                error          TEXT,
                created_at     INTEGER,
                updated_at     INTEGER
            );
            CREATE INDEX idx_download_tasks_state ON download_tasks(state);

            CREATE TABLE java_runtimes (
                id      TEXT PRIMARY KEY,
                path    TEXT NOT NULL,
                major   INTEGER NOT NULL,
                vendor  TEXT,
                version TEXT,
                arch    TEXT,
                source  TEXT NOT NULL,
                added_at INTEGER
            );

            CREATE TABLE version_cache (
                id            TEXT PRIMARY KEY,
                version_type  TEXT,
                release_time  TEXT,
                url           TEXT,
                manifest_json TEXT,
                fetched_at    INTEGER
            );

            CREATE TABLE news_cache (
                id        TEXT PRIMARY KEY,
                title     TEXT NOT NULL,
                url       TEXT NOT NULL,
                published TEXT,
                summary   TEXT,
                fetched_at INTEGER
            );
            "#,
        )
    }
}
