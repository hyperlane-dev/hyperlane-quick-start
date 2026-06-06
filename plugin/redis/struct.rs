use super::*;

/// Plugin for managing Redis connections and auto-creation.
#[derive(Clone, Copy, Data, Debug, Default)]
pub struct RedisPlugin;

/// Auto-creation handler for Redis instances, managing server validation and namespace setup.
#[derive(Clone, Data, Debug, New)]
pub struct RedisAutoCreation {
    /// The Redis instance configuration for this auto-creation handler.
    pub(super) instance: RedisInstanceConfig,
    /// The database schema (unused for Redis but required by the trait).
    #[new(skip)]
    pub(super) schema: DatabaseSchema,
}
