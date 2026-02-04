use super::*;

#[route("/chat/users/online")]
#[derive(Clone, Copy, Data, Debug, Default)]
pub struct OnlineUsersRoute;

#[route("/api/chat")]
#[derive(Clone, Copy, Data, Debug, Default)]
pub struct ChatRoute;

#[route("/api/chat/history")]
#[derive(Clone, Copy, Data, Debug, Default)]
pub struct ChatHistoryRoute;
