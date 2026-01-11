use super::*;

#[instrument_trace]
pub fn init_log() {
    Logger::init(LOG_LEVEL_FILTER);
}
