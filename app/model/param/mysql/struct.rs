use super::*;

#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct MysqlRecord {
    key: String,
    value: String,
}
