//! URL rewriting for download mirrors.
//!
//! Agent B owns the authoritative `yuhina-download::mirror` module. Until it
//! lands, this built-in rewriter covers the common Mojang/BMCLAPI rules so
//! core is never blocked. When B merges, switch `YuhinaCore` to delegate here.

use yuhina_api::Source;

/// Host of the default BMCLAPI mirror.
pub const BMCLAPI_HOST: &str = "https://bmclapi2.bangbang93.com";

/// Rewrite a download URL according to the configured `Source`.
///
/// - `Official`: unchanged.
/// - `Bmclapi`: translate well-known Mojang hosts to the BMCLAPI host.
/// - `Custom(host)`: replace the URL's authority with the custom host,
///   preserving path + query (mirror that keeps the same path layout).
pub fn rewrite_url(source: &Source, url: &str) -> String {
    match source {
        Source::Official => url.to_string(),
        Source::Bmclapi => rewrite_bmclapi(url),
        Source::Custom(host) => rewrite_custom(host, url),
    }
}

fn rewrite_bmclapi(url: &str) -> String {
    for (from, to) in [
        (
            "https://piston-meta.mojang.com/",
            format!("{BMCLAPI_HOST}/"),
        ),
        ("https://launchermeta.mojang.com/", format!("{BMCLAPI_HOST}/")),
        ("https://piston-data.mojang.com/", format!("{BMCLAPI_HOST}/")),
        (
            "https://libraries.minecraft.net/",
            format!("{BMCLAPI_HOST}/libraries/"),
        ),
        (
            "https://resources.download.minecraft.net/",
            format!("{BMCLAPI_HOST}/assets/"),
        ),
        (
            "https://meta.fabricmc.net/",
            format!("{BMCLAPI_HOST}/fabric-meta/"),
        ),
        (
            "https://maven.fabricmc.net/",
            format!("{BMCLAPI_HOST}/maven/"),
        ),
        (
            "https://maven.quiltmc.org/repository/release/",
            format!("{BMCLAPI_HOST}/maven/"),
        ),
        (
            "https://maven.minecraftforge.net/",
            format!("{BMCLAPI_HOST}/maven/"),
        ),
        (
            "https://maven.neoforged.net/releases/",
            format!("{BMCLAPI_HOST}/maven/"),
        ),
        (
            "https://files.minecraftforge.net/",
            format!("{BMCLAPI_HOST}/maven/"),
        ),
        ("https://launcher.mojang.com/", format!("{BMCLAPI_HOST}/")),
    ] {
        if let Some(rest) = url.strip_prefix(from) {
            return format!("{to}{rest}");
        }
    }
    url.to_string()
}

fn rewrite_custom(host: &str, url: &str) -> String {
    let host = host.trim_end_matches('/');
    // Preserve the path (and query) after the authority.
    match url.find("://") {
        Some(colon) => {
            let after_scheme = &url[colon + 3..];
            let slash = after_scheme.find('/').unwrap_or(after_scheme.len());
            let path = &after_scheme[slash..];
            if path.is_empty() {
                format!("{host}/")
            } else {
                format!("{host}{path}")
            }
        }
        None => url.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn official_is_unchanged() {
        let u = "https://piston-meta.mojang.com/mc/game/version_manifest_v2.json";
        assert_eq!(rewrite_url(&Source::Official, u), u);
    }

    #[test]
    fn bmclapi_rewrites_meta() {
        let u = "https://piston-meta.mojang.com/mc/game/version_manifest_v2.json";
        assert_eq!(
            rewrite_url(&Source::Bmclapi, u),
            format!("{BMCLAPI_HOST}/mc/game/version_manifest_v2.json")
        );
    }

    #[test]
    fn bmclapi_rewrites_libraries() {
        let u = "https://libraries.minecraft.net/com/mojang/patchy/2.2.10/patchy-2.2.10.jar";
        assert_eq!(
            rewrite_url(&Source::Bmclapi, u),
            format!("{BMCLAPI_HOST}/libraries/com/mojang/patchy/2.2.10/patchy-2.2.10.jar")
        );
    }

    #[test]
    fn bmclapi_rewrites_assets() {
        let u = "https://resources.download.minecraft.net/ab/abcdef0123456789";
        assert_eq!(
            rewrite_url(&Source::Bmclapi, u),
            format!("{BMCLAPI_HOST}/assets/ab/abcdef0123456789")
        );
    }

    #[test]
    fn custom_host_replaces_authority() {
        let u = "https://libraries.minecraft.net/a/b.jar?x=1";
        assert_eq!(
            rewrite_url(&Source::Custom("https://mirror.example.com".into()), u),
            "https://mirror.example.com/a/b.jar?x=1"
        );
    }

    #[test]
    fn unknown_url_passes_through_on_bmclapi() {
        let u = "https://api.adoptium.net/v3/assets/21";
        assert_eq!(rewrite_url(&Source::Bmclapi, u), u);
    }
}