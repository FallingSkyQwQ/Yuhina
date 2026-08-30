//! Download task domain types and persistence mapping.

use std::path::PathBuf;

use yuhina_api::DownloadState;

use crate::store::{StoredTask, now_ms};

/// Queue priority. Higher value = popped first (启动类 > 库 > 资产).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Priority {
    Asset,
    Library,
    Launch,
}

impl Default for Priority {
    fn default() -> Self {
        Priority::Library
    }
}

/// Classification of a download, persisted as the `kind` column.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TaskKind {
    Instance,
    Library,
    Asset,
    Mod,
    Java,
    Modpack,
    Other,
}

impl TaskKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            TaskKind::Instance => "instance",
            TaskKind::Library => "library",
            TaskKind::Asset => "asset",
            TaskKind::Mod => "mod",
            TaskKind::Java => "java",
            TaskKind::Modpack => "modpack",
            TaskKind::Other => "other",
        }
    }

    pub fn from_str(s: &str) -> TaskKind {
        match s {
            "instance" => TaskKind::Instance,
            "library" => TaskKind::Library,
            "asset" => TaskKind::Asset,
            "mod" => TaskKind::Mod,
            "java" => TaskKind::Java,
            "modpack" => TaskKind::Modpack,
            _ => TaskKind::Other,
        }
    }
}

/// A single file download request (manager entry point).
#[derive(Debug, Clone)]
pub struct FileReq {
    /// Optional caller-supplied id (UUID v4). Generated when `None`.
    pub id: Option<String>,
    /// Human readable title shown in the download center.
    pub title: String,
    pub url: String,
    pub dest: PathBuf,
    /// Optional expected sha1 (hex). Verified after download.
    pub sha1: Option<String>,
    pub priority: Priority,
    pub kind: TaskKind,
    pub instance_id: Option<String>,
}

/// Maps a request to a persisted row (used when enqueueing a new task).
pub fn row_from_req(id: String, req: &FileReq, state: DownloadState, created_at: u64) -> StoredTask {
    StoredTask {
        id,
        kind: req.kind.as_str().to_string(),
        title: req.title.clone(),
        instance_id: req.instance_id.clone(),
        url: req.url.clone(),
        target_path: req.dest.to_string_lossy().into_owned(),
        total_bytes: 0,
        done_bytes: 0,
        state,
        checksum_sha1: req.sha1.clone(),
        error: None,
        created_at,
        updated_at: now_ms(),
    }
}

/// Rebuilds a request from a persisted row (used on restart / resume).
/// Priority is runtime-only and not persisted, so a default is supplied.
pub fn req_from_row(row: &StoredTask, priority: Priority) -> FileReq {
    FileReq {
        id: Some(row.id.clone()),
        title: row.title.clone(),
        url: row.url.clone(),
        dest: PathBuf::from(&row.target_path),
        sha1: row.checksum_sha1.clone(),
        priority,
        kind: TaskKind::from_str(&row.kind),
        instance_id: row.instance_id.clone(),
    }
}

/// Whether a state allows pause/cancel (contract §2.6 semantics).
pub fn flags_for_state(state: &DownloadState) -> (bool, bool) {
    match state {
        DownloadState::Queued | DownloadState::Running => (true, true),
        DownloadState::Paused => (false, true),
        _ => (false, false),
    }
}

/// The temporary in-progress file path: `<dest>.part`.
pub fn part_path(dest: &std::path::Path) -> PathBuf {
    let mut s = dest.as_os_str().to_owned();
    s.push(".part");
    PathBuf::from(s)
}

