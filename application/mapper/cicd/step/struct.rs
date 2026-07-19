use super::*;

/// SeaORM entity model for the `cicd_step` table, representing an individual step within a CI/CD job.
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
    /// Unique primary key identifier for the step.
    #[sea_orm(primary_key, auto_increment = true)]
    #[get(type(copy))]
    pub(super) id: i32,
    /// The foreign key referencing the job this step belongs to.
    #[get(type(copy))]
    pub(super) job_id: i32,
    /// The name of the step.
    pub(super) name: String,
    /// The optional shell command to execute in this step.
    pub(super) command: Option<String>,
    /// The current status of the step (e.g., "pending", "running", "success", "failed").
    pub(super) status: String,
    /// The optional captured output from the step execution.
    pub(super) output: Option<String>,
    /// The optional Dockerfile content used for building the step.
    pub(super) dockerfile: Option<String>,
    /// The optional Docker image name used for running the step.
    pub(super) image: Option<String>,
    /// The timestamp when the step started execution.
    pub(super) started_at: Option<NaiveDateTime>,
    /// The timestamp when the step completed execution.
    pub(super) completed_at: Option<NaiveDateTime>,
    /// The duration of the step execution in milliseconds.
    #[get(type(copy))]
    pub(super) duration_ms: i32,
    /// The timestamp when the step record was created.
    pub(super) created_at: Option<NaiveDateTime>,
    /// The timestamp when the step record was last updated.
    pub(super) updated_at: Option<NaiveDateTime>,
}

/// Data access object for CI/CD step, used for transferring step data with string-formatted timestamps.
#[derive(Clone, Data, Debug, Default, Deserialize, Serialize)]
pub struct CicdStepDao {
    /// Unique identifier for the step.
    #[get(type(copy))]
    pub(super) id: i32,
    /// The foreign key referencing the job this step belongs to.
    #[get(type(copy))]
    pub(super) job_id: i32,
    /// The name of the step.
    pub(super) name: String,
    /// The optional shell command to execute in this step.
    pub(super) command: Option<String>,
    /// The current status of the step.
    pub(super) status: String,
    /// The optional captured output from the step execution.
    pub(super) output: Option<String>,
    /// The string-formatted timestamp when the step started execution.
    pub(super) started_at: Option<String>,
    /// The string-formatted timestamp when the step completed execution.
    pub(super) completed_at: Option<String>,
    /// The duration of the step execution in milliseconds.
    #[get(type(copy))]
    pub(super) duration_ms: i32,
    /// The string-formatted timestamp when the step record was created.
    pub(super) created_at: Option<String>,
    /// The string-formatted timestamp when the step record was last updated.
    pub(super) updated_at: Option<String>,
}
