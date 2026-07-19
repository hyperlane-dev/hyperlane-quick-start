/// Template string for github pages base url template.
pub const BASE_URL_TEMPLATE: &str = "https://{owner}.github.io/{repository}/";

/// Key for github pages owner key.
pub const OWNER_KEY: &str = "owner";

/// Key for github pages repository key.
pub const REPOSITORY_KEY: &str = "repository";

/// Repositories to synchronize on startup.
///
/// Each tuple contains `(owner, repository)` pairs that will be automatically
/// synced when the server starts, fetching the latest resources from GitHub Pages.
pub const SYNC_REPOSITORIES: &[(&str, &str)] = &[("euv-dev", "euv"), ("docs-pages", "pages")];

/// Message queue topic name for GitHub Pages sync tasks.
pub const TOPIC_GITHUB_PAGES_SYNC: &str = "github_pages_sync";

/// Message queue consumer group name for GitHub Pages sync workers.
pub const CONSUMER_GROUP_SYNC_WORKER: &str = "sync_worker";

/// Directory path for github pages cache dir.
#[cfg(debug_assertions)]
pub const CACHE_DIR: &str = "./data/dev/github_pages";

/// Directory path for github pages cache dir.
#[cfg(not(debug_assertions))]
pub const CACHE_DIR: &str = "./data/release/github_pages";
