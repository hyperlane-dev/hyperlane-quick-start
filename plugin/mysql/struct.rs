use super::*;

/// Plugin for managing MySQL database connections and auto-creation.
#[derive(Clone, Copy, Data, Debug, Default)]
pub struct MySqlPlugin;

/// Auto-creation handler for MySQL databases, managing database and table creation for a specific instance.
#[derive(Clone, Data, Debug, New)]
pub struct MySqlAutoCreation {
    /// The MySQL instance configuration for this auto-creation handler.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) instance: MySqlInstanceConfig,
    /// The database schema containing table, index, and constraint definitions.
    #[new(skip)]
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) schema: DatabaseSchema,
}
