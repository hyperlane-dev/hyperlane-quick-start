use super::*;

/// One notification of the list.
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[serde(default)]
pub struct NotificationItem {
    /// The opaque identifier of the notification.
    pub id: String,
    /// The identifier of the user the notification belongs to.
    pub user_id: String,
    /// The title of the notification.
    pub title: String,
    /// The body of the notification.
    pub content: String,
    /// The kind of the notification.
    pub notification_type: String,
    /// Whether the notification has been read.
    pub is_read: bool,
    /// The creation time in milliseconds since the Unix epoch.
    pub created_at: i64,
}

/// One page of the notifications of the server.
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[serde(default)]
pub struct NotificationList {
    /// The notifications of the page.
    pub notifications: Vec<NotificationItem>,
    /// The number of notifications the filter holds in total.
    pub total: i64,
    /// The page the response holds.
    pub page: i32,
    /// The number of notifications per page.
    pub limit: u64,
}

/// The number of notifications that have not been read.
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[serde(default)]
pub struct UnreadCount {
    /// The number of notifications that have not been read.
    pub count: i64,
}

/// The notification a visitor writes.
#[derive(Clone, Debug, Default, Serialize)]
pub struct CreateNotificationRequest {
    /// The title of the notification.
    pub title: String,
    /// The body of the notification.
    pub content: String,
    /// The kind of the notification.
    pub notification_type: String,
}
