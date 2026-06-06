pub const GITHUB_PAGES_BASE_URL_TEMPLATE: &str = "https://{owner}.github.io/{repository}/";
pub const GITHUB_PAGES_OWNER_KEY: &str = "owner";
pub const GITHUB_PAGES_REPOSITORY_KEY: &str = "repository";

#[cfg(debug_assertions)]
pub const GITHUB_PAGES_CACHE_DIR: &str = "./data/dev/github_pages";
#[cfg(not(debug_assertions))]
pub const GITHUB_PAGES_CACHE_DIR: &str = "./data/release/github_pages";

pub const GITHUB_PAGES_SYNC_INTERVAL_SECS: u64 = 600;
pub const GITHUB_PAGES_AUTO_SYNC_REPOSITORIES: &[(&str, &str)] = &[("euv-dev", "euv")];
