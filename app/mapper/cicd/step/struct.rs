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
    pub id: i32,
    pub job_id: i32,
    pub name: String,
    pub command: Option<String>,
    pub status: String,
    pub output: Option<String>,
    pub dockerfile: Option<String>,
    pub image: Option<String>,
    pub started_at: Option<NaiveDateTime>,
    pub completed_at: Option<NaiveDateTime>,
    pub duration_ms: i32,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

pub type StepEntity = Entity;
pub type StepActiveModel = ActiveModel;
pub type StepColumn = Column;
