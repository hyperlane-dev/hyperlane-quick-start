use super::*;

pub static NETWORK_CAPTURE_STATS: OnceLock<Arc<Mutex<Option<NetworkStats>>>> = OnceLock::new();
pub static CAPTURE_STATUS: OnceLock<Arc<Mutex<CaptureStatus>>> = OnceLock::new();
pub static ACTIVE_CONNECTIONS: OnceLock<Arc<Mutex<HashMap<String, ConnectionInfo>>>> = OnceLock::new();

pub fn init_network_capture_globals() {
    NETWORK_CAPTURE_STATS.set(Arc::new(Mutex::new(None))).unwrap();
    CAPTURE_STATUS.set(Arc::new(Mutex::new(CaptureStatus::Stopped))).unwrap();
    ACTIVE_CONNECTIONS.set(Arc::new(Mutex::new(HashMap::new()))).unwrap();
}

pub fn get_network_stats() -> Option<NetworkStats> {
    NETWORK_CAPTURE_STATS
        .get()?
        .lock()
        .ok()?
        .clone()
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
