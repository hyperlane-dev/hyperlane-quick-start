use super::*;

/// Response DTO for a single notification, containing its ID, content, and read status.
#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct NotificationResponse {
    /// The encoded identifier for the notification.
    pub(super) id: String,
    /// The encoded identifier of the user who owns this notification.
    pub(super) user_id: String,
    /// The title of the notification.
    pub(super) title: String,
    /// The body content of the notification.
    pub(super) content: String,
    /// The category type of the notification (e.g., "system", "message", "alert").
    pub(super) notification_type: String,
    /// Flag indicating whether the notification has been read by the user.
    #[get(type(copy))]
    pub(super) is_read: bool,
    /// The Unix timestamp (in milliseconds) when the notification was created.
    #[get(type(copy))]
    pub(super) created_at: i64,
}

/// Paginated response containing a list of notification responses.
#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct NotificationListResponse {
    /// The list of notification DTOs for the current page.
    pub(super) notifications: Vec<NotificationResponse>,
    /// The total number of notifications matching the query.
    #[get(type(copy))]
    pub(super) total: i64,
    /// The current page number (1-based).
    #[get(type(copy))]
    pub(super) page: i32,
    /// The maximum number of items per page.
    #[get(type(copy))]
    pub(super) limit: u64,
}

/// Response containing the count of unread notifications for the current user.
#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct UnreadCountResponse {
    /// The number of unread notifications.
    #[get(type(copy))]
    pub(super) count: i64,
}
