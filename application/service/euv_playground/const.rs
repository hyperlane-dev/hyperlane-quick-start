/// Directory under a user home that contains Cargo-managed tools.
pub const EUV_PLAYGROUND_CARGO_HOME_DIR: &str = ".cargo";

/// Cargo subdirectory that contains installed executable binaries.
pub const EUV_PLAYGROUND_CARGO_BIN_DIR: &str = "bin";

/// Environment variable that provides executable search directories.
pub const EUV_PLAYGROUND_PATH_ENV: &str = "PATH";

/// Environment variable that explicitly overrides the wasm-pack executable.
pub const EUV_PLAYGROUND_WASM_PACK_ENV: &str = "EUV_PLAYGROUND_WASM_PACK";

/// Environment variable that points to Cargo's installation root.
pub const EUV_PLAYGROUND_CARGO_HOME_ENV: &str = "CARGO_HOME";

/// Environment variable that points to the current user's home directory.
pub const EUV_PLAYGROUND_HOME_ENV: &str = "HOME";

/// Environment variable that points to the current Windows user's profile.
pub const EUV_PLAYGROUND_USERPROFILE_ENV: &str = "USERPROFILE";

/// Executable filename used for wasm-pack on Windows.
#[cfg(windows)]
pub const EUV_PLAYGROUND_WASM_PACK_BINARY_NAME: &str = "wasm-pack.exe";

/// Executable filename used for wasm-pack on Unix-like systems.
#[cfg(not(windows))]
pub const EUV_PLAYGROUND_WASM_PACK_BINARY_NAME: &str = "wasm-pack";

/// Prefix for the temporary directory created per
/// `POST /api/euv-playground/run` request. The pid + counter + epoch second
/// disambiguator makes a unique path even when two requests land in the
/// same millisecond.
pub const EUV_PLAYGROUND_BUILD_DIR_PREFIX: &str = "euv-playground-";

/// Hard cap on source code size submitted through the playground. Larger
/// payloads are rejected to keep temporary builds cheap and predictable.
pub const EUV_PLAYGROUND_MAX_CODE_BYTES: usize = 64 * 1024;

/// Hard cap on project name length (chars).
pub const EUV_PLAYGROUND_MAX_NAME_LEN: usize = 64;

/// Hard cap on list size returned to the frontend (most-recent first).
pub const EUV_PLAYGROUND_MAX_LIST_ITEMS: usize = 200;

/// Default timeout for a single `wasm-pack build` invocation. Cold builds
/// can take ~30s while euv + wasm-bindgen are compiled from scratch;
/// subsequent runs are typically <2s once the cargo target dir is warm.
pub const EUV_PLAYGROUND_BUILD_TIMEOUT_SECS: u64 = 180;

/// Root directory under `data/` where all per-user playground projects
/// are persisted. Layout:
///   `{ROOT}/{user_id}/{project_id}/code.rs`
///   `{ROOT}/{user_id}/{project_id}/metadata.json`
/// The root lives under `data/` (alongside the existing dev/release log
/// trees) and is created lazily on first write.
pub const EUV_PLAYGROUND_DATA_DIR: &str = "./data/euv_playground";

/// Per-project build output root. Each project has at most one
/// `tmp/{project_id}/` directory; running the project overwrites it.
/// Layout:
///   `{ROOT}/{project_id}/www/index.html`
///   `{ROOT}/{project_id}/www/pkg/euv_app.js`
///   `{ROOT}/{project_id}/www/pkg/euv_app_bg.wasm`
///   ... (rest of the wasm-pack pkg output)
/// The root lives under `resources/static/` so the existing static-resource
/// route serves it directly — no extra view/controller needed.
pub const EUV_PLAYGROUND_BUILDS_DIR: &str = "./resources/static/euv-playground/tmp";

/// Filename for the Rust source code inside a project directory.
pub const EUV_PLAYGROUND_CODE_FILE: &str = "code.rs";

/// Filename for the JSON metadata file (name + timestamps).
pub const EUV_PLAYGROUND_META_FILE: &str = "metadata.json";

/// Filename for the per-user monotonic project-id counter.
pub const EUV_PLAYGROUND_SEQ_FILE: &str = "_seq";

/// Default code pre-filled into a brand-new project so the user has
/// something runnable from the start.
///
/// The starter renders a centered euv-ui counter card with a reactive
/// count and Add / Reset buttons. Two handlers demonstrate the
/// multi-event pattern. We use `r##"..."##` because the body contains
/// `App::mount("#app", ...)`.
pub const EUV_PLAYGROUND_DEFAULT_CODE: &str = r##"use {euv::*, euv_ui::*, wasm_bindgen::prelude::*, web_sys::*};

class! {
    c_euv_playground_root {
        display: "flex";
        flex-direction: "column";
        justify-content: "center";
        align-items: "center";
        min-height: "100vh";
        box-sizing: "border-box";
        padding: var!(space-2xl);
        gap: var!(gap-section);
        background: var!(background);
        color: var!(foreground);
        text-align: "center";
    }

    c_euv_card {
        c_card();
        width: "168px";
    }
}

fn app() -> VirtualNode {
    let count: Signal<i32> = App::use_signal(|| 0);

    let add_event = move |_: Event| {
        count.set(count.get() + 1);
    };

    html! {
        div {
            class: c_euv_playground_root()
            div {
                class: c_euv_card()
                h3 {
                    class: c_card_title()
                    "Hello euv playground"
                }
                div {
                    class: c_info_row()
                    span {
                        class: c_info_label()
                        "Count: " count
                    }
                }
                div {
                    class: c_button_controls()
                    button {
                        class: c_euv_button_primary_md()
                        onclick: add_event
                        "Add"
                    }
                }
            }
        }
    }
}

#[wasm_bindgen]
pub fn main() {
    App::mount("#app", app);
}
"##;

/// `Cargo.toml` body generated for every playground build. Pins euv +
/// wasm-bindgen to versions known to compile against the server toolchain;
/// users cannot pull arbitrary crates so cold-start latency stays bounded.
///
/// Versions are pinned to a specific release (not `*`) so cargo
/// metadata resolves the same crate every build and so dependency
/// churn in the wider crates.io ecosystem can never silently break a
/// playground compile. Bump these in lockstep with the euv-frontend
/// crates' published versions.
pub const EUV_PLAYGROUND_BUILD_CARGO_TOML: &str = r#"[package]
name = "euv_app"
version = "0.0.0"
edition = "2024"

[lib]
crate-type = ["cdylib"]

[dependencies]
euv = "*"
euv-ui = "*"
wasm-bindgen = "*"
console_error_panic_hook = "*"
"#;

/// `www/index.html` shell injected into every playground build. Uses `src=`
/// on the module script so `app.js` can rewrite the URL to a `data:` blob
/// URL that inlines the wasm + glue JS without needing same-origin access.
pub const EUV_PLAYGROUND_BUILD_INDEX_HTML: &str = r#"<!doctype html>
<html lang="en">
  <head>
    <meta charset="utf-8" />
    <meta
      name="viewport"
      content="width=device-width, initial-scale=1.0, maximum-scale=1.0, user-scalable=no, viewport-fit=cover, interactive-widget=resizes-visual"
    />
    <meta name="mobile-web-app-capable" content="yes" />
    <meta name="apple-mobile-web-app-capable" content="yes" />
    <meta
      name="apple-mobile-web-app-status-bar-style"
      content="black-translucent"
    />
    <meta
      name="description"
      content="A declarative, cross-platform UI framework for Rust with virtual DOM, reactive signals, and HTML macros for WebAssembly."
    />
    <meta
      name="keywords"
      content="rust, webassembly, wasm, ui-framework, virtual-dom, reactive, declarative-ui, euv"
    />
    <meta property="og:title" content="euv" />
    <meta
      property="og:description"
      content="A declarative, cross-platform UI framework for Rust with virtual DOM, reactive signals, and HTML macros for WebAssembly."
    />
    <meta property="og:type" content="website" />
    <title>Euv</title>
    <style>
      * {
        -webkit-font-smoothing: antialiased;
        -moz-osx-font-smoothing: grayscale;
        text-rendering: optimizeLegibility;
      }
      canvas {
        image-rendering: auto;
      }
    </style>
  </head>
  <body>
    <div id="app"></div>
  </body>
  <script type="module">
    import init, { main } from './pkg/euv_app.js';
    await init();
    main();
  </script>
</html>
"#;

/// Error returned by [`EuvPlaygroundService::encode_id`] when the
/// underlying `Encode::execute` call fails.
pub const ERROR_FAILED_TO_ENCODE_ID: &str = "Failed to encode ID";

/// Error returned by [`EuvPlaygroundService::decode_id`] when the input
/// does not round-trip through `Decode::execute` and then `parse::<i64>()`.
pub const ERROR_INVALID_ID_FORMAT: &str = "Invalid ID format";
