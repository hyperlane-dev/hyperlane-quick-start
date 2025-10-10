use super::*;

#[derive(Debug, Clone, Default, Serialize, Deserialize, ToSchema)]
pub struct PostgresqlRecord {
    pub key: String,
    pub value: String,
}
