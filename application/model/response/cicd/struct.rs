use super::*;

/// Represents a CI/CD pipeline data transfer object.
#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct PipelineDto {
    /// The id.
    #[get(type(copy))]
    pub(super) id: i32,
    /// The name.
    pub(super) name: String,
    /// The description.
    pub(super) description: Option<String>,
    /// The config content.
    pub(super) config_content: Option<String>,
    /// The created at.
    pub(super) created_at: Option<i64>,
    /// The updated at.
    pub(super) updated_at: Option<i64>,
}

/// Represents a CI/CD pipeline run data transfer object.
#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct RunDto {
    /// The id.
    #[get(type(copy))]
    pub(super) id: i32,
    /// The pipeline id.
    #[get(type(copy))]
    pub(super) pipeline_id: i32,
    /// The pipeline name.
    pub(super) pipeline_name: Option<String>,
    /// The run number.
    #[get(type(copy))]
    pub(super) run_number: i32,
    /// The status.
    pub(super) status: CicdStatus,
    /// The triggered by.
    pub(super) triggered_by: Option<String>,
    /// The commit hash.
    pub(super) commit_hash: Option<String>,
    /// The commit message.
    pub(super) commit_message: Option<String>,
    /// The started at.
    pub(super) started_at: Option<i64>,
    /// The completed at.
    pub(super) completed_at: Option<i64>,
    /// The duration ms.
    #[get(type(copy))]
    pub(super) duration_ms: i32,
    /// The created at.
    pub(super) created_at: Option<i64>,
}

/// Represents a CI/CD job data transfer object with step information.
#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct JobDto {
    /// The id.
    #[get(type(copy))]
    pub(super) id: i32,
    /// The run id.
    #[get(type(copy))]
    pub(super) run_id: i32,
    /// The name.
    pub(super) name: String,
    /// The status.
    pub(super) status: CicdStatus,
    /// The runner.
    pub(super) runner: Option<String>,
    /// The started at.
    pub(super) started_at: Option<i64>,
    /// The completed at.
    pub(super) completed_at: Option<i64>,
    /// The duration ms.
    #[get(type(copy))]
    pub(super) duration_ms: i32,
}

/// Represents a CI/CD step data transfer object with log output.
#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct StepDto {
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
    pub(super) status: CicdStatus,
    /// The output.
    pub(super) output: Option<String>,
    /// The started at.
    pub(super) started_at: Option<i64>,
    /// The completed at.
    pub(super) completed_at: Option<i64>,
    /// The duration ms.
    #[get(type(copy))]
    pub(super) duration_ms: i32,
}

/// Represents a detailed pipeline response with associated runs.
#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct PipelineDetailDto {
    /// The pipeline.
    pub(super) pipeline: PipelineDto,
    /// The runs.
    pub(super) runs: Vec<RunDto>,
}

/// Represents a detailed run response with associated jobs.
#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct RunDetailDto {
    /// The run.
    pub(super) run: RunDto,
    /// The jobs.
    pub(super) jobs: Vec<JobWithStepsDto>,
}

/// Represents a job response with its associated steps.
#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct JobWithStepsDto {
    /// The job.
    pub(super) job: JobDto,
    /// The steps.
    pub(super) steps: Vec<StepDto>,
}

/// Represents a paginated list of pipeline run data transfer objects.
#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct PaginatedRunsDto {
    /// The total.
    #[get(type(copy))]
    pub(super) total: i32,
    /// The runs.
    pub(super) runs: Vec<RunDto>,
    /// The has more.
    #[get(type(copy))]
    pub(super) has_more: bool,
}

/// Represents a step log data transfer object with output content.
#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct StepLogDto {
    /// The step id.
    #[get(type(copy))]
    pub(super) step_id: i32,
    /// The step name.
    pub(super) step_name: String,
    /// The status.
    pub(super) status: CicdStatus,
    /// The output.
    pub(super) output: Option<String>,
    /// The output length.
    #[get(type(copy))]
    pub(super) output_length: usize,
    /// The new output.
    pub(super) new_output: Option<String>,
    /// The output offset.
    #[get(type(copy))]
    pub(super) output_offset: usize,
    /// The stderr output.
    pub(super) stderr_output: Option<String>,
    /// The stderr length.
    #[get(type(copy))]
    pub(super) stderr_length: usize,
    /// The new stderr.
    pub(super) new_stderr: Option<String>,
    /// The stderr offset.
    #[get(type(copy))]
    pub(super) stderr_offset: usize,
}

/// incremental run detail dto.
#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct IncrementalRunDetailDto {
    /// The run.
    pub(super) run: RunDto,
    /// The jobs.
    pub(super) jobs: Vec<JobWithIncrementalStepsDto>,
}

/// job with incremental steps dto.
#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct JobWithIncrementalStepsDto {
    /// The job.
    pub(super) job: JobDto,
    /// The steps.
    pub(super) steps: Vec<StepLogDto>,
}
