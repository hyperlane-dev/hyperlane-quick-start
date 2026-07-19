use super::*;

/// Application-level model representing a notification with its metadata and read status.
#[derive(Clone, Data, Debug, Default, Deserialize, Serialize)]
pub struct Notification {
    /// The unique identifier for the notification.
    pub(super) id: i32,
    /// The foreign key referencing the user who owns this notification.
    pub(super) user_id: i32,
    /// The title of the notification.
    pub(super) title: String,
    /// The body content of the notification.
    pub(super) content: String,
    /// The category type of the notification (e.g., "system", "message", "alert").
    pub(super) notification_type: String,
    /// Flag indicating whether the notification has been read by the user.
    pub(super) is_read: bool,
    /// The timestamp when the notification was created.
    pub(super) created_at: Option<NaiveDateTime>,
}
