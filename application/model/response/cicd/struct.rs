use super::*;

#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct PipelineDto {
    #[get(type(copy), pub)]
    pub(super) id: i32,
    pub(super) name: String,
    pub(super) description: Option<String>,
    pub(super) config_content: Option<String>,
    pub(super) created_at: Option<String>,
    pub(super) updated_at: Option<String>,
}

#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct RunDto {
    #[get(type(copy), pub)]
    pub(super) id: i32,
    #[get(type(copy), pub)]
    pub(super) pipeline_id: i32,
    pub(super) pipeline_name: Option<String>,
    #[get(type(copy), pub)]
    pub(super) run_number: i32,
    pub(super) status: CicdStatus,
    pub(super) triggered_by: Option<String>,
    pub(super) commit_hash: Option<String>,
    pub(super) commit_message: Option<String>,
    pub(super) started_at: Option<String>,
    pub(super) completed_at: Option<String>,
    #[get(type(copy), pub)]
    pub(super) duration_ms: i32,
    pub(super) created_at: Option<String>,
}

#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct JobDto {
    #[get(type(copy), pub)]
    pub(super) id: i32,
    #[get(type(copy), pub)]
    pub(super) run_id: i32,
    pub(super) name: String,
    pub(super) status: CicdStatus,
    pub(super) runner: Option<String>,
    pub(super) started_at: Option<String>,
    pub(super) completed_at: Option<String>,
    #[get(type(copy), pub)]
    pub(super) duration_ms: i32,
}

#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct StepDto {
    #[get(type(copy), pub)]
    pub(super) id: i32,
    #[get(type(copy), pub)]
    pub(super) job_id: i32,
    pub(super) name: String,
    pub(super) command: Option<String>,
    pub(super) status: CicdStatus,
    pub(super) output: Option<String>,
    pub(super) started_at: Option<String>,
    pub(super) completed_at: Option<String>,
    #[get(type(copy), pub)]
    pub(super) duration_ms: i32,
}

#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct PipelineDetailDto {
    pub(super) pipeline: PipelineDto,
    pub(super) runs: Vec<RunDto>,
}

#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct RunDetailDto {
    pub(super) run: RunDto,
    pub(super) jobs: Vec<JobWithStepsDto>,
}

#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct JobWithStepsDto {
    pub(super) job: JobDto,
    pub(super) steps: Vec<StepDto>,
}

#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct PaginatedRunsDto {
    #[get(type(copy), pub)]
    pub(super) total: i32,
    pub(super) runs: Vec<RunDto>,
    #[get(type(copy), pub)]
    pub(super) has_more: bool,
}

#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct StepLogDto {
    #[get(type(copy), pub)]
    pub(super) step_id: i32,
    pub(super) step_name: String,
    pub(super) status: CicdStatus,
    pub(super) output: Option<String>,
    #[get(type(copy), pub)]
    pub(super) output_length: usize,
    pub(super) new_output: Option<String>,
    #[get(type(copy), pub)]
    pub(super) output_offset: usize,
    pub(super) stderr_output: Option<String>,
    #[get(type(copy), pub)]
    pub(super) stderr_length: usize,
    pub(super) new_stderr: Option<String>,
    #[get(type(copy), pub)]
    pub(super) stderr_offset: usize,
}

#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct IncrementalRunDetailDto {
    pub(super) run: RunDto,
    pub(super) jobs: Vec<JobWithIncrementalStepsDto>,
}

#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct JobWithIncrementalStepsDto {
    pub(super) job: JobDto,
    pub(super) steps: Vec<StepLogDto>,
}
