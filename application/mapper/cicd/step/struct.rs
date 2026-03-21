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
#[sea_orm(table_name = "cicd_step")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    #[get(type(copy))]
    pub(super) id: i32,
    #[get(type(copy))]
    pub(super) job_id: i32,
    pub(super) name: String,
    pub(super) command: Option<String>,
    pub(super) status: String,
    pub(super) output: Option<String>,
    pub(super) dockerfile: Option<String>,
    pub(super) image: Option<String>,
    pub(super) started_at: Option<NaiveDateTime>,
    pub(super) completed_at: Option<NaiveDateTime>,
    #[get(type(copy))]
    pub(super) duration_ms: i32,
    pub(super) created_at: Option<NaiveDateTime>,
    pub(super) updated_at: Option<NaiveDateTime>,
}

#[derive(Clone, Data, Debug, Default, Deserialize, Serialize)]
pub struct CicdStepDao {
    #[get(type(copy))]
    pub(super) id: i32,
    #[get(type(copy))]
    pub(super) job_id: i32,
    pub(super) name: String,
    pub(super) command: Option<String>,
    pub(super) status: String,
    pub(super) output: Option<String>,
    pub(super) started_at: Option<String>,
    pub(super) completed_at: Option<String>,
    #[get(type(copy))]
    pub(super) duration_ms: i32,
    pub(super) created_at: Option<String>,
    pub(super) updated_at: Option<String>,
}
