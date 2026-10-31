# hyperlane_frontend_blog

The blog page of the Hyperlane Quick Start server.

The page is a WebAssembly application of the [euv](https://crates.io/crates/euv)
framework: the server serves it from the static resource directory and the
browser boots it on its own. It lists the published posts, the posts of the
signed in visitor and the favorited posts, opens a post with its comments,
and writes a new post.

## Building

The build script of the server package builds this package with the
[euv](https://crates.io/crates/euv-cli) command line interface and writes the
artifacts into `resources/static/blog/`. See the [frontend
README](../../README.md) for the toolchain the build needs.

## Layout

```
src/
├── app/       the application shell of the page
├── hook/      the state of the page and the calls it makes
├── model/     the models of the blog API
├── page/      the view of the page
└── style/     the class registry of the page
```
