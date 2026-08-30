//! Shared mock HTTP server for download/network integration tests.
//!
//! - [`MockServer`] (tiny_http): fixed data with `Range` (206) support, N
//!   failing (500) responses before succeeding, per-chunk delay.
//! - [`DropServer`] (raw TCP): truncates the first response mid-body then
//!   closes the connection — tiny_http cannot do this with keep-alive
//!   clients, so this one uses a plain socket.

#![allow(dead_code)]

use std::io::{self, Read, Write};
use std::net::{Shutdown, TcpListener, TcpStream};
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use std::thread::JoinHandle;
use std::time::Duration;

use tiny_http::{Header, Response, Server, StatusCode};

use yuhina_download::ManagerConfig;

#[derive(Debug, Clone)]
pub struct RequestInfo {
    pub path: String,
    pub range: Option<String>,
}

#[derive(Default, Clone)]
pub struct MockConfig {
    pub data: Vec<u8>,
    /// Respond 500 to the first N requests.
    pub fail_count: usize,
    /// Sleep before responding (builds up concurrency).
    pub delay: Option<Duration>,
    /// Bytes per chunk for slow streaming.
    pub chunk: usize,
}

struct Inner {
    server: Arc<Server>,
    requests: Mutex<Vec<RequestInfo>>,
    active: AtomicUsize,
    max_active: AtomicUsize,
    fail_left: AtomicUsize,
    config: MockConfig,
}

pub struct MockServer {
    pub base_url: String,
    inner: Arc<Inner>,
    thread: Option<JoinHandle<()>>,
}

impl MockServer {
    pub fn start(config: MockConfig) -> Self {
        let server = Arc::new(Server::http("127.0.0.1:0").expect("bind mock server"));
        let port = server.server_addr().to_ip().expect("ip addr").port();
        let inner = Arc::new(Inner {
            server: Arc::clone(&server),
            requests: Mutex::new(Vec::new()),
            active: AtomicUsize::new(0),
            max_active: AtomicUsize::new(0),
            fail_left: AtomicUsize::new(config.fail_count),
            config,
        });

        let thread_inner = Arc::clone(&inner);
        let thread = std::thread::spawn(move || {
            for request in thread_inner.server.incoming_requests() {
                let inner = Arc::clone(&thread_inner);
                std::thread::spawn(move || handle(inner, request));
            }
        });

        Self {
            base_url: format!("http://127.0.0.1:{port}"),
            inner,
            thread: Some(thread),
        }
    }

    /// URL for a path on this server.
    pub fn url(&self, path: &str) -> String {
        format!("{}{}", self.base_url, path)
    }

    pub fn requests(&self) -> Vec<RequestInfo> {
        self.inner.requests.lock().unwrap().clone()
    }

    pub fn hit_count(&self) -> usize {
        self.requests().len()
    }

    pub fn max_active(&self) -> usize {
        self.inner.max_active.load(Ordering::SeqCst)
    }

    /// Shuts the server down (unblocks the listener thread and joins it).
    pub fn shutdown(&mut self) {
        if let Some(t) = self.thread.take() {
            self.inner.server.unblock();
            let _ = t.join();
        }
    }
}

impl Drop for MockServer {
    fn drop(&mut self) {
        self.shutdown();
    }
}

/// A manager config tuned for fast local tests (tiny backoff, fast throttle).
pub fn fast_config(concurrency: usize) -> ManagerConfig {
    ManagerConfig {
        concurrency,
        retry_max: 3,
        backoff_base_ms: 5,
        backoff_cap_ms: 50,
        progress_interval_ms: 40,
        persist_interval_ms: 80,
        connect_timeout: Duration::from_secs(5),
    }
}

/// Polls `cond` until true or timeout.
pub async fn wait_for(cond: impl Fn() -> bool, timeout: Duration) -> bool {
    let start = std::time::Instant::now();
    while start.elapsed() < timeout {
        if cond() {
            return true;
        }
        tokio::time::sleep(Duration::from_millis(10)).await;
    }
    cond()
}

fn handle(inner: Arc<Inner>, request: tiny_http::Request) {
    let path = request.url().to_string();
    let range = request
        .headers()
        .iter()
        .find(|h| h.field.equiv("Range"))
        .map(|h| h.value.as_str().to_string());
    inner.requests.lock().unwrap().push(RequestInfo {
        path: path.clone(),
        range: range.clone(),
    });

    let active = inner.active.fetch_add(1, Ordering::SeqCst) + 1;
    inner.max_active.fetch_max(active, Ordering::SeqCst);

    let cfg = &inner.config;
    if let Some(d) = cfg.delay {
        std::thread::sleep(d);
    }

    let failed = inner
        .fail_left
        .fetch_update(Ordering::SeqCst, Ordering::SeqCst, |n| {
            if n > 0 {
                Some(n - 1)
            } else {
                None
            }
        })
        .is_ok();
    let result = if failed {
        request.respond(Response::from_string("boom").with_status_code(500))
    } else if cfg.delay.is_some() {
        let (status, headers, body) = range_response(cfg, &range);
        let body_len = body.len();
        let reader = SlowReader::new(body, cfg.chunk.max(1), cfg.delay.unwrap());
        request.respond(Response::new(
            StatusCode(status),
            headers,
            Box::new(reader) as Box<dyn Read + Send>,
            Some(body_len),
            None,
        ))
    } else {
        let (status, headers, body) = range_response(cfg, &range);
        let body_len = body.len();
        request.respond(Response::new(
            StatusCode(status),
            headers,
            Box::new(io::Cursor::new(body)) as Box<dyn Read + Send>,
            Some(body_len),
            None,
        ))
    };
    let _ = result;
    inner.active.fetch_sub(1, Ordering::SeqCst);
}

/// Builds (status, headers, body) honouring an optional `bytes=N-` Range.
fn range_response(cfg: &MockConfig, range: &Option<String>) -> (u16, Vec<Header>, Vec<u8>) {
    let data = &cfg.data;
    let start = match range {
        Some(r) if r.starts_with("bytes=") => r
            .trim_start_matches("bytes=")
            .trim_end_matches('-')
            .parse::<usize>()
            .unwrap_or(0),
        _ => 0,
    };
    if start > data.len() {
        return (416, vec![], Vec::new());
    }
    let body = data[start..].to_vec();
    let status = if start > 0 { 206 } else { 200 };
    let mut headers =
        vec![
            Header::from_bytes(&b"Content-Length"[..], body.len().to_string().as_bytes()).unwrap(),
        ];
    if status == 206 {
        headers.push(
            Header::from_bytes(
                &b"Content-Range"[..],
                format!("bytes {}-{}/{}", start, data.len() - 1, data.len()).as_bytes(),
            )
            .unwrap(),
        );
    }
    (status, headers, body)
}

/// Streams `data` in chunks with a sleep between chunks.
struct SlowReader {
    data: Vec<u8>,
    pos: usize,
    chunk: usize,
    delay: Duration,
}

impl SlowReader {
    fn new(data: Vec<u8>, chunk: usize, delay: Duration) -> Self {
        Self {
            data,
            pos: 0,
            chunk,
            delay,
        }
    }
}

impl Read for SlowReader {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        if self.pos >= self.data.len() {
            return Ok(0);
        }
        std::thread::sleep(self.delay);
        let n = self.chunk.min(self.data.len() - self.pos).min(buf.len());
        buf[..n].copy_from_slice(&self.data[self.pos..self.pos + n]);
        self.pos += n;
        Ok(n)
    }
}
/// Raw-TCP server that serves `data`, truncating the FIRST response after
/// `drop_at` bytes then closing the connection (simulates a mid-stream drop).
/// Subsequent connections get the full body.
pub struct DropServer {
    pub base_url: String,
    thread: Option<JoinHandle<()>>,
    served: Arc<AtomicUsize>,
    stop: Arc<AtomicBool>,
}

impl DropServer {
    pub fn start(data: Vec<u8>, drop_at: usize) -> Self {
        let listener = TcpListener::bind("127.0.0.1:0").expect("bind drop server");
        listener.set_nonblocking(true).expect("set nonblocking");
        let port = listener.local_addr().expect("addr").port();
        let data = Arc::new(data);
        let served = Arc::new(AtomicUsize::new(0));
        let stop = Arc::new(AtomicBool::new(false));
        let t_data = Arc::clone(&data);
        let t_served = Arc::clone(&served);
        let t_stop = Arc::clone(&stop);
        let thread = std::thread::spawn(move || {
            while !t_stop.load(Ordering::SeqCst) {
                match listener.accept() {
                    Ok((stream, _)) => {
                        let data = Arc::clone(&t_data);
                        let served = t_served.fetch_add(1, Ordering::SeqCst);
                        std::thread::spawn(move || serve_one(stream, data, served == 0, drop_at));
                    }
                    Err(e) if e.kind() == io::ErrorKind::WouldBlock => {
                        std::thread::sleep(Duration::from_millis(5));
                    }
                    Err(_) => break,
                }
            }
        });
        Self {
            base_url: format!("http://127.0.0.1:{port}"),
            thread: Some(thread),
            served,
            stop,
        }
    }

    pub fn url(&self, path: &str) -> String {
        format!("{}{}", self.base_url, path)
    }

    pub fn hit_count(&self) -> usize {
        self.served.load(Ordering::SeqCst)
    }
}

impl Drop for DropServer {
    fn drop(&mut self) {
        if let Some(t) = self.thread.take() {
            self.stop.store(true, Ordering::SeqCst);
            let _ = t.join();
        }
    }
}

fn serve_one(mut stream: TcpStream, data: Arc<Vec<u8>>, is_first: bool, drop_at: usize) {
    let mut buf = [0u8; 4096];
    let _ = stream.read(&mut buf);
    let body = data.as_slice();
    let n = if is_first {
        drop_at.min(body.len())
    } else {
        body.len()
    };
    let head = format!(
        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
        body.len()
    );
    let _ = stream.write_all(head.as_bytes());
    let _ = stream.write_all(&body[..n]);
    let _ = stream.flush();
    let _ = stream.shutdown(Shutdown::Both);
}
