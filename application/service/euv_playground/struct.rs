use super::*;

/// Service for the euv online playground — encodes project ids, manages
/// per-user project directories, and drives `wasm-pack` builds. All
/// methods are stateless so the struct is zero-sized and
/// `#[derive(Clone, Copy, Default)]` is enough to make it freely
/// shareable.
#[derive(Clone, Copy, Data, Debug, Default)]
pub struct EuvPlaygroundService;

/// One queued or completed build job.
///
/// The worker writes to this struct while the controller / status endpoint
/// reads from it, so the mutable parts are guarded by an inner `RwLock`.
/// The struct itself stays inside an `Arc` inside the registry so multiple
/// status-readers can hold a reference without re-locking the registry.
#[derive(Clone, Data, Debug)]
pub struct BuildJob {
    /// Globally unique build job id (monotonic across server lifetime).
    /// Typed as concrete `u64` so lombok's generated getter returns
    /// `&u64` and the controller setter accepts `u64` by value without
    /// any extra deref ceremony.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) job_id: u64,
    /// Owning user id (so status polling can authorize the request).
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) user_id: i32,
    /// Owning project id (used to publish the final `build_url`).
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) project_id: i64,
    /// Current status — one of `build_status::*`.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) status: String,
    /// Absolute URL the frontend should load in its preview iframe. Empty
    /// until the build succeeds.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) build_url: String,
    /// Combined `wasm-pack` stderr; non-empty when `status` is `failed`.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) stderr: String,
    /// Wall-clock timestamp (ms since unix epoch) the job was created.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) created_at_ms: i64,
    /// Wall-clock timestamp (ms since unix epoch) of the last status change.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) updated_at_ms: i64,
}
