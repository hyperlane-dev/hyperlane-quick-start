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
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) id: i32,
    /// The name of the pipeline.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) name: String,
    /// The optional description of the pipeline.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) description: Option<String>,
    /// The optional YAML/JSON configuration content of the pipeline.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) config_content: Option<String>,
    /// The timestamp when the pipeline record was created.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) created_at: Option<NaiveDateTime>,
    /// The timestamp when the pipeline record was last updated.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) updated_at: Option<NaiveDateTime>,
}

/// Data access object for CI/CD pipeline, used for transferring pipeline data with string-formatted timestamps.
#[derive(Clone, Data, Debug, Default, Deserialize, Serialize)]
pub struct CicdPipelineDao {
    /// Unique identifier for the pipeline.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) id: i32,
    /// The name of the pipeline.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) name: String,
    /// The optional description of the pipeline.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) description: Option<String>,
    /// The optional repository URL associated with the pipeline.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) repository_url: Option<String>,
    /// The branch name the pipeline is configured for.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) branch: String,
    /// The optional YAML/JSON configuration content of the pipeline.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) config_content: Option<String>,
    /// The trigger type for the pipeline (e.g., "manual", "push", "schedule").
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) trigger_type: String,
    /// Flag indicating whether the pipeline is currently active.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) is_active: bool,
    /// The string-formatted timestamp when the pipeline record was created.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) created_at: Option<String>,
    /// The string-formatted timestamp when the pipeline record was last updated.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) updated_at: Option<String>,
}
