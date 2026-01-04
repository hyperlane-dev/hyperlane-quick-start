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
