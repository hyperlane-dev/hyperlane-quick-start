use super::*;

pub static MYSQL_CONNECTIONS: Lazy<RwLock<HashMap<String, ConnectionCache<DatabaseConnection>>>> =
    Lazy::new(|| RwLock::new(HashMap::new()));
