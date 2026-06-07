use super::*;

#[instrument_trace]
fn get_or_init_network_capture_stats() -> &'static RwLock<Option<NetworkStats>> {
    NETWORK_CAPTURE_STATS.get_or_init(|| RwLock::new(None))
}

#[instrument_trace]
fn get_or_init_capture_status() -> &'static RwLock<CaptureStatus> {
    CAPTURE_STATUS.get_or_init(|| RwLock::new(CaptureStatus::Stopped))
}

#[instrument_trace]
fn get_or_init_active_connections() -> &'static RwLock<HashMap<String, ConnectionInfo>> {
    ACTIVE_CONNECTIONS.get_or_init(|| RwLock::new(HashMap::new()))
}

#[instrument_trace]
fn get_or_init_performance_history() -> &'static RwLock<PerformanceRingBuffer> {
    PERFORMANCE_HISTORY.get_or_init(|| RwLock::new(PerformanceRingBuffer::default()))
}

/// init network capture globals.
#[instrument_trace]
pub fn init_network_capture_globals() {
    let _: &RwLock<Option<NetworkStats>> = get_or_init_network_capture_stats();
    let _: &RwLock<CaptureStatus> = get_or_init_capture_status();
    let _: &RwLock<HashMap<String, ConnectionInfo>> = get_or_init_active_connections();
    let _: &RwLock<PerformanceRingBuffer> = get_or_init_performance_history();
}

/// get network stats.
#[instrument_trace]
pub async fn get_network_stats() -> Option<NetworkStats> {
    get_or_init_network_capture_stats().read().await.clone()
}

/// set network stats.
pub async fn set_network_stats(stats: NetworkStats) {
    *get_or_init_network_capture_stats().write().await = Some(stats);
}

/// get capture status.
#[instrument_trace]
pub async fn get_capture_status() -> CaptureStatus {
    get_or_init_capture_status().read().await.clone()
}

/// set capture status.
#[instrument_trace]
pub async fn set_capture_status(status: CaptureStatus) {
    *get_or_init_capture_status().write().await = status;
}

/// Adds a network connection to the active connections map.
///
/// # Arguments
///
/// - `String`: The unique connection identifier.
/// - `ConnectionInfo`: The connection metadata to store.
#[instrument_trace]
pub async fn add_connection(connection_id: String, info: ConnectionInfo) {
    get_or_init_active_connections()
        .write()
        .await
        .insert(connection_id, info);
}

/// Removes a network connection from the active connections map by its identifier.
///
/// # Arguments
///
/// - `&str`: The unique connection identifier to remove.
#[instrument_trace]
pub async fn remove_connection(connection_id: &str) {
    get_or_init_active_connections()
        .write()
        .await
        .remove(connection_id);
}

/// get active connections.
#[instrument_trace]
pub async fn get_active_connections() -> HashMap<String, ConnectionInfo> {
    get_or_init_active_connections().read().await.clone()
}

/// add performance data point.
#[instrument_trace]
pub async fn add_performance_data_point(data_point: PerformanceDataPoint) {
    get_or_init_performance_history()
        .write()
        .await
        .push(data_point);
}

/// get performance history.
#[instrument_trace]
pub async fn get_performance_history() -> Vec<PerformanceDataPoint> {
    get_or_init_performance_history()
        .read()
        .await
        .get_all_sorted()
}

/// get performance history range.
#[instrument_trace]
pub async fn get_performance_history_range(
    start_timestamp: u64,
    end_timestamp: u64,
) -> Vec<PerformanceDataPoint> {
    get_or_init_performance_history()
        .read()
        .await
        .get_range(start_timestamp, end_timestamp)
}

/// get recent performance data.
#[instrument_trace]
pub async fn get_recent_performance_data(n: usize) -> Vec<PerformanceDataPoint> {
    get_or_init_performance_history().read().await.get_recent(n)
}

/// clear performance history.
#[instrument_trace]
pub async fn clear_performance_history() {
    get_or_init_performance_history().write().await.clear();
}
