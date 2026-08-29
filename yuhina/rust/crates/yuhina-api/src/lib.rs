//! Shared types and error model for Yuhina.
//!
//! This crate is the FFI contract layer (see `docs/api-contract.md`).
//! Every other crate (db/core/download/instance/auth/bridge) depends on it.
//! Type and field names must match the contract exactly — do not rename.

pub mod error;
pub mod types;

pub use error::{YuhinaError, YuhinaErrorKind, YuhinaResult};
pub use types::*;