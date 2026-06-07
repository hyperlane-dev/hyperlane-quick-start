use super::*;

/// Implementation of `GetOrInit` for `MySqlPlugin`, providing lazy initialization of the global MySQL connection cache.
impl GetOrInit for MySqlPlugin {
    type Instance = RwLock<HashMap<String, ConnectionCache<DatabaseConnection>>>;

    /// Lazily initializes and returns a static reference to the global MySQL connection cache.
    ///
    /// # Returns
    ///
    /// - `&'static RwLock<HashMap<String, ConnectionCache<DatabaseConnection>>>`: The static reference to the global MySQL connection map.
    #[instrument_trace]
    fn get_or_init() -> &'static Self::Instance {
        MYSQL_CONNECTIONS.get_or_init(|| RwLock::new(HashMap::new()))
    }
}

/// Implementation of `DatabaseConnectionPlugin` for `MySqlPlugin`, managing MySQL connections and auto-creation.
impl DatabaseConnectionPlugin for MySqlPlugin {
    type InstanceConfig = MySqlInstanceConfig;

    type AutoCreation = MySqlAutoCreation;

    type Connection = DatabaseConnection;

    type ConnectionCache = RwLock<HashMap<String, ConnectionCache<Self::Connection>>>;

    /// Returns the plugin type identifier for MySQL.
    ///
    /// # Returns
    ///
    /// - `PluginType::MySQL`: The MySQL plugin type.
    #[instrument_trace]
    fn plugin_type() -> PluginType {
        PluginType::MySQL
    }

    /// Creates a new MySQL database connection for the specified instance, performing auto-creation if needed.
    ///
    /// # Arguments
    ///
    /// - `I`: The instance name identifier.
    /// - `Option<DatabaseSchema>`: The optional database schema for auto-creation.
    ///
    /// # Returns
    ///
    /// - `Result<Self::Connection, String>`: The connection on success, or an error message on failure.
    #[instrument_trace]
    async fn connection_db<I>(
        instance_name: I,
        schema: Option<DatabaseSchema>,
    ) -> Result<Self::Connection, String>
    where
        I: AsRef<str> + Send,
    {
        let instance_name_str: &str = instance_name.as_ref();
        let env: &'static EnvConfig = EnvPlugin::get_or_init();
        let instance: &MySqlInstanceConfig = env
            .get_mysql_instance(instance_name_str)
            .ok_or_else(|| format!("MySQL instance '{instance_name_str}' not found"))?;
        match Self::perform_auto_creation(instance, schema.clone()).await {
            Ok(result) => {
                if result.has_changes() {
                    AutoCreationLogger::log_auto_creation_complete(PluginType::MySQL, &result)
                        .await;
                }
            }
            Err(error) => {
                AutoCreationLogger::log_auto_creation_error(
                    &error,
                    "Auto-creation process",
                    PluginType::MySQL,
                    Some(instance.get_database().as_str()),
                )
                .await;
                if !error.should_continue() {
                    return Err(error.to_string());
                }
            }
        }
        let db_url: String = instance.get_connection_url();
        let timeout_duration: Duration = DatabasePlugin::get_connection_timeout_duration();
        let timeout_seconds: u64 = timeout_duration.as_secs();
        let connection_result: Result<DatabaseConnection, DbErr> =
            match timeout(timeout_duration, Database::connect(&db_url)).await {
                Ok(result) => result,
                Err(_) => Err(DbErr::Custom(format!(
                    "MySQL connection timeout after {timeout_seconds} seconds"
                ))),
            };
        connection_result.map_err(|error: DbErr| {
            let error_msg: String = error.to_string();
            let database_name: String = instance.get_database().clone();
            let error_msg_clone: String = error_msg.clone();
            spawn(async move {
                AutoCreationLogger::log_connection_verification(
                    PluginType::MySQL,
                    &database_name,
                    false,
                    Some(&error_msg_clone),
                )
                .await;
            });
            error_msg
        })
    }

    /// Retrieves an existing cached MySQL connection or creates a new one for the specified instance.
    ///
    /// # Arguments
    ///
    /// - `I`: The instance name identifier.
    /// - `Option<DatabaseSchema>`: The optional database schema for auto-creation.
    ///
    /// # Returns
    ///
    /// - `Result<Self::Connection, String>`: The connection on success, or an error message on failure.
    #[instrument_trace]
    async fn get_connection<I>(
        instance_name: I,
        schema: Option<DatabaseSchema>,
    ) -> Result<Self::Connection, String>
    where
        I: AsRef<str> + Send,
    {
        let instance_name_str: &str = instance_name.as_ref();
        let duration: Duration = DatabasePlugin::get_retry_duration();
        {
            if let Some(cache) = Self::get_or_init().read().await.get(instance_name_str) {
                match cache.try_get_result() {
                    Ok(conn) => return Ok(conn.clone()),
                    Err(error) => {
                        if !cache.is_expired(duration) {
                            return Err(error.clone());
                        }
                    }
                }
            }
        }
        let mut connections: RwLockWriteGuard<
            '_,
            HashMap<String, ConnectionCache<DatabaseConnection>>,
        > = Self::get_or_init().write().await;
        if let Some(cache) = connections.get(instance_name_str) {
            match cache.try_get_result() {
                Ok(conn) => return Ok(conn.clone()),
                Err(error) => {
                    if !cache.is_expired(duration) {
                        return Err(error.clone());
                    }
                }
            }
        }
        connections.remove(instance_name_str);
        drop(connections);
        let new_connection: Result<DatabaseConnection, String> =
            Self::connection_db(instance_name_str, schema).await;
        let mut connections: RwLockWriteGuard<
            '_,
            HashMap<String, ConnectionCache<DatabaseConnection>>,
        > = Self::get_or_init().write().await;
        connections.insert(
            instance_name_str.to_string(),
            ConnectionCache::new(new_connection.clone()),
        );
        new_connection
    }

    /// Performs the full auto-creation process for a MySQL instance, including database, tables, indexes, and data initialization.
    ///
    /// # Arguments
    ///
    /// - `&Self::InstanceConfig`: The MySQL instance configuration.
    /// - `Option<DatabaseSchema>`: The optional database schema for table creation.
    ///
    /// # Returns
    ///
    /// - `Result<AutoCreationResult, AutoCreationError>`: The auto-creation result on success, or an error on failure.
    #[instrument_trace]
    async fn perform_auto_creation(
        instance: &Self::InstanceConfig,
        schema: Option<DatabaseSchema>,
    ) -> Result<AutoCreationResult, AutoCreationError> {
        let start_time: Instant = Instant::now();
        let mut result: AutoCreationResult = AutoCreationResult::default();
        AutoCreationLogger::log_auto_creation_start(PluginType::MySQL, instance.get_database())
            .await;
        let auto_creator: MySqlAutoCreation = match schema {
            Some(s) => MySqlAutoCreation::with_schema(instance.clone(), s),
            None => MySqlAutoCreation::new(instance.clone()),
        };
        match auto_creator.create_database_if_not_exists().await {
            Ok(created) => {
                result.set_database_created(created);
            }
            Err(error) => {
                AutoCreationLogger::log_auto_creation_error(
                    &error,
                    "Database creation",
                    PluginType::MySQL,
                    Some(instance.get_database()),
                )
                .await;
                if !error.should_continue() {
                    result.set_duration(start_time.elapsed());
                    return Err(error);
                }
                result.get_mut_errors().push(error.to_string());
            }
        }
        match auto_creator.create_tables_if_not_exist().await {
            Ok(tables) => {
                result.set_tables_created(tables);
            }
            Err(error) => {
                AutoCreationLogger::log_auto_creation_error(
                    &error,
                    "Table creation",
                    PluginType::MySQL,
                    Some(instance.get_database().as_str()),
                )
                .await;
                result.get_mut_errors().push(error.to_string());
            }
        }
        if let Err(error) = auto_creator.create_indexes().await {
            AutoCreationLogger::log_auto_creation_error(
                &error,
                "Index creation",
                PluginType::MySQL,
                Some(instance.get_database().as_str()),
            )
            .await;
            result.get_mut_errors().push(error.to_string());
        }
        if let Err(error) = auto_creator.init_data().await {
            AutoCreationLogger::log_auto_creation_error(
                &error,
                "Init data",
                PluginType::MySQL,
                Some(instance.get_database().as_str()),
            )
            .await;
            result.get_mut_errors().push(error.to_string());
        }
        if let Err(error) = auto_creator.verify_connection().await {
            AutoCreationLogger::log_auto_creation_error(
                &error,
                "Connection verification",
                PluginType::MySQL,
                Some(instance.get_database().as_str()),
            )
            .await;
            if !error.should_continue() {
                result.set_duration(start_time.elapsed());
                return Err(error);
            }
            result.get_mut_errors().push(error.to_string());
        }
        result.set_duration(start_time.elapsed());
        AutoCreationLogger::log_auto_creation_complete(PluginType::MySQL, &result).await;
        Ok(result)
    }
}

/// Implementation of `Default` for `MySqlAutoCreation`, using the default MySQL instance from the environment configuration.
impl Default for MySqlAutoCreation {
    /// Returns the default `MySqlAutoCreation` instance, using the first configured MySQL instance or a default configuration.
    ///
    /// # Returns
    ///
    /// - `MySqlAutoCreation`: The default auto-creation handler.
    #[instrument_trace]
    fn default() -> Self {
        let env: &'static EnvConfig = EnvPlugin::get_or_init();
        if let Some(instance) = env.get_default_mysql_instance() {
            Self::new(instance.clone())
        } else {
            let default_instance: MySqlInstanceConfig = MySqlInstanceConfig::default();
            Self::new(default_instance)
        }
    }
}

/// Implementation of database and table creation methods for `MySqlAutoCreation`.
impl MySqlAutoCreation {
    /// Creates an admin connection to the MySQL server without specifying a database.
    ///
    /// # Returns
    ///
    /// - `Result<DatabaseConnection, AutoCreationError>`: The admin connection on success, or an error on failure.
    #[instrument_trace]
    async fn create_admin_connection(&self) -> Result<DatabaseConnection, AutoCreationError> {
        let admin_url: String = self.instance.get_admin_url();
        let timeout_duration: Duration = DatabasePlugin::get_connection_timeout_duration();
        let timeout_seconds: u64 = timeout_duration.as_secs();
        let connection_result: Result<DatabaseConnection, DbErr> =
            match timeout(timeout_duration, Database::connect(&admin_url)).await {
                Ok(result) => result,
                Err(_) => {
                    return Err(AutoCreationError::Timeout(format!(
                        "MySQL admin connection timeout after {timeout_seconds} seconds"
                    )));
                }
            };
        connection_result.map_err(|error: DbErr| {
            let error_msg: String = error.to_string();
            if error_msg.contains("Access denied") || error_msg.contains("permission") {
                AutoCreationError::InsufficientPermissions(format!(
                    "Cannot connect to MySQL server for database creation {error_msg}"
                ))
            } else if error_msg.contains("timeout") || error_msg.contains("Connection refused") {
                AutoCreationError::ConnectionFailed(format!(
                    "Cannot connect to MySQL server {error_msg}"
                ))
            } else {
                AutoCreationError::DatabaseError(format!("MySQL connection error {error_msg}"))
            }
        })
    }

    /// Checks whether the target database already exists on the MySQL server.
    ///
    /// # Arguments
    ///
    /// - `&DatabaseConnection`: The admin connection to the MySQL server.
    ///
    /// # Returns
    ///
    /// - `Result<bool, AutoCreationError>`: True if the database exists, false otherwise.
    #[instrument_trace]
    async fn database_exists(
        &self,
        connection: &DatabaseConnection,
    ) -> Result<bool, AutoCreationError> {
        let query: String = format!(
            "SELECT SCHEMA_NAME FROM INFORMATION_SCHEMA.SCHEMATA WHERE SCHEMA_NAME = '{}'",
            self.instance.get_database()
        );
        let statement: Statement = Statement::from_string(DatabaseBackend::MySql, query);
        match connection.query_all(statement).await {
            Ok(results) => Ok(!results.is_empty()),
            Err(error) => Err(AutoCreationError::DatabaseError(format!(
                "Failed to check if database exists {error}"
            ))),
        }
    }

    /// Creates the target database if it does not already exist.
    ///
    /// # Arguments
    ///
    /// - `&DatabaseConnection`: The admin connection to the MySQL server.
    ///
    /// # Returns
    ///
    /// - `Result<bool, AutoCreationError>`: True if the database was created, false if it already existed.
    #[instrument_trace]
    async fn create_database(
        &self,
        connection: &DatabaseConnection,
    ) -> Result<bool, AutoCreationError> {
        if self.database_exists(connection).await? {
            AutoCreationLogger::log_database_exists(
                self.instance.get_database().as_str(),
                PluginType::MySQL,
            )
            .await;
            return Ok(false);
        }
        let create_query: String = format!(
            "CREATE DATABASE IF NOT EXISTS `{}` CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci",
            self.instance.get_database()
        );
        let statement: Statement = Statement::from_string(DatabaseBackend::MySql, create_query);
        match connection.execute(statement).await {
            Ok(_) => {
                AutoCreationLogger::log_database_created(
                    self.instance.get_database().as_str(),
                    PluginType::MySQL,
                )
                .await;
                Ok(true)
            }
            Err(error) => {
                let error_msg: String = error.to_string();
                if error_msg.contains("Access denied") || error_msg.contains("permission") {
                    Err(AutoCreationError::InsufficientPermissions(format!(
                        "Cannot create MySQL database '{}' {}",
                        self.instance.get_database().as_str(),
                        error_msg
                    )))
                } else {
                    Err(AutoCreationError::DatabaseError(format!(
                        "Failed to create MySQL database '{}' {}",
                        self.instance.get_database().as_str(),
                        error_msg
                    )))
                }
            }
        }
    }

    /// Creates a connection to the target MySQL database.
    ///
    /// # Returns
    ///
    /// - `Result<DatabaseConnection, AutoCreationError>`: The database connection on success, or an error on failure.
    #[instrument_trace]
    async fn create_target_connection(&self) -> Result<DatabaseConnection, AutoCreationError> {
        let db_url: String = self.instance.get_connection_url();
        let timeout_duration: Duration = DatabasePlugin::get_connection_timeout_duration();
        let timeout_seconds: u64 = timeout_duration.as_secs();
        let connection_result: Result<DatabaseConnection, DbErr> =
            match timeout(timeout_duration, Database::connect(&db_url)).await {
                Ok(result) => result,
                Err(_) => {
                    return Err(AutoCreationError::Timeout(format!(
                        "MySQL database connection timeout after {timeout_seconds} seconds {}",
                        self.instance.get_database()
                    )));
                }
            };
        connection_result.map_err(|error: DbErr| {
            AutoCreationError::ConnectionFailed(format!(
                "Cannot connect to MySQL database '{}' {}",
                self.instance.get_database().as_str(),
                error
            ))
        })
    }

    /// Checks whether a table already exists in the target MySQL database.
    ///
    /// # Arguments
    ///
    /// - `&DatabaseConnection`: The connection to the target database.
    /// - `T`: The table name to check.
    ///
    /// # Returns
    ///
    /// - `Result<bool, AutoCreationError>`: True if the table exists, false otherwise.
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
            "SELECT TABLE_NAME FROM INFORMATION_SCHEMA.TABLES WHERE TABLE_SCHEMA = '{}' AND TABLE_NAME = '{table_name_str}'",
            self.instance.get_database()
        );
        let statement: Statement = Statement::from_string(DatabaseBackend::MySql, query);
        match connection.query_all(statement).await {
            Ok(results) => Ok(!results.is_empty()),
            Err(error) => Err(AutoCreationError::DatabaseError(format!(
                "Failed to check if table '{table_name_str}' exists {error}"
            ))),
        }
    }

    /// Creates a single table in the target MySQL database using the provided table schema.
    ///
    /// # Arguments
    ///
    /// - `&DatabaseConnection`: The connection to the target database.
    /// - `&TableSchema`: The table schema containing the creation SQL.
    ///
    /// # Returns
    ///
    /// - `Result<(), AutoCreationError>`: Ok on success, or an error on failure.
    #[instrument_trace]
    async fn create_table(
        &self,
        connection: &DatabaseConnection,
        table: &TableSchema,
    ) -> Result<(), AutoCreationError> {
        let statement: Statement =
            Statement::from_string(DatabaseBackend::MySql, table.get_sql().clone());
        match connection.execute(statement).await {
            Ok(_) => Ok(()),
            Err(error) => {
                let error_msg: String = error.to_string();
                if error_msg.contains("Access denied") || error_msg.contains("permission") {
                    Err(AutoCreationError::InsufficientPermissions(format!(
                        "Cannot create MySQL table '{}' {}",
                        table.get_name(),
                        error_msg
                    )))
                } else {
                    Err(AutoCreationError::SchemaError(format!(
                        "Failed to create MySQL table '{}' {}",
                        table.get_name(),
                        error_msg
                    )))
                }
            }
        }
    }

    /// Executes a raw SQL statement on the target MySQL database.
    ///
    /// # Arguments
    ///
    /// - `&DatabaseConnection`: The connection to the target database.
    /// - `S`: The SQL statement to execute.
    ///
    /// # Returns
    ///
    /// - `Result<(), AutoCreationError>`: Ok on success, or an error on failure.
    #[instrument_trace]
    async fn execute_sql<S>(
        &self,
        connection: &DatabaseConnection,
        sql: S,
    ) -> Result<(), AutoCreationError>
    where
        S: AsRef<str>,
    {
        let statement: Statement = Statement::from_string(DatabaseBackend::MySql, sql.as_ref());
        match connection.execute(statement).await {
            Ok(_) => Ok(()),
            Err(error) => Err(AutoCreationError::DatabaseError(format!(
                "Failed to execute SQL {error}"
            ))),
        }
    }

    /// Returns a reference to the database schema associated with this auto-creation handler.
    ///
    /// # Returns
    ///
    /// - `&DatabaseSchema`: The database schema reference.
    #[instrument_trace]
    fn get_database_schema(&self) -> &DatabaseSchema {
        &self.schema
    }

    /// Creates indexes and constraints defined in the database schema on the target MySQL database.
    ///
    /// # Returns
    ///
    /// - `Result<(), AutoCreationError>`: Ok on success, or an error on failure.
    #[instrument_trace]
    async fn create_indexes(&self) -> Result<(), AutoCreationError> {
        let connection: DatabaseConnection = self.create_target_connection().await?;
        let schema: &DatabaseSchema = self.get_database_schema();
        for index_sql in schema.get_indexes() {
            if let Err(error) = self.execute_sql(&connection, index_sql).await {
                AutoCreationLogger::log_auto_creation_error(
                    &error,
                    "Index creation",
                    PluginType::MySQL,
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
                    PluginType::MySQL,
                    Some(self.instance.get_database().as_str()),
                )
                .await;
            }
        }
        let _: Result<(), DbErr> = connection.close().await;
        Ok(())
    }
}

/// Implementation of `DatabaseAutoCreation` for `MySqlAutoCreation`, providing the trait methods for MySQL database lifecycle management.
impl DatabaseAutoCreation for MySqlAutoCreation {
    type InstanceConfig = MySqlInstanceConfig;

    /// Creates a new `MySqlAutoCreation` handler from the given MySQL instance configuration.
    ///
    /// # Arguments
    ///
    /// - `MySqlInstanceConfig`: The MySQL instance configuration.
    #[instrument_trace]
    fn new(instance: Self::InstanceConfig) -> Self {
        Self {
            instance,
            schema: DatabaseSchema::default(),
        }
    }

    /// Creates a new `MySqlAutoCreation` handler with an explicit database schema.
    ///
    /// # Arguments
    ///
    /// - `MySqlInstanceConfig`: The MySQL instance configuration.
    /// - `DatabaseSchema`: The database schema containing table definitions.
    #[instrument_trace]
    fn with_schema(instance: Self::InstanceConfig, schema: DatabaseSchema) -> Self
    where
        Self: Sized,
    {
        Self { instance, schema }
    }

    /// Creates the MySQL database if it does not already exist.
    ///
    /// # Returns
    ///
    /// - `Result<bool, AutoCreationError>`: True if the database was created, false if it already existed.
    #[instrument_trace]
    async fn create_database_if_not_exists(&self) -> Result<bool, AutoCreationError> {
        let admin_connection: DatabaseConnection = self.create_admin_connection().await?;
        let result: Result<bool, AutoCreationError> = self.create_database(&admin_connection).await;
        let _: Result<(), DbErr> = admin_connection.close().await;
        result
    }

    /// Creates all tables defined in the schema that do not already exist, in topological order.
    ///
    /// # Returns
    ///
    /// - `Result<Vec<String>, AutoCreationError>`: A list of table names that were created.
    #[instrument_trace]
    async fn create_tables_if_not_exist(&self) -> Result<Vec<String>, AutoCreationError> {
        let connection: DatabaseConnection = self.create_target_connection().await?;
        let schema: &DatabaseSchema = self.get_database_schema();
        let mut created_tables: Vec<String> = vec![];
        for table in schema.ordered_tables() {
            if !self.table_exists(&connection, table.get_name()).await? {
                self.create_table(&connection, table).await?;
                created_tables.push(table.get_name().clone());
                AutoCreationLogger::log_table_created(
                    table.get_name(),
                    self.instance.get_database().as_str(),
                    PluginType::MySQL,
                )
                .await;
            } else {
                AutoCreationLogger::log_table_exists(
                    table.get_name(),
                    self.instance.get_database().as_str(),
                    PluginType::MySQL,
                )
                .await;
            }
        }
        let _: Result<(), DbErr> = connection.close().await;
        AutoCreationLogger::log_tables_created(
            &created_tables,
            self.instance.get_database().as_str(),
            PluginType::MySQL,
        )
        .await;
        Ok(created_tables)
    }

    /// Initializes data in the MySQL database using the init data SQL statements from the schema.
    ///
    /// # Returns
    ///
    /// - `Result<(), AutoCreationError>`: Ok on success, or an error on failure.
    #[instrument_trace]
    async fn init_data(&self) -> Result<(), AutoCreationError> {
        let connection: DatabaseConnection = self.create_target_connection().await?;
        let schema: &DatabaseSchema = self.get_database_schema();
        for init_data_sql in schema.get_init_data() {
            if let Err(error) = self.execute_sql(&connection, init_data_sql).await {
                AutoCreationLogger::log_auto_creation_error(
                    &error,
                    "Init data insertion",
                    PluginType::MySQL,
                    Some(self.instance.get_database().as_str()),
                )
                .await;
            }
        }
        let _: Result<(), DbErr> = connection.close().await;
        Ok(())
    }

    /// Verifies the MySQL database connection is working by executing a simple SELECT query.
    ///
    /// # Returns
    ///
    /// - `Result<(), AutoCreationError>`: Ok if the connection is valid, or an error on failure.
    #[instrument_trace]
    async fn verify_connection(&self) -> Result<(), AutoCreationError> {
        let db_url: String = self.instance.get_connection_url();
        let timeout_duration: Duration = DatabasePlugin::get_connection_timeout_duration();
        let timeout_seconds: u64 = timeout_duration.as_secs();
        let connection_result: Result<DatabaseConnection, DbErr> =
            match timeout(timeout_duration, Database::connect(&db_url)).await {
                Ok(result) => result,
                Err(_) => {
                    return Err(AutoCreationError::Timeout(format!(
                        "Failed to verify MySQL connection within {timeout_seconds} seconds"
                    )));
                }
            };
        let connection: DatabaseConnection = connection_result.map_err(|error: DbErr| {
            AutoCreationError::ConnectionFailed(format!(
                "Failed to verify MySQL connection {error}"
            ))
        })?;
        let statement: Statement =
            Statement::from_string(DatabaseBackend::MySql, "SELECT 1".to_string());
        match connection.query_all(statement).await {
            Ok(_) => {
                let _: Result<(), DbErr> = connection.close().await;
                AutoCreationLogger::log_connection_verification(
                    PluginType::MySQL,
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
                    PluginType::MySQL,
                    self.instance.get_database().as_str(),
                    false,
                    Some(&error_msg),
                )
                .await;
                Err(AutoCreationError::ConnectionFailed(format!(
                    "MySQL connection verification failed {error_msg}"
                )))
            }
        }
    }
}
