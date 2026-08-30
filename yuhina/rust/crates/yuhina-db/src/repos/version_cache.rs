//! `version_cache` table repository (api-contract.md §5).
//!
//! Latest flags are not stored in the schema; they are reconstructed by
//! parsing the manifest JSON cached on the rows (`manifest_json` column),
//! which `fetch_version_list` persists alongside each `VersionMeta`.

use std::sync::{Arc, Mutex};

use rusqlite::Connection;

use yuhina_api::VersionMeta;

use crate::repos::now_ms;

/// A cached version row including the raw manifest JSON.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct VersionCacheEntry {
    pub meta: VersionMeta,
    pub manifest_json: Option<String>,
    pub fetched_at: i64,
}

#[derive(Debug, Clone)]
pub struct VersionCacheRepo {
    pub(crate) conn: Arc<Mutex<Connection>>,
}

impl VersionCacheRepo {
    /// Upsert a cached version entry; `manifest_json` is the raw manifest body
    /// (stored once so `list` can restore `is_latest_*` flags).
    pub fn upsert(&self, m: &VersionMeta, manifest_json: Option<&String>) -> anyhow::Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            r#"
            INSERT INTO version_cache (id, version_type, release_time, url, manifest_json, fetched_at)
            VALUES (?1,?2,?3,?4,?5,?6)
            ON CONFLICT(id) DO UPDATE SET
                version_type = excluded.version_type,
                release_time = excluded.release_time,
                url = excluded.url,
                fetched_at = excluded.fetched_at
            "#,
            rusqlite::params![
                m.id,
                m.version_type,
                m.release_time,
                m.url,
                manifest_json,
                now_ms(),
            ],
        )?;
        Ok(())
    }

    pub fn get(&self, id: &str) -> anyhow::Result<Option<VersionCacheEntry>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare("SELECT * FROM version_cache WHERE id = ?1")?;
        let mut rows = stmt.query([id])?;
        if let Some(r) = rows.next()? {
            Ok(Some(entry_from(r)?))
        } else {
            Ok(None)
        }
    }

    pub fn list(&self) -> anyhow::Result<Vec<VersionMeta>> {
        let conn = self.conn.lock().unwrap();
        let latest = latest_ids(&conn)?;
        let mut stmt = conn.prepare("SELECT * FROM version_cache ORDER BY release_time DESC")?;
        let rows = stmt.query_map([], meta_from)?;
        let mut out = Vec::new();
        for row in rows {
            out.push(with_latest(row?, &latest));
        }
        Ok(out)
    }
}

fn latest_ids(conn: &Connection) -> anyhow::Result<(Option<String>, Option<String>)> {
    let mut latest = (None, None);
    let mut stmt = conn.prepare(
        "SELECT manifest_json FROM version_cache WHERE manifest_json IS NOT NULL LIMIT 1",
    )?;
    let mut rows = stmt.query([])?;
    if let Some(r) = rows.next()? {
        let raw: String = r.get(0)?;
        if let Ok(v) = serde_json::from_str::<serde_json::Value>(&raw) {
            latest.0 = v
                .get("latest")
                .and_then(|l| l.get("release"))
                .and_then(|x| x.as_str())
                .map(|s| s.to_string());
            latest.1 = v
                .get("latest")
                .and_then(|l| l.get("snapshot"))
                .and_then(|x| x.as_str())
                .map(|s| s.to_string());
        }
    }
    Ok(latest)
}

fn with_latest(mut m: VersionMeta, latest: &(Option<String>, Option<String>)) -> VersionMeta {
    m.is_latest_release = latest.0.as_deref() == Some(m.id.as_str());
    m.is_latest_snapshot = latest.1.as_deref() == Some(m.id.as_str());
    m
}

fn meta_from(r: &rusqlite::Row<'_>) -> rusqlite::Result<VersionMeta> {
    Ok(VersionMeta {
        id: r.get(0)?,
        version_type: r.get(1)?,
        release_time: r.get(2)?,
        url: r.get(3)?,
        is_latest_release: false,
        is_latest_snapshot: false,
    })
}

fn entry_from(r: &rusqlite::Row<'_>) -> rusqlite::Result<VersionCacheEntry> {
    Ok(VersionCacheEntry {
        meta: VersionMeta {
            id: r.get(0)?,
            version_type: r.get(1)?,
            release_time: r.get(2)?,
            url: r.get(3)?,
            is_latest_release: false,
            is_latest_snapshot: false,
        },
        manifest_json: r.get(4)?,
        fetched_at: r.get(5)?,
    })
}
