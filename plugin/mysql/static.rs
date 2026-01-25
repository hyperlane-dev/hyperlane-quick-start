use super::*;

pub static MYSQL_CONNECTIONS: Lazy<RwLock<HashMap<String, Result<DatabaseConnection, String>>>> =
    Lazy::new(|| RwLock::new(HashMap::new()));
