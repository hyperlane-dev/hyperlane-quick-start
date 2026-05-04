use super::*;

#[derive(Clone, Data, Debug, Default, Deserialize, Serialize)]
pub struct Notification {
    pub(super) id: i32,
    pub(super) user_id: i32,
    pub(super) title: String,
    pub(super) content: String,
    pub(super) notification_type: String,
    pub(super) is_read: bool,
    pub(super) created_at: Option<NaiveDateTime>,
}
