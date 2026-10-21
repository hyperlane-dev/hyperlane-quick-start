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

The build uses the command line interface of the version the project pins, and
it stops when another version answers, because the artifacts of another version
are not the ones the pipeline and the images of the server produce. A machine
that keeps the tool outside its path points the build at the binary of the
pinned version:

```bash
EUV_CLI=/path/to/euv cargo build --release
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

  The few native elements the pages still use are the ones the library has no
  component for, or a component that cannot carry what the page needs:
  the `<textarea>` (the library has no text area component), the `<select>` of
  the page size (the library has a dropdown, not a select), the hidden file
  input of the pickers (the upload component hands the caller a display
  string, not the files), the read only fields of the JSON editor (the input
  component has no read only mode), the tables of the monitor and the tracking
  page (the table component takes static cell texts, the pages show the
  values of the running server) and the `<nav>` of the shell (the navbar of
  the library is a horizontal bar, the shell carries a sidebar).
- A class is written with the design tokens of the framework (`var!(…)`),
  never with a hard coded colour or spacing.
- The pages hold their state in signals, and every piece of state that a
  user can change lives in a hook of the package.
- The dependencies of a manifest follow the order of the project: the crate of
  the workspace first, a blank line, then the crates of the registry by the
  length of their entry. The shared crate is declared in the workspace
  dependencies of `frontend/Cargo.toml` and the pages inherit it, the way the
  other packages of the repository inherit theirs.

  The check of the project reads a single manifest on its own, and a manifest
  that is read on its own does not name the members of its workspace, so the
  shared crate looks like a crate of the registry to it and the order of the
  entries looks wrong. The order of the entries is right: the check of the
  workspace
  (`verify_dep_order.py frontend`) passes, and the manifests of the server
  itself are written the same way.
