use super::*;

#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct RedisRecord {
    key: String,
    value: String,
}
