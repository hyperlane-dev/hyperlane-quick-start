use super::*;

#[derive(Clone, Data, Debug, Default)]
pub struct ConfigBootstrap {
    pub(super) server_config: ServerConfig,
    pub(super) request_config: RequestConfig,
}
