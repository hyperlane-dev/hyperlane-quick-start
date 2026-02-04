use super::*;

pub(super) static TRACKING_DB_CONNECTION: OnceLock<DatabaseConnection> = OnceLock::new();
