<center>

## hyperlane-config

<img src="https://docs.ltpp.vip/img/hyperlane.png" alt="" height="160">

[![](https://img.shields.io/crates/v/hyperlane.svg)](https://crates.io/crates/hyperlane)
[![](https://img.shields.io/crates/d/hyperlane.svg)](https://img.shields.io/crates/d/hyperlane.svg)
[![](https://docs.rs/hyperlane/badge.svg)](https://docs.rs/hyperlane)
[![](https://github.com/hyperlane-dev/hyperlane/workflows/Rust/badge.svg)](https://github.com/hyperlane-dev/hyperlane/actions?query=workflow:Rust)
[![](https://img.shields.io/crates/l/hyperlane.svg)](./license)

</center>

> Hyperlane configuration module providing comprehensive configuration management capabilities for the framework.

## Overview

The `hyperlane-config` module is responsible for all configuration-related operations in the hyperlane framework. It provides a flexible, type-safe configuration system that supports multiple formats and environments.

## Directory Structure

```
config/
├── application/          # Application configuration
│   ├── hello/           # Hello route configuration
│   └── logger/          # Logger configuration
├── framework/           # Framework configuration
└── Cargo.toml
```

## Features

- **Multi-format Support**: Supports JSON, YAML, TOML configuration files
- **Environment Variables**: Built-in environment variable override support
- **Type Safety**: Strongly typed configuration with serde
- **Hot Reload**: Automatic configuration reload on file changes
- **Validation**: Configuration validation with descriptive error messages
- **Layered Configuration**: Merge multiple configuration sources

## Quick Start

```rust
use hyperlane_config::Config;

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub host: String,
    pub port: u16,
    pub workers: usize,
}

let config = AppConfig::load("config.json")?;
println!("Server listening on {}:{}", config.host, config.port);
```

## Configuration Sources

Configurations are loaded in the following priority order (highest to lowest):

1. Command line arguments
2. Environment variables
3. `config.{env}.{ext}` (e.g., `config.production.json`)
4. `config.{ext}` (e.g., `config.json`)
5. Default values

## Dependencies

```
hyperlane-config ├── serde          ← Workspace dependency
├── tracing       ← Workspace dependency
├── hyperlane     ← Workspace dependency
└── hyperlane-utils ← Workspace dependency
```

## License

This project is licensed under the MIT License. For more details, please see the [license](license) file.

## Contributing

Contributions are welcome! Please submit an issue or create a pull request.

## Contact

If you have any questions, please contact the author: [root@ltpp.vip](mailto:root@ltpp.vip).
