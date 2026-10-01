use super::*;

/// get global websocket.
///
/// # Returns
///
/// - `&'static WebSocket` - The global websocket.
#[instrument_trace]
pub fn get_global_websocket() -> &'static WebSocket {
    GLOBAL_WEBSOCKET.get_or_init(WebSocket::new)
}
