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
The repository leaves build parallelism to Cargo's defaults. Set it explicitly
with `cargo build --release -j <N>` when needed. The release profile disables
LTO to reduce build-time memory pressure. CI compiles the tree inside a
container capped at 2 CPUs and 4 GiB with no swap and prints the measured peak
in the job log. To skip the build entirely, pull the published image instead:

| Branch       | Image                                                                           |
| ------------ | ------------------------------------------------------------------------------- |
| `master`     | `hkccr.ccs.tencentyun.com/hyperlane-dev/hyperlane-quick-start:1.0.0`            |
| `playground` | `hkccr.ccs.tencentyun.com/hyperlane-dev/hyperlane-quick-start-playground:1.0.0` |

```sh
docker compose pull;
docker compose up -d;
```

The image is a two-stage build: the builder stage compiles a statically linked
binary and the runtime stage keeps only that executable plus the static assets
the server serves. The Rust toolchain, the crate registry and the whole build
directory are discarded with the builder stage, so the published image carries
no compiler and no intermediate objects.

## Run the binary directly

Every release also ships the static binary as a release asset, so the server
can run without a container and without a libc:

```sh
curl -L -o hyperlane-quick-start \
  https://github.com/hyperlane-dev/hyperlane-quick-start/releases/latest/download/hyperlane-quick-start-<version>-linux-x86_64
chmod +x hyperlane-quick-start
./hyperlane-quick-start
```

The binary is built for `x86_64-unknown-linux-musl` and is statically linked,
so it runs on any Linux distribution and needs no shared library. It does need
the repository next to it, because the server reads `./resources/env` and
serves `./resources/static`; a clone of this repository is the simplest way to
provide them.

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

If you have any questions, please contact the author: [root@devhub.vip](mailto:root@devhub.vip).
