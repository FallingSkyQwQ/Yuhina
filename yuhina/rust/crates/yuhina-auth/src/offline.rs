//! Offline accounts: standard offline UUID derivation (HMCL/Prism compatible)
//! and username validation (api-contract.md §2.2, 04-agent-auth.md T2).

use yuhina_api::{Account, AccountKind, Result, YuhinaError};

pub const MAX_USERNAME_LEN: usize = 16;

/// Validate an offline username: 1..=16 chars, `[A-Za-z0-9_]`.
pub fn validate_username(name: &str) -> Result<()> {
    let name = name.trim();
    if name.is_empty() {
        return Err(YuhinaError::auth("Username must not be empty."));
    }
    if name.chars().count() > MAX_USERNAME_LEN {
        return Err(YuhinaError::auth(format!(
            "Username must be at most {MAX_USERNAME_LEN} characters."
        )));
    }
    if !name
        .chars()
        .all(|c| c.is_ascii_alphanumeric() || c == '_')
    {
        return Err(YuhinaError::auth(
            "Username may only contain letters, digits and underscore.",
        ));
    }
    Ok(())
}

/// Classic offline UUID: `MD5("OfflinePlayer:" + name)` with version 3 /
/// variant 1 bits set, formatted as a canonical UUID string.
pub fn offline_uuid(name: &str) -> String {
    use md5::{Digest, Md5};
    let name = name.trim();
    let mut hasher = Md5::new();
    hasher.update(format!("OfflinePlayer:{name}"));
    let digest = hasher.finalize();
    let mut b: [u8; 16] = digest.into();
    // RFC 4122 version 3 (name-based MD5) + RFC 4122 variant (RFC 4122).
    b[6] = (b[6] & 0x0f) | 0x30;
    b[8] = (b[8] & 0x3f) | 0x80;
    let h = hex::encode(b);
    format!(
        "{}-{}-{}-{}-{}",
        &h[0..8],
        &h[8..12],
        &h[12..16],
        &h[16..20],
        &h[20..32]
    )
}

/// Build a new offline `Account`. The new account becomes active only when it
/// is the first account (no active account yet).
pub fn build_offline_account(name: &str, make_active: bool) -> Result<Account> {
    validate_username(name)?;
    let name = name.trim().to_string();
    Ok(Account {
        id: uuid::Uuid::new_v4().to_string(),
        kind: AccountKind::Offline,
        username: name.clone(),
        uuid: offline_uuid(&name),
        yggdrasil_server: None,
        skin_url: None,
        is_active: make_active,
        expires_at: None,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn known_offline_uuids() {
        // Reference vectors match Java `UUID.nameUUIDFromBytes("OfflinePlayer:"+name)`
        // (RFC 4122 name-based MD5), the algorithm used by HMCL/Prism/vanilla.
        assert_eq!(
            offline_uuid("Steve"),
            "5627dd98-e6be-3c21-b8a8-e92344183641"
        );
        assert_eq!(
            offline_uuid("Notch"),
            "b50ad385-829d-3141-a216-7e7d7539ba7f"
        );
    }

    #[test]
    fn uuid_is_derived_from_name_not_random() {
        let a = offline_uuid("Alice");
        let b = offline_uuid("Alice");
        assert_eq!(a, b);
        assert_ne!(a, offline_uuid("alice"));
    }

    #[test]
    fn uuid_format_is_canonical() {
        let u = offline_uuid("TestUser");
        let parts: Vec<&str> = u.split('-').collect();
        assert_eq!(parts.len(), 5);
        assert_eq!(parts[0].len(), 8);
        assert_eq!(parts[1].len(), 4);
        assert_eq!(parts[2].len(), 4);
        assert_eq!(parts[3].len(), 4);
        assert_eq!(parts[4].len(), 12);
        assert_eq!(parts[2].chars().next().unwrap(), '3');
    }

    #[test]
    fn validation_rules() {
        assert!(validate_username("Steve").is_ok());
        assert!(validate_username("_abc123_XYZ").is_ok());
        assert!(validate_username("  Player_1 ").is_ok()); // trimmed
        assert!(validate_username("").is_err());
        assert!(validate_username("   ").is_err());
        assert!(validate_username("averylongusername123456").is_err()); // > 16
        assert!(validate_username("bad name!").is_err());
        assert!(validate_username("日本語").is_err());
        assert!(validate_username("no-hyphen").is_err());
    }

    #[test]
    fn offline_account_built_without_tokens() {
        let acc = build_offline_account("Steve", true).unwrap();
        assert_eq!(acc.kind, AccountKind::Offline);
        assert_eq!(acc.username, "Steve");
        assert!(acc.is_active);
        assert!(acc.expires_at.is_none());
        assert!(acc.yggdrasil_server.is_none());
        assert!(acc.skin_url.is_none());
    }
}