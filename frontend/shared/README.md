# hyperlane_frontend_shared

Shared frontend building blocks for the pages of the Hyperlane Quick Start server.

The package holds the parts every page of the server shares: the shell that
carries the navigation and the theme, the client that calls the API of the
server and the scope that gives the content of a page a reactive scope of
its own. It is a library of the [euv](https://crates.io/crates/euv)
framework, and every page of the workspace uses it.

## Building

The build script of the server package builds this package with the
[euv](https://crates.io/crates/euv-cli) command line interface before it
builds the pages that use it. See the [frontend
README](../README.md) for the toolchain the build needs.

## Layout

```
src/
├── api/               the client that calls the API of the server
├── component/         the shell, the form fields and the scope of a page
└── style/             the class registry the pages share
```
