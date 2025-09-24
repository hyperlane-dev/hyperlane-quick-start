use super::*;
use bb8_postgres::PostgresConnectionManager;
use once_cell::sync::OnceCell;
use std::sync::Arc;
use std::time::Duration;
use tokio_postgres::NoTls;

impl DatabaseConfig {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from_env() -> Self {
        Self {
            host: std::env::var("DATABASE_HOST").unwrap_or_else(|_| DATABASE_HOST.to_string()),
            port: std::env::var("DATABASE_PORT")
                .unwrap_or_else(|_| DATABASE_PORT.to_string())
                .parse()
                .unwrap_or(DATABASE_PORT),
            database: std::env::var("DATABASE_NAME").unwrap_or_else(|_| DATABASE_NAME.to_string()),
            username: std::env::var("DATABASE_USER").unwrap_or_else(|_| DATABASE_USER.to_string()),
            password: std::env::var("DATABASE_PASSWORD")
                .unwrap_or_else(|_| DATABASE_PASSWORD.to_string()),
            max_connections: std::env::var("DATABASE_MAX_CONNECTIONS")
                .unwrap_or_else(|_| DATABASE_MAX_CONNECTIONS.to_string())
                .parse()
                .unwrap_or(DATABASE_MAX_CONNECTIONS),
            min_connections: std::env::var("DATABASE_MIN_CONNECTIONS")
                .unwrap_or_else(|_| DATABASE_MIN_CONNECTIONS.to_string())
                .parse()
                .unwrap_or(DATABASE_MIN_CONNECTIONS),
            connection_timeout: Duration::from_secs(
                std::env::var("DATABASE_CONNECTION_TIMEOUT")
                    .unwrap_or_else(|_| DATABASE_CONNECTION_TIMEOUT_SECS.to_string())
                    .parse()
                    .unwrap_or(DATABASE_CONNECTION_TIMEOUT_SECS),
            ),
            idle_timeout: Duration::from_secs(
                std::env::var("DATABASE_IDLE_TIMEOUT")
                    .unwrap_or_else(|_| DATABASE_IDLE_TIMEOUT_SECS.to_string())
                    .parse()
                    .unwrap_or(DATABASE_IDLE_TIMEOUT_SECS),
            ),
        }
    }

    pub fn connection_string(&self) -> String {
        format!(
            "host={} port={} dbname={} user={} password={}",
            self.host, self.port, self.database, self.username, self.password
        )
    }

    pub fn validate(&self) -> Result<(), DatabaseConfigError> {
        if self.host.is_empty() {
            return Err(DatabaseConfigError::InvalidHost);
        }
        if self.port == 0 {
            return Err(DatabaseConfigError::InvalidPort);
        }
        if self.database.is_empty() {
            return Err(DatabaseConfigError::InvalidDatabase);
        }
        if self.username.is_empty() {
            return Err(DatabaseConfigError::InvalidUsername);
        }
        if self.max_connections == 0 {
            return Err(DatabaseConfigError::InvalidMaxConnections);
        }
        if self.min_connections > self.max_connections {
            return Err(DatabaseConfigError::InvalidMinConnections);
        }
        Ok(())
    }
}

impl DatabaseConnectionPool {
    pub async fn new(config: DatabaseConfig) -> Result<Self, ConnectionPoolError> {
        config.validate()?;

        let connection_string = config.connection_string();
        let manager = PostgresConnectionManager::new_from_stringlike(connection_string, NoTls)
            .map_err(|e| ConnectionPoolError::ManagerCreationFailed(e.to_string()))?;

        let pool = bb8::Pool::builder()
            .max_size(config.max_connections)
            .min_idle(Some(config.min_connections))
            .connection_timeout(config.connection_timeout)
            .idle_timeout(Some(config.idle_timeout))
            .build(manager)
            .await
            .map_err(|e| ConnectionPoolError::PoolCreationFailed(e.to_string()))?;

        Ok(Self { pool, config })
    }

    pub async fn get_connection(
        &self,
    ) -> Result<bb8::PooledConnection<'_, PostgresConnectionManager<NoTls>>, ConnectionPoolError>
    {
        self.pool
            .get()
            .await
            .map_err(|e| ConnectionPoolError::ConnectionAcquisitionFailed(e.to_string()))
    }

    pub async fn health_check(&self) -> Result<(), ConnectionPoolError> {
        let conn = self.get_connection().await?;
        conn.simple_query("SELECT 1")
            .await
            .map_err(|e| ConnectionPoolError::HealthCheckFailed(e.to_string()))?;
        Ok(())
    }

    pub fn get_status(&self) -> PoolStatus {
        let state = self.pool.state();
        PoolStatus {
            connections: state.connections,
            idle_connections: state.idle_connections,
            max_size: self.config.max_connections,
            min_idle: self.config.min_connections,
        }
    }
}

impl PoolStatus {
    pub fn is_healthy(&self) -> bool {
        self.connections > 0 && self.connections <= self.max_size
    }

    pub fn utilization_percentage(&self) -> f64 {
        if self.max_size == 0 {
            0.0
        } else {
            (self.connections as f64 / self.max_size as f64) * 100.0
        }
    }
}

// Global connection pool management
static GLOBAL_POOL: OnceCell<Arc<DatabaseConnectionPool>> = OnceCell::new();

pub async fn initialize_global_pool(config: DatabaseConfig) -> Result<(), ConnectionPoolError> {
    let pool = DatabaseConnectionPool::new(config).await?;
    GLOBAL_POOL
        .set(Arc::new(pool))
        .map_err(|_| ConnectionPoolError::GlobalPoolAlreadyInitialized)?;
    Ok(())
}

pub fn get_global_pool() -> Result<Arc<DatabaseConnectionPool>, ConnectionPoolError> {
    GLOBAL_POOL
        .get()
        .cloned()
        .ok_or(ConnectionPoolError::GlobalPoolNotInitialized)
}

pub async fn global_health_check() -> Result<(), ConnectionPoolError> {
    let pool = get_global_pool()?;
    pool.health_check().await
}
