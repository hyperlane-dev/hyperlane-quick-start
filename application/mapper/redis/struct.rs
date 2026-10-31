use super::*;

/// redis record dao.
#[derive(Clone, Data, Debug, Default, Deserialize, Serialize)]
pub struct RedisRecordDao {
    /// The key.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) key: String,
    /// The value.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) value: String,
}
