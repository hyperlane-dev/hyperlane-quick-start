<center>

## hyperlane-quick-start

<img src="https://ltpp.vip/github/pages/docs-pages/pages/img/hyperlane.png" alt="" height="160">

[![](https://img.shields.io/crates/v/hyperlane.svg)](https://crates.io/crates/hyperlane)
[![](https://img.shields.io/crates/d/hyperlane.svg)](https://img.shields.io/crates/d/hyperlane.svg)
[![](https://docs.rs/hyperlane/badge.svg)](https://docs.rs/hyperlane)
[![](https://github.com/hyperlane-dev/hyperlane/workflows/Rust/badge.svg)](https://github.com/hyperlane-dev/hyperlane/actions?query=workflow:Rust)
[![](https://img.shields.io/crates/l/hyperlane.svg)](./license)

</center>

> A lightweight, high-performance, and cross-platform Rust HTTP server library built on Tokio. It simplifies modern web service development by providing built-in support for middleware, WebSocket, Server-Sent Events (SSE), and raw TCP communication. With a unified and ergonomic API across Windows, Linux, and MacOS, it enables developers to build robust, scalable, and event-driven network applications with minimal overhead and maximum flexibility.

## Run it

```sh
git clone https://github.com/hyperlane-dev/hyperlane-quick-start.git && cd hyperlane-quick-start
docker compose up -d
```

Then open <http://127.0.0.1:65002>.

That is the whole procedure. `compose.yaml` in the repository root builds the
server image from source, starts MySQL, PostgreSQL and Redis, waits for all three
to report healthy, and starts the server. Nothing has to be configured first — the
release environment file is read automatically in a release build and already
names the three containers.

The first run compiles from source and can take a long time on a small machine.
The repository builds serially by default (`.cargo/config.toml` sets
`build.jobs = 1`) and the release profile has no LTO, so it stays within reach of
a 2-core / 4 GiB box instead of exhausting it. On a stronger machine, raise
parallelism explicitly with `cargo build --release -j <N>`. To skip the build
entirely, pull the published image instead:

| Branch       | Image                                                                           |
| ------------ | ------------------------------------------------------------------------------- |
| `master`     | `hkccr.ccs.tencentyun.com/hyperlane-dev/hyperlane-quick-start:1.0.0`            |
| `playground` | `hkccr.ccs.tencentyun.com/hyperlane-dev/hyperlane-quick-start-playground:1.0.0` |

```sh
docker compose pull;
docker compose up -d;
```

The server logs its own progress; this is what a healthy start looks like:

```
[AUTO-CREATION] Initialization complete MySQL (mysql_default) verified,
PostgreSQL (postgres_default) verified, Redis (redis_default) verified
Server listen in 0.0.0.0:65002
```

```sh
docker compose ps
docker compose logs -f server
docker compose down            # stop, keep the data
docker compose down -v         # stop and discard the data
```

The three databases are reachable by their compose service names and are not
published to the host, so the stack does not collide with anything else already
listening. To inspect one:

```sh
docker compose exec postgresql psql -U hyperlane -d hyperlane;
docker compose exec mysql mysql -uhyperlane -phyperlane hyperlane;
docker compose exec redis redis-cli -a hyperlane ping;
```

Add `-T` and a command when running these from a script, or they wait on stdin
forever.

## Api Docs

- [Api Docs](https://docs.rs/hyperlane/latest/)

## License

This project is licensed under the MIT License. For more details, please see the [license](license) file.

## Contributing

Contributions are welcome! Please submit an issue or create a pull request.

## Contact

If you have any questions, please contact the author: [root@ltpp.vip](mailto:root@ltpp.vip).
