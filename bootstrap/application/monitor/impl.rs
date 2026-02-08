use super::*;

impl BootstrapAsyncInit for MonitorBootstrap {
    #[instrument_trace]
    async fn init() -> Self {
        MonitorService::start_network_capture().await;
        MonitorService::start_performance_data_collection().await;
        Self
    }
}
