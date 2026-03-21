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
#[sea_orm(table_name = "cicd_run")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    #[get(type(copy))]
    pub(super) id: i32,
    #[get(type(copy))]
    pub(super) pipeline_id: i32,
    #[get(type(copy))]
    pub(super) run_number: i32,
    pub(super) status: String,
    pub(super) triggered_by: Option<String>,
    pub(super) commit_hash: Option<String>,
    pub(super) commit_message: Option<String>,
    pub(super) started_at: Option<NaiveDateTime>,
    pub(super) completed_at: Option<NaiveDateTime>,
    #[get(type(copy))]
    pub(super) duration_ms: i32,
    pub(super) created_at: Option<NaiveDateTime>,
    pub(super) updated_at: Option<NaiveDateTime>,
}

#[derive(Clone, Data, Debug, Default, Deserialize, Serialize)]
pub struct CicdRunDao {
    #[get(type(copy))]
    pub(super) id: i32,
    #[get(type(copy))]
    pub(super) pipeline_id: i32,
    #[get(type(copy))]
    pub(super) run_number: i32,
    pub(super) status: String,
    pub(super) trigger_type: String,
    pub(super) triggered_by: Option<String>,
    pub(super) commit_hash: Option<String>,
    pub(super) commit_message: Option<String>,
    pub(super) started_at: Option<String>,
    pub(super) completed_at: Option<String>,
    #[get(type(copy))]
    pub(super) duration_ms: i32,
    pub(super) created_at: Option<String>,
    pub(super) updated_at: Option<String>,
}
