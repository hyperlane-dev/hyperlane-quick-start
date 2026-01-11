use super::*;

#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct PostgresqlRecord {
    key: String,
    value: String,
}
