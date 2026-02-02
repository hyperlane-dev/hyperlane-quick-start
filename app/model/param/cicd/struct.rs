use super::*;

#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct CreatePipelineParam {
    pub name: String,
    pub description: Option<String>,
    pub config_content: Option<String>,
}

#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct TriggerRunParam {
    pub pipeline_id: i32,
    pub triggered_by: Option<String>,
    pub commit_hash: Option<String>,
    pub commit_message: Option<String>,
}

#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct UpdateJobStatusParam {
    pub job_id: i32,
    pub status: CicdStatus,
    pub runner: Option<String>,
}

#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct UpdateStepStatusParam {
    pub step_id: i32,
    pub status: CicdStatus,
    pub output: Option<String>,
}

#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct QueryRunsParam {
    pub pipeline_id: Option<i32>,
    pub status: Option<CicdStatus>,
    pub page_size: Option<i32>,
    pub last_id: Option<i32>,
}
