use super::*;

#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct RedisRecord {
    pub(super) key: String,
    pub(super) value: String,
}
