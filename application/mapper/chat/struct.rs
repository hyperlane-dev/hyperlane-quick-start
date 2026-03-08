use super::*;

pub struct ChatHistoryMapper;

#[derive(Clone, Data, Debug, DeriveEntityModel, PartialEq)]
#[sea_orm(table_name = "chat_history")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    #[get(type(copy), pub(crate))]
    pub(super) id: i64,
    #[get(pub(crate))]
    pub(super) session_id: String,
    #[get(pub(crate))]
    pub(super) sender_name: String,
    #[get(pub(crate))]
    pub(super) sender_type: String,
    #[get(pub(crate))]
    pub(super) message_type: String,
    #[get(pub(crate))]
    pub(super) content: String,
    #[get(pub(crate))]
    pub(super) created_at: Option<NaiveDateTime>,
}
