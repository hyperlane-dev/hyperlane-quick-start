use super::*;

/// Bootstrap handler for initializing the server and request configuration from the environment.
#[derive(Clone, Data, Debug, Default)]
pub struct ConfigBootstrap {
    /// The server configuration containing address, TTL, and no-delay settings.
    pub(super) server_config: ServerConfig,
    /// The request configuration containing max body size and read timeout.
    pub(super) request_config: RequestConfig,
}
