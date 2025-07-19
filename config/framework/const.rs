use super::*;

pub const SERVER_PORT: usize = 60006;
pub const SERVER_HOST: &str = "0.0.0.0";
pub const SERVER_WS_BUFFER: usize = 4096;
pub const SERVER_HTTP_BUFFER: usize = 4096;
pub const SERVER_LOG_SIZE: usize = 100_024_00;
pub const SERVER_LOG_DIR: &str = "/shell/logs";
pub const SERVER_INNER_PRINT: bool = true;
pub const SERVER_INNER_LOG: bool = true;
pub const SERVER_NODELAY: bool = true;
pub const SERVER_LINGER: OptionDuration = None;
pub const SERVER_TTI: u32 = 128;
