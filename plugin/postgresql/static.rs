use super::*;

pub static POSTGRESQL_CONNECTIONS: Lazy<
    RwLock<HashMap<String, Result<DatabaseConnection, String>>>,
> = Lazy::new(|| RwLock::new(HashMap::new()));
