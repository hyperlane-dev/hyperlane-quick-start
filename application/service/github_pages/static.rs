use super::*;

/// Shared HTTP client reused across all fetch calls to avoid creating a new
/// connection pool (DNS resolution + TCP connection) for every request.
///
/// Initialized on first access with the configured timeout and redirect policy.
pub(crate) static SHARED_HTTP_CLIENT: OnceLock<Client> = OnceLock::new();

/// Global map of resource keys (`"{owner}/{repository}/{normalized_path}"`) to
/// pending fetch handles, used to deduplicate concurrent remote fetches.
///
/// When `fetch_resource` needs to fetch a resource from the remote origin, it
/// first checks this map. If an entry exists, the caller waits on the watch
/// receiver instead of starting a second fetch. The designated fetcher removes
/// the entry after notifying all waiters.
pub(crate) static PENDING_FETCHES: OnceLock<RwLock<HashMap<String, Arc<PendingFetch>>>> =
    OnceLock::new();
