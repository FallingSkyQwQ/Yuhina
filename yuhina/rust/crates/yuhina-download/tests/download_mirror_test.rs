//! Mirror rewriting integration tests (official ↔ BMCLAPI / custom).

mod common;

use yuhina_api::Source;
use yuhina_download::rewrite_url;

#[test]
fn official_source_is_identity() {
    let url = "https://launcher.mojang.com/v1/objects/a/b.jar";
    assert_eq!(rewrite_url(url, &Source::Official), url);
}

#[test]
fn bmclapi_maps_common_hosts() {
    let cases = [
        (
            "https://piston-meta.mojang.com/mc/game/version_manifest_v2.json",
            "https://bmclapi2.bangbang93.com/mc/game/version_manifest_v2.json",
        ),
        (
            "https://resources.download.minecraft.net/ab/0123456789abcdef",
            "https://bmclapi2.bangbang93.com/ab/0123456789abcdef",
        ),
        (
            "https://maven.minecraftforge.net/net/minecraftforge/forge/x.jar",
            "https://bmclapi2.bangbang93.com/forge/net/minecraftforge/forge/x.jar",
        ),
        (
            "https://maven.neoforged.net/net/neoforged/neoforge/y.jar",
            "https://bmclapi2.bangbang93.com/neoforge/net/neoforged/neoforge/y.jar",
        ),
    ];
    for (input, expected) in cases {
        assert_eq!(rewrite_url(input, &Source::Bmclapi), expected);
    }
}

#[test]
fn query_string_and_path_are_preserved() {
    let src = Source::Bmclapi;
    let out = rewrite_url("https://api.adoptium.net/v3/binary/latest/21/ga?os=linux&arch=x64", &src);
    assert!(out.starts_with("https://bmclapi2.bangbang93.com/v3/binary/latest/21/ga?"));
    assert!(out.contains("os=linux"));
    assert!(out.contains("arch=x64"));
}

#[test]
fn unknown_host_degrades_safely() {
    let url = "https://cdn.modrinth.com/data/x.jar";
    assert_eq!(rewrite_url(url, &Source::Bmclapi), url);
    assert_eq!(
        rewrite_url(url, &Source::Custom("https://mirror.example.com".into())),
        url
    );
}

#[test]
fn custom_prefix_replaces_origin() {
    let src = Source::Custom("https://mirror.example.com".into());
    assert_eq!(
        rewrite_url("https://libraries.mojang.com/net/minecraft/1.0/1.0.jar", &src),
        "https://mirror.example.com/net/minecraft/1.0/1.0.jar"
    );
}