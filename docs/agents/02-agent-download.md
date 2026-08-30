# Agent B — 下载与网络服务 (yuhina-download)

> 依据 `00-master-plan.md` 与 `api-contract.md` 执行。

## 1. 职责范围

| 负责 | 不负责 |
|---|---|
| `DownloadManager`：并发队列、优先级、断点续传、重试、sha1 校验 | 版本元数据解析（A） |
| `mirror`：官方源 / BMCLAPI / 自定义源的 URL 重写 | 实例与 Mod（C） |
| 下载任务持久化与重启恢复 | 账号（D） |
| 下载进度节流事件流 | UI（E） |
| Mojang RSS 资讯拉取与缓存 | |
| 启动器自更新检查（GitHub Releases） | |

## 2. 交付物（文件结构）

```
rust/crates/yuhina-download/src/{lib,manager,worker,mirror,resume,checksum,news,update,task}.rs
rust/rust/                    ← 集成测试 (download_mirror_test.rs, resume_test.rs)
```

## 3. 关键任务分解

### T1 镜像模块 (`mirror.rs`)
- `enum Source { Official, Bmclapi, Custom(String) }` 已由契约定义。
- 建立**主机映射表**（以 host 为 key 重写）：
  | 官方 host | BMCLAPI host |
  |---|---|
  | `launchermeta.mojang.com` | `bmclapi2.bangbang93.com` |
  | `launcher.mojang.com` | `bmclapi2.bangbang93.com` |
  | `libraries.mojang.com` | `bmclapi2.bangbang93.com` |
  | `resources.download.minecraft.net` | `bmclapi2.bangbang93.com` |
  | `piston-meta.mojang.com` | `bmclapi2.bangbang93.com` |
  | `api.adoptium.net` | `bmclapi2.bangbang93.com`（BMCLAPI 也代理 Java） |
  | `maven.fabricmc.net` / `meta.fabricmc.net` | `bmclapi2.bangbang93.com` 对应路径 |
  | `meta.quiltmc.org` / `maven.quiltmc.org` | 同上 |
  | `maven.minecraftforge.net` | `bmclapi2.bangbang93.com/forge` |
  | `maven.neoforged.net` | `bmclapi2.bangbang93.com/neoforge` |
  | Modrinth API `api.modrinth.com` | 不重写（Modrinth 无镜像） |
- 重写函数输入 `(url, source)` → `url`；未知 host 时**原样返回**（安全降级）。
- `Custom(String)`：仅替换域名前缀，由设置页说明「自定义镜像需兼容 BMCLAPI 路径约定」。
- **验收**：映射表单测全覆盖；未知 host 降级行为单测。

### T2 下载管理器 (`manager.rs` / `worker.rs`)
- tokio 任务池，默认并发 8，可配置；任务队列支持优先级（启动类 > 库 > 资产）。
- 下载器：`reqwest`，支持 `Range` 断点续传，临时文件 `.part`，完成后原子改名。
- 重试策略：指数退避（1s/2s/4s…封顶 30s），3 次后标记 `Failed(Network)`；可区分可重试错误（网络）与不可重试（404/校验失败）。
- sha1 校验：下载完成后比对；失败 → 删除重试 → 仍失败标记 `ChecksumMismatch`。
- 单任务可 pause/resume/cancel（cancel 丢弃 `.part`）。
- 进度节流：每 100ms 聚合广播一次 `DownloadProgressEvent`。
- 统一入口 `DownloadManager::enqueue(FileReq) -> task_id`；`FileReq { url, dest, sha1, priority, kind }`。
- **验收**：本地起 `axum`/`tiny_http` 测试服务器，模拟慢速/中断响应；断点续传（中断后从 Range 恢复）、并发正确性、重试、校验失败路径全部单测/集成测试覆盖。

### T3 任务持久化与恢复
- 通过 `yuhina-db` 的 `DownloadTaskRepo` 落库。
- 服务启动时：`Running` 任务 → 恢复为 `Queued` 继续；`Paused` 保持暂停；`Failed` 保留供用户重试。
- **验收**：模拟重启后任务状态恢复正确；进度已持久化（`done_bytes` 参与续传起点）。

### T4 资讯拉取 (`news.rs`)
- Mojang 资讯：官方 RSS/状态页（`https://www.minecraft.net/en-us/feeds/community-content/rss` 或 status page）→ 解析 title/url/published/summary → `NewsItem`。
- 缓存到 `news_cache`，TTL 1 小时；失败静默返回缓存。
- **验收**：mock 端点解析正确；缓存未命中时返回空数组不抛错。

### T5 启动器自更新 (`update.rs`)
- 请求本仓库 GitHub Releases 最新 tag（`api.github.com/repos/FallingSkyQwQ/Yuhina/releases/latest`），与本地版本号（`Cargo.toml`/`pubspec`）比较，返回最新版本号或 `None`。
- **验收**：mock 响应比较逻辑单测；无网络时返回 `Ok(None)`。

## 4. 依赖与前置
- 依赖 `yuhina-api`（类型）、`yuhina-db`（`DownloadTaskRepo`/`NewsCacheRepo`）。
- **前置**：`api-contract.md` 冻结。与 A/D 并行，仅依赖其冻结的 db 仓储接口。

## 5. 测试策略
- `wiremock`/自建 test http server 覆盖所有网络路径，**禁止依赖真实外网**。
- 断点续传：模拟 `206 Partial Content`；校验失败：坏 sha1。
- 镜像：纯函数单测。
- CI 每 PR 跑 `cargo test`（网络测试用 mock）。

## 6. 交接清单（给下游）
- [ ] `DownloadManager` + `mirror` API 合入，A 的 T6 编排可对接。
- [ ] 进度事件流 `watch_progress` 可用，E 侧下载中心可联调。
- [ ] 资讯 + 自更新 API 合入，E 侧首页/设置可联调。
- [ ] 镜像映射表若有新增官方 host，必须同步更新本文档表格。
- [ ] 未完成/风险项在 PR 描述中明示。