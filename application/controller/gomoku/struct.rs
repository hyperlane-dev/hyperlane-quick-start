use super::*;

/// Route handler for the gomoku game WebSocket endpoint.
#[route("/api/gomoku")]
#[derive(Clone, Copy, Data, Debug, Default)]
pub struct GomokuRoute;
