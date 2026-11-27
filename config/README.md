<div align="center">

# hyperlane_config

**Static Application & Framework Configuration Layer**

[![crates.io](https://flat.badgen.net/crates/v/hyperlane_config)](https://crates.io/crates/hyperlane_config)
[![docs.rs](https://img.shields.io/docsrs/hyperlane_config?style=flat-square)](https://docs.rs/hyperlane_config)
[![Rust 2024](https://img.shields.io/badge/edition-2024-orange.svg?style=flat-square)](https://www.rust-lang.org)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg?style=flat-square)](./LICENSE)

[Workspace Overview](../README.md) · [API Docs](https://docs.rs/hyperlane_config/latest/) · [Agent Engineering Rules](../AGENTS.md)

</div>

---

## Overview

Static configuration workspace crate for [`hyperlane-quick-start`](../README.md), holding compile-time application and framework constants without coupling to database pools or HTTP route handlers.

## Responsibilities & Module Layout

- `application/logger` / `application/logo_img` — Application logger defaults and static branding asset constants
- `framework` — Framework-level server and runtime configuration constants

## Installation

```sh
cargo add hyperlane_config
```

## Verification

Run crate-scoped lint and test gates from the workspace root:

```sh
cargo clippy -p hyperlane_config --all-features --all-targets -- -D warnings
cargo test -p hyperlane_config --all-features
```

## Documentation & Governance

- Update this `README.md` alongside any static configuration module change per [`../AGENTS.md`](../AGENTS.md).
- Project standards and release workflows live in [`../.agents/skills/hyperlane-quick-start/SKILL.md`](../.agents/skills/hyperlane-quick-start/SKILL.md).

## License

This project is licensed under the MIT License. See the [LICENSE](./LICENSE) file for details.

## Contact

For inquiries, please contact the author at [root@devhub.vip](mailto:root@devhub.vip).
