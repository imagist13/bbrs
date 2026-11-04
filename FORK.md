# bbrs：Fork 说明与进度

> 本文档记录 `bbrs` 作为 aionrs fork 的来源、改名范围、刻意保留项、当前进度和后续待办。
> 状态截至 2026-11-03。

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

- `.git` 已由用户重新初始化：分支 `main`，**0 个提交**，无 remote，
  上游历史不在本仓库中。
- `Cargo.lock` 已按新包名重新生成：`aion-*` 条目 0 个，`bbrs-*` 条目 12 个。
- 工具链：`rust-toolchain.toml` 钉 `1.96.1`，本机已安装。
- 构建产物：`target/` 尚未生成（构建验证未完成）。

## 5. 验证进度

已通过：

- `Cargo.lock` 包名审计（0 个 `aion-*`，12 个 `bbrs-*`）
- `justfile`、`release.yml`、`release-please-config.json`、`.cargo/config.toml`、
  `.config/hakari.toml` 一致性检查
- 全仓 `aion` 残留审计（仅剩第 3 节白名单）

未完成：

- `cargo check --workspace --all-targets`
  首次依赖下载经由代理速度过慢（单个 crate 约 8 秒），中途停止，未得到结果。
- `cargo test -p bbrs-config -p bbrs-skills -p bbrs-types`
- `cargo run -p bbrs-cli -- --help`

## 6. 后续待办

网络恢复后依次执行：

```bash
cargo check --workspace --all-targets
cargo test -p bbrs-config -p bbrs-skills -p bbrs-types
cargo run -p bbrs-cli -- --help     # 应显示 bbrs，版本 0.2.11
```

全部通过后提交并打 tag：

```bash
git add -A
git commit -m "chore: fork aionrs v0.2.11 as bbrs"
git tag v0.2.11-bbrs.1
```

remote 由用户自建仓库后配置，未配置前不要 push。

## 7. 后续接入 bb-agent-core 的注意事项（本次不做）

改动面已核实：

- 仅 `bb-ai-agent` 引用 aion crate，涉及约 15 个文件、38 处 `use aion_*`。
- `bb-app/Cargo.toml` 有一个未使用的 `aion-config` 依赖。
- 两处 aion 类型泄漏到公共 API：`AionrsResolvedConfig` 字段、
  `sanitize_session_messages` 的签名。
- `bb-agent-core/scripts/just/cargo.{sh,ps1}` 已有 `AIONRS` 环境变量做本地
  patch 的能力，接入时需同步改为新版包名。
