# AGENTS.md — hyperlane-quick-start Workspace Engineering & Documentation Governance

Mandatory architectural, engineering, formatting, container delivery, release, and documentation governance rules for all contributors and AI coding agents operating in the `hyperlane-quick-start` repository (`hyperlane-dev/hyperlane-quick-start`).

---

## 1. Repository Architecture & Layered Workspace Matrix

`hyperlane-quick-start` is a Rust 2024 unified Cargo workspace (resolver `3`) providing a production-ready starter service built on `hyperlane`, `sea-orm`, `redis`, and Tokio. The root package produces the `hyperlane-quick-start` binary (`src/main.rs`) wired across **5** strictly layered workspace crates sharing a synchronized workspace version:

| Layer                  | Crate                   | Path           | Primary Responsibility                                                                                                                                                                                                           |
| :--------------------- | :---------------------- | :------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| **Executable Root**    | `hyperlane-quick-start` | `.` (`src/`)   | Binary entry point orchestrating `EnvBootstrap`, `LoggerBootstrap`, `RuntimeBootstrap`, `DbBootstrap`, `ProcessPlugin`, and `ServerBootstrap`                                                                                    |
| **Resources**          | `hyperlane_resources`   | `resources/`   | Environment templates (`env/{dev,release}/server.env`), Dockerfiles & Compose manifests (`docker/{dev,release}/`), SQL scripts (`sql/`), templates (`templates/`)                                                                |
| **Configuration**      | `hyperlane_config`      | `config/`      | Static application and framework constants (`application/logger`, `application/logo_img`, `framework`); zero database or HTTP business logic                                                                                     |
| **Infrastructure**     | `hyperlane_plugin`      | `plugin/`      | Runtime infrastructure adapters (`database`, `env`, `logger`, `mysql`, `postgresql`, `process`, `redis`, `shutdown`) via `OnceLock`/`GetOrInit`; no HTTP routes                                                                  |
| **Application Domain** | `hyperlane_application` | `application/` | HTTP controllers, middleware (`HttpRequestMiddleware`, `CrossMiddleware`, `ResponseHeaderMiddleware`, `SendMiddleware`, `LogMiddleware`), views, exceptions, models, services, repositories, mappers, domain, and response utils |
| **Composition Root**   | `hyperlane_bootstrap`   | `bootstrap/`   | Startup orchestration (`application/{db,env,logger}`, `framework/{config,runtime,server}`) registering `hyperlane_application` hooks via `#[hyperlane(server: Server)]` and `#[import(hyperlane_application)]`                   |

### Core Architectural Invariants

1. **Strict Acyclic Layer Boundaries**:
  - Dependency flow is strictly one-way: `hyperlane_resources` -> `hyperlane_config` -> `hyperlane_plugin` -> `hyperlane_application` -> `hyperlane_bootstrap` -> `hyperlane-quick-start`.
  - `hyperlane_config` must remain pure static configuration; never add database pools, network clients, or route handlers to `config/`.
  - `hyperlane_plugin` must remain HTTP-route-agnostic infrastructure; never add controllers or views to `plugin/`.
  - `hyperlane_resources` must never commit production secrets, credentials, or generated build outputs.

2. **`hyperlane` Wire & `ServerHook` Contracts**:
  - Register routes, request/response middlewares, task-panic hooks, and request-error hooks synchronously via Hyperlane's attribute/inventory mechanism (`#[hyperlane(server: Server)]` + `#[import(hyperlane_application)]`).
  - Propagate or explicitly log errors from `server.run().await`; wire `ShutdownPlugin::set(server_hook.get_shutdown_hook())` before awaiting `server_hook.wait().await`.
  - Keep `#[prologue_macros]` / `#[epilogue_macros]` hook semantics consistent with `hyperlane-core` (`Status::Continue` vs `Status::Reject` and HEAD-aware response serialization).

3. **Clean Repository Hygiene**:
  - Never commit generated runtime artifacts: `data/`, `*.log`, `*.pid`, `dist/`, `target/`, `.codegraph/`, or untracked `resources/static/` build outputs.

---

## 2. Engineering, Rust & Unified Formatting Standards

1. **Rust Standards (Highest Priority)**: Follow `~/.agents/skills/rust-standards/SKILL.md` without exception:
  - Pure 9-keyword module filenames (`mod.rs`, `const.rs`, `enum.rs`, `struct.rs`, `trait.rs`, `impl.rs`, `fn.rs`, `type.rs`, `static.rs`).
  - Three-section `mod.rs` with zero comments; centralized imports in crate roots (`lib.rs`, `main.rs`, `tests/mod.rs`) and `use super::*;` in submodules.
  - Explicit type annotations on every `let` binding and closure parameter; generic bounds in `where` clauses; zero `#[allow(...)]`; zero `use ... as ...`; zero `.unwrap()`/`.expect()`/`panic!()` in production code; zero comments in `tests/` or `*.toml`.
  - Strict `[workspace.dependencies]` and `[dependencies]` ordering (workspace crates first, blank line, third-party sorted by name length then lexicographically; verified with `verify_dep_order.py`).

2. **Unified Formatting (`crate fmt`)**:
  - Format Markdown, YAML, JSON, TOML, and Rust files with `crate fmt` (aligned with VS Code Prettier `tabWidth: 2`, `singleQuote: true`, `printWidth: 80`, `proseWrap: "preserve"`, `endOfLine: "lf"`) and confirm zero diff with `crate fmt --check` and `cargo fmt --all -- --check`.

3. **Commit & PR Hygiene**:
  - All code, comments, commit messages, PR titles/bodies, docs, and skills must be pure English and pass `~/.agents/skills/git-standards/scripts/verify_english_only.py`.
  - Never bypass `~/.git-hooks/pre-commit` (`--no-verify` is forbidden).

---

## 3. Runtime Topology, Environment & Docker/musl Delivery

- **Environment Resolution**: Debug builds read `resources/env/dev/server.env` (default port `80`, logs in `./data/dev/logs`); release builds read `resources/env/release/server.env` (default port `65002`, logs in `./data/release/logs`).
- **Database Auto-Creation**: On startup, `DbBootstrap::init().await` connects to configured MySQL, PostgreSQL, and Redis instances and executes `DatabasePlugin::initialize_auto_creation().await` with topological dependency ordering for schemas.
- **Two-Stage Static musl Image (`resources/docker/release/server.dockerfile`)**:
  - Builder stage (`rust:1.99-alpine`) compiles a statically linked `x86_64-unknown-linux-musl` binary and verifies it with `file`.
  - Runtime stage (`alpine:3.22` + `ca-certificates`) ships only the static binary, `resources/`, and empty runtime log/pid directories—no Rust toolchain or `target/` artifacts.
- **Low-Memory CI Guarantee (`.github/workflows/rust.yml`)**:
  - Release builds disable LTO (`lto = "off"`) and are validated inside a strict `--cpus=2 --memory=4g --memory-swap=4g` Alpine container before uploading static release binaries and publishing crates.
- **One-Command Compose Stack (`compose.yaml`)**:
  - Root `compose.yaml` mirrors `resources/docker/release/server_docker_compose.yml` with paths rebased to repository root, orchestrating healthy MySQL, PostgreSQL, Redis, and the server on port `65002`.

---

## 4. Verification Gates & Tier 3 Ecosystem Release Workflow

### Local Verification Gates (Mandatory Before Commit / Push)

```sh
crate fmt
crate fmt --check
cargo fmt --all -- --check
cargo clippy --workspace --all-features --all-targets -- -D warnings
cargo test --workspace --all-features
cargo check --release --workspace --all-features
python3 ~/.agents/skills/rust-standards/scripts/staged_file_gate.py .
python3 ~/.agents/skills/rust-standards/scripts/verify_dep_order.py .
```

When modifying `compose.yaml`, `resources/docker/**`, or `resources/env/**`, also validate `docker compose config -q`.

### Tier 3 Release & Dependency Upgrade Workflow

`hyperlane-quick-start` is **Tier 3** in the ecosystem dependency graph (downstream of `ctares` Tier 1 and `hyperlane` Tier 2):

1. Confirm upstream `ctares` and `hyperlane` versions are live on `crates.io`.
2. Update `hyperlane`, `color-output`, `lombok-macros`, and `instrument-level` in root `Cargo.toml` `[workspace.dependencies]` while preserving `verify_dep_order.py` ordering.
3. Bump `[package].version` and internal workspace version pins across root `Cargo.toml` and all 5 member `Cargo.toml` manifests (`.github/workflows/rust.yml` also runs `sync_workspace_version`).
4. Run all local gates, validate commit text via `verify_english_only.py`, push to `master`, and verify `.github/workflows/rust.yml` (`setup`, `sync_workspace_version`, `check`, `tests`, `clippy`, `build`, `publish` in strict order `hyperlane_resources` -> `hyperlane_config` -> `hyperlane_plugin` -> `hyperlane_application` -> `hyperlane_bootstrap` -> `hyperlane-quick-start`, and `release`) plus `docker-push.yml` finish green.

---

## 5. Unified Documentation & Skill Maintenance Policy

| Change Trigger                                                    | Target Document(s)                                                                                         | Mandatory Update Procedure                                                                                                                                                                                                    |
| :---------------------------------------------------------------- | :--------------------------------------------------------------------------------------------------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Add, rename, or reorganize a workspace layer or plugin adapter    | Root `README.md`, `<member>/README.md`, root `AGENTS.md`, `.agents/skills/hyperlane-quick-start/SKILL.md`  | Update the 5-member workspace matrix, Mermaid architecture diagram, and module layout in the same commit.                                                                                                                     |
| Change startup hooks, middleware pipeline, or `ServerHook` usage  | Root `README.md`, `application/README.md`, `bootstrap/README.md`, root `AGENTS.md`                         | Keep documented boot order and middleware/exception hooks aligned with `src/main.rs` and `bootstrap/`.                                                                                                                        |
| Change env keys, Dockerfiles, `compose.yaml`, or CI release build | Root `README.md`, `resources/README.md`, root `AGENTS.md`, `.agents/skills/hyperlane-quick-start/SKILL.md` | Synchronize env documentation, container image tags, and `docker compose` verification commands.                                                                                                                              |
| Edit any `README.md` badges or links                              | Root `README.md` and all `<member>/README.md`                                                              | Use `flat.badgen.net/crates/v/<crate>` (never `img.shields.io/crates/*`), static `license-MIT-blue` badges, exact-case `LICENSE` links, and never place a `docs.rs` badge on the binary root crate (`hyperlane-quick-start`). |

- Run `crate fmt --check` after any Markdown, YAML, TOML, or Rust edit.
- Keep `.agents/skills/hyperlane-quick-start/SKILL.md` identical to `~/.agents/skills/hyperlane-quick-start/SKILL.md` (standards, requirements, and workflows only; `<= 160` lines; validated via `python3 ~/.agents/scripts/audit_skills.py` and `python3 ~/.agents/scripts/update_skill_lock.py`).
