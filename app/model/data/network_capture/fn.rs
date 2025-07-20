use super::*;

/// Initialize global network capture state variables
///
/// This function sets up the global static variables for network capture.
/// It should only be called once during application startup.
pub fn init_network_capture_globals() {
    let _ = NETWORK_CAPTURE_STATS.set(Arc::new(Mutex::new(None)));
    let _ = CAPTURE_STATUS.set(Arc::new(Mutex::new(CaptureStatus::Stopped)));
    let _ = ACTIVE_CONNECTIONS.set(Arc::new(Mutex::new(HashMap::new())));
}

/// Get the current network statistics
///
/// # Returns
/// An Option containing the current NetworkStats if available
pub fn get_network_stats() -> Option<NetworkStats> {
    NETWORK_CAPTURE_STATS.get()?.lock().ok()?.clone()
}

/// Set the network statistics
///
/// # Arguments
/// * `stats` - The NetworkStats to store
pub fn set_network_stats(stats: NetworkStats) {
    if let Some(global_stats) = NETWORK_CAPTURE_STATS.get() {
        if let Ok(mut guard) = global_stats.lock() {
            *guard = Some(stats);
        }
    }
}

/// Get the current capture status
///
/// # Returns
/// The current CaptureStatus, defaults to Stopped if unavailable
pub fn get_capture_status() -> CaptureStatus {
    CAPTURE_STATUS
        .get()
        .and_then(|status| status.lock().ok())
        .map(|guard| guard.clone())
        .unwrap_or(CaptureStatus::Stopped)
}

/// Set the capture status
///
/// # Arguments
/// * `status` - The CaptureStatus to set
pub fn set_capture_status(status: CaptureStatus) {
    if let Some(global_status) = CAPTURE_STATUS.get() {
        if let Ok(mut guard) = global_status.lock() {
            *guard = status;
        }
    }
}

/// Add or update connection information
///
/// # Arguments
/// * `connection_id` - Unique identifier for the connection
/// * `info` - ConnectionInfo to store
pub fn add_connection(connection_id: String, info: ConnectionInfo) {
    if let Some(connections) = ACTIVE_CONNECTIONS.get() {
        if let Ok(mut guard) = connections.lock() {
            guard.insert(connection_id, info);
        }
    }
}

/// Remove a connection by ID
///
/// # Arguments
/// * `connection_id` - The connection ID to remove
pub fn remove_connection(connection_id: &str) {
    if let Some(connections) = ACTIVE_CONNECTIONS.get() {
        if let Ok(mut guard) = connections.lock() {
            guard.remove(connection_id);
        }
    }
}

/// Get all active connections
///
/// # Returns
/// A HashMap of all active connections
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
