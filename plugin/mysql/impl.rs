use super::*;

impl Default for MySqlAutoCreation {
    fn default() -> Self {
        Self::new()
    }
}

impl MySqlAutoCreation {
    pub fn new() -> Self {
        Self {
            env: get_global_env_config(),
        }
    }

    async fn create_admin_connection(&self) -> Result<DatabaseConnection, AutoCreationError> {
        let admin_url: String = format!(
            "mysql://{}:{}@{}:{}",
            self.env.get_mysql_username(),
            self.env.get_mysql_password(),
            self.env.get_mysql_host(),
            self.env.get_mysql_port()
        );
        Database::connect(&admin_url).await.map_err(|error: DbErr| {
            let error_msg: String = error.to_string();
            if error_msg.contains("Access denied") || error_msg.contains("permission") {
                AutoCreationError::InsufficientPermissions(format!(
                    "Cannot connect to MySQL server for database creation: {error_msg}"
                ))
            } else if error_msg.contains("timeout") || error_msg.contains("Connection refused") {
                AutoCreationError::ConnectionFailed(format!(
                    "Cannot connect to MySQL server: {error_msg}"
                ))
            } else {
                AutoCreationError::DatabaseError(format!("MySQL connection error: {error_msg}"))
            }
        })
    }

    async fn database_exists(
        &self,
        connection: &DatabaseConnection,
    ) -> Result<bool, AutoCreationError> {
        let query: String = format!(
            "SELECT SCHEMA_NAME FROM INFORMATION_SCHEMA.SCHEMATA WHERE SCHEMA_NAME = '{}'",
            self.env.get_mysql_database()
        );
        let statement: Statement = Statement::from_string(DatabaseBackend::MySql, query);
        match connection.query_all(statement).await {
            Ok(results) => Ok(!results.is_empty()),
            Err(error) => Err(AutoCreationError::DatabaseError(format!(
                "Failed to check if database exists: {error}"
            ))),
        }
    }

    async fn create_database(
        &self,
        connection: &DatabaseConnection,
    ) -> Result<bool, AutoCreationError> {
        if self.database_exists(connection).await? {
            AutoCreationLogger::log_database_exists(
                self.env.get_mysql_database(),
                crate::database::PluginType::MySQL,
            )
            .await;
            return Ok(false);
        }
        let create_query: String = format!(
            "CREATE DATABASE IF NOT EXISTS `{}` CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci",
            self.env.get_mysql_database()
        );
        let statement: Statement = Statement::from_string(DatabaseBackend::MySql, create_query);
        match connection.execute(statement).await {
            Ok(_) => {
                AutoCreationLogger::log_database_created(
                    self.env.get_mysql_database(),
                    crate::database::PluginType::MySQL,
                )
                .await;
                Ok(true)
            }
            Err(error) => {
                let error_msg: String = error.to_string();
                if error_msg.contains("Access denied") || error_msg.contains("permission") {
                    Err(AutoCreationError::InsufficientPermissions(format!(
                        "Cannot create MySQL database '{}': {}",
                        self.env.get_mysql_database(),
                        error_msg
                    )))
                } else {
                    Err(AutoCreationError::DatabaseError(format!(
                        "Failed to create MySQL database '{}': {}",
                        self.env.get_mysql_database(),
                        error_msg
                    )))
                }
            }
        }
    }

    async fn create_target_connection(&self) -> Result<DatabaseConnection, AutoCreationError> {
        let db_url: String = format!(
            "mysql://{}:{}@{}:{}/{}",
            self.env.get_mysql_username(),
            self.env.get_mysql_password(),
            self.env.get_mysql_host(),
            self.env.get_mysql_port(),
            self.env.get_mysql_database()
        );
        Database::connect(&db_url).await.map_err(|error: DbErr| {
            AutoCreationError::ConnectionFailed(format!(
                "Cannot connect to MySQL database '{}': {}",
                self.env.get_mysql_database(),
                error
            ))
        })
    }

    async fn table_exists(
        &self,
        connection: &DatabaseConnection,
        table_name: &str,
    ) -> Result<bool, AutoCreationError> {
        let query: String = format!(
            "SELECT TABLE_NAME FROM INFORMATION_SCHEMA.TABLES WHERE TABLE_SCHEMA = '{}' AND TABLE_NAME = '{table_name}'",
            self.env.get_mysql_database()
        );
        let statement: Statement = Statement::from_string(DatabaseBackend::MySql, query);
        match connection.query_all(statement).await {
            Ok(results) => Ok(!results.is_empty()),
            Err(error) => Err(AutoCreationError::DatabaseError(format!(
                "Failed to check if table '{table_name}' exists: {error}"
            ))),
        }
    }

    async fn create_table(
        &self,
        connection: &DatabaseConnection,
        table: &crate::database::TableSchema,
    ) -> Result<(), AutoCreationError> {
        let statement: Statement =
            Statement::from_string(DatabaseBackend::MySql, table.sql.clone());
        match connection.execute(statement).await {
            Ok(_) => Ok(()),
            Err(error) => {
                let error_msg: String = error.to_string();
                if error_msg.contains("Access denied") || error_msg.contains("permission") {
                    Err(AutoCreationError::InsufficientPermissions(format!(
                        "Cannot create MySQL table '{}': {}",
                        table.name, error_msg
                    )))
                } else {
                    Err(AutoCreationError::SchemaError(format!(
                        "Failed to create MySQL table '{}': {}",
                        table.name, error_msg
                    )))
                }
            }
        }
    }

    async fn execute_sql(
        &self,
        connection: &DatabaseConnection,
        sql: &str,
    ) -> Result<(), AutoCreationError> {
        let statement: Statement = Statement::from_string(DatabaseBackend::MySql, sql.to_string());
        match connection.execute(statement).await {
            Ok(_) => Ok(()),
            Err(error) => Err(AutoCreationError::DatabaseError(format!(
                "Failed to execute SQL: {error}"
            ))),
        }
    }

    fn get_mysql_schema(&self) -> crate::database::DatabaseSchema {
        DatabaseSchema::new()
            .add_table(TableSchema::new(
                "record".to_string(),
                MYSQL_RECORD_SQL.to_string(),
            ))
            .add_table(TableSchema::new(
                "chat_history".to_string(),
                MYSQL_CHAT_HISTORY_SQL.to_string(),
            ))
            .add_table(TableSchema::new(
                "tracking_record".to_string(),
                MYSQL_TRACKING_RECORD_SQL.to_string(),
            ))
    }
}

impl DatabaseAutoCreation for MySqlAutoCreation {
    async fn create_database_if_not_exists(&self) -> Result<bool, AutoCreationError> {
        let admin_connection: DatabaseConnection = self.create_admin_connection().await?;
        let result: Result<bool, AutoCreationError> = self.create_database(&admin_connection).await;
        let _: Result<(), DbErr> = admin_connection.close().await;
        result
    }

    async fn create_tables_if_not_exist(&self) -> Result<Vec<String>, AutoCreationError> {
        let connection: DatabaseConnection = self.create_target_connection().await?;
        let schema: DatabaseSchema = self.get_mysql_schema();
        let mut created_tables: Vec<String> = Vec::new();
        for table in schema.ordered_tables() {
            if !self.table_exists(&connection, &table.name).await? {
                self.create_table(&connection, table).await?;
                created_tables.push(table.name.clone());
                AutoCreationLogger::log_table_created(
                    &table.name,
                    self.env.get_mysql_database(),
                    crate::database::PluginType::MySQL,
                )
                .await;
            } else {
                AutoCreationLogger::log_table_exists(
                    &table.name,
                    self.env.get_mysql_database(),
                    crate::database::PluginType::MySQL,
                )
                .await;
            }
        }
        for index_sql in &schema.indexes {
            if let Err(error) = self.execute_sql(&connection, index_sql).await {
                AutoCreationLogger::log_auto_creation_error(
                    &error,
                    "Index creation",
                    crate::database::PluginType::MySQL,
                    Some(self.env.get_mysql_database()),
                )
                .await;
            }
        }
        for constraint_sql in &schema.constraints {
            if let Err(error) = self.execute_sql(&connection, constraint_sql).await {
                AutoCreationLogger::log_auto_creation_error(
                    &error,
                    "Constraint creation",
                    crate::database::PluginType::MySQL,
                    Some(self.env.get_mysql_database()),
                )
                .await;
            }
        }
        let _: Result<(), DbErr> = connection.close().await;
        AutoCreationLogger::log_tables_created(
            &created_tables,
            self.env.get_mysql_database(),
            crate::database::PluginType::MySQL,
        )
        .await;
        Ok(created_tables)
    }

    async fn verify_connection(&self) -> Result<(), AutoCreationError> {
        let db_url: String = format!(
            "mysql://{}:{}@{}:{}/{}",
            self.env.get_mysql_username(),
            self.env.get_mysql_password(),
            self.env.get_mysql_host(),
            self.env.get_mysql_port(),
            self.env.get_mysql_database()
        );
        let connection: DatabaseConnection = Database::connect(&db_url).await.map_err(|error| {
            AutoCreationError::ConnectionFailed(format!(
                "Failed to verify MySQL connection: {error}"
            ))
        })?;

        let statement: Statement =
            Statement::from_string(DatabaseBackend::MySql, "SELECT 1".to_string());
        match connection.query_all(statement).await {
            Ok(_) => {
                let _: Result<(), DbErr> = connection.close().await;
                AutoCreationLogger::log_connection_verification(
                    crate::database::PluginType::MySQL,
                    self.env.get_mysql_database(),
                    true,
                    None,
                )
                .await;
                Ok(())
            }
            Err(error) => {
                let _: Result<(), DbErr> = connection.close().await;
                let error_msg: String = error.to_string();
                AutoCreationLogger::log_connection_verification(
                    crate::database::PluginType::MySQL,
                    self.env.get_mysql_database(),
                    false,
                    Some(&error_msg),
                )
                .await;
                Err(AutoCreationError::ConnectionFailed(format!(
                    "MySQL connection verification failed: {error_msg}"
                )))
            }
        }
    }
}
