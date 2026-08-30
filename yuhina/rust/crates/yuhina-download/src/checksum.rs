//! Checksum helpers (sha1).

use std::fs::File;
use std::io::Read;
use std::path::Path;

use sha1::{Digest, Sha1};

/// Lowercase hex sha1 of a byte slice.
pub fn sha1_hex(data: &[u8]) -> String {
    let mut hasher = Sha1::new();
    hasher.update(data);
    hex(&hasher.finalize())
}

/// Lowercase hex sha1 of a file's contents (streamed, 128 KiB buffer).
pub fn sha1_hex_file(path: &Path) -> anyhow::Result<String> {
    let mut file = File::open(path)?;
    let mut hasher = Sha1::new();
    let mut buf = vec![0u8; 128 * 1024];
    loop {
        let n = file.read(&mut buf)?;
        if n == 0 {
            break;
        }
        hasher.update(&buf[..n]);
    }
    Ok(hex(&hasher.finalize()))
}

/// Verifies a file's sha1 against an expected hex string (case-insensitive).
pub fn verify_file(path: &Path, expected: &str) -> anyhow::Result<bool> {
    Ok(sha1_hex_file(path)? == expected.trim().to_ascii_lowercase())
}

fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sha1_known_vector() {
        assert_eq!(sha1_hex(b"abc"), "a9993e364706816aba3e25717850c26c9cd0d89d");
    }

    #[test]
    fn sha1_file_matches_bytes() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("f.bin");
        std::fs::write(&path, b"hello world").unwrap();
        assert_eq!(sha1_hex_file(&path).unwrap(), sha1_hex(b"hello world"));
        assert!(verify_file(&path, "2AAE6C35C94FCFB415DBE95F408B9CE91EE846ED").unwrap());
        assert!(!verify_file(&path, "deadbeef").unwrap());
    }
}
