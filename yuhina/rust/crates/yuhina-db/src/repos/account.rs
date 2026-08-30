//! `accounts` table repository (api-contract.md §5).
//! Token columns hold encrypted blobs; encryption lives in `yuhina-auth`.

use std::sync::{Arc, Mutex};

use rusqlite::Connection;

use yuhina_api::{Account, AccountKind};

use crate::repos::now_ms;

/// A row in the `accounts` table. Token fields hold encrypted blobs.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AccountRow {
    pub id: String,
    pub kind: String,
    pub username: String,
    pub uuid: String,
    pub yggdrasil_server: Option<String>,
    pub skin_url: Option<String>,
    pub access_token_enc: Option<String>,
    pub refresh_token_enc: Option<String>,
    pub expires_at: Option<i64>,
    pub is_active: bool,
    pub created_at: Option<i64>,
    pub updated_at: Option<i64>,
}

#[derive(Debug, Clone)]
pub struct AccountRepo {
    pub(crate) conn: Arc<Mutex<Connection>>,
}

impl AccountRepo {
    /// Upsert a row (INSERT ... ON CONFLICT(id) DO UPDATE).
    pub fn upsert(&self, row: &AccountRow) -> anyhow::Result<()> {
        let conn = self.conn.lock().unwrap();
        let now = now_ms();
        let created_at = row.created_at.unwrap_or(now);
        conn.execute(
            r#"
            INSERT INTO accounts
                (id, kind, username, uuid, yggdrasil_server, skin_url,
                 access_token_enc, refresh_token_enc, expires_at, is_active,
                 created_at, updated_at)
            VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9,?10,?11,?12)
            ON CONFLICT(id) DO UPDATE SET
                kind = excluded.kind,
                username = excluded.username,
                uuid = excluded.uuid,
                yggdrasil_server = excluded.yggdrasil_server,
                skin_url = excluded.skin_url,
                access_token_enc = excluded.access_token_enc,
                refresh_token_enc = excluded.refresh_token_enc,
                expires_at = excluded.expires_at,
                is_active = excluded.is_active,
                updated_at = excluded.updated_at
            "#,
            rusqlite::params![
                row.id,
                row.kind,
                row.username,
                row.uuid,
                row.yggdrasil_server,
                row.skin_url,
                row.access_token_enc,
                row.refresh_token_enc,
                row.expires_at,
                row.is_active as i64,
                created_at,
                row.updated_at.unwrap_or(now),
            ],
        )?;
        Ok(())
    }

    pub fn get(&self, id: &str) -> anyhow::Result<Option<AccountRow>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(&format!("{SELECT} WHERE id = ?1"))?;
        let mut rows = stmt.query([id])?;
        if let Some(r) = rows.next()? {
            Ok(Some(row_from(r)?))
        } else {
            Ok(None)
        }
    }

    pub fn list(&self) -> anyhow::Result<Vec<AccountRow>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(&format!("{SELECT} ORDER BY created_at ASC"))?;
        let rows = stmt.query_map([], |r| row_from(r))?;
        let mut out = Vec::new();
        for row in rows {
            out.push(row?);
        }
        Ok(out)
    }

    pub fn delete(&self, id: &str) -> anyhow::Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute("DELETE FROM accounts WHERE id = ?1", [id])?;
        Ok(())
    }

    /// Set exactly one account active (clears all others first).
    pub fn set_active(&self, id: &str) -> anyhow::Result<()> {
        let conn = self.conn.lock().unwrap();
        let tx = conn.unchecked_transaction()?;
        tx.execute("UPDATE accounts SET is_active = 0 WHERE is_active = 1", [])?;
        tx.execute(
            "UPDATE accounts SET is_active = 1, updated_at = ?1 WHERE id = ?2",
            rusqlite::params![now_ms(), id],
        )?;
        tx.commit()?;
        Ok(())
    }

    pub fn clear_active(&self) -> anyhow::Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute("UPDATE accounts SET is_active = 0 WHERE is_active = 1", [])?;
        Ok(())
    }

    pub fn get_active(&self) -> anyhow::Result<Option<Account>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(&format!("{SELECT} WHERE is_active = 1 LIMIT 1"))?;
        let mut rows = stmt.query([])?;
        if let Some(r) = rows.next()? {
            Ok(Some(
                Self::to_account(row_from(r)?).map_err(|e| anyhow::anyhow!(e.to_string()))?,
            ))
        } else {
            Ok(None)
        }
    }

    pub fn has_active(&self) -> anyhow::Result<bool> {
        let conn = self.conn.lock().unwrap();
        let n: i64 = conn.query_row(
            "SELECT COUNT(*) FROM accounts WHERE is_active = 1",
            [],
            |r| r.get(0),
        )?;
        Ok(n > 0)
    }

    /// Map a row to the public `Account` (no tokens).
    pub fn to_account(row: AccountRow) -> yuhina_api::YuhinaResult<Account> {
        let kind = AccountKind::from_str(&row.kind).ok_or_else(|| {
            yuhina_api::YuhinaError::internal(format!("unknown account kind: {}", row.kind))
        })?;
        Ok(Account {
            id: row.id,
            kind,
            username: row.username,
            uuid: row.uuid,
            yggdrasil_server: row.yggdrasil_server,
            skin_url: row.skin_url,
            is_active: row.is_active,
            expires_at: row.expires_at.map(|v| v as u64),
        })
    }
}

const SELECT: &str = r#"
    SELECT id, kind, username, uuid, yggdrasil_server, skin_url,
           access_token_enc, refresh_token_enc, expires_at, is_active,
           created_at, updated_at
    FROM accounts
"#;

fn row_from(r: &rusqlite::Row<'_>) -> rusqlite::Result<AccountRow> {
    Ok(AccountRow {
        id: r.get(0)?,
        kind: r.get(1)?,
        username: r.get(2)?,
        uuid: r.get(3)?,
        yggdrasil_server: r.get(4)?,
        skin_url: r.get(5)?,
        access_token_enc: r.get(6)?,
        refresh_token_enc: r.get(7)?,
        expires_at: r.get(8)?,
        is_active: r.get::<_, i64>(9)? != 0,
        created_at: r.get(10)?,
        updated_at: r.get(11)?,
    })
}