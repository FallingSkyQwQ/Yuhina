# Agent A — 核心引擎 (yuhina-core)

> 依据 `00-master-plan.md` 与 `api-contract.md` 执行。本文档定义本 Agent 的边界、任务、验收与交接。

## 1. 职责范围

| 负责 | 不负责 |
|---|---|
| `yuhina-api`：共享类型、`YuhinaError`/`YuhinaErrorKind` | 具体下载执行（交给 B） |
| `yuhina-db`：SQLite schema + 迁移 + 仓储层 | Modrinth/依赖/冲突（交给 C） |
| 版本清单/元数据解析与缓存 | 账号登录（交给 D） |
| 游戏文件下载**编排**（libraries/assets/natives） | UI（交给 E） |
| Java 运行时发现/下载/选择 | |
| 启动命令行构建、游戏进程 spawn/监控、日志持久化 | |
| 加载器（Forge/Fabric/NeoForge/Quilt）版本解析与安装编排（与 C 协作） | |

## 2. 交付物（文件结构）

```
rust/crates/yuhina-api/src/{lib,error,types}.rs
rust/crates/yuhina-db/src/{lib,schema,migrations,repo}.rs
rust/crates/yuhina-core/src/{lib,version,assets,manifest,libraries,arguments,java,launch,process,loader,config}.rs
rust/rust/                      ← 集成测试 (core_launch_test.rs 等)
```

## 3. 关键任务分解

### T1 共享类型层（`yuhina-api`）
- 定义 `api-contract.md` §2 全部类型 + §1 错误枚举，`serde` derive。
- 提供 `From<anyhow::Error>` → `YuhinaError` 的映射工具。
- **验收**：`cargo test` 通过；类型名与契约一字不差。

### T2 数据层（`yuhina-db`）
- 用 `rusqlite` (bundled) 建库，`user_version` 做迁移版本。
- 迁移 001：建 `api-contract.md` §5 全部表 + 索引（`installed_mods(instance_id)`、`download_tasks(state)`）。
- 仓储层函数：`AccountRepo`/`InstanceRepo`/`InstalledModRepo`/`DownloadTaskRepo`/`JavaRepo`/`VersionCacheRepo`/`NewsCacheRepo`。
- 提供 `Db::new(path)`，含 `WAL` 模式。
- **验收**：迁移幂等；CRUD 单测覆盖；删除实例级联删除 mod 记录。

### T3 版本元数据
- 拉取版本清单（官方 `launchermeta.mojang.com/mc/game/version_manifest_v2.json`，镜像时经 B 的 URL 重写）。
- 按版本 id 拉取 version json，解析：`libraries`（含 `rules` 的 os/arch 过滤、`natives`）、`assetIndex`、`downloads`、`javaVersion`、`arguments`（兼容旧 `minecraftArguments`）、`mainClass`、`logging`。
- 缓存到 `version_cache`（文件级缓存到 `<data_dir>/versions/`），支持校验与过期。
- **验收**：单测覆盖「os/arch rules 矩阵」「natives 展开」「新旧参数格式」；集成测试解析真实 release 版本 json。

### T4 Java 运行时
- 系统扫描：`JAVA_HOME`、PATH、常见安装路径（Windows 注册表/Program Files、Linux `/usr/lib/jvm`）；解析 `release` 文件或 `java -version`。
- 手动添加：校验路径含 `bin/java` 可执行。
- 自动下载：Adoptium API（`api.adoptium.net/v3/...`），按所需 major（8/17/21）选 HotSpot JRE/JDK；走 B 的下载任务系统并报进度。
- Java major 需求表：MC < 1.17 → 8；1.17~1.20.4 → 17；≥ 1.20.5 → 21。
- **验收**：扫描本机 Java 得到至少一项；`install_java(21)` 在测试环境（或 CI 缓存）可用；major 映射单测。

### T5 启动命令行构建
- 组装 JVM args（Xmx/Xms、GC 参数）、classpath（libraries 全路径 + natives 目录）、MC args（老/新格式）、auth 参数（username/uuid/accessToken/version/user_type）、resolution、`--gameDir`/`--assetsDir`、`--version`。
- 优先复用已下载的 `libraries`/`assets`，缺失时触发编排下载。
- **验收**：golden 单测：给定实例+账号输入，断言生成的命令数组逐项正确（含引号与空格路径）。

### T6 游戏文件下载编排
- 输入：version json + 已解析库列表 → 产出「待下载文件清单」（URL、目标路径、sha1）。
- URL 重写委托 B 的 `mirror` 模块；下载执行委托 B 的 `DownloadManager`；B 完成后回调校验 sha1。
- assets：按 assetIndex objects 生成 key→(path, sha1, size)，稀疏下载。
- **验收**：对真实 release 版本（如 1.20.4）在 CI 缓存下完成 libraries+assets 下载；sha1 校验失败被标记为错误。

### T7 加载器安装编排（与 Agent C 协作）
- Fabric：`meta.fabricmc.net/v2/versions/loader/{mc}` 取 loader 列表 → 下载 `fabric-installer` 或直接按 `fabric-server-launch` 方式生成 launch（约定：先跑 installer `server -dir <gameDir>` 头less，镜像源可替换）。
- Quilt：`meta.quiltmc.org/v3/versions/loader/{mc}` 同理。
- Forge：`maven.minecraftforge.net/net/minecraftforge/forge/promotions_slim.json` 选版本 → 下载 `forge-{mc}-{ver}-installer.jar`，用匹配 Java 执行 `--installServer`。
- NeoForge：`maven.neoforged.net/releases/net/neoforged/neoforge/maven-metadata.xml` + installer jar 同理。
- 所有 installer 执行通过 T5/T6 的进程 + 下载框架，失败时输出完整 stdout/stderr 到错误。
- **交付给 C**：`install_loader(instance, loader)` 的底层能力 + 安装结果（含 installed 标志）。
- **验收**：CI 慢速测试（tag 后跑）Fabric+Forge 各一版能安装成功；安装失败错误含日志。

### T8 进程管理 / 日志
- spawn Java 子进程，管道 stdout/stderr，按行 `lossy` 切分 → `GameOutput` 流 + 持久化 `logs/<session>/game.log`。
- 监控退出码：0 → `Stopped(0)`；非 0 → `Crashed(exitcode)`，并尝试解析 crash-report（`crash-reports/*.txt`）首段。
- `stop_game`：优雅请求（Windows `taskkill /pid` 友好方式 / Linux SIGTERM），超时 SIGKILL。
- **验收**：集成测试启动 `java -version` 假进程验证流/退出码；会话日志文件落盘。

## 4. 依赖与前置
- 依赖 `yuhina-api`（自产）、`yuhina-db`（自产）、`yuhina-download`（Agent B 的 `DownloadManager` + `mirror`）。
- **前置**：M0 完成、`api-contract.md` 冻结。可与 B/D 并行，但 T6 编排依赖 B 的下载 API——先以 trait/接口占位，B 合入后对接。

## 5. 测试策略
- 单元：rules 矩阵、arguments、URL 重写委托、Java 版本映射、命令构建 golden。
- 集成（`#[ignore]` 慢测或 CI 缓存）: 真实版本解析、installer 安装、真实子进程启动。
- 用 `wiremock`/axum test server 模拟 manifest/json/资产端点，避免依赖外网。

## 6. 交接清单（给下游）
- [ ] `yuhina-api` 类型与错误枚举冻结并合入 main（B/C/D/E 依赖）。
- [ ] `yuhina-db` schema + 迁移 + 仓储合入，schema 与契约 §5 一致。
- [ ] 暴露 `install_loader` 底层 API 给 C（含进度/错误）。
- [ ] 启动/进程 API 完成，E 侧可联调 `launch_instance`（先用离线账号）。
- [ ] 慢速集成测试标签说明写入 `handoff.md`。
- [ ] 未完成/风险项在 PR 描述中明示，不留静默 TODO。