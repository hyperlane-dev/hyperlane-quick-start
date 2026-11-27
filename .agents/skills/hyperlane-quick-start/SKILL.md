---
name: hyperlane-quick-start
description: 'Use when maintaining hyperlane-quick-start: layered workspace boundaries (application, bootstrap, config, plugin, resources), Docker/musl delivery, dependency upgrades, and doc governance.'
license: MIT
metadata:
  hermes:
    related_skills: [rust-standards, hyperlane, git-standards]
---

# hyperlane-quick-start standards and workflow

## Mandatory Standards

1. Load `rust-standards` and `hyperlane` before modifying any Rust or Cargo file.
2. Enforce strict 5-member layered workspace boundaries:
  - `config/` (`hyperlane_config`): static application and framework configuration types only; zero database or HTTP business logic.
  - `resources/` (`hyperlane_resources`): environment templates (`env/`), SQL migrations (`sql/`), Dockerfiles (`docker/`), and static/template assets; never commit production secrets.
  - `plugin/` (`hyperlane_plugin`): infrastructure adapters (`mysql`, `postgresql`, `redis`, `database`, `env`, `logger`, `process`, `shutdown`); no HTTP route controllers.
  - `application/` (`hyperlane_application`): business layers (`controller`, `service`, `repository`, `mapper`, `domain`, `model`, `middleware`, `exception`, `utils`, `view`).
  - `bootstrap/` (`hyperlane_bootstrap`): composition root wiring config, plugins, routes, and server lifecycle.

3. Preserve all `hyperlane` wire and `ServerHook` contracts: synchronous route/middleware registration, `ctx.build_response()` for HEAD-aware output, explicit error propagation from `Server::run().await`.
4. Never commit generated runtime artifacts (`data/`, `*.log`, `*.pid`, untracked `resources/static/` outputs).
5. Keep all code, comments, commit messages, docs, and skills strictly English-only.

## Verification Workflow

Run every gate locally before committing or pushing:

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

When changing Dockerfiles, `compose.yaml`, or `resources/env/`, also validate `docker compose config` and verify runtime environment fallback behavior.

## Dependency & Release Workflow

`hyperlane-quick-start` is **Tier 3** (depends on published `ctares` and `hyperlane`):

1. Wait until upstream `ctares` and `hyperlane` versions are live on `crates.io`.
2. Upgrade `hyperlane`, `color-output`, `lombok-macros`, and `instrument-level` in root `Cargo.toml` `[workspace.dependencies]` while preserving `verify_dep_order.py` order.
3. Bump `[package].version` and internal `[workspace.dependencies]` pins in root `Cargo.toml` (and member `Cargo.toml` files if not using `version.workspace = true`).
4. Run all local verification gates, validate commit text via `verify_english_only.py`, push to `master`, and wait until `.github/workflows/rust.yml` (check, tests, musl binary build, release) completes green.

## Documentation & Skill Governance

- Update root `README.md`, `<member>/README.md`, and `AGENTS.md` in the same commit whenever changing workspace layers, runtime env keys, Docker/musl startup, or CI gates.
- Keep `.agents/skills/hyperlane-quick-start/SKILL.md` and `~/.agents/skills/hyperlane-quick-start/SKILL.md` synchronized and strictly limited to standards, requirements, and workflows (`<= 160` lines).
- After updating any skill, run `python3 ~/.agents/scripts/audit_skills.py` and `python3 ~/.agents/scripts/update_skill_lock.py`.
