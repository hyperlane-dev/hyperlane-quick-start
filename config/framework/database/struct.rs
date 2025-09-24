use super::*;
use bb8::Pool;
use bb8_postgres::PostgresConnectionManager;
use serde::{Deserialize, Serialize};
use std::time::Duration;
use thiserror::Error;
use tokio_postgres::NoTls;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseConfig {
    pub host: String,
    pub port: u16,
    pub database: String,
    pub username: String,
    pub password: String,
    pub max_connections: u32,
    pub min_connections: u32,
    pub connection_timeout: Duration,
    pub idle_timeout: Duration,
}

impl Default for DatabaseConfig {
    fn default() -> Self {
        Self {
            host: DATABASE_HOST.to_string(),
            port: DATABASE_PORT,
            database: DATABASE_NAME.to_string(),
            username: DATABASE_USER.to_string(),
            password: DATABASE_PASSWORD.to_string(),
            max_connections: DATABASE_MAX_CONNECTIONS,
            min_connections: DATABASE_MIN_CONNECTIONS,
            connection_timeout: Duration::from_secs(DATABASE_CONNECTION_TIMEOUT_SECS),
            idle_timeout: Duration::from_secs(DATABASE_IDLE_TIMEOUT_SECS),
        }
    }
}

pub type ConnectionPool = Pool<PostgresConnectionManager<NoTls>>;

pub struct DatabaseConnectionPool {
    pub pool: ConnectionPool,
    pub config: DatabaseConfig,
}

#[derive(Debug, Clone)]
pub struct PoolStatus {
    pub connections: u32,
    pub idle_connections: u32,
    pub max_size: u32,
    pub min_idle: u32,
}

#[derive(Debug, Error)]
pub enum DatabaseConfigError {
    #[error("Invalid host configuration")]
    InvalidHost,

    #[error("Invalid port configuration")]
    InvalidPort,

    #[error("Invalid database name")]
    InvalidDatabase,

    #[error("Invalid username")]
    InvalidUsername,

    #[error("Invalid max connections configuration")]
    InvalidMaxConnections,

    #[error("Invalid min connections configuration (must be <= max_connections)")]
    InvalidMinConnections,
}

#[derive(Debug, Error)]
pub enum ConnectionPoolError {
    #[error("Configuration error: {0}")]
    ConfigError(#[from] DatabaseConfigError),

    #[error("Failed to create connection manager: {0}")]
    ManagerCreationFailed(String),

    #[error("Failed to create connection pool: {0}")]
    PoolCreationFailed(String),

    #[error("Failed to acquire connection: {0}")]
    ConnectionAcquisitionFailed(String),

    #[error("Health check failed: {0}")]
    HealthCheckFailed(String),

    #[error("Global pool not initialized")]
    GlobalPoolNotInitialized,

    #[error("Global pool already initialized")]
    GlobalPoolAlreadyInitialized,
}
