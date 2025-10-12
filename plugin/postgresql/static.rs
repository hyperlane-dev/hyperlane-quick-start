use super::*;

pub static POSTGRESQL_DB: Lazy<Result<DatabaseConnection, String>> =
    Lazy::new(|| block_on(async { connection_postgresql_db().await }));
