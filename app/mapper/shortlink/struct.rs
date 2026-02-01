use super::*;

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct ShortlinkDao {
    pub id: i32,
    pub url: String,
    pub created_at: String,
}

#[derive(
    Clone,
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
    pub id: i32,
    pub url: String,
    pub created_at: Option<NaiveDateTime>,
}
