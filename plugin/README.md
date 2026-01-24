<center>

## hyperlane-plugin

<img src="https://docs.ltpp.vip/img/hyperlane.png" alt="" height="160">

[![](https://img.shields.io/crates/v/hyperlane.svg)](https://crates.io/crates/hyperlane)
[![](https://img.shields.io/crates/d/hyperlane.svg)](https://img.shields.io/crates/d/hyperlane.svg)
[![](https://docs.rs/hyperlane/badge.svg)](https://docs.rs/hyperlane)
[![](https://github.com/hyperlane-dev/hyperlane/workflows/Rust/badge.svg)](https://github.com/hyperlane-dev/hyperlane/actions?query=workflow:Rust)
[![](https://img.shields.io/crates/l/hyperlane.svg)](./license)

</center>

> A powerful and extensible plugin system for the hyperlane framework, providing modularity and customization capabilities.

## Overview

The `hyperlane-plugin` module provides a comprehensive plugin architecture that allows developers to extend the framework's functionality without modifying core code. Plugins can register routes, middleware, services, and event handlers.

## Directory Structure

```
plugin/
├── logger/              # Logger plugin implementation
│   ├── impl.rs
│   ├── mod.rs
│   └── static.rs
├── process/             # Process plugin implementation
│   ├── fn.rs
│   └── mod.rs
├── lib.rs
└── Cargo.toml
```

## Features

- **Plugin Registration**: Simple trait-based plugin registration
- **Lifecycle Hooks**: Initialize, start, stop, and shutdown hooks
- **Dependency Management**: Automatic dependency resolution between plugins
- **Dynamic Loading**: Runtime plugin loading and unloading
- **Middleware Integration**: Plugins can register middleware
- **Service Binding**: Plugins can expose services to other components

## Plugin Lifecycle

```
┌─────────────┐
│   Register  │  ← Plugin::register()
└──────┬──────┘
       ▼
┌─────────────┐
│  Initialize │  ← Plugin::initialize()
└──────┬──────┘
       ▼
┌─────────────┐
│    Start    │  ← Plugin::start()
└──────┬──────┘
       ▼
┌─────────────┐
│    Stop     │  ← Plugin::stop() (on shutdown)
└──────┬──────┘
       ▼
┌─────────────┐
│  Shutdown   │  ← Plugin::shutdown()
└─────────────┘
```

## Quick Start

```rust
use hyperlane_plugin::{Plugin, PluginContext};

#[derive(Default)]
struct MyPlugin;

impl Plugin for MyPlugin {
    fn name(&self) -> &'static str {
        "my-plugin"
    }

    fn initialize(&mut self, ctx: &PluginContext) -> Result<(), Box<dyn std::error::Error>> {
        // Plugin initialization logic
        Ok(())
    }
}

// Register the plugin
let plugin = MyPlugin::default();
PluginManager::register(plugin);
```

## Built-in Plugins

### Logger Plugin

Provides structured logging capabilities with configurable log levels and outputs.

### Process Plugin

Handles process management including graceful shutdown and restart functionality.

## Dependencies

```
hyperlane-plugin ├── serde          ← Workspace dependency
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
