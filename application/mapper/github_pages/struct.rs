use super::*;

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
#[sea_orm(table_name = "github_pages", schema_name = "public")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    #[get(type(copy))]
    pub(super) id: i32,
    pub(super) owner: String,
    pub(super) repository: String,
    pub(super) base_url: String,
    pub(super) last_synced_at: Option<NaiveDateTime>,
    pub(super) created_at: Option<NaiveDateTime>,
}
