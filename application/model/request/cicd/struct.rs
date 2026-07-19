use super::*;

/// create pipeline param.
#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct CreatePipelineParam {
    /// The name.
    pub(super) name: String,
    /// The description.
    pub(super) description: Option<String>,
    /// The config content.
    pub(super) config_content: Option<String>,
}

/// Represents a request to trigger a new CI/CD pipeline run.
#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct TriggerRunParam {
    /// The pipeline id.
    #[get(type(copy))]
    pub(super) pipeline_id: i32,
    /// The triggered by.
    pub(super) triggered_by: Option<String>,
    /// The commit hash.
    pub(super) commit_hash: Option<String>,
    /// The commit message.
    pub(super) commit_message: Option<String>,
}

/// update job status param.
#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct UpdateJobStatusParam {
    /// The job id.
    #[get(type(copy))]
    pub(super) job_id: i32,
    /// The status.
    pub(super) status: CicdStatus,
    /// The runner.
    pub(super) runner: Option<String>,
}

/// update step status param.
#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct UpdateStepStatusParam {
    /// The step id.
    #[get(type(copy))]
    pub(super) step_id: i32,
    /// The status.
    pub(super) status: CicdStatus,
    /// The output.
    pub(super) output: Option<String>,
}

/// Represents query parameters for filtering pipeline runs.
#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct QueryRunsParam {
    /// The pipeline id.
    #[get(type(copy))]
    pub(super) pipeline_id: Option<i32>,
    /// The status.
    #[get(type(copy))]
    pub(super) status: Option<CicdStatus>,
    /// The page size.
    #[get(type(copy))]
    pub(super) page_size: Option<i32>,
    /// The last id.
    #[get(type(copy))]
    pub(super) last_id: Option<i32>,
}

/// incremental run detail param.
#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct IncrementalRunDetailParam {
    /// The run id.
    #[get(type(copy))]
    pub(super) run_id: i32,
    /// The step offsets.
    pub(super) step_offsets: Vec<StepOffsetParam>,
}

/// Represents a step offset parameter for streaming step logs.
#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct StepOffsetParam {
    /// The step id.
    #[get(type(copy))]
    pub(super) step_id: i32,
    /// The offset.
    #[get(type(copy))]
    pub(super) offset: usize,
    /// The stderr offset.
    #[serde(default)]
    #[get(type(copy))]
    pub(super) stderr_offset: usize,
}
