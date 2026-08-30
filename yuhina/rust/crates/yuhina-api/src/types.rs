//! Shared data types (api-contract.md §2).
//! Implements the complete frozen contract type surface so all crates can
//! build in parallel. Account types and `AppEvent` are owned by Agent D;
//! the remaining types are owned by Agents A/B/C and match the contract
//! field-for-field.

// ---------------------------------------------------------------------------
// §2.1 配置
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct LauncherConfig {
    /// 启动器数据目录(实例/下载/缓存)
    pub data_dir: String,
    /// 游戏目录根
    pub game_root: String,
    pub download_source: Source,
    pub custom_source_host: Option<String>,
    /// 全局默认 JVM/GC/分辨率
    pub launch_args: LaunchArgs,
    /// "zh-CN" | "en-US"
    pub locale: String,
    /// UI 动态主题种子色(由 UI 使用)
    pub theme_seed: u32,
    /// 启动器自更新开关
    pub auto_update: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct LaunchArgs {
    pub min_memory_mb: u32,
    pub max_memory_mb: u32,
    pub extra_jvm_args: Vec<String>,
    pub extra_mc_args: Vec<String>,
    pub window_width: Option<u32>,
    pub window_height: Option<u32>,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum Source {
    Official,
    Bmclapi,
    Custom(String),
}

// ---------------------------------------------------------------------------
// §2.2 账号
// ---------------------------------------------------------------------------

/// Account as exposed to the UI. Tokens are NOT part of this struct; they are
/// stored encrypted in `yuhina-db`.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct Account {
    /// UUID v4 identifying the stored account.
    pub id: String,
    pub kind: AccountKind,
    /// Display name (in-game name for online accounts).
    pub username: String,
    /// Player UUID (offline uuid for offline accounts).
    pub uuid: String,
    /// Only set for Yggdrasil accounts.
    pub yggdrasil_server: Option<String>,
    pub skin_url: Option<String>,
    pub is_active: bool,
    /// Millisecond timestamp; None for offline accounts.
    pub expires_at: Option<u64>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum AccountKind {
    Microsoft,
    Yggdrasil,
    Offline,
}

/// Handle returned by `begin_microsoft_login`, used to poll/cancel the flow.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct MicrosoftLoginHandle {
    pub handle_id: String,
}

/// Minimal launch credentials for an active account (Agent A consumption via
/// `yuhina-bridge`). Not part of api-contract.md §2.2; added so the bridge can
/// hand auth parameters to the launch pipeline without exposing tokens over FFI.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct AccountAuth {
    pub username: String,
    pub uuid: String,
    pub access_token: String,
    /// "msa" | "mojang" (yggdrasil) | "legacy" (offline)
    pub user_type: String,
}

impl AccountKind {
    pub fn as_str(self) -> &'static str {
        match self {
            AccountKind::Microsoft => "microsoft",
            AccountKind::Yggdrasil => "yggdrasil",
            AccountKind::Offline => "offline",
        }
    }

    #[allow(clippy::should_implement_trait)]
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "microsoft" => Some(AccountKind::Microsoft),
            "yggdrasil" => Some(AccountKind::Yggdrasil),
            "offline" => Some(AccountKind::Offline),
            _ => None,
        }
    }
}

// ---------------------------------------------------------------------------
// §2.3 版本 / Java
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct VersionMeta {
    pub id: String,
    /// release|snapshot|old_beta|old_alpha
    pub version_type: String,
    pub release_time: String,
    /// manifest 内 url
    pub url: String,
    pub is_latest_release: bool,
    pub is_latest_snapshot: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct JavaRuntime {
    pub id: String,
    pub path: String,
    pub major: u32,
    pub vendor: String,
    pub version: String,
    pub arch: String,
    pub source: JavaSource,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum JavaSource {
    Bundled,
    System,
    Manual,
}

// ---------------------------------------------------------------------------
// §2.4 实例
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct InstanceSummary {
    pub id: String,
    pub name: String,
    /// emoji 或路径
    pub icon: String,
    pub mc_version: String,
    /// None = 原版
    pub loader: Option<Loader>,
    /// 是否已下载完成可启动
    pub is_installed: bool,
    pub last_launched_at: Option<u64>,
    pub mod_count: u32,
    pub total_size_bytes: u64,
    pub created_at: u64,
    pub updated_at: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct Loader {
    pub kind: LoaderKind,
    pub version: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum LoaderKind {
    Forge,
    Fabric,
    NeoForge,
    Quilt,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct InstanceDetail {
    pub summary: InstanceSummary,
    pub game_dir: String,
    /// Manual(path) | Auto(major)
    pub java: JavaSelection,
    /// 覆盖全局
    pub launch_args: Option<LaunchArgs>,
    pub notes: String,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct CreateInstanceRequest {
    pub name: String,
    pub icon: String,
    pub mc_version: String,
    pub loader: Option<Loader>,
    pub java: JavaSelection,
    pub dir_name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum JavaSelection {
    Auto(u32),
    Manual(String),
}

// ---------------------------------------------------------------------------
// §2.5 Mod
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct InstalledMod {
    /// instance内唯一(文件hash)
    pub id: String,
    pub file_name: String,
    pub file_size: u64,
    pub sha1: String,
    pub name: String,
    pub modid: String,
    pub description: String,
    pub loaders: Vec<String>,
    pub mc_versions: Vec<String>,
    /// Modrinth 关联
    pub project_id: Option<String>,
    pub version_id: Option<String>,
    pub enabled: bool,
    pub installed_at: u64,
}

/// 搜索/详情共用
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
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

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
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

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct ModrinthFile {
    pub name: String,
    pub size: u64,
    pub url: String,
    pub sha1: String,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct ModrinthDependency {
    pub project_id: Option<String>,
    pub version_id: Option<String>,
    /// required | optional | incompatible
    pub dep_type: String,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct SearchResult {
    pub hits: Vec<ModrinthProject>,
    pub total: u64,
    pub offset: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct ModUpdate {
    pub installed: InstalledMod,
    pub latest: ModrinthVersion,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct ModConflict {
    pub severity: ConflictSeverity,
    pub kind: ConflictKind,
    pub message: String,
    pub related_files: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum ConflictSeverity {
    Warning,
    Error,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
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

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum DownloadState {
    Queued,
    Running,
    Paused,
    Done,
    Failed,
    Canceled,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
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

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct GameSession {
    pub session_id: String,
    pub instance_id: String,
    pub pid: u32,
    pub state: GameState,
    pub started_at: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum GameState {
    Starting,
    Running,
    Stopped(i32),
    Crashed(String),
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct GameOutput {
    pub session_id: String,
    pub level: LogLevel,
    pub text: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum LogLevel {
    Info,
    Warn,
    Error,
    Debug,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct NewsItem {
    pub title: String,
    pub url: String,
    pub published: String,
    pub summary: String,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct GameLogEntry {
    pub index: u64,
    pub level: LogLevel,
    pub text: String,
    pub ts: u64,
}

// ---------------------------------------------------------------------------
// §2.8 事件（服务端 → UI）
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum AppEvent {
    ConfigChanged,
    AccountsChanged,
    InstancesChanged,
    TaskChanged(String),
    JavaRuntimesChanged,
    VersionListChanged,
}