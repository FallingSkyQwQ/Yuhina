//! Official source ↔ BMCLAPI mirror host mapping (task T1).
//!
//! Only *known official hosts* are rewritten; any unknown host is returned
//! unchanged (safe downgrade). `Source::Custom(prefix)` replaces the origin
//! (scheme + host [+ port]) with a user-provided prefix and expects the
//! mirror to follow BMCLAPI path conventions.

use url::Url;

use yuhina_api::Source;

/// BMCLAPI mirror host used by every rule.
pub const BMCLAPI_HOST: &str = "bmclapi2.bangbang93.com";

/// Official host → optional path prefix on the mirror.
///
/// - `None` keeps the official request path verbatim (most Mojang hosts).
/// - `Some(p)` prepends `/p/` to the path (forge/neoforge/fabric/quilt,
///   which BMCLAPI serves under dedicated path namespaces).
struct HostRule {
    official: &'static str,
    path_prefix: Option<&'static str>,
}

const HOST_RULES: &[HostRule] = &[
    HostRule {
        official: "launchermeta.mojang.com",
        path_prefix: None,
    },
    HostRule {
        official: "launcher.mojang.com",
        path_prefix: None,
    },
    HostRule {
        official: "libraries.mojang.com",
        path_prefix: None,
    },
    HostRule {
        official: "resources.download.minecraft.net",
        path_prefix: None,
    },
    HostRule {
        official: "piston-meta.mojang.com",
        path_prefix: None,
    },
    HostRule {
        official: "api.adoptium.net",
        path_prefix: None,
    },
    HostRule {
        official: "maven.fabricmc.net",
        path_prefix: Some("fabric-maven"),
    },
    HostRule {
        official: "meta.fabricmc.net",
        path_prefix: Some("fabric-meta"),
    },
    HostRule {
        official: "meta.quiltmc.org",
        path_prefix: Some("quilt-meta"),
    },
    HostRule {
        official: "maven.quiltmc.org",
        path_prefix: Some("quilt-maven"),
    },
    HostRule {
        official: "maven.minecraftforge.net",
        path_prefix: Some("forge"),
    },
    HostRule {
        official: "maven.neoforged.net",
        path_prefix: Some("neoforge"),
    },
];

/// Rewrites `input` according to the selected download source.
///
/// - `Source::Official` → unchanged.
/// - `Source::Bmclapi` → host + path-prefix rewrite via the mapping table.
/// - `Source::Custom(prefix)` → origin replaced by `prefix` (a full URL or a
///   bare host); path rewrite still follows the mapping table.
pub fn rewrite_url(input: &str, source: &Source) -> String {
    match source {
        Source::Official => input.to_string(),
        Source::Bmclapi => rewrite_with_origin(input, "https", BMCLAPI_HOST, None),
        Source::Custom(prefix) => rewrite_custom(input, prefix),
    }
}

/// Is `host` a known official host in the mapping table?
pub fn is_official_host(host: &str) -> bool {
    HOST_RULES.iter().any(|r| r.official == host)
}

/// Rewrites a known official URL to `scheme://host[:port]` (BMCLAPI rules).
/// Unknown / malformed URLs are returned unchanged.
fn rewrite_with_origin(input: &str, scheme: &str, host: &str, port: Option<u16>) -> String {
    let Ok(mut url) = Url::parse(input) else {
        return input.to_string();
    };
    let Some(current_host) = url.host_str() else {
        return input.to_string();
    };
    let Some(rule) = HOST_RULES.iter().find(|r| r.official == current_host) else {
        // Unknown host → safe downgrade: leave the URL untouched.
        return input.to_string();
    };
    let _ = url.set_scheme(scheme);
    let _ = url.set_host(Some(host));
    let _ = url.set_port(port);
    if let Some(prefix) = rule.path_prefix {
        let base = url.path().trim_start_matches('/');
        let new_path = if base.is_empty() {
            format!("/{prefix}")
        } else {
            format!("/{prefix}/{base}")
        };
        url.set_path(&new_path);
    }
    url.to_string()
}

fn rewrite_custom(input: &str, prefix: &str) -> String {
    let prefix = prefix.trim().trim_end_matches('/');
    if prefix.is_empty() {
        return rewrite_with_origin(input, "https", BMCLAPI_HOST, None);
    }
    if let Ok(parsed) = Url::parse(prefix) {
        if let Some(host) = parsed.host_str() {
            return rewrite_with_origin(input, parsed.scheme(), host, parsed.port());
        }
    }
    // Treat the prefix as a bare host and keep the https scheme.
    rewrite_with_origin(input, "https", prefix, None)
}

#[cfg(test)]
mod tests {
    use super::*;

    const BMCLAPI: &str = BMCLAPI_HOST;

    fn rewrite(input: &str) -> String {
        rewrite_url(input, &Source::Bmclapi)
    }

    #[test]
    fn official_is_identity() {
        assert_eq!(
            rewrite_url(
                "https://launcher.mojang.com/v1/objects/x/file.jar",
                &Source::Official
            ),
            "https://launcher.mojang.com/v1/objects/x/file.jar"
        );
    }

    #[test]
    fn mojang_hosts_keep_path() {
        let cases = [
            (
                "https://launchermeta.mojang.com/mc/game/version_manifest_v2.json",
                format!("https://{BMCLAPI}/mc/game/version_manifest_v2.json"),
            ),
            (
                "https://launcher.mojang.com/v1/objects/a1b2/1.20.jar",
                format!("https://{BMCLAPI}/v1/objects/a1b2/1.20.jar"),
            ),
            (
                "https://libraries.mojang.com/net/minecraft/1.0/1.0.jar",
                format!("https://{BMCLAPI}/net/minecraft/1.0/1.0.jar"),
            ),
            (
                "https://resources.download.minecraft.net/ab/abcdef0123",
                format!("https://{BMCLAPI}/ab/abcdef0123"),
            ),
            (
                "https://piston-meta.mojang.com/mc/game/version_manifest_v2.json",
                format!("https://{BMCLAPI}/mc/game/version_manifest_v2.json"),
            ),
            (
                "https://api.adoptium.net/v3/binary/latest/21/ga/linux/x64/jdk/hotspot/normal/eclipse",
                format!("https://{BMCLAPI}/v3/binary/latest/21/ga/linux/x64/jdk/hotspot/normal/eclipse"),
            ),
        ];
        for (input, expected) in cases {
            assert_eq!(rewrite(input), expected, "rewriting {input}");
        }
    }

    #[test]
    fn path_prefixed_hosts() {
        assert_eq!(
            rewrite("https://maven.minecraftforge.net/net/minecraftforge/forge/1.20.4/forge.jar"),
            format!("https://{BMCLAPI}/forge/net/minecraftforge/forge/1.20.4/forge.jar")
        );
        assert_eq!(
            rewrite("https://maven.neoforged.net/net/neoforged/neoforge/20.4/neoforge.jar"),
            format!("https://{BMCLAPI}/neoforge/net/neoforged/neoforge/20.4/neoforge.jar")
        );
        assert_eq!(
            rewrite("https://maven.fabricmc.net/net/fabricmc/fabric-loader/0.15.0/loader.jar"),
            format!("https://{BMCLAPI}/fabric-maven/net/fabricmc/fabric-loader/0.15.0/loader.jar")
        );
        assert_eq!(
            rewrite("https://meta.fabricmc.net/v2/versions/loader/1.20.4"),
            format!("https://{BMCLAPI}/fabric-meta/v2/versions/loader/1.20.4")
        );
        assert_eq!(
            rewrite("https://maven.quiltmc.org/repository/release/org/quiltmc/quilt-loader/0.26/loader.jar"),
            format!("https://{BMCLAPI}/quilt-maven/repository/release/org/quiltmc/quilt-loader/0.26/loader.jar")
        );
        assert_eq!(
            rewrite("https://meta.quiltmc.org/v3/versions/loader/1.20.4"),
            format!("https://{BMCLAPI}/quilt-meta/v3/versions/loader/1.20.4")
        );
    }

    #[test]
    fn modrinth_not_rewritten() {
        // Modrinth has no mirror → unknown host → unchanged.
        assert_eq!(
            rewrite("https://api.modrinth.com/v2/project/abcdef"),
            "https://api.modrinth.com/v2/project/abcdef"
        );
    }

    #[test]
    fn unknown_host_safe_downgrade() {
        assert_eq!(
            rewrite("https://cdn.unknown.example.com/file.jar"),
            "https://cdn.unknown.example.com/file.jar"
        );
    }

    #[test]
    fn malformed_url_unchanged() {
        assert_eq!(rewrite("not a url"), "not a url");
        // Valid URL with a known host still gets rewritten (mirror serves https).
        assert_eq!(
            rewrite("ftp://launcher.mojang.com/x"),
            "https://bmclapi2.bangbang93.com/x"
        );
        // Unparseable host-less input stays as-is.
        assert_eq!(rewrite("//no-scheme/x"), "//no-scheme/x");
    }

    #[test]
    fn custom_replaces_origin_only() {
        let src = Source::Custom("https://mirror.example.com".into());
        assert_eq!(
            rewrite_url(
                "https://libraries.mojang.com/net/minecraft/1.0/1.0.jar",
                &src
            ),
            "https://mirror.example.com/net/minecraft/1.0/1.0.jar"
        );
        // Path prefixes still follow BMCLAPI conventions.
        assert_eq!(
            rewrite_url(
                "https://maven.minecraftforge.net/net/minecraftforge/forge/x.jar",
                &src
            ),
            "https://mirror.example.com/forge/net/minecraftforge/forge/x.jar"
        );
        // Unknown host with custom source → unchanged.
        assert_eq!(
            rewrite_url("https://example.com/other", &src),
            "https://example.com/other"
        );
    }

    #[test]
    fn custom_bare_host_prefix() {
        let src = Source::Custom("mirror.example.com".into());
        assert_eq!(
            rewrite_url(
                "https://piston-meta.mojang.com/mc/game/version_manifest_v2.json",
                &src
            ),
            "https://mirror.example.com/mc/game/version_manifest_v2.json"
        );
    }

    #[test]
    fn all_known_hosts_mapped() {
        let known = [
            "launchermeta.mojang.com",
            "launcher.mojang.com",
            "libraries.mojang.com",
            "resources.download.minecraft.net",
            "piston-meta.mojang.com",
            "api.adoptium.net",
            "maven.fabricmc.net",
            "meta.fabricmc.net",
            "meta.quiltmc.org",
            "maven.quiltmc.org",
            "maven.minecraftforge.net",
            "maven.neoforged.net",
        ];
        for host in known {
            let out = rewrite(&format!("https://{host}/path/to/file"));
            assert!(
                out.starts_with(&format!("https://{BMCLAPI}/")),
                "{host} → {out}"
            );
            assert!(is_official_host(host));
        }
        assert!(!is_official_host("api.modrinth.com"));
    }
}
