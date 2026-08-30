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