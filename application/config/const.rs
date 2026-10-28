/// Separator embedded in a build-task payload.
pub(crate) const BUILD_TASK_SEPARATOR: &str = "\u{1f}";

/// Directory used for uploaded files in development mode.
#[cfg(debug_assertions)]
pub(crate) const UPLOAD_DIR: &str = "./data/dev/upload";

/// Directory used for uploaded files in production mode.
#[cfg(not(debug_assertions))]
pub(crate) const UPLOAD_DIR: &str = "./data/release/upload";

/// URL route prefix used for uploaded files.
pub(crate) const STATIC_ROUTE: &str = "upload/file";

/// Directory used for GitHub Pages content in development mode.
#[cfg(debug_assertions)]
pub(crate) const CACHE_DIR: &str = "./data/dev/github_pages";

/// Directory used for GitHub Pages content in production mode.
#[cfg(not(debug_assertions))]
pub(crate) const CACHE_DIR: &str = "./data/release/github_pages";

/// Directory containing the frontend's static resources.
pub(crate) const STATIC_RESOURCES_DIR: &str = "./resources/static";
