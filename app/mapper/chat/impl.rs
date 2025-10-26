use super::*;

impl ChatHistoryMapper {
    pub async fn insert_message(
        session_id: &str,
        sender_name: &str,
        sender_type: &str,
        message_type: &str,
        content: &str,
    ) -> Result<(), String> {
        let env: &EnvConfig = get_global_env_config();
        let mysql_result: Result<(), String> = if *env.get_enable_mysql() {
            Self::insert_to_mysql(session_id, sender_name, sender_type, message_type, content).await
        } else {
            Ok(())
        };
        let postgresql_result: Result<(), String> = if *env.get_enable_postgresql() {
            Self::insert_to_postgresql(session_id, sender_name, sender_type, message_type, content)
                .await
        } else {
            Ok(())
        };
        match (mysql_result, postgresql_result) {
            (Err(mysql_err), Err(pg_err)) => {
                log_error(&format!(
                    "Both databases failed - MySQL: {mysql_err}, PostgreSQL: {pg_err}"
                ))
                .await;
                Err(format!(
                    "Both databases failed - MySQL: {mysql_err}, PostgreSQL: {pg_err}"
                ))
            }
            (Err(mysql_err), Ok(_)) => {
                log_error(&format!("MySQL insert failed: {mysql_err}")).await;
                Ok(())
            }
            (Ok(_), Err(pg_err)) => {
                log_error(&format!("PostgreSQL insert failed: {pg_err}")).await;
                Ok(())
            }
            (Ok(_), Ok(_)) => Ok(()),
        }
    }

    async fn insert_to_mysql(
        session_id: &str,
        sender_name: &str,
        sender_type: &str,
        message_type: &str,
        content: &str,
    ) -> Result<(), String> {
        let env: &EnvConfig = get_global_env_config();
        let db_url: String = format!(
            "mysql://{}:{}@{}:{}/{}",
            env.get_mysql_username(),
            env.get_mysql_password(),
            env.get_mysql_host(),
            env.get_mysql_port(),
            env.get_mysql_database()
        );
        let db: DatabaseConnection = Database::connect(&db_url)
            .await
            .map_err(|error| format!("Failed to connect to MySQL: {error}"))?;
        let active_model = ActiveModel {
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
            .map_err(|error| format!("Failed to insert to MySQL: {error}"))?;
        Ok(())
    }

    async fn insert_to_postgresql(
        session_id: &str,
        sender_name: &str,
        sender_type: &str,
        message_type: &str,
        content: &str,
    ) -> Result<(), String> {
        let env: &EnvConfig = get_global_env_config();
        let db_url = format!(
            "postgres://{}:{}@{}:{}/{}",
            env.get_postgresql_username(),
            env.get_postgresql_password(),
            env.get_postgresql_host(),
            env.get_postgresql_port(),
            env.get_postgresql_database()
        );
        let db: DatabaseConnection = Database::connect(&db_url)
            .await
            .map_err(|error| format!("Failed to connect to PostgreSQL: {error}"))?;
        let active_model = ActiveModel {
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
            .map_err(|error| format!("PostgreSQL insert error: {error}"))?;
        Ok(())
    }

    pub async fn get_history(
        session_id: &str,
        offset: i64,
        limit: i64,
    ) -> Result<Vec<ChatHistory>, String> {
        match Self::get_history_from_mysql(session_id, offset, limit).await {
            Ok(history) => Ok(history),
            Err(mysql_err) => {
                log_error(&format!(
                    "MySQL query failed, trying PostgreSQL: {mysql_err}"
                ))
                .await;
                Self::get_history_from_postgresql(session_id, offset, limit).await
            }
        }
    }

    async fn get_history_from_mysql(
        session_id: &str,
        offset: i64,
        limit: i64,
    ) -> Result<Vec<ChatHistory>, String> {
        let db: DatabaseConnection = get_mysql_connection().await?;
        let records: Vec<Model> = Entity::find()
            .filter(Column::SessionId.eq(session_id))
            .order_by_asc(Column::Id)
            .offset(offset as u64)
            .limit(limit as u64)
            .all(&db)
            .await
            .map_err(|error: DbErr| format!("Failed to query from MySQL: {error}"))?;
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

    async fn get_history_from_postgresql(
        session_id: &str,
        offset: i64,
        limit: i64,
    ) -> Result<Vec<ChatHistory>, String> {
        let db: DatabaseConnection = get_postgresql_connection().await?;
        let records: Vec<Model> = Entity::find()
            .filter(Column::SessionId.eq(session_id))
            .order_by_asc(Column::Id)
            .offset(offset as u64)
            .limit(limit as u64)
            .all(&db)
            .await
            .map_err(|error: DbErr| format!("Failed to query from PostgreSQL: {error}"))?;
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

    pub async fn count_messages(session_id: &str) -> Result<i64, String> {
        match Self::count_messages_from_mysql(session_id).await {
            Ok(count) => Ok(count),
            Err(mysql_err) => {
                log_error(&format!(
                    "MySQL count failed, trying PostgreSQL: {mysql_err}"
                ))
                .await;
                Self::count_messages_from_postgresql(session_id).await
            }
        }
    }

    async fn count_messages_from_mysql(session_id: &str) -> Result<i64, String> {
        let db: DatabaseConnection = get_mysql_connection().await?;
        let count: u64 = Entity::find()
            .filter(Column::SessionId.eq(session_id))
            .count(&db)
            .await
            .map_err(|error: DbErr| format!("Failed to count from MySQL: {error}"))?;
        Ok(count as i64)
    }

    async fn count_messages_from_postgresql(session_id: &str) -> Result<i64, String> {
        let db: DatabaseConnection = get_postgresql_connection().await?;
        let count: u64 = Entity::find()
            .filter(Column::SessionId.eq(session_id))
            .count(&db)
            .await
            .map_err(|error: DbErr| format!("Failed to count from PostgreSQL: {error}"))?;
        Ok(count as i64)
    }
}
