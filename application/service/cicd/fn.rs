use super::*;

/// get log stream manager.
#[instrument_trace]
pub fn get_log_stream_manager() -> &'static LogStreamManager {
    LOG_STREAM_MANAGER.get_or_init(LogStreamManager::new)
}
