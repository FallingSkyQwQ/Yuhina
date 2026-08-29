# Agent 交接矩阵与里程碑 (handoff)

> 本文档由规划层维护，Agent F 负责勾选里程碑。所有 Agent 完成工作后回填交接清单。

## 1. 并行/串行矩阵

```
                    M0 冻结 api-contract + db schema (由 A 首跑)
                    ┌────────────┬────────────┬─────────────┐
                    ▼            ▼            ▼             ▼
               A 核心引擎    B 下载网络   D 账号认证    F CI/打包
               (api+db+core) (download)   (auth)       (workflows)
                    │            │                          │
                    └──────┬─────┘                          │
                           ▼                                │
                        C 实例与Mod                          │
                   (instance+modrinth+mrpack)                │
                           │                                │
                           ├──────────────┬─────────────────┘
                           ▼              ▼
                        E UI整合     （E 的 UI 骨架在 M0 即并行，mock 数据源）
                           │
                           ▼
                     M4 手工 E2E → M5 发布
```

| Agent | 启动时机 | 等待 | 交付给 |
|---|---|---|---|
| A | 立即（先冻结 api/db） | 无 | B/C/D/E 的类型与 schema |
| B | api-contract 冻结后 | A 的 `api`/`db` | A（下载）、C、E |
| D | api-contract 冻结后 | A 的 `api`/`db` | A（启动参数）、E |
| F | 立即（脚手架） | 无 | 全部 |
| C | A+B 可用后 | A 的 loader/下载、B 的下载 API | E |
| E | 立即（mock UI 并行） | 真 FFI 在 M3 后接入 | 手工 E2E |

## 2. 里程碑勾选

- [ ] **M0 地基**：Cargo workspace 构建通过；FRB 集成 + codegen 跑通；db schema 合入；`ci.yml` 绿。
- [ ] **M1 核心启动**：Linux 离线启动真实 MC 实例成功并捕获日志。
- [ ] **M2 实例+Mod**：Fabric 实例安装带依赖 Mod 运行正常；mrpack 导出→重新导入成功。
- [ ] **M3 账号**：真实微软账号与 LittleSkin 账号登录并启动成功。
- [ ] **M4 UI 收口**：手工 E2E 清单（§3）全过。
- [ ] **M5 发布**：tag → 双平台安装包/便携包自动发布到 GitHub Release。

## 3. 手工 E2E 清单（M4 执行）

### 登录
- [ ] 微软：浏览器弹出 → 授权 → 回跳 → 账号显示且 active
- [ ] LittleSkin：填入预设服务器账号密码 → 登录成功 → 皮肤正确
- [ ] 离线：输入任意名称 → 生成标准 UUID
- [ ] 换用另一个账号再启动，游戏内名字正确

### 启动
- [ ] 离线实例启动原版 MC 到主菜单（可借助 `--quickPlaySingleplayer` 验证到主菜单）
- [ ] 带 Forge/Fabric 实例启动成功，游戏内 mods 生效
- [ ] 停止按钮可优雅退出游戏；强杀兜底生效
- [ ] 日志页实时滚动、级别过滤、崩溃报告可读

### Mod 与整合包
- [ ] 搜索 Modrinth → 安装 → 重启实例生效
- [ ] 依赖 mod 自动安装；缺失依赖给出提示
- [ ] 更新检测 → 一键更新到最新兼容版
- [ ] 冲突（同 modid）提示可见
- [ ] 导出 mrpack → 新建实例导入 → 文件一致

### 设置与下载
- [ ] 镜像切换（官方↔BMCLAPI）后重新安装实例成功
- [ ] Java 自动下载 + 手动指定路径均可用
- [ ] 下载中心暂停/恢复/取消正常；重启启动器后续传恢复

## 4. 慢速测试清单（`#[ignore]`，本地/CI 缓存执行）

| 测试 | 归属 | 说明 |
|---|---|---|
| 真实 release 版本元数据解析 | A | 需外网或缓存；`yuhina-core` 已用离线 fixture 单测覆盖，`version_list_cached` 网络测试标 `#[ignore]` |
| Forge/Fabric/NeoForge/Quilt installer 安装 | A/C | 需真实 jar + 匹配 Java；A 已提供 `install_loader` 编排 + 版本解析（离线 fixture 单测），真实安装为 `#[ignore]` 慢测 |
| 真实 Java 启动子进程 | A | 需已装 Java；进程模块用假进程（`sh`）集成测试覆盖流/退出码，真实 java 启动为 `#[ignore]` |
| Modrinth 真实搜索/版本拉取 | C | 需外网 |
| 真实微软/XBL/XSTS 联调 | D | 手动，不进 CI |

## 5. 分支与评审约定

- `main` 受保护，`ci.yml` 全绿才可合入。
- 分支命名 `feature/<agent>-<slug>`，如 `feature/a-db-schema`、`feature/e-instances-page`。
- PR 描述必须含：变更摘要、与 api-contract 的偏差（若无则写明「无偏差」）、测试证据、交接项。
- `api-contract.md` 的破坏性变更需规划层评审并在本文件记录影响面。

## 6. 版本号与自更新

- 单一版本来源：`yuhina/pubspec.yaml` `version:`（如 `0.1.0+1`）。tag 格式 `v0.1.0`。
- 自更新产物 URL 模板：`https://github.com/FallingSkyQwQ/Yuhina/releases/latest/download/yuhina-{ver}-{os}-{arch}.{ext}`。
- Agent B `check_launcher_update` 与该模板对齐。