use super::*;

pub static MYSQL_DB: Lazy<Pool<MySql>> =
    Lazy::new(|| block_on(async { connection_mysql_db().await }));
