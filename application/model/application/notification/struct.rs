use super::*;

/// Application-level model representing a notification with its metadata and read status.
#[derive(Clone, Data, Debug, Default, Deserialize, Serialize)]
pub struct Notification {
    /// The unique identifier for the notification.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) id: i32,
    /// The foreign key referencing the user who owns this notification.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) user_id: i32,
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
    /// The timestamp when the notification was created.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) created_at: Option<NaiveDateTime>,
}
