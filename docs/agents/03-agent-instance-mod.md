# Agent C — 实例与 Mod 生态 (yuhina-instance)

> 依据 `00-master-plan.md` 与 `api-contract.md` 执行。

## 1. 职责范围

| 负责 | 不负责 |
|---|---|
| 实例 CRUD/复制/图标/删除/游戏目录管理 | 版本元数据与下载执行（A/B） |
| 加载器（Forge/Fabric/NeoForge/Quilt）安装编排（调用 A 的底层能力） | 账号（D） |
| Modrinth API 客户端（搜索/详情/版本/文件/依赖） | UI（E） |
| Mod 安装/启停/删除/元数据解析/更新检测/冲突检测/依赖解析 | |
| `.mrpack` 导入导出 | |
| Modrinth 整合包一键安装 | |

## 2. 交付物（文件结构）

```
rust/crates/yuhina-instance/src/{lib,instance,loader,modrinth,modfile,modmeta,conflict,dependency,modpack}.rs
rust/rust/                    ← 集成测试 (modrinth_test.rs, modpack_roundtrip_test.rs)
```

## 3. 关键任务分解

### T1 实例管理 (`instance.rs`)
- 基于 `InstanceRepo` 实现 CRUD；`game_dir` 默认 `<game_root>/<dir_name>`，`dir_name` 可指定，冲突时自动加后缀。
- 创建实例时校验 mc 版本存在、loader 组合合法（如 Quilt 不支持某版本则报错）。
- 复制 = 深拷贝游戏目录 + 新记录；删除时可选删除文件。
- 启动前 `ensure_installed`：未安装则触发下载编排（调用 A）+ loader 安装（T2），并更新 `is_installed`。
- **验收**：CRUD/复制/删除单测；目录隔离正确（`game_dir` 唯一）。

### T2 加载器安装 (`loader.rs`)
- 对 `Loader { kind, version }` 调用 A 暴露的 `install_loader` 底层；本层负责：版本选择 UI 数据（可用 loader 版本列表查询）、安装进度上报、失败回滚（清残留文件）。
- 提供 `available_loader_versions(mc_version, kind) -> Vec<LoaderVersion>`，聚合 Fabric/Quilt/Forge/NeoForge 的 meta 源（经 B 的镜像）。
- **验收**：Fabric 实例从「未安装」到「is_installed=true」端到端（CI 慢测）；不支持组合给出明确错误。

### T3 Modrinth 客户端 (`modrinth.rs`)
- 对接 Modrinth API v2：
  - `GET /v2/search`（query、facets：loaders/game_versions/categories，limit/offset，`index=relevance`）
  - `GET /v2/project/{id|slug}`
  - `GET /v2/project/{id}/version`（支持 `loaders`/`game_versions` 过滤）
  - `GET /v2/version/{id}`
- 客户端统一 header（`User-Agent: yuhina/<ver> (github.com/.../yuhina)`，Modrinth 要求 UA）。
- 分页与索引映射到契约 `SearchResult`。
- **验收**：mock 服务器下搜索/详情/版本过滤正确；`User-Agent` 断言。

### T4 Mod 文件管理 (`modfile.rs` / `modmeta.rs`)
- 启停：将文件移出/移入加载器扫描范围（约定：`<game_dir>/mods/<name>.jar` 与 `<game_dir>/mods/.disabled/<name>.jar`），原子移动。
- 元数据解析：读 jar 内 `fabric.mod.json`、`quilt.mod.json`、`META-INF/mods.toml`（Forge/NeoForge）→ name/modid/loaders/mc_versions；解析失败文件标记 `Unknown` 仍可启停。
- 关联 Modrinth：安装时记录 `project_id`/`version_id`；`sha1` 作为实例内唯一键。
- **验收**：构造测试 jar（zip 内放各格式 json）解析单测；`.disabled` 移动逻辑单测。

### T5 依赖解析与更新 (`dependency.rs`)
- 安装某 mod 时：读取其 `ModrinthDependency` 列表 → 递归解析 `required` 依赖 → 自动选兼容版本一并安装；`incompatible` 命中已装 mod 时记录冲突。
- 更新检测：遍历已装 mod，用其 mc_versions/loaders 匹配最新版本 → `ModUpdate` 列表；排除被依赖方引用的「锁定」版本（可选 M2 简化）。
- 版本选择策略：匹配 `game_versions` 含实例 MC 版本，且 `loaders` 与实例 loader 相交，取最新发布。
- **验收**：依赖图解析（mock 数据）单测：闭环检测、缺失依赖、版本不兼容三类用例。

### T6 冲突检测 (`conflict.rs`)
- 规则（启发式，M2 为提示级）：
  1. 同 `modid` 两个文件 → `DuplicateModId (Error)`
  2. 同 sha1 文件重复 → `DuplicateFile (Warning)`
  3. 已装 mod 的 loaders 与实例 loader 无交集 → `LoaderMismatch (Error)`
  4. 已装 mod 的 mc_versions 不含实例版本 → `McVersionMismatch (Warning)`
  5. Modrinth `incompatible` 依赖命中 → `IncompatibleDependency (Error)`
  6. `required` 依赖缺失且未安装 → `MissingDependency (Warning)`
- **验收**：六类冲突各构造测试用例单测。

### T7 整合包导入导出 (`modpack.rs`)
- 导出：扫描 `<game_dir>/mods`（含 `.disabled`，标记为 `optional`），写 `index.json`（formatVersion 1、name、mc_version、modloaders、files: 带 Modrinth 关联的 mod 用 sha1+url，本地文件用本地 hash）、打包 overrides（config/scripts/resourcepacks 等非 mods 文件）→ 生成 `.mrpack`（zip）。
- 导入：解包 → 读 index.json → 建实例 → 经 Modrinth 下载 mod 文件（sha1 校验）→ 复制 overrides；Modrinth 不可达时提示并跳过文件。
- 自建包安装：支持 index.json 内 `env.client.force`/`env.client.optional` 语义。
- **验收**：round-trip 测试：导出 → 导入 → 文件集合一致；含本地文件（非 Modrinth）场景。

### T8 Modrinth 整合包安装 (`download_modpack_from_modrinth`)
- 经 Modrinth version 文件下载 `.mrpack` → 走 T7 导入流程；进度上报到下载中心。
- **验收**：mock 下 end-to-end。

## 4. 依赖与前置
- 依赖：`yuhina-api`、`yuhina-db`、`yuhina-core`（版本安装/loader 底层）、`yuhina-download`（下载 + 镜像 + sha1）。
- **前置**：A 的 loader 底层能力、B 的下载 API 可用（M1 后）。**本 Agent 依赖最多，排在 A/B 之后启动**。

## 5. 测试策略
- 全部网络调用走 mock（Modrinth 用 wiremock 或本地 axum 伪造），慢速安装类测试标 `#[ignore]`。
- 构造最小测试 jar 用 `zip` crate 生成。
- 冲突/依赖为纯逻辑，密集单测。

## 6. 交接清单（给下游）
- [ ] 实例 API（契约 §3.4）合入，E 侧实例库可联调。
- [ ] Mod 全流程 API（契约 §3.5）合入，E 侧 Mod 管理页可联调。
- [ ] mrpack 导入导出 API 合入。
- [ ] 更新 `handoff.md`：慢测清单、真实 Modrinth 联调需网络（CI 不跑）。
- [ ] 未完成/风险项在 PR 描述中明示。