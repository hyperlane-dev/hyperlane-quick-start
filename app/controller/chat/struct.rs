use super::*;

#[route("/chat/users/online")]
#[derive(Clone, Copy, Debug, Default)]
pub struct OnlineUsersRoute;

#[route("/api/chat")]
#[derive(Clone, Copy, Debug, Default)]
pub struct ChatRoute;

#[route("/api/chat/history")]
#[derive(Clone, Copy, Debug, Default)]
pub struct ChatHistoryRoute;
