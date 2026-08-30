//! Shared types and error model for Yuhina (api-contract.md §1–§2).

pub mod error;
pub mod types;

pub use error::{YuhinaError, YuhinaErrorKind, Result, YuhinaResult};
pub use types::*;