/// The environment variable that names the command line interface binary.
pub(crate) const ENV_CLI: &str = "EUV_CLI";

/// The environment variable that turns the frontend build off.
pub(crate) const ENV_SKIP: &str = "EUV_FRONTEND_SKIP";

/// The environment variable that carries the profile of the build.
pub(crate) const ENV_PROFILE: &str = "PROFILE";

/// The environment variable that carries the output directory of the build.
pub(crate) const ENV_OUT_DIR: &str = "OUT_DIR";

/// The environment variable that carries the directory of the package.
pub(crate) const ENV_MANIFEST_DIR: &str = "CARGO_MANIFEST_DIR";

/// The name of the release profile.
pub(crate) const PROFILE_RELEASE: &str = "release";

/// The name of the development profile.
pub(crate) const PROFILE_DEV: &str = "dev";

/// The directory the frontend workspace lives in.
pub(crate) const FRONTEND_DIR: &str = "frontend";

/// The directory the shared package lives in.
pub(crate) const SHARED_DIR: &str = "shared";

/// The directory the page packages live in.
pub(crate) const PAGES_DIR: &str = "pages";

/// The directory the browser assets are served from.
pub(crate) const STATIC_DIR: &str = "resources/static";

/// The directory the stamps of the finished builds live in.
pub(crate) const STAMP_DIR: &str = "frontend";

/// The name of the manifest of a package.
pub(crate) const MANIFEST_NAME: &str = "Cargo.toml";

/// The environment variable that carries the target directory of a build.
pub(crate) const ENV_TARGET_DIR: &str = "CARGO_TARGET_DIR";

/// The environment variable that carries the link flags of the host build.
pub(crate) const ENV_ENCODED_RUSTFLAGS: &str = "CARGO_ENCODED_RUSTFLAGS";

/// The environment variable that carries the link flags of a build.
pub(crate) const ENV_RUSTFLAGS: &str = "RUSTFLAGS";

/// The name of the target directory of the frontend workspace.
pub(crate) const TARGET_DIR: &str = "target";

/// The name of the log the frontend build writes to.
pub(crate) const LOG_NAME: &str = "frontend-build.log";

/// The name of the page the server serves.
pub(crate) const INDEX_NAME: &str = "index.html";

/// The name of the command line interface of the framework.
pub(crate) const CLI_NAME: &str = "euv";

/// The command of the command line interface that builds a package.
pub(crate) const CLI_BUILD: &str = "build";

/// The argument that names the package the command line interface builds.
pub(crate) const ARG_CRATE_PATH: &str = "--crate-path";

/// The argument that names the directory the artifacts are written to.
pub(crate) const ARG_WWW_DIR: &str = "--www-dir";

/// The argument that builds the release profile.
pub(crate) const ARG_RELEASE: &str = "--release";

/// The argument that builds the development profile.
pub(crate) const ARG_DEV: &str = "--dev";

/// The message reported when the command line interface is missing.
pub(crate) const MESSAGE_NO_CLI: &str =
    "the euv command line interface is required to build the frontend pages; install it with `cargo install euv-cli --version 0.31.3`";
