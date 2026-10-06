use super::*;

/// Monotonic build-job id assigned by [`EuvPlaygroundService::next_job_id`].
///
/// Stored as `u64` so the value fits cleanly into a JSON `Number` and the
/// `/api/euv/playground/run/status/{id}` URL path segment without quoting.
pub type BuildJobId = u64;

/// Inner guard around a single [`BuildJob`] so multiple status-readers
/// can hold their own `Arc` without locking the outer map. The
/// `tokio::sync::RwLock` is used (not `std::sync`) because the worker
/// updates the row from a tokio task and `try_read` is the only way to
/// peek without `.await`.
pub type BuildJobSlot = Arc<tokio::sync::RwLock<BuildJob>>;

/// Registry of in-flight and recently-completed build jobs.
///
/// A single global instance backs both the controller (which inserts
/// `pending` rows and reads the latest status) and the worker (which
/// transitions rows to `running` → `success`/`failed`). Keys are
/// [`BuildJobId`]; values are [`BuildJobSlot`]s so multiple status
/// readers can hold their own reference without re-locking the outer
/// map.
pub type BuildJobMap = HashMap<BuildJobId, BuildJobSlot>;

/// Registry handle — the global shared [`BuildJobMap`]. Wrapped in an
/// outer `Arc<RwLock<_>>` so the GC task, the worker, and concurrent
/// status readers can race without one starving the others.
pub type BuildJobRegistry = Arc<tokio::sync::RwLock<BuildJobMap>>;
