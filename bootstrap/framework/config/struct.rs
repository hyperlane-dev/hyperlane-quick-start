use super::*;

/// Bootstrap handler for initializing the server and request configuration from the environment.
#[derive(Clone, Data, Debug, Default)]
pub struct ConfigBootstrap {
    /// The server configuration containing address, TTL, and no-delay settings.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) server_config: ServerConfig,
    /// The request configuration containing max body size and read timeout.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) request_config: RequestConfig,
}
