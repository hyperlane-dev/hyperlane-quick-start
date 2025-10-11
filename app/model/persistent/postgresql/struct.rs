use super::*;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PostgresqlRecordDao {
    pub key: String,
    pub value: String,
}

#[derive(
    Clone,
    Debug,
    PartialEq,
    Serialize,
    Deserialize,
    Default,
    DeriveEntityModel,
    DeriveActiveModelBehavior,
)]
#[sea_orm(table_name = "postgresql_record", schema_name = "public")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    pub id: i32,
    #[sea_orm(unique)]
    pub key: String,
    pub value: String,
}
