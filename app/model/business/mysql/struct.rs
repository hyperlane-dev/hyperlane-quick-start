use super::*;

#[derive(Debug, Clone, Default, Serialize, Deserialize, ToSchema)]
pub struct MysqlRecord {
    pub key: String,
    pub value: String,
}
