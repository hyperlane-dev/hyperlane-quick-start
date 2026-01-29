use super::*;

#[instrument_trace]
pub async fn init_network_capture() {
    MonitorService::start_network_capture().await;
}
