use super::*;

pub const SERVER_PORT: usize = 60000;
pub const SERVER_HOST: &'static str = "0.0.0.0";
pub const SERVER_BUFFER: usize = 4096;
pub const SERVER_LOG_SIZE: usize = 100_024_000;
pub const SERVER_LOG_DIR: &'static str = "./tmp/logs";
pub const SERVER_INNER_PRINT: bool = true;
pub const SERVER_INNER_LOG: bool = true;
pub const SERVER_NODELAY: bool = false;
pub const SERVER_LINGER: OptionDuration = None;
pub const SERVER_TTI: u32 = 128;
