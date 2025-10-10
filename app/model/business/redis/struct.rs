use super::*;

#[derive(Debug, Clone, Default, Serialize, Deserialize, ToSchema)]
pub struct RedisRecord {
    pub key: String,
    pub value: String,
}
