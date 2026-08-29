# Agent E — Flutter UI 与 FFI 桥 (yuhina-bridge + app)

> 依据 `00-master-plan.md` 与 `api-contract.md` 执行。本 Agent 交付 UI 与桥接门面，是玩家直接体验层。

## 1. 职责范围

| 负责 | 不负责 |
|---|---|
| `yuhina-bridge`：FRB 门面 `YuhinaService` + 事件流装配 | Rust 领域逻辑（A–D） |
| 全部页面：首页/实例库/实例详情/Mod/下载中心/设置/日志/登录 | |
| Material 3 Expressive 主题 + 动效 | |
| 本地化 zh/en | |
| Riverpod 状态管理、go_router 路由 | |
| FFI 错误→UI 提示、事件流→状态同步 | |

## 2. 交付物（文件结构）

```
rust/crates/yuhina-bridge/src/{lib,service}.rs   ← FRB 扫描入口
yuhina/flutter_rust_bridge.yaml
yuhina/lib/{main.dart,
           app.dart,
           theme/app_theme.dart, theme/m3_expressive.dart,
           router/app_router.dart,
           core/{di.dart, bridge_provider.dart, event_bus.dart},
           features/
             home/{home_page.dart, news_section.dart, quick_start.dart, account_chip.dart},
             instances/{instances_page.dart, instance_card.dart, instance_detail_page.dart,
                        create_instance_sheet.dart, instance_edit_sheet.dart},
             mods/{mods_page.dart, mod_tile.dart, mod_detail_sheet.dart,
                   mod_search_page.dart, conflicts_banner.dart, updates_section.dart},
             downloads/{downloads_page.dart, task_tile.dart},
             settings/{settings_page.dart, accounts_tab.dart, mirrors_tab.dart,
                       java_tab.dart, general_tab.dart, about_tab.dart},
             logs/{logs_page.dart, log_view.dart},
             auth/{login_sheet.dart, microsoft_login_flow.dart, yggdrasil_form.dart},
           l10n/{app_zh.arb, app_en.arb}}
yuhina/integration_test/smoke_test.dart
```

## 3. 关键任务分解

### T1 FRB 集成与门面 (`yuhina-bridge`)
- 在现有 Flutter 项目配置 `flutter_rust_bridge.yaml`（`rust_input: crate::api`、`dart_output: lib/src/rust/`），引入 `flutter_rust_bridge` 依赖。
- 实现 `YuhinaService`：内部聚合 A–D 的 domain 服务，暴露契约 §3 全部方法；错误经 `anyhow` → `YuhinaError` 转换。
- 事件流装配：`watch_events`/`watch_progress` 用 `broadcast` channel 桥接各域。
- `lib/core/bridge_provider.dart`：`YuhinaService` 单例 + 启动初始化（`service = await YuhinaService.new(config)`）。
- **验收**：`flutter_rust_bridge_codegen generate` 成功；Dart 端可调用并收到事件流。

### T2 M3 Expressive 主题 (`theme/`)
- 基于 Flutter 3.35+ 组件：`FilledCard`、`ExpressiveButton`（如可用）、新版 `NavigationBar`（浮动胶囊指示器）、`Avatar` 渐变、大圆角（`ThemeData.cardTheme`/`dialogTheme` 28px）、tonal surface 层级。
- `ColorScheme.fromSeed(seedColor: config.theme_seed)`，支持深/浅色 + 跟随系统；设置页可选 seed。
- 动效：页面切换 300ms 曲线、列表 staggered、卡片 hover 抬升（桌面 hover 状态）。
- 无障碍：对比度校验、键盘导航、焦点环。
- **验收**：golden 测试覆盖首页/实例库（浅/深两态）；`flutter test` 通过。

### T3 路由与导航 (`router/`)
- `go_router`：`/` 首页、`/instances`、`/instances/:id`、`/instances/:id/mods`、`/downloads`、`/settings`、`/settings/:tab`、`/logs/:sessionId`、登录为弹层而非路由。
- `NavigationBar` 三项：首页 / 实例 / 下载；设置与日志从 AppBar 进入。
- **验收**：导航测试覆盖所有路由，含深链参数。

### T4 状态层 (Riverpod)
- Provider 层级：`serviceProvider` → 各 `FutureProvider`/`StreamProvider` 包装 FFI。
- 事件总线：`watch_events` 流驱动 invalidate：`AccountsChanged`→accounts、`InstancesChanged`→instances 等；`watch_progress` 驱动下载列表局部更新（节流已由 Rust 保证）。
- 游戏输出：每个 session 一个 `StreamProvider<GameOutput>`，UI 可筛选级别。
- 错误呈现：`YuhinaErrorKind` → 本地化文案 + `SnackBar`/对话框。
- **验收**：事件触发 → provider 刷新 → widget 更新的集成测试。

### T5 页面实现（按 M4 收口，先 mock 后接真）
1. **首页**：公告/资讯卡片（`fetch_news`）、快速启动（active instance + 账号 chip）、状态摘要。
2. **实例库**：网格卡片（icon/名称/版本/loader/mod 数/大小/最后启动）；新建实例底部弹层（名称/图标/MC 版本/loader）；右键/菜单：启动/复制/重命名/删除/打开目录。
3. **实例详情**：大播放按钮、版本与 Java 选择、启动参数（内存/分辨率/JVM/GC）、Mod 标签、日志入口。
4. **Mod 管理**：已装列表（启停开关、更新红点、冲突 banner）、搜索/安装弹层（Modrinth 结果 + 版本选择 + 依赖说明）、文件安装。
5. **下载中心**：任务列表（进度条/速度/状态）、暂停/恢复/取消/清除；整合包下载入口。
6. **设置**：账号管理（登录/激活/退出）、镜像与源、Java 管理（扫描/手动/下载/删除）、常规（语言/主题/自更新）、关于。
7. **日志页**：实时输出 + 级别过滤 + 崩溃报告摘要 + 导出/打开日志文件。
8. **登录**：微软流程（进度态 + 浏览器提示 + 轮询）、Yggdrasil 表单（预设 + 自定义 URL）、离线表单。
- **验收**：M4 手工 E2E 清单全过（清单写入 `handoff.md`）；widget 测试覆盖关键交互。

### T6 本地化 (`l10n/`)
- 配置 `l10n.yaml` + `intl`，arb 双语文案（zh/en），全部用户可见文案走 `AppLocalizations`。
- 错误枚举、日期格式、大小格式化本地化。
- **验收**：`flutter gen-l10n` 无缺 key 警告；切换语言即时生效。

### T7 集成测试 (`integration_test/`)
- 冒烟：初始化 → 建离线实例 → 启动 → 收到日志 → 停止；全走真实 FFI（CI 用 xvfb，Linux 上可跑）。
- **验收**：Linux CI 冒烟通过（M4/M5 gate）。

## 4. 依赖与前置
- 依赖全部 Rust crate（经 `yuhina-bridge`）。
- **前置**：`api-contract.md` 冻结（M0）；M0–M2 期间用 mock 数据源驱动 UI 并行开发；M3 后逐步切真。

## 5. 测试策略
- 单元：provider/state/format 函数。
- Widget：页面渲染 + 关键交互（golden 首页/实例库）。
- 集成：`integration_test` 真实启动冒烟（Linux CI）。
- 手工 E2E：登录/启动/装 Mod 清单在 M4 门禁执行。

## 6. 交接清单（给下游/上游）
- [ ] `flutter_rust_bridge.yaml` + 门面合入，codegen 可复现（CI 校验）。
- [ ] 全部页面与本地化合入；golden 图入库。
- [ ] 手工 E2E 清单更新至 `handoff.md`（登录、启动、Mod、整合包、日志）。
- [ ] 反馈给 A–D 的契约偏差（若有）走 `api-contract.md` 变更流程。