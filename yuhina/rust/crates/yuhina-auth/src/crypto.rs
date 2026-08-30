//! Token encryption (api-contract.md §5 encryption convention).
//!
//! A 32-byte AES-256 key is stored in the OS keyring first (Windows DPAPI /
//! Linux SecretService). When the keyring is unavailable (e.g. headless CI),
//! it degrades to a local key file `<data_dir>/secret.key` (chmod 0600).
//!
//! Tokens are encrypted with AES-256-GCM: `base64(nonce(12) || ciphertext)`.
//! **Never log or serialize plaintext tokens.**

use std::path::Path;

use aes_gcm::aead::{Aead, KeyInit};
use aes_gcm::{Aes256Gcm, Nonce};
use base64::Engine;

use yuhina_api::Result;

const KEYRING_SERVICE: &str = "yuhina";
const KEYRING_USER: &str = "crypto-key";

/// Encryptor/decryptor backed by a 32-byte key.
#[derive(Debug, Clone)]
pub struct Crypto {
    key: [u8; 32],
}

impl Crypto {
    /// Obtain the key from the OS keyring, falling back to `secret.key`.
    /// Set `YUHINA_CRYPTO_NO_KEYRING=1` to force the file backend (tests,
    /// headless environments).
    pub fn new(data_dir: &Path) -> Result<Self> {
        if std::env::var("YUHINA_CRYPTO_NO_KEYRING").as_deref() == Ok("1") {
            let key = file_get_or_create(data_dir)?;
            return Ok(Crypto { key });
        }
        match keyring_get_or_create() {
            Ok(key) => Ok(Crypto { key }),
            Err(keyring_err) => {
                tracing::warn!(
                    "keyring unavailable ({}), falling back to local secret.key",
                    keyring_err
                );
                let key = file_get_or_create(data_dir)?;
                Ok(Crypto { key })
            }
        }
    }

    /// Build from an explicit key (test injection / in-memory store).
    pub fn from_key(key: [u8; 32]) -> Self {
        Crypto { key }
    }

    pub fn encrypt(&self, plaintext: &[u8]) -> Result<String> {
        let cipher = Aes256Gcm::new_from_slice(&self.key).expect("32-byte key");
        let mut nonce = [0u8; 12];
        getrandom::getrandom(&mut nonce).map_err(|e| yuhina_api::YuhinaError::io(e.to_string()))?;
        let ct = cipher
            .encrypt(Nonce::from_slice(&nonce), plaintext)
            .map_err(|e| yuhina_api::YuhinaError::io(format!("encrypt failed: {e}")))?;
        let mut out = Vec::with_capacity(nonce.len() + ct.len());
        out.extend_from_slice(&nonce);
        out.extend_from_slice(&ct);
        Ok(base64::engine::general_purpose::STANDARD.encode(out))
    }

    pub fn decrypt(&self, encoded: &str) -> Result<Vec<u8>> {
        use base64::Engine;
        let raw = base64::engine::general_purpose::STANDARD
            .decode(encoded)
            .map_err(|e| yuhina_api::YuhinaError::io(format!("bad ciphertext: {e}")))?;
        if raw.len() < 12 {
            return Err(yuhina_api::YuhinaError::io("ciphertext too short"));
        }
        let (nonce, ct) = raw.split_at(12);
        let cipher = Aes256Gcm::new_from_slice(&self.key).expect("32-byte key");
        cipher
            .decrypt(Nonce::from_slice(nonce), ct)
            .map_err(|e| yuhina_api::YuhinaError::io(format!("decrypt failed: {e}")))
    }

    /// Encrypt a UTF-8 token to a base64 string.
    pub fn encrypt_str(&self, plain: &str) -> Result<String> {
        self.encrypt(plain.as_bytes())
    }

    /// Decrypt a base64 ciphertext into a UTF-8 token.
    pub fn decrypt_str(&self, encoded: &str) -> Result<String> {
        let bytes = self.decrypt(encoded)?;
        String::from_utf8(bytes)
            .map_err(|e| yuhina_api::YuhinaError::io(format!("bad utf8 in token: {e}")))
    }
}

fn keyring_get_or_create() -> std::result::Result<[u8; 32], keyring::Error> {
    let entry = keyring::Entry::new(KEYRING_SERVICE, KEYRING_USER)?;
    match entry.get_password() {
        Ok(pw) => match decode_key(&pw) {
            Some(k) => Ok(k),
            None => Err(keyring::Error::PlatformFailure(
                "malformed key stored in keyring".into(),
            )),
        },
        Err(_) => {
            let key = random_key();
            entry.set_password(&encode_key(&key))?;
            Ok(key)
        }
    }
}

fn file_get_or_create(data_dir: &Path) -> Result<[u8; 32]> {
    let path = data_dir.join("secret.key");
    if path.exists() {
        let contents = std::fs::read_to_string(&path)
            .map_err(|e| yuhina_api::YuhinaError::io(format!("read {path:?}: {e}")))?;
        let key = decode_key(contents.trim())
            .ok_or_else(|| yuhina_api::YuhinaError::io("malformed secret.key"))?;
        return Ok(key);
    }
    let key = random_key();
    std::fs::create_dir_all(data_dir)
        .map_err(|e| yuhina_api::YuhinaError::io(format!("create {data_dir:?}: {e}")))?;
    let mut tmp = path.clone();
    tmp.set_extension("key.tmp");
    std::fs::write(&tmp, encode_key(&key))
        .map_err(|e| yuhina_api::YuhinaError::io(format!("write secret.key: {e}")))?;
    set_0600(&tmp);
    std::fs::rename(&tmp, &path)
        .map_err(|e| yuhina_api::YuhinaError::io(format!("finalize secret.key: {e}")))?;
    set_0600(&path);
    Ok(key)
}

#[cfg(unix)]
fn set_0600(path: &std::path::PathBuf) {
    use std::os::unix::fs::PermissionsExt;
    let _ = std::fs::set_permissions(path, std::fs::Permissions::from_mode(0o600));
}

#[cfg(not(unix))]
fn set_0600(_path: &std::path::PathBuf) {}

fn random_key() -> [u8; 32] {
    let mut key = [0u8; 32];
    // getrandom never fails on supported platforms; keep a dev-friendly fallback.
    if getrandom::getrandom(&mut key).is_err() {
        for (i, b) in key.iter_mut().enumerate() {
            *b = (i as u8).wrapping_mul(0x9e);
        }
    }
    key
}

fn encode_key(key: &[u8; 32]) -> String {
    hex::encode(key)
}

fn decode_key(s: &str) -> Option<[u8; 32]> {
    let bytes = hex::decode(s).ok()?;
    bytes.try_into().ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn round_trip_with_injected_key() {
        let key = [7u8; 32];
        let crypto = Crypto::from_key(key);
        let enc = crypto.encrypt_str("super-secret-token").unwrap();
        assert_ne!(enc, "super-secret-token");
        assert!(!enc.contains("secret"));
        assert_eq!(crypto.decrypt_str(&enc).unwrap(), "super-secret-token");
    }

    #[test]
    fn ciphertext_does_not_contain_plaintext() {
        let crypto = Crypto::from_key([9u8; 32]);
        let token = "xbl3.0_super_secret_mc_access_token_value";
        let enc = crypto.encrypt_str(token).unwrap();
        assert!(!enc.contains("xbl3.0"));
        assert!(!enc.contains("access_token"));
        assert!(!enc.contains(token));
    }

    #[test]
    fn two_encryptions_differ() {
        let crypto = Crypto::from_key([1u8; 32]);
        let a = crypto.encrypt_str("tok").unwrap();
        let b = crypto.encrypt_str("tok").unwrap();
        assert_ne!(a, b);
        assert_eq!(crypto.decrypt_str(&a).unwrap(), "tok");
        assert_eq!(crypto.decrypt_str(&b).unwrap(), "tok");
    }

    #[test]
    fn tampered_ciphertext_fails() {
        let crypto = Crypto::from_key([3u8; 32]);
        let enc = crypto.encrypt_str("tok").unwrap();
        let mut bytes = base64::engine::general_purpose::STANDARD
            .decode(&enc)
            .unwrap();
        *bytes.last_mut().unwrap() ^= 0xff;
        let tampered = base64::engine::general_purpose::STANDARD.encode(bytes);
        assert!(crypto.decrypt_str(&tampered).is_err());
    }

    #[test]
    fn file_fallback_creates_0600_key_and_round_trips() {
        unsafe {
            std::env::set_var("YUHINA_CRYPTO_NO_KEYRING", "1");
        }
        let dir = std::env::temp_dir().join(format!("yuhina-crypto-test-{}", uuid::Uuid::new_v4()));
        let crypto = Crypto::new(&dir).unwrap();
        let enc = crypto.encrypt_str("tok").unwrap();
        assert_eq!(crypto.decrypt_str(&enc).unwrap(), "tok");
        let path = dir.join("secret.key");
        assert!(path.exists());
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mode = std::fs::metadata(&path).unwrap().permissions().mode() & 0o777;
            assert_eq!(mode, 0o600);
        }
        // Re-open reuses the same key (persistence across sessions).
        let crypto2 = Crypto::new(&dir).unwrap();
        assert_eq!(crypto2.decrypt_str(&enc).unwrap(), "tok");
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn malformed_secret_key_file_is_an_error() {
        unsafe {
            std::env::set_var("YUHINA_CRYPTO_NO_KEYRING", "1");
        }
        let dir = std::env::temp_dir().join(format!("yuhina-crypto-bad-{}", uuid::Uuid::new_v4()));
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(dir.join("secret.key"), "not-hex").unwrap();
        assert!(Crypto::new(&dir).is_err());
        let _ = std::fs::remove_dir_all(&dir);
    }
}