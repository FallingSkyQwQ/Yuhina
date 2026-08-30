//! `installed_mods` table repository.
//!
//! Owned by Agent C. The public `InstalledMod.id` (contract §2.5) is the
//! per-instance file hash (sha1). The table's primary key `id` is a UUID so
//! the *same* mod file (identical sha1) can be installed in several instances
//! without a global key collision; every read maps the row back to the
//! contract shape where `InstalledMod.id == sha1`.

use std::sync::{Arc, Mutex};

use rusqlite::Connection;

use yuhina_api::InstalledMod;

#[derive(Debug, Clone)]
pub struct InstalledModRepo {
    pub(crate) conn: Arc<Mutex<Connection>>,
}

impl InstalledModRepo {
    /// Insert a mod row for `instance_id`. `m.id` (sha1) is stored in the
    /// `sha1` column; a fresh UUID is used as the table primary key.
    pub fn insert(&self, instance_id: &str, m: &InstalledMod) -> anyhow::Result<()> {
        let conn = self.conn.lock().unwrap();
        let row_id = uuid::Uuid::new_v4().to_string();
        conn.execute(
            "INSERT INTO installed_mods (
                id, instance_id, file_name, file_size, sha1, name, modid, description,
                loaders_json, mc_versions_json, project_id, version_id, enabled, installed_at
             ) VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9,?10,?11,?12,?13,?14)",
            rusqlite::params![
                row_id,
                instance_id,
                m.file_name,
                m.file_size as i64,
                m.sha1,
                m.name,
                m.modid,
                m.description,
                serde_json::to_string(&m.loaders).unwrap_or_else(|_| "[]".into()),
                serde_json::to_string(&m.mc_versions).unwrap_or_else(|_| "[]".into()),
                m.project_id,
                m.version_id,
                m.enabled as i64,
                m.installed_at as i64,
            ],
        )?;
        Ok(())
    }

    /// Look up a mod by its file-hash id within an instance.
    pub fn get(&self, instance_id: &str, mod_id: &str) -> anyhow::Result<Option<InstalledMod>> {
        self.get_by_sha1(instance_id, mod_id)
    }

    pub fn get_by_sha1(
        &self,
        instance_id: &str,
        sha1: &str,
    ) -> anyhow::Result<Option<InstalledMod>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT file_name, file_size, sha1, name, modid, description,
                    loaders_json, mc_versions_json, project_id, version_id,
                    enabled, installed_at
             FROM installed_mods WHERE instance_id = ?1 AND sha1 = ?2",
        )?;
        let mut rows = stmt.query([instance_id, sha1])?;
        if let Some(r) = rows.next()? {
            Ok(Some(mod_from_row(r)?))
        } else {
            Ok(None)
        }
    }

    /// List all mods of an instance (installed order).
    pub fn list(&self, instance_id: &str) -> anyhow::Result<Vec<InstalledMod>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT file_name, file_size, sha1, name, modid, description,
                    loaders_json, mc_versions_json, project_id, version_id,
                    enabled, installed_at
             FROM installed_mods WHERE instance_id = ?1 ORDER BY installed_at",
        )?;
        let rows = stmt.query_map([instance_id], mod_from_row)?;
        let mut out = Vec::new();
        for row in rows {
            out.push(row?);
        }
        Ok(out)
    }

    /// Delete a mod row by file-hash id within an instance.
    pub fn delete(&self, instance_id: &str, mod_id: &str) -> anyhow::Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "DELETE FROM installed_mods WHERE instance_id = ?1 AND sha1 = ?2",
            [instance_id, mod_id],
        )?;
        Ok(())
    }

    pub fn set_enabled(
        &self,
        instance_id: &str,
        mod_id: &str,
        enabled: bool,
    ) -> anyhow::Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "UPDATE installed_mods SET enabled = ?1 WHERE instance_id = ?2 AND sha1 = ?3",
            rusqlite::params![enabled as i64, instance_id, mod_id],
        )?;
        Ok(())
    }

    /// Attach Modrinth linkage (project/version ids) to an installed mod.
    pub fn set_modrinth(
        &self,
        instance_id: &str,
        mod_id: &str,
        project_id: Option<&str>,
        version_id: Option<&str>,
    ) -> anyhow::Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "UPDATE installed_mods SET project_id = ?1, version_id = ?2
             WHERE instance_id = ?3 AND sha1 = ?4",
            rusqlite::params![project_id, version_id, instance_id, mod_id],
        )?;
        Ok(())
    }
}

fn mod_from_row(r: &rusqlite::Row<'_>) -> rusqlite::Result<InstalledMod> {
    let loaders: Vec<String> = serde_json::from_str(
        &r.get::<_, Option<String>>(6)?
            .unwrap_or_else(|| "[]".into()),
    )
    .unwrap_or_default();
    let mc_versions: Vec<String> = serde_json::from_str(
        &r.get::<_, Option<String>>(7)?
            .unwrap_or_else(|| "[]".into()),
    )
    .unwrap_or_default();
    Ok(InstalledMod {
        // Contract: public id == file hash, unique within the instance.
        id: r.get(2)?,
        file_name: r.get(0)?,
        file_size: r.get::<_, Option<i64>>(1)?.unwrap_or(0) as u64,
        sha1: r.get(2)?,
        name: r.get(3)?,
        modid: r.get(4)?,
        description: r.get(5)?,
        loaders,
        mc_versions,
        project_id: r.get(8)?,
        version_id: r.get(9)?,
        enabled: r.get::<_, i64>(10)? != 0,
        installed_at: r.get::<_, Option<i64>>(11)?.unwrap_or(0) as u64,
    })
}
