use super::*;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RedisRecordDao {
    pub key: String,
    pub value: String,
}
