use super::*;

impl ActiveModelBehavior for ActiveModel {}

impl ChatHistoryMapper {
    #[instrument_trace]
    pub async fn insert_message(
        session_id: &str,
        sender_name: &str,
        sender_type: &str,
        message_type: &str,
        content: &str,
    ) -> Result<(), String> {
        let db: DatabaseConnection =
            get_postgresql_connection(DEFAULT_POSTGRESQL_INSTANCE_NAME).await?;
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
            .map_err(|error| format!("PostgreSQL insert error{COLON_SPACE}{error}"))?;
        Ok(())
    }

    #[instrument_trace]
    pub async fn get_history(
        before_id: Option<i64>,
        limit: i64,
    ) -> Result<Vec<ChatHistory>, String> {
        Self::get_history_from_postgresql(before_id, limit).await
    }

    #[instrument_trace]
    async fn get_history_from_postgresql(
        before_id: Option<i64>,
        limit: i64,
    ) -> Result<Vec<ChatHistory>, String> {
        let db: DatabaseConnection =
            get_postgresql_connection(DEFAULT_POSTGRESQL_INSTANCE_NAME).await?;
        let mut query: Select<Entity> = Entity::find();
        if let Some(id) = before_id {
            query = query.filter(Column::Id.lt(id));
        }
        let mut records: Vec<Model> = query
            .order_by_desc(Column::Id)
            .limit(limit as u64)
            .all(&db)
            .await
            .map_err(|error: DbErr| {
                format!("Failed to query from PostgreSQL{COLON_SPACE}{error}")
            })?;
        records.reverse();
        Ok(records
            .into_iter()
            .map(|r: Model| ChatHistory {
                id: r.id,
                session_id: r.session_id,
                sender_name: r.sender_name,
                sender_type: r.sender_type,
                message_type: r.message_type,
                content: r.content,
                created_at: r
                    .created_at
                    .map(|dt| dt.format("%Y-%m-%d %H:%M:%S").to_string())
                    .unwrap_or_default(),
            })
            .collect())
    }

    #[instrument_trace]
    pub async fn count_messages() -> Result<i64, String> {
        let db: DatabaseConnection =
            get_postgresql_connection(DEFAULT_POSTGRESQL_INSTANCE_NAME).await?;
        let count: u64 = Entity::find().count(&db).await.map_err(|error: DbErr| {
            format!("Failed to count from PostgreSQL{COLON_SPACE}{error}")
        })?;
        Ok(count as i64)
    }
}
