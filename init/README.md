<center>

## hyperlane-init

<img src="https://docs.ltpp.vip/img/hyperlane.png" alt="" height="160">

[![](https://img.shields.io/crates/v/hyperlane.svg)](https://crates.io/crates/hyperlane)
[![](https://img.shields.io/crates/d/hyperlane.svg)](https://img.shields.io/crates/d/hyperlane.svg)
[![](https://docs.rs/hyperlane/badge.svg)](https://docs.rs/hyperlane)
[![](https://github.com/hyperlane-dev/hyperlane/workflows/Rust/badge.svg)](https://github.com/hyperlane-dev/hyperlane/actions?query=workflow:Rust)
[![](https://img.shields.io/crates/l/hyperlane.svg)](./license)

</center>

> Hyperlane initialization module responsible for application bootstrap, configuration loading, and graceful shutdown coordination.

## Overview

The `hyperlane-init` module orchestrates the startup sequence of the hyperlane application. It ensures all components are properly initialized in the correct order and provides graceful shutdown capabilities.

## Directory Structure

```
init/
├── application/
│   └── logger/         # Logger initialization
├── framework/
│   ├── shutdown/       # Shutdown coordination
│   └── wait/           # Wait/pause utilities
├── lib.rs
└── Cargo.toml
```

## Initialization Sequence

```
┌─────────────────────────────────────────────────────────────┐
│                    Application Startup                       │
├─────────────────────────────────────────────────────────────┤
│  1. Load Configuration                                       │
│     ├── Load config files                                    │
│     ├── Parse environment variables                          │
│     └── Apply default values                                 │
├─────────────────────────────────────────────────────────────┤
│  2. Initialize Logging System                                │
│     ├── Setup log level                                      │
│     ├── Configure log targets                                │
│     └── Initialize log subscribers                           │
├─────────────────────────────────────────────────────────────┤
│  3. Initialize Plugin System                                 │
│     ├── Load plugin registry                                 │
│     ├── Initialize plugins in dependency order               │
│     └── Start all plugins                                    │
├─────────────────────────────────────────────────────────────┤
│  4. Initialize Application Components                        │
│     ├── Initialize services                                  │
│     ├── Register routes                                      │
│     └── Setup middleware                                     │
├─────────────────────────────────────────────────────────────┤
│  5. Start Server                                             │
│     ├── Bind to address                                      │
│     ├── Accept connections                                   │
│     └── Handle requests                                      │
└─────────────────────────────────────────────────────────────┘
```

## Features

- **Sequential Initialization**: Components initialized in proper order
- **Graceful Shutdown**: Coordinated shutdown with signal handling
- **Error Propagation**: Initialization errors are properly reported
- **Resource Cleanup**: Automatic cleanup of resources on shutdown
- **Wait Groups**: Synchronization of async initialization tasks

## Quick Start

```rust
use hyperlane_init::initialize;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app = initialize().await?;
    app.run().await?;
    Ok(())
}
```

## Shutdown Flow

```
┌─────────────────────────────────────────────────────────────┐
│                    Shutdown Sequence                         │
├─────────────────────────────────────────────────────────────┤
│  1. Receive Shutdown Signal (SIGINT, SIGTERM, etc.)          │
├─────────────────────────────────────────────────────────────┤
│  2. Stop Accepting New Requests                              │
├─────────────────────────────────────────────────────────────┤
│  3. Wait for Active Requests to Complete                     │
├─────────────────────────────────────────────────────────────┤
│  4. Stop Plugins (in reverse dependency order)               │
├─────────────────────────────────────────────────────────────┤
│  5. Flush Logs                                               │
├─────────────────────────────────────────────────────────────┤
│  6. Close Connections                                        │
├─────────────────────────────────────────────────────────────┤
│  7. Release Resources                                        │
└─────────────────────────────────────────────────────────────┘
```

## Dependencies

```
hyperlane-init ├── hyperlane-app     ← Workspace dependency
├── hyperlane_config  ← Workspace dependency
├── hyperlane_plugin  ← Workspace dependency
├── serde             ← Workspace dependency
├── tracing           ← Workspace dependency
├── hyperlane         ← Workspace dependency
└── hyperlane-utils   ← Workspace dependency
```

## License

This project is licensed under the MIT License. For more details, please see the [license](license) file.

## Contributing

Contributions are welcome! Please submit an issue or create a pull request.

## Contact

If you have any questions, please contact the author: [root@ltpp.vip](mailto:root@ltpp.vip).
