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
    #[get(type(copy))]
    pub(super) id: i32,
    pub(super) name: String,
    pub(super) description: Option<String>,
    pub(super) config_content: Option<String>,
    pub(super) created_at: Option<NaiveDateTime>,
    pub(super) updated_at: Option<NaiveDateTime>,
}

#[derive(Clone, Data, Debug, Default, Deserialize, Serialize)]
pub struct CicdPipelineDao {
    #[get(type(copy))]
    pub(super) id: i32,
    pub(super) name: String,
    pub(super) description: Option<String>,
    pub(super) repository_url: Option<String>,
    pub(super) branch: String,
    pub(super) config_content: Option<String>,
    pub(super) trigger_type: String,
    #[get(type(copy))]
    pub(super) is_active: bool,
    pub(super) created_at: Option<String>,
    pub(super) updated_at: Option<String>,
}
