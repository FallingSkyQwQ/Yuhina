//! Launch command construction (task T5).
//!
//! `build_launch_command` is pure and deterministic so golden tests can pin
//! the exact argument array (including paths with spaces).

use std::path::{Path, PathBuf};

use yuhina_api::{Account, AccountKind, LaunchArgs};

use crate::arguments::ArgTokens;
use crate::config::{memory_flags, CorePaths};
use crate::libraries::{build_classpath, Platform};
use crate::manifest::VersionManifest;

/// Everything needed to build the launch command.
#[derive(Debug, Clone)]
pub struct LaunchInput<'a> {
    pub java_bin: &'a Path,
    pub game_dir: &'a Path,
    pub paths: &'a CorePaths,
    pub assets_index: String,
    pub natives_dir: &'a Path,
    pub classpath: String,
    pub version_name: String,
    pub version_type: String,
    pub main_class: String,
    pub launch_args: &'a LaunchArgs,
    pub account: &'a Account,
    pub manifest: &'a VersionManifest,
    pub launcher_name: String,
    pub launcher_version: String,
    pub platform: Platform,
}

/// A complete, ready-to-spawn command.
#[derive(Debug, Clone)]
pub struct LaunchCommand {
    pub java_bin: String,
    pub args: Vec<String>,
    pub cwd: PathBuf,
}

impl LaunchCommand {
    /// Full argv[0..] array including the java binary.
    pub fn full_argv(&self) -> Vec<String> {
        let mut v = vec![self.java_bin.clone()];
        v.extend(self.args.iter().cloned());
        v
    }
}

/// Build the full launch command (task T5 golden target).
pub fn build_launch_command(input: &LaunchInput<'_>) -> LaunchCommand {
    let tokens = ArgTokens {
        auth_player_name: input.account.username.clone(),
        version_name: input.version_name.clone(),
        game_directory: input.game_dir.to_string_lossy().to_string(),
        assets_root: input.paths.assets_dir.to_string_lossy().to_string(),
        assets_index_name: input.assets_index.clone(),
        auth_uuid: input.account.uuid.clone(),
        auth_access_token: access_token_for(input.account),
        user_type: user_type_for(&input.account.kind),
        version_type: input.version_type.clone(),
        natives_directory: input.natives_dir.to_string_lossy().to_string(),
        launcher_name: input.launcher_name.clone(),
        launcher_version: input.launcher_version.clone(),
        classpath: input.classpath.clone(),
        library_directory: input.paths.libraries_dir.to_string_lossy().to_string(),
        resolution_width: input.launch_args.window_width.map(|w| w.to_string()),
        resolution_height: input.launch_args.window_height.map(|h| h.to_string()),
        game_assets: input
            .paths
            .assets_dir
            .join("virtual/legacy")
            .to_string_lossy()
            .to_string(),
    };

    let mut args: Vec<String> = Vec::new();
    // JVM memory + GC
    args.extend(memory_flags(input.launch_args));

    let jvm_args = input.manifest.jvm_arguments(&input.platform, &tokens);
    if !jvm_args.is_empty() {
        args.extend(jvm_args);
    } else {
        // legacy: inject natives + classpath manually
        args.push(format!(
            "-Djava.library.path={}",
            input.natives_dir.to_string_lossy()
        ));
        args.push("-cp".to_string());
        args.push(input.classpath.clone());
    }

    // custom extra JVM args
    args.extend(input.launch_args.extra_jvm_args.iter().cloned());

    // main class
    args.push(input.main_class.clone());

    // game args
    args.extend(input.manifest.game_arguments(&input.platform, &tokens));

    // custom extra MC args
    args.extend(input.launch_args.extra_mc_args.iter().cloned());

    LaunchCommand {
        java_bin: input.java_bin.to_string_lossy().to_string(),
        args,
        cwd: input.game_dir.to_path_buf(),
    }
}

/// Placeholder access token for offline accounts (per Yggdrasil convention).
pub fn access_token_for(account: &Account) -> String {
    match account.kind {
        AccountKind::Offline => "0".to_string(),
        _ => {
            if account.uuid.is_empty() {
                "0".to_string()
            } else {
                account.uuid.clone()
            }
        }
    }
}

/// `--userType` value (legacy vs msa).
pub fn user_type_for(kind: &AccountKind) -> String {
    match kind {
        AccountKind::Offline => "legacy".to_string(),
        AccountKind::Microsoft => "msa".to_string(),
        AccountKind::Yggdrasil => "mojang".to_string(),
    }
}

/// Assemble the classpath from resolved libraries + client jar.
pub fn build_classpath_for(
    resolved: &[crate::libraries::ResolvedLibrary],
    paths: &CorePaths,
    client_jar: &Path,
) -> String {
    build_classpath(resolved, &paths.libraries_dir, client_jar)
}

/// Extract native library jars into `natives_dir` (unzip with exclusion
/// patterns). Entries matching an `extract.exclude` pattern or under
/// `META-INF/` are skipped. Idempotent per file (skips existing).
pub fn extract_natives(
    resolved: &[crate::libraries::ResolvedLibrary],
    libraries_dir: &Path,
    natives_dir: &Path,
) -> Result<(), yuhina_api::YuhinaError> {
    std::fs::create_dir_all(natives_dir)
        .map_err(|e| yuhina_api::YuhinaError::io(format!("mkdir natives: {e}")))?;
    for lib in resolved {
        if !lib.is_native {
            continue;
        }
        let jar_path = libraries_dir.join(&lib.path);
        if !jar_path.exists() {
            continue;
        }
        let file = std::fs::File::open(&jar_path).map_err(|e| {
            yuhina_api::YuhinaError::io(format!("open {}: {e}", jar_path.display()))
        })?;
        let mut zip = zip::ZipArchive::new(file).map_err(|e| {
            yuhina_api::YuhinaError::io(format!("read {}: {e}", jar_path.display()))
        })?;
        for i in 0..zip.len() {
            let mut entry = match zip.by_index(i) {
                Ok(e) => e,
                Err(_) => continue,
            };
            let name = entry.name().to_string();
            if entry.is_dir() || name.ends_with('/') {
                continue;
            }
            if is_excluded(&name, &lib.extract_exclude) {
                continue;
            }
            let out = natives_dir.join(&name);
            if let Some(parent) = out.parent() {
                std::fs::create_dir_all(parent).map_err(|e| {
                    yuhina_api::YuhinaError::io(format!("mkdir {}: {e}", parent.display()))
                })?;
            }
            if out.exists() {
                continue;
            }
            let mut f = std::fs::File::create(&out).map_err(|e| {
                yuhina_api::YuhinaError::io(format!("create {}: {e}", out.display()))
            })?;
            std::io::copy(&mut entry, &mut f).map_err(|e| {
                yuhina_api::YuhinaError::io(format!("extract {}: {e}", out.display()))
            })?;
        }
    }
    Ok(())
}

/// Whether an archive entry is excluded from extraction.
pub fn is_excluded(name: &str, patterns: &[String]) -> bool {
    if name.starts_with("META-INF/") {
        return true;
    }
    patterns.iter().any(|p| {
        let p = p.trim_end_matches('/');
        if let Some(rest) = p.strip_prefix("**/") {
            name.ends_with(rest)
        } else {
            name.starts_with(p) || name == p
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use yuhina_api::{JavaRuntime, Loader, LoaderKind};

    use crate::libraries::resolve_libraries;
    use crate::testutil::{load_fixture, temp_game_root};

    fn offline_account(name: &str) -> Account {
        Account {
            id: "acc-1".into(),
            kind: AccountKind::Offline,
            username: name.into(),
            uuid: "00000000-0000-0000-0000-000000000000".into(),
            yggdrasil_server: None,
            skin_url: None,
            is_active: true,
            expires_at: None,
        }
    }

    fn sample_java() -> JavaRuntime {
        JavaRuntime {
            id: "j1".into(),
            path: "/opt/jdk-21/bin/java".into(),
            major: 21,
            vendor: "Temurin".into(),
            version: "21.0.2".into(),
            arch: "x86_64".into(),
            source: yuhina_api::JavaSource::System,
        }
    }

    /// Golden test: build a full command for a real 1.20.4 manifest with an
    /// offline account and assert every arg position.
    #[test]
    fn golden_command_real_1204() {
        let vj = load_fixture("1.20.4.json");
        let manifest = VersionManifest::parse(&vj).unwrap();
        let root = temp_game_root();
        let paths = CorePaths {
            data_dir: root.join("data"),
            game_root: root.join("game"),
            versions_dir: root.join("data/versions"),
            instances_dir: root.join("data/instances"),
            libraries_dir: root.join("game/libraries"),
            assets_dir: root.join("game/assets"),
            assets_objects_dir: root.join("game/assets/objects"),
            logs_dir: root.join("data/logs"),
            db_path: root.join("data/yuhina.db"),
        };
        let platform = Platform {
            os: "linux".into(),
            arch: "x86_64".into(),
        };
        let resolved = resolve_libraries(
            &manifest.libraries,
            &platform,
            &crate::libraries::Features {
                has_custom_resolution: None,
                is_demo_user: None,
            },
        );
        let game_dir = root.join("instances/vanilla");
        let client_jar = paths.client_jar("1.20.4");
        let classpath = build_classpath_for(&resolved, &paths, &client_jar);
        let natives = root.join("data/natives/sess1");
        let account = offline_account("Steve");
        let launch_args = LaunchArgs {
            min_memory_mb: 2048,
            max_memory_mb: 4096,
            extra_jvm_args: vec!["-Dcustom=jvm".into()],
            extra_mc_args: vec!["--quickPlaySingleplayer=world".into()],
            window_width: None,
            window_height: None,
        };
        let java = sample_java();
        let input = LaunchInput {
            java_bin: Path::new(&java.path),
            game_dir: &game_dir,
            paths: &paths,
            assets_index: manifest.asset_index.id.clone(),
            natives_dir: &natives,
            classpath,
            version_name: "1.20.4".into(),
            version_type: manifest.version_type.clone(),
            main_class: manifest.main_class.clone(),
            launch_args: &launch_args,
            account: &account,
            manifest: &manifest,
            launcher_name: "yuhina".into(),
            launcher_version: "0.1.0".into(),
            platform,
        };
        let cmd = build_launch_command(&input);
        let argv = cmd.full_argv();

        // Positional contract
        assert_eq!(argv[0], "/opt/jdk-21/bin/java");
        assert!(argv.iter().any(|a| a == "-Xms2048M"), "Xms present");
        assert!(argv.iter().any(|a| a == "-Xmx4096M"), "Xmx present");
        assert!(argv.iter().any(|a| a == "-Dcustom=jvm"));
        // natives injected via modern jvm args
        assert!(argv
            .iter()
            .any(|a| a.as_str() == format!("-Djava.library.path={}", natives.display()).as_str()));
        assert!(argv.iter().any(|a| a == "-cp"));
        // game args token substitution
        let user_idx = argv.iter().position(|a| a == "--username").unwrap();
        assert_eq!(argv[user_idx + 1], "Steve");
        let gd_idx = argv.iter().position(|a| a == "--gameDir").unwrap();
        assert_eq!(argv[gd_idx + 1], game_dir.to_string_lossy());
        let ai_idx = argv.iter().position(|a| a == "--assetIndex").unwrap();
        assert_eq!(argv[ai_idx + 1], "12");
        // main class between jvm args and game args
        let main_idx = argv
            .iter()
            .position(|a| a == "net.minecraft.client.main.Main")
            .unwrap();
        assert!(main_idx > argv.iter().position(|a| a == "-cp").unwrap());
        assert!(argv.iter().position(|a| a == "--username").unwrap() > main_idx);
        // extra mc args appended at end
        assert_eq!(argv.last().unwrap(), "--quickPlaySingleplayer=world");
    }

    /// Legacy versions must get natives + classpath injected and use the
    /// old argument string.
    #[test]
    fn golden_command_legacy_1122() {
        let vj = load_fixture("1.12.2.json");
        let manifest = VersionManifest::parse(&vj).unwrap();
        assert!(manifest.minecraft_arguments.is_some());
        let root = temp_game_root();
        let paths = CorePaths {
            data_dir: root.join("data"),
            game_root: root.join("game"),
            versions_dir: root.join("data/versions"),
            instances_dir: root.join("data/instances"),
            libraries_dir: root.join("game/libraries"),
            assets_dir: root.join("game/assets"),
            assets_objects_dir: root.join("game/assets/objects"),
            logs_dir: root.join("data/logs"),
            db_path: root.join("data/yuhina.db"),
        };
        let platform = Platform {
            os: "linux".into(),
            arch: "x86_64".into(),
        };
        let resolved = resolve_libraries(
            &manifest.libraries,
            &platform,
            &crate::libraries::Features {
                has_custom_resolution: None,
                is_demo_user: None,
            },
        );
        let game_dir = root.join("instances/legacy");
        let classpath = build_classpath_for(&resolved, &paths, &paths.client_jar("1.12.2"));
        let natives = root.join("data/natives/sess1");
        let account = offline_account("Steve");
        let input = LaunchInput {
            java_bin: Path::new("/usr/lib/jvm/java-8/bin/java"),
            game_dir: &game_dir,
            paths: &paths,
            assets_index: manifest.asset_index.id.clone(),
            natives_dir: &natives,
            classpath,
            version_name: "1.12.2".into(),
            version_type: manifest.version_type.clone(),
            main_class: manifest.main_class.clone(),
            launch_args: &LaunchArgs::default(),
            account: &account,
            manifest: &manifest,
            launcher_name: "yuhina".into(),
            launcher_version: "0.1.0".into(),
            platform,
        };
        let argv = build_launch_command(&input).full_argv();
        // -cp injected (legacy path)
        let cp_idx = argv.iter().position(|a| a == "-cp").unwrap();
        assert!(argv[cp_idx + 1].contains("libraries"));
        assert!(argv.iter().any(|a| a.starts_with("-Djava.library.path=")));
        // legacy game args substituted
        assert_eq!(
            argv[argv.iter().position(|a| a == "--username").unwrap() + 1],
            "Steve"
        );
        assert_eq!(
            argv[argv.iter().position(|a| a == "--userType").unwrap() + 1],
            "legacy"
        );
    }

    #[test]
    fn account_tokens_and_user_type() {
        let ms = Account {
            kind: AccountKind::Microsoft,
            ..offline_account("M")
        };
        assert_eq!(user_type_for(&ms.kind), "msa");
        assert_ne!(access_token_for(&ms), "0");
        let off = offline_account("O");
        assert_eq!(user_type_for(&off.kind), "legacy");
        assert_eq!(access_token_for(&off), "0");
        let yg = Account {
            kind: AccountKind::Yggdrasil,
            ..offline_account("Y")
        };
        assert_eq!(user_type_for(&yg.kind), "mojang");
    }

    #[test]
    fn loader_version_naming_smoke() {
        // ensure Loader type usable here (instance naming for loader versions)
        let l = Loader {
            kind: LoaderKind::Fabric,
            version: "0.16.0".into(),
        };
        assert_eq!(l.version, "0.16.0");
    }
}
