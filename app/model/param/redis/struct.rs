use super::*;

#[derive(Debug, Clone, Default, Serialize, Deserialize, ToSchema, Data)]
pub struct RedisRecord {
    key: String,
    value: String,
}
