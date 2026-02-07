use super::*;

pub static MYSQL_CONNECTIONS: OnceLock<
    RwLock<HashMap<String, ConnectionCache<DatabaseConnection>>>,
> = OnceLock::new();
