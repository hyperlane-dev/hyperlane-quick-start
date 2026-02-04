use super::*;

#[instrument_trace]
pub fn get_global_websocket() -> &'static WebSocket {
    GLOBAL_WEBSOCKET.get_or_init(WebSocket::new)
}
