use super::*;

/// Service for managing GitHub Pages on-demand caching and resource proxying.
///
/// Provides operations to fetch resources with local cache-first strategy.
#[derive(Clone, Copy, Data, Debug, Default)]
pub struct GithubPagesService;

/// Tracks an in-flight remote resource fetch so that concurrent callers
/// requesting the same resource can await the same result instead of each
/// performing a separate remote fetch.
#[derive(Clone, Debug, Default)]
pub(crate) struct PendingFetch {
    pub(crate) tx: FetchPendingSender,
}
