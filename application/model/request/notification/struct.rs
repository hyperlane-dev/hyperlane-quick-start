use super::*;

/// create notification request.
#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct CreateNotificationRequest {
    /// The title.
    pub(super) title: String,
    /// The content.
    pub(super) content: String,
    /// The notification type.
    pub(super) notification_type: String,
}

/// notification list query request.
#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct NotificationListQueryRequest {
    /// The notification type.
    pub(super) notification_type: Option<String>,
    /// The is read.
    #[get(type(copy))]
    pub(super) is_read: Option<bool>,
    /// The page.
    #[get(type(copy))]
    pub(super) page: Option<i32>,
    /// The limit.
    #[get(type(copy))]
    pub(super) limit: Option<u64>,
}
