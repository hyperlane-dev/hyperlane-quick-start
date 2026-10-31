/// Message queue topic name for euv playground build tasks.
///
/// When a client POSTs to `/api/euv/playground/run`, the controller
/// publishes a `BuildTaskPayload` onto this topic and immediately returns
/// the new build job id; the consumer started by
/// `bootstrap::application::euv_playground` consumes the message, runs
/// `wasm-pack`, and writes the final status back to the in-memory job
/// registry that `EuvPlaygroundService::get_build_status` reads.
pub(crate) const TOPIC_EUV_PLAYGROUND_BUILD: &str = "euv_playground_build";

/// Consumer group name for the euv playground build worker.
///
/// Each instance of the server joins this group; every published message
/// is delivered to exactly one consumer so concurrent `run` requests
/// are load-balanced across the worker pool.
pub(crate) const CONSUMER_GROUP_BUILD_WORKER: &str = "build_worker";

/// Separator used inside the wire payload to delimit the build job id
/// from the user / project identifiers.
///
/// The full payload layout is:
/// `{job_id}{SEPARATOR}{user_id}{SEPARATOR}{project_id}{SEPARATOR}{code}`
///
/// Using a byte that is allowed inside JSON-escaped Rust source is not
/// possible, so the worker reads the payload as raw bytes and splits on
/// the first two occurrences of `SEPARATOR` only — the trailing `code`
/// is treated as the remainder. This keeps the wire format simple while
/// still round-tripping arbitrary user code.
pub(crate) const BUILD_TASK_SEPARATOR: &str = "\u{1f}";

/// How long a finished (`success` / `failed`) build job is retained in
/// memory before being purged. Long enough for the frontend to poll the
/// terminal state several times after completion, short enough that
/// abandoned clients don't grow the registry without bound.
pub(crate) const BUILD_JOB_TTL_MS: i64 = 5 * 60 * 1000;

/// How often the worker purges expired finished jobs from the registry.
pub(crate) const BUILD_JOB_GC_INTERVAL_MS: u64 = 30 * 1000;

/// Capacity hint passed to `MessageQueueBroker::create_topic_with_capacity`
/// for the build topic. Cold builds can queue up while a hot build is in
/// flight, so the buffer is sized generously.
pub(crate) const BUILD_TOPIC_CAPACITY: usize = 1024;

/// Status of a build job stored in the in-memory registry.
pub mod build_status {
    /// Job has been queued but the worker has not started wasm-pack yet.
    pub const PENDING: &str = "pending";
    /// Worker has picked up the job and `wasm-pack` is running.
    pub const RUNNING: &str = "running";
    /// Worker finished and the build directory was published successfully.
    pub const SUCCESS: &str = "success";
    /// Worker finished but the build failed (compile error, timeout, etc.).
    pub const FAILED: &str = "failed";
}
