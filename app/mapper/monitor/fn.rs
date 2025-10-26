use super::*;

pub fn init_network_capture_globals() {
    let _: Result<(), Arc<RwLock<Option<NetworkStats>>>> =
        NETWORK_CAPTURE_STATS.set(Arc::new(RwLock::new(None)));
    let _: Result<(), Arc<RwLock<CaptureStatus>>> =
        CAPTURE_STATUS.set(Arc::new(RwLock::new(CaptureStatus::Stopped)));
    let _: Result<(), Arc<RwLock<HashMap<String, ConnectionInfo>>>> =
        ACTIVE_CONNECTIONS.set(Arc::new(RwLock::new(HashMap::new())));
}

pub fn get_network_stats() -> Option<NetworkStats> {
    NETWORK_CAPTURE_STATS.get()?.read().ok()?.clone()
}

pub fn set_network_stats(stats: NetworkStats) {
    if let Some(global_stats) = NETWORK_CAPTURE_STATS.get() {
        if let Ok(mut guard) = global_stats.write() {
            *guard = Some(stats);
        }
    }
}

pub fn get_capture_status() -> CaptureStatus {
    CAPTURE_STATUS
        .get()
        .and_then(|status| status.read().ok())
        .map(|guard| guard.clone())
        .unwrap_or(CaptureStatus::Stopped)
}

pub fn set_capture_status(status: CaptureStatus) {
    if let Some(global_status) = CAPTURE_STATUS.get() {
        if let Ok(mut guard) = global_status.write() {
            *guard = status;
        }
    }
}

pub fn add_connection(connection_id: String, info: ConnectionInfo) {
    if let Some(connections) = ACTIVE_CONNECTIONS.get() {
        if let Ok(mut guard) = connections.write() {
            guard.insert(connection_id, info);
        }
    }
}

pub fn remove_connection(connection_id: &str) {
    if let Some(connections) = ACTIVE_CONNECTIONS.get() {
        if let Ok(mut guard) = connections.write() {
            guard.remove(connection_id);
        }
    }
}

pub fn get_active_connections() -> HashMap<String, ConnectionInfo> {
    if let Some(connections) = ACTIVE_CONNECTIONS.get() {
        if let Ok(guard) = connections.read() {
            guard.clone()
        } else {
            HashMap::new()
        }
    } else {
        HashMap::new()
    }
}
