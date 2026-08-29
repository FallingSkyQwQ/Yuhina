//! Error model for Yuhina (see api-contract.md §1).

use std::fmt;

/// Categorised error kind. `Http(status, url)` carries the HTTP status code
/// and the failing URL for diagnostics.
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

/// The single error type returned by every fallible `yuhina-bridge` method.
/// Mapped by flutter_rust_bridge to Dart `YuhinaError` with `kind` + `message`.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize, thiserror::Error)]
#[error("{message}")]
pub struct YuhinaError {
    pub kind: YuhinaErrorKind,
    pub message: String,
}

pub type YuhinaResult<T> = Result<T, YuhinaError>;

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

    pub fn io(message: impl Into<String>) -> Self {
        Self::new(YuhinaErrorKind::Io, message)
    }

    pub fn internal(message: impl Into<String>) -> Self {
        Self::new(YuhinaErrorKind::Internal, message)
    }

    pub fn not_found_version(id: &str) -> Self {
        Self::new(
            YuhinaErrorKind::VersionNotFound,
            format!("Minecraft version '{id}' not found"),
        )
    }

    pub fn java_not_found(message: impl Into<String>) -> Self {
        Self::new(YuhinaErrorKind::JavaNotFound, message)
    }

    pub fn download_failed(message: impl Into<String>) -> Self {
        Self::new(YuhinaErrorKind::DownloadFailed, message)
    }

    pub fn checksum_mismatch(message: impl Into<String>) -> Self {
        Self::new(YuhinaErrorKind::ChecksumMismatch, message)
    }

    pub fn canceled(message: impl Into<String>) -> Self {
        Self::new(YuhinaErrorKind::Canceled, message)
    }

    pub fn loader_not_installed(message: impl Into<String>) -> Self {
        Self::new(YuhinaErrorKind::LoaderNotInstalled, message)
    }

    pub fn version_not_found(message: impl Into<String>) -> Self {
        Self::new(YuhinaErrorKind::VersionNotFound, message)
    }
}

impl fmt::Display for YuhinaErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            YuhinaErrorKind::Network => write!(f, "Network"),
            YuhinaErrorKind::Http(status, url) => write!(f, "Http({status}, {url})"),
            YuhinaErrorKind::Auth => write!(f, "Auth"),
            YuhinaErrorKind::AuthExpired => write!(f, "AuthExpired"),
            YuhinaErrorKind::NotLoggedIn => write!(f, "NotLoggedIn"),
            YuhinaErrorKind::VersionNotFound => write!(f, "VersionNotFound"),
            YuhinaErrorKind::LoaderNotInstalled => write!(f, "LoaderNotInstalled"),
            YuhinaErrorKind::JavaNotFound => write!(f, "JavaNotFound"),
            YuhinaErrorKind::InvalidInstance => write!(f, "InvalidInstance"),
            YuhinaErrorKind::ModConflict => write!(f, "ModConflict"),
            YuhinaErrorKind::ModpackInvalid => write!(f, "ModpackInvalid"),
            YuhinaErrorKind::ChecksumMismatch => write!(f, "ChecksumMismatch"),
            YuhinaErrorKind::DownloadFailed => write!(f, "DownloadFailed"),
            YuhinaErrorKind::Canceled => write!(f, "Canceled"),
            YuhinaErrorKind::Io => write!(f, "Io"),
            YuhinaErrorKind::Internal => write!(f, "Internal"),
        }
    }
}

impl From<anyhow::Error> for YuhinaError {
    fn from(err: anyhow::Error) -> Self {
        YuhinaError::new(YuhinaErrorKind::Internal, err.to_string())
    }
}

impl From<std::io::Error> for YuhinaError {
    fn from(err: std::io::Error) -> Self {
        YuhinaError::io(err.to_string())
    }
}

impl From<serde_json::Error> for YuhinaError {
    fn from(err: serde_json::Error) -> Self {
        YuhinaError::internal(format!("JSON error: {err}"))
    }
}

impl From<YuhinaErrorKind> for YuhinaError {
    fn from(kind: YuhinaErrorKind) -> Self {
        Self {
            message: kind.to_string(),
            kind,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serde_roundtrip() {
        let e = YuhinaError::new(
            YuhinaErrorKind::Http(404, String::from("https://example.com/x.json")),
            String::from("HTTP 404"),
        );
        let json = serde_json::to_string(&e).unwrap();
        let back: YuhinaError = serde_json::from_str(&json).unwrap();
        assert_eq!(e, back);
    }

    #[test]
    fn anyhow_mapping() {
        let e: YuhinaError = anyhow::anyhow!("boom").into();
        assert_eq!(e.kind, YuhinaErrorKind::Internal);
        assert_eq!(e.message, "boom");
    }

    #[test]
    fn kind_display() {
        assert_eq!(
            YuhinaErrorKind::Http(500, "u".into()).to_string(),
            "Http(500, u)"
        );
    }
}