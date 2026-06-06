use super::*;

/// Global static storage for MySQL connection caches, mapping instance names to their cached connections.
pub static MYSQL_CONNECTIONS: OnceLock<
    RwLock<HashMap<String, ConnectionCache<DatabaseConnection>>>,
> = OnceLock::new();
