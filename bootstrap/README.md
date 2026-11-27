<div align="center">

# hyperlane_bootstrap

**Composition Root & Lifecycle Bootstrap Layer**

[![crates.io](https://flat.badgen.net/crates/v/hyperlane_bootstrap)](https://crates.io/crates/hyperlane_bootstrap)
[![docs.rs](https://img.shields.io/docsrs/hyperlane_bootstrap?style=flat-square)](https://docs.rs/hyperlane_bootstrap)
[![Rust 2024](https://img.shields.io/badge/edition-2024-orange.svg?style=flat-square)](https://www.rust-lang.org)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg?style=flat-square)](./LICENSE)

[Workspace Overview](../README.md) · [API Docs](https://docs.rs/hyperlane_bootstrap/latest/) · [Agent Engineering Rules](../AGENTS.md)

</div>

---

## Overview

Composition-root workspace crate for [`hyperlane-quick-start`](../README.md), orchestrating environment loading, logger setup, Tokio runtime construction, MySQL/PostgreSQL/Redis initialization, and Hyperlane server startup with graceful shutdown wiring.

## Responsibilities & Module Layout

- `common` — `BootstrapInit` and `BootstrapAsyncInit` lifecycle traits
- `application/{env,logger,db}` — `EnvBootstrap`, `LoggerBootstrap`, and `DbBootstrap` startup adapters
- `framework/{config,runtime,server}` — `ConfigBootstrap`, `RuntimeBootstrap`, and `ServerBootstrap` wiring `hyperlane_application` hooks via `#[hyperlane(server: Server)]` and `#[import(hyperlane_application)]`

## Installation

```sh
cargo add hyperlane_bootstrap
```

## Verification

Run crate-scoped lint and test gates from the workspace root:

```sh
cargo clippy -p hyperlane_bootstrap --all-features --all-targets -- -D warnings
cargo test -p hyperlane_bootstrap --all-features
```

## Documentation & Governance

- Update this `README.md` alongside any boot-sequence or server-wiring change per [`../AGENTS.md`](../AGENTS.md).
- Project standards and release workflows live in [`../.agents/skills/hyperlane-quick-start/SKILL.md`](../.agents/skills/hyperlane-quick-start/SKILL.md).

## License

This project is licensed under the MIT License. See the [LICENSE](./LICENSE) file for details.

## Contact

For inquiries, please contact the author at [root@devhub.vip](mailto:root@devhub.vip).
