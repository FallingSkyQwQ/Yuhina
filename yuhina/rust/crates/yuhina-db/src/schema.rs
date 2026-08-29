//! SQLite schema DDL — matches `docs/api-contract.md` §5 exactly.

/// Migration 001: initial schema (all tables + indexes).
pub const MIGRATION_001: &str = r#"
CREATE TABLE settings (
  key TEXT PRIMARY KEY,
  value TEXT NOT NULL
);

CREATE TABLE accounts (
  id TEXT PRIMARY KEY,
  kind TEXT NOT NULL,
  username TEXT NOT NULL,
  uuid TEXT NOT NULL,
  yggdrasil_server TEXT,
  skin_url TEXT,
  access_token_enc TEXT,
  refresh_token_enc TEXT,
  expires_at INTEGER,
  is_active INTEGER DEFAULT 0,
  created_at INTEGER,
  updated_at INTEGER
);

CREATE TABLE instances (
  id TEXT PRIMARY KEY,
  name TEXT NOT NULL,
  icon TEXT NOT NULL DEFAULT '🎮',
  mc_version TEXT NOT NULL,
  loader_kind TEXT,
  loader_version TEXT,
  game_dir TEXT NOT NULL,
  java_auto_major INTEGER,
  java_manual_path TEXT,
  launch_args_json TEXT,
  notes TEXT DEFAULT '',
  is_installed INTEGER DEFAULT 0,
  last_launched_at INTEGER,
  created_at INTEGER,
  updated_at INTEGER
);

CREATE TABLE installed_mods (
  id TEXT PRIMARY KEY,
  instance_id TEXT NOT NULL REFERENCES instances(id) ON DELETE CASCADE,
  file_name TEXT NOT NULL,
  file_size INTEGER,
  sha1 TEXT,
  name TEXT,
  modid TEXT,
  description TEXT,
  loaders_json TEXT,
  mc_versions_json TEXT,
  project_id TEXT,
  version_id TEXT,
  enabled INTEGER DEFAULT 1,
  installed_at INTEGER
);

CREATE TABLE download_tasks (
  id TEXT PRIMARY KEY,
  kind TEXT NOT NULL,
  title TEXT NOT NULL,
  instance_id TEXT,
  url TEXT NOT NULL,
  target_path TEXT NOT NULL,
  total_bytes INTEGER,
  done_bytes INTEGER,
  state TEXT NOT NULL,
  checksum_sha1 TEXT,
  error TEXT,
  created_at INTEGER,
  updated_at INTEGER
);

CREATE TABLE java_runtimes (
  id TEXT PRIMARY KEY,
  path TEXT NOT NULL,
  major INTEGER NOT NULL,
  vendor TEXT,
  version TEXT,
  arch TEXT,
  source TEXT NOT NULL,
  added_at INTEGER
);

CREATE TABLE version_cache (
  id TEXT PRIMARY KEY,
  version_type TEXT,
  release_time TEXT,
  url TEXT,
  manifest_json TEXT,
  fetched_at INTEGER
);

CREATE TABLE news_cache (
  id TEXT PRIMARY KEY,
  title TEXT NOT NULL,
  url TEXT NOT NULL,
  published TEXT,
  summary TEXT,
  fetched_at INTEGER
);

CREATE INDEX idx_installed_mods_instance ON installed_mods(instance_id);
CREATE INDEX idx_download_tasks_state ON download_tasks(state);
"#;

/// Ordered list of migrations. Index 0 applies at user_version 1, etc.
pub const MIGRATIONS: &[&str] = &[MIGRATION_001];

/// Expected schema version (highest applied user_version).
pub const SCHEMA_VERSION: i64 = MIGRATIONS.len() as i64;

/// Applies pending migrations in order and bumps `PRAGMA user_version`.
/// Idempotent: calling repeatedly is a no-op once up to date.
pub fn migrate(conn: &rusqlite::Connection) -> anyhow::Result<()> {
    let current: i64 = conn.query_row("PRAGMA user_version", [], |r| r.get(0))?;
    for (i, sql) in MIGRATIONS.iter().enumerate() {
        let target = (i + 1) as i64;
        if target > current {
            let tx = conn.unchecked_transaction()?;
            tx.execute_batch(sql)?;
            tx.pragma_update(None, "user_version", target)?;
            tx.commit()?;
            tracing::info!(target, "applied db migration");
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn schema_version_matches_migration_count() {
        assert_eq!(SCHEMA_VERSION, 1);
        assert_eq!(MIGRATIONS.len(), SCHEMA_VERSION as usize);
    }
}