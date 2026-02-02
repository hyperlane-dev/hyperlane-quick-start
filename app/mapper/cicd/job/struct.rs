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
#[sea_orm(table_name = "cicd_job")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    #[get(type(copy), pub(crate))]
    pub(super) id: i32,
    #[get(type(copy), pub(crate))]
    pub(super) run_id: i32,
    #[get(pub(crate))]
    pub(super) name: String,
    #[get(pub(crate))]
    pub(super) status: String,
    #[get(pub(crate))]
    pub(super) runner: Option<String>,
    #[get(pub(crate))]
    pub(super) started_at: Option<NaiveDateTime>,
    #[get(pub(crate))]
    pub(super) completed_at: Option<NaiveDateTime>,
    #[get(type(copy), pub(crate))]
    pub(super) duration_ms: i32,
    #[get(pub(crate))]
    pub(super) created_at: Option<NaiveDateTime>,
    #[get(pub(crate))]
    pub(super) updated_at: Option<NaiveDateTime>,
}
