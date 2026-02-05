use super::*;

impl Default for PostgreSqlAutoCreation {
    #[instrument_trace]
    fn default() -> Self {
        let env: &'static EnvConfig = get_global_env_config();
        if let Some(instance) = env.get_default_postgresql_instance() {
            Self::new(instance.clone())
        } else {
            let default_instance: PostgreSqlInstanceConfig = PostgreSqlInstanceConfig::default();
            Self::new(default_instance)
        }
    }
}

impl PostgreSqlAutoCreation {
    #[instrument_trace]
    pub fn with_schema(instance: PostgreSqlInstanceConfig, schema: DatabaseSchema) -> Self {
        Self { instance, schema }
    }

    #[instrument_trace]
    async fn create_admin_connection(&self) -> Result<DatabaseConnection, AutoCreationError> {
        let admin_url: String = self.instance.get_admin_url();
        let timeout_duration: Duration = get_connection_timeout_duration();
        let timeout_seconds: u64 = timeout_duration.as_secs();
        let connection_result: Result<DatabaseConnection, DbErr> =
            match timeout(timeout_duration, Database::connect(&admin_url)).await {
                Ok(result) => result,
                Err(_) => {
                    return Err(AutoCreationError::Timeout(format!(
                        "PostgreSQL admin connection timeout after {timeout_seconds} seconds"
                    )));
                }
            };
        connection_result.map_err(|error: DbErr| {
            let error_msg: String = error.to_string();
            if error_msg.contains("authentication failed") || error_msg.contains("permission") {
                AutoCreationError::InsufficientPermissions(format!(
                    "Cannot connect to PostgreSQL server for database creation {error_msg}"
                ))
            } else if error_msg.contains("timeout") || error_msg.contains("Connection refused") {
                AutoCreationError::ConnectionFailed(format!(
                    "Cannot connect to PostgreSQL server {error_msg}"
                ))
            } else {
                AutoCreationError::DatabaseError(format!("PostgreSQL connection error {error_msg}"))
            }
        })
    }

    #[instrument_trace]
    async fn create_target_connection(&self) -> Result<DatabaseConnection, AutoCreationError> {
        let db_url: String = self.instance.get_connection_url();
        let timeout_duration: Duration = get_connection_timeout_duration();
        let timeout_seconds: u64 = timeout_duration.as_secs();
        let connection_result: Result<DatabaseConnection, DbErr> =
            match timeout(timeout_duration, Database::connect(&db_url)).await {
                Ok(result) => result,
                Err(_) => {
                    return Err(AutoCreationError::Timeout(format!(
                        "PostgreSQL database connection timeout after {timeout_seconds} seconds {}",
                        self.instance.get_database().as_str()
                    )));
                }
            };
        connection_result.map_err(|error: DbErr| {
            AutoCreationError::ConnectionFailed(format!(
                "Cannot connect to PostgreSQL database '{}' {error}",
                self.instance.get_database().as_str(),
            ))
        })
    }

    #[instrument_trace]
    async fn database_exists(
        &self,
        connection: &DatabaseConnection,
    ) -> Result<bool, AutoCreationError> {
        let query: String = format!(
            "SELECT 1 FROM pg_database WHERE datname = '{}'",
            self.instance.get_database().as_str()
        );
        let statement: Statement = Statement::from_string(DatabaseBackend::Postgres, query);
        match connection.query_all(statement).await {
            Ok(results) => Ok(!results.is_empty()),
            Err(error) => Err(AutoCreationError::DatabaseError(format!(
                "Failed to check if database exists {error}"
            ))),
        }
    }

    #[instrument_trace]
    async fn create_database(
        &self,
        connection: &DatabaseConnection,
    ) -> Result<bool, AutoCreationError> {
        if self.database_exists(connection).await? {
            AutoCreationLogger::log_database_exists(
                self.instance.get_database().as_str(),
                database::PluginType::PostgreSQL,
            )
            .await;
            return Ok(false);
        }
        let create_query: String = format!(
            "CREATE DATABASE \"{}\" WITH ENCODING='UTF8' LC_COLLATE='en_US.UTF-8' LC_CTYPE='en_US.UTF-8'",
            self.instance.get_database().as_str()
        );
        let statement: Statement = Statement::from_string(DatabaseBackend::Postgres, create_query);
        match connection.execute(statement).await {
            Ok(_) => {
                AutoCreationLogger::log_database_created(
                    self.instance.get_database().as_str(),
                    database::PluginType::PostgreSQL,
                )
                .await;
                Ok(true)
            }
            Err(error) => {
                let error_msg: String = error.to_string();
                if error_msg.contains("permission denied") || error_msg.contains("must be owner") {
                    Err(AutoCreationError::InsufficientPermissions(format!(
                        "Cannot create PostgreSQL database '{}' {}",
                        self.instance.get_database().as_str(),
                        error_msg
                    )))
                } else if error_msg.contains("already exists") {
                    AutoCreationLogger::log_database_exists(
                        self.instance.get_database().as_str(),
                        database::PluginType::PostgreSQL,
                    )
                    .await;
                    Ok(false)
                } else {
                    Err(AutoCreationError::DatabaseError(format!(
                        "Failed to create PostgreSQL database '{}' {}",
                        self.instance.get_database().as_str(),
                        error_msg
                    )))
                }
            }
        }
    }

    #[instrument_trace]
    async fn table_exists<T>(
        &self,
        connection: &DatabaseConnection,
        table_name: T,
    ) -> Result<bool, AutoCreationError>
    where
        T: AsRef<str>,
    {
        let table_name_str: &str = table_name.as_ref();
        let query: String = format!(
            "SELECT 1 FROM information_schema.tables WHERE table_schema = 'public' AND table_name = '{table_name_str}'"
        );
        let statement: Statement = Statement::from_string(DatabaseBackend::Postgres, query);
        match connection.query_all(statement).await {
            Ok(results) => Ok(!results.is_empty()),
            Err(error) => Err(AutoCreationError::DatabaseError(format!(
                "Failed to check if table '{table_name_str}' exists {error}"
            ))),
        }
    }

    #[instrument_trace]
    async fn create_table(
        &self,
        connection: &DatabaseConnection,
        table: &database::TableSchema,
    ) -> Result<(), AutoCreationError> {
        let statement: Statement =
            Statement::from_string(DatabaseBackend::Postgres, table.get_sql().clone());
        match connection.execute(statement).await {
            Ok(_) => Ok(()),
            Err(error) => {
                let error_msg: String = error.to_string();
                if error_msg.contains("permission denied") {
                    Err(AutoCreationError::InsufficientPermissions(format!(
                        "Cannot create PostgreSQL table '{}' {}",
                        table.get_name(),
                        error_msg
                    )))
                } else {
                    Err(AutoCreationError::SchemaError(format!(
                        "Failed to create PostgreSQL table '{}' {}",
                        table.get_name(),
                        error_msg
                    )))
                }
            }
        }
    }

    #[instrument_trace]
    async fn execute_sql<S>(
        &self,
        connection: &DatabaseConnection,
        sql: S,
    ) -> Result<(), AutoCreationError>
    where
        S: AsRef<str>,
    {
        let statement: Statement = Statement::from_string(DatabaseBackend::Postgres, sql.as_ref());
        match connection.execute(statement).await {
            Ok(_) => Ok(()),
            Err(error) => Err(AutoCreationError::DatabaseError(format!(
                "Failed to execute SQL {error}"
            ))),
        }
    }

    #[instrument_trace]
    fn get_database_schema(&self) -> &DatabaseSchema {
        &self.schema
    }
}

impl DatabaseAutoCreation for PostgreSqlAutoCreation {
    #[instrument_trace]
    async fn create_database_if_not_exists(&self) -> Result<bool, AutoCreationError> {
        let admin_connection: DatabaseConnection = self.create_admin_connection().await?;
        let result: Result<bool, AutoCreationError> = self.create_database(&admin_connection).await;
        let _: Result<(), DbErr> = admin_connection.close().await;
        result
    }

    #[instrument_trace]
    async fn create_tables_if_not_exist(&self) -> Result<Vec<String>, AutoCreationError> {
        let connection: DatabaseConnection = self.create_target_connection().await?;
        let schema: &DatabaseSchema = self.get_database_schema();
        let mut created_tables: Vec<String> = Vec::new();
        for table in schema.ordered_tables() {
            if !self.table_exists(&connection, table.get_name()).await? {
                self.create_table(&connection, table).await?;
                created_tables.push(table.get_name().clone());
                AutoCreationLogger::log_table_created(
                    table.get_name(),
                    self.instance.get_database().as_str(),
                    database::PluginType::PostgreSQL,
                )
                .await;
            } else {
                AutoCreationLogger::log_table_exists(
                    table.get_name(),
                    self.instance.get_database().as_str(),
                    database::PluginType::PostgreSQL,
                )
                .await;
            }
        }
        for index_sql in schema.get_indexes() {
            if let Err(error) = self.execute_sql(&connection, index_sql).await {
                AutoCreationLogger::log_auto_creation_error(
                    &error,
                    "Index creation",
                    database::PluginType::PostgreSQL,
                    Some(self.instance.get_database().as_str()),
                )
                .await;
            }
        }
        for constraint_sql in schema.get_constraints() {
            if let Err(error) = self.execute_sql(&connection, constraint_sql).await {
                AutoCreationLogger::log_auto_creation_error(
                    &error,
                    "Constraint creation",
                    database::PluginType::PostgreSQL,
                    Some(self.instance.get_database().as_str()),
                )
                .await;
            }
        }
        let _: Result<(), DbErr> = connection.close().await;
        AutoCreationLogger::log_tables_created(
            &created_tables,
            self.instance.get_database().as_str(),
            database::PluginType::PostgreSQL,
        )
        .await;
        Ok(created_tables)
    }

    #[instrument_trace]
    async fn verify_connection(&self) -> Result<(), AutoCreationError> {
        let connection: DatabaseConnection = self.create_target_connection().await?;
        let statement: Statement =
            Statement::from_string(DatabaseBackend::Postgres, "SELECT 1".to_string());
        match connection.query_all(statement).await {
            Ok(_) => {
                let _: Result<(), DbErr> = connection.close().await;
                AutoCreationLogger::log_connection_verification(
                    database::PluginType::PostgreSQL,
                    self.instance.get_database().as_str(),
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
                    self.instance.get_database().as_str(),
                    false,
                    Some(&error_msg),
                )
                .await;
                Err(AutoCreationError::ConnectionFailed(format!(
                    "PostgreSQL connection verification failed {error_msg}"
                )))
            }
        }
    }
}
