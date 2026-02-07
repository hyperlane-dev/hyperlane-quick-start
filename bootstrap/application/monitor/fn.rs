use super::*;

#[instrument_trace]
pub async fn init_monitor() {
    MonitorService::start_network_capture().await;
    MonitorService::start_performance_data_collection().await;
}
