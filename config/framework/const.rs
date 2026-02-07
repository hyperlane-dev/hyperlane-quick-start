use super::*;

#[cfg(debug_assertions)]
pub const SERVER_PORT: u16 = DEFAULT_WEB_PORT;
#[cfg(not(debug_assertions))]
pub const SERVER_PORT: u16 = 65002;
pub const SERVER_HOST: &str = DEFAULT_HOST;
pub const SERVER_BUFFER: usize = DEFAULT_BUFFER_SIZE;
pub const SERVER_LOG_SIZE: usize = 100_024_000;
pub const SERVER_LOG_DIR: &str = "./tmp/logs";
pub const SERVER_INNER_PRINT: bool = true;
pub const SERVER_INNER_LOG: bool = true;
pub const SERVER_NODELAY: bool = false;
pub const SERVER_TTI: u32 = 128;
pub const SERVER_PID_FILE_PATH: &str = "./tmp/process/hyperlane.pid";
pub const SERVER_REQUEST_HTTP_READ_TIMEOUT_MS: u64 = 60000;
pub const SERVER_REQUEST_MAX_BODY_SIZE: usize = MB_100;

pub const CACHE_CONTROL_STATIC_ASSETS: &str = "public, max-age=31536000, immutable";
pub const CACHE_CONTROL_SHORT_TERM: &str = "public, max-age=3600";
pub const EXPIRES_FAR_FUTURE: &str = "Wed, 1 Apr 8888 00:00:00 GMT";
