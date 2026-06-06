use super::*;

/// Bootstrap handler for initializing the Tokio async runtime with multi-thread configuration.
#[derive(Data, Debug)]
pub struct RuntimeBootstrap {
    /// The Tokio runtime instance used for async operations.
    pub(super) runtime: Runtime,
}
