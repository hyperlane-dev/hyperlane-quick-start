use super::*;

pub static MYSQL_DB: Lazy<Result<DatabaseConnection, String>> =
    Lazy::new(|| block_on(async { connection_mysql_db().await }));
