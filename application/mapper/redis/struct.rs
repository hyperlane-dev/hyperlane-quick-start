use super::*;

#[derive(Clone, Data, Debug, Default, Deserialize, Serialize)]
pub struct RedisRecordDao {
    pub(crate) key: String,
    pub(crate) value: String,
}
