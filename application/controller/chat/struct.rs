use super::*;

/// online users route.
#[route("/chat/users/online")]
#[derive(Clone, Copy, Data, Debug, Default)]
pub struct OnlineUsersRoute;

/// Route handler for the chat WebSocket endpoint.
#[route("/api/chat")]
#[derive(Clone, Copy, Data, Debug, Default)]
pub struct ChatRoute;

/// chat history route.
#[route("/api/chat/history")]
#[derive(Clone, Copy, Data, Debug, Default)]
pub struct ChatHistoryRoute;
