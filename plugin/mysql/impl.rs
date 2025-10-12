use super::*;

impl MySqlAutoCreation {
    pub fn new() -> Self {
        Self {
            env: get_global_env_config(),
        }
    }

    async fn create_admin_connection(&self) -> Result<DatabaseConnection, AutoCreationError> {
        let admin_url: String = format!(
            "mysql://{}:{}@{}:{}",
            self.env.mysql_username,
            self.env.mysql_password,
            self.env.mysql_host,
            self.env.mysql_port
        );
        Database::connect(&admin_url)
            .await
            .map_err(|error: sea_orm::DbErr| {
                let error_msg: String = error.to_string();
                if error_msg.contains("Access denied") || error_msg.contains("permission") {
                    AutoCreationError::InsufficientPermissions(format!(
                        "Cannot connect to MySQL server for database creation: {error_msg}"
                    ))
                } else if error_msg.contains("timeout") || error_msg.contains("Connection refused")
                {
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
            self.env.mysql_database
        );
        let statement: Statement = Statement::from_string(DatabaseBackend::MySql, query);
        match connection.query_all(statement).await {
            Ok(results) => Ok(!results.is_empty()),
            Err(error) => Err(AutoCreationError::DatabaseError(format!(
                "Failed to check if database exists: {}",
                error
            ))),
        }
    }

    async fn create_database(
        &self,
        connection: &DatabaseConnection,
    ) -> Result<bool, AutoCreationError> {
        if self.database_exists(connection).await? {
            AutoCreationLogger::log_database_exists(
                &self.env.mysql_database,
                crate::database::PluginType::MySQL,
            )
            .await;
            return Ok(false);
        }
        let create_query: String = format!(
            "CREATE DATABASE IF NOT EXISTS `{}` CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci",
            self.env.mysql_database
        );
        let statement: Statement = Statement::from_string(DatabaseBackend::MySql, create_query);
        match connection.execute(statement).await {
            Ok(_) => {
                AutoCreationLogger::log_database_created(
                    &self.env.mysql_database,
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
                        self.env.mysql_database, error_msg
                    )))
                } else {
                    Err(AutoCreationError::DatabaseError(format!(
                        "Failed to create MySQL database '{}': {}",
                        self.env.mysql_database, error_msg
                    )))
                }
            }
        }
    }

    async fn create_target_connection(&self) -> Result<DatabaseConnection, AutoCreationError> {
        let db_url: String = format!(
            "mysql://{}:{}@{}:{}/{}",
            self.env.mysql_username,
            self.env.mysql_password,
            self.env.mysql_host,
            self.env.mysql_port,
            self.env.mysql_database
        );
        Database::connect(&db_url)
            .await
            .map_err(|error: sea_orm::DbErr| {
                AutoCreationError::ConnectionFailed(format!(
                    "Cannot connect to MySQL database '{}': {}",
                    self.env.mysql_database, error
                ))
            })
    }

    async fn table_exists(
        &self,
        connection: &DatabaseConnection,
        table_name: &str,
    ) -> Result<bool, AutoCreationError> {
        let query = format!(
            "SELECT TABLE_NAME FROM INFORMATION_SCHEMA.TABLES WHERE TABLE_SCHEMA = '{}' AND TABLE_NAME = '{}'",
            self.env.mysql_database, table_name
        );

        let statement = Statement::from_string(DatabaseBackend::MySql, query);

        match connection.query_all(statement).await {
            Ok(results) => Ok(!results.is_empty()),
            Err(error) => Err(AutoCreationError::DatabaseError(format!(
                "Failed to check if table '{}' exists: {}",
                table_name, error
            ))),
        }
    }

    async fn create_table(
        &self,
        connection: &DatabaseConnection,
        table: &crate::database::TableSchema,
    ) -> Result<(), AutoCreationError> {
        let statement = Statement::from_string(DatabaseBackend::MySql, table.sql.clone());

        match connection.execute(statement).await {
            Ok(_) => Ok(()),
            Err(error) => {
                let error_msg = error.to_string();
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
        let statement = Statement::from_string(DatabaseBackend::MySql, sql.to_string());

        match connection.execute(statement).await {
            Ok(_) => Ok(()),
            Err(error) => Err(AutoCreationError::DatabaseError(format!(
                "Failed to execute SQL: {}",
                error
            ))),
        }
    }

    fn get_mysql_schema(&self) -> crate::database::DatabaseSchema {
        DatabaseSchema::new()
            .add_table(
                TableSchema::new(
                    "hyperlane_config".to_string(),
                    r#"CREATE TABLE `hyperlane_config` (
                        `id` bigint unsigned NOT NULL AUTO_INCREMENT,
                        `config_key` varchar(255) NOT NULL,
                        `config_value` text,
                        `created_at` timestamp NULL DEFAULT CURRENT_TIMESTAMP,
                        `updated_at` timestamp NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
                        PRIMARY KEY (`id`),
                        UNIQUE KEY `uk_config_key` (`config_key`)
                    ) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci"#.to_string(),
                )
            )
            .add_table(
                TableSchema::new(
                    "hyperlane_logs".to_string(),
                    r#"CREATE TABLE `hyperlane_logs` (
                        `id` bigint unsigned NOT NULL AUTO_INCREMENT,
                        `level` varchar(50) NOT NULL,
                        `message` text NOT NULL,
                        `context` json DEFAULT NULL,
                        `created_at` timestamp NULL DEFAULT CURRENT_TIMESTAMP,
                        PRIMARY KEY (`id`),
                        KEY `idx_level` (`level`),
                        KEY `idx_created_at` (`created_at`)
                    ) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci"#.to_string(),
                )
            )
            .add_table(
                TableSchema::new(
                    "hyperlane_sessions".to_string(),
                    r#"CREATE TABLE `hyperlane_sessions` (
                        `id` varchar(255) NOT NULL,
                        `data` text,
                        `expires_at` timestamp NULL DEFAULT NULL,
                        `created_at` timestamp NULL DEFAULT CURRENT_TIMESTAMP,
                        `updated_at` timestamp NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
                        PRIMARY KEY (`id`),
                        KEY `idx_expires_at` (`expires_at`)
                    ) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci"#.to_string(),
                )
            )
            .add_index("CREATE INDEX `idx_hyperlane_logs_level_created` ON `hyperlane_logs` (`level`, `created_at`)".to_string())
    }
}

impl DatabaseAutoCreation for MySqlAutoCreation {
    fn create_database_if_not_exists(
        &self,
    ) -> impl std::future::Future<Output = Result<bool, AutoCreationError>> + Send {
        async move {
            let start_time = Instant::now();
            let timeout = Duration::from_secs(self.env.auto_creation_timeout_seconds);

            let admin_connection = self.create_admin_connection().await?;

            if start_time.elapsed() > timeout {
                return Err(AutoCreationError::Timeout(format!(
                    "MySQL database creation timed out after {} seconds",
                    self.env.auto_creation_timeout_seconds
                )));
            }

            let result = self.create_database(&admin_connection).await;

            let _ = admin_connection.close().await;

            result
        }
    }

    fn create_tables_if_not_exist(
        &self,
    ) -> impl std::future::Future<Output = Result<Vec<String>, AutoCreationError>> + Send {
        async move {
            let connection = self.create_target_connection().await?;
            let schema = self.get_mysql_schema();
            let mut created_tables = Vec::new();

            for table in schema.ordered_tables() {
                if !self.table_exists(&connection, &table.name).await? {
                    self.create_table(&connection, table).await?;
                    created_tables.push(table.name.clone());
                    AutoCreationLogger::log_table_created(
                        &table.name,
                        &self.env.mysql_database,
                        crate::database::PluginType::MySQL,
                    )
                    .await;
                } else {
                    AutoCreationLogger::log_table_exists(
                        &table.name,
                        &self.env.mysql_database,
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
                        Some(&self.env.mysql_database),
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
                        Some(&self.env.mysql_database),
                    )
                    .await;
                }
            }

            let _ = connection.close().await;
            AutoCreationLogger::log_tables_created(
                &created_tables,
                &self.env.mysql_database,
                crate::database::PluginType::MySQL,
            )
            .await;

            Ok(created_tables)
        }
    }

    fn verify_connection(
        &self,
    ) -> impl std::future::Future<Output = Result<(), AutoCreationError>> + Send {
        async move {
            let db_url = format!(
                "mysql://{}:{}@{}:{}/{}",
                self.env.mysql_username,
                self.env.mysql_password,
                self.env.mysql_host,
                self.env.mysql_port,
                self.env.mysql_database
            );

            let connection = Database::connect(&db_url).await.map_err(|error| {
                AutoCreationError::ConnectionFailed(format!(
                    "Failed to verify MySQL connection: {}",
                    error
                ))
            })?;

            let statement = Statement::from_string(DatabaseBackend::MySql, "SELECT 1".to_string());

            match connection.query_all(statement).await {
                Ok(_) => {
                    let _ = connection.close().await;
                    AutoCreationLogger::log_connection_verification(
                        crate::database::PluginType::MySQL,
                        &self.env.mysql_database,
                        true,
                        None,
                    )
                    .await;
                    Ok(())
                }
                Err(error) => {
                    let _ = connection.close().await;
                    let error_msg = error.to_string();
                    AutoCreationLogger::log_connection_verification(
                        crate::database::PluginType::MySQL,
                        &self.env.mysql_database,
                        false,
                        Some(&error_msg),
                    )
                    .await;
                    Err(AutoCreationError::ConnectionFailed(format!(
                        "MySQL connection verification failed: {}",
                        error_msg
                    )))
                }
            }
        }
    }
}
