use super::*;

#[derive(Clone, Data, Debug, Default, Deserialize, Serialize)]
pub struct RedisRecordDao {
    #[get(pub(crate))]
    pub(crate) key: String,
    #[get(pub(crate))]
    pub(crate) value: String,
}
