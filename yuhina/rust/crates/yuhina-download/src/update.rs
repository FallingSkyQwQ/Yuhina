//! Launcher self-update check (task T5): GitHub Releases latest tag vs the
//! local version. Any network/parse failure degrades to `Ok(None)`.

use serde::Deserialize;

use crate::YuhinaResult;

/// GitHub Releases "latest" API for the Yuhina repository.
pub const DEFAULT_UPDATE_API_URL: &str =
    "https://api.github.com/repos/FallingSkyQwQ/Yuhina/releases/latest";

/// Compares two version strings (`x.y.z`, optional `+build` / `-pre` suffix).
/// Returns `Some(remote)` when the remote is strictly newer, else `None`.
pub fn compare_versions(local: &str, remote: &str) -> Option<String> {
    let l = parse_version(local)?;
    let r = parse_version(remote)?;
    if r > l {
        Some(remote.trim().to_string())
    } else {
        None
    }
}

/// Parses `major.minor.patch` from a version string (ignores pre-release /
/// build metadata). Returns `None` when the shape is unrecognised.
pub fn parse_version(s: &str) -> Option<(u64, u64, u64)> {
    let s = s.trim().trim_start_matches('v');
    let core = s
        .split('+')
        .next()
        .unwrap_or(s)
        .split('-')
        .next()
        .unwrap_or(s);
    let mut parts = core.split('.');
    let major = parts.next()?.trim().parse().ok()?;
    let minor = parts.next().map(|x| x.trim().parse().unwrap_or(0)).unwrap_or(0);
    let patch = parts.next().map(|x| x.trim().parse().unwrap_or(0)).unwrap_or(0);
    Some((major, minor, patch))
}

/// Checks the GitHub Releases API for the latest tag and compares it with
/// `current`. Returns `Ok(Some(latest_tag))` when an update exists, otherwise
/// `Ok(None)` — including for any network/HTTP/parse failure.
pub async fn check_launcher_update(
    client: &reqwest::Client,
    current: &str,
    api_url: &str,
) -> YuhinaResult<Option<String>> {
    let resp = match client
        .get(api_url)
        .header("User-Agent", "Yuhina/0.1")
        .send()
        .await
    {
        Ok(r) => r,
        Err(_) => return Ok(None),
    };
    if !resp.status().is_success() {
        // 404 = no releases yet; any other error also degrades to None.
        return Ok(None);
    }
    let release: GitHubRelease = match resp.json().await {
        Ok(r) => r,
        Err(_) => return Ok(None),
    };
    let tag = release.tag_name.trim_start_matches('v').to_string();
    Ok(compare_versions(current, &tag))
}

#[derive(Deserialize)]
struct GitHubRelease {
    tag_name: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn version_comparison() {
        assert_eq!(compare_versions("0.1.0", "0.2.0").as_deref(), Some("0.2.0"));
        assert_eq!(compare_versions("1.0.0", "1.0.0"), None);
        assert_eq!(compare_versions("0.2.0", "0.1.9"), None);
        assert_eq!(
            compare_versions("0.1.0+1", "0.1.1").as_deref(),
            Some("0.1.1")
        );
        assert_eq!(compare_versions("0.1.0", "0.1.0-alpha"), None);
        assert_eq!(compare_versions("0.9", "0.10.0").as_deref(), Some("0.10.0"));
        assert_eq!(compare_versions("garbage", "0.1.0"), None);
    }

    #[test]
    fn parse_shapes() {
        assert_eq!(parse_version("0.1.0"), Some((0, 1, 0)));
        assert_eq!(parse_version("v0.1.0"), Some((0, 1, 0)));
        assert_eq!(parse_version("1.2"), Some((1, 2, 0)));
        assert_eq!(parse_version("0.1.0+3"), Some((0, 1, 0)));
        assert_eq!(parse_version("abc"), None);
    }
}