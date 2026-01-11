use super::*;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PostgresqlRecordDao {
    pub key: String,
    pub value: String,
}

#[derive(
    Clone,
    Data,
    Debug,
    Default,
    PartialEq,
    Serialize,
    Deserialize,
    DeriveEntityModel,
    DeriveActiveModelBehavior,
)]
#[sea_orm(table_name = "record", schema_name = "public")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    pub id: i32,
    #[sea_orm(unique)]
    pub key: String,
    pub value: String,
}
