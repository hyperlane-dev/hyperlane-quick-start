use super::*;
use hyperlane_config::framework::database::*;
use std::sync::Arc;

pub struct PostgresUserRepository {
    pub pool: Arc<DatabaseConnectionPool>,
}
