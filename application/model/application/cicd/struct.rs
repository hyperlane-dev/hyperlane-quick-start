use super::*;

#[derive(Clone, Data, Debug, Default, Deserialize, Serialize)]
pub struct CicdPipeline {
    #[get(type(copy), pub)]
    pub(super) id: i32,
    pub(super) name: String,
    pub(super) description: Option<String>,
    pub(super) config_content: Option<String>,
    pub(super) created_at: Option<String>,
    pub(super) updated_at: Option<String>,
}

#[derive(Clone, Data, Debug, Default, Deserialize, Serialize)]
pub struct CicdRun {
    #[get(type(copy), pub)]
    pub(super) id: i32,
    #[get(type(copy), pub)]
    pub(super) pipeline_id: i32,
    #[get(type(copy), pub)]
    pub(super) run_number: i32,
    pub(super) status: String,
    pub(super) triggered_by: Option<String>,
    pub(super) commit_hash: Option<String>,
    pub(super) commit_message: Option<String>,
    pub(super) started_at: Option<String>,
    pub(super) completed_at: Option<String>,
    #[get(type(copy), pub)]
    pub(super) duration_ms: i32,
    pub(super) created_at: Option<String>,
    pub(super) updated_at: Option<String>,
}

#[derive(Clone, Data, Debug, Default, Deserialize, Serialize)]
pub struct CicdJob {
    #[get(type(copy), pub)]
    pub(super) id: i32,
    #[get(type(copy), pub)]
    pub(super) run_id: i32,
    pub(super) name: String,
    pub(super) status: String,
    pub(super) runner: Option<String>,
    pub(super) started_at: Option<String>,
    pub(super) completed_at: Option<String>,
    #[get(type(copy), pub)]
    pub(super) duration_ms: i32,
    pub(super) created_at: Option<String>,
    pub(super) updated_at: Option<String>,
}

#[derive(Clone, Data, Debug, Default, Deserialize, Serialize)]
pub struct CicdStep {
    #[get(type(copy), pub)]
    pub(super) id: i32,
    #[get(type(copy), pub)]
    pub(super) job_id: i32,
    pub(super) name: String,
    pub(super) command: Option<String>,
    pub(super) status: String,
    pub(super) output: Option<String>,
    pub(super) started_at: Option<String>,
    pub(super) completed_at: Option<String>,
    #[get(type(copy), pub)]
    pub(super) duration_ms: i32,
    pub(super) created_at: Option<String>,
    pub(super) updated_at: Option<String>,
}

use std::collections::HashMap;

#[derive(Clone, Data, Debug, Default, Deserialize, Serialize)]
pub struct PipelineConfig {
    pub(super) name: Option<String>,
    pub(super) jobs: HashMap<String, JobConfig>,
}

#[derive(Clone, Data, Debug, Default, Deserialize, Serialize)]
pub struct JobConfig {
    #[serde(rename = "runs-on")]
    pub(super) runs_on: Option<String>,
    pub(super) steps: Vec<StepConfig>,
}

#[derive(Clone, Data, Debug, Default, Deserialize, Serialize)]
pub struct StepConfig {
    pub(super) name: String,
    pub(super) run: Option<String>,
}
