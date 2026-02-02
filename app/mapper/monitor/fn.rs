use super::*;

#[instrument_trace]
pub fn init_network_capture_globals() {
    let _: Result<(), Arc<RwLock<Option<NetworkStats>>>> =
        NETWORK_CAPTURE_STATS.set(arc_rwlock(None));
    let _: Result<(), Arc<RwLock<CaptureStatus>>> =
        CAPTURE_STATUS.set(arc_rwlock(CaptureStatus::Stopped));
    let _: Result<(), Arc<RwLock<HashMap<String, ConnectionInfo>>>> =
        ACTIVE_CONNECTIONS.set(arc_rwlock(HashMap::new()));
}

#[instrument_trace]
pub async fn get_network_stats() -> Option<NetworkStats> {
    NETWORK_CAPTURE_STATS.get()?.read().await.clone()
}

pub async fn set_network_stats(stats: NetworkStats) {
    if let Some(global_stats) = NETWORK_CAPTURE_STATS.get() {
        *global_stats.write().await = Some(stats);
    }
}

#[instrument_trace]
pub async fn get_capture_status() -> CaptureStatus {
    match CAPTURE_STATUS.get() {
        Some(status) => status.read().await.clone(),
        None => CaptureStatus::Stopped,
    }
}

#[instrument_trace]
pub async fn set_capture_status(status: CaptureStatus) {
    if let Some(global_status) = CAPTURE_STATUS.get() {
        *global_status.write().await = status;
    }
}

#[instrument_trace]
pub async fn add_connection(connection_id: String, info: ConnectionInfo) {
    if let Some(connections) = ACTIVE_CONNECTIONS.get() {
        connections.write().await.insert(connection_id, info);
    }
}

#[instrument_trace]
pub async fn remove_connection(connection_id: &str) {
    if let Some(connections) = ACTIVE_CONNECTIONS.get() {
        connections.write().await.remove(connection_id);
    }
}

#[instrument_trace]
pub async fn get_active_connections() -> HashMap<String, ConnectionInfo> {
    if let Some(connections) = ACTIVE_CONNECTIONS.get() {
        connections.read().await.clone()
    } else {
        HashMap::new()
    }
}
