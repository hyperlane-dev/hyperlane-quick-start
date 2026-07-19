/// File system directory for storing uploaded files in development mode.
#[cfg(debug_assertions)]
pub const UPLOAD_DIR: &str = "./data/dev/upload";

/// File system directory for storing uploaded files in release/production mode.
#[cfg(not(debug_assertions))]
pub const UPLOAD_DIR: &str = "./data/release/upload";

/// URL route prefix for accessing uploaded files via HTTP.
pub const STATIC_ROUTE: &str = "upload/file";

/// Upload dir key.
pub const UPLOAD_DIR_KEY: &str = "upload_dir";

/// Upload file key.
pub const UPLOAD_FILE_KEY: &str = "upload_file";

/// Status code for partial content status.
pub const PARTIAL_CONTENT_STATUS: usize = 206;
