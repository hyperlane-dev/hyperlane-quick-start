use super::*;

/// notification create route.
#[route("/api/notification/create")]
#[derive(Clone, Copy, Data, Debug, Default)]
pub struct NotificationCreateRoute;

/// notification list route.
#[route("/api/notification/list")]
#[derive(Clone, Copy, Data, Debug, Default)]
pub struct NotificationListRoute;

/// notification get route.
#[route("/api/notification/get/{id}")]
#[derive(Clone, Copy, Data, Debug, Default)]
pub struct NotificationGetRoute;

/// notification read route.
#[route("/api/notification/read/{id}")]
#[derive(Clone, Copy, Data, Debug, Default)]
pub struct NotificationReadRoute;

/// notification read all route.
#[route("/api/notification/read-all")]
#[derive(Clone, Copy, Data, Debug, Default)]
pub struct NotificationReadAllRoute;

/// notification delete route.
#[route("/api/notification/delete/{id}")]
#[derive(Clone, Copy, Data, Debug, Default)]
pub struct NotificationDeleteRoute;

/// notification unread count route.
#[route("/api/notification/unread-count")]
#[derive(Clone, Copy, Data, Debug, Default)]
pub struct NotificationUnreadCountRoute;
