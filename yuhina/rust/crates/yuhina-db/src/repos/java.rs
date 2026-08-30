//! `java_runtimes` table repository (api-contract.md §5).

use std::sync::{Arc, Mutex};

use rusqlite::Connection;

use yuhina_api::{JavaRuntime, JavaSource, YuhinaError};

use crate::repos::now_ms;

#[derive(Debug, Clone)]
pub struct JavaRepo {
    pub(crate) conn: Arc<Mutex<Connection>>,
}

impl JavaRepo {
    pub fn insert(&self, j: &JavaRuntime) -> anyhow::Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            r#"
            INSERT OR REPLACE INTO java_runtimes
                (id, path, major, vendor, version, arch, source, added_at)
            VALUES (?1,?2,?3,?4,?5,?6,?7,?8)
            "#,
            rusqlite::params![
                j.id,
                j.path,
                j.major as i64,
                j.vendor,
                j.version,
                j.arch,
                java_source_str(j.source),
                now_ms(),
            ],
        )?;
        Ok(())
    }

    pub fn get(&self, id: &str) -> anyhow::Result<Option<JavaRuntime>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare("SELECT * FROM java_runtimes WHERE id = ?1")?;
        let mut rows = stmt.query([id])?;
        if let Some(r) = rows.next()? {
            Ok(Some(java_from(r)?))
        } else {
            Ok(None)
        }
    }

    pub fn get_by_path(&self, path: &str) -> anyhow::Result<Option<JavaRuntime>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare("SELECT * FROM java_runtimes WHERE path = ?1 LIMIT 1")?;
        let mut rows = stmt.query([path])?;
        if let Some(r) = rows.next()? {
            Ok(Some(java_from(r)?))
        } else {
            Ok(None)
        }
    }

    pub fn list(&self) -> anyhow::Result<Vec<JavaRuntime>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare("SELECT * FROM java_runtimes ORDER BY added_at ASC")?;
        let rows = stmt.query_map([], java_from)?;
        let mut out = Vec::new();
        for row in rows {
            out.push(row?);
        }
        Ok(out)
    }

    pub fn delete(&self, id: &str) -> anyhow::Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute("DELETE FROM java_runtimes WHERE id = ?1", [id])?;
        Ok(())
    }
}

fn java_source_str(s: JavaSource) -> &'static str {
    match s {
        JavaSource::Bundled => "bundled",
        JavaSource::System => "system",
        JavaSource::Manual => "manual",
    }
}

fn java_from(r: &rusqlite::Row<'_>) -> rusqlite::Result<JavaRuntime> {
    let source: String = r.get(6)?;
    let source = match source.as_str() {
        "bundled" => JavaSource::Bundled,
        "system" => JavaSource::System,
        _ => JavaSource::Manual,
    };
    Ok(JavaRuntime {
        id: r.get(0)?,
        path: r.get(1)?,
        major: r.get::<_, i64>(2)? as u32,
        vendor: r.get(3)?,
        version: r.get(4)?,
        arch: r.get(5)?,
        source,
    })
}

pub fn internal(e: impl std::fmt::Display) -> YuhinaError {
    YuhinaError::internal(e.to_string())
}
