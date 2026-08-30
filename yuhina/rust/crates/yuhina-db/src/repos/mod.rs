//! Repository modules. Each repo holds a clone of the shared connection.

pub mod account;
pub mod instance;
pub mod java;
pub mod version_cache;

pub use account::{AccountRepo, AccountRow};
pub use instance::InstanceRepo;
pub use java::JavaRepo;
pub use version_cache::{VersionCacheEntry, VersionCacheRepo};

pub(crate) fn now_ms() -> i64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_millis() as i64)
        .unwrap_or(0)
}