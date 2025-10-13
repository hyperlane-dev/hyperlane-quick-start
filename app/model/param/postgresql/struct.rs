use super::*;

#[derive(Debug, Clone, Default, Serialize, Deserialize, ToSchema, Data)]
pub struct PostgresqlRecord {
    key: String,
    value: String,
}
