use super::*;
use hyperlane_config::framework::*;
use thiserror::Error;
use tokio_postgres::{Error as PostgresError, NoTls};

pub struct DatabaseInitializer;

#[derive(Debug, Error)]
pub enum DatabaseInitError {
    #[error("Connection failed: {0}")]
    ConnectionFailed(String),

    #[error("Query execution failed: {0}")]
    QueryFailed(String),

    #[error("Database creation failed: {0}")]
    DatabaseCreationFailed(String),

    #[error("Table creation failed: {0}")]
    TableCreationFailed(String),

    #[error("Index creation failed: {0}")]
    IndexCreationFailed(String),

    #[error("Data seeding failed: {0}")]
    DataSeedingFailed(String),

    #[error("Password hashing failed: {0}")]
    PasswordHashingFailed(String),

    #[error("Pool initialization failed: {0}")]
    PoolInitializationFailed(ConnectionPoolError),

    #[error("Pool not initialized")]
    PoolNotInitialized(ConnectionPoolError),

    #[error("Connection acquisition failed: {0}")]
    ConnectionAcquisitionFailed(String),

    #[error("Health check failed: {0}")]
    HealthCheckFailed(#[from] ConnectionPoolError),

    #[error("Transaction failed: {0}")]
    TransactionFailed(String),

    #[error("Migration failed: {0}")]
    MigrationFailed(String),
}
