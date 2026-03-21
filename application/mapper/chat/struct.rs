use super::*;

pub struct ChatHistoryMapper;

#[derive(Clone, Data, Debug, DeriveEntityModel, PartialEq)]
#[sea_orm(table_name = "chat_history")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    #[get(type(copy))]
    pub(super) id: i64,
    pub(super) session_id: String,
    pub(super) sender_name: String,
    pub(super) sender_type: String,
    pub(super) message_type: String,
    pub(super) content: String,
    pub(super) created_at: Option<NaiveDateTime>,
}
