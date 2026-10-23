# hyperlane_frontend_templates

The landing page of the Hyperlane Quick Start server.

The page is a WebAssembly application of the [euv](https://crates.io/crates/euv)
framework: the server redirects the `/templates` route to it and serves the
built page from the static resource directory. It greets the visitor, shows
the moment the page was opened and offers the links into the server.

## Building

The build script of the server package builds this package with the
[euv](https://crates.io/crates/euv-cli) command line interface and writes the
artifacts into `resources/static/templates/`. See the [frontend
README](../../README.md) for the toolchain the build needs.
