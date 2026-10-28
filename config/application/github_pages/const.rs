/// Template string for github pages base url template.
pub(crate) const BASE_URL_TEMPLATE: &str = "https://{owner}.github.io/{repository}/";

/// Key for github pages owner key.
pub(crate) const OWNER_KEY: &str = "owner";

/// Key for github pages repository key.
pub(crate) const REPOSITORY_KEY: &str = "repository";

/// Repositories to synchronize on startup.
///
/// Each tuple contains `(owner, repository)` pairs that will be automatically
/// synced when the server starts, fetching the latest resources from GitHub Pages.
pub(crate) const SYNC_REPOSITORIES: &[(&str, &str)] = &[
    ("euv-dev", "euv"),
    ("euv-dev", "euv-docs"),
    ("docs-pages", "pages"),
];

/// Message queue topic name for GitHub Pages sync tasks.
pub(crate) const TOPIC_GITHUB_PAGES_SYNC: &str = "github_pages_sync";

/// Message queue consumer group name for GitHub Pages sync workers.
pub(crate) const CONSUMER_GROUP_SYNC_WORKER: &str = "sync_worker";

/// Directory path for github pages cache dir.
#[cfg(debug_assertions)]
pub(crate) const CACHE_DIR: &str = "./data/dev/github_pages";

/// Directory path for github pages cache dir.
#[cfg(not(debug_assertions))]
pub(crate) const CACHE_DIR: &str = "./data/release/github_pages";
