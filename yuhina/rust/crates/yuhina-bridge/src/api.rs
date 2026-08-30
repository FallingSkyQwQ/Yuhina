//! FRB scan entry point (`rust_input: crate::api`).
//!
//! Re-exports the `YuhinaService` facade and every shared type from
//! `yuhina-api` so flutter_rust_bridge can generate Dart bindings that match
//! `docs/api-contract.md` exactly.

pub use crate::service::YuhinaService;

pub use yuhina_api::{
    Account, AccountKind, AppEvent, ConflictKind, ConflictSeverity, CreateInstanceRequest,
    DownloadProgressEvent, DownloadState, DownloadTask, GameLogEntry, GameOutput, GameSession,
    GameState, InstalledMod, InstanceDetail, InstanceSummary, JavaRuntime, JavaSelection,
    JavaSource, LauncherConfig, LaunchArgs, Loader, LoaderKind, LogLevel, MicrosoftLoginHandle,
    ModConflict, ModUpdate, ModrinthDependency, ModrinthFile, ModrinthProject, ModrinthVersion,
    NewsItem, SearchResult, Source, VersionMeta, YuhinaError, YuhinaErrorKind,
};

use flutter_rust_bridge::frb;

// FRB auto-opaques third-party enums whose payload fields are detected as
// non-public (tuple variants). Mirrors make them translatable (self-crate),
// keeping the api-contract.md type surface byte-for-byte.
#[frb(mirror(JavaSelection))]
pub enum _JavaSelection {
    Auto(u32),
    Manual(String),
}

#[frb(mirror(Source))]
pub enum _Source {
    Official,
    Bmclapi,
    Custom(String),
}

#[frb(mirror(GameState))]
pub enum _GameState {
    Starting,
    Running,
    Stopped(i32),
    Crashed(String),
}

#[frb(mirror(AppEvent))]
pub enum _AppEvent {
    ConfigChanged,
    AccountsChanged,
    InstancesChanged,
    TaskChanged(String),
    JavaRuntimesChanged,
    VersionListChanged,
}

#[frb(mirror(YuhinaErrorKind))]
pub enum _YuhinaErrorKind {
    Network,
    Http(u16, String),
    Auth,
    AuthExpired,
    NotLoggedIn,
    VersionNotFound,
    LoaderNotInstalled,
    JavaNotFound,
    InvalidInstance,
    ModConflict,
    ModpackInvalid,
    ChecksumMismatch,
    DownloadFailed,
    Canceled,
    Io,
    Internal,
}