use super::*;

impl ChatHistoryMapper {
    pub async fn insert_message(
        session_id: &str,
        sender_name: &str,
        sender_type: &str,
        message_type: &str,
        content: &str,
    ) -> Result<(), String> {
        let connection: sea_orm::DatabaseConnection =
            hyperlane_plugin::mysql::get_mysql_connection()
                .await
                .map_err(|e| format!("Failed to get database connection: {e}"))?;

        let sql: String = format!(
            "INSERT INTO chat_history (session_id, sender_name, sender_type, message_type, content, created_at) VALUES ('{}', '{}', '{}', '{}', '{}', NOW())",
            session_id.replace("'", "''"),
            sender_name.replace("'", "''"),
            sender_type.replace("'", "''"),
            message_type.replace("'", "''"),
            content.replace("'", "''")
        );

        let statement: sea_orm::Statement =
            sea_orm::Statement::from_string(sea_orm::DatabaseBackend::MySql, sql);
        connection
            .execute(statement)
            .await
            .map_err(|e| format!("Failed to insert chat message: {e}"))?;

        Ok(())
    }

    pub async fn get_history(
        session_id: &str,
        offset: i64,
        limit: i64,
    ) -> Result<Vec<ChatHistory>, String> {
        let connection: sea_orm::DatabaseConnection =
            hyperlane_plugin::mysql::get_mysql_connection()
                .await
                .map_err(|e| format!("Failed to get database connection: {e}"))?;

        let sql: String = format!(
            "SELECT id, session_id, sender_name, sender_type, message_type, content, DATE_FORMAT(created_at, '%Y-%m-%d %H:%i:%s') as created_at FROM chat_history WHERE session_id = '{}' ORDER BY id DESC LIMIT {} OFFSET {}",
            session_id.replace("'", "''"),
            limit,
            offset
        );

        let statement: sea_orm::Statement =
            sea_orm::Statement::from_string(sea_orm::DatabaseBackend::MySql, sql);
        let results: Vec<sea_orm::QueryResult> = connection
            .query_all(statement)
            .await
            .map_err(|e| format!("Failed to query chat history: {e}"))?;

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
        let connection: sea_orm::DatabaseConnection =
            hyperlane_plugin::mysql::get_mysql_connection()
                .await
                .map_err(|e| format!("Failed to get database connection: {e}"))?;

        let sql: String = format!(
            "SELECT COUNT(*) as total FROM chat_history WHERE session_id = '{}'",
            session_id.replace("'", "''")
        );

        let statement: sea_orm::Statement =
            sea_orm::Statement::from_string(sea_orm::DatabaseBackend::MySql, sql);
        let result: Option<sea_orm::QueryResult> = connection
            .query_one(statement)
            .await
            .map_err(|e| format!("Failed to count messages: {e}"))?;

        if let Some(row) = result {
            let total: i64 = row.try_get("", "total").unwrap_or(0);
            Ok(total)
        } else {
            Ok(0)
        }
    }
}
