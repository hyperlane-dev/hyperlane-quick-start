use super::*;

/// Represents a CI/CD pipeline data transfer object.
#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct PipelineDto {
    /// The id.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) id: i32,
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
    #[get(pub(crate), type(copy))]
    /// The created at.
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) created_at: Option<i64>,
    #[get(pub(crate), type(copy))]
    /// The updated at.
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) updated_at: Option<i64>,
}

/// Represents a CI/CD pipeline run data transfer object.
#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct RunDto {
    /// The id.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) id: i32,
    /// The pipeline id.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) pipeline_id: i32,
    /// The pipeline name.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) pipeline_name: Option<String>,
    /// The run number.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) run_number: i32,
    /// The status.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) status: CicdStatus,
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
    #[get(pub(crate), type(copy))]
    /// The started at.
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) started_at: Option<i64>,
    #[get(pub(crate), type(copy))]
    /// The completed at.
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) completed_at: Option<i64>,
    /// The duration ms.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) duration_ms: i32,
    #[get(pub(crate), type(copy))]
    /// The created at.
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) created_at: Option<i64>,
}

/// Represents a CI/CD job data transfer object with step information.
#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct JobDto {
    /// The id.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) id: i32,
    /// The run id.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) run_id: i32,
    /// The name.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) name: String,
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
    #[get(pub(crate), type(copy))]
    /// The started at.
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) started_at: Option<i64>,
    #[get(pub(crate), type(copy))]
    /// The completed at.
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) completed_at: Option<i64>,
    /// The duration ms.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) duration_ms: i32,
}

/// Represents a CI/CD step data transfer object with log output.
#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct StepDto {
    /// The id.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) id: i32,
    /// The job id.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) job_id: i32,
    /// The name.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) name: String,
    /// The command.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) command: Option<String>,
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
    #[get(pub(crate), type(copy))]
    /// The started at.
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) started_at: Option<i64>,
    #[get(pub(crate), type(copy))]
    /// The completed at.
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) completed_at: Option<i64>,
    /// The duration ms.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) duration_ms: i32,
}

/// Represents a detailed pipeline response with associated runs.
#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct PipelineDetailDto {
    /// The pipeline.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) pipeline: PipelineDto,
    /// The runs.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) runs: Vec<RunDto>,
}

/// Represents a detailed run response with associated jobs.
#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct RunDetailDto {
    /// The run.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) run: RunDto,
    /// The jobs.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) jobs: Vec<JobWithStepsDto>,
}

/// Represents a job response with its associated steps.
#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct JobWithStepsDto {
    /// The job.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) job: JobDto,
    /// The steps.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) steps: Vec<StepDto>,
}

/// Represents a paginated list of pipeline run data transfer objects.
#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct PaginatedRunsDto {
    /// The total.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) total: i32,
    /// The runs.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) runs: Vec<RunDto>,
    /// The has more.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) has_more: bool,
}

/// Represents a step log data transfer object with output content.
#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct StepLogDto {
    /// The step id.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) step_id: i32,
    /// The step name.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) step_name: String,
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
    /// The output length.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) output_length: usize,
    /// The new output.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) new_output: Option<String>,
    /// The output offset.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) output_offset: usize,
    /// The stderr output.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) stderr_output: Option<String>,
    /// The stderr length.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) stderr_length: usize,
    /// The new stderr.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) new_stderr: Option<String>,
    /// The stderr offset.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) stderr_offset: usize,
}

/// incremental run detail dto.
#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct IncrementalRunDetailDto {
    /// The run.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) run: RunDto,
    /// The jobs.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) jobs: Vec<JobWithIncrementalStepsDto>,
}

/// job with incremental steps dto.
#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct JobWithIncrementalStepsDto {
    /// The job.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) job: JobDto,
    /// The steps.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) steps: Vec<StepLogDto>,
}
