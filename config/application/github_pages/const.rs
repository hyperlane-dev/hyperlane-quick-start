/// Template string for github pages base url template.
pub const GITHUB_PAGES_BASE_URL_TEMPLATE: &str = "https://{owner}.github.io/{repository}/";

/// Key for github pages owner key.
pub const GITHUB_PAGES_OWNER_KEY: &str = "owner";

/// Key for github pages repository key.
pub const GITHUB_PAGES_REPOSITORY_KEY: &str = "repository";

/// Directory path for github pages cache dir.
#[cfg(debug_assertions)]
pub const GITHUB_PAGES_CACHE_DIR: &str = "./data/dev/github_pages";

/// Directory path for github pages cache dir.
#[cfg(not(debug_assertions))]
pub const GITHUB_PAGES_CACHE_DIR: &str = "./data/release/github_pages";

/// Interval value for github pages sync secs.
pub const GITHUB_PAGES_SYNC_INTERVAL_SECS: u64 = 600;

/// Github pages auto sync repositories.
pub const GITHUB_PAGES_AUTO_SYNC_REPOSITORIES: &[(&str, &str)] = &[("euv-dev", "euv")];
