use super::*;

/// Represents a Redis key-value record for creation and update operations.
#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct RedisRecord {
    /// The key.
    pub(super) key: String,
    /// The value.
    pub(super) value: String,
}
