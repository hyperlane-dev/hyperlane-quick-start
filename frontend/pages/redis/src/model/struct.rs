use super::*;

/// One key value record of the cache.
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[serde(default)]
pub struct RedisRecord {
    /// The key of the record.
    pub key: String,
    /// The value of the record.
    pub value: String,
}

/// The body of a record the visitor writes.
#[derive(Clone, Debug, Default, Serialize)]
pub struct RedisDraft {
    /// The key of the record.
    pub key: String,
    /// The value of the record.
    pub value: String,
}
