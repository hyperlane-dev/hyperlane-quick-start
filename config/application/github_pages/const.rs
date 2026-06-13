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
pub const SYNC_REPOSITORIES: &[(&str, &str)] = &[("euv-dev", "euv"), ("eastspire", "docs-pages")];

/// Directory path for github pages cache dir.
#[cfg(debug_assertions)]
pub const CACHE_DIR: &str = "./data/dev/github_pages";

/// Directory path for github pages cache dir.
#[cfg(not(debug_assertions))]
pub const CACHE_DIR: &str = "./data/release/github_pages";
