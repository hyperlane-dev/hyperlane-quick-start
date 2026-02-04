use super::*;

#[derive(Clone, Data, Debug, Default, Deserialize, Serialize)]
pub struct MysqlRecordDao {
    #[get(pub(crate))]
    pub(super) key: String,
    #[get(pub(crate))]
    pub(super) value: String,
}

#[derive(
    Clone,
    Data,
    Debug,
    Default,
    DeriveActiveModelBehavior,
    DeriveEntityModel,
    Deserialize,
    PartialEq,
    Serialize,
)]
#[sea_orm(table_name = "record", schema_name = "public")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    #[get(type(copy), pub(crate))]
    pub(super) id: i32,
    #[sea_orm(unique)]
    #[get(pub(crate))]
    pub(super) key: String,
    #[get(pub(crate))]
    pub(super) value: String,
}
