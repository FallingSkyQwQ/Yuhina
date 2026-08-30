# Agent D — 账号与认证 (yuhina-auth)

> 依据 `00-master-plan.md` 与 `api-contract.md` 执行。

## 1. 职责范围

| 负责 | 不负责 |
|---|---|
| 微软 OAuth（授权码+PKCE，localhost loopback 回调） | 版本/下载（A/B） |
| 设备码流程兜底 | 实例/Mod（C） |
| Yggdrasil（LittleSkin 预设 + 自定义服务器） | UI（E） |
| 离线账号（标准离线 UUID 算法） | |
| Token 刷新 / 过期处理 / 加密存储 | |
| 会话状态事件 → `AccountsChanged` | |

## 2. 交付物（文件结构）

```
rust/crates/yuhina-auth/src/{lib,ms_auth,yggdrasil,offline,store,crypto}.rs
rust/rust/                    ← 集成测试 (yggdrasil_mock_test.rs, ms_mock_test.rs)
```

## 3. 关键任务分解

### T1 加密存储 (`crypto.rs` / `store.rs`)
- 优先 `keyring` crate：Windows 用 Credential Manager，Linux 用 SecretService（无桌面时提示降级）。
- 降级路径：本地 AES-256-GCM，密钥存 `<data_dir>/secret.key`（chmod 0600）。
- 封装 `Store::save_account(Account, tokens)` / `load_account` / `purge`，token 永远密文。
- **验收**：加密/解密 round-trip 单测（可注入内存 keyring）；token 不出现在日志与明文序列化中。

### T2 离线账号 (`offline.rs`)
- `add_offline_account(username)`：校验非空、长度 ≤16、合法字符；UUID = `MD5("OfflinePlayer:" + name)` 标准算法（与 HMCL/Prism 一致）。
- 返回 `Account { kind: Offline, uuid, is_active }`，无需 token。
- **验收**：UUID 算法单测（固定输入断言固定输出）。

### T3 微软 OAuth (`ms_auth.rs`)
- 流程：授权码 + PKCE（`S256`），`client_id` 来自配置（真实值注入，代码用占位 + 环境变量覆盖）。
- **真实 client_id（产品方提供，Azure 应用）**：`ff0aea8c-fc13-40b7-9f40-1c29fa20979b`，写死为默认值；环境变量 `YUHINA_MS_CLIENT_ID` 可覆盖（CI mock 用）。重定向 URI 需在 Azure 中登记 `http://127.0.0.1:<port>/callback`（loopback 端口任意，Azure 支持通配 loopback）。
- 回调方式：Rust 内嵌 `tiny_http` 监听 `127.0.0.1:<随机高位端口>/callback`，弹出系统浏览器打开微软授权页（`open` crate / `xdg-open` / `cmd /c start`）；收到 code 即关闭监听。
- 后续换取与验证链：
  1. code → access/refresh token（`login.microsoftonline.com/consumers/oauth2/v2.0/token`）
  2. XBL：`user.auth.xboxlive.com/user/authenticate`（RelyingParty `http://auth.xboxlive.com`）
  3. XSTS：`xsts.auth.xboxlive.com/xsts/authorize`（RelyingParty `rp://api.minecraftservices.com/`）
  4. Minecraft：`api.minecraftservices.com/authentication/login_with_xbox`
  5. 资料：`api.minecraftservices.com/minecraft/profile`（uuid + name + skins）
- `begin_microsoft_login` 返回 handle；`poll_microsoft_login` 非阻塞返回 `Ok(None)` 表示仍在等待，`Ok(Some(account))` 成功；失败返回带原因的 `Auth` 错误。取消时清理监听与临时文件。
- Token 刷新：`refresh_account` 用 refresh_token 刷新，`AuthExpired` 引导重登。
- **验收**：mock http 服务器模拟完整 5 步链，断言请求体/header（含 `Authorization` 透传）；回调服务器端口回收正确；取消路径清理。

### T4 Yggdrasil (`yggdrasil.rs`)
- 内置预设：LittleSkin（`https://littleskin.cn/api/yggdrasil`），另留 `authlib-injector` 风格通用服务器入口。
- 流程：`POST {server}/authserver/authenticate`（agent: Minecraft, username, password, clientToken）→ 存 `accessToken` + `clientToken`；`GET {server}/sessionserver/session/minecraft/profile/{uuid}` 取皮肤。
- 刷新：`POST /authserver/refresh`（accessToken + clientToken）。
- 多账号并存：不同 `clientToken` 隔离。
- **验收**：mock Yggdrasil 服务器 round-trip（authenticate→profile→refresh）；错误（密码错/封禁）映射为可读 `YuhinaError`。

### T5 会话管理 (`lib.rs` 门面)
- 实现契约 §3.2 全部方法；`set_active_account` 保证全局唯一 active。
- `launch_instance` 时由 A 侧调 `get_active_account` 拿 auth 参数（`username/uuid/accessToken/user_type`），Offline 生成 `accessToken: "0"`、`user_type: "legacy"`。
- 状态变化广播 `AppEvent::AccountsChanged`。
- **验收**：契约方法全实现；激活切换单测；离线账号可被 A 消费。

## 4. 依赖与前置
- 依赖 `yuhina-api`、`yuhina-db`（`AccountRepo`）。
- **前置**：`api-contract.md` 冻结。微软 client_id 为真实值（已有），另设环境变量 `YUHINA_MS_CLIENT_ID` 便于 CI mock。

## 5. 测试策略
- 网络全部 mock（本地 axum 模拟微软/XBL/Yggdrasil）。
- 真实微软联调为手动用例（M3 gate 时执行一次，不进 CI）。
- 密钥存储注入内存实现做单测；CI 上 keyring 需 Linux SecretService——降级路径单测在 CI 跑。

## 6. 交接清单（给下游）
- [ ] 契约 §3.2 全部方法合入，A 的启动参数可消费 active account。
- [ ] E 侧登录页可联调 `begin/poll_microsoft_login` 与 Yggdrasil 表单。
- [ ] 真实微软 + LittleSkin 手动联调清单写入 `handoff.md`。
- [ ] 未完成/风险项在 PR 描述中明示。