# Frontend

The browser assets of the server are built from this workspace. Every page of
the server is a package of the workspace, and every package is a WebAssembly
application of the [euv](https://crates.io/crates/euv) framework that renders
its page with the components of the
[euv-ui](https://crates.io/crates/euv-ui) library.

## Layout

```
frontend/
├── shared/          the building blocks every page shares
└── pages/<page>/    one package per page
```

`shared` carries the application shell (the navigation, the theme toggle and
the drawer of the narrow layout), the JSON client the pages call the API
with, and the class registry the shell's own styles live in. A page package
carries the models of the API it talks to, the state of the page and the
view that renders it.

## Building

The server builds the pages for you: the build script of the server package
walks `pages/`, builds every package with the
[euv](https://crates.io/crates/euv-cli) command line interface and writes the
artifacts into `resources/static/<page>/`, which is the directory the server
serves the browser assets from.

```bash
cargo build --release
```

The build needs the `wasm32-unknown-unknown` target, `wasm-pack` and the
`euv` command line interface:

```bash
rustup target add wasm32-unknown-unknown
cargo install wasm-pack
cargo install euv-cli --version 0.31.3
```

A package is only rebuilt when its sources are newer than its artifacts, so
a rebuild of the server alone does not rebuild the browser assets. The
`EUV_FRONTEND_SKIP` environment variable turns the frontend build off
entirely, which is what the pipeline jobs that do not ship the browser
assets do.

## Working on a page

The `euv` command line interface serves a package with a live reload server:

```bash
cd frontend/pages/blog
euv run --crate-path .
```

The framework formats the macro bodies of the sources before it builds them,
so `euv fmt --check` is the formatting gate of this workspace:

```bash
cd frontend/pages/blog
euv fmt --check
```

## Conventions

- Every component comes from `euv-ui`. A page that needs a style of its own
  adds a class to the class registry of its own package, never a hand rolled
  element.
- A class is written with the design tokens of the framework (`var!(…)`),
  never with a hard coded colour or spacing.
- The pages hold their state in signals, and every piece of state that a
  user can change lives in a hook of the package.
