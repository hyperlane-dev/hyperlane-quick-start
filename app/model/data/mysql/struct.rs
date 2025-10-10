use super::*;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MysqlRecordDao {
    pub key: String,
    pub value: String,
}
