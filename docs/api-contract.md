# Yuhina FFI 接口契约 (api-contract)

> 版本: v1.0 · 冻结于 M0 · 变更需规划层评审
> 本文档定义 `yuhina-bridge` 对 Flutter 暴露的**全部公开 API**。
> - **Agent A–D**：按此实现 Rust 侧类型与行为。
> - **Agent E**：按此消费绑定（可用 mock 先行开发 UI）。
> - 错误模型、事件流语义、命名约定为硬性规范，禁止单方面改动。

---

## 1. 通用约定

- **错误模型**：所有可失败方法返回 `Result<T, YuhinaError>`。`YuhinaError` 由 FRB 映射为 Dart 的 `YuhinaError`（含 `kind` 与 `message`）。
- **错误枚举** `YuhinaErrorKind`：
  `Network, Http(status, url), Auth, AuthExpired, NotLoggedIn, VersionNotFound, LoaderNotInstalled, JavaNotFound, InvalidInstance, ModConflict, ModpackInvalid, ChecksumMismatch, DownloadFailed, Canceled, Io, Internal`
- **事件流**：服务端通过 `StreamSink<T>` 推送；Dart 侧以 `Stream` 订阅。所有流都是**只读、非广播、背压由 FRB 处理**。
- **ID 类型**：一律用 `String`（UUID v4 / 项目 ID），禁止用数字。
- **路径**：一律返回绝对路径字符串；Dart 侧只做展示。
- **命名**：Rust snake_case，FRB 自动转 Dart camelCase；方法名、字段名以本文档为准。

## 2. 核心类型（结构定义）

### 2.1 配置
```rust
pub struct LauncherConfig {
    pub data_dir: String,          // 启动器数据目录(实例/下载/缓存)
    pub game_root: String,         // 游戏目录根
    pub download_source: Source,   // Official | Bmclapi | Custom(String)
    pub custom_source_host: Option<String>,
    pub launch_args: LaunchArgs,   // 全局默认 JVM/GC/分辨率
    pub locale: String,            // "zh-CN" | "en-US"
    pub theme_seed: u32,           // UI 动态主题种子色(由 UI 使用)
    pub auto_update: bool,         // 启动器自更新开关
}
pub struct LaunchArgs {
    pub min_memory_mb: u32, pub max_memory_mb: u32,
    pub extra_jvm_args: Vec<String>, pub extra_mc_args: Vec<String>,
    pub window_width: Option<u32>, pub window_height: Option<u32>,
}
pub enum Source { Official, Bmclapi, Custom(String) }
```

### 2.2 账号
```rust
pub struct Account {
    pub id: String,                // uuid v4
    pub kind: AccountKind,         // Microsoft | Yggdrasil | Offline
    pub username: String,          // 展示名
    pub uuid: String,              // 玩家 UUID
    pub yggdrasil_server: Option<String>, // 仅 Yggdrasil
    pub skin_url: Option<String>,
    pub is_active: bool,
    pub expires_at: Option<u64>,   // 毫秒时间戳
}
pub enum AccountKind { Microsoft, Yggdrasil, Offline }
pub struct MicrosoftLoginHandle { pub handle_id: String } // 轮询句柄
```

### 2.3 版本 / Java
```rust
pub struct VersionMeta {
    pub id: String, pub version_type: String, // release|snapshot|old_beta|old_alpha
    pub release_time: String, pub url: String, // manifest 内 url
    pub is_latest_release: bool, pub is_latest_snapshot: bool,
}
pub struct JavaRuntime {
    pub id: String, pub path: String, pub major: u32,
    pub vendor: String, pub version: String,
    pub arch: String, pub source: JavaSource, // Bundled | System | Manual
}
pub enum JavaSource { Bundled, System, Manual }
```

### 2.4 实例
```rust
pub struct InstanceSummary {
    pub id: String, pub name: String, pub icon: String, // emoji 或路径
    pub mc_version: String, pub loader: Option<Loader>, // None=原版
    pub is_installed: bool,          // 是否已下载完成可启动
    pub last_launched_at: Option<u64>,
    pub mod_count: u32,
    pub total_size_bytes: u64,
    pub created_at: u64, pub updated_at: u64,
}
pub struct Loader { pub kind: LoaderKind, pub version: String }
pub enum LoaderKind { Forge, Fabric, NeoForge, Quilt }
pub struct InstanceDetail {
    pub summary: InstanceSummary,
    pub game_dir: String,
    pub java: JavaSelection,        // Manual(path) | Auto(major) 
    pub launch_args: Option<LaunchArgs>, // 覆盖全局
    pub notes: String,
}
pub struct CreateInstanceRequest {
    pub name: String, pub icon: String,
    pub mc_version: String, pub loader: Option<Loader>,
    pub java: JavaSelection, pub dir_name: Option<String>,
}
pub enum JavaSelection { Auto(u32), Manual(String) }
```

### 2.5 Mod
```rust
pub struct InstalledMod {
    pub id: String,                 // instance内唯一(文件hash)
    pub file_name: String, pub file_size: u64,
    pub sha1: String,
    pub name: String, pub modid: String, pub description: String,
    pub loaders: Vec<String>, pub mc_versions: Vec<String>,
    pub project_id: Option<String>, pub version_id: Option<String>, // Modrinth 关联
    pub enabled: bool, pub installed_at: u64,
}
pub struct ModrinthProject {       // 搜索/详情共用
    pub project_id: String, pub slug: String, pub title: String,
    pub description: String, pub icon_url: Option<String>,
    pub downloads: u64, pub follows: u64,
    pub loaders: Vec<String>, pub game_versions: Vec<String>,
    pub categories: Vec<String>, pub versions: Vec<String>,
}
pub struct ModrinthVersion {
    pub version_id: String, pub name: String,
    pub version_number: String, pub game_versions: Vec<String>,
    pub loaders: Vec<String>, pub files: Vec<ModrinthFile>,
    pub dependencies: Vec<ModrinthDependency>,
    pub published: String,
}
pub struct ModrinthFile { pub name: String, pub size: u64, pub url: String, pub sha1: String }
pub struct ModrinthDependency {
    pub project_id: Option<String>, pub version_id: Option<String>,
    pub dep_type: String, // required | optional | incompatible
}
pub struct SearchResult { pub hits: Vec<ModrinthProject>, pub total: u64, pub offset: u32 }
pub struct ModUpdate { pub installed: InstalledMod, pub latest: ModrinthVersion }
pub struct ModConflict {
    pub severity: ConflictSeverity, // Warning | Error
    pub kind: ConflictKind, pub message: String,
    pub related_files: Vec<String>,
}
pub enum ConflictKind {
    DuplicateModId, DuplicateFile, LoaderMismatch, McVersionMismatch,
    IncompatibleDependency, MissingDependency,
}
```

### 2.6 下载任务
```rust
pub struct DownloadTask {
    pub id: String, pub title: String, pub state: DownloadState,
    pub total_bytes: u64, pub done_bytes: u64,
    pub speed_bps: u64, pub error: Option<String>,
    pub created_at: u64, pub can_pause: bool, pub can_cancel: bool,
}
pub enum DownloadState { Queued, Running, Paused, Done, Failed, Canceled }
pub struct DownloadProgressEvent { pub task_id: String, pub state: DownloadState,
    pub done_bytes: u64, pub total_bytes: u64, pub speed_bps: u64 }
```

### 2.7 游戏会话 / 日志 / 资讯
```rust
pub struct GameSession {
    pub session_id: String, pub instance_id: String,
    pub pid: u32, pub state: GameState, pub started_at: u64,
}
pub enum GameState { Starting, Running, Stopped(i32), Crashed(String) }
pub struct GameOutput { pub session_id: String, pub level: LogLevel, pub text: String }
pub enum LogLevel { Info, Warn, Error, Debug }
pub struct NewsItem { pub title: String, pub url: String, pub published: String, pub summary: String }
```

### 2.8 事件（服务端 → UI）
```rust
pub enum AppEvent {
    ConfigChanged,
    AccountsChanged,
    InstancesChanged,
    TaskChanged(String),          // 任务id
    JavaRuntimesChanged,
    VersionListChanged,
}
```

## 3. 门面方法（`YuhinaService`）

> 生命周期：`YuhinaService::new(config) -> Result<YuhinaService>`；应用单例。
> 所有 `async` 方法可在 Dart 侧直接 await。

### 3.1 配置与初始化
```rust
pub async fn new(config: LauncherConfig) -> Result<YuhinaService>;
pub async fn get_config(&self) -> LauncherConfig;
pub async fn set_config(&self, config: LauncherConfig) -> Result<()>;  // 触发 ConfigChanged
pub async fn watch_events(&self) -> StreamSink<AppEvent>;             // 全局事件流
pub async fn resolve_data_paths(&self) -> Result<(String, String)>;   // 输出数据/游戏根目录
```

### 3.2 账号 (Agent D 实现)
```rust
pub async fn list_accounts(&self) -> Vec<Account>;
pub async fn set_active_account(&self, id: String) -> Result<()>;
pub async fn add_offline_account(&self, username: String) -> Result<Account>;
pub async fn begin_microsoft_login(&self) -> Result<MicrosoftLoginHandle>; // 打开浏览器+loopback
pub async fn poll_microsoft_login(&self, handle: MicrosoftLoginHandle) -> Result<Option<Account>>;
pub async fn cancel_microsoft_login(&self, handle: MicrosoftLoginHandle) -> Result<()>;
pub async fn add_yggdrasil_account(&self, server_url: String, username: String,
                                   password: String) -> Result<Account>;
pub async fn refresh_account(&self, id: String) -> Result<Account>;
pub async fn remove_account(&self, id: String) -> Result<()>;
pub async fn get_active_account(&self) -> Result<Account>; // 未登录 → Auth(NotLoggedIn)
```

### 3.3 版本 / Java (Agent A 实现)
```rust
pub async fn fetch_version_list(&self) -> Result<Vec<VersionMeta>>;     // 拉取并缓存
pub async fn get_version_list(&self) -> Vec<VersionMeta>;               // 仅读缓存
pub async fn list_java_runtimes(&self) -> Vec<JavaRuntime>;
pub async fn scan_system_java(&self) -> Result<()>;
pub async fn add_manual_java(&self, path: String) -> Result<JavaRuntime>;
pub async fn install_java(&self, major: u32) -> Result<JavaRuntime>;    // 进度走任务系统
pub async fn remove_java(&self, id: String) -> Result<()>;
```

### 3.4 实例 (Agent A/C 实现)
```rust
pub async fn list_instances(&self) -> Vec<InstanceSummary>;
pub async fn get_instance(&self, id: String) -> Result<InstanceDetail>;
pub async fn create_instance(&self, req: CreateInstanceRequest) -> Result<InstanceSummary>;
pub async fn rename_instance(&self, id: String, name: String) -> Result<()>;
pub async fn set_instance_icon(&self, id: String, icon: String) -> Result<()>;
pub async fn clone_instance(&self, id: String, new_name: String) -> Result<InstanceSummary>;
pub async fn delete_instance(&self, id: String, delete_files: bool) -> Result<()>;
pub async fn install_instance_loader(&self, id: String, loader: Loader) -> Result<()>;
```

### 3.5 Mod / 整合包 (Agent C 实现)
```rust
pub async fn list_mods(&self, instance_id: String) -> Vec<InstalledMod>;
pub async fn set_mod_enabled(&self, instance_id: String, mod_id: String, enabled: bool) -> Result<()>;
pub async fn delete_mod(&self, instance_id: String, mod_id: String) -> Result<()>;
pub async fn search_mods(&self, query: String, loaders: Vec<String>, game_versions: Vec<String>,
                         index: u32, limit: u32) -> Result<SearchResult>;
pub async fn get_mod_project(&self, project_id: String) -> Result<ModrinthProject>;
pub async fn list_mod_versions(&self, project_id: String, loaders: Vec<String>,
                               game_versions: Vec<String>) -> Vec<ModrinthVersion>;
pub async fn install_mod(&self, instance_id: String, project_id: String,
                         version_id: Option<String>) -> Result<InstalledMod>; // None=自动选兼容
pub async fn install_mod_file(&self, instance_id: String, path: String) -> Result<InstalledMod>;
pub async fn check_mod_updates(&self, instance_id: String) -> Result<Vec<ModUpdate>>;
pub async fn update_mod(&self, instance_id: String, mod_id: String,
                        to_version_id: String) -> Result<InstalledMod>;
pub async fn check_mod_conflicts(&self, instance_id: String) -> Result<Vec<ModConflict>>;
pub async fn export_modpack(&self, instance_id: String, dest_path: String) -> Result<String>;
pub async fn import_modpack(&self, mrpack_path: String, name: String) -> Result<InstanceSummary>;
pub async fn download_modpack_from_modrinth(&self, project_id: String, version_id: String)
    -> Result<InstanceSummary>;   // 走 Modrinth 版本文件直接装
```

### 3.6 下载中心 (Agent B 实现)
```rust
pub async fn list_download_tasks(&self) -> Vec<DownloadTask>;
pub async fn pause_task(&self, id: String) -> Result<()>;
pub async fn resume_task(&self, id: String) -> Result<()>;
pub async fn cancel_task(&self, id: String) -> Result<()>;
pub async fn clear_finished_tasks(&self) -> Result<()>;
pub async fn watch_progress(&self) -> StreamSink<DownloadProgressEvent>; // 全局进度流
```

### 3.7 启动 / 进程 (Agent A 实现)
```rust
pub async fn launch_instance(&self, instance_id: String) -> Result<GameSession>;
pub async fn stop_game(&self, session_id: String) -> Result<()>;
pub async fn get_game_session(&self, session_id: String) -> Result<GameSession>;
pub async fn list_game_sessions(&self) -> Vec<GameSession>;
pub async fn watch_game_output(&self, session_id: String) -> StreamSink<GameOutput>;
pub async fn get_game_logs(&self, session_id: String, after_index: u64) -> Vec<GameLogEntry>;
pub struct GameLogEntry { pub index: u64, pub level: LogLevel, pub text: String, pub ts: u64 }
pub async fn open_game_dir(&self, instance_id: String) -> Result<()>;   // 打开目录
```

### 3.8 资讯 / 更新
```rust
pub async fn fetch_news(&self) -> Result<Vec<NewsItem>>;
pub async fn get_news(&self) -> Vec<NewsItem>;                 // 读缓存
pub async fn check_launcher_update(&self) -> Result<Option<String>>; // 最新版本号, 无则None
```

## 4. 流（Stream）语义

| 流 | 生命周期 | 语义 |
|---|---|---|
| `watch_events()` | 全局单例 | `AppEvent` 事件源，UI 用于刷新 Riverpod 状态 |
| `watch_progress()` | 全局单例 | 所有下载任务合并为一个进度流，含 `task_id` 区分 |
| `watch_game_output(session_id)` | 每会话一个 | 实时游戏 stdout/stderr，按行切分，UTF-8 容错 |
| 日志持久化 | — | Rust 侧同时写 `logs/<session>/game.log`，`get_game_logs` 用于回放 |

**关键约束：**
1. 流必须线程安全；Rust 侧 `tokio::sync::broadcast`/`mpsc` → `StreamSink` 转发，Dart 侧自动调度。
2. 游戏输出**按行**推送，行内不得截断；非 UTF-8 字节用 `lossy` 处理并标注级别。
3. 下载进度每 100ms 节流推送，避免 UI 抖动。

## 5. 数据模型（SQLite schema，Agent A 冻结）

> 库路径：`<data_dir>/yuhina.db`。所有表 `created_at`/`updated_at` 采用整数毫秒。

```sql
CREATE TABLE settings (key TEXT PRIMARY KEY, value TEXT NOT NULL);

CREATE TABLE accounts (
  id TEXT PRIMARY KEY, kind TEXT NOT NULL, username TEXT NOT NULL,
  uuid TEXT NOT NULL, yggdrasil_server TEXT,
  skin_url TEXT, access_token_enc TEXT, refresh_token_enc TEXT,
  expires_at INTEGER, is_active INTEGER DEFAULT 0,
  created_at INTEGER, updated_at INTEGER);

CREATE TABLE instances (
  id TEXT PRIMARY KEY, name TEXT NOT NULL, icon TEXT NOT NULL DEFAULT '🎮',
  mc_version TEXT NOT NULL, loader_kind TEXT, loader_version TEXT,
  game_dir TEXT NOT NULL, java_auto_major INTEGER, java_manual_path TEXT,
  launch_args_json TEXT, notes TEXT DEFAULT '',
  is_installed INTEGER DEFAULT 0, last_launched_at INTEGER,
  created_at INTEGER, updated_at INTEGER);

CREATE TABLE installed_mods (
  id TEXT PRIMARY KEY, instance_id TEXT NOT NULL REFERENCES instances(id) ON DELETE CASCADE,
  file_name TEXT NOT NULL, file_size INTEGER, sha1 TEXT,
  name TEXT, modid TEXT, description TEXT, loaders_json TEXT, mc_versions_json TEXT,
  project_id TEXT, version_id TEXT,
  enabled INTEGER DEFAULT 1, installed_at INTEGER);

CREATE TABLE download_tasks (
  id TEXT PRIMARY KEY, kind TEXT NOT NULL, title TEXT NOT NULL,
  instance_id TEXT, url TEXT NOT NULL, target_path TEXT NOT NULL,
  total_bytes INTEGER, done_bytes INTEGER, state TEXT NOT NULL,
  checksum_sha1 TEXT, error TEXT, created_at INTEGER, updated_at INTEGER);

CREATE TABLE java_runtimes (
  id TEXT PRIMARY KEY, path TEXT NOT NULL, major INTEGER NOT NULL,
  vendor TEXT, version TEXT, arch TEXT, source TEXT NOT NULL, added_at INTEGER);

CREATE TABLE version_cache (
  id TEXT PRIMARY KEY, version_type TEXT, release_time TEXT,
  url TEXT, manifest_json TEXT, fetched_at INTEGER);

CREATE TABLE news_cache (
  id TEXT PRIMARY KEY, title TEXT NOT NULL, url TEXT NOT NULL,
  published TEXT, summary TEXT, fetched_at INTEGER);
```

**加密约定**：`access_token_enc`/`refresh_token_enc` 使用 OS 密钥环（`keyring` crate；Windows DPAPI / Linux SecretService），不可用时降级为本地 AES-GCM（密钥存 `data_dir` 权限 0600 的文件）。**禁止明文存 token。**

## 6. 命名与风格硬性要求

1. FRB 暴露的 struct/enum 字段命名必须与本文档**完全一致**（含 `Option` 类型）。
2. 新增 API 必须：① 更新本文档 ② 在 `yuhina-bridge` 实现 ③ FRB 重新 codegen ④ 通过 E2E 冒烟。
3. Dart 侧不得 `import 'package:ffi'` 或直接调用底层 C；一切走生成绑定。
4. Rust crate 间依赖只允许：`db←api, core←api+db+download, instance←api+db+core+download, auth←api+db, bridge←一切`。禁止反向/平级互引。

## 7. 变更流程

- 变更提出 → 在 `docs/api-contract.md` 开 PR → 规划层（维护者）评审 → 合并后**各 Agent 在一个 PR 周期内同步**。
- 破坏性变更（删除/改签名）必须带迁移说明，且禁止在 M1 之后随意发生。