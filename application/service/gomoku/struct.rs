use super::*;

/// Hook invoked when a gomoku WebSocket connection is established.
#[derive(Clone, Copy, Data, Debug, Default)]
pub struct GomokuConnectedHook;

/// Hook invoked when a gomoku WebSocket request is received.
#[derive(Clone, Copy, Data, Debug, Default)]
pub struct GomokuRequestHook;

/// Hook invoked after a gomoku WebSocket message has been sent.
#[derive(Clone, Copy, Data, Debug, Default)]
pub struct GomokuSendedHook;

/// Hook invoked when a gomoku WebSocket connection is closed.
#[derive(Clone, Copy, Data, Debug, Default)]
pub struct GomokuClosedHook;

/// Service for handling gomoku game WebSocket connections and game logic.
#[derive(Clone, Copy, Data, Debug, Default)]
pub struct GomokuWebSocketService;

/// Manager for broadcasting gomoku game messages to room subscribers and tracking user-room associations.
#[derive(Clone, Data, Debug)]
pub struct RoomBroadcastManager {
    /// The broadcast channel map for publishing game messages to room subscribers.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) broadcast_map: Arc<BroadcastMap<String>>,
    /// The map of user IDs to their subscribed room IDs.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) user_subscriptions: ArcRwLock<HashMap<String, String>>,
}
