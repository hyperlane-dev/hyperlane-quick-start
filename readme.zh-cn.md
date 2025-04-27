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

> Hyperlane 是一个轻量级且高性能的 Rust HTTP 服务器库，旨在简化网络服务开发。它支持 HTTP 请求解析、响应构建和 TCP 通信，非常适合构建现代 Web 服务。此外，它还支持请求和响应中间件、WebSocket 和 Server-Sent Events (SSE)，从而实现灵活高效的实时通信。Hyperlane 使用纯 Rust 和标准库构建，提供跨 Windows、Linux 和 macOS 的真正跨平台兼容性，且所有平台上的 API 体验一致，依托 Tokio 的异步运行时实现无缝网络通信，无需特定于平台的依赖。

## API 文档

- [API 文档](https://docs.rs/hyperlane/latest/hyperlane/)

## 官方文档

- [官方文档](https://docs.ltpp.vip/hyperlane/)

## 运行

### 运行

```sh
cargo run
```

### 在后台运行

```sh
cargo run -- -d
```

### 停止

```sh
cargo run stop
```

### 重启

```sh
cargo run restart
```

### 重启在后台运行

```sh
cargo run restart -d
```

## 性能测试

- [性能测试](https://docs.ltpp.vip/hyperlane/speed)

## 赞赏

**如果你觉得 `hyperlane` 对你有所帮助，欢迎捐赠**

<img src="https://docs.ltpp.vip/img/wechat-pay.png" width="200">  
<img src="https://docs.ltpp.vip/img/alipay-pay.jpg" width="200">

## 许可证

此项目基于 MIT 许可证授权。详细信息请查看 [license](license) 文件。

## 贡献

欢迎贡献！请提交 issue 或创建 pull request。

## 联系方式

如有任何疑问，请联系作者：[ltpp-universe <root@ltpp.vip>](mailto:root@ltpp.vip)。
