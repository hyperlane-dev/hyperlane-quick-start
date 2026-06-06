/// Error type for database auto-creation operations, representing various failure modes.
#[derive(Clone, Debug)]
pub enum AutoCreationError {
    /// The database user has insufficient permissions to perform the operation.
    InsufficientPermissions(String),
    /// The connection to the database server failed.
    ConnectionFailed(String),
    /// A schema-related error occurred during table or constraint creation.
    SchemaError(String),
    /// The operation timed out before completion.
    Timeout(String),
    /// A general database error occurred.
    DatabaseError(String),
}

/// Enumeration of supported database plugin types.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PluginType {
    /// MySQL database plugin.
    MySQL,
    /// PostgreSQL database plugin.
    PostgreSQL,
    /// Redis in-memory data store plugin.
    Redis,
}
