# hyperlane_frontend_auth

The authentication page of the Hyperlane Quick Start server.

The page is a WebAssembly application of the [euv](https://crates.io/crates/euv)
framework: the server serves it from the static resource directory and the
browser boots it on its own. It logs a visitor in and registers a new
account, the credentials are encrypted with the public key of the server.

## Building

The build script of the server package builds this package with the
[euv](https://crates.io/crates/euv-cli) command line interface and writes the
artifacts into `resources/static/auth/`. See the [frontend
README](../../README.md) for the toolchain the build needs.

## Layout

```
src/
├── app/       the application shell of the page
├── crypto/    the encryption of the credentials of the visitor
├── hook/      the state of the page and the calls it makes
├── model/     the models of the auth API
├── page/      the view of the page
└── style/     the class registry of the page
```
