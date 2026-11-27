<div align="center">

# hyperlane_resources

**Environment Templates, Docker Manifests, SQL Scripts & Static Resource Paths**

[![crates.io](https://flat.badgen.net/crates/v/hyperlane_resources)](https://crates.io/crates/hyperlane_resources)
[![docs.rs](https://img.shields.io/docsrs/hyperlane_resources?style=flat-square)](https://docs.rs/hyperlane_resources)
[![Rust 2024](https://img.shields.io/badge/edition-2024-orange.svg?style=flat-square)](https://www.rust-lang.org)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg?style=flat-square)](./LICENSE)

[Workspace Overview](../README.md) · [API Docs](https://docs.rs/hyperlane_resources/latest/) · [Agent Engineering Rules](../AGENTS.md)

</div>

---

## Overview

Resource-layer workspace crate for [`hyperlane-quick-start`](../README.md), organizing development and release `.env` templates, multi-stage Alpine/musl Dockerfiles, Docker Compose topologies, SQL bootstrap scripts, and template resource paths.

## Responsibilities & Module Layout

- `env/{dev,release,path}` — Development and release `server.env` templates and canonical path constants
- `docker/{dev,release,path}` — Two-stage static musl `server.dockerfile` definitions and Compose stacks
- `sql` / `templates` — Database SQL assets and template resources

## Installation

```sh
cargo add hyperlane_resources
```

## Verification

Run crate-scoped lint and test gates from the workspace root:

```sh
cargo clippy -p hyperlane_resources --all-features --all-targets -- -D warnings
cargo test -p hyperlane_resources --all-features
```

## Documentation & Governance

- Update this `README.md` alongside any environment file, Dockerfile, or Compose layout change per [`../AGENTS.md`](../AGENTS.md).
- Project standards and release workflows live in [`../.agents/skills/hyperlane-quick-start/SKILL.md`](../.agents/skills/hyperlane-quick-start/SKILL.md).

## License

This project is licensed under the MIT License. See the [LICENSE](./LICENSE) file for details.

## Contact

For inquiries, please contact the author at [root@devhub.vip](mailto:root@devhub.vip).
