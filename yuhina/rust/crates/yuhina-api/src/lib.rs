//! Shared types and error model for Yuhina (api-contract.md §1–§2).

pub mod error;
pub mod types;

pub use error::{YuhinaError, YuhinaErrorKind, YuhinaResult};
pub use types::*;

/// Convenience alias used across the workspace. Defined at the crate root (not
/// in `error`) so FRB's glob imports of the error module never collide with
/// `std::result::Result` inside generated bridge code.
pub type Result<T> = YuhinaResult<T>;