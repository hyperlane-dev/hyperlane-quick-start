use super::*;

/// Implementation of methods for `ChatHistoryRepository`.
impl ChatHistoryRepository {
    /// Inserts a new chat message record into the database.
    ///
    /// # Arguments
    ///
    /// - `&str`: The session identifier.
    /// - `&str`: The sender name.
    /// - `&str`: The sender type (e.g., "user" or "system").
    /// - `&str`: The message type.
    /// - `&str`: The message content.
    ///
    /// # Returns
    ///
    /// - `Result<(), String>`: Ok on success, or an error message on failure.
    #[instrument_trace]
    pub async fn insert_message(
        session_id: &str,
        sender_name: &str,
        sender_type: &str,
        message_type: &str,
        content: &str,
    ) -> Result<(), String> {
        let db: DatabaseConnection =
            PostgreSqlPlugin::connection_db(DEFAULT_POSTGRESQL_INSTANCE_NAME, None).await?;
        let active_model: ActiveModel = ActiveModel {
            id: ActiveValue::NotSet,
            session_id: ActiveValue::Set(session_id.to_string()),
            sender_name: ActiveValue::Set(sender_name.to_string()),
            sender_type: ActiveValue::Set(sender_type.to_string()),
            message_type: ActiveValue::Set(message_type.to_string()),
            content: ActiveValue::Set(content.to_string()),
            created_at: ActiveValue::NotSet,
        };
        active_model
            .insert(&db)
            .await
            .map_err(|error: DbErr| format!("PostgreSQL insert error {error}"))?;
        Ok(())
    }

    /// Retrieves chat history messages with an optional cursor-based filter.
    ///
    /// Messages are fetched in descending ID order and then reversed for chronological display.
    ///
    /// # Arguments
    ///
    /// - `Option<i64>`: An optional message ID to fetch messages before (cursor).
    /// - `u64`: The maximum number of messages to return.
    ///
    /// # Returns
    ///
    /// - `Result<Vec<ChatHistory>, String>`: The list of chat history messages.
    #[instrument_trace]
    pub async fn get_history(
        before_id: Option<i64>,
        limit: u64,
    ) -> Result<Vec<ChatHistory>, String> {
        let db: DatabaseConnection =
            PostgreSqlPlugin::connection_db(DEFAULT_POSTGRESQL_INSTANCE_NAME, None).await?;
        let mut query: Select<Entity> = Entity::find();
        if let Some(id) = before_id {
            query = query.filter(Column::Id.lt(id));
        }
        let mut records: Vec<Model> = query
            .order_by_desc(Column::Id)
            .limit(limit)
            .all(&db)
            .await
            .map_err(|error: DbErr| format!("Failed to query from PostgreSQL {error}"))?;
        records.reverse();
        Ok(records.into_iter().map(Into::into).collect())
    }

    /// Counts the total number of chat messages in the database.
    ///
    /// # Returns
    ///
    /// - `Result<i64, String>`: The total message count.
    #[instrument_trace]
    pub async fn count_messages() -> Result<i64, String> {
        let db: DatabaseConnection =
            PostgreSqlPlugin::connection_db(DEFAULT_POSTGRESQL_INSTANCE_NAME, None).await?;
        let count: u64 = Entity::find()
            .count(&db)
            .await
            .map_err(|error: DbErr| format!("Failed to count from PostgreSQL {error}"))?;
        Ok(count as i64)
    }
}
