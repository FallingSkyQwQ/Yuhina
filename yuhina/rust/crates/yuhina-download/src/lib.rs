//! Download manager, mirrors, news and launcher self-update (Agent B).

use yuhina_api::YuhinaError;

/// Result alias used across this crate (contract §1).
pub type YuhinaResult<T> = std::result::Result<T, YuhinaError>;

pub mod checksum;
pub mod manager;
pub mod mirror;
pub mod news;
pub mod resume;
pub mod store;
pub mod task;
pub mod update;
pub mod worker;

pub use manager::{
    DownloadManager, ManagerConfig, DEFAULT_BACKOFF_BASE_MS, DEFAULT_BACKOFF_CAP_MS,
    DEFAULT_CONCURRENCY, DEFAULT_PERSIST_INTERVAL_MS, DEFAULT_PROGRESS_INTERVAL_MS,
    DEFAULT_RETRY_MAX,
};
pub use mirror::rewrite_url;
pub use news::{NewsService, DEFAULT_NEWS_RSS_URL};
pub use store::{Store, StoredTask};
pub use task::{FileReq, Priority, TaskKind};
pub use update::{check_launcher_update, compare_versions, DEFAULT_UPDATE_API_URL};
