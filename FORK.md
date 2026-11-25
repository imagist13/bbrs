# bbrs：Fork 说明与进度

> 本文档记录 `bbrs` 作为 aionrs fork 的来源、改名范围、刻意保留项、当前进度和后续待办。
> 状态截至 2026-11-25。

## 1. 来源

| 项目 | 值 |
|---|---|
| 上游仓库 | [iOfficeAI/aionrs](https://github.com/iOfficeAI/aionrs) |
| 基线版本 | tag `v0.2.11` |
| 基线 commit | `8e61a90329fa9f67c4fdf7e97fe02c24dba33f75` |
| 许可证 | Apache-2.0（`LICENSE` 原文未改） |
| 本仓库位置 | `workspace/bbrs` |
| 用途 | Buckyball 垂类 agent 的可自由修改基线 |

选择 `v0.2.11` 而不是上游最新 `main`，是为了与 `bb-agent-core`
当前实际锁定的版本保持一致，便于后续接入时行为对齐。

## 2. 已完成的改名

### 2.1 crate 改名

| 原 | 新 |
|---|---|
| `aion-agent` | `bbrs-agent` |
| `aion-cli` | `bbrs-cli` |
| `aion-compact` | `bbrs-compact` |
| `aion-config` | `bbrs-config` |
| `aion-mcp` | `bbrs-mcp` |
| `aion-memory` | `bbrs-memory` |
| `aion-process` | `bbrs-process` |
| `aion-protocol` | `bbrs-protocol` |
| `aion-providers` | `bbrs-providers` |
| `aion-skills` | `bbrs-skills` |
| `aion-tools` | `bbrs-tools` |
| `aion-types` | `bbrs-types` |

同步修改的内容：目录名、`[package] name`、`Cargo.toml` 内部依赖键、
`[workspace.dependencies]`、全部 `use aion_*` 与 `aion_*::` 调用路径、
`justfile` 与 `.github/workflows/*.yml` 中的 `-p` 参数。
`workspace-hack/` 只依赖第三方 crate，无需修改。

使用 `bbrs-*` 前缀而不是 `bb-*`，是为了避免与 `bb-agent-core`
已有的 `bb-mcp`、`bb-process` 重名。

### 2.2 运行期身份

| 项目 | 原 | 新 |
|---|---|---|
| CLI 二进制 | `aionrs` | `bbrs` |
| 配置目录 | `~/.config/aionrs` | `~/.config/bbrs` |
| 项目配置 | `.aionrs.toml` | `.bbrs.toml` |
| 项目数据 | `.aionrs/{sessions,plans,skills,commands}` | `.bbrs/...` |
| 环境变量 | `AIONRS_SKILL_DIR` 等 | `BBRS_SKILL_DIR` 等 |
| 发布归档 | `aionrs-v*` | `bbrs-v*` |
| release-please | `aionrs` | `bbrs` |

### 2.3 文档归属

- `README.md` 顶部增加 Fork 声明，注明上游、tag、commit 与许可证。
- `CHANGELOG.md` 顶部增加说明：历史条目属于上游，fork 后继续沿用。
- 历史条目、`LICENSE` 原文均未改写。

## 3. 刻意保留（不要改）

| 保留项 | 位置 | 原因 |
|---|---|---|
| `client_id = "aionrs"` | `crates/bbrs-config/src/auth.rs`、`crates/bbrs-config/src/config.rs` 模板 | 上游注册的 OAuth 凭据，Claude.ai 服务端校验该值，改名会导致登录失效 |
| `[[AION_FILES]]` 标记 | `crates/bbrs-agent/src/engine_test.rs` | bb-agent-core 的宿主线协议标记，不属于 aionrs 自身 |
| `iOfficeAI/aionrs` 链接 | README / CHANGELOG | 上游归属与溯源 |
| `LICENSE` | 仓库根目录 | Apache-2.0 原文 |

除以上白名单与上游 URL 外，仓库内已无 `aion` 字样残留。

## 4. 当前仓库状态

- `.git` 已重新初始化：分支 `main`，初始提交
  `8397926 chore: fork aionrs v0.2.11 as bbrs`（后续提交见 `git log`），
  remote `origin` 指向 `https://github.com/imagist13/bbrs.git`；上游 aionrs
  历史不在本仓库中。
- 发布 tag：`v0.1.0`。这是 fork 自身的版本号，与 crate 的
  `version = "0.2.11"`（上游基线版本）不是同一套编号，见第 7 节。
- `Cargo.lock` 已按新包名重新生成：`aion-*` 条目 0 个，`bbrs-*` 条目 12 个。
- 工具链：`rust-toolchain.toml` 钉 `1.96.1`，本机已安装。
- 构建产物：`target/debug/bbrs.exe` 已生成。
- `README.md` 已补充 BBUI 角色说明与 fork 定位。

## 5. 验证进度

已通过：

- `Cargo.lock` 包名审计（0 个 `aion-*`，12 个 `bbrs-*`）
- `justfile`、`release.yml`、`release-please-config.json`、`.cargo/config.toml`、
  `.config/hakari.toml` 一致性检查
- 全仓 `aion` 残留审计（仅剩第 3 节白名单）
- CLI 构建与运行期身份：`target/debug/bbrs.exe --version` 输出 `bbrs 0.2.11`
- 作为依赖被宿主编译：`bb-agent-core` 的 `bb-ai-agent`、`bb-team` 测试目标
  在 `tag = "v0.1.0"` 解析出的源码上编译通过（11 个 `bbrs-*` crate，
  `bbrs-cli` 除外）

未单独在本仓库执行（宿主构建只覆盖编译，不覆盖本仓库自有测试）：

- `cargo test --workspace`
- `cargo clippy --workspace`
- `cargo fmt --all -- --check`

## 6. 后续待办

在本仓库依次执行：

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets
cargo test --workspace
```

已发布的 `v0.1.0` tag 不可移动；需要包含新提交时另打新 tag（例如 `v0.1.1`），
再同步 `bb-agent-core/Cargo.toml` 的 tag。

## 7. 与 bb-agent-core 的接入现状

- `bb-agent-core/Cargo.toml` 以
  `git = "https://github.com/imagist13/bbrs.git", tag = "v0.1.0"` 钉住六个
  crate：`bbrs-agent`、`bbrs-providers`、`bbrs-types`、`bbrs-protocol`、
  `bbrs-config`、`bbrs-mcp`；其余 crate 传递引入，共 11 个，`bbrs-cli` 不参与。
- **tag 与 crate 版本是两套编号**：tag `v0.1.0` 是 fork 的发布号，crate 的
  `version = "0.2.11"` 是上游基线版本。`Cargo.lock` 里二者同时出现
  （`version = "0.2.11"` + `source = "...?tag=v0.1.0#<commit>`）属于预期。
  判断宿主编译用的是哪份源码，看 `Cargo.lock` 的 commit 哈希，不要看 tag 号。
- 本地联调走 `scripts/just/cargo.{sh,ps1}` 的 `BBRS` 环境变量：设置后按路径
  覆盖依赖，并校验 patch 确实生效；未设置时从 git tag 拉取。
- 接入后宿主侧不再有 `use aion_*`；`bb-app` 的未使用 `aion-config` 依赖和两处
  泄漏到公共 API 的 aion 类型（`AionrsResolvedConfig`、
  `sanitize_session_messages`）已在接入时清理。
