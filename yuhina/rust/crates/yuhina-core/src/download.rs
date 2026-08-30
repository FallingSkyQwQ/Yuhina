//! Download abstraction.
//!
//! Core never performs its own HTTP in a hard-coded way: everything goes
//! through the [`Downloader`] trait. Agent B's `DownloadManager`/`mirror`
//! will provide a full-featured implementation (progress, resume, mirror
//! health); until it lands, [`HttpDownloader`] is a working fallback built on
//! reqwest + sha1 verification. Swap the implementation by replacing the
//! `Arc<dyn Downloader>` in `YuhinaCore`.

use std::path::Path;

use sha1::{Digest, Sha1};
use tracing::{debug, warn};
use yuhina_api::{Source, YuhinaError, YuhinaErrorKind};

use crate::mirror::rewrite_url;

/// A file that must be downloaded.
#[derive(Debug, Clone)]
pub struct DownloadItem {
    pub url: String,
    /// Path where the file is written (already the final destination).
    pub target_path: String,
    /// Optional expected sha1; verified when present.
    pub sha1: Option<String>,
    pub size: Option<u64>,
}

/// Abstraction over download execution + URL rewriting.
#[async_trait::async_trait]
pub trait Downloader: Send + Sync {
    /// Rewrite `url` according to the configured mirror source.
    fn rewrite(&self, url: &str) -> String;

    /// Download `url` to `dest`. Fails (leaving no partial file) on network
    /// errors or sha1 mismatch.
    async fn download(&self, url: &str, dest: &Path, sha1: Option<&str>)
        -> Result<(), YuhinaError>;

    /// Fetch a URL into memory (small payloads: manifest/json/meta).
    async fn fetch_bytes(&self, url: &str) -> Result<Vec<u8>, YuhinaError>;
}

pub fn sha1_hex(data: &[u8]) -> String {
    let digest = Sha1::digest(data);
    hex::encode(digest)
}

pub fn sha1_file(path: &Path) -> Result<String, YuhinaError> {
    let data = std::fs::read(path)
        .map_err(|e| YuhinaError::io(format!("read {}: {e}", path.display())))?;
    Ok(sha1_hex(&data))
}

/// Reqwest-based fallback downloader.
pub struct HttpDownloader {
    client: reqwest::Client,
    source: Source,
}

impl HttpDownloader {
    pub fn new(source: Source) -> Self {
        let client = reqwest::Client::builder()
            .user_agent("yuhina/0.1")
            .build()
            .expect("build reqwest client");
        Self { client, source }
    }

    pub fn source(&self) -> &Source {
        &self.source
    }

    /// Switch the mirror source (config change).
    pub fn set_source(&mut self, source: Source) {
        self.source = source;
    }
}

#[async_trait::async_trait]
impl Downloader for HttpDownloader {
    fn rewrite(&self, url: &str) -> String {
        rewrite_url(&self.source, url)
    }

    async fn download(
        &self,
        url: &str,
        dest: &Path,
        sha1: Option<&str>,
    ) -> Result<(), YuhinaError> {
        let url = self.rewrite(url);
        debug!(url, dest = %dest.display(), "download start");
        let resp = self
            .client
            .get(&url)
            .send()
            .await
            .map_err(|e| YuhinaError::network(format!("GET {url}: {e}")))?;
        if !resp.status().is_success() {
            return Err(YuhinaError::new(
                YuhinaErrorKind::Http(resp.status().as_u16(), url.clone()),
                format!("GET {url} -> HTTP {}", resp.status()),
            ));
        }
        let bytes = resp
            .bytes()
            .await
            .map_err(|e| YuhinaError::network(format!("read body {url}: {e}")))?;
        verify_and_write(dest, &bytes, sha1)
    }

    async fn fetch_bytes(&self, url: &str) -> Result<Vec<u8>, YuhinaError> {
        let url = self.rewrite(url);
        let resp = self
            .client
            .get(&url)
            .send()
            .await
            .map_err(|e| YuhinaError::network(format!("GET {url}: {e}")))?;
        if !resp.status().is_success() {
            return Err(YuhinaError::new(
                YuhinaErrorKind::Http(resp.status().as_u16(), url.clone()),
                format!("GET {url} -> HTTP {}", resp.status()),
            ));
        }
        resp.bytes()
            .await
            .map(|b| b.to_vec())
            .map_err(|e| YuhinaError::network(format!("read body {url}: {e}")))
    }
}

/// Write bytes to `dest`, verifying sha1 when provided. Removes the file on
/// mismatch so a failed download never looks complete.
pub fn verify_and_write(dest: &Path, bytes: &[u8], sha1: Option<&str>) -> Result<(), YuhinaError> {
    if let Some(expected) = sha1 {
        let actual = sha1_hex(bytes);
        if !actual.eq_ignore_ascii_case(expected) {
            let _ = std::fs::remove_file(dest);
            warn!(dest = %dest.display(), "sha1 mismatch");
            return Err(YuhinaError::new(
                YuhinaErrorKind::ChecksumMismatch,
                format!(
                    "sha1 mismatch for {}: expected {expected}, got {actual}",
                    dest.display()
                ),
            ));
        }
    }
    if let Some(parent) = dest.parent() {
        std::fs::create_dir_all(parent)
            .map_err(|e| YuhinaError::io(format!("mkdir {}: {e}", parent.display())))?;
    }
    std::fs::write(dest, bytes)
        .map_err(|e| YuhinaError::io(format!("write {}: {e}", dest.display())))?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sha1_hex_known_vector() {
        // echo -n "abc" | sha1sum
        assert_eq!(sha1_hex(b"abc"), "a9993e364706816aba3e25717850c26c9cd0d89d");
    }

    #[test]
    fn verify_and_write_mismatch_removes_file() {
        let dir = tempfile::tempdir().unwrap();
        let dest = dir.path().join("f.bin");
        let err = verify_and_write(&dest, b"data", Some("deadbeef")).unwrap_err();
        assert_eq!(err.kind, YuhinaErrorKind::ChecksumMismatch);
        assert!(!dest.exists());
    }

    #[test]
    fn verify_and_write_ok() {
        let dir = tempfile::tempdir().unwrap();
        let dest = dir.path().join("f.bin");
        verify_and_write(&dest, b"abc", Some(&sha1_hex(b"abc"))).unwrap();
        assert_eq!(std::fs::read(&dest).unwrap(), b"abc");
    }
}
