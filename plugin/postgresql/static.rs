use super::*;

pub static POSTGRESQL_CONNECTIONS: Lazy<
    RwLock<HashMap<String, ConnectionCache<DatabaseConnection>>>,
> = Lazy::new(|| RwLock::new(HashMap::new()));
