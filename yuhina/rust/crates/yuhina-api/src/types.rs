//! Shared types. Field names and shapes match `docs/api-contract.md` §2 exactly.
//! These types are consumed by every crate and by `yuhina-bridge` (FRB).

use serde::{Deserialize, Serialize};

// ---------------------------------------------------------------------------
// §2.1 配置
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LauncherConfig {
    pub data_dir: String,
    pub game_root: String,
    pub download_source: Source,
    pub custom_source_host: Option<String>,
    pub launch_args: LaunchArgs,
    pub locale: String,
    pub theme_seed: u32,
    pub auto_update: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LaunchArgs {
    pub min_memory_mb: u32,
    pub max_memory_mb: u32,
    pub extra_jvm_args: Vec<String>,
    pub extra_mc_args: Vec<String>,
    pub window_width: Option<u32>,
    pub window_height: Option<u32>,
}

impl Default for LaunchArgs {
    fn default() -> Self {
        Self {
            min_memory_mb: 1024,
            max_memory_mb: 4096,
            extra_jvm_args: Vec::new(),
            extra_mc_args: Vec::new(),
            window_width: None,
            window_height: None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Source {
    Official,
    Bmclapi,
    Custom(String),
}

// ---------------------------------------------------------------------------
// §2.2 账号
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Account {
    pub id: String,
    pub kind: AccountKind,
    pub username: String,
    pub uuid: String,
    pub yggdrasil_server: Option<String>,
    pub skin_url: Option<String>,
    pub is_active: bool,
    pub expires_at: Option<u64>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum AccountKind {
    Microsoft,
    Yggdrasil,
    Offline,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MicrosoftLoginHandle {
    pub handle_id: String,
}

// ---------------------------------------------------------------------------
// §2.3 版本 / Java
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VersionMeta {
    pub id: String,
    pub version_type: String,
    pub release_time: String,
    pub url: String,
    pub is_latest_release: bool,
    pub is_latest_snapshot: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct JavaRuntime {
    pub id: String,
    pub path: String,
    pub major: u32,
    pub vendor: String,
    pub version: String,
    pub arch: String,
    pub source: JavaSource,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum JavaSource {
    Bundled,
    System,
    Manual,
}

// ---------------------------------------------------------------------------
// §2.4 实例
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InstanceSummary {
    pub id: String,
    pub name: String,
    pub icon: String,
    pub mc_version: String,
    pub loader: Option<Loader>,
    pub is_installed: bool,
    pub last_launched_at: Option<u64>,
    pub mod_count: u32,
    pub total_size_bytes: u64,
    pub created_at: u64,
    pub updated_at: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Loader {
    pub kind: LoaderKind,
    pub version: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum LoaderKind {
    Forge,
    Fabric,
    NeoForge,
    Quilt,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InstanceDetail {
    pub summary: InstanceSummary,
    pub game_dir: String,
    pub java: JavaSelection,
    pub launch_args: Option<LaunchArgs>,
    pub notes: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CreateInstanceRequest {
    pub name: String,
    pub icon: String,
    pub mc_version: String,
    pub loader: Option<Loader>,
    pub java: JavaSelection,
    pub dir_name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum JavaSelection {
    Auto(u32),
    Manual(String),
}

// ---------------------------------------------------------------------------
// §2.5 Mod
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InstalledMod {
    pub id: String,
    pub file_name: String,
    pub file_size: u64,
    pub sha1: String,
    pub name: String,
    pub modid: String,
    pub description: String,
    pub loaders: Vec<String>,
    pub mc_versions: Vec<String>,
    pub project_id: Option<String>,
    pub version_id: Option<String>,
    pub enabled: bool,
    pub installed_at: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ModrinthProject {
    pub project_id: String,
    pub slug: String,
    pub title: String,
    pub description: String,
    pub icon_url: Option<String>,
    pub downloads: u64,
    pub follows: u64,
    pub loaders: Vec<String>,
    pub game_versions: Vec<String>,
    pub categories: Vec<String>,
    pub versions: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ModrinthVersion {
    pub version_id: String,
    pub name: String,
    pub version_number: String,
    pub game_versions: Vec<String>,
    pub loaders: Vec<String>,
    pub files: Vec<ModrinthFile>,
    pub dependencies: Vec<ModrinthDependency>,
    pub published: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ModrinthFile {
    pub name: String,
    pub size: u64,
    pub url: String,
    pub sha1: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ModrinthDependency {
    pub project_id: Option<String>,
    pub version_id: Option<String>,
    pub dep_type: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SearchResult {
    pub hits: Vec<ModrinthProject>,
    pub total: u64,
    pub offset: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ModUpdate {
    pub installed: InstalledMod,
    pub latest: ModrinthVersion,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ModConflict {
    pub severity: ConflictSeverity,
    pub kind: ConflictKind,
    pub message: String,
    pub related_files: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConflictSeverity {
    Warning,
    Error,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConflictKind {
    DuplicateModId,
    DuplicateFile,
    LoaderMismatch,
    McVersionMismatch,
    IncompatibleDependency,
    MissingDependency,
}

// ---------------------------------------------------------------------------
// §2.6 下载任务
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DownloadTask {
    pub id: String,
    pub title: String,
    pub state: DownloadState,
    pub total_bytes: u64,
    pub done_bytes: u64,
    pub speed_bps: u64,
    pub error: Option<String>,
    pub created_at: u64,
    pub can_pause: bool,
    pub can_cancel: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum DownloadState {
    Queued,
    Running,
    Paused,
    Done,
    Failed,
    Canceled,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DownloadProgressEvent {
    pub task_id: String,
    pub state: DownloadState,
    pub done_bytes: u64,
    pub total_bytes: u64,
    pub speed_bps: u64,
}

// ---------------------------------------------------------------------------
// §2.7 游戏会话 / 日志 / 资讯
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GameSession {
    pub session_id: String,
    pub instance_id: String,
    pub pid: u32,
    pub state: GameState,
    pub started_at: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum GameState {
    Starting,
    Running,
    Stopped(i32),
    Crashed(String),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GameOutput {
    pub session_id: String,
    pub level: LogLevel,
    pub text: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum LogLevel {
    Info,
    Warn,
    Error,
    Debug,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NewsItem {
    pub title: String,
    pub url: String,
    pub published: String,
    pub summary: String,
}

// ---------------------------------------------------------------------------
// §3.7 游戏日志回放
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GameLogEntry {
    pub index: u64,
    pub level: LogLevel,
    pub text: String,
    pub ts: u64,
}

// ---------------------------------------------------------------------------
// §2.8 事件（服务端 → UI）
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum AppEvent {
    ConfigChanged,
    AccountsChanged,
    InstancesChanged,
    TaskChanged(String),
    JavaRuntimesChanged,
    VersionListChanged,
}