use super::*;

/// redis record dao.
#[derive(Clone, Data, Debug, Default, Deserialize, Serialize)]
pub struct RedisRecordDao {
    /// The key.
    pub(crate) key: String,
    /// The value.
    pub(crate) value: String,
}
