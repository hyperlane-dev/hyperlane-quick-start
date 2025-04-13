use hyperlane::OptionDuration;

pub(crate) const SERVER_PORT: usize = 60000;
pub(crate) const SERVER_HOST: &str = "0.0.0.0";
pub(crate) const SERVER_WEB_SOCKET_BUFFER_SIZE: usize = 4096;
pub(crate) const SERVER_HTTP_LINE_BUFFER_SIZE: usize = 4096;
pub(crate) const SERVER_LOG_SIZE: usize = 100_024_000;
pub(crate) const SERVER_LOG_DIR: &str = "./logs";
pub(crate) const SERVER_INNER_PRINT: bool = true;
pub(crate) const SERVER_INNER_LOG: bool = true;
pub(crate) const SERVER_NODELAY: bool = true;
pub(crate) const SERVER_LINGER: OptionDuration = None;
pub(crate) const SERVER_TTI: u32 = 128;
