use super::*;

/// Plugin for managing PostgreSQL database connections and auto-creation.
#[derive(Clone, Copy, Data, Debug, Default)]
pub struct PostgreSqlPlugin;

/// Auto-creation handler for PostgreSQL databases, managing database and table creation for a specific instance.
#[derive(Clone, Data, Debug, New)]
pub struct PostgreSqlAutoCreation {
    /// The PostgreSQL instance configuration for this auto-creation handler.
    pub(super) instance: PostgreSqlInstanceConfig,
    /// The database schema containing table, index, and constraint definitions.
    #[new(skip)]
    pub(super) schema: DatabaseSchema,
}
