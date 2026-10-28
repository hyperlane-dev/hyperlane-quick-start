use super::*;

/// Returns the `BASE_URL_TEMPLATE` configuration value.
///
/// # Returns
///
/// - `&'static str` - The configured value.
pub fn base_url_template() -> &'static str {
    BASE_URL_TEMPLATE
}

/// Returns the `OWNER_KEY` configuration value.
///
/// # Returns
///
/// - `&'static str` - The configured value.
pub fn owner_key() -> &'static str {
    OWNER_KEY
}

/// Returns the `REPOSITORY_KEY` configuration value.
///
/// # Returns
///
/// - `&'static str` - The configured value.
pub fn repository_key() -> &'static str {
    REPOSITORY_KEY
}

/// Returns the `SYNC_REPOSITORIES` configuration value.
///
/// # Returns
///
/// - `&'static [(&'static str, &'static str)]` - The configured value.
pub fn sync_repositories() -> &'static [(&'static str, &'static str)] {
    SYNC_REPOSITORIES
}

/// Returns the `TOPIC_GITHUB_PAGES_SYNC` configuration value.
///
/// # Returns
///
/// - `&'static str` - The configured value.
pub fn topic_github_pages_sync() -> &'static str {
    TOPIC_GITHUB_PAGES_SYNC
}

/// Returns the `CONSUMER_GROUP_SYNC_WORKER` configuration value.
///
/// # Returns
///
/// - `&'static str` - The configured value.
pub fn consumer_group_sync_worker() -> &'static str {
    CONSUMER_GROUP_SYNC_WORKER
}

#[cfg(debug_assertions)]
/// Returns the `CACHE_DIR` configuration value.
///
/// # Returns
///
/// - `&'static str` - The configured value.
pub fn cache_dir() -> &'static str {
    CACHE_DIR
}

#[cfg(not(debug_assertions))]
/// Returns the `CACHE_DIR` configuration value.
///
/// # Returns
///
/// - `&'static str` - The configured value.
pub fn cache_dir() -> &'static str {
    CACHE_DIR
}
