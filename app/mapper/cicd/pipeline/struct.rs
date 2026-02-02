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
#[sea_orm(table_name = "cicd_pipeline")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    #[get(type(copy), pub(crate))]
    pub(super) id: i32,
    #[get(pub(crate))]
    pub(super) name: String,
    #[get(pub(crate))]
    pub(super) description: Option<String>,
    #[get(pub(crate))]
    pub(super) config_content: Option<String>,
    #[get(pub(crate))]
    pub(super) created_at: Option<NaiveDateTime>,
    #[get(pub(crate))]
    pub(super) updated_at: Option<NaiveDateTime>,
}
