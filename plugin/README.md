<div align="center">

# hyperlane_plugin

**Infrastructure Adapters for Databases, Environment, Logging, Process & Shutdown**

[![crates.io](https://flat.badgen.net/crates/v/hyperlane_plugin)](https://crates.io/crates/hyperlane_plugin)
[![docs.rs](https://img.shields.io/docsrs/hyperlane_plugin?style=flat-square)](https://docs.rs/hyperlane_plugin)
[![Rust 2024](https://img.shields.io/badge/edition-2024-orange.svg?style=flat-square)](https://www.rust-lang.org)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg?style=flat-square)](./LICENSE)

[Workspace Overview](../README.md) · [API Docs](https://docs.rs/hyperlane_plugin/latest/) · [Agent Engineering Rules](../AGENTS.md)

</div>

---

## Overview

Infrastructure-plugin workspace crate for [`hyperlane-quick-start`](../README.md), providing `OnceLock`-backed runtime adapters for environment parsing, structured file/console logging, MySQL/PostgreSQL/Redis connectivity, topological database schema auto-creation, PID process management, and graceful shutdown hooks.

## Responsibilities & Module Layout

- `common` — `GetOrInit` plugin initialization trait
- `database` / `mysql` / `postgresql` / `redis` — Pooled SeaORM and Redis connectors plus topological `DatabaseSchema` auto-creation
- `env` / `logger` — `.env` loader (`EnvPlugin`, `EnvConfig`) and size-rotating file/console logger (`LoggerPlugin`)
- `process` / `shutdown` — PID file lifecycle (`ProcessPlugin`) and shutdown hook storage (`ShutdownPlugin`)

## Installation

```sh
cargo add hyperlane_plugin
```

## Verification

Run crate-scoped lint and test gates from the workspace root:

```sh
cargo clippy -p hyperlane_plugin --all-features --all-targets -- -D warnings
cargo test -p hyperlane_plugin --all-features
```

## Documentation & Governance

- Update this `README.md` alongside any infrastructure adapter or `GetOrInit` contract change per [`../AGENTS.md`](../AGENTS.md).
- Project standards and release workflows live in [`../.agents/skills/hyperlane-quick-start/SKILL.md`](../.agents/skills/hyperlane-quick-start/SKILL.md).

## License

This project is licensed under the MIT License. See the [LICENSE](./LICENSE) file for details.

## Contact

For inquiries, please contact the author at [root@devhub.vip](mailto:root@devhub.vip).
