# Agent F — CI/CD 与打包发布

> 依据 `00-master-plan.md` 与 `api-contract.md` 执行。本 Agent 从 M0 起跑，M5 收口。

## 1. 职责范围

| 负责 | 不负责 |
|---|---|
| 本地工具链安装指引（Flutter/Rust/代码生成） | 应用功能代码 |
| GitHub Actions 工作流（CI 门禁 + Release 构建） | |
| FRB codegen 可复现性与 CI 校验 | |
| 打包：Linux tar.gz + AppImage；Windows zip + NSIS 安装器 | |
| 自动发布 GitHub Release + 自更新产物 URL 约定 | |
| 版本号与自更新（配合 B 的 update 模块） | |

## 2. 交付物（文件结构）

```
.github/workflows/{ci.yml, release.yml}
build/                            ← 打包脚本
build/linux/{package_linux.sh, yuhina.appdata.xml, AppImageBuilder.yml}
build/windows/{package_windows.ps1, installer.nsi}
docs/handoff.md                   ← 由 F 维护里程碑勾选
```

## 3. 关键任务分解

### T1 工具链安装（M0 第一件事）
- 本机（Linux）需装 Flutter stable：文档化 `git clone -b stable https://github.com/flutter/flutter.git ~/flutter` + PATH + `flutter doctor`。
- Rust 已装（1.97.1）；`rustup component add rustfmt clippy`。
- FRB 工具：`cargo install flutter_rust_bridge_codegen`（锁定与 pubspec 一致的版本）。
- **验收**：`flutter --version`、`cargo --version`、`frb_codegen --version` 可执行。

### T2 CI 工作流 (`ci.yml`)
- 触发：push 到 main、所有 PR。
- Job 矩阵：`ubuntu-latest` + `windows-latest`。
- 步骤：
  1. checkout + 缓存（cargo registry、`~/.pub-cache`）
  2. 装 Rust/Flutter（`subosito/flutter-action@v5` 锁 stable）
  3. `cargo fmt --check` + `cargo clippy --workspace -- -D warnings`
  4. `cargo test`（workspace，`#[ignore]` 慢测除外）
  5. `flutter_rust_bridge_codegen generate` 后 `git diff --exit-code`（校验契约一致）
  6. `flutter analyze` + `flutter test`
  7. Linux 额外：`xvfb-run flutter test integration_test`（冒烟，缓存了 MC 版本数据时）
- **验收**：PR 全绿才可合入；契约漂移（codegen diff）直接红。

### T3 Release 工作流 (`release.yml`)
- 触发：push tag `v*`。
- 矩阵同上，产出：
  - Linux：`tar.gz`（portable，含可执行 + 运行库）+ `AppImage`（`appimagetool`，FUSE 版打包）
  - Windows：`zip`（portable）+ NSIS `Setup.exe`（makensis，含安装/卸载/桌面快捷方式/可选便携模式）
- 版本读取：单一来源 `pubspec.yaml version`，脚本生成 tag 校验；自更新模块（B）用该版本号。
- 发布：`softprops/action-gh-release@v2` 草稿 Release，附件上传；命名 `yuhina-{ver}-{os}-{arch}.{ext}`。
- **验收**：打 tag → 双平台产物出现在 Release 草稿，可下载安装。

### T4 打包细节
- Linux AppImage：用 `linuxdeploy` 收集 Qt/gtk 依赖（GTK3/GL 库）；`AppImageBuilder.yml` 含 icon/appdata。
- Linux tar.gz：复制构建产物 + 版本文件 + README；不依赖 system 缺失库。
- Windows：`flutter build windows --release`；`installer.nsi` 引 `dist/`，含 VC runtime（`vcredist` 可选合并）、图标。
- 自更新产物 URL 约定：`https://github.com/{owner}/{repo}/releases/latest/download/{filename}`，B 模块按此拼接。
- **验收**：在干净 CI runner 上 AppImage 可执行启动（`--no-sandbox` 兼容性验证）；Windows 安装器安装后桌面图标可启动（手动）。

### T5 里程碑维护
- 维护 `handoff.md` 的里程碑勾选与慢测清单（`#[ignore]` 测试的本地执行指引）。
- **验收**：每次 M 里程碑收官时勾选状态与代码一致。

## 4. 依赖与前置
- 依赖所有 Agent 的代码合入节奏；工作流骨架 M0 即建，逐步加断言。
- **前置**：git 已 init（done）；GitHub 远程仓库由产品方创建后接入 secrets（如有）。

## 5. 测试策略
- CI 本身就是主要验证手段；本地打包脚本用 shellcheck/`-ErrorAction Stop` 防御。
- AppImage/NSIS 产物做「启动冒烟」：Linux 用 `timeout 10 ./yuhina.AppImage` 检查进程存活；Windows 留手动清单。

## 6. 交接清单（给下游）
- [ ] 工具链安装指引文档化（本机可复现）。
- [ ] `ci.yml` 全绿接入 main 保护。
- [ ] `release.yml` 产出双平台安装包/便携包，命名规范可用。
- [ ] 自更新版本号约定与 B 模块对齐。
- [ ] 里程碑勾选清单最新。