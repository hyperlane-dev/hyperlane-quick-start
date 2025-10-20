use super::*;

pub const SERVER_PORT: usize = 65002;
pub const SERVER_HOST: &str = "0.0.0.0";
pub const SERVER_BUFFER: usize = 4096;
pub const SERVER_LOG_SIZE: usize = 100_024_000;
pub const SERVER_LOG_DIR: &str = "/shell/logs";
pub const SERVER_LOG_LEVEL: [&str; 3] = ["info", "warn", "error"];
pub const SERVER_INNER_PRINT: bool = true;
pub const SERVER_INNER_LOG: bool = true;
pub const SERVER_NODELAY: bool = false;
pub const SERVER_LINGER: OptionDuration = None;
pub const SERVER_TTI: u32 = 128;
pub const SERVER_PID_FILE_PATH: &str = "./tmp/process/hyperlane.pid";

pub const CACHE_CONTROL_STATIC_ASSETS: &str = "public, max-age=31536000, immutable";
pub const CACHE_CONTROL_SHORT_TERM: &str = "public, max-age=3600";
pub const EXPIRES_FAR_FUTURE: &str = "Wed, 1 Apr 8888 00:00:00 GMT";
