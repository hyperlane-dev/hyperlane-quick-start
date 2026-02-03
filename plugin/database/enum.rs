#[derive(Clone, Debug)]
pub enum AutoCreationError {
    InsufficientPermissions(String),
    ConnectionFailed(String),
    SchemaError(String),
    Timeout(String),
    DatabaseError(String),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PluginType {
    MySQL,
    PostgreSQL,
    Redis,
}
