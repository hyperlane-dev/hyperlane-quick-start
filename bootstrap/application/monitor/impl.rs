use super::*;

/// Implementation of `MonitorBootstrap` for `BootstrapAsyncInit`.
impl BootstrapAsyncInit for MonitorBootstrap {
    /// Starts the network capture and performance data collection coroutines.
    #[instrument_trace]
    async fn init() -> Self {
        MonitorService::start_network_capture().await;
        MonitorService::start_performance_data_collection().await;
        Self
    }
}
