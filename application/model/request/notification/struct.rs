use super::*;

#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct CreateNotificationRequest {
    pub(super) title: String,
    pub(super) content: String,
    pub(super) notification_type: String,
}

#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct NotificationListQueryRequest {
    pub(super) notification_type: Option<String>,
    #[get(type(copy))]
    pub(super) is_read: Option<bool>,
    #[get(type(copy))]
    pub(super) page: Option<i32>,
    #[get(type(copy))]
    pub(super) limit: Option<u64>,
}
