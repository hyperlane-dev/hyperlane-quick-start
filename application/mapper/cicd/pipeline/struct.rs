use super::*;

/// SeaORM entity model for the `cicd_pipeline` table, representing a CI/CD pipeline configuration.
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
    /// Unique primary key identifier for the pipeline.
    #[sea_orm(primary_key, auto_increment = true)]
    #[get(type(copy))]
    pub(super) id: i32,
    /// The name of the pipeline.
    pub(super) name: String,
    /// The optional description of the pipeline.
    pub(super) description: Option<String>,
    /// The optional YAML/JSON configuration content of the pipeline.
    pub(super) config_content: Option<String>,
    /// The timestamp when the pipeline record was created.
    pub(super) created_at: Option<NaiveDateTime>,
    /// The timestamp when the pipeline record was last updated.
    pub(super) updated_at: Option<NaiveDateTime>,
}

/// Data access object for CI/CD pipeline, used for transferring pipeline data with string-formatted timestamps.
#[derive(Clone, Data, Debug, Default, Deserialize, Serialize)]
pub struct CicdPipelineDao {
    /// Unique identifier for the pipeline.
    #[get(type(copy))]
    pub(super) id: i32,
    /// The name of the pipeline.
    pub(super) name: String,
    /// The optional description of the pipeline.
    pub(super) description: Option<String>,
    /// The optional repository URL associated with the pipeline.
    pub(super) repository_url: Option<String>,
    /// The branch name the pipeline is configured for.
    pub(super) branch: String,
    /// The optional YAML/JSON configuration content of the pipeline.
    pub(super) config_content: Option<String>,
    /// The trigger type for the pipeline (e.g., "manual", "push", "schedule").
    pub(super) trigger_type: String,
    /// Flag indicating whether the pipeline is currently active.
    #[get(type(copy))]
    pub(super) is_active: bool,
    /// The string-formatted timestamp when the pipeline record was created.
    pub(super) created_at: Option<String>,
    /// The string-formatted timestamp when the pipeline record was last updated.
    pub(super) updated_at: Option<String>,
}
