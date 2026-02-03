use super::*;

#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct PipelineDto {
    #[get(type(copy), pub)]
    pub(super) id: i32,
    #[get(pub)]
    pub(super) name: String,
    #[get(pub)]
    pub(super) description: Option<String>,
    #[get(pub)]
    pub(super) config_content: Option<String>,
    #[get(pub)]
    pub(super) created_at: Option<String>,
    #[get(pub)]
    pub(super) updated_at: Option<String>,
}

#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct RunDto {
    #[get(type(copy), pub)]
    pub(super) id: i32,
    #[get(type(copy), pub)]
    pub(super) pipeline_id: i32,
    #[get(pub)]
    pub(super) pipeline_name: Option<String>,
    #[get(type(copy), pub)]
    pub(super) run_number: i32,
    #[get(pub)]
    pub(super) status: CicdStatus,
    #[get(pub)]
    pub(super) triggered_by: Option<String>,
    #[get(pub)]
    pub(super) commit_hash: Option<String>,
    #[get(pub)]
    pub(super) commit_message: Option<String>,
    #[get(pub)]
    pub(super) started_at: Option<String>,
    #[get(pub)]
    pub(super) completed_at: Option<String>,
    #[get(type(copy), pub)]
    pub(super) duration_ms: i32,
    #[get(pub)]
    pub(super) created_at: Option<String>,
}

#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct JobDto {
    #[get(type(copy), pub)]
    pub(super) id: i32,
    #[get(type(copy), pub)]
    pub(super) run_id: i32,
    #[get(pub)]
    pub(super) name: String,
    #[get(pub)]
    pub(super) status: CicdStatus,
    #[get(pub)]
    pub(super) runner: Option<String>,
    #[get(pub)]
    pub(super) started_at: Option<String>,
    #[get(pub)]
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
    #[get(pub)]
    pub(super) name: String,
    #[get(pub)]
    pub(super) command: Option<String>,
    #[get(pub)]
    pub(super) status: CicdStatus,
    #[get(pub)]
    pub(super) output: Option<String>,
    #[get(pub)]
    pub(super) dockerfile: Option<String>,
    #[get(pub)]
    pub(super) image: Option<String>,
    #[get(pub)]
    pub(super) started_at: Option<String>,
    #[get(pub)]
    pub(super) completed_at: Option<String>,
    #[get(type(copy), pub)]
    pub(super) duration_ms: i32,
}

#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct PipelineDetailDto {
    #[get(pub)]
    pub(super) pipeline: PipelineDto,
    #[get(pub)]
    pub(super) runs: Vec<RunDto>,
}

#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct RunDetailDto {
    #[get(pub)]
    pub(super) run: RunDto,
    #[get(pub)]
    pub(super) jobs: Vec<JobWithStepsDto>,
}

#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct JobWithStepsDto {
    #[get(pub)]
    pub(super) job: JobDto,
    #[get(pub)]
    pub(super) steps: Vec<StepDto>,
}

#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct PaginatedRunsDto {
    #[get(type(copy), pub)]
    pub(super) total: i32,
    #[get(pub)]
    pub(super) runs: Vec<RunDto>,
    #[get(type(copy), pub)]
    pub(super) has_more: bool,
}

#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct StepLogDto {
    #[get(type(copy), pub)]
    pub(super) step_id: i32,
    #[get(pub)]
    pub(super) step_name: String,
    #[get(pub)]
    pub(super) status: CicdStatus,
    #[get(pub)]
    pub(super) output: Option<String>,
    #[get(type(copy), pub)]
    pub(super) output_length: usize,
    #[get(pub)]
    pub(super) new_output: Option<String>,
    #[get(type(copy), pub)]
    pub(super) output_offset: usize,
}

#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct IncrementalRunDetailDto {
    #[get(pub)]
    pub(super) run: RunDto,
    #[get(pub)]
    pub(super) jobs: Vec<JobWithIncrementalStepsDto>,
}

#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct JobWithIncrementalStepsDto {
    #[get(pub)]
    pub(super) job: JobDto,
    #[get(pub)]
    pub(super) steps: Vec<StepLogDto>,
}
