use super::*;

pub static POSTGRESQL_CONNECTIONS: OnceLock<
    RwLock<HashMap<String, ConnectionCache<DatabaseConnection>>>,
> = OnceLock::new();
