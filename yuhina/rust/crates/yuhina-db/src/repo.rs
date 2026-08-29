//! Repository layer over the SQLite connection.
//!
//! Each repo owns a borrowed `&Connection` and exposes typed CRUD.
//! Row-level structs carry persistence-only fields (e.g. encrypted tokens)
//! that are not part of the public FFI contract.

use anyhow::{Result, anyhow};
use rusqlite::{Connection, OptionalExtension, params};
use yuhina_api::{
    Account, AccountKind, DownloadState, DownloadTask, InstanceDetail, InstanceSummary,
    InstalledMod, JavaRuntime, JavaSelection, JavaSource, LaunchArgs, Loader, LoaderKind, NewsItem,
    VersionMeta,
};

use crate::now_millis;

// ---------------------------------------------------------------------------
// settings
// ---------------------------------------------------------------------------

/// Key-value settings table accessor.
pub struct SettingsRepo<'a> {
    conn: &'a Connection,
}

impl<'a> SettingsRepo<'a> {
    pub fn new(conn: &'a Connection) -> Self {
        Self { conn }
    }

    pub fn set(&self, key: &str, value: &str) -> Result<()> {
        self.conn.execute(
            "INSERT INTO settings(key, value) VALUES(?1, ?2)
             ON CONFLICT(key) DO UPDATE SET value = excluded.value",
            params![key, value],
        )?;
        Ok(())
    }

    pub fn get(&self, key: &str) -> Result<Option<String>> {
        self.conn
            .query_row("SELECT value FROM settings WHERE key = ?1", [key], |r| {
                r.get(0)
            })
            .optional()
            .map_err(anyhow::Error::from)
    }

    pub fn get_u64(&self, key: &str) -> Result<Option<u64>> {
        Ok(self
            .get(key)?
            .and_then(|v| v.parse::<u64>().ok()))
    }
}

// ---------------------------------------------------------------------------
// accounts
// ---------------------------------------------------------------------------

/// Full account row including encrypted tokens (never exposed via FFI).
#[derive(Debug, Clone)]
pub struct AccountRow {
    pub account: Account,
    pub access_token_enc: Option<String>,
    pub refresh_token_enc: Option<String>,
}

impl AccountRow {
    pub fn new(account: Account) -> Self {
        Self {
            account,
            access_token_enc: None,
            refresh_token_enc: None,
        }
    }

    pub fn with_tokens(mut self, at: Option<String>, rt: Option<String>) -> Self {
        self.access_token_enc = at;
        self.refresh_token_enc = rt;
        self
    }
}

pub struct AccountRepo<'a> {
    conn: &'a Connection,
}

impl<'a> AccountRepo<'a> {
    pub fn new(conn: &'a Connection) -> Self {
        Self { conn }
    }

    fn from_row(r: &rusqlite::Row<'_>) -> rusqlite::Result<AccountRow> {
        let kind: String = r.get("kind")?;
        let is_active: i64 = r.get("is_active")?;
        let account = Account {
            id: r.get("id")?,
            kind: parse_account_kind(&kind),
            username: r.get("username")?,
            uuid: r.get("uuid")?,
            yggdrasil_server: r.get("yggdrasil_server")?,
            skin_url: r.get("skin_url")?,
            is_active: is_active != 0,
            expires_at: r.get::<_, Option<i64>>("expires_at")?.map(|v| v as u64),
        };
        Ok(AccountRow {
            account,
            access_token_enc: r.get("access_token_enc")?,
            refresh_token_enc: r.get("refresh_token_enc")?,
        })
    }

    /// Insert or update (upsert by id).
    pub fn upsert(&self, row: &AccountRow) -> Result<()> {
        let a = &row.account;
        self.conn.execute(
            "INSERT INTO accounts(id, kind, username, uuid, yggdrasil_server, skin_url,
                 access_token_enc, refresh_token_enc, expires_at, is_active, created_at, updated_at)
             VALUES(?1,?2,?3,?4,?5,?6,?7,?8,?9,?10,?11,?12)
             ON CONFLICT(id) DO UPDATE SET
               kind=excluded.kind, username=excluded.username, uuid=excluded.uuid,
               yggdrasil_server=excluded.yggdrasil_server, skin_url=excluded.skin_url,
               access_token_enc=excluded.access_token_enc,
               refresh_token_enc=excluded.refresh_token_enc,
               expires_at=excluded.expires_at, is_active=excluded.is_active,
               updated_at=excluded.updated_at",
            params![
                a.id,
                account_kind_str(&a.kind),
                a.username,
                a.uuid,
                a.yggdrasil_server,
                a.skin_url,
                row.access_token_enc,
                row.refresh_token_enc,
                a.expires_at.map(|v| v as i64),
                a.is_active as i64,
                now_millis(),
                now_millis(),
            ],
        )?;
        Ok(())
    }

    pub fn list(&self) -> Result<Vec<Account>> {
        let mut stmt =
            self.conn
                .prepare("SELECT * FROM accounts ORDER BY created_at ASC")?;
        let rows = stmt
            .query_map([], Self::from_row)?
            .collect::<rusqlite::Result<Vec<_>>>()?;
        Ok(rows.into_iter().map(|r| r.account).collect())
    }

    pub fn get(&self, id: &str) -> Result<Option<Account>> {
        Ok(self.get_row(id)?.map(|r| r.account))
    }

    pub fn get_row(&self, id: &str) -> Result<Option<AccountRow>> {
        self.conn
            .query_row("SELECT * FROM accounts WHERE id = ?1", [id], Self::from_row)
            .optional()
            .map_err(anyhow::Error::from)
    }

    /// Deactivate every account then activate `id`.
    pub fn set_active(&self, id: &str) -> Result<()> {
        let tx = self.conn.unchecked_transaction()?;
        tx.execute("UPDATE accounts SET is_active = 0", [])?;
        let n = tx.execute(
            "UPDATE accounts SET is_active = 1, updated_at = ?1 WHERE id = ?2",
            params![now_millis(), id],
        )?;
        if n == 0 {
            return Err(anyhow!("account '{id}' not found"));
        }
        tx.commit()?;
        Ok(())
    }

    pub fn get_active(&self) -> Result<Option<Account>> {
        self.conn
            .query_row(
                "SELECT * FROM accounts WHERE is_active = 1 LIMIT 1",
                [],
                Self::from_row,
            )
            .optional()
            .map(|o| o.map(|r| r.account))
            .map_err(anyhow::Error::from)
    }

    pub fn delete(&self, id: &str) -> Result<()> {
        self.conn
            .execute("DELETE FROM accounts WHERE id = ?1", [id])?;
        Ok(())
    }

    pub fn set_tokens(&self, id: &str, at_enc: Option<&str>, rt_enc: Option<&str>) -> Result<()> {
        self.conn.execute(
            "UPDATE accounts SET access_token_enc = ?1, refresh_token_enc = ?2, updated_at = ?3
             WHERE id = ?4",
            params![at_enc, rt_enc, now_millis(), id],
        )?;
        Ok(())
    }
}

fn parse_account_kind(s: &str) -> AccountKind {
    match s {
        "microsoft" => AccountKind::Microsoft,
        "yggdrasil" => AccountKind::Yggdrasil,
        _ => AccountKind::Offline,
    }
}

fn account_kind_str(k: &AccountKind) -> &'static str {
    match k {
        AccountKind::Microsoft => "microsoft",
        AccountKind::Yggdrasil => "yggdrasil",
        AccountKind::Offline => "offline",
    }
}

// ---------------------------------------------------------------------------
// instances
// ---------------------------------------------------------------------------

pub struct InstanceRepo<'a> {
    conn: &'a Connection,
}

impl<'a> InstanceRepo<'a> {
    pub fn new(conn: &'a Connection) -> Self {
        Self { conn }
    }

    const SUMMARY_SQL: &'static str = r#"
        SELECT i.*,
          (SELECT COUNT(*) FROM installed_mods m WHERE m.instance_id = i.id) AS mod_count,
          COALESCE((SELECT SUM(m.file_size) FROM installed_mods m WHERE m.instance_id = i.id), 0) AS total_size
        FROM instances i
    "#;

    fn summary_from_row(r: &rusqlite::Row<'_>) -> rusqlite::Result<InstanceSummary> {
        let loader = match (r.get::<_, Option<String>>("loader_kind")?, r.get::<_, Option<String>>("loader_version")?) {
            (Some(kind), Some(version)) => Some(Loader {
                kind: parse_loader_kind(&kind),
                version,
            }),
            _ => None,
        };
        let is_installed: i64 = r.get("is_installed")?;
        Ok(InstanceSummary {
            id: r.get("id")?,
            name: r.get("name")?,
            icon: r.get("icon")?,
            mc_version: r.get("mc_version")?,
            loader,
            is_installed: is_installed != 0,
            last_launched_at: r
                .get::<_, Option<i64>>("last_launched_at")?
                .map(|v| v as u64),
            mod_count: r.get::<_, i64>("mod_count")? as u32,
            total_size_bytes: r.get::<_, i64>("total_size")? as u64,
            created_at: r.get::<_, i64>("created_at")? as u64,
            updated_at: r.get::<_, i64>("updated_at")? as u64,
        })
    }

    pub fn insert(&self, s: &InstanceSummary, game_dir: &str, notes: &str) -> Result<()> {
        self.conn.execute(
            "INSERT INTO instances(id, name, icon, mc_version, loader_kind, loader_version,
                 game_dir, java_auto_major, java_manual_path, launch_args_json, notes,
                 is_installed, last_launched_at, created_at, updated_at)
             VALUES(?1,?2,?3,?4,?5,?6,?7,NULL,NULL,NULL,?8,?9,?10,?11,?12)",
            params![
                s.id,
                s.name,
                s.icon,
                s.mc_version,
                s.loader.as_ref().map(|l| loader_kind_str(&l.kind).to_string()),
                s.loader.as_ref().map(|l| l.version.clone()),
                game_dir,
                notes,
                s.is_installed as i64,
                s.last_launched_at.map(|v| v as i64),
                now_millis(),
                now_millis(),
            ],
        )?;
        Ok(())
    }

    pub fn list(&self) -> Result<Vec<InstanceSummary>> {
        let mut stmt = self.conn.prepare(Self::SUMMARY_SQL)?;
        let rows = stmt
            .query_map([], Self::summary_from_row)?
            .collect::<rusqlite::Result<Vec<_>>>()?;
        Ok(rows)
    }

    pub fn get(&self, id: &str) -> Result<Option<InstanceSummary>> {
        let mut stmt = self.conn.prepare(&format!("{} WHERE i.id = ?1", Self::SUMMARY_SQL))?;
        let mut rows = stmt.query_map([id], Self::summary_from_row)?;
        match rows.next() {
            Some(row) => Ok(Some(row?)),
            None => Ok(None),
        }
    }

    pub fn get_detail(&self, id: &str) -> Result<Option<InstanceDetail>> {
        let summary = match self.get(id)? {
            Some(s) => s,
            None => return Ok(None),
        };
        let mut stmt = self.conn.prepare("SELECT * FROM instances WHERE id = ?1")?;
        let mut rows = stmt.query([id])?;
        let row = match rows.next()? {
            Some(r) => r,
            None => return Ok(None),
        };
        let game_dir: String = row.get("game_dir")?;
        let notes: String = row.get::<_, Option<String>>("notes")?.unwrap_or_default();
        let java = match (
            row.get::<_, Option<i64>>("java_auto_major")?,
            row.get::<_, Option<String>>("java_manual_path")?,
        ) {
            (Some(major), _) => JavaSelection::Auto(major as u32),
            (None, Some(path)) => JavaSelection::Manual(path),
            _ => JavaSelection::Auto(0),
        };
        let launch_args = row
            .get::<_, Option<String>>("launch_args_json")?
            .and_then(|j| serde_json::from_str::<LaunchArgs>(&j).ok());
        Ok(Some(InstanceDetail {
            summary,
            game_dir,
            java,
            launch_args,
            notes,
        }))
    }

    /// Update mutable fields. `None` means "leave unchanged" except for
    /// `loader`/`java` where `Some(None)` clears them.
    #[allow(clippy::too_many_arguments)]
    pub fn update(
        &self,
        id: &str,
        name: Option<&str>,
        icon: Option<&str>,
        loader: Option<Option<&Loader>>,
        java: Option<&JavaSelection>,
        launch_args: Option<Option<&LaunchArgs>>,
        notes: Option<&str>,
        is_installed: Option<bool>,
        last_launched_at: Option<Option<u64>>,
    ) -> Result<()> {
        let mut parts: Vec<String> = vec![];
        let mut values: Vec<Box<dyn rusqlite::types::ToSql>> = Vec::new();
        let mut add = |col: &str, v: Box<dyn rusqlite::types::ToSql>| {
            parts.push(format!("{col} = ?{}", parts.len() + 1));
            values.push(v);
        };
        if let Some(n) = name {
            add("name", Box::new(n.to_string()));
        }
        if let Some(i) = icon {
            add("icon", Box::new(i.to_string()));
        }
        if let Some(l) = loader {
            match l {
                Some(Loader { kind, version }) => {
                    add("loader_kind", Box::new(loader_kind_str(kind).to_string()));
                    add("loader_version", Box::new(version.clone()));
                }
                None => {
                    add("loader_kind", Box::new(None::<String>));
                    add("loader_version", Box::new(None::<String>));
                }
            }
        }
        if let Some(j) = java {
            match j {
                JavaSelection::Auto(major) => {
                    add("java_auto_major", Box::new(Some(*major as i64)));
                    add("java_manual_path", Box::new(None::<String>));
                }
                JavaSelection::Manual(path) => {
                    add("java_auto_major", Box::new(None::<i64>));
                    add("java_manual_path", Box::new(Some(path.clone())));
                }
            }
        }
        if let Some(la) = launch_args {
            let json = la.map(|v| serde_json::to_string(v).unwrap_or_default());
            add("launch_args_json", Box::new(json));
        }
        if let Some(no) = notes {
            add("notes", Box::new(no.to_string()));
        }
        if let Some(inst) = is_installed {
            add("is_installed", Box::new(inst as i64));
        }
        if let Some(lla) = last_launched_at {
            add("last_launched_at", Box::new(lla.map(|v| v as i64)));
        }
        add("updated_at", Box::new(now_millis()));
        if parts.is_empty() {
            return Ok(());
        }
        let sql = format!("UPDATE instances SET {} WHERE id = ?{}", parts.join(", "), parts.len() + 1);
        let mut final_params = values;
        final_params.push(Box::new(id.to_string()));
        let mut stmt = self.conn.prepare(&sql)?;
        stmt.execute(rusqlite::params_from_iter(final_params.iter().map(|b| b.as_ref())))?;
        Ok(())
    }

    pub fn delete(&self, id: &str) -> Result<()> {
        self.conn
            .execute("DELETE FROM instances WHERE id = ?1", [id])?;
        Ok(())
    }
}

fn parse_loader_kind(s: &str) -> LoaderKind {
    match s {
        "forge" => LoaderKind::Forge,
        "fabric" => LoaderKind::Fabric,
        "neoforge" => LoaderKind::NeoForge,
        "quilt" => LoaderKind::Quilt,
        _ => LoaderKind::Fabric,
    }
}

fn loader_kind_str(k: &LoaderKind) -> &'static str {
    match k {
        LoaderKind::Forge => "forge",
        LoaderKind::Fabric => "fabric",
        LoaderKind::NeoForge => "neoforge",
        LoaderKind::Quilt => "quilt",
    }
}

// ---------------------------------------------------------------------------
// installed_mods
// ---------------------------------------------------------------------------

pub struct InstalledModRepo<'a> {
    conn: &'a Connection,
}

impl<'a> InstalledModRepo<'a> {
    pub fn new(conn: &'a Connection) -> Self {
        Self { conn }
    }

    fn from_row(r: &rusqlite::Row<'_>) -> rusqlite::Result<InstalledMod> {
        let enabled: i64 = r.get("enabled")?;
        Ok(InstalledMod {
            id: r.get("id")?,
            file_name: r.get("file_name")?,
            file_size: r.get::<_, i64>("file_size")? as u64,
            sha1: r.get("sha1")?,
            name: r.get::<_, Option<String>>("name")?.unwrap_or_default(),
            modid: r.get::<_, Option<String>>("modid")?.unwrap_or_default(),
            description: r
                .get::<_, Option<String>>("description")?
                .unwrap_or_default(),
            loaders: r
                .get::<_, Option<String>>("loaders_json")?
                .and_then(|j| serde_json::from_str(&j).ok())
                .unwrap_or_default(),
            mc_versions: r
                .get::<_, Option<String>>("mc_versions_json")?
                .and_then(|j| serde_json::from_str(&j).ok())
                .unwrap_or_default(),
            project_id: r.get("project_id")?,
            version_id: r.get("version_id")?,
            enabled: enabled != 0,
            installed_at: r.get::<_, i64>("installed_at")? as u64,
        })
    }

    /// The public type does not carry `instance_id`, so inserts go through
    /// this explicit two-arg form.
    pub fn insert_in(&self, instance_id: &str, m: &InstalledMod) -> Result<()> {
        self.conn.execute(
            "INSERT INTO installed_mods(id, instance_id, file_name, file_size, sha1,
                 name, modid, description, loaders_json, mc_versions_json,
                 project_id, version_id, enabled, installed_at)
             VALUES(?1,?2,?3,?4,?5,?6,?7,?8,?9,?10,?11,?12,?13,?14)",
            params![
                m.id,
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

    pub fn list(&self, instance_id: &str) -> Result<Vec<InstalledMod>> {
        let mut stmt = self.conn.prepare(
            "SELECT * FROM installed_mods WHERE instance_id = ?1 ORDER BY installed_at ASC",
        )?;
        let rows = stmt
            .query_map([instance_id], Self::from_row)?
            .collect::<rusqlite::Result<Vec<_>>>()?;
        Ok(rows)
    }

    pub fn get(&self, id: &str) -> Result<Option<InstalledMod>> {
        self.conn
            .query_row("SELECT * FROM installed_mods WHERE id = ?1", [id], Self::from_row)
            .optional()
            .map_err(anyhow::Error::from)
    }

    pub fn count(&self, instance_id: &str) -> Result<u32> {
        let n: i64 = self.conn.query_row(
            "SELECT COUNT(*) FROM installed_mods WHERE instance_id = ?1",
            [instance_id],
            |r| r.get(0),
        )?;
        Ok(n as u32)
    }

    pub fn set_enabled(&self, id: &str, enabled: bool) -> Result<()> {
        self.conn.execute(
            "UPDATE installed_mods SET enabled = ?1 WHERE id = ?2",
            params![enabled as i64, id],
        )?;
        Ok(())
    }

    pub fn delete(&self, id: &str) -> Result<()> {
        self.conn
            .execute("DELETE FROM installed_mods WHERE id = ?1", [id])?;
        Ok(())
    }
}

// ---------------------------------------------------------------------------
// download_tasks
// ---------------------------------------------------------------------------

/// Full download task row including persistence-only fields.
#[derive(Debug, Clone)]
pub struct DownloadTaskRow {
    pub task: DownloadTask,
    pub kind: String,
    pub instance_id: Option<String>,
    pub url: String,
    pub target_path: String,
    pub checksum_sha1: Option<String>,
}

pub struct DownloadTaskRepo<'a> {
    conn: &'a Connection,
}

impl<'a> DownloadTaskRepo<'a> {
    pub fn new(conn: &'a Connection) -> Self {
        Self { conn }
    }

    fn from_row(r: &rusqlite::Row<'_>) -> rusqlite::Result<DownloadTaskRow> {
        let state: String = r.get("state")?;
        let task = DownloadTask {
            id: r.get("id")?,
            title: r.get("title")?,
            state: parse_download_state(&state),
            total_bytes: r.get::<_, i64>("total_bytes")? as u64,
            done_bytes: r.get::<_, i64>("done_bytes")? as u64,
            speed_bps: 0,
            error: r.get("error")?,
            created_at: r.get::<_, i64>("created_at")? as u64,
            can_pause: true,
            can_cancel: true,
        };
        Ok(DownloadTaskRow {
            task,
            kind: r.get("kind")?,
            instance_id: r.get("instance_id")?,
            url: r.get("url")?,
            target_path: r.get("target_path")?,
            checksum_sha1: r.get("checksum_sha1")?,
        })
    }

    pub fn insert(&self, row: &DownloadTaskRow) -> Result<()> {
        let t = &row.task;
        self.conn.execute(
            "INSERT INTO download_tasks(id, kind, title, instance_id, url, target_path,
                 total_bytes, done_bytes, state, checksum_sha1, error, created_at, updated_at)
             VALUES(?1,?2,?3,?4,?5,?6,?7,?8,?9,?10,?11,?12,?13)",
            params![
                t.id,
                row.kind,
                t.title,
                row.instance_id,
                row.url,
                row.target_path,
                t.total_bytes as i64,
                t.done_bytes as i64,
                download_state_str(&t.state),
                row.checksum_sha1,
                t.error,
                t.created_at as i64,
                now_millis(),
            ],
        )?;
        Ok(())
    }

    /// Update progress fields (state, done_bytes, error).
    pub fn update_progress(&self, id: &str, state: &DownloadState, done_bytes: u64, error: Option<&str>) -> Result<()> {
        self.conn.execute(
            "UPDATE download_tasks SET state = ?1, done_bytes = ?2, error = ?3, updated_at = ?4
             WHERE id = ?5",
            params![
                download_state_str(state),
                done_bytes as i64,
                error,
                now_millis(),
                id,
            ],
        )?;
        Ok(())
    }

    pub fn set_total(&self, id: &str, total: u64) -> Result<()> {
        self.conn.execute(
            "UPDATE download_tasks SET total_bytes = ?1, updated_at = ?2 WHERE id = ?3",
            params![total as i64, now_millis(), id],
        )?;
        Ok(())
    }

    pub fn list(&self) -> Result<Vec<DownloadTask>> {
        let mut stmt =
            self.conn
                .prepare("SELECT * FROM download_tasks ORDER BY created_at DESC")?;
        let rows = stmt
            .query_map([], Self::from_row)?
            .collect::<rusqlite::Result<Vec<_>>>()?;
        Ok(rows.into_iter().map(|r| r.task).collect())
    }

    pub fn get(&self, id: &str) -> Result<Option<DownloadTask>> {
        Ok(self
            .get_row(id)?
            .map(|r| r.task))
    }

    pub fn get_row(&self, id: &str) -> Result<Option<DownloadTaskRow>> {
        self.conn
            .query_row(
                "SELECT * FROM download_tasks WHERE id = ?1",
                [id],
                Self::from_row,
            )
            .optional()
            .map_err(anyhow::Error::from)
    }

    pub fn list_by_state(&self, state: &DownloadState) -> Result<Vec<DownloadTask>> {
        let mut stmt = self.conn.prepare(
            "SELECT * FROM download_tasks WHERE state = ?1 ORDER BY created_at ASC",
        )?;
        let rows = stmt
            .query_map([download_state_str(state)], Self::from_row)?
            .collect::<rusqlite::Result<Vec<_>>>()?;
        Ok(rows.into_iter().map(|r| r.task).collect())
    }

    pub fn delete(&self, id: &str) -> Result<()> {
        self.conn
            .execute("DELETE FROM download_tasks WHERE id = ?1", [id])?;
        Ok(())
    }

    pub fn clear_finished(&self) -> Result<()> {
        self.conn.execute(
            "DELETE FROM download_tasks WHERE state IN ('done','failed','canceled')",
            [],
        )?;
        Ok(())
    }
}

fn parse_download_state(s: &str) -> DownloadState {
    match s {
        "queued" => DownloadState::Queued,
        "running" => DownloadState::Running,
        "paused" => DownloadState::Paused,
        "done" => DownloadState::Done,
        "failed" => DownloadState::Failed,
        "canceled" => DownloadState::Canceled,
        _ => DownloadState::Queued,
    }
}

fn download_state_str(s: &DownloadState) -> &'static str {
    match s {
        DownloadState::Queued => "queued",
        DownloadState::Running => "running",
        DownloadState::Paused => "paused",
        DownloadState::Done => "done",
        DownloadState::Failed => "failed",
        DownloadState::Canceled => "canceled",
    }
}

// ---------------------------------------------------------------------------
// java_runtimes
// ---------------------------------------------------------------------------

pub struct JavaRepo<'a> {
    conn: &'a Connection,
}

impl<'a> JavaRepo<'a> {
    pub fn new(conn: &'a Connection) -> Self {
        Self { conn }
    }

    fn from_row(r: &rusqlite::Row<'_>) -> rusqlite::Result<JavaRuntime> {
        let source: String = r.get("source")?;
        Ok(JavaRuntime {
            id: r.get("id")?,
            path: r.get("path")?,
            major: r.get::<_, i64>("major")? as u32,
            vendor: r.get::<_, Option<String>>("vendor")?.unwrap_or_default(),
            version: r.get::<_, Option<String>>("version")?.unwrap_or_default(),
            arch: r.get::<_, Option<String>>("arch")?.unwrap_or_default(),
            source: parse_java_source(&source),
        })
    }

    pub fn insert(&self, r: &JavaRuntime) -> Result<()> {
        self.conn.execute(
            "INSERT INTO java_runtimes(id, path, major, vendor, version, arch, source, added_at)
             VALUES(?1,?2,?3,?4,?5,?6,?7,?8)",
            params![
                r.id,
                r.path,
                r.major as i64,
                r.vendor,
                r.version,
                r.arch,
                java_source_str(&r.source),
                now_millis(),
            ],
        )?;
        Ok(())
    }

    pub fn list(&self) -> Result<Vec<JavaRuntime>> {
        let mut stmt =
            self.conn
                .prepare("SELECT * FROM java_runtimes ORDER BY added_at ASC")?;
        let rows = stmt
            .query_map([], Self::from_row)?
            .collect::<rusqlite::Result<Vec<_>>>()?;
        Ok(rows)
    }

    pub fn get(&self, id: &str) -> Result<Option<JavaRuntime>> {
        self.conn
            .query_row(
                "SELECT * FROM java_runtimes WHERE id = ?1",
                [id],
                Self::from_row,
            )
            .optional()
            .map_err(anyhow::Error::from)
    }

    pub fn get_by_path(&self, path: &str) -> Result<Option<JavaRuntime>> {
        self.conn
            .query_row(
                "SELECT * FROM java_runtimes WHERE path = ?1",
                [path],
                Self::from_row,
            )
            .optional()
            .map_err(anyhow::Error::from)
    }

    pub fn delete(&self, id: &str) -> Result<()> {
        self.conn
            .execute("DELETE FROM java_runtimes WHERE id = ?1", [id])?;
        Ok(())
    }
}

fn parse_java_source(s: &str) -> JavaSource {
    match s {
        "bundled" => JavaSource::Bundled,
        "manual" => JavaSource::Manual,
        _ => JavaSource::System,
    }
}

fn java_source_str(s: &JavaSource) -> &'static str {
    match s {
        JavaSource::Bundled => "bundled",
        JavaSource::System => "system",
        JavaSource::Manual => "manual",
    }
}

// ---------------------------------------------------------------------------
// version_cache
// ---------------------------------------------------------------------------

/// Cached version metadata plus the raw manifest json text for that version.
#[derive(Debug, Clone)]
pub struct VersionCacheRow {
    pub meta: VersionMeta,
    pub manifest_json: Option<String>,
    pub fetched_at: i64,
}

pub struct VersionCacheRepo<'a> {
    conn: &'a Connection,
}

impl<'a> VersionCacheRepo<'a> {
    pub fn new(conn: &'a Connection) -> Self {
        Self { conn }
    }

    fn from_row(r: &rusqlite::Row<'_>) -> rusqlite::Result<VersionCacheRow> {
        Ok(VersionCacheRow {
            meta: VersionMeta {
                id: r.get("id")?,
                version_type: r.get::<_, Option<String>>("version_type")?.unwrap_or_default(),
                release_time: r
                    .get::<_, Option<String>>("release_time")?
                    .unwrap_or_default(),
                url: r.get::<_, Option<String>>("url")?.unwrap_or_default(),
                is_latest_release: false,
                is_latest_snapshot: false,
            },
            manifest_json: r.get("manifest_json")?,
            fetched_at: r.get("fetched_at")?,
        })
    }

    /// Upsert version metadata + optional raw manifest json.
    pub fn upsert(&self, meta: &VersionMeta, manifest_json: Option<&str>) -> Result<()> {
        self.conn.execute(
            "INSERT INTO version_cache(id, version_type, release_time, url, manifest_json, fetched_at)
             VALUES(?1,?2,?3,?4,?5,?6)
             ON CONFLICT(id) DO UPDATE SET
               version_type=excluded.version_type, release_time=excluded.release_time,
               url=excluded.url, manifest_json=excluded.manifest_json,
               fetched_at=excluded.fetched_at",
            params![
                meta.id,
                meta.version_type,
                meta.release_time,
                meta.url,
                manifest_json,
                now_millis(),
            ],
        )?;
        Ok(())
    }

    pub fn list(&self) -> Result<Vec<VersionMeta>> {
        let mut stmt =
            self.conn
                .prepare("SELECT * FROM version_cache ORDER BY release_time DESC")?;
        let rows = stmt
            .query_map([], Self::from_row)?
            .collect::<rusqlite::Result<Vec<_>>>()?;
        Ok(rows.into_iter().map(|r| r.meta).collect())
    }

    pub fn get(&self, id: &str) -> Result<Option<VersionCacheRow>> {
        self.conn
            .query_row(
                "SELECT * FROM version_cache WHERE id = ?1",
                [id],
                Self::from_row,
            )
            .optional()
            .map_err(anyhow::Error::from)
    }

    pub fn delete(&self, id: &str) -> Result<()> {
        self.conn
            .execute("DELETE FROM version_cache WHERE id = ?1", [id])?;
        Ok(())
    }
}

// ---------------------------------------------------------------------------
// news_cache
// ---------------------------------------------------------------------------

pub struct NewsCacheRepo<'a> {
    conn: &'a Connection,
}

impl<'a> NewsCacheRepo<'a> {
    pub fn new(conn: &'a Connection) -> Self {
        Self { conn }
    }

    fn from_row(r: &rusqlite::Row<'_>) -> rusqlite::Result<NewsItem> {
        Ok(NewsItem {
            title: r.get("title")?,
            url: r.get("url")?,
            published: r.get::<_, Option<String>>("published")?.unwrap_or_default(),
            summary: r.get::<_, Option<String>>("summary")?.unwrap_or_default(),
        })
    }

    pub fn upsert(&self, item: &NewsItem) -> Result<()> {
        self.conn.execute(
            "INSERT INTO news_cache(id, title, url, published, summary, fetched_at)
             VALUES(?1,?2,?3,?4,?5,?6)
             ON CONFLICT(id) DO UPDATE SET
               title=excluded.title, url=excluded.url, published=excluded.published,
               summary=excluded.summary, fetched_at=excluded.fetched_at",
            params![
                hash_title(&item.title),
                item.title,
                item.url,
                item.published,
                item.summary,
                now_millis(),
            ],
        )?;
        Ok(())
    }

    pub fn list(&self) -> Result<Vec<NewsItem>> {
        let mut stmt =
            self.conn
                .prepare("SELECT * FROM news_cache ORDER BY published DESC")?;
        let rows = stmt
            .query_map([], Self::from_row)?
            .collect::<rusqlite::Result<Vec<_>>>()?;
        Ok(rows)
    }

    pub fn clear(&self) -> Result<()> {
        self.conn.execute("DELETE FROM news_cache", [])?;
        Ok(())
    }
}

fn hash_title(title: &str) -> String {
    // news items have no stable id; use title as the unique key source
    title.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Db;

    fn test_db() -> Db {
        let dir = tempfile::tempdir().unwrap();
        Db::new(dir.path().join("test.db")).unwrap()
    }

    fn sample_account(id: &str) -> AccountRow {
        AccountRow::new(Account {
            id: id.into(),
            kind: AccountKind::Offline,
            username: format!("player_{id}"),
            uuid: "00000000-0000-0000-0000-000000000000".into(),
            yggdrasil_server: None,
            skin_url: None,
            is_active: false,
            expires_at: None,
        })
    }

    fn sample_instance(id: &str, name: &str) -> InstanceSummary {
        InstanceSummary {
            id: id.into(),
            name: name.into(),
            icon: "🎮".into(),
            mc_version: "1.20.4".into(),
            loader: None,
            is_installed: false,
            last_launched_at: None,
            mod_count: 0,
            total_size_bytes: 0,
            created_at: 0,
            updated_at: 0,
        }
    }

    #[test]
    fn migration_is_idempotent() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("t.db");
        let db = Db::new(&path).unwrap();
        db.migrate().unwrap();
        db.migrate().unwrap();
        let v: i64 = db
            .conn()
            .query_row("PRAGMA user_version", [], |r| r.get(0))
            .unwrap();
        assert_eq!(v, crate::schema::SCHEMA_VERSION);
    }

    #[test]
    fn account_crud_and_active() {
        let db = test_db();
        let repo = db.account_repo();
        repo.upsert(&sample_account("a1").with_tokens(Some("enc-at".into()), None))
            .unwrap();
        repo.upsert(&sample_account("a2")).unwrap();
        assert_eq!(repo.list().unwrap().len(), 2);
        repo.set_active("a2").unwrap();
        let active = repo.get_active().unwrap().unwrap();
        assert_eq!(active.id, "a2");
        assert!(active.is_active);
        let row = repo.get_row("a1").unwrap().unwrap();
        assert_eq!(row.access_token_enc.as_deref(), Some("enc-at"));
        repo.delete("a1").unwrap();
        assert_eq!(repo.list().unwrap().len(), 1);
    }

    #[test]
    fn instance_crud_and_cascade_delete_mods() {
        let db = test_db();
        let irepo = db.instance_repo();
        irepo
            .insert(&sample_instance("i1", "Vanilla"), "/tmp/game/i1", "")
            .unwrap();
        let mod_repo = db.installed_mod_repo();
        let m = InstalledMod {
            id: "m1".into(),
            file_name: "mod.jar".into(),
            file_size: 1234,
            sha1: "abc".into(),
            name: "Mod".into(),
            modid: "mod".into(),
            description: String::new(),
            loaders: vec![],
            mc_versions: vec!["1.20.4".into()],
            project_id: None,
            version_id: None,
            enabled: true,
            installed_at: 1,
        };
        mod_repo.insert_in("i1", &m).unwrap();
        assert_eq!(mod_repo.count("i1").unwrap(), 1);
        let list = irepo.list().unwrap();
        assert_eq!(list[0].mod_count, 1);
        assert_eq!(list[0].total_size_bytes, 1234);

        irepo.update("i1", Some("Renamed"), Some("🔥"), None, None, None, None, Some(true), None)
            .unwrap();
        let detail = irepo.get_detail("i1").unwrap().unwrap();
        assert_eq!(detail.summary.name, "Renamed");
        assert!(detail.summary.is_installed);

        irepo.delete("i1").unwrap();
        assert_eq!(mod_repo.count("i1").unwrap(), 0);
        assert!(irepo.get("i1").unwrap().is_none());
    }

    #[test]
    fn download_task_repo_roundtrip() {
        let db = test_db();
        let repo = db.download_task_repo();
        let row = DownloadTaskRow {
            task: DownloadTask {
                id: "d1".into(),
                title: "Download".into(),
                state: DownloadState::Queued,
                total_bytes: 100,
                done_bytes: 0,
                speed_bps: 0,
                error: None,
                created_at: 1,
                can_pause: true,
                can_cancel: true,
            },
            kind: "asset".into(),
            instance_id: None,
            url: "https://example.com/a".into(),
            target_path: "/tmp/a".into(),
            checksum_sha1: Some("sha".into()),
        };
        repo.insert(&row).unwrap();
        repo.update_progress("d1", &DownloadState::Running, 50, None)
            .unwrap();
        repo.set_total("d1", 200).unwrap();
        let t = repo.get("d1").unwrap().unwrap();
        assert_eq!(t.state, DownloadState::Running);
        assert_eq!(t.done_bytes, 50);
        assert_eq!(t.total_bytes, 200);
        assert_eq!(repo.list_by_state(&DownloadState::Running).unwrap().len(), 1);
        repo.update_progress("d1", &DownloadState::Done, 200, None)
            .unwrap();
        repo.clear_finished().unwrap();
        assert!(repo.get("d1").unwrap().is_none());
    }

    #[test]
    fn java_repo_roundtrip() {
        let db = test_db();
        let repo = db.java_repo();
        repo.insert(&JavaRuntime {
            id: "j1".into(),
            path: "/usr/bin/java".into(),
            major: 21,
            vendor: "Temurin".into(),
            version: "21.0.2".into(),
            arch: "x64".into(),
            source: JavaSource::System,
        })
        .unwrap();
        let all = repo.list().unwrap();
        assert_eq!(all.len(), 1);
        assert_eq!(all[0].major, 21);
        assert_eq!(all[0].source, JavaSource::System);
        repo.delete("j1").unwrap();
        assert!(repo.list().unwrap().is_empty());
    }

    #[test]
    fn version_cache_upsert() {
        let db = test_db();
        let repo = db.version_cache_repo();
        let meta = VersionMeta {
            id: "1.20.4".into(),
            version_type: "release".into(),
            release_time: "2023-12-07T00:00:00Z".into(),
            url: "https://example.com/1.20.4.json".into(),
            is_latest_release: true,
            is_latest_snapshot: false,
        };
        repo.upsert(&meta, Some("{\"id\":\"1.20.4\"}")).unwrap();
        repo.upsert(&meta, Some("{\"id\":\"1.20.4\",\"x\":1}")).unwrap();
        let row = repo.get("1.20.4").unwrap().unwrap();
        assert!(row.manifest_json.as_deref().unwrap().contains("\"x\":1"));
        assert_eq!(repo.list().unwrap().len(), 1);
        repo.delete("1.20.4").unwrap();
        assert!(repo.get("1.20.4").unwrap().is_none());
    }

    #[test]
    fn news_cache_upsert_and_clear() {
        let db = test_db();
        let repo = db.news_cache_repo();
        repo.upsert(&NewsItem {
            title: "News".into(),
            url: "https://example.com".into(),
            published: "2024-01-01".into(),
            summary: "sum".into(),
        })
        .unwrap();
        assert_eq!(repo.list().unwrap().len(), 1);
        repo.clear().unwrap();
        assert!(repo.list().unwrap().is_empty());
    }

    #[test]
    fn settings_repo() {
        let db = test_db();
        let repo = db.settings_repo();
        repo.set("download_source", "bmclapi").unwrap();
        assert_eq!(
            repo.get("download_source").unwrap().as_deref(),
            Some("bmclapi")
        );
        repo.set("download_source", "official").unwrap();
        assert_eq!(
            repo.get("download_source").unwrap().as_deref(),
            Some("official")
        );
    }
}