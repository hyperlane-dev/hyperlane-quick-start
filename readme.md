<center>

## hyperlane-quick-start

[English](readme.md) | [简体中文](readme.zh-cn.md)

<img src="./static/img/logo.png" alt="" height="160">

[![](https://img.shields.io/crates/v/hyperlane.svg)](https://crates.io/crates/hyperlane)
[![](https://img.shields.io/crates/d/hyperlane.svg)](https://img.shields.io/crates/d/hyperlane.svg)
[![](https://docs.rs/hyperlane/badge.svg)](https://docs.rs/hyperlane)
[![](https://github.com/ltpp-universe/hyperlane/workflows/Rust/badge.svg)](https://github.com/ltpp-universe/hyperlane/actions?query=workflow:Rust)
[![](https://img.shields.io/crates/l/hyperlane.svg)](./license)

</center>

> Hyperlane is a lightweight and high-performance Rust HTTP server library designed to simplify network service development. It supports HTTP request parsing, response building, and TCP communication, making it ideal for building modern web services. Additionally, it provides support for request and response middleware, WebSocket, and Server-Sent Events (SSE), enabling flexible and efficient real-time communication. Built with pure Rust and standard library, Hyperlane offers true cross-platform compatibility across Windows, Linux and macOS, with the same API experience on all platforms, powered by Tokio's async runtime for seamless networking without platform-specific dependencies.

## Api Docs

- [Api Docs](https://docs.rs/hyperlane/latest/hyperlane/)

## Official Documentation

- [Official Documentation](https://docs.ltpp.vip/hyperlane/)

## Run

### start

```sh
cargo run
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

**If you feel that `hyperlane` is helpful to you, feel free to donate.**

<img src="https://docs.ltpp.vip/img/wechat-pay.png" width="200">
<img src="https://docs.ltpp.vip/img/alipay-pay.jpg" width="200">

## LICENSE

This project is licensed under the MIT LICENSE. See the [LICENSE](LICENSE) file for details.

## Contributing

Contributions are welcome! Please open an issue or submit a pull request.

## Contact

For any inquiries, please reach out to the author at [ltpp-universe <root@ltpp.vip>](mailto:root@ltpp.vip).
