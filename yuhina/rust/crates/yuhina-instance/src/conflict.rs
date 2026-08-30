//! Mod conflict detection (task T6).
//!
//! Heuristic, advisory checks per the task book:
//! 1. DuplicateModId (Error), 2. DuplicateFile (Warning),
//! 3. LoaderMismatch (Error), 4. McVersionMismatch (Warning),
//! 5. IncompatibleDependency (Error), 6. MissingDependency (Warning).

use std::collections::HashMap;

use yuhina_api::{
    ConflictKind, ConflictSeverity, InstalledMod, InstanceSummary, ModConflict, ModrinthDependency,
};

use crate::dependency::loader_tag;

/// Supplies the Modrinth dependency list for an installed version.
#[async_trait::async_trait]
pub trait VersionDepProvider: Send + Sync {
    async fn deps_for_version(&self, version_id: &str) -> Vec<ModrinthDependency>;
}

/// A provider backed by the Modrinth client (network). Failures yield an
/// empty list so conflict checks degrade gracefully.
#[derive(Clone)]
pub struct ModrinthDepProvider {
    pub client: crate::modrinth::ModrinthClient,
}

#[async_trait::async_trait]
impl VersionDepProvider for ModrinthDepProvider {
    async fn deps_for_version(&self, version_id: &str) -> Vec<ModrinthDependency> {
        self.client
            .get_version(version_id)
            .await
            .map(|v| v.dependencies)
            .unwrap_or_default()
    }
}

pub struct ConflictChecker;

impl ConflictChecker {
    /// Run all six checks over the installed mods of an instance.
    pub async fn check(
        &self,
        instance: &InstanceSummary,
        mods: &[InstalledMod],
        deps: &dyn VersionDepProvider,
    ) -> Vec<ModConflict> {
        let mut out = Vec::new();
        duplicate_modid(mods, &mut out);
        duplicate_file(mods, &mut out);
        loader_mismatch(instance, mods, &mut out);
        mc_version_mismatch(instance, mods, &mut out);
        dependency_conflicts(mods, deps, &mut out).await;
        out
    }
}

fn duplicate_modid(mods: &[InstalledMod], out: &mut Vec<ModConflict>) {
    let mut by_modid: HashMap<&str, Vec<&InstalledMod>> = HashMap::new();
    for m in mods {
        if !m.modid.is_empty() {
            by_modid.entry(&m.modid).or_default().push(m);
        }
    }
    for (modid, group) in by_modid {
        if group.len() > 1 {
            out.push(ModConflict {
                severity: ConflictSeverity::Error,
                kind: ConflictKind::DuplicateModId,
                message: format!(
                    "multiple mods declare the mod id '{modid}' ({} files)",
                    group.len()
                ),
                related_files: file_names(&group),
            });
        }
    }
}

fn duplicate_file(mods: &[InstalledMod], out: &mut Vec<ModConflict>) {
    let mut by_sha1: HashMap<&str, Vec<&InstalledMod>> = HashMap::new();
    for m in mods {
        by_sha1.entry(&m.sha1).or_default().push(m);
    }
    for (sha1, group) in by_sha1 {
        if group.len() > 1 {
            out.push(ModConflict {
                severity: ConflictSeverity::Warning,
                kind: ConflictKind::DuplicateFile,
                message: format!(
                    "the same file (sha1 {sha1}) is installed {} times",
                    group.len()
                ),
                related_files: file_names(&group),
            });
        }
    }
}

fn loader_mismatch(instance: &InstanceSummary, mods: &[InstalledMod], out: &mut Vec<ModConflict>) {
    for m in mods {
        if m.loaders.is_empty() {
            continue;
        }
        let mismatch = match &instance.loader {
            Some(l) => !m.loaders.iter().any(|x| x == loader_tag(l.kind)),
            None => true, // vanilla instance cannot run loader mods
        };
        if mismatch {
            out.push(ModConflict {
                severity: ConflictSeverity::Error,
                kind: ConflictKind::LoaderMismatch,
                message: format!(
                    "'{}' targets loader(s) {} but the instance uses {}",
                    m.file_name,
                    m.loaders.join(", "),
                    instance
                        .loader
                        .as_ref()
                        .map(|l| loader_tag(l.kind).to_string())
                        .unwrap_or_else(|| "vanilla".into())
                ),
                related_files: vec![m.file_name.clone()],
            });
        }
    }
}

fn mc_version_mismatch(
    instance: &InstanceSummary,
    mods: &[InstalledMod],
    out: &mut Vec<ModConflict>,
) {
    for m in mods {
        if !m.mc_versions.is_empty() && !m.mc_versions.contains(&instance.mc_version) {
            out.push(ModConflict {
                severity: ConflictSeverity::Warning,
                kind: ConflictKind::McVersionMismatch,
                message: format!(
                    "'{}' supports {} but the instance runs {}",
                    m.file_name,
                    m.mc_versions.join(", "),
                    instance.mc_version
                ),
                related_files: vec![m.file_name.clone()],
            });
        }
    }
}

async fn dependency_conflicts(
    mods: &[InstalledMod],
    deps: &dyn VersionDepProvider,
    out: &mut Vec<ModConflict>,
) {
    for m in mods {
        let Some(vid) = &m.version_id else { continue };
        let deps_list = deps.deps_for_version(vid).await;
        for d in deps_list {
            match d.dep_type.as_str() {
                "incompatible" => {
                    if let Some(pid) = &d.project_id {
                        if let Some(other) = mods
                            .iter()
                            .find(|x| x.project_id.as_deref() == Some(pid.as_str()))
                        {
                            out.push(ModConflict {
                                severity: ConflictSeverity::Error,
                                kind: ConflictKind::IncompatibleDependency,
                                message: format!(
                                    "'{}' is incompatible with '{}'",
                                    m.file_name, other.file_name
                                ),
                                related_files: vec![m.file_name.clone(), other.file_name.clone()],
                            });
                        }
                    }
                }
                "required" => {
                    let present = d
                        .project_id
                        .as_deref()
                        .map(|pid| mods.iter().any(|x| x.project_id.as_deref() == Some(pid)))
                        .unwrap_or(false)
                        || d.version_id
                            .as_deref()
                            .map(|vid| mods.iter().any(|x| x.version_id.as_deref() == Some(vid)))
                            .unwrap_or(false);
                    if !present {
                        out.push(ModConflict {
                            severity: ConflictSeverity::Warning,
                            kind: ConflictKind::MissingDependency,
                            message: format!(
                                "'{}' requires a missing dependency ({})",
                                m.file_name,
                                d.project_id
                                    .clone()
                                    .unwrap_or_else(|| "unknown project".into())
                            ),
                            related_files: vec![m.file_name.clone()],
                        });
                    }
                }
                _ => {}
            }
        }
    }
}

fn file_names(mods: &[&InstalledMod]) -> Vec<String> {
    let mut names: Vec<String> = mods.iter().map(|m| m.file_name.clone()).collect();
    names.sort();
    names
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    use yuhina_api::{Loader, LoaderKind};

    fn mod_row(
        sha1: &str,
        name: &str,
        modid: &str,
        project: Option<&str>,
        version: Option<&str>,
    ) -> InstalledMod {
        InstalledMod {
            id: sha1.into(),
            file_name: name.into(),
            file_size: 1,
            sha1: sha1.into(),
            name: name.into(),
            modid: modid.into(),
            description: String::new(),
            loaders: vec![],
            mc_versions: vec![],
            project_id: project.map(String::from),
            version_id: version.map(String::from),
            enabled: true,
            installed_at: 0,
        }
    }

    fn instance(loader: Option<Loader>, mc: &str) -> InstanceSummary {
        InstanceSummary {
            id: "i".into(),
            name: "n".into(),
            icon: "".into(),
            mc_version: mc.into(),
            loader,
            is_installed: true,
            last_launched_at: None,
            mod_count: 0,
            total_size_bytes: 0,
            created_at: 0,
            updated_at: 0,
        }
    }

    struct FakeDeps(HashMap<String, Vec<ModrinthDependency>>);

    #[async_trait::async_trait]
    impl VersionDepProvider for FakeDeps {
        async fn deps_for_version(&self, version_id: &str) -> Vec<ModrinthDependency> {
            self.0.get(version_id).cloned().unwrap_or_default()
        }
    }

    fn dep(project: &str, version: Option<&str>, t: &str) -> ModrinthDependency {
        ModrinthDependency {
            project_id: Some(project.into()),
            version_id: version.map(String::from),
            dep_type: t.into(),
        }
    }

    #[tokio::test]
    async fn duplicate_modid_is_error() {
        let mods = vec![
            mod_row("a", "a.jar", "same", None, None),
            mod_row("b", "b.jar", "same", None, None),
        ];
        let conflicts = ConflictChecker
            .check(&instance(None, "1.20.4"), &mods, &FakeDeps(HashMap::new()))
            .await;
        let dup = conflicts
            .iter()
            .find(|c| c.kind == ConflictKind::DuplicateModId)
            .expect("duplicate modid conflict");
        assert_eq!(dup.severity, ConflictSeverity::Error);
        assert_eq!(dup.related_files, vec!["a.jar", "b.jar"]);
    }

    #[tokio::test]
    async fn duplicate_file_is_warning() {
        let mods = vec![
            mod_row("abc", "a.jar", "a", None, None),
            mod_row("abc", "copy.jar", "b", None, None),
        ];
        let conflicts = ConflictChecker
            .check(&instance(None, "1.20.4"), &mods, &FakeDeps(HashMap::new()))
            .await;
        let dup = conflicts
            .iter()
            .find(|c| c.kind == ConflictKind::DuplicateFile)
            .expect("duplicate file conflict");
        assert_eq!(dup.severity, ConflictSeverity::Warning);
    }

    #[tokio::test]
    async fn loader_mismatch_is_error() {
        let mut m = mod_row("a", "a.jar", "a", None, None);
        m.loaders = vec!["fabric".into()];
        let mods = vec![m];
        let inst = instance(
            Some(Loader {
                kind: LoaderKind::Forge,
                version: "49".into(),
            }),
            "1.20.4",
        );
        let conflicts = ConflictChecker
            .check(&inst, &mods, &FakeDeps(HashMap::new()))
            .await;
        assert!(conflicts
            .iter()
            .any(|c| c.kind == ConflictKind::LoaderMismatch));
        // fabric mod in a fabric instance is fine
        let inst2 = instance(
            Some(Loader {
                kind: LoaderKind::Fabric,
                version: "0.16".into(),
            }),
            "1.20.4",
        );
        let mods2 = {
            let mut m = mod_row("a", "a.jar", "a", None, None);
            m.loaders = vec!["fabric".into()];
            vec![m]
        };
        assert!(!ConflictChecker
            .check(&inst2, &mods2, &FakeDeps(HashMap::new()))
            .await
            .iter()
            .any(|c| c.kind == ConflictKind::LoaderMismatch));
    }

    #[tokio::test]
    async fn mc_version_mismatch_is_warning() {
        let mut m = mod_row("a", "a.jar", "a", None, None);
        m.mc_versions = vec!["1.19.2".into()];
        let mods = vec![m];
        let conflicts = ConflictChecker
            .check(&instance(None, "1.20.4"), &mods, &FakeDeps(HashMap::new()))
            .await;
        let mc = conflicts
            .iter()
            .find(|c| c.kind == ConflictKind::McVersionMismatch)
            .expect("mc mismatch");
        assert_eq!(mc.severity, ConflictSeverity::Warning);
    }

    #[tokio::test]
    async fn incompatible_dependency_is_error() {
        let mods = vec![
            mod_row("a", "a.jar", "a", Some("A"), Some("va")),
            mod_row("b", "b.jar", "b", Some("B"), Some("vb")),
        ];
        let mut deps = HashMap::new();
        deps.insert("va".to_string(), vec![dep("B", Some("vb"), "incompatible")]);
        let conflicts = ConflictChecker
            .check(&instance(None, "1.20.4"), &mods, &FakeDeps(deps))
            .await;
        let inc = conflicts
            .iter()
            .find(|c| c.kind == ConflictKind::IncompatibleDependency)
            .expect("incompatible conflict");
        assert_eq!(inc.severity, ConflictSeverity::Error);
    }

    #[tokio::test]
    async fn missing_dependency_is_warning() {
        let mods = vec![mod_row("a", "a.jar", "a", Some("A"), Some("va"))];
        let mut deps = HashMap::new();
        deps.insert("va".to_string(), vec![dep("MISSING", None, "required")]);
        let conflicts = ConflictChecker
            .check(&instance(None, "1.20.4"), &mods, &FakeDeps(deps))
            .await;
        let miss = conflicts
            .iter()
            .find(|c| c.kind == ConflictKind::MissingDependency)
            .expect("missing dep conflict");
        assert_eq!(miss.severity, ConflictSeverity::Warning);
    }

    #[tokio::test]
    async fn satisfied_required_dependency_not_reported() {
        let mods = vec![
            mod_row("a", "a.jar", "a", Some("A"), Some("va")),
            mod_row("b", "b.jar", "b", Some("B"), Some("vb")),
        ];
        let mut deps = HashMap::new();
        deps.insert("va".to_string(), vec![dep("B", Some("vb"), "required")]);
        let conflicts = ConflictChecker
            .check(&instance(None, "1.20.4"), &mods, &FakeDeps(deps))
            .await;
        assert!(!conflicts
            .iter()
            .any(|c| c.kind == ConflictKind::MissingDependency));
    }
}
