use super::*;

#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct PipelineDto {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub config_content: Option<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct RunDto {
    pub id: i32,
    pub pipeline_id: i32,
    pub pipeline_name: Option<String>,
    pub run_number: i32,
    pub status: CicdStatus,
    pub triggered_by: Option<String>,
    pub commit_hash: Option<String>,
    pub commit_message: Option<String>,
    pub started_at: Option<String>,
    pub completed_at: Option<String>,
    pub duration_ms: i32,
    pub created_at: Option<String>,
}

#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct JobDto {
    pub id: i32,
    pub run_id: i32,
    pub name: String,
    pub status: CicdStatus,
    pub runner: Option<String>,
    pub started_at: Option<String>,
    pub completed_at: Option<String>,
    pub duration_ms: i32,
}

#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct StepDto {
    pub id: i32,
    pub job_id: i32,
    pub name: String,
    pub command: Option<String>,
    pub status: CicdStatus,
    pub output: Option<String>,
    pub dockerfile: Option<String>,
    pub image: Option<String>,
    pub started_at: Option<String>,
    pub completed_at: Option<String>,
    pub duration_ms: i32,
}

#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct PipelineDetailDto {
    pub pipeline: PipelineDto,
    pub runs: Vec<RunDto>,
}

#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct RunDetailDto {
    pub run: RunDto,
    pub jobs: Vec<JobWithStepsDto>,
}

#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct JobWithStepsDto {
    pub job: JobDto,
    pub steps: Vec<StepDto>,
}

#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct PaginatedRunsDto {
    pub total: i32,
    pub page: i32,
    pub page_size: i32,
    pub runs: Vec<RunDto>,
}
