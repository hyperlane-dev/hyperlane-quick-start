use super::*;

/// Hook invoked when an online status WebSocket connection is established.
#[derive(Clone, Copy, Data, Debug, Default)]
pub struct OnlineConnectedHook;

/// Hook invoked when an online status WebSocket connection is closed.
#[derive(Clone, Copy, Data, Debug, Default)]
pub struct OnlineClosedHook;

/// Service for tracking online user presence via WebSocket connections.
#[derive(Clone, Copy, Data, Debug, Default)]
pub struct OnlineService;
