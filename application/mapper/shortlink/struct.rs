use super::*;

#[derive(Clone, Data, Debug, Default, Deserialize, Serialize)]
pub struct ShortlinkDao {
    #[get(type(copy), pub(crate))]
    pub(super) id: i32,
    #[get(pub(crate))]
    pub(super) url: String,
    #[get(pub(crate))]
    pub(super) created_at: String,
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
#[sea_orm(table_name = "shortlink", schema_name = "public")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    #[get(type(copy), pub(crate))]
    pub(super) id: i32,
    #[get(pub(crate))]
    pub(super) url: String,
    #[get(pub(crate))]
    pub(super) created_at: Option<NaiveDateTime>,
}
