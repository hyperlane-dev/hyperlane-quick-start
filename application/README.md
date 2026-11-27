<div align="center">

# hyperlane_application

**Application Layer for Controllers, Services, Middleware, Views & Exception Hooks**

[![crates.io](https://flat.badgen.net/crates/v/hyperlane_application)](https://crates.io/crates/hyperlane_application)
[![docs.rs](https://img.shields.io/docsrs/hyperlane_application?style=flat-square)](https://docs.rs/hyperlane_application)
[![Rust 2024](https://img.shields.io/badge/edition-2024-orange.svg?style=flat-square)](https://www.rust-lang.org)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg?style=flat-square)](./LICENSE)

[Workspace Overview](../README.md) · [API Docs](https://docs.rs/hyperlane_application/latest/) · [Agent Engineering Rules](../AGENTS.md)

</div>

---

## Overview

Application-layer workspace crate for [`hyperlane-quick-start`](../README.md), encapsulating HTTP controllers, business services, persistence repositories/mappers, request/response models, middlewares, views, and panic/error hooks wired into Hyperlane's `ServerHook` lifecycle.

## Responsibilities & Module Layout

- `controller` / `service` / `repository` / `mapper` / `domain` — Layered domain and HTTP endpoint modules
- `middleware/request` / `middleware/response` — `HttpRequestMiddleware`, `CrossMiddleware`, `ResponseHeaderMiddleware`, `ResponseStatusCodeMiddleware`, `SendMiddleware`, and `LogMiddleware`
- `exception` — `TaskPanicHook` and `RequestErrorHook` structured JSON error responses
- `model` / `view` / `utils` — Common `ApiResponse`, `FaviconRoute`, JSON serialization, and response-sending helpers

## Installation

```sh
cargo add hyperlane_application
```

## Verification

Run crate-scoped lint and test gates from the workspace root:

```sh
cargo clippy -p hyperlane_application --all-features --all-targets -- -D warnings
cargo test -p hyperlane_application --all-features
```

## Documentation & Governance

- Update this `README.md` alongside any middleware, hook, or module-boundary change per [`../AGENTS.md`](../AGENTS.md).
- Project standards and release workflows live in [`../.agents/skills/hyperlane-quick-start/SKILL.md`](../.agents/skills/hyperlane-quick-start/SKILL.md).

## License

This project is licensed under the MIT License. See the [LICENSE](./LICENSE) file for details.

## Contact

For inquiries, please contact the author at [root@devhub.vip](mailto:root@devhub.vip).
