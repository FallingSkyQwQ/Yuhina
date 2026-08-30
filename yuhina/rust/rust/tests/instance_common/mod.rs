//! Shared fakes for Agent C integration tests: a mock Modrinth API server
//! (tiny_http), a mirror-less HTTP downloader and a stub core adapter.

#![allow(dead_code)]

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread::JoinHandle;

use serde_json::Value;
use tiny_http::{Response, Server};
use yuhina_api::{Loader, LoaderKind, VersionMeta, YuhinaError, YuhinaResult};

/// One captured request (path + query + User-Agent).
#[derive(Debug, Clone)]
pub struct ReqRecord {
    pub path: String,
    pub user_agent: Option<String>,
}

struct Inner {
    server: Arc<Server>,
    projects: Mutex<HashMap<String, Value>>,
    versions_by_project: Mutex<HashMap<String, Vec<Value>>>,
    versions: Mutex<HashMap<String, Value>>,
    files: Mutex<HashMap<String, Vec<u8>>>,
    search_hits: Mutex<Vec<Value>>,
    requests: Mutex<Vec<ReqRecord>>,
}

/// Minimal Modrinth API v2 mock. Routes:
/// - `/v2/search`
/// - `/v2/project/{id}/version` (honours `loaders` / `game_versions` filters)
/// - `/v2/project/{id}`
/// - `/v2/version/{id}`
/// - anything else → file bytes
pub struct MockModrinth {
    pub base: String,
    inner: Arc<Inner>,
    thread: Option<JoinHandle<()>>,
}

impl MockModrinth {
    pub fn start() -> Self {
        let server = Arc::new(Server::http("127.0.0.1:0").expect("bind mock modrinth"));
        let port = server.server_addr().to_ip().expect("ip").port();
        let inner = Arc::new(Inner {
            server: Arc::clone(&server),
            projects: Mutex::new(HashMap::new()),
            versions_by_project: Mutex::new(HashMap::new()),
            versions: Mutex::new(HashMap::new()),
            files: Mutex::new(HashMap::new()),
            search_hits: Mutex::new(Vec::new()),
            requests: Mutex::new(Vec::new()),
        });
        let t_inner = Arc::clone(&inner);
        let thread = std::thread::spawn(move || {
            for req in t_inner.server.incoming_requests() {
                let inner = Arc::clone(&t_inner);
                std::thread::spawn(move || handle(inner, req));
            }
        });
        Self {
            base: format!("http://127.0.0.1:{port}"),
            inner,
            thread: Some(thread),
        }
    }

    /// Client base URL (`<base>/v2`).
    pub fn api_base(&self) -> String {
        format!("{}/v2", self.base)
    }

    pub fn url(&self, path: &str) -> String {
        format!("{}{}", self.base, path)
    }

    pub fn set_project(&self, id: &str, json: Value) {
        self.inner.projects.lock().unwrap().insert(id.into(), json);
    }

    pub fn add_version(&self, project_id: &str, version: Value) {
        let vid = version["id"].as_str().unwrap_or("v").to_string();
        self.inner
            .versions_by_project
            .lock()
            .unwrap()
            .entry(project_id.into())
            .or_default()
            .push(version.clone());
        self.inner.versions.lock().unwrap().insert(vid, version);
    }

    pub fn add_file(&self, path: &str, bytes: Vec<u8>) {
        self.inner.files.lock().unwrap().insert(path.into(), bytes);
    }

    pub fn set_search_hits(&self, hits: Vec<Value>) {
        *self.inner.search_hits.lock().unwrap() = hits;
    }

    pub fn requests(&self) -> Vec<ReqRecord> {
        self.inner.requests.lock().unwrap().clone()
    }

    /// User-Agents observed for a request path prefix (e.g. `/v2/search`).
    pub fn uas_for(&self, prefix: &str) -> Vec<String> {
        self.requests()
            .into_iter()
            .filter(|r| r.path.starts_with(prefix))
            .filter_map(|r| r.user_agent)
            .collect()
    }

    pub fn hit_count(&self) -> usize {
        self.requests().len()
    }
}

impl Drop for MockModrinth {
    fn drop(&mut self) {
        if let Some(t) = self.thread.take() {
            self.inner.server.unblock();
            let _ = t.join();
        }
    }
}

fn handle(inner: Arc<Inner>, req: tiny_http::Request) {
    let path = req.url().to_string();
    let path_only = path.split('?').next().unwrap_or(&path).to_string();
    let ua = req
        .headers()
        .iter()
        .find(|h| h.field.equiv("User-Agent"))
        .map(|h| h.value.as_str().to_string());
    inner.requests.lock().unwrap().push(ReqRecord {
        path: path_only.clone(),
        user_agent: ua,
    });

    let query = parse_query(&path);

    if path_only.starts_with("/v2/search") {
        let hits = inner.search_hits.lock().unwrap().clone();
        let total = hits.len() as u64;
        respond_json(
            req,
            &serde_json::json!({
                "hits": hits, "total_hits": total, "offset": 0, "limit": 20
            }),
        );
        return;
    }
    if let Some(rest) = path_only.strip_prefix("/v2/project/") {
        let project_id = rest.trim_end_matches("/version");
        if rest.ends_with("/version") {
            let versions = inner
                .versions_by_project
                .lock()
                .unwrap()
                .get(project_id)
                .cloned()
                .unwrap_or_default();
            let loaders: Vec<String> = query
                .get("loaders")
                .and_then(|v| serde_json::from_str(v).ok())
                .unwrap_or_default();
            let game_versions: Vec<String> = query
                .get("game_versions")
                .and_then(|v| serde_json::from_str(v).ok())
                .unwrap_or_default();
            let filtered: Vec<Value> = versions
                .into_iter()
                .filter(|v| {
                    let v_loaders = v["loaders"]
                        .as_array()
                        .map(|a| {
                            a.iter()
                                .filter_map(Value::as_str)
                                .map(String::from)
                                .collect::<Vec<_>>()
                        })
                        .unwrap_or_default();
                    let v_games = v["game_versions"]
                        .as_array()
                        .map(|a| {
                            a.iter()
                                .filter_map(Value::as_str)
                                .map(String::from)
                                .collect::<Vec<_>>()
                        })
                        .unwrap_or_default();
                    (loaders.is_empty() || v_loaders.iter().any(|l| loaders.contains(l)))
                        && (game_versions.is_empty()
                            || v_games.iter().any(|g| game_versions.contains(g)))
                })
                .collect();
            respond_json(req, &serde_json::json!(filtered));
            return;
        }
        if let Some(p) = inner.projects.lock().unwrap().get(project_id).cloned() {
            respond_json(req, &p);
        } else {
            respond_404(req);
        }
        return;
    }
    if let Some(vid) = path_only.strip_prefix("/v2/version/") {
        if let Some(v) = inner.versions.lock().unwrap().get(vid).cloned() {
            respond_json(req, &v);
        } else {
            respond_404(req);
        }
        return;
    }
    // file download
    if let Some(bytes) = inner.files.lock().unwrap().get(&path_only).cloned() {
        let len = bytes.len();
        req.respond(
            Response::from_data(bytes).with_header(
                tiny_http::Header::from_bytes(&b"Content-Length"[..], len.to_string().as_bytes())
                    .unwrap(),
            ),
        )
        .ok();
    } else {
        respond_404(req);
    }
}

fn respond_json(req: tiny_http::Request, v: &Value) {
    let _ = req.respond(Response::from_string(v.to_string()).with_header(
        tiny_http::Header::from_bytes(&b"Content-Type"[..], &b"application/json"[..]).unwrap(),
    ));
}

fn respond_404(req: tiny_http::Request) {
    let _ = req.respond(Response::from_string(r#"{"error":"not found"}"#).with_status_code(404));
}

fn parse_query(url: &str) -> HashMap<String, String> {
    let mut out = HashMap::new();
    if let Some(q) = url.split_once('?') {
        for pair in q.1.split('&') {
            if let Some((k, v)) = pair.split_once('=') {
                let key = url::form_urlencoded::parse(k.as_bytes())
                    .map(|(x, _)| x)
                    .collect::<String>();
                let value = url::form_urlencoded::parse(v.as_bytes())
                    .map(|(x, _)| x)
                    .collect::<String>();
                out.insert(key, value);
            }
        }
    }
    out
}

/// Mirror-less HTTP downloader hitting the mock server directly.
pub struct MockHttpDownloader;

#[async_trait::async_trait]
impl yuhina_core::download::Downloader for MockHttpDownloader {
    fn rewrite(&self, url: &str) -> String {
        url.to_string()
    }
    async fn download(
        &self,
        url: &str,
        dest: &std::path::Path,
        sha1: Option<&str>,
    ) -> YuhinaResult<()> {
        let resp = reqwest::get(url)
            .await
            .map_err(|e| YuhinaError::network(format!("GET {url}: {e}")))?;
        let bytes = resp
            .bytes()
            .await
            .map_err(|e| YuhinaError::network(format!("read {url}: {e}")))?;
        yuhina_core::download::verify_and_write(dest, &bytes, sha1)
    }
    async fn fetch_bytes(&self, url: &str) -> YuhinaResult<Vec<u8>> {
        let resp = reqwest::get(url)
            .await
            .map_err(|e| YuhinaError::network(format!("GET {url}: {e}")))?;
        resp.bytes()
            .await
            .map(|b| b.to_vec())
            .map_err(|e| YuhinaError::network(format!("read {url}: {e}")))
    }
}

/// Core adapter stub exposing a fixed version list and no-op installs.
pub struct StubCore;

#[async_trait::async_trait]
impl yuhina_instance::CoreAdapter for StubCore {
    fn get_version_list(&self) -> Vec<VersionMeta> {
        ["1.20.4", "1.21.1"]
            .into_iter()
            .map(|id| VersionMeta {
                id: id.into(),
                version_type: "release".into(),
                release_time: String::new(),
                url: String::new(),
                is_latest_release: false,
                is_latest_snapshot: false,
            })
            .collect()
    }
    async fn ensure_version_files(&self, _mc: &str) -> YuhinaResult<u32> {
        Ok(0)
    }
    async fn resolve_loader_versions(
        &self,
        _mc: &str,
        _k: LoaderKind,
    ) -> YuhinaResult<Vec<String>> {
        Ok(vec!["0.16.0".into()])
    }
    async fn install_loader(&self, _id: &str, l: &Loader) -> YuhinaResult<Loader> {
        Ok(l.clone())
    }
}

/// Build a `ModrinthVersion`-shaped JSON document for the mock.
pub fn version_json(
    vid: &str,
    pid: &str,
    published: &str,
    game_versions: &[&str],
    loaders: &[&str],
    deps: &[Value],
) -> Value {
    serde_json::json!({
        "id": vid,
        "project_id": pid,
        "name": vid,
        "version_number": vid,
        "game_versions": game_versions,
        "loaders": loaders,
        "files": [],
        "dependencies": deps,
        "date_published": published,
    })
}

pub fn dep(project: &str, dep_type: &str) -> Value {
    serde_json::json!({ "project_id": project, "dependency_type": dep_type })
}

/// Version document with one downloadable file.
#[allow(clippy::too_many_arguments)]
pub fn version_with_file(
    vid: &str,
    pid: &str,
    published: &str,
    game_versions: &[&str],
    loaders: &[&str],
    deps: &[Value],
    file_url: &str,
    file_sha1: &str,
) -> Value {
    let mut v = version_json(vid, pid, published, game_versions, loaders, deps);
    v["files"] = serde_json::json!([{
        "filename": format!("{vid}.jar"),
        "size": 1,
        "url": file_url,
        "hashes": {"sha1": file_sha1}
    }]);
    v
}

/// Build a tiny fabric mod jar for tests.
pub fn fabric_jar(dir: &std::path::Path, name: &str, modid: &str) -> std::path::PathBuf {
    let path = dir.join(name);
    let file = std::fs::File::create(&path).unwrap();
    let mut zip = zip::ZipWriter::new(file);
    zip.start_file("fabric.mod.json", zip::write::SimpleFileOptions::default())
        .unwrap();
    use std::io::Write;
    write!(
        zip,
        r#"{{"id":"{modid}","name":"{modid}","depends":{{"minecraft":["1.20.4"]}}}}"#
    )
    .unwrap();
    zip.finish().unwrap();
    path
}
