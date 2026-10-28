use super::*;

/// create pipeline param.
#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct CreatePipelineParam {
    /// The name.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) name: String,
    /// The description.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) description: Option<String>,
    /// The config content.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) config_content: Option<String>,
}

/// Represents a request to trigger a new CI/CD pipeline run.
#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct TriggerRunParam {
    /// The pipeline id.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) pipeline_id: i32,
    /// The triggered by.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) triggered_by: Option<String>,
    /// The commit hash.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) commit_hash: Option<String>,
    /// The commit message.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) commit_message: Option<String>,
}

/// update job status param.
#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct UpdateJobStatusParam {
    /// The job id.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) job_id: i32,
    /// The status.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) status: CicdStatus,
    /// The runner.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) runner: Option<String>,
}

/// update step status param.
#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct UpdateStepStatusParam {
    /// The step id.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) step_id: i32,
    /// The status.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) status: CicdStatus,
    /// The output.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) output: Option<String>,
}

/// Represents query parameters for filtering pipeline runs.
#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct QueryRunsParam {
    /// The pipeline id.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) pipeline_id: Option<i32>,
    /// The status.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) status: Option<CicdStatus>,
    /// The page size.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) page_size: Option<i32>,
    /// The last id.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) last_id: Option<i32>,
}

/// incremental run detail param.
#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct IncrementalRunDetailParam {
    /// The run id.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) run_id: i32,
    /// The step offsets.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) step_offsets: Vec<StepOffsetParam>,
}

/// Represents a step offset parameter for streaming step logs.
#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct StepOffsetParam {
    /// The step id.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) step_id: i32,
    /// The offset.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) offset: usize,
    /// The stderr offset.
    #[serde(default)]
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) stderr_offset: usize,
}
