#[cfg(debug_assertions)]
pub const UPLOAD_DIR: &str = "./data/dev/upload";
#[cfg(not(debug_assertions))]
pub const UPLOAD_DIR: &str = "./data/release/upload";
pub const STATIC_ROUTE: &str = "upload/file";
pub const UPLOAD_DIR_KEY: &str = "upload_dir";
pub const UPLOAD_FILE_KEY: &str = "upload_file";
pub const PARTIAL_CONTENT_STATUS: usize = 206;
