# hyperlane_frontend_notification

The notification page of the Hyperlane Quick Start server.

The page is a WebAssembly application of the [euv](https://crates.io/crates/euv)
framework: the server serves it from the static resource directory and the
browser boots it on its own. It lists the notifications of the visitor,
filters them by their kind and their read state, marks them as read and
writes new ones.

## Building

The build script of the server package builds this package with the
[euv](https://crates.io/crates/euv-cli) command line interface and writes the
artifacts into `resources/static/notification/`. See the [frontend
README](../../README.md) for the toolchain the build needs.

## Layout

```
src/
├── app/       the application shell of the page
├── hook/      the state of the page and the calls it makes
├── model/     the models of the notification API
├── page/      the view of the page
└── style/     the class registry of the page
```
