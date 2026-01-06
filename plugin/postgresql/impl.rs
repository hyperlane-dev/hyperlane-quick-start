use super::*;

impl Default for PostgreSqlAutoCreation {
    fn default() -> Self {
        Self::new()
    }
}

impl PostgreSqlAutoCreation {
    pub fn new() -> Self {
        Self {
            env: get_global_env_config(),
        }
    }

    async fn create_admin_connection(&self) -> Result<DatabaseConnection, AutoCreationError> {
        let admin_url: String = format!(
            "postgres://{}:{}@{}:{}/postgres",
            self.env.get_postgresql_username(),
            self.env.get_postgresql_password(),
            self.env.get_postgresql_host(),
            self.env.get_postgresql_port()
        );

        Database::connect(&admin_url).await.map_err(|error: DbErr| {
            let error_msg: String = error.to_string();
            if error_msg.contains("authentication failed") || error_msg.contains("permission") {
                AutoCreationError::InsufficientPermissions(format!(
                    "Cannot connect to PostgreSQL server for database creation: {error_msg}"
                ))
            } else if error_msg.contains("timeout") || error_msg.contains("Connection refused") {
                AutoCreationError::ConnectionFailed(format!(
                    "Cannot connect to PostgreSQL server: {error_msg}"
                ))
            } else {
                AutoCreationError::DatabaseError(format!(
                    "PostgreSQL connection error: {error_msg}"
                ))
            }
        })
    }

    async fn create_target_connection(&self) -> Result<DatabaseConnection, AutoCreationError> {
        let db_url: String = format!(
            "postgres://{}:{}@{}:{}/{}",
            self.env.get_postgresql_username(),
            self.env.get_postgresql_password(),
            self.env.get_postgresql_host(),
            self.env.get_postgresql_port(),
            self.env.get_postgresql_database()
        );
        Database::connect(&db_url).await.map_err(|error: DbErr| {
            AutoCreationError::ConnectionFailed(format!(
                "Cannot connect to PostgreSQL database '{}': {error}",
                self.env.get_postgresql_database(),
            ))
        })
    }

    async fn database_exists(
        &self,
        connection: &DatabaseConnection,
    ) -> Result<bool, AutoCreationError> {
        let query: String = format!(
            "SELECT 1 FROM pg_database WHERE datname = '{}'",
            self.env.get_postgresql_database()
        );
        let statement: Statement = Statement::from_string(DatabaseBackend::Postgres, query);
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
                self.env.get_postgresql_database(),
                database::PluginType::PostgreSQL,
            )
            .await;
            return Ok(false);
        }
        let create_query: String = format!(
            "CREATE DATABASE \"{}\" WITH ENCODING='UTF8' LC_COLLATE='en_US.UTF-8' LC_CTYPE='en_US.UTF-8'",
            self.env.get_postgresql_database()
        );
        let statement: Statement = Statement::from_string(DatabaseBackend::Postgres, create_query);
        match connection.execute(statement).await {
            Ok(_) => {
                AutoCreationLogger::log_database_created(
                    self.env.get_postgresql_database(),
                    database::PluginType::PostgreSQL,
                )
                .await;
                Ok(true)
            }
            Err(error) => {
                let error_msg: String = error.to_string();
                if error_msg.contains("permission denied") || error_msg.contains("must be owner") {
                    Err(AutoCreationError::InsufficientPermissions(format!(
                        "Cannot create PostgreSQL database '{}': {}",
                        self.env.get_postgresql_database(),
                        error_msg
                    )))
                } else if error_msg.contains("already exists") {
                    AutoCreationLogger::log_database_exists(
                        self.env.get_postgresql_database(),
                        database::PluginType::PostgreSQL,
                    )
                    .await;
                    Ok(false)
                } else {
                    Err(AutoCreationError::DatabaseError(format!(
                        "Failed to create PostgreSQL database '{}': {}",
                        self.env.get_postgresql_database(),
                        error_msg
                    )))
                }
            }
        }
    }

    async fn table_exists(
        &self,
        connection: &DatabaseConnection,
        table_name: &str,
    ) -> Result<bool, AutoCreationError> {
        let query: String = format!(
            "SELECT 1 FROM information_schema.tables WHERE table_schema = 'public' AND table_name = '{table_name}'"
        );
        let statement: Statement = Statement::from_string(DatabaseBackend::Postgres, query);
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
        table: &database::TableSchema,
    ) -> Result<(), AutoCreationError> {
        let statement: Statement =
            Statement::from_string(DatabaseBackend::Postgres, table.sql.clone());
        match connection.execute(statement).await {
            Ok(_) => Ok(()),
            Err(error) => {
                let error_msg: String = error.to_string();
                if error_msg.contains("permission denied") {
                    Err(AutoCreationError::InsufficientPermissions(format!(
                        "Cannot create PostgreSQL table '{}': {}",
                        table.name, error_msg
                    )))
                } else {
                    Err(AutoCreationError::SchemaError(format!(
                        "Failed to create PostgreSQL table '{}': {}",
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
        let statement: Statement =
            Statement::from_string(DatabaseBackend::Postgres, sql.to_string());
        match connection.execute(statement).await {
            Ok(_) => Ok(()),
            Err(error) => Err(AutoCreationError::DatabaseError(format!(
                "Failed to execute SQL: {error}"
            ))),
        }
    }

    fn get_postgresql_schema(&self) -> DatabaseSchema {
        let indexes: Vec<String> = POSTGRESQL_INDEXES_SQL
            .lines()
            .filter(|line| !line.trim().is_empty())
            .map(|line| line.trim().to_string())
            .collect();
        let mut schema: DatabaseSchema = DatabaseSchema::new()
            .add_table(TableSchema::new(
                "record".to_string(),
                POSTGRESQL_RECORD_SQL.to_string(),
            ))
            .add_table(TableSchema::new(
                "chat_history".to_string(),
                POSTGRESQL_CHAT_HISTORY_SQL.to_string(),
            ))
            .add_table(TableSchema::new(
                "tracking_record".to_string(),
                POSTGRESQL_TRACKING_RECORD_SQL.to_string(),
            ))
            .add_table(TableSchema::new(
                "shortlink".to_string(),
                POSTGRESQL_SHORTLINK_SQL.to_string(),
            ));
        for index in indexes {
            schema = schema.add_index(index);
        }
        schema
    }
}

impl DatabaseAutoCreation for PostgreSqlAutoCreation {
    async fn create_database_if_not_exists(&self) -> Result<bool, AutoCreationError> {
        let admin_connection: DatabaseConnection = self.create_admin_connection().await?;
        let result: Result<bool, AutoCreationError> = self.create_database(&admin_connection).await;
        let _: Result<(), DbErr> = admin_connection.close().await;
        result
    }

    async fn create_tables_if_not_exist(&self) -> Result<Vec<String>, AutoCreationError> {
        let connection: DatabaseConnection = self.create_target_connection().await?;
        let schema: DatabaseSchema = self.get_postgresql_schema();
        let mut created_tables: Vec<String> = Vec::new();
        for table in schema.ordered_tables() {
            if !self.table_exists(&connection, &table.name).await? {
                self.create_table(&connection, table).await?;
                created_tables.push(table.name.clone());
                AutoCreationLogger::log_table_created(
                    &table.name,
                    self.env.get_postgresql_database(),
                    database::PluginType::PostgreSQL,
                )
                .await;
            } else {
                AutoCreationLogger::log_table_exists(
                    &table.name,
                    self.env.get_postgresql_database(),
                    database::PluginType::PostgreSQL,
                )
                .await;
            }
        }
        for index_sql in &schema.indexes {
            if let Err(error) = self.execute_sql(&connection, index_sql).await {
                AutoCreationLogger::log_auto_creation_error(
                    &error,
                    "Index creation",
                    database::PluginType::PostgreSQL,
                    Some(self.env.get_postgresql_database()),
                )
                .await;
            }
        }
        for constraint_sql in &schema.constraints {
            if let Err(error) = self.execute_sql(&connection, constraint_sql).await {
                AutoCreationLogger::log_auto_creation_error(
                    &error,
                    "Constraint creation",
                    database::PluginType::PostgreSQL,
                    Some(self.env.get_postgresql_database()),
                )
                .await;
            }
        }
        let _: Result<(), DbErr> = connection.close().await;
        AutoCreationLogger::log_tables_created(
            &created_tables,
            self.env.get_postgresql_database(),
            database::PluginType::PostgreSQL,
        )
        .await;

        Ok(created_tables)
    }

    async fn verify_connection(&self) -> Result<(), AutoCreationError> {
        let connection: DatabaseConnection = self.create_target_connection().await?;
        let statement: Statement =
            Statement::from_string(DatabaseBackend::Postgres, "SELECT 1".to_string());
        match connection.query_all(statement).await {
            Ok(_) => {
                let _: Result<(), DbErr> = connection.close().await;
                AutoCreationLogger::log_connection_verification(
                    database::PluginType::PostgreSQL,
                    self.env.get_postgresql_database(),
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
                    database::PluginType::PostgreSQL,
                    self.env.get_postgresql_database(),
                    false,
                    Some(&error_msg),
                )
                .await;
                Err(AutoCreationError::ConnectionFailed(format!(
                    "PostgreSQL connection verification failed: {error_msg}"
                )))
            }
        }
    }
}
