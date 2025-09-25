use super::*;

#[derive(Debug, Error)]
pub enum DatabaseException {
    #[error("Connection failed: {0}")]
    ConnectionFailed(String),

    #[error("Query execution failed: {0}")]
    QueryFailed(String),

    #[error("Transaction failed: {0}")]
    TransactionFailed(String),

    #[error("Pool exhausted")]
    PoolExhausted,

    #[error("Timeout occurred: {0}")]
    Timeout(String),

    #[error("Data integrity violation: {0}")]
    IntegrityViolation(String),

    #[error("Serialization failed: {0}")]
    SerializationFailed(String),
}
