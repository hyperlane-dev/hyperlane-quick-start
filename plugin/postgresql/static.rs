use super::*;

/// Global static storage for PostgreSQL connection caches, mapping instance names to their cached connections.
pub static POSTGRESQL_CONNECTIONS: OnceLock<
    RwLock<HashMap<String, ConnectionCache<DatabaseConnection>>>,
> = OnceLock::new();
