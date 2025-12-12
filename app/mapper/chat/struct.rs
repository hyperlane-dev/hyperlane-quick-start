use super::*;

pub struct ChatHistoryMapper;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "chat_history")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    pub id: i64,
    pub session_id: String,
    pub sender_name: String,
    pub sender_type: String,
    pub message_type: String,
    pub content: String,
    pub created_at: Option<chrono::NaiveDateTime>,
}
