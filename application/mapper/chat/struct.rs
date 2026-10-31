use super::*;

/// Marker struct for the chat history SeaORM entity mapping.
pub struct ChatHistoryMapper;

/// SeaORM entity model for the `chat_history` table, representing a persisted chat message record.
#[derive(Clone, Data, Debug, DeriveEntityModel, PartialEq)]
#[sea_orm(table_name = "chat_history")]
pub struct Model {
    /// Unique primary key identifier for the chat message.
    #[sea_orm(primary_key, auto_increment = true)]
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) id: i64,
    /// The session identifier grouping related messages together.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) session_id: String,
    /// The display name of the message sender.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) sender_name: String,
    /// The type of the sender (e.g., "user", "assistant").
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) sender_type: String,
    /// The type of the message (e.g., "text", "image").
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) message_type: String,
    /// The textual content of the chat message.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) content: String,
    /// The timestamp when the message was created.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) created_at: Option<NaiveDateTime>,
}
