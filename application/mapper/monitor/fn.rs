use super::*;

/// Returns the global network capture statistics slot, initializing it on first use.
///
/// # Returns
///
/// - `&'static RwLock<Option<NetworkStats>>` - The global network statistics slot.
#[instrument_trace]
fn get_or_init_network_capture_stats() -> &'static RwLock<Option<NetworkStats>> {
    NETWORK_CAPTURE_STATS.get_or_init(|| RwLock::new(None))
}

/// Returns the global capture status slot, initializing it on first use.
///
/// # Returns
///
/// - `&'static RwLock<CaptureStatus>` - The global capture status slot.
#[instrument_trace]
fn get_or_init_capture_status() -> &'static RwLock<CaptureStatus> {
    CAPTURE_STATUS.get_or_init(|| RwLock::new(CaptureStatus::Stopped))
}

/// Returns the global active connections map, initializing it on first use.
///
/// # Returns
///
/// - `&'static RwLock<HashMap<String, ConnectionInfo>>` - The global active connections map.
#[instrument_trace]
fn get_or_init_active_connections() -> &'static RwLock<HashMap<String, ConnectionInfo>> {
    ACTIVE_CONNECTIONS.get_or_init(|| RwLock::new(HashMap::new()))
}

/// Returns the global performance history buffer, initializing it on first use.
///
/// # Returns
///
/// - `&'static RwLock<PerformanceRingBuffer>` - The global performance history buffer.
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

/// Returns a copy of the currently recorded network statistics.
///
/// # Returns
///
/// - `Option<NetworkStats>` - The recorded statistics, or `None` when nothing is captured.
#[instrument_trace]
pub async fn get_network_stats() -> Option<NetworkStats> {
    get_or_init_network_capture_stats().read().await.clone()
}

/// Replaces the currently recorded network statistics.
///
/// # Arguments
///
/// - `NetworkStats` - The statistics snapshot to store.
pub async fn set_network_stats(stats: NetworkStats) {
    *get_or_init_network_capture_stats().write().await = Some(stats);
}

/// Returns a copy of the current packet capture status.
///
/// # Returns
///
/// - `CaptureStatus` - The current capture status.
#[instrument_trace]
pub async fn get_capture_status() -> CaptureStatus {
    get_or_init_capture_status().read().await.clone()
}

/// Updates the current packet capture status.
///
/// # Arguments
///
/// - `CaptureStatus` - The status to store.
#[instrument_trace]
pub async fn set_capture_status(status: CaptureStatus) {
    *get_or_init_capture_status().write().await = status;
}

/// Adds a network connection to the active connections map.
///
/// # Arguments
///
/// - `String` - The unique connection identifier.
/// - `ConnectionInfo` - The connection metadata to store.
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
/// - `&str` - The unique connection identifier to remove.
#[instrument_trace]
pub async fn remove_connection(connection_id: &str) {
    get_or_init_active_connections()
        .write()
        .await
        .remove(connection_id);
}

/// Returns a copy of all currently active network connections.
///
/// # Returns
///
/// - `HashMap<String, ConnectionInfo>` - The active connections keyed by identifier.
#[instrument_trace]
pub async fn get_active_connections() -> HashMap<String, ConnectionInfo> {
    get_or_init_active_connections().read().await.clone()
}

/// Appends a performance sample to the global history buffer.
///
/// # Arguments
///
/// - `PerformanceDataPoint` - The sample to append.
#[instrument_trace]
pub async fn add_performance_data_point(data_point: PerformanceDataPoint) {
    get_or_init_performance_history()
        .write()
        .await
        .push(data_point);
}

/// Returns every retained performance sample in chronological order.
///
/// # Returns
///
/// - `Vec<PerformanceDataPoint>` - The retained performance samples.
#[instrument_trace]
pub async fn get_performance_history() -> Vec<PerformanceDataPoint> {
    get_or_init_performance_history()
        .read()
        .await
        .get_all_sorted()
}

/// Returns the retained performance samples whose timestamp lies within a range.
///
/// # Arguments
///
/// - `u64` - The inclusive range boundary, in milliseconds.
///
/// # Returns
///
/// - `Vec<PerformanceDataPoint>` - The performance samples inside the range.
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

/// Returns the most recent performance samples, newest last.
///
/// # Arguments
///
/// - `usize` - The maximum number of samples to return.
///
/// # Returns
///
/// - `Vec<PerformanceDataPoint>` - The most recent performance samples.
#[instrument_trace]
pub async fn get_recent_performance_data(n: usize) -> Vec<PerformanceDataPoint> {
    get_or_init_performance_history().read().await.get_recent(n)
}

/// clear performance history.
#[instrument_trace]
pub async fn clear_performance_history() {
    get_or_init_performance_history().write().await.clear();
}
