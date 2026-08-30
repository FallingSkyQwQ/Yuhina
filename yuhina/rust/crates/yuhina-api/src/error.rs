//! Error model (api-contract.md §1).

use std::fmt;

/// Structured error kind. Mirrors `YuhinaErrorKind` in the FFI contract.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum YuhinaErrorKind {
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

/// Application error surfaced to the UI as a Dart `YuhinaError` with `kind` + `message`.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct YuhinaError {
    pub kind: YuhinaErrorKind,
    pub message: String,
}

impl fmt::Display for YuhinaError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.kind_label(), self.message)
    }
}

impl std::error::Error for YuhinaError {}

impl YuhinaError {
    pub fn new(kind: YuhinaErrorKind, message: impl Into<String>) -> Self {
        Self {
            kind,
            message: message.into(),
        }
    }

    pub fn network(message: impl Into<String>) -> Self {
        Self::new(YuhinaErrorKind::Network, message)
    }

    pub fn http(status: u16, url: impl Into<String>) -> Self {
        Self::new(
            YuhinaErrorKind::Http(status, url.into()),
            format!("HTTP {status}"),
        )
    }

    pub fn auth(message: impl Into<String>) -> Self {
        Self::new(YuhinaErrorKind::Auth, message)
    }

    pub fn auth_expired(message: impl Into<String>) -> Self {
        Self::new(YuhinaErrorKind::AuthExpired, message)
    }

    pub fn not_logged_in() -> Self {
        Self::new(
            YuhinaErrorKind::NotLoggedIn,
            "No active account is selected. Please log in first.",
        )
    }

    pub fn canceled() -> Self {
        Self::new(YuhinaErrorKind::Canceled, "Operation canceled.")
    }

    pub fn version_not_found(message: impl Into<String>) -> Self {
        Self::new(YuhinaErrorKind::VersionNotFound, message)
    }

    pub fn loader_not_installed(message: impl Into<String>) -> Self {
        Self::new(YuhinaErrorKind::LoaderNotInstalled, message)
    }

    pub fn java_not_found(message: impl Into<String>) -> Self {
        Self::new(YuhinaErrorKind::JavaNotFound, message)
    }

    pub fn invalid_instance(message: impl Into<String>) -> Self {
        Self::new(YuhinaErrorKind::InvalidInstance, message)
    }

    pub fn mod_conflict(message: impl Into<String>) -> Self {
        Self::new(YuhinaErrorKind::ModConflict, message)
    }

    pub fn modpack_invalid(message: impl Into<String>) -> Self {
        Self::new(YuhinaErrorKind::ModpackInvalid, message)
    }

    pub fn checksum_mismatch(message: impl Into<String>) -> Self {
        Self::new(YuhinaErrorKind::ChecksumMismatch, message)
    }

    pub fn download_failed(message: impl Into<String>) -> Self {
        Self::new(YuhinaErrorKind::DownloadFailed, message)
    }

    /// Alias used by Agent A's core (`not_found_version`).
    pub fn not_found_version(message: impl Into<String>) -> Self {
        Self::version_not_found(message)
    }

    pub fn io(message: impl Into<String>) -> Self {
        Self::new(YuhinaErrorKind::Io, message)
    }

    pub fn internal(message: impl Into<String>) -> Self {
        Self::new(YuhinaErrorKind::Internal, message)
    }

    fn kind_label(&self) -> &'static str {
        match self.kind {
            YuhinaErrorKind::Network => "Network",
            YuhinaErrorKind::Http(..) => "Http",
            YuhinaErrorKind::Auth => "Auth",
            YuhinaErrorKind::AuthExpired => "AuthExpired",
            YuhinaErrorKind::NotLoggedIn => "NotLoggedIn",
            YuhinaErrorKind::VersionNotFound => "VersionNotFound",
            YuhinaErrorKind::LoaderNotInstalled => "LoaderNotInstalled",
            YuhinaErrorKind::JavaNotFound => "JavaNotFound",
            YuhinaErrorKind::InvalidInstance => "InvalidInstance",
            YuhinaErrorKind::ModConflict => "ModConflict",
            YuhinaErrorKind::ModpackInvalid => "ModpackInvalid",
            YuhinaErrorKind::ChecksumMismatch => "ChecksumMismatch",
            YuhinaErrorKind::DownloadFailed => "DownloadFailed",
            YuhinaErrorKind::Canceled => "Canceled",
            YuhinaErrorKind::Io => "Io",
            YuhinaErrorKind::Internal => "Internal",
        }
    }
}

/// Convenience alias used across crates (e.g. `YuhinaResult<T>`).
pub type YuhinaResult<T> = std::result::Result<T, YuhinaError>;

impl From<anyhow::Error> for YuhinaError {
    fn from(err: anyhow::Error) -> Self {
        YuhinaError::internal(err.to_string())
    }
}

impl From<std::io::Error> for YuhinaError {
    fn from(err: std::io::Error) -> Self {
        YuhinaError::io(err.to_string())
    }
}

impl From<serde_json::Error> for YuhinaError {
    fn from(err: serde_json::Error) -> Self {
        YuhinaError::internal(format!("serialization error: {err}"))
    }
}
