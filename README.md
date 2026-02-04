<center>

## hyperlane-quick-start

<img src="https://docs.ltpp.vip/img/hyperlane.png" alt="" height="160">

[![](https://img.shields.io/crates/v/hyperlane.svg)](https://crates.io/crates/hyperlane)
[![](https://img.shields.io/crates/d/hyperlane.svg)](https://img.shields.io/crates/d/hyperlane.svg)
[![](https://docs.rs/hyperlane/badge.svg)](https://docs.rs/hyperlane)
[![](https://github.com/hyperlane-dev/hyperlane/workflows/Rust/badge.svg)](https://github.com/hyperlane-dev/hyperlane/actions?query=workflow:Rust)
[![](https://img.shields.io/crates/l/hyperlane.svg)](./license)

</center>

> A lightweight, high-performance, and cross-platform Rust HTTP server library built on Tokio. It simplifies modern web service development by providing built-in support for middleware, WebSocket, Server-Sent Events (SSE), and raw TCP communication. With a unified and ergonomic API across Windows, Linux, and MacOS, it enables developers to build robust, scalable, and event-driven network applications with minimal overhead and maximum flexibility.

## Official Documentation

- [Official Documentation](https://docs.ltpp.vip/hyperlane/)

## Api Docs

- [Api Docs](https://docs.rs/hyperlane/latest/)

## Directory Structure

```txt
├── app                      # Application service
│   ├── controller           # Interface control layer
│   ├── domain               # Business domain layer
│   ├── exception            # Exception handling layer
│      ├── application       # Application exceptions
│      ├── framework         # Framework exceptions
│   ├── mapper               # Data mapping layer
│   ├── middleware           # Middleware layer
│   ├── model                # Data model layer
│      ├── request           # Request parameter objects
│      ├── response          # Response parameter objects
│   ├── repository           # Data access layer
│   ├── service              # Business logic layer
│   ├── utils                # Utility layer
│   ├── view                 # View layer
├── config                   # Service configuration
│   ├── application          # Application configuration
│   ├── framework            # Framework configuration
├── init                     # Service initialization
│   ├── application          # Application initialization
│   ├── framework            # Framework initialization
├── plugin                   # Service plugins
│   ├── database             # Database plugin
│   ├── env                  # Environment variable plugin
│   ├── logger               # Logging plugin
│   ├── mysql                # MySQL plugin
│   ├── postgresql           # PostgreSQL plugin
│   ├── process              # Process management plugin
│   ├── redis                # Redis plugin
├── resources                # Service resources
│   ├── sql                  # SQL files
│   ├── static               # Static resource files
│   ├── templates            # Template files
```

## Run

### start

```sh
cargo run
```

### hot-restart

```sh
cargo run hot-restart
```

### started in background

```sh
cargo run -- -d
```

### stop

```sh
cargo run stop
```

### restart

```sh
cargo run restart
```

### restarted in background

```sh
cargo run restart -d
```

## Performance

- [Performance](https://docs.ltpp.vip/hyperlane/speed)

## Appreciate

> If you feel that `hyperlane` is helpful to you, feel free to donate

### WeChat Pay

<img src="https://docs.ltpp.vip/img/wechatpay.png" width="200">

### Alipay

<img src="https://docs.ltpp.vip/img/alipay.png" width="200">

### Virtual Currency Pay

| Virtual Currency | Virtual Currency Address                   |
| ---------------- | ------------------------------------------ |
| BTC              | 3QndxCJTf3mEniTgyRRQ1jcNTJajm9qSCy         |
| ETH              | 0x8EB3794f67897ED397584d3a1248a79e0B8e97A6 |
| BSC              | 0x8EB3794f67897ED397584d3a1248a79e0B8e97A6 |

## License

This project is licensed under the MIT License. For more details, please see the [license](license) file.

## Contributing

Contributions are welcome! Please submit an issue or create a pull request.

## Contact

If you have any questions, please contact the author: [root@ltpp.vip](mailto:root@ltpp.vip).
