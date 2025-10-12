#[derive(Debug, Clone)]
pub enum AutoCreationError {
    InsufficientPermissions(String),
    ConnectionFailed(String),
    SchemaError(String),
    Timeout(String),
    DatabaseError(String),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PluginType {
    MySQL,
    PostgreSQL,
    Redis,
}
