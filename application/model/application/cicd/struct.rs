use super::*;

/// Application-level model representing a CI/CD pipeline with string-formatted timestamps.
#[derive(Clone, Data, Debug, Default, Deserialize, Serialize)]
pub struct CicdPipeline {
    /// The unique identifier for the pipeline.
    #[get(type(copy))]
    pub(super) id: i32,
    /// The name of the pipeline.
    pub(super) name: String,
    /// The optional description of the pipeline.
    pub(super) description: Option<String>,
    /// The optional YAML/JSON configuration content of the pipeline.
    pub(super) config_content: Option<String>,
    /// The string-formatted timestamp when the pipeline was created.
    pub(super) created_at: Option<String>,
    /// The string-formatted timestamp when the pipeline was last updated.
    pub(super) updated_at: Option<String>,
}

/// Application-level model representing a CI/CD pipeline run with string-formatted timestamps.
#[derive(Clone, Data, Debug, Default, Deserialize, Serialize)]
pub struct CicdRun {
    /// The id.
    #[get(type(copy))]
    pub(super) id: i32,
    /// The pipeline id.
    #[get(type(copy))]
    pub(super) pipeline_id: i32,
    /// The run number.
    #[get(type(copy))]
    pub(super) run_number: i32,
    /// The status.
    pub(super) status: String,
    /// The triggered by.
    pub(super) triggered_by: Option<String>,
    /// The commit hash.
    pub(super) commit_hash: Option<String>,
    /// The commit message.
    pub(super) commit_message: Option<String>,
    /// The started at.
    pub(super) started_at: Option<String>,
    /// The completed at.
    pub(super) completed_at: Option<String>,
    /// The duration ms.
    #[get(type(copy))]
    pub(super) duration_ms: i32,
    /// The created at.
    pub(super) created_at: Option<String>,
    /// The updated at.
    pub(super) updated_at: Option<String>,
}

/// Application-level model representing a CI/CD job within a run with string-formatted timestamps.
#[derive(Clone, Data, Debug, Default, Deserialize, Serialize)]
pub struct CicdJob {
    /// The id.
    #[get(type(copy))]
    pub(super) id: i32,
    /// The run id.
    #[get(type(copy))]
    pub(super) run_id: i32,
    /// The name.
    pub(super) name: String,
    /// The status.
    pub(super) status: String,
    /// The runner.
    pub(super) runner: Option<String>,
    /// The started at.
    pub(super) started_at: Option<String>,
    /// The completed at.
    pub(super) completed_at: Option<String>,
    /// The duration ms.
    #[get(type(copy))]
    pub(super) duration_ms: i32,
    /// The created at.
    pub(super) created_at: Option<String>,
    /// The updated at.
    pub(super) updated_at: Option<String>,
}

/// Application-level model representing a CI/CD step within a job with string-formatted timestamps.
#[derive(Clone, Data, Debug, Default, Deserialize, Serialize)]
pub struct CicdStep {
    /// The id.
    #[get(type(copy))]
    pub(super) id: i32,
    /// The job id.
    #[get(type(copy))]
    pub(super) job_id: i32,
    /// The name.
    pub(super) name: String,
    /// The command.
    pub(super) command: Option<String>,
    /// The status.
    pub(super) status: String,
    /// The output.
    pub(super) output: Option<String>,
    /// The started at.
    pub(super) started_at: Option<String>,
    /// The completed at.
    pub(super) completed_at: Option<String>,
    /// The duration ms.
    #[get(type(copy))]
    pub(super) duration_ms: i32,
    /// The created at.
    pub(super) created_at: Option<String>,
    /// The updated at.
    pub(super) updated_at: Option<String>,
}

/// Parsed pipeline configuration deserialized from YAML/JSON, containing the pipeline name and job definitions.
#[derive(Clone, Data, Debug, Default, Deserialize, Serialize)]
pub struct PipelineConfig {
    /// The name.
    pub(super) name: Option<String>,
    /// The jobs.
    pub(super) jobs: HashMap<String, JobConfig>,
}

/// Configuration for a single CI/CD job, including the runner type and ordered list of steps.
#[derive(Clone, Data, Debug, Default, Deserialize, Serialize)]
pub struct JobConfig {
    /// The runs on.
    #[serde(rename = "runs-on")]
    pub(super) runs_on: Option<String>,
    /// The steps.
    pub(super) steps: Vec<StepConfig>,
}

/// Configuration for a single CI/CD step, including its name and optional shell command to execute.
#[derive(Clone, Data, Debug, Default, Deserialize, Serialize)]
pub struct StepConfig {
    /// The name.
    pub(super) name: String,
    /// The run.
    pub(super) run: Option<String>,
}
