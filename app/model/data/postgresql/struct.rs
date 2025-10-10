use super::*;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PostgresqlRecordDao {
    pub key: String,
    pub value: String,
}
