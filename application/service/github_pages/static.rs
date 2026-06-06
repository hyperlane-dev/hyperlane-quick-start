use super::*;

/// Global static storage for cached GitHub Pages resources.
///
/// Maps `"owner/repository"` keys to their corresponding cached resource lists.
/// Initialized lazily on first access via `GithubPagesService::get_or_init_resources`.
pub(super) static GITHUB_PAGES_RESOURCES: OnceLock<
    RwLock<HashMap<String, Vec<GithubPagesResource>>>,
> = OnceLock::new();
