use super::*;

pub const SERVER_PORT: usize = 65001;
pub const SERVER_HOST: &str = "0.0.0.0";
pub const SERVER_WS_BUFFER: usize = 4096;
pub const SERVER_HTTP_BUFFER: usize = 4096;
pub const SERVER_LOG_SIZE: usize = 100_024_00;
pub const SERVER_LOG_DIR: &str = "/shell/logs";
pub const SERVER_INNER_PRINT: bool = true;
pub const SERVER_INNER_LOG: bool = true;
pub const SERVER_NODELAY: bool = true;
pub const SERVER_LINGER: Duration = Duration::ZERO;
pub const SERVER_TTI: u32 = 128;

pub const CACHE_CONTROL_STATIC_ASSETS: &str = "public, max-age=31536000, immutable";
pub const CACHE_CONTROL_SHORT_TERM: &str = "public, max-age=3600";
pub const EXPIRES_FAR_FUTURE: &str = "Wed, 1 Apr 8888 00:00:00 GMT";
