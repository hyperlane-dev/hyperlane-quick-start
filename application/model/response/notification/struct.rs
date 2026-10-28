use super::*;

/// Response DTO for a single notification, containing its ID, content, and read status.
#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct NotificationResponse {
    /// The encoded identifier for the notification.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) id: String,
    /// The encoded identifier of the user who owns this notification.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) user_id: String,
    /// The title of the notification.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) title: String,
    /// The body content of the notification.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) content: String,
    /// The category type of the notification (e.g., "system", "message", "alert").
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) notification_type: String,
    /// Flag indicating whether the notification has been read by the user.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) is_read: bool,
    /// The Unix timestamp (in milliseconds) when the notification was created.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) created_at: i64,
}

/// Paginated response containing a list of notification responses.
#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct NotificationListResponse {
    /// The list of notification DTOs for the current page.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) notifications: Vec<NotificationResponse>,
    /// The total number of notifications matching the query.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) total: i64,
    /// The current page number (1-based).
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) page: i32,
    /// The maximum number of items per page.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) limit: u64,
}

/// Response containing the count of unread notifications for the current user.
#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct UnreadCountResponse {
    /// The number of unread notifications.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) count: i64,
}
