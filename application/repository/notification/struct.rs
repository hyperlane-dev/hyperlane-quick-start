use super::*;

#[derive(Clone, Copy, Data, Debug, Default)]
pub struct NotificationRepository;

#[derive(Clone, Data, Debug, Default)]
pub struct NotificationQuery {
    #[get(type(copy))]
    pub(super) user_id: Option<i32>,
    pub(super) notification_type: Option<String>,
    #[get(type(copy))]
    pub(super) is_read: Option<bool>,
    #[get(type(copy))]
    pub(super) page: i32,
    #[get(type(copy))]
    pub(super) limit: u64,
}
