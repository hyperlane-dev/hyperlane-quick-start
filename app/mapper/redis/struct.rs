use super::*;

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct RedisRecordDao {
    pub key: String,
    pub value: String,
}
