use super::*;

#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct CreatePipelineParam {
    #[get(pub)]
    pub(super) name: String,
    #[get(pub)]
    pub(super) description: Option<String>,
    #[get(pub)]
    pub(super) config_content: Option<String>,
}

#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct TriggerRunParam {
    #[get(type(copy), pub)]
    pub(super) pipeline_id: i32,
    #[get(pub)]
    pub(super) triggered_by: Option<String>,
    #[get(pub)]
    pub(super) commit_hash: Option<String>,
    #[get(pub)]
    pub(super) commit_message: Option<String>,
}

#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct UpdateJobStatusParam {
    #[get(type(copy), pub)]
    pub(super) job_id: i32,
    #[get(pub)]
    pub(super) status: CicdStatus,
    #[get(pub)]
    pub(super) runner: Option<String>,
}

#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct UpdateStepStatusParam {
    #[get(type(copy), pub)]
    pub(super) step_id: i32,
    #[get(pub)]
    pub(super) status: CicdStatus,
    #[get(pub)]
    pub(super) output: Option<String>,
}

#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct QueryRunsParam {
    #[get(type(copy), pub)]
    pub(super) pipeline_id: Option<i32>,
    #[get(type(copy), pub)]
    pub(super) status: Option<CicdStatus>,
    #[get(type(copy), pub)]
    pub(super) page_size: Option<i32>,
    #[get(type(copy), pub)]
    pub(super) last_id: Option<i32>,
}

#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct IncrementalRunDetailParam {
    #[get(type(copy), pub)]
    pub(super) run_id: i32,
    #[get(pub)]
    pub(super) step_offsets: Vec<StepOffsetParam>,
}

#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct StepOffsetParam {
    #[get(type(copy), pub)]
    pub(super) step_id: i32,
    #[get(type(copy), pub)]
    pub(super) offset: usize,
}
