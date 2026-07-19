use super::*;

/// SeaORM entity model for the `cicd_job` table, representing a CI/CD job within a pipeline run.
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
    /// Unique primary key identifier for the job.
    #[sea_orm(primary_key, auto_increment = true)]
    #[get(type(copy))]
    pub(super) id: i32,
    /// The foreign key referencing the pipeline run this job belongs to.
    #[get(type(copy))]
    pub(super) run_id: i32,
    /// The name of the job.
    pub(super) name: String,
    /// The current status of the job (e.g., "pending", "running", "success", "failed").
    pub(super) status: String,
    /// The optional runner identifier that executed the job.
    pub(super) runner: Option<String>,
    /// The timestamp when the job started execution.
    pub(super) started_at: Option<NaiveDateTime>,
    /// The timestamp when the job completed execution.
    pub(super) completed_at: Option<NaiveDateTime>,
    /// The duration of the job execution in milliseconds.
    #[get(type(copy))]
    pub(super) duration_ms: i32,
    /// The timestamp when the job record was created.
    pub(super) created_at: Option<NaiveDateTime>,
    /// The timestamp when the job record was last updated.
    pub(super) updated_at: Option<NaiveDateTime>,
}

/// Data access object for CI/CD job, used for transferring job data with string-formatted timestamps.
#[derive(Clone, Data, Debug, Default, Deserialize, Serialize)]
pub struct CicdJobDao {
    /// Unique identifier for the job.
    #[get(type(copy))]
    pub(super) id: i32,
    /// The foreign key referencing the pipeline run this job belongs to.
    #[get(type(copy))]
    pub(super) run_id: i32,
    /// The name of the job.
    pub(super) name: String,
    /// The current status of the job.
    pub(super) status: String,
    /// The optional runner identifier that executed the job.
    pub(super) runner: Option<String>,
    /// The string-formatted timestamp when the job started execution.
    pub(super) started_at: Option<String>,
    /// The string-formatted timestamp when the job completed execution.
    pub(super) completed_at: Option<String>,
    /// The duration of the job execution in milliseconds.
    #[get(type(copy))]
    pub(super) duration_ms: i32,
    /// The string-formatted timestamp when the job record was created.
    pub(super) created_at: Option<String>,
    /// The string-formatted timestamp when the job record was last updated.
    pub(super) updated_at: Option<String>,
}
