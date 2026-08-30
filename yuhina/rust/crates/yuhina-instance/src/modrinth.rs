//! Modrinth API v2 client (task T3).
//!
//! All requests carry the required `User-Agent: yuhina/<ver> (...)` header.
//! The base URL is configurable so tests can point at a mock server; the
//! production default is `https://api.modrinth.com/v2`.

use std::collections::HashMap;

use serde::Deserialize;
use yuhina_api::{
    ModrinthDependency, ModrinthFile, ModrinthProject, ModrinthVersion, SearchResult, YuhinaError,
    YuhinaResult,
};

use crate::{DEFAULT_MODRINTH_BASE, USER_AGENT_BASE};

/// Modrinth API v2 client.
#[derive(Clone)]
pub struct ModrinthClient {
    base: String,
    http: reqwest::Client,
}

impl Default for ModrinthClient {
    fn default() -> Self {
        Self::new()
    }
}

impl ModrinthClient {
    pub fn new() -> Self {
        Self::new_with_base(DEFAULT_MODRINTH_BASE.to_string())
    }

    /// Client pointed at a custom base URL (mock servers in tests).
    pub fn new_with_base(base: String) -> Self {
        let http = reqwest::Client::builder()
            .user_agent(format!(
                "yuhina/{} ({})",
                env!("CARGO_PKG_VERSION"),
                USER_AGENT_BASE
            ))
            .build()
            .expect("build modrinth reqwest client");
        Self { base, http }
    }

    /// `GET /v2/search` — paginated search over projects.
    pub async fn search(
        &self,
        query: &str,
        loaders: &[String],
        game_versions: &[String],
        index: u32,
        limit: u32,
    ) -> YuhinaResult<SearchResult> {
        let mut facets: Vec<Vec<String>> = vec![vec!["project_type:mod".into()]];
        if !loaders.is_empty() {
            facets.push(loaders.iter().map(|l| format!("categories:{l}")).collect());
        }
        if !game_versions.is_empty() {
            facets.push(
                game_versions
                    .iter()
                    .map(|g| format!("versions:{g}"))
                    .collect(),
            );
        }
        let params: Vec<(&str, String)> = vec![
            ("query", query.to_string()),
            ("index", "relevance".to_string()),
            ("offset", index.to_string()),
            ("limit", limit.to_string()),
            (
                "facets",
                serde_json::to_string(&facets).map_err(|e| YuhinaError::internal(e.to_string()))?,
            ),
        ];
        let url = format!("{}/search", self.base);
        let resp = self
            .http
            .get(&url)
            .query(&params)
            .send()
            .await
            .map_err(|e| YuhinaError::network(format!("GET {url}: {e}")))?;
        let wire: SearchResponse = parse_json(resp, &url).await?;
        Ok(SearchResult {
            hits: wire.hits.into_iter().map(SearchHit::into_project).collect(),
            total: wire.total_hits,
            offset: wire.offset,
        })
    }

    /// `GET /v2/project/{id}` — project detail (id or slug).
    pub async fn get_project(&self, project_id: &str) -> YuhinaResult<ModrinthProject> {
        let url = format!("{}/project/{project_id}", self.base);
        let resp = self
            .http
            .get(&url)
            .send()
            .await
            .map_err(|e| YuhinaError::network(format!("GET {url}: {e}")))?;
        let wire: ProjectWire = parse_json(resp, &url).await?;
        Ok(wire.into_project())
    }

    /// `GET /v2/project/{id}/version` — versions, optionally filtered by
    /// loaders / game versions (AND semantics).
    pub async fn get_project_versions(
        &self,
        project_id: &str,
        loaders: &[String],
        game_versions: &[String],
    ) -> YuhinaResult<Vec<ModrinthVersion>> {
        let url = format!("{}/project/{project_id}/version", self.base);
        let mut params: Vec<(&str, String)> = Vec::new();
        if !loaders.is_empty() {
            params.push((
                "loaders",
                serde_json::to_string(loaders).map_err(|e| YuhinaError::internal(e.to_string()))?,
            ));
        }
        if !game_versions.is_empty() {
            params.push((
                "game_versions",
                serde_json::to_string(game_versions)
                    .map_err(|e| YuhinaError::internal(e.to_string()))?,
            ));
        }
        let resp = self
            .http
            .get(&url)
            .query(&params)
            .send()
            .await
            .map_err(|e| YuhinaError::network(format!("GET {url}: {e}")))?;
        let wires: Vec<VersionWire> = parse_json(resp, &url).await?;
        Ok(wires.into_iter().map(VersionWire::into_version).collect())
    }

    /// `GET /v2/version/{id}` — a single version.
    pub async fn get_version(&self, version_id: &str) -> YuhinaResult<ModrinthVersion> {
        let url = format!("{}/version/{version_id}", self.base);
        let resp = self
            .http
            .get(&url)
            .send()
            .await
            .map_err(|e| YuhinaError::network(format!("GET {url}: {e}")))?;
        let wire: VersionWire = parse_json(resp, &url).await?;
        Ok(wire.into_version())
    }

    /// Fetch a version plus its owning project id (the contract
    /// `ModrinthVersion` does not carry `project_id`).
    pub async fn get_version_with_project(
        &self,
        version_id: &str,
    ) -> YuhinaResult<(String, ModrinthVersion)> {
        let url = format!("{}/version/{version_id}", self.base);
        let resp = self
            .http
            .get(&url)
            .send()
            .await
            .map_err(|e| YuhinaError::network(format!("GET {url}: {e}")))?;
        let wire: VersionWire = parse_json(resp, &url).await?;
        let project_id = wire.project_id.clone();
        Ok((project_id, wire.into_version()))
    }

    /// Download a version file to `dest` verifying its sha1. Modrinth CDN
    /// URLs are not rewritten by the mirror (safe pass-through).
    pub async fn download_version_file(
        &self,
        downloader: &dyn yuhina_core::download::Downloader,
        file: &ModrinthFile,
        dest: &std::path::Path,
    ) -> YuhinaResult<()> {
        downloader.download(&file.url, dest, Some(&file.sha1)).await
    }
}

// ---------------------------------------------------------------------------
// Wire types (Modrinth API v2 JSON shapes)
// ---------------------------------------------------------------------------

#[derive(Deserialize)]
struct SearchResponse {
    hits: Vec<SearchHit>,
    #[serde(rename = "total_hits")]
    total_hits: u64,
    offset: u32,
    #[allow(dead_code)]
    limit: u32,
}

#[derive(Deserialize)]
struct SearchHit {
    project_id: String,
    slug: String,
    title: String,
    description: String,
    icon_url: Option<String>,
    downloads: u64,
    follows: u64,
    loaders: Vec<String>,
    game_versions: Vec<String>,
    categories: Vec<String>,
    versions: Vec<String>,
}

impl SearchHit {
    fn into_project(self) -> ModrinthProject {
        ModrinthProject {
            project_id: self.project_id,
            slug: self.slug,
            title: self.title,
            description: self.description,
            icon_url: self.icon_url,
            downloads: self.downloads,
            follows: self.follows,
            loaders: self.loaders,
            game_versions: self.game_versions,
            categories: self.categories,
            versions: self.versions,
        }
    }
}

#[derive(Deserialize)]
struct ProjectWire {
    id: String,
    slug: String,
    title: String,
    description: String,
    icon_url: Option<String>,
    downloads: u64,
    followers: u64,
    loaders: Vec<String>,
    game_versions: Vec<String>,
    categories: Vec<String>,
    versions: Vec<String>,
}

impl ProjectWire {
    fn into_project(self) -> ModrinthProject {
        ModrinthProject {
            project_id: self.id,
            slug: self.slug,
            title: self.title,
            description: self.description,
            icon_url: self.icon_url,
            downloads: self.downloads,
            follows: self.followers,
            loaders: self.loaders,
            game_versions: self.game_versions,
            categories: self.categories,
            versions: self.versions,
        }
    }
}

#[derive(Deserialize)]
struct VersionWire {
    id: String,
    project_id: String,
    name: String,
    version_number: String,
    game_versions: Vec<String>,
    loaders: Vec<String>,
    files: Vec<FileWire>,
    dependencies: Vec<DepWire>,
    date_published: String,
}

impl VersionWire {
    fn into_version(self) -> ModrinthVersion {
        ModrinthVersion {
            version_id: self.id,
            name: self.name,
            version_number: self.version_number,
            game_versions: self.game_versions,
            loaders: self.loaders,
            files: self.files.into_iter().map(FileWire::into_file).collect(),
            dependencies: self
                .dependencies
                .into_iter()
                .map(DepWire::into_dependency)
                .collect(),
            published: self.date_published,
        }
    }
}

#[derive(Deserialize)]
struct FileWire {
    name: String,
    size: u64,
    url: String,
    hashes: HashMap<String, String>,
}

impl FileWire {
    fn into_file(self) -> ModrinthFile {
        ModrinthFile {
            name: self.name,
            size: self.size,
            url: self.url,
            sha1: self.hashes.get("sha1").cloned().unwrap_or_default(),
        }
    }
}

#[derive(Deserialize)]
struct DepWire {
    project_id: Option<String>,
    version_id: Option<String>,
    dependency_type: String,
}

impl DepWire {
    fn into_dependency(self) -> ModrinthDependency {
        ModrinthDependency {
            project_id: self.project_id,
            version_id: self.version_id,
            dep_type: self.dependency_type,
        }
    }
}

async fn parse_json<T: for<'de> Deserialize<'de>>(
    resp: reqwest::Response,
    url: &str,
) -> YuhinaResult<T> {
    if !resp.status().is_success() {
        return Err(YuhinaError::new(
            YuhinaErrorKind::Http(resp.status().as_u16(), url.to_string()),
            format!("GET {url} -> HTTP {}", resp.status()),
        ));
    }
    let bytes = resp
        .bytes()
        .await
        .map_err(|e| YuhinaError::network(format!("read body {url}: {e}")))?;
    serde_json::from_slice(&bytes).map_err(|e| YuhinaError::internal(format!("parse {url}: {e}")))
}

use yuhina_api::YuhinaErrorKind;
