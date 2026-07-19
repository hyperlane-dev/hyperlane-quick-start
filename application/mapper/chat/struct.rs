use super::*;

/// Marker struct for the chat history SeaORM entity mapping.
pub struct ChatHistoryMapper;

/// SeaORM entity model for the `chat_history` table, representing a persisted chat message record.
#[derive(Clone, Data, Debug, DeriveEntityModel, PartialEq)]
#[sea_orm(table_name = "chat_history")]
pub struct Model {
    /// Unique primary key identifier for the chat message.
    #[sea_orm(primary_key, auto_increment = true)]
    #[get(type(copy))]
    pub(super) id: i64,
    /// The session identifier grouping related messages together.
    pub(super) session_id: String,
    /// The display name of the message sender.
    pub(super) sender_name: String,
    /// The type of the sender (e.g., "user", "assistant").
    pub(super) sender_type: String,
    /// The type of the message (e.g., "text", "image").
    pub(super) message_type: String,
    /// The textual content of the chat message.
    pub(super) content: String,
    /// The timestamp when the message was created.
    pub(super) created_at: Option<NaiveDateTime>,
}
