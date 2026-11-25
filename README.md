# burncloud-template

BurnCloud 各 Rust crate 仓库的模板。仓库根的清单、锁文件和工具链可以直接复制到新仓库。`.github/workflows/ci.yml` 调用 [burncloud/ci](https://github.com/burncloud/ci) 的共享工作流，用来在另一个仓库里跑出真实的检查名。

调用方 `uses` 用 `@latest`。每次 [burncloud/ci](https://github.com/burncloud/ci) 合并到 main 之后，先打 `vX.Y.Z`，再把 `latest` 移到同一个提交。要钉死某一次发布，可以用 40 位完整 SHA，或不可移动的 `vX.Y.Z`。`v1`、`main` 这类其他浮动名仍然禁止。

## 用这个模板新建 crate 仓库

1. 用 GitHub 的模板仓库功能创建新仓库，或把本仓库里提交的文件复制到新仓库根目录。
2. 这些文件放进**默认分支的第一笔提交**。不要靠之后的 pull request 补 `.github/`、`rust-toolchain.toml` 或 `Cargo.toml` 的 `[lints]`，防作弊检查会拒绝。
3. 把包名、说明和库代码换成新 crate 的内容。保持 `edition`、`rust-version`、`[lints.rust] unsafe_code = "forbid"`，并重新生成后提交 `Cargo.lock`。
4. 不引入第三方依赖时，不必提交 `supply-chain/`。一旦依赖来自 crates.io，或其他不是 `git+https://github.com/burncloud/` 的来源，就要 `cargo vet init` 并提交 `supply-chain/`，否则 `cargo vet` 会失败。
5. 工具链继续用仓库根的 `rust-toolchain.toml`，不要改成浮动的 `stable`。

本模板不提交 `deny.toml`。共享工作流会从调用方仓库向上查找；找不到时使用 `burncloud/ci` 里的默认 `deny.toml`。只有策略和默认值不同时才在调用方提交自己的文件，而那个文件一旦提交就受保护。

## 从本模板新建仓库后必须做的设置

下面的设置由新建仓库的人来做。本模板只把要求写在这里，不改组织设置，也不改任何仓库的规则集。

调用方工作流 `.github/workflows/ci.yml` 的名字是 `CI`，不要改成 `Rust`。`uses` 用 `@latest`。检查名里的 `rust /` 来自 job id `rust`。

### 仓库必须公开

仓库必须是 public。GitHub 免费套餐下，私有仓库用不了规则集和分支保护。

### 单仓库规则集或分支保护

规则集 JSON 以 [burncloud/ci 的 `docs/rulesets/crate-main.json`](https://github.com/burncloud/ci/blob/main/docs/rulesets/crate-main.json) 为准。本仓库不复制这份 JSON。

在 `main` 上按那份 JSON 配置本仓库自己的规则集：

- 禁止直接推送，禁止强推，合并必须走 pull request。
- 至少 1 人批准，并且要求 Code Owner 审批。
- 推了新提交之后，已有批准作废，需要重新审批。
- 必需状态检查使用下面实测的 `rust / ...` 名字。每一项的 `integration_id` 都是 `15368`（GitHub Actions）。`rust / resolve` 的结论是 skipped，不要把它设为必需检查。
- `bypass_actors` 使用 `{"actor_type":"RepositoryRole","actor_id":5,"bypass_mode":"pull_request"}`。这是仓库管理员，而且只能在 pull request 里绕过，用来合并改了受保护路径的 PR。
- 免费套餐没有 `evaluate`。规则集先以 Disabled 建立，把 API 读回的内容和 `crate-main.json` 核对一致，再改成 Active。

仓库设置里还要做两件事：外部贡献者的 pull request 在跑 Actions 之前必须先批准；关闭「允许 GitHub Actions 创建和批准 PR」。

这次在私有调用方上实测到的 19 个检查名：

| 检查名 | 设为必需 |
| --- | --- |
| `rust / lint / fmt` | 是 |
| `rust / lint / clippy` | 是 |
| `rust / lint / unsafe` | 是 |
| `rust / test / cargo-test` | 是 |
| `rust / test / doc-test` | 是 |
| `rust / test / cargo-doc` | 是 |
| `rust / test / msrv` | 是 |
| `rust / test / extra-targets` | 是 |
| `rust / test / wit` | 是 |
| `rust / test / proptest` | 是 |
| `rust / test / fuzz` | 是 |
| `rust / test / mutants` | 是 |
| `rust / supply-chain / deny` | 是 |
| `rust / supply-chain / audit` | 是 |
| `rust / supply-chain / vet` | 是 |
| `rust / api / semver` | 是 |
| `rust / api / capabilities` | 是 |
| `rust / protect / protected-paths` | 是 |
| `rust / resolve` | 否。结论是 skipped |

### 审批账号

`.github/CODEOWNERS` 用前导斜杠写出仓库根上的受保护路径。`**/Cargo.toml` 和 `**/build.rs` 覆盖 workspace 成员的清单和构建脚本。`Cargo.lock` 不在里面。

每一行是 `@rustburn` 和 `@ivories`。rustburn 写代码，是仓库所有者，紧急时走管理员在 pull request 里的 bypass。ivories（GitHub id 12606587）负责审批，是 Code Owner。配好后执行：

```bash
gh api repos/{owner}/{repo}/codeowners/errors
```

返回的 `errors` 必须为空。然后再启用上一节的 Code Owner 审批。

### 局限

pull request 的作者可以修改 `.github/workflows/ci.yml`，例如换掉 `uses`，或把 job 改成空的。调用方这份 YAML 挡不住这种修改。`.github/` 已经交给 CODEOWNERS，安全性靠审批账号批准这些改动来兜底。

## 受保护、不能在 pull request 里改的路径

名单来自 `burncloud/ci` 的 `scripts/check-protected-paths.sh`。调用方不传 `protected-paths` 时用这份默认列表。名单匹配的是新增、修改和删除。

| 路径 | 含义 |
| --- | --- |
| `.github/` | 工作流和 Actions 配置，包含本仓库的 `ci.yml` |
| `.cargo/` | cargo 配置 |
| `vendor/` | 厂商化依赖 |
| `contracts/` | 契约文件 |
| `supply-chain/` | `cargo vet` 的审计策略 |
| `clippy.toml` | Clippy 配置 |
| `rustfmt.toml` | rustfmt 配置 |
| `deny.toml` | cargo-deny 配置 |
| `rust-toolchain` | 旧式工具链文件 |
| `rust-toolchain.toml` | 钉死的工具链 |
| `unsafe-crates.txt` | 允许使用 unsafe 的包名 |
| 删除仓库根 `Cargo.toml` | 不允许拿掉清单 |
| `Cargo.toml` 的 `lints`、`workspace.lints`、`patch`、`replace` | 用 TOML 解析后比较，内容变化即失败 |

豁免只在调用方和工作流定义仓库都是 `burncloud/ci` 时生效。本仓库改这些路径会失败。

包名、`rust-version`、`publish` 和库代码不在上面的表里，可以在 pull request 中修改。`[lints]` 不行。

## 本地复现

需要 bash、jq、python3，以及 `rust-toolchain.toml` 里的 `1.100.0`（含 `clippy` 和 `rustfmt`）。在仓库根执行：

```bash
cargo fmt --all --check
cargo clippy --locked --workspace --all-targets --all-features -- -D warnings -W clippy::pedantic -D clippy::allow_attributes_without_reason
cargo test --locked --workspace --all-targets --all-features
cargo test --locked --workspace --doc
RUSTDOCFLAGS='-D warnings' cargo doc --locked --workspace --no-deps --all-features
```

供应链三项需要与 `burncloud/ci` 相同的工具版本：cargo-deny 0.22.0、cargo-audit 0.23.0、cargo-vet 0.10.3。本仓库没有 `deny.toml`，deny 要显式指向 `latest` 上的默认文件：

```bash
git clone https://github.com/burncloud/ci.git .burncloud-ci
git -C .burncloud-ci checkout latest
cargo deny --config .burncloud-ci/deny.toml --manifest-path Cargo.toml check
cargo audit
bash .burncloud-ci/scripts/run-cargo-vet.sh .
bash .burncloud-ci/scripts/check-unsafe.sh .
```

`cargo audit` 在没有 `Cargo.lock` 时会失败，所以锁文件必须提交。没有第三方依赖时，`run-cargo-vet.sh` 用临时空策略运行 `cargo vet`，不需要把 `supply-chain/` 提交进仓库。
