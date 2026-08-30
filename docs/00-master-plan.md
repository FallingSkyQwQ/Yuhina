# Yuhina 启动器 — 总开发规划 (Master Plan)

> 版本: v0.1 · 作者: 规划 Agent · 适用读者: 全体开发 Agent
> 本文档是唯一权威规划来源。所有 Agent 开工前必须先读本文档 + `api-contract.md` + 自己的 Agent 文档。

---

## 1. 项目愿景

`Yuhina` 是一个现代化、可扩展的 Minecraft 启动器，覆盖 **Windows + Linux** 双平台。
采用 **Flutter (UI) + Rust (服务)** 双层架构，服务与 UI 彻底解耦，UI 采用 **Material 3 Expressive** 风格。
面向中文区与全球用户，内置 **BMCLAPI 镜像加速** 与 **多来源切换**，支持第三方账号体系。

## 2. 需求汇总（已与产品方确认）

| 维度 | 决策 |
|---|---|
| 目标平台 | Windows + Linux；**Linux 为第一开发平台** |
| 架构 | Flutter UI + Rust 服务，`flutter_rust_bridge (FFI)` 解耦 |
| UI 风格 | Material 3 Expressive (Flutter 3.35+ 组件) |
| Mod 加载器 | Forge + Fabric + NeoForge + Quilt |
| 整合包 | Modrinth 来源；支持导入 `.mrpack` **与导出制作** |
| 账号 | **微软 OAuth**(已有 client_id) + **Yggdrasil**(LittleSkin 等自定义服务器) + **离线模式** |
| Java 管理 | 自动下载(Adoptium) + 系统检测 + 手动指定路径 |
| 实例体系 | 多实例管理（Prism 风格），支持复制/导入导出 |
| 下载镜像 | 官方源 / BMCLAPI 可切换 |
| Mod 管理 | 全流程：浏览/搜索/下载/启停/更新/冲突检测/依赖解析 |
| 数据存储 | SQLite（Rust 服务层 `rusqlite`） |
| 资讯/日志 | Mojang RSS + 启动日志页 + 崩溃报告 |
| 本地化 | 中文 + 英文 |
| UI 页面 | 首页 + 实例库 + Mod 管理 + 下载中心 + 设置 |
| CI/CD | git + GitHub Actions 多平台构建、自动发布 GitHub Release |
| 发布形态 | 便携 zip/tar.gz + 安装器（Windows NSIS / Linux AppImage） |

## 3. 架构总览

```
┌──────────────────────────────────────────────────────────────┐
│                    Flutter UI (Dart)                          │
│  Material 3 Expressive · Riverpod · go_router · i18n         │
│  首页/实例/Mod/下载中心/设置/日志/登录                          │
└──────────────────────────┬───────────────────────────────────┘
                           │  flutter_rust_bridge v2 (FFI)
                           │  类型安全 · 异步 · 事件流 (StreamSink)
┌──────────────────────────▼───────────────────────────────────┐
│                    Rust 服务层 (Cargo workspace)              │
│                                                              │
│  ┌─────────────┐ ┌──────────────┐ ┌────────────────────────┐ │
│  │ yuhina-core │ │ yuhina-      │ │ yuhina-instance        │ │
│  │ 版本解析/    │ │ download     │ │ 实例/Mod/整合包/加载器  │ │
│  │ 启动/进程    │ │ 下载/镜像/   │ │ (Modrinth客户端/冲突/   │ │
│  │ Java构建    │ │ 资讯/更新    │ │  依赖解析)              │ │
│  └─────────────┘ └──────────────┘ └────────────────────────┘ │
│  ┌─────────────┐ ┌──────────────┐ ┌────────────────────────┐ │
│  │ yuhina-auth │ │ yuhina-db    │ │ yuhina-bridge (FRB)    │ │
│  │ 微软/Ygg/   │ │ SQLite schema│ │ 对外FFI门面+事件流       │ │
│  │ 离线        │ │ + 迁移       │ │                        │ │
│  └─────────────┘ └──────────────┘ └────────────────────────┘ │
│  ┌─────────────┐                                              │
│  │ yuhina-api  │ ← 共享类型/错误枚举，所有 crate 依赖         │
│  └─────────────┘                                              │
└──────────────────────────────────────────────────────────────┘
                           │
            ┌──────────────┼─────────────────┐
     ┌──────▼─────┐  ┌─────▼──────┐  ┌───────▼──────┐
     │ 微软/XBL   │  │ Mojang/    │  │ Modrinth/    │
     │ Minecraft  │  │ BMCLAPI/   │  │ meta.fabric/ │
     │ services   │  │ Adoptium   │  │ meta.quilt   │
     └────────────┘  └────────────┘  │ forge/neo-   │
                                     │ forge maven  │
                                     └──────────────┘
```

**解耦原则：**
- Dart 只依赖 `flutter_rust_bridge` 生成的绑定，**绝不直接触碰文件系统/网络/进程**。
- 所有跨域调用必须走 `yuhina-bridge` 暴露的门面方法（见 `api-contract.md`）。
- Rust 内部按 crate 分域，域间通过 `yuhina-api` 中的公开类型交互，禁止跨 crate 私有依赖。
- 游戏本体的运行是**独立子进程**（Java 进程），Rust 负责 spawn/监控/转发日志。

## 4. 技术栈（版本基线）

| 组件 | 选型 | 版本基线 |
|---|---|---|
| Flutter | stable channel | ≥ 3.35（需 M3 Expressive 组件），锁定到**最新 stable** |
| Dart | 随 Flutter | ≥ 3.13（现有 pubspec 基线） |
| Rust | stable toolchain | 已装 1.97.1 |
| 桥接 | flutter_rust_bridge | v2.x（latest），`frb_codegen` + `flutter_rust_bridge_codegen` |
| 异步 | tokio + tokio-util | latest |
| HTTP | reqwest (rustls) | latest |
| SQLite | rusqlite (bundled) | latest |
| 序列化 | serde / serde_json | latest |
| 日志 | tracing + tracing-subscriber | latest |
| 解压/打包 | zip | latest |
| 系统密钥 | keyring（DPAPI/SecretService 后备） | latest |
| 状态管理 | Riverpod 2.x + freezed | latest |
| 路由 | go_router | latest |
| 本地化 | flutter_localizations + l10n.yaml (arb) | 内置 |
| 测试 | Rust: cargo test / Flutter: flutter test + integration_test | — |
| CI | GitHub Actions (ubuntu-latest + windows-latest) | — |

**前置环境要求（M0 由 CI Agent 落地）**：本机尚无 Flutter，需先安装并 `flutter doctor` 通过；Rust 已就绪。

## 5. 仓库布局

- **远程仓库**：`https://github.com/FallingSkyQwQ/Yuhina`（origin，main 分支）
- **微软 OAuth client_id**：`ff0aea8c-fc13-40b7-9f40-1c29fa20979b`（见 `04-agent-auth.md`）

```
/home/youzilm/Documents/projects/yuhina/      ← git 根 (main 分支)
├── docs/                          ← 全部规划文档（本目录即交付物）
│   ├── 00-master-plan.md
│   ├── api-contract.md
│   ├── agents/01-agent-core.md … 06-agent-ci.md
│   └── handoff.md
├── yuhina/                        ← 现有 Flutter 应用根（保留原位）
│   ├── lib/                       ← Dart UI
│   ├── rust/                      ← Cargo workspace（FRB 默认布局，与 lib 同级）
│   │   ├── Cargo.toml             ← workspace 根
│   │   ├── crates/
│   │   │   ├── yuhina-api/
│   │   │   ├── yuhina-db/
│   │   │   ├── yuhina-core/
│   │   │   ├── yuhina-download/
│   │   │   ├── yuhina-instance/
│   │   │   ├── yuhina-auth/
│   │   │   └── yuhina-bridge/     ← 被 FRB 扫描的 crate
│   │   └── rust/                  ← Rust 集成测试
│   ├── linux/  windows/           ← 平台 runner（已有）
│   ├── flutter_rust_bridge.yaml   ← FRB 配置
│   └── l10n.yaml
└── .github/workflows/             ← ci.yml / release.yml
```

## 6. Agent 划分与模块边界

| Agent | 文档 | 负责 crate/领域 | 输出 |
|---|---|---|---|
| **A · 核心引擎** | `agents/01-agent-core.md` | `yuhina-api`、`yuhina-db`、`yuhina-core` | 类型契约、数据库、版本下载编排、启动/进程 |
| **B · 下载网络** | `agents/02-agent-download.md` | `yuhina-download` | 下载管理器、镜像、断点续传、资讯、更新检查 |
| **C · 实例与Mod** | `agents/03-agent-instance-mod.md` | `yuhina-instance` | 实例 CRUD、加载器安装、Modrinth、依赖/冲突、mrpack 导入导出 |
| **D · 账号认证** | `agents/04-agent-auth.md` | `yuhina-auth` | 微软 OAuth、Yggdrasil、离线、会话加密存储 |
| **E · UI与桥** | `agents/05-agent-ui.md` | Flutter app + `yuhina-bridge` | Material3 Expressive UI、i18n、FRB 门面 |
| **F · CI与打包** | `agents/06-agent-ci.md` | workflows + 打包脚本 | CI 流水线、安装器、自动发布 |

依赖关系：
```
api-contract 冻结 (A/E 协同，M0)
      │
      ├──────────┬──────────┐
      ▼          ▼          ▼
   A(核心)    B(下载)    D(账号)      ← 三者并行，只依赖 api-contract + db schema
      │          │
      └────┬─────┘
           ▼
      C(实例/Mod)                ← 依赖 A 的版本安装 + B 的下载
           │
           ▼
      E(UI 整合)                 ← M0 用 mock 先做 UI，M3 后接入真实 FFI
           │
           ▼
      F(CI/打包)                 ← M0 起跑脚手架，M4/M5 收口
```

## 7. 里程碑 (Milestones)

| 里程碑 | 内容 | 完成门禁 (Gate) |
|---|---|---|
| **M0 地基** | git + 目录结构；Cargo workspace；FRB 集成并跑通「Dart 调 Rust 返回 hello」；`api-contract.md` 冻结；db schema + 迁移；CI 骨架 | `cargo build` + `flutter_rust_bridge_codegen generate` 成功；CI 绿 |
| **M1 核心启动** | 版本清单/元数据、下载编排、Java 运行时下载、命令行构建、进程监控；**离线启动**走通 | 本机 Linux 上成功启动一个离线 MC 实例并捕获日志 |
| **M2 实例+Mod** | 实例 CRUD、加载器安装、Modrinth 搜索/下载/启停/更新/冲突/依赖、mrpack 导入导出 | Fabric 实例装 2 个带依赖的 Mod 正常运行；导出 mrpack 可被本启动器重新导入 |
| **M3 账号** | 微软 OAuth(PKCE+loopback)、Yggdrasil、离线、加密会话 | 真实微软账号与 LittleSkin 账号均可登录并启动 |
| **M4 UI 收口** | 全部页面接真实 FFI、M3 Expressive 打磨、i18n 双语、日志/资讯、动效 | Linux 上手工 E2E 测试清单全过 |
| **M5 发布** | Windows 构建、NSIS/AppImage 安装器、GitHub Release 自动发布 | 打 tag → 双平台产物自动出现在 Release |

## 8. 协作与交接原则

- **接口先行**：`api-contract.md` 是 B/C/D/E 的「合同」，M0 冻结后改动必须经规划层评审并在文档同步。
- **分支策略**：`main` 受保护；每个 Agent 用 `feature/<agent>-<issue>` 分支，PR 至少 1 人评审。
- **交接物**：每个 Agent 文档末尾都有「交接清单」，明确产出的文件路径、测试通过项、给下游的注意事项。
- **流水线顺序**：见 `handoff.md` 的并行/串行矩阵与时间线。

## 9. 风险与对策

| 风险 | 影响 | 对策 |
|---|---|---|
| 微软 OAuth 需 Azure 应用配置 | 阻塞 M3 | 已有 client_id；规划已写明重定向 URI 需配 `http://127.0.0.1:<port>/callback`；E/D Agent 先 mock 再联调 |
| Forge/NeoForge 安装器为 Java jar，需 spawn 子进程 | 复杂且需特定 Java | 核心引擎统一 Java 版本选择；安装器失败时给出可读错误与日志 |
| FRB 与平台 FFI 在 CI 上的构建差异 | CI 红 | F Agent M0 起即接入 codegen + 构建，尽早暴露 |
| 下载镜像 URL 规则漂移 | 下载失败 | B Agent 维护独立 URL 重写模块 + 单测 + 镜像健康检查 |
| Mod 依赖冲突导致启动崩溃 | 体验差 | C Agent 冲突检测为「提示级」而非「阻断级」，M2 先做启发式，迭代加强 |
| Flutter 本机未安装 | 阻塞一切 | F Agent M0 首任务安装并锁定版本；文档记录安装命令 |

## 10. 完成定义 (DoD)

一个 Agent 的任务被判定为完成，必须满足：
1. 代码合入 `main`，PR 通过评审与 CI（fmt/clippy/test/analyze）。
2. 单测覆盖关键逻辑；对外契约行为与 `api-contract.md` 一致。
3. 交接清单中的条目全部完成并在 `handoff.md` 勾选。
4. 不留下 TODO 占位（若必须占位，需在交接清单显式说明由谁接手）。