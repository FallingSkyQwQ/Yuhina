//! `instances` table repository (api-contract.md §5) — read side used by
//! Agent A's `instance_detail`. Full CRUD is owned by Agent C.

use std::sync::{Arc, Mutex};

use rusqlite::Connection;

use yuhina_api::{InstanceDetail, InstanceSummary, JavaSelection, LaunchArgs, Loader, LoaderKind};

#[derive(Debug, Clone)]
pub struct InstanceRepo {
    pub(crate) conn: Arc<Mutex<Connection>>,
}

impl InstanceRepo {
    pub fn get(&self, id: &str) -> anyhow::Result<Option<InstanceSummary>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(SELECT_INSTANCE)?;
        let mut rows = stmt.query([id])?;
        if let Some(r) = rows.next()? {
            Ok(Some(summary_from(&conn, r)?))
        } else {
            Ok(None)
        }
    }

    pub fn get_detail(&self, id: &str) -> anyhow::Result<Option<InstanceDetail>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(SELECT_INSTANCE)?;
        let mut rows = stmt.query([id])?;
        if let Some(r) = rows.next()? {
            let summary = summary_from(&conn, r)?;
            let java: JavaSelection = match (r.get::<_, Option<i64>>(11)?, r.get::<_, Option<String>>(12)?) {
                (Some(major), _) => JavaSelection::Auto(major as u32),
                (None, Some(path)) => JavaSelection::Manual(path),
                _ => JavaSelection::Auto(21),
            };
            let launch_args: Option<LaunchArgs> = match r.get::<_, Option<String>>(13)? {
                Some(json) => serde_json::from_str(&json).ok(),
                None => None,
            };
            Ok(Some(InstanceDetail {
                summary,
                game_dir: r.get(10)?,
                java,
                launch_args,
                notes: r.get(14)?,
            }))
        } else {
            Ok(None)
        }
    }

    pub fn list(&self) -> anyhow::Result<Vec<InstanceSummary>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(&format!("{SELECT_INSTANCE} ORDER BY updated_at DESC"))?;
        let rows = stmt.query_map([], |r| summary_from(&conn, r))?;
        let mut out = Vec::new();
        for row in rows {
            out.push(row?);
        }
        Ok(out)
    }

    /// Patch an instance row. `None` = leave unchanged; `Some(None)` clears
    /// the nullable column; `Some(Some(v))` sets it.
    #[allow(clippy::too_many_arguments)]
    pub fn update(
        &self,
        id: &str,
        name: Option<String>,
        icon: Option<String>,
        loader: Option<Option<&Loader>>,
        java_auto_major: Option<u32>,
        java_manual_path: Option<String>,
        launch_args: Option<Option<LaunchArgs>>,
        is_installed: Option<bool>,
        last_launched_at: Option<u64>,
    ) -> anyhow::Result<()> {
        let conn = self.conn.lock().unwrap();
        let mut sets: Vec<String> = Vec::new();
        let mut params: Vec<rusqlite::types::Value> = Vec::new();
        let mut sql = UpdateBuilder { sets: &mut sets, params: &mut params };

        if let Some(v) = name {
            sql.push("name", v.into());
        }
        if let Some(v) = icon {
            sql.push("icon", v.into());
        }
        match loader {
            Some(Some(l)) => {
                sql.push("loader_kind", loader_kind_str(l.kind).to_string().into());
                sql.push("loader_version", l.version.clone().into());
            }
            Some(None) => {
                sql.push_null("loader_kind");
                sql.push_null("loader_version");
            }
            None => {}
        }
        if let Some(v) = java_auto_major {
            sql.push("java_auto_major", (v as i64).into());
        }
        if let Some(v) = java_manual_path {
            sql.push("java_manual_path", v.into());
        }
        match launch_args {
            Some(Some(args)) => {
                let json = serde_json::to_string(&args)?;
                sql.push("launch_args_json", json.into());
            }
            Some(None) => sql.push_null("launch_args_json"),
            None => {}
        }
        if let Some(v) = is_installed {
            sql.push("is_installed", (v as i64).into());
        }
        if let Some(v) = last_launched_at {
            sql.push("last_launched_at", (v as i64).into());
        }
        sql.push("updated_at", crate::repos::now_ms().into());

        if sql.sets.is_empty() {
            return Ok(());
        }
        let params = std::mem::take(sql.params);
        let assign = sql
            .sets
            .iter()
            .zip(1..)
            .map(|(c, i)| format!("{c} = ?{i}"))
            .collect::<Vec<_>>()
            .join(", ");
        let mut params = params;
        params.push(id.to_string().into());
        let sql_text = format!(
            "UPDATE instances SET {assign} WHERE id = ?{}",
            params.len()
        );
        conn.execute(&sql_text, rusqlite::params_from_iter(params))?;
        Ok(())
    }
}

struct UpdateBuilder<'a> {
    sets: &'a mut Vec<String>,
    params: &'a mut Vec<rusqlite::types::Value>,
}

impl UpdateBuilder<'_> {
    fn push(&mut self, col: &str, value: rusqlite::types::Value) {
        self.sets.push(col.to_string());
        self.params.push(value);
    }

    fn push_null(&mut self, col: &str) {
        self.sets.push(col.to_string());
        self.params.push(rusqlite::types::Value::Null);
    }
}

fn loader_kind_str(kind: LoaderKind) -> &'static str {
    match kind {
        LoaderKind::Forge => "forge",
        LoaderKind::Fabric => "fabric",
        LoaderKind::NeoForge => "neoforge",
        LoaderKind::Quilt => "quilt",
    }
}

const SELECT_INSTANCE: &str = r#"
    SELECT id, name, icon, mc_version, loader_kind, loader_version,
           is_installed, last_launched_at, created_at, updated_at,
           game_dir, java_auto_major, java_manual_path, launch_args_json, notes
    FROM instances
"#;

/// Build a summary row; column order follows SELECT_INSTANCE.
fn summary_from(conn: &Connection, r: &rusqlite::Row<'_>) -> rusqlite::Result<InstanceSummary> {
    let loader_kind: Option<String> = r.get(4)?;
    let loader_version: Option<String> = r.get(5)?;
    let loader = match (loader_kind, loader_version) {
        (Some(kind), Some(version)) => {
            let kind = match kind.as_str() {
                "forge" => LoaderKind::Forge,
                "fabric" => LoaderKind::Fabric,
                "neoforge" => LoaderKind::NeoForge,
                "quilt" => LoaderKind::Quilt,
                other => {
                    return Err(rusqlite::Error::FromSqlConversionFailure(
                        4,
                        rusqlite::types::Type::Text,
                        format!("unknown loader kind {other}").into(),
                    ))
                }
            };
            Some(Loader { kind, version })
        }
        _ => None,
    };
    let (mod_count, total_size_bytes) = mod_stats(conn, &r.get::<_, String>(0)?);
    Ok(InstanceSummary {
        id: r.get(0)?,
        name: r.get(1)?,
        icon: r.get(2)?,
        mc_version: r.get(3)?,
        loader,
        is_installed: r.get::<_, i64>(6)? != 0,
        last_launched_at: r.get::<_, Option<i64>>(7)?.map(|v| v as u64),
        mod_count,
        total_size_bytes,
        created_at: r.get::<_, i64>(8)? as u64,
        updated_at: r.get::<_, i64>(9)? as u64,
    })
}

fn mod_stats(conn: &Connection, instance_id: &str) -> (u32, u64) {
    let mut stmt = match conn.prepare(
        "SELECT COUNT(*), COALESCE(SUM(file_size), 0) FROM installed_mods WHERE instance_id = ?1",
    ) {
        Ok(s) => s,
        Err(_) => return (0, 0),
    };
    stmt.query_row([instance_id], |r| {
        Ok((r.get::<_, i64>(0)? as u32, r.get::<_, i64>(1)? as u64))
    })
    .unwrap_or((0, 0))
}