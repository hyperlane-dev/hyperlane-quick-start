use super::*;

/// Service for managing GitHub Pages on-demand caching and resource proxying.
///
/// Provides operations to fetch resources with local cache-first strategy.
#[derive(Clone, Copy, Data, Debug, Default)]
pub struct GithubPagesService;
