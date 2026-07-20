use super::*;

/// Shared cargo target directory for all `wasm-pack build` invocations.
/// Persisted across requests so cargo only has to compile the euv +
/// wasm-bindgen + web-sys dependency tree once; subsequent builds
/// reuse cached artifacts and complete in 1-3s instead of 20s+.
pub static EUV_PLAYGROUND_SHARED_TARGET_DIR: LazyLock<PathBuf> =
    LazyLock::new(|| temp_dir().join("euv-playground-target"));
