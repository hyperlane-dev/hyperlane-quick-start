use hyperlane::OptionDuration;

pub(crate) static SERVER_PORT: usize = 60000;
pub(crate) static SERVER_HOST: &str = "0.0.0.0";
pub(crate) static SERVER_WEB_SOCKET_BUFFER_SIZE: usize = 4096;
pub(crate) static SERVER_HTTP_LINE_BUFFER_SIZE: usize = 4096;
pub(crate) static SERVER_LOG_SIZE: usize = 100_024_000;
pub(crate) static SERVER_LOG_DIR: &str = "./logs";
pub(crate) static SERVER_INNER_PRINT: bool = true;
pub(crate) static SERVER_INNER_LOG: bool = true;
pub(crate) static SERVER_NODELAY: bool = true;
pub(crate) static SERVER_LINGER: OptionDuration = None;
pub(crate) static SERVER_TTI: u32 = 128;
