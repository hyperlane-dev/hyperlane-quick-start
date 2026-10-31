# hyperlane_frontend_order

The order page of the Hyperlane Quick Start server.

The page is a WebAssembly application of the [euv](https://crates.io/crates/euv)
framework: the server serves it from the static resource directory and the
browser boots it on its own. It shows the statistics of the ledger, lists and
writes the records of it, lists the accounts of the server and holds the
profile of the visitor.

## Building

The build script of the server package builds this package with the
[euv](https://crates.io/crates/euv-cli) command line interface and writes the
artifacts into `resources/static/order/`. See the [frontend
README](../../README.md) for the toolchain the build needs.

## Layout

```
src/
├── app/       the application shell of the page
├── crypto/    the encryption of the fields of the profile
├── hook/      the state of the page and the calls it makes
├── model/     the models of the order API
├── page/      the view of the page
└── style/     the class registry of the page
```
