use super::*;

thread_local! {
    /// The open socket, absent while the client is disconnected.
    pub static SOCKET: RefCell<Option<WebSocket>> = const { RefCell::new(None) };

    /// The number of times the client tried to connect again.
    pub static RECONNECT_ATTEMPTS: Cell<u32> = const { Cell::new(0) };
}
