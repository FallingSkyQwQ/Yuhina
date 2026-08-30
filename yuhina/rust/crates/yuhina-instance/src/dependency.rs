//! Modrinth dependency resolution + update detection (task T5).
//!
//! Version selection strategy: match `game_versions` containing the instance's
//! MC version AND `loaders` intersecting the instance loader, newest published
//! first. Required dependencies are resolved recursively with cycle detection;
//! incompatible dependencies hitting an installed project are reported.

use std::collections::HashSet;

use yuhina_api::{
    InstalledMod, InstanceSummary, LoaderKind, ModUpdate, ModrinthFile, ModrinthVersion,
    YuhinaResult,
};

use crate::modrinth::ModrinthClient;

/// One dependency that should be installed.
#[derive(Debug, Clone)]
pub struct ResolvedDependency {
    pub project_id: String,
    pub version: ModrinthVersion,
    pub file: ModrinthFile,
}

/// A required dependency that could not be satisfied.
#[derive(Debug, Clone)]
pub struct MissingDependency {
    pub project_id: Option<String>,
    pub version_id: Option<String>,
    pub reason: String,
}

/// Result of resolving a version's dependency graph.
#[derive(Debug, Clone, Default)]
pub struct ResolvedDependencies {
    /// Transitively required versions to install (deduplicated).
    pub to_install: Vec<ResolvedDependency>,
    /// Required dependencies with no compatible version available.
    pub missing: Vec<MissingDependency>,
    /// Incompatible dependencies that hit an installed mod (conflicts).
    pub incompatible: Vec<yuhina_api::ModrinthDependency>,
}

/// Resolves compatible versions and dependency graphs from Modrinth.
#[derive(Clone)]
pub struct DependencyResolver {
    modrinth: ModrinthClient,
}

impl DependencyResolver {
    pub fn new(modrinth: ModrinthClient) -> Self {
        Self { modrinth }
    }

    /// Loader tags the instance supports (empty for a vanilla instance).
    pub fn instance_loaders(instance: &InstanceSummary) -> Vec<String> {
        match &instance.loader {
            Some(l) => vec![loader_tag(l.kind).to_string()],
            None => Vec::new(),
        }
    }

    /// Latest compatible version of `project_id` for `instance`.
    pub async fn latest_compatible(
        &self,
        project_id: &str,
        instance: &InstanceSummary,
    ) -> YuhinaResult<Option<ModrinthVersion>> {
        let loaders = Self::instance_loaders(instance);
        let versions = self
            .modrinth
            .get_project_versions(
                project_id,
                &loaders,
                std::slice::from_ref(&instance.mc_version),
            )
            .await?;
        let candidates: Vec<ModrinthVersion> = versions
            .into_iter()
            .filter(|v| version_compatible(v, instance))
            .collect();
        Ok(select_latest(&candidates).cloned())
    }

    /// Resolve the full required-dependency graph of `root`. The root project
    /// itself is assumed present (it is being installed by the caller) and is
    /// skipped if the graph loops back to it.
    pub async fn resolve_required(
        &self,
        root: &ModrinthVersion,
        root_project_id: Option<&str>,
        instance: &InstanceSummary,
        installed: &[InstalledMod],
    ) -> YuhinaResult<ResolvedDependencies> {
        let mut out = ResolvedDependencies::default();
        let mut visited: HashSet<String> = HashSet::new();
        let mut queue: Vec<ModrinthVersion> = vec![root.clone()];
        let mut guard = 0usize;

        while let Some(v) = queue.pop() {
            guard += 1;
            if guard > 256 {
                break; // safety net against degenerate graphs
            }
            if !visited.insert(v.version_id.clone()) {
                continue;
            }
            for dep in v.dependencies {
                match dep.dep_type.as_str() {
                    "required" => {
                        // The root project is already being installed.
                        if dep.project_id.as_deref().is_some()
                            && dep.project_id.as_deref() == root_project_id
                        {
                            continue;
                        }
                        // Skip if already installed (project or exact version).
                        let already = dep
                            .project_id
                            .as_deref()
                            .map(|pid| {
                                installed
                                    .iter()
                                    .any(|m| m.project_id.as_deref() == Some(pid))
                            })
                            .unwrap_or(false)
                            || dep
                                .version_id
                                .as_deref()
                                .map(|vid| {
                                    installed
                                        .iter()
                                        .any(|m| m.version_id.as_deref() == Some(vid))
                                })
                                .unwrap_or(false);
                        if already {
                            continue;
                        }
                        match self.resolve_dependency(&dep, instance).await {
                            Some(resolved) => {
                                // Skip versions already processed (cycle guard).
                                if visited.contains(&resolved.version.version_id) {
                                    continue;
                                }
                                if out
                                    .to_install
                                    .iter()
                                    .any(|r| r.project_id == resolved.project_id)
                                {
                                    continue;
                                }
                                // Recurse into the dependency's own deps.
                                queue.push(resolved.version.clone());
                                out.to_install.push(resolved);
                            }
                            None => out.missing.push(MissingDependency {
                                project_id: dep.project_id.clone(),
                                version_id: dep.version_id.clone(),
                                reason: "no compatible version available".into(),
                            }),
                        }
                    }
                    "incompatible" => {
                        let hits_installed = dep.project_id.as_deref().map(|pid| {
                            installed
                                .iter()
                                .any(|m| m.project_id.as_deref() == Some(pid))
                        });
                        if hits_installed == Some(true) {
                            out.incompatible.push(dep);
                        }
                    }
                    _ => {}
                }
            }
        }
        Ok(out)
    }

    /// Detect updates: for every installed mod with a Modrinth linkage, find
    /// the latest compatible version; report when it differs from the
    /// installed one.
    pub async fn check_updates(
        &self,
        installed: &[InstalledMod],
        instance: &InstanceSummary,
    ) -> YuhinaResult<Vec<ModUpdate>> {
        let mut out = Vec::new();
        for m in installed {
            let (Some(pid), Some(vid)) = (&m.project_id, &m.version_id) else {
                continue;
            };
            if let Ok(Some(latest)) = self.latest_compatible(pid, instance).await {
                if latest.version_id != *vid {
                    out.push(ModUpdate {
                        installed: m.clone(),
                        latest,
                    });
                }
            }
        }
        Ok(out)
    }

    /// Fetch a dependency to a concrete, compatible version if possible.
    async fn resolve_dependency(
        &self,
        dep: &yuhina_api::ModrinthDependency,
        instance: &InstanceSummary,
    ) -> Option<ResolvedDependency> {
        let (project_id, version) = if let Some(vid) = &dep.version_id {
            let (pid, version) = self.modrinth.get_version_with_project(vid).await.ok()?;
            (pid, version)
        } else if let Some(pid) = &dep.project_id {
            let version = self.latest_compatible(pid, instance).await.ok().flatten()?;
            (pid.clone(), version)
        } else {
            return None;
        };
        if !version_compatible(&version, instance) {
            return None;
        }
        let file = version.files.first()?.clone();
        Some(ResolvedDependency {
            project_id,
            version,
            file,
        })
    }
}

/// A version is usable for `instance` when its game versions contain the
/// instance's MC version and (if the instance has a loader) its loaders
/// overlap with the instance's loader.
pub fn version_compatible(v: &ModrinthVersion, instance: &InstanceSummary) -> bool {
    if !v.game_versions.contains(&instance.mc_version) {
        return false;
    }
    match &instance.loader {
        None => true,
        Some(l) => v.loaders.is_empty() || v.loaders.iter().any(|x| x == loader_tag(l.kind)),
    }
}

/// Pick the version with the newest `published` timestamp (ISO-8601 strings
/// compare lexicographically).
pub fn select_latest(versions: &[ModrinthVersion]) -> Option<&ModrinthVersion> {
    versions.iter().max_by(|a, b| a.published.cmp(&b.published))
}

pub fn loader_tag(kind: LoaderKind) -> &'static str {
    match kind {
        LoaderKind::Forge => "forge",
        LoaderKind::Fabric => "fabric",
        LoaderKind::NeoForge => "neoforge",
        LoaderKind::Quilt => "quilt",
    }
}
