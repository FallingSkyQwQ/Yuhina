//! Game subprocess spawn / monitoring / log persistence (task T8).
//!
//! The java process runs as an independent child. We stream stdout/stderr by
//! line (lossy UTF-8), broadcast `GameOutput`, persist to
//! `<data_dir>/logs/<session>/game.log` and classify the exit.

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::process::{Child, Command};
use tokio::sync::{Mutex, broadcast};
use tokio::task::JoinHandle;
use yuhina_api::{
    GameLogEntry, GameOutput, GameSession, GameState, LogLevel, YuhinaError, YuhinaErrorKind,
};

use crate::arguments::classify_level;
use crate::launch::LaunchCommand;

/// Grace period before force-killing after a stop request.
const STOP_GRACE: std::time::Duration = std::time::Duration::from_secs(5);

/// A live game process handle.
pub struct GameProcess {
    pub session_id: String,
    pub instance_id: String,
    pub pid: u32,
    pub started_at: u64,
    pub state: Arc<Mutex<GameState>>,
    pub output_tx: broadcast::Sender<GameOutput>,
    pub log_path: PathBuf,
    pub game_dir: PathBuf,
    pub task: JoinHandle<()>,
}

/// Manages all live sessions. Sessions live for the launcher process; the
/// persisted log file survives for replay via `get_game_logs`.
pub struct GameManager {
    sessions: Mutex<HashMap<String, Arc<GameProcess>>>,
}

impl GameManager {
    pub fn new() -> Self {
        Self {
            sessions: Mutex::new(HashMap::new()),
        }
    }

    /// Spawn the java subprocess and start streaming logs.
    pub async fn spawn(&self, cmd: LaunchCommand, instance_id: &str, log_path: &Path, game_dir: &Path) -> Result<GameSession, YuhinaError> {
        let session_id = uuid::Uuid::new_v4().to_string();
        let started_at = crate::now_millis() as u64;

        if let Some(parent) = log_path.parent() {
            std::fs::create_dir_all(parent)
                .map_err(|e| YuhinaError::io(format!("mkdir {}: {e}", parent.display())))?;
        }

        let mut child = Command::new(&cmd.java_bin)
            .args(&cmd.args)
            .current_dir(&cmd.cwd)
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped())
            .stdin(std::process::Stdio::null())
            .kill_on_drop(true)
            .spawn()
            .map_err(|e| YuhinaError::io(format!("spawn java: {e}")))?;

        let pid = child.id().unwrap_or(0);
        let state = Arc::new(Mutex::new(GameState::Running));
        let (output_tx, _) = broadcast::channel(1024);

        let log_path = log_path.to_path_buf();
        let game_dir = game_dir.to_path_buf();
        let instance_id = instance_id.to_string();

        let task = {
            let state = state.clone();
            let output_tx = output_tx.clone();
            let log_path = log_path.clone();
            let game_dir = game_dir.clone();
            let session_id = session_id.clone();
            tokio::spawn(async move {
                monitor_and_stream(
                    &mut child,
                    session_id,
                    state,
                    output_tx,
                    &log_path,
                    &game_dir,
                )
                .await;
            })
        };

        let proc = Arc::new(GameProcess {
            session_id: session_id.clone(),
            instance_id: instance_id.clone(),
            pid,
            started_at,
            state,
            output_tx,
            log_path,
            game_dir,
            task,
        });
        self.sessions.lock().await.insert(session_id.clone(), proc.clone());

        Ok(GameSession {
            session_id,
            instance_id,
            pid,
            state: GameState::Running,
            started_at,
        })
    }

    pub async fn stop(&self, session_id: &str) -> Result<(), YuhinaError> {
        let sessions = self.sessions.lock().await;
        let proc = sessions
            .get(session_id)
            .ok_or_else(|| YuhinaError::new(YuhinaErrorKind::InvalidInstance, "session not found"))?;
        // The monitor task holds the child; signal through a separate mechanism:
        // we keep the child inside the task, so request a graceful shutdown via
        // a dedicated signal. For simplicity the monitor task registers the
        // child id and we terminate the process tree from here.
        terminate(proc.pid, false)?;
        // wait briefly for graceful exit, then force
        let deadline = tokio::time::Instant::now() + STOP_GRACE;
        loop {
            if !is_running(&proc.state).await {
                break;
            }
            if tokio::time::Instant::now() >= deadline {
                terminate(proc.pid, true)?;
                break;
            }
            tokio::time::sleep(std::time::Duration::from_millis(100)).await;
        }
        Ok(())
    }

    pub async fn get(&self, session_id: &str) -> Result<GameSession, YuhinaError> {
        let sessions = self.sessions.lock().await;
        let proc = sessions
            .get(session_id)
            .ok_or_else(|| YuhinaError::new(YuhinaErrorKind::InvalidInstance, "session not found"))?;
        let state = proc.state.lock().await.clone();
        Ok(GameSession {
            session_id: proc.session_id.clone(),
            instance_id: proc.instance_id.clone(),
            pid: proc.pid,
            state,
            started_at: proc.started_at,
        })
    }

    pub async fn list(&self) -> Vec<GameSession> {
        let sessions = self.sessions.lock().await;
        let mut out = Vec::new();
        for proc in sessions.values() {
            let state = proc.state.lock().await.clone();
            out.push(GameSession {
                session_id: proc.session_id.clone(),
                instance_id: proc.instance_id.clone(),
                pid: proc.pid,
                state,
                started_at: proc.started_at,
            });
        }
        out
    }

    pub fn subscribe(&self, session_id: &str) -> Option<broadcast::Receiver<GameOutput>> {
        let guard = self.sessions.try_lock().ok()?;
        let proc = guard.get(session_id)?;
        Some(proc.output_tx.subscribe())
    }

    pub fn log_path(&self, session_id: &str) -> Option<PathBuf> {
        let guard = self.sessions.try_lock().ok()?;
        Some(guard.get(session_id)?.log_path.clone())
    }
}

impl Default for GameManager {
    fn default() -> Self {
        Self::new()
    }
}

async fn is_running(state: &Mutex<GameState>) -> bool {
    matches!(*state.lock().await, GameState::Running | GameState::Starting)
}

/// Stream child output into the broadcast channel + log file, then resolve
/// the final state (Stopped/Crashed).
async fn monitor_and_stream(
    child: &mut Child,
    session_id: String,
    state: Arc<Mutex<GameState>>,
    output_tx: broadcast::Sender<GameOutput>,
    log_path: &Path,
    game_dir: &Path,
) {
    let mut file = match tokio::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(log_path)
        .await
    {
        Ok(f) => f,
        Err(e) => {
            tracing::error!("open game log {}: {e}", log_path.display());
            return;
        }
    };

    let stdout = child.stdout.take();
    let stderr = child.stderr.take();

    let mut tasks = Vec::new();
    if let Some(so) = stdout {
        let tx = output_tx.clone();
        let f = file.try_clone().await.ok();
        let sid = session_id.clone();
        tasks.push(tokio::spawn(async move {
            stream_lines(BufReader::new(so), sid, tx, f, LevelHint::Stdout).await;
        }));
    }
    if let Some(se) = stderr {
        let tx = output_tx.clone();
        let f = file.try_clone().await.ok();
        let sid = session_id.clone();
        tasks.push(tokio::spawn(async move {
            stream_lines(BufReader::new(se), sid, tx, f, LevelHint::Stderr).await;
        }));
    }

    let status = child.wait().await;
    let _ = file.flush().await;

    for t in tasks {
        let _ = t.await;
    }

    let exit_code = status.map(|s| s.code()).unwrap_or(None);
    let final_state = match exit_code {
        Some(0) => GameState::Stopped(0),
        Some(code) => {
            let snippet = parse_crash_report(game_dir);
            if snippet.is_empty() {
                GameState::Crashed(format!("exit code {code}"))
            } else {
                GameState::Crashed(format!("exit code {code}\n{snippet}"))
            }
        }
        None => GameState::Crashed("killed by signal".to_string()),
    };
    *state.lock().await = final_state.clone();
    match &final_state {
        GameState::Stopped(0) => tracing::info!(session_id, "game exited cleanly"),
        other => tracing::warn!(session_id, "game ended: {other:?}"),
    }
}

enum LevelHint {
    Stdout,
    Stderr,
}

async fn stream_lines<R: tokio::io::AsyncRead + Unpin>(
    mut reader: BufReader<R>,
    session_id: String,
    tx: broadcast::Sender<GameOutput>,
    mut file: Option<tokio::fs::File>,
    hint: LevelHint,
) {
    loop {
        let mut buf: Vec<u8> = Vec::with_capacity(1024);
        match reader.read_until(b'\n', &mut buf).await {
            Ok(0) => break,
            Ok(_) => {
                let mut text = String::from_utf8_lossy(&buf).to_string();
                while text.ends_with('\n') || text.ends_with('\r') {
                    text.pop();
                }
                if text.is_empty() {
                    continue;
                }
                let level = match hint {
                    LevelHint::Stderr => LogLevel::Warn,
                    LevelHint::Stdout => classify_level(&text),
                };
                let ts = crate::now_millis();
                if let Some(f) = file.as_mut() {
                    let line = format!("[{ts}] {text}\n");
                    let _ = f.write_all(line.as_bytes()).await;
                    let _ = f.flush().await;
                }
                let _ = tx.send(GameOutput {
                    session_id: session_id.clone(),
                    level,
                    text,
                });
            }
            Err(_) => break,
        }
    }
}

/// Best-effort SIGTERM/SIGKILL (linux) or `taskkill` (windows).
fn terminate(pid: u32, force: bool) -> Result<(), YuhinaError> {
    if pid == 0 {
        return Ok(());
    }
    #[cfg(unix)]
    {
        let sig = if force { libc::SIGKILL } else { libc::SIGTERM };
        // SAFETY: signal number is valid; pid is a live child id.
        let ret = unsafe { libc::kill(pid as i32, sig) };
        if ret != 0 {
            // ESRCH = already dead, fine.
            let err = std::io::Error::last_os_error();
            if err.raw_os_error() != Some(libc::ESRCH) {
                return Err(YuhinaError::io(format!("kill {pid}: {err}")));
            }
        }
        Ok(())
    }
    #[cfg(windows)]
    {
        let flag = if force { "/F" } else { "" };
        let _ = std::process::Command::new("taskkill")
            .args(["/PID", &pid.to_string(), "/T", flag])
            .output();
        Ok(())
    }
}

/// Read the newest crash report and return its head (first ~6 lines).
pub fn parse_crash_report(game_dir: &Path) -> String {
    let dir = game_dir.join("crash-reports");
    let Ok(entries) = std::fs::read_dir(&dir) else {
        return String::new();
    };
    let mut files: Vec<_> = entries
        .flatten()
        .map(|e| e.path())
        .filter(|p| p.extension().is_some_and(|e| e == "txt"))
        .collect();
    files.sort();
    let newest = files.last().cloned();
    let Some(path) = newest else {
        return String::new();
    };
    let Ok(text) = std::fs::read_to_string(&path) else {
        return String::new();
    };
    text.lines().take(6).collect::<Vec<_>>().join("\n")
}

/// Read persisted log lines, filtered by `after_index` (0-based entry index).
pub fn read_game_log(path: &Path, after_index: u64) -> Result<Vec<GameLogEntry>, YuhinaError> {
    if !path.exists() {
        return Ok(Vec::new());
    }
    let text = std::fs::read_to_string(path)
        .map_err(|e| YuhinaError::io(format!("read log {}: {e}", path.display())))?;
    let mut out = Vec::new();
    for (i, line) in text.lines().enumerate() {
        let idx = i as u64;
        if idx < after_index {
            continue;
        }
        // Strip our "[ts] " prefix if present.
        let (level, text) = split_log_line(line);
        out.push(GameLogEntry {
            index: idx,
            level,
            text,
            ts: 0,
        });
    }
    Ok(out)
}

fn split_log_line(line: &str) -> (LogLevel, String) {
    if let Some(rest) = line.strip_prefix('[') {
        if let Some(end) = rest.find("] ") {
            return (classify_level(&rest[end + 2..]), rest[end + 2..].to_string());
        }
    }
    (classify_level(line), line.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn log_line_split() {
        let (level, text) = split_log_line("[123] Loading Minecraft");
        assert_eq!(text, "Loading Minecraft");
        assert_eq!(level, LogLevel::Info);
        let (level, text) = split_log_line("ERROR crash");
        assert_eq!(level, LogLevel::Error);
        assert_eq!(text, "ERROR crash");
    }

    #[tokio::test]
    async fn fake_process_streams_output_and_exit_code() {
        let mgr = GameManager::new();
        let dir = tempfile::tempdir().unwrap();
        let log_path = dir.path().join("logs/s1/game.log");
        // Use `sh` as a fake game process (no java dependency).
        let cmd = LaunchCommand {
            java_bin: if cfg!(windows) { "cmd".into() } else { "sh".into() },
            args: if cfg!(windows) {
                vec!["/C".to_string(), "echo hello & exit /b 0".to_string()]
            } else {
                vec!["-c".into(), "echo hello; exit 0".into()]
            },
            cwd: dir.path().to_path_buf(),
        };
        let session = mgr.spawn(cmd, "inst-1", &log_path, dir.path()).await.unwrap();
        assert!(session.pid > 0);

        let mut rx = mgr.subscribe(&session.session_id).unwrap();
        let mut saw_hello = false;
        loop {
            match tokio::time::timeout(std::time::Duration::from_secs(5), rx.recv()).await {
                Ok(Ok(out)) => {
                    if out.text.contains("hello") {
                        saw_hello = true;
                    }
                }
                _ => break,
            }
            let s = mgr.get(&session.session_id).await.unwrap();
            if matches!(s.state, GameState::Stopped(0)) {
                break;
            }
        }
        assert!(saw_hello, "expected hello line from fake process");
        let s = mgr.get(&session.session_id).await.unwrap();
        assert_eq!(s.state, GameState::Stopped(0));
        // log file persisted
        let log = read_game_log(&log_path, 0).unwrap();
        assert!(log.iter().any(|e| e.text.contains("hello")));
        // after_index replay works
        assert!(read_game_log(&log_path, 1000).unwrap().is_empty());
    }

    #[tokio::test]
    async fn fake_process_nonzero_exit_is_crash() {
        let mgr = GameManager::new();
        let dir = tempfile::tempdir().unwrap();
        let log_path = dir.path().join("logs/s2/game.log");
        let cmd = LaunchCommand {
            java_bin: if cfg!(windows) { "cmd".into() } else { "sh".into() },
            args: if cfg!(windows) {
                vec!["/C".into(), "exit /b 3".into()]
            } else {
                vec!["-c".into(), "exit 3".into()]
            },
            cwd: dir.path().to_path_buf(),
        };
        let session = mgr.spawn(cmd, "inst-2", &log_path, dir.path()).await.unwrap();
        // wait for termination
        for _ in 0..50 {
            let s = mgr.get(&session.session_id).await.unwrap();
            if !matches!(s.state, GameState::Running) {
                break;
            }
            tokio::time::sleep(std::time::Duration::from_millis(100)).await;
        }
        let s = mgr.get(&session.session_id).await.unwrap();
        assert!(matches!(s.state, GameState::Crashed(_)), "got {s:?}");
    }

    #[test]
    fn crash_report_parsing_empty_when_missing() {
        let dir = tempfile::tempdir().unwrap();
        assert!(parse_crash_report(dir.path()).is_empty());
    }
}