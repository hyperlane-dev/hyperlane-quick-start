use super::*;

/// Plugin for managing Redis connections and auto-creation.
#[derive(Clone, Copy, Data, Debug, Default)]
pub struct RedisPlugin;

/// Auto-creation handler for Redis instances, managing server validation and namespace setup.
#[derive(Clone, Data, Debug, New)]
pub struct RedisAutoCreation {
    /// The Redis instance configuration for this auto-creation handler.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) instance: RedisInstanceConfig,
    /// The database schema (unused for Redis but required by the trait).
    #[new(skip)]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub schema: DatabaseSchema,
}
