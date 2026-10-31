# hyperlane_frontend_diff

The diff page of the Hyperlane Quick Start server.

The package is a WebAssembly application of the [euv](https://crates.io/crates/euv)
framework: the server serves it from the static resource directory and the
browser boots it on its own.

## Building

The build script of the server package builds this package with the
[euv](https://crates.io/crates/euv-cli) command line interface and writes the
artifacts into `resources/static/diff/`. See the [frontend
README](../../README.md) for the toolchain the build needs.

## Layout

```
src/
├── app/               the application shell of the page
├── diff/              the sources of the page
├── hook/              the state of the page and the calls it makes
├── model/             the models of the API of the page
├── page/              the view of the page
├── style/             the class registry of the page
```
