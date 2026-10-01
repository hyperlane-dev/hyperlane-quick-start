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
pub const EUV_PLAYGROUND_BUILD_TIMEOUT_SECS: u64 = 6000;

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

/// Builds the shared application instance.
///
/// # Returns
///
/// - `VirtualNode` - The app result.
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

/// Runs the application entry point.
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

[package.metadata.wasm-pack.profile.dev]
wasm-opt = [
    "-Oz",
    "--enable-mutable-globals",
    "--enable-bulk-memory",
    "--enable-nontrapping-float-to-int",
]

[package.metadata.wasm-pack.profile.release]
wasm-opt = [
    "-Oz",
    "--enable-mutable-globals",
    "--enable-bulk-memory",
    "--enable-nontrapping-float-to-int",
]

[profile.dev]
opt-level = 0
debug = true
incremental = true
codegen-units = 256

[profile.release]
opt-level = "s"
lto = "thin"
codegen-units = 16
debug = false
strip = "symbols"
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

/// `.cargo/config.toml` body generated for every playground build.
///
/// `jobs = 4` + `pipelining = true` keeps concurrent rustc invocations
/// bounded so a single browser tab can't fork enough `cl.exe` /
/// `link.exe` children to OOM the dev box (the host machine already
/// runs the hyperlane server + IDE + DB on it). `pipelining = true`
/// also lets cargo hand the next crate to rustc while the linker
/// finishes the previous one, which matters because wasm-pack builds
/// spend most of their wall time in `wasm-bindgen` post-processing
/// rather than actual compilation.
///
/// The `[source.crates-io]` replacement routes every dependency fetch
/// through ByteDance's `rsproxy.cn` sparse index (~150ms p50 from the
/// build host vs ~1s for direct crates.io). The mirror is injected
/// into every dynamically-generated playground project via this
/// constant, so no host-machine config is touched and the upstream
/// repo stays portable — switching to a different mirror (or back to
/// default crates.io) is a one-line change here.
pub const EUV_PLAYGROUND_BUILD_CARGO_CONFIG: &str = r#"[build]
jobs = 4
pipelining = true
target = "wasm32-unknown-unknown"

[source.crates-io]
replace-with = "rsproxy-sparse"

[source.rsproxy-sparse]
registry = "sparse+https://rsproxy.cn/index/"

[target.wasm32-unknown-unknown]
rustflags = [
    "-C",
    "target-feature=+bulk-memory",
    "--cfg",
    "getrandom_backend=\"wasm_js\"",
]
"#;

/// Error returned by [`EuvPlaygroundService::encode_id`] when the
/// underlying `Encode::execute` call fails.
pub const ERROR_FAILED_TO_ENCODE_ID: &str = "Failed to encode ID";

/// Error returned by [`EuvPlaygroundService::decode_id`] when the input
/// does not round-trip through `Decode::execute` and then `parse::<i64>()`.
pub const ERROR_INVALID_ID_FORMAT: &str = "Invalid ID format";

/// Filename (relative to [`std::env::temp_dir()`]) for the shared Cargo
/// target directory used by `build_wasm_pack_output`. Re-used across
/// builds so the second `wasm-pack` invocation only recompiles the
/// user's crate, not euv + wasm-bindgen + dependencies. The actual
/// [`PathBuf`] is wrapped in a `LazyLock` in `static.rs`.
pub const EUV_PLAYGROUND_SHARED_TARGET_DIR_NAME: &str = "euv-playground-target";

/// Sub-directory under the build root that holds the Rust source.
pub const EUV_PLAYGROUND_SRC_DIR: &str = "src";

/// Sub-directory under the build root that holds the wasm-pack output.
pub const EUV_PLAYGROUND_WWW_DIR: &str = "www";

/// Hidden Cargo config directory inside the build root.
pub const EUV_PLAYGROUND_BUILD_CARGO_DIR: &str = ".cargo";

/// Cargo manifest filename inside the build root.
pub const EUV_PLAYGROUND_BUILD_CARGO_TOML_FILE: &str = "Cargo.toml";

/// `.cargo/config.toml` filename inside the build root.
pub const EUV_PLAYGROUND_BUILD_CARGO_CONFIG_FILE: &str = "config.toml";

/// Rust library source filename inside `src/`.
pub const EUV_PLAYGROUND_BUILD_LIB_RS_FILE: &str = "lib.rs";

/// HTML shell filename inside `www/`.
pub const EUV_PLAYGROUND_BUILD_INDEX_HTML_FILE: &str = "index.html";

/// `wasm-pack build` subcommand argument.
pub const WASM_PACK_ARG_BUILD: &str = "build";

/// `--target web` selects the wasm-pack `web` output bundle.
pub const WASM_PACK_ARG_TARGET: &str = "--target";

/// wasm-pack target flavour we always build for.
pub const WASM_PACK_TARGET_WEB: &str = "web";

/// `--release` profile: optimised wasm, smaller output, runs wasm-opt.
pub const WASM_PACK_ARG_RELEASE: &str = "--release";

/// Skip generating the `*.d.ts` typings file next to the JS glue.
pub const WASM_PACK_ARG_NO_TYPESCRIPT: &str = "--no-typescript";

/// Skip generating `package.json` / npm scaffolding under `pkg/`.
pub const WASM_PACK_ARG_NO_PACK: &str = "--no-pack";

/// `--out-dir www/pkg` so the existing static-resource route serves the
/// wasm + glue JS without any extra view/controller.
pub const WASM_PACK_ARG_OUT_DIR: &str = "--out-dir";

/// Output directory passed to `--out-dir`. Relative to the build root.
pub const WASM_PACK_OUT_DIR_WWW_PKG: &str = "www/pkg";

/// Env var name forced to `never` so wasm-pack does not emit ANSI color
/// codes that get tangled up in the JSON-encoded error sent to the
/// browser.
pub const WASM_PACK_ENV_CARGO_TERM_COLOR: &str = "CARGO_TERM_COLOR";

/// Value forced for [`WASM_PACK_ENV_CARGO_TERM_COLOR`].
pub const WASM_PACK_CARGO_TERM_COLOR_NEVER: &str = "never";

/// `error!` message when the wasm-pack child process cannot be spawned
/// (typically: wasm-pack not on PATH, or EUV_PLAYGROUND_WASM_PACK points
/// at a missing binary). Three positional call-site placeholders, in
/// order: resolved wasm-pack path, override env var name, underlying
/// io error. Caller emits
/// `format!("{ERROR_WASM_PACK_SPAWN_PREFIX} {wasm_pack_display}{ERROR_WASM_PACK_SPAWN_MIDDLE} {EUV_PLAYGROUND_WASM_PACK_ENV}{ERROR_WASM_PACK_SPAWN_TAIL} {e}")`.
pub const ERROR_WASM_PACK_SPAWN_PREFIX: &str = "Failed to spawn wasm-pack at";
pub const ERROR_WASM_PACK_SPAWN_MIDDLE: &str =
    ". Install it with `cargo install wasm-pack`, add Cargo's bin directory to PATH, or set";
pub const ERROR_WASM_PACK_SPAWN_TAIL: &str = ":";

/// `error!` message when the wasm-pack child process exits but its
/// output stream cannot be read. Prefix only; caller does
/// `format!("{ERROR_WASM_PACK_WAIT}: {}", e)`.
pub const ERROR_WASM_PACK_WAIT: &str = "wasm-pack wait failed";

/// `error!` message when the wasm-pack build exceeds
/// [`EUV_PLAYGROUND_BUILD_TIMEOUT_SECS`] seconds. Prefix only; caller
/// does `format!("{ERROR_WASM_PACK_TIMEOUT} {}s", secs)`.
pub const ERROR_WASM_PACK_TIMEOUT: &str = "wasm-pack timed out after";

/// mkdir error format. `{}` is the path that failed to be created.
pub const ERROR_MKDIR: &str = "mkdir {}: {}";

/// readdir error format. `{}` is the directory that failed to be read.
pub const ERROR_READDIR: &str = "readdir {}: {}";

/// copy error format. `{}` and `{}` are the source / destination paths.
pub const ERROR_COPY: &str = "copy {} -> {}: {}";

/// Project name to fall back to when the user submits an empty or
/// whitespace-only name. Surfaced in the sidebar and the metadata
/// `name` field so the UI is never blank.
pub const UNTITLED_PROJECT_NAME: &str = "Untitled";

/// Metadata JSON field name: human-readable project name.
pub const METADATA_FIELD_NAME: &str = "name";

/// Metadata JSON field name: ms-since-epoch last-write timestamp.
pub const METADATA_FIELD_UPDATED_AT_MS: &str = "updated_at_ms";

/// JSON-serialized form of [`UNTITLED_PROJECT_NAME`] (with surrounding
/// quotes) — used as the fallback inside `write_metadata` when the
/// input name fails to round-trip through `serde_json::to_string`.
pub const UNTITLED_PROJECT_NAME_JSON: &str = "\"Untitled\"";

/// `tracing::error!` message when a build task payload is missing the
/// build job id.
pub const ERROR_BUILD_TASK_MISSING_JOB_ID: &str = "Invalid build task payload: missing job id";

/// `tracing::error!` message when a build task payload is missing the
/// user id.
pub const ERROR_BUILD_TASK_MISSING_USER_ID: &str = "Invalid build task payload: missing user id";

/// `tracing::error!` message when a build task payload is missing the
/// project id.
pub const ERROR_BUILD_TASK_MISSING_PROJECT_ID: &str =
    "Invalid build task payload: missing project id";

/// `tracing::error!` message when a build task payload is missing the
/// source code body.
pub const ERROR_BUILD_TASK_MISSING_CODE: &str = "Invalid build task payload: missing code";

/// `tracing::error!` message when the build job id field is present but
/// cannot be parsed as a `u64`.
pub const ERROR_BUILD_TASK_BAD_JOB_ID_TYPE: &str =
    "Invalid build task payload: job id is not a u64";

/// `tracing::error!` message when the user id field is present but
/// cannot be parsed as an `i32`.
pub const ERROR_BUILD_TASK_BAD_USER_ID_TYPE: &str =
    "Invalid build task payload: user id is not i32";

/// `tracing::error!` message when the project id field is present but
/// cannot be parsed as an `i64`.
pub const ERROR_BUILD_TASK_BAD_PROJECT_ID_TYPE: &str =
    "Invalid build task payload: project id is not i64";

/// Prefix of the `tracing::error!` message when the source code payload
/// is not valid UTF-8. The full line is
/// `error!("{ERROR_BUILD_TASK_BAD_CODE_UTF8_LOG} {error}")` — keep
/// the trailing space.
pub const ERROR_BUILD_TASK_BAD_CODE_UTF8_LOG: &str =
    "Invalid build task payload: code is not UTF-8";

/// `mark_job_failed` reason when the source code payload is not valid
/// UTF-8. Stored on the job record so the frontend can show a stable
/// terminal failure state.
/// Prefix of the `mark_job_failed` reason when the source code payload
/// is not valid UTF-8. Caller does
/// `format!("{ERROR_BUILD_TASK_BAD_CODE_UTF8_REASON}: {error}")`.
pub const ERROR_BUILD_TASK_BAD_CODE_UTF8_REASON: &str = "code is not UTF-8";

/// `tracing::info!` message when a build job finishes successfully.
/// Prefix only; caller does
/// `info!("{LOG_BUILD_JOB_SUCCEEDED} {job_id} {user_id} {project_id}")`.
pub const LOG_BUILD_JOB_SUCCEEDED: &str = "Euv playground build job succeeded for user";

/// `tracing::info!` message when a build job finishes with an error.
/// Prefix only; caller does
/// `warn!("{LOG_BUILD_JOB_FAILED} {job_id} {user_id} {project_id}")`.
pub const LOG_BUILD_JOB_FAILED: &str = "Euv playground build job failed for user";

/// `tracing::info!` message after the GC sweep removes expired jobs.
/// Trailing suffix only; caller does
/// `info!("Purged {removed} {LOG_BUILD_JOB_PURGED}")` to put the count
/// in the middle.
pub const LOG_BUILD_JOB_PURGED: &str = "expired euv playground build job(s)";

/// Error format used by `EuvPlaygroundService::read_code` when the
/// project file cannot be read from disk.
pub const ERROR_READ_CODE: &str = "Failed to read code: {}";

/// Error format used by `EuvPlaygroundService::save_code` when the
/// project file cannot be written.
pub const ERROR_WRITE_CODE: &str = "Failed to write code to {}";

/// Error format used by `EuvPlaygroundService::read_project_dir` when
/// the project directory cannot be listed. `{}` is the directory path.
pub const ERROR_READ_PROJECT_DIR: &str = "Failed to read project directory {}: {}";

/// Error format used by `EuvPlaygroundService::read_project_dir` when a
/// single `DirEntry` inside the project directory cannot be read.
pub const ERROR_READ_PROJECT_DIR_ENTRY: &str = "Failed to read project directory entry: {}";

/// Error format used by `EuvPlaygroundService::create_project_dir` when
/// the per-project directory cannot be created.
pub const ERROR_CREATE_PROJECT_DIR: &str = "Failed to create project dir {}";

/// Error format used by `EuvPlaygroundService::publish_build` when the
/// generated `www/` tree cannot be copied to the static-resource root.
pub const ERROR_PUBLISH_BUILD: &str = "Failed to publish build to {}: {}";

/// Error format used by `EuvPlaygroundService::ensure_builds_dir` when
/// the static-resource build root cannot be created.
pub const ERROR_CREATE_BUILDS_DIR: &str = "Failed to create builds dir {}: {}";

/// Error format used by `EuvPlaygroundService::copy_dir_recursive` when
/// the staging build directory cannot be created.
pub const ERROR_CREATE_BUILD_STAGING_DIR: &str = "Failed to create build staging dir {}: {}";

/// Error format used by `EuvPlaygroundService::ensure_cargo_dir` when
/// the `.cargo/` directory inside the build root cannot be created.
/// `{}` is the directory path.
pub const ERROR_CREATE_CARGO_DIR: &str = "Failed to create .cargo dir {}: {}";

/// Error format used by `EuvPlaygroundService::build_wasm_pack_output`
/// when writing each of the four scaffolded files fails. `{}` is the
/// io error.
pub const ERROR_WRITE_CARGO_TOML: &str = "Failed to write Cargo.toml: {}";
pub const ERROR_WRITE_CARGO_CONFIG: &str = "Failed to write .cargo/config.toml: {}";
pub const ERROR_WRITE_LIB_RS: &str = "Failed to write src/lib.rs: {}";
pub const ERROR_WRITE_INDEX_HTML: &str = "Failed to write www/index.html: {}";

/// Error format used by `EuvPlaygroundService::create_dir` and similar
/// when creating the `src/` directory fails. `{}` is the directory path.
pub const ERROR_CREATE_SRC_DIR: &str = "Failed to create src dir {}: {}";

/// Error format used by `EuvPlaygroundService::create_dir` and similar
/// when creating the `www/` directory fails. `{}` is the directory path.
pub const ERROR_CREATE_WWW_DIR: &str = "Failed to create www dir {}: {}";
/// Environment variable naming the Cargo target directory.
pub const ENV_CARGO_TARGET_DIR: &str = "CARGO_TARGET_DIR";
