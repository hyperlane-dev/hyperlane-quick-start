use super::*;

pub static POSTGRESQL_DB: Lazy<Pool<Postgres>> =
    Lazy::new(|| block_on(async { connection_postgresql_db().await }));
