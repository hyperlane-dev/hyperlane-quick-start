use super::*;

#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct NotificationResponse {
    pub(super) id: String,
    pub(super) user_id: String,
    pub(super) title: String,
    pub(super) content: String,
    pub(super) notification_type: String,
    #[get(type(copy))]
    pub(super) is_read: bool,
    #[get(type(copy))]
    pub(super) created_at: i64,
}

#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct NotificationListResponse {
    pub(super) notifications: Vec<NotificationResponse>,
    #[get(type(copy))]
    pub(super) total: i64,
    #[get(type(copy))]
    pub(super) page: i32,
    #[get(type(copy))]
    pub(super) limit: u64,
}

#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct UnreadCountResponse {
    #[get(type(copy))]
    pub(super) count: i64,
}
