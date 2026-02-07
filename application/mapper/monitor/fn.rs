use super::*;

#[instrument_trace]
pub fn init_network_capture_globals() {}

#[instrument_trace]
pub async fn get_network_stats() -> Option<NetworkStats> {
    NETWORK_CAPTURE_STATS.read().await.clone()
}

pub async fn set_network_stats(stats: NetworkStats) {
    *NETWORK_CAPTURE_STATS.write().await = Some(stats);
}

#[instrument_trace]
pub async fn get_capture_status() -> CaptureStatus {
    CAPTURE_STATUS.read().await.clone()
}

#[instrument_trace]
pub async fn set_capture_status(status: CaptureStatus) {
    *CAPTURE_STATUS.write().await = status;
}

#[instrument_trace]
pub async fn add_connection(connection_id: String, info: ConnectionInfo) {
    ACTIVE_CONNECTIONS.write().await.insert(connection_id, info);
}

#[instrument_trace]
pub async fn remove_connection(connection_id: &str) {
    ACTIVE_CONNECTIONS.write().await.remove(connection_id);
}

#[instrument_trace]
pub async fn get_active_connections() -> HashMap<String, ConnectionInfo> {
    ACTIVE_CONNECTIONS.read().await.clone()
}

#[instrument_trace]
pub async fn add_performance_data_point(data_point: PerformanceDataPoint) {
    PERFORMANCE_HISTORY.write().await.push(data_point);
}

#[instrument_trace]
pub async fn get_performance_history() -> Vec<PerformanceDataPoint> {
    PERFORMANCE_HISTORY.read().await.get_all_sorted()
}

#[instrument_trace]
pub async fn get_performance_history_range(
    start_timestamp: u64,
    end_timestamp: u64,
) -> Vec<PerformanceDataPoint> {
    PERFORMANCE_HISTORY
        .read()
        .await
        .get_range(start_timestamp, end_timestamp)
}

#[instrument_trace]
pub async fn get_recent_performance_data(n: usize) -> Vec<PerformanceDataPoint> {
    PERFORMANCE_HISTORY.read().await.get_recent(n)
}

#[instrument_trace]
pub async fn clear_performance_history() {
    PERFORMANCE_HISTORY.write().await.clear();
}
