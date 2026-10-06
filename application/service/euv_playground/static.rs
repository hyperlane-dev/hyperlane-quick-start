use super::*;

/// Shared cargo target directory for all `wasm-pack build` invocations.
/// Persisted across requests so cargo only has to compile the euv +
/// wasm-bindgen + web-sys dependency tree once; subsequent builds
/// reuse cached artifacts and complete in 1-3s instead of 20s+.
pub static EUV_PLAYGROUND_SHARED_TARGET_DIR: LazyLock<PathBuf> =
    LazyLock::new(|| temp_dir().join(EUV_PLAYGROUND_SHARED_TARGET_DIR_NAME));

/// Global in-memory registry of build jobs.
///
/// Both the controller (which inserts `pending` rows) and the worker
/// (which transitions them through `running` → `success`/`failed`)
/// reach this map through the same lazy handle so there is no risk of
/// accidentally constructing a second instance that would lose jobs.
pub static BUILD_JOB_REGISTRY: LazyLock<BuildJobRegistry> =
    LazyLock::new(|| Arc::new(tokio::sync::RwLock::new(HashMap::new())));

/// Process-wide counter that hands out unique [`BuildJobId`] values.
pub(crate) static NEXT_BUILD_JOB_ID: AtomicU64 = AtomicU64::new(1);
