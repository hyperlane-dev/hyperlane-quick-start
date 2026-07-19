use super::*;

/// SeaORM entity model for the `cicd_run` table, representing a single execution run of a CI/CD pipeline.
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
    /// Unique primary key identifier for the run.
    #[sea_orm(primary_key, auto_increment = true)]
    #[get(type(copy))]
    pub(super) id: i32,
    /// The foreign key referencing the pipeline this run belongs to.
    #[get(type(copy))]
    pub(super) pipeline_id: i32,
    /// The sequential run number within the pipeline.
    #[get(type(copy))]
    pub(super) run_number: i32,
    /// The current status of the run (e.g., "pending", "running", "success", "failed").
    pub(super) status: String,
    /// The optional username or identifier of who triggered the run.
    pub(super) triggered_by: Option<String>,
    /// The optional Git commit hash associated with the run.
    pub(super) commit_hash: Option<String>,
    /// The optional Git commit message associated with the run.
    pub(super) commit_message: Option<String>,
    /// The timestamp when the run started execution.
    pub(super) started_at: Option<NaiveDateTime>,
    /// The timestamp when the run completed execution.
    pub(super) completed_at: Option<NaiveDateTime>,
    /// The total duration of the run in milliseconds.
    #[get(type(copy))]
    pub(super) duration_ms: i32,
    /// The timestamp when the run record was created.
    pub(super) created_at: Option<NaiveDateTime>,
    /// The timestamp when the run record was last updated.
    pub(super) updated_at: Option<NaiveDateTime>,
}

/// Data access object for CI/CD run, used for transferring run data with string-formatted timestamps.
#[derive(Clone, Data, Debug, Default, Deserialize, Serialize)]
pub struct CicdRunDao {
    /// Unique identifier for the run.
    #[get(type(copy))]
    pub(super) id: i32,
    /// The foreign key referencing the pipeline this run belongs to.
    #[get(type(copy))]
    pub(super) pipeline_id: i32,
    /// The sequential run number within the pipeline.
    #[get(type(copy))]
    pub(super) run_number: i32,
    /// The current status of the run.
    pub(super) status: String,
    /// The trigger type for the run (e.g., "manual", "push", "schedule").
    pub(super) trigger_type: String,
    /// The optional username or identifier of who triggered the run.
    pub(super) triggered_by: Option<String>,
    /// The optional Git commit hash associated with the run.
    pub(super) commit_hash: Option<String>,
    /// The optional Git commit message associated with the run.
    pub(super) commit_message: Option<String>,
    /// The string-formatted timestamp when the run started execution.
    pub(super) started_at: Option<String>,
    /// The string-formatted timestamp when the run completed execution.
    pub(super) completed_at: Option<String>,
    /// The total duration of the run in milliseconds.
    #[get(type(copy))]
    pub(super) duration_ms: i32,
    /// The string-formatted timestamp when the run record was created.
    pub(super) created_at: Option<String>,
    /// The string-formatted timestamp when the run record was last updated.
    pub(super) updated_at: Option<String>,
}
