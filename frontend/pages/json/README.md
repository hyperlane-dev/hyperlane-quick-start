# hyperlane_frontend_json

The JSON editor page of the Hyperlane Quick Start server.

The page is a WebAssembly application of the [euv](https://crates.io/crates/euv)
framework: the server serves it from the static resource directory and the
browser boots it on its own. It parses a document, browses it as a tree,
edits one field at a time and copies the formatted result to the clipboard.
The page works on its own, it never calls the API of the server.

## Building

The build script of the server package builds this package with the
[euv](https://crates.io/crates/euv-cli) command line interface and writes the
artifacts into `resources/static/json/`. See the [frontend
README](../../README.md) for the toolchain the build needs.

## Layout

```
src/
├── app/       the application shell of the page
├── hook/      the state of the page and the tree it builds
├── model/     the models of the tree of the document
├── page/      the view of the page
└── style/     the class registry of the page
```
