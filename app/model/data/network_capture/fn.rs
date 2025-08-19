use super::*;

pub fn init_network_capture_globals() {
    let _ = NETWORK_CAPTURE_STATS.set(Arc::new(Mutex::new(None)));
    let _ = CAPTURE_STATUS.set(Arc::new(Mutex::new(CaptureStatus::Stopped)));
    let _ = ACTIVE_CONNECTIONS.set(Arc::new(Mutex::new(HashMap::new())));
}

pub fn get_network_stats() -> Option<NetworkStats> {
    NETWORK_CAPTURE_STATS.get()?.lock().ok()?.clone()
}

pub fn set_network_stats(stats: NetworkStats) {
    if let Some(global_stats) = NETWORK_CAPTURE_STATS.get() {
        if let Ok(mut guard) = global_stats.lock() {
            *guard = Some(stats);
        }
    }
}

pub fn get_capture_status() -> CaptureStatus {
    CAPTURE_STATUS
        .get()
        .and_then(|status| status.lock().ok())
        .map(|guard| guard.clone())
        .unwrap_or(CaptureStatus::Stopped)
}

pub fn set_capture_status(status: CaptureStatus) {
    if let Some(global_status) = CAPTURE_STATUS.get() {
        if let Ok(mut guard) = global_status.lock() {
            *guard = status;
        }
    }
}

pub fn add_connection(connection_id: String, info: ConnectionInfo) {
    if let Some(connections) = ACTIVE_CONNECTIONS.get() {
        if let Ok(mut guard) = connections.lock() {
            guard.insert(connection_id, info);
        }
    }
}

pub fn remove_connection(connection_id: &str) {
    if let Some(connections) = ACTIVE_CONNECTIONS.get() {
        if let Ok(mut guard) = connections.lock() {
            guard.remove(connection_id);
        }
    }
}

pub fn get_active_connections() -> HashMap<String, ConnectionInfo> {
    if let Some(connections) = ACTIVE_CONNECTIONS.get() {
        if let Ok(guard) = connections.lock() {
            guard.clone()
        } else {
            HashMap::new()
        }
    } else {
        HashMap::new()
    }
}
