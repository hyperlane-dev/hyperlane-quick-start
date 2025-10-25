use super::*;

impl ChatHistoryMapper {
    pub async fn insert_message(
        session_id: &str,
        sender_name: &str,
        sender_type: &str,
        message_type: &str,
        content: &str,
    ) -> Result<(), String> {
        let connection: DatabaseConnection = get_mysql_connection()
            .await
            .map_err(|e| format!("Failed to get database connection: {e}"))?;
        let statement: Statement = Statement::from_sql_and_values(
            DatabaseBackend::MySql,
            "INSERT INTO chat_history (session_id, sender_name, sender_type, message_type, content, created_at) VALUES (?, ?, ?, ?, ?, NOW())",
            vec![
                session_id.into(),
                sender_name.into(),
                sender_type.into(),
                message_type.into(),
                content.into(),
            ],
        );
        connection
            .execute(statement)
            .await
            .map_err(|e| {
                let error_msg = e.to_string();
                if error_msg.contains("doesn't exist") || error_msg.contains("Table") && error_msg.contains("not found") {
                    format!("Table 'chat_history' does not exist. Please ensure database initialization completed successfully: {error_msg}")
                } else {
                    format!("Failed to insert chat message: {error_msg}")
                }
            })?;
        Ok(())
    }

    pub async fn get_history(
        session_id: &str,
        offset: i64,
        limit: i64,
    ) -> Result<Vec<ChatHistory>, String> {
        let connection: DatabaseConnection = get_mysql_connection()
            .await
            .map_err(|e| format!("Failed to get database connection: {e}"))?;
        let statement: Statement = Statement::from_sql_and_values(
            DatabaseBackend::MySql,
            "SELECT id, session_id, sender_name, sender_type, message_type, content, DATE_FORMAT(created_at, '%Y-%m-%d %H:%i:%s') as created_at FROM chat_history WHERE session_id = ? ORDER BY id DESC LIMIT ? OFFSET ?",
            vec![session_id.into(), limit.into(), offset.into()],
        );
        let results: Vec<QueryResult> = connection
            .query_all(statement)
            .await
            .map_err(|e| {
                let error_msg = e.to_string();
                if error_msg.contains("doesn't exist") || error_msg.contains("Table") && error_msg.contains("not found") {
                    format!("Table 'chat_history' does not exist. Please ensure database initialization completed successfully: {error_msg}")
                } else {
                    format!("Failed to query chat history: {error_msg}")
                }
            })?;

        let mut messages: Vec<ChatHistory> = Vec::new();
        for row in results {
            let id: i64 = row.try_get("", "id").unwrap_or(0);
            let session_id: String = row.try_get("", "session_id").unwrap_or_default();
            let sender_name: String = row.try_get("", "sender_name").unwrap_or_default();
            let sender_type: String = row.try_get("", "sender_type").unwrap_or_default();
            let message_type: String = row.try_get("", "message_type").unwrap_or_default();
            let content: String = row.try_get("", "content").unwrap_or_default();
            let created_at: String = row.try_get("", "created_at").unwrap_or_default();
            messages.push(ChatHistory {
                id,
                session_id,
                sender_name,
                sender_type,
                message_type,
                content,
                created_at,
            });
        }

        Ok(messages)
    }

    pub async fn count_messages(session_id: &str) -> Result<i64, String> {
        let connection: DatabaseConnection = get_mysql_connection()
            .await
            .map_err(|e| format!("Failed to get database connection: {e}"))?;
        let statement: Statement = Statement::from_sql_and_values(
            DatabaseBackend::MySql,
            "SELECT COUNT(*) as total FROM chat_history WHERE session_id = ?",
            vec![session_id.into()],
        );
        let result: Option<QueryResult> = connection
            .query_one(statement)
            .await
            .map_err(|e| {
                let error_msg = e.to_string();
                if error_msg.contains("doesn't exist") || error_msg.contains("Table") && error_msg.contains("not found") {
                    format!("Table 'chat_history' does not exist. Please ensure database initialization completed successfully: {error_msg}")
                } else {
                    format!("Failed to count messages: {error_msg}")
                }
            })?;

        if let Some(row) = result {
            let total: i64 = row.try_get("", "total").unwrap_or(0);
            Ok(total)
        } else {
            Ok(0)
        }
    }
}
