use super::*;

#[derive(Debug, Clone, Default, Serialize, Deserialize, ToSchema, Data)]
pub struct MysqlRecord {
    key: String,
    value: String,
}
