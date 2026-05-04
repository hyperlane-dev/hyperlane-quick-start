use super::*;

#[route("/api/notification/create")]
#[derive(Clone, Copy, Data, Debug, Default)]
pub struct NotificationCreateRoute;

#[route("/api/notification/list")]
#[derive(Clone, Copy, Data, Debug, Default)]
pub struct NotificationListRoute;

#[route("/api/notification/get/{id}")]
#[derive(Clone, Copy, Data, Debug, Default)]
pub struct NotificationGetRoute;

#[route("/api/notification/read/{id}")]
#[derive(Clone, Copy, Data, Debug, Default)]
pub struct NotificationReadRoute;

#[route("/api/notification/read-all")]
#[derive(Clone, Copy, Data, Debug, Default)]
pub struct NotificationReadAllRoute;

#[route("/api/notification/delete/{id}")]
#[derive(Clone, Copy, Data, Debug, Default)]
pub struct NotificationDeleteRoute;

#[route("/api/notification/unread-count")]
#[derive(Clone, Copy, Data, Debug, Default)]
pub struct NotificationUnreadCountRoute;
