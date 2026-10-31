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
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) id: i32,
    /// The foreign key referencing the pipeline this run belongs to.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) pipeline_id: i32,
    /// The sequential run number within the pipeline.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) run_number: i32,
    /// The current status of the run (e.g., "pending", "running", "success", "failed").
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) status: String,
    /// The optional username or identifier of who triggered the run.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) triggered_by: Option<String>,
    /// The optional Git commit hash associated with the run.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) commit_hash: Option<String>,
    /// The optional Git commit message associated with the run.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) commit_message: Option<String>,
    /// The timestamp when the run started execution.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) started_at: Option<NaiveDateTime>,
    /// The timestamp when the run completed execution.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) completed_at: Option<NaiveDateTime>,
    /// The total duration of the run in milliseconds.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) duration_ms: i32,
    /// The timestamp when the run record was created.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) created_at: Option<NaiveDateTime>,
    /// The timestamp when the run record was last updated.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) updated_at: Option<NaiveDateTime>,
}

/// Data access object for CI/CD run, used for transferring run data with string-formatted timestamps.
#[derive(Clone, Data, Debug, Default, Deserialize, Serialize)]
pub struct CicdRunDao {
    /// Unique identifier for the run.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) id: i32,
    /// The foreign key referencing the pipeline this run belongs to.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) pipeline_id: i32,
    /// The sequential run number within the pipeline.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) run_number: i32,
    /// The current status of the run.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) status: String,
    /// The trigger type for the run (e.g., "manual", "push", "schedule").
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) trigger_type: String,
    /// The optional username or identifier of who triggered the run.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) triggered_by: Option<String>,
    /// The optional Git commit hash associated with the run.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) commit_hash: Option<String>,
    /// The optional Git commit message associated with the run.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) commit_message: Option<String>,
    /// The string-formatted timestamp when the run started execution.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) started_at: Option<String>,
    /// The string-formatted timestamp when the run completed execution.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) completed_at: Option<String>,
    /// The total duration of the run in milliseconds.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) duration_ms: i32,
    /// The string-formatted timestamp when the run record was created.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) created_at: Option<String>,
    /// The string-formatted timestamp when the run record was last updated.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) updated_at: Option<String>,
}
