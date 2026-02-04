use super::*;

#[derive(Clone, Data, Debug, Default, Deserialize, Serialize)]
pub struct CicdPipeline {
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

#[derive(Clone, Data, Debug, Default, Deserialize, Serialize)]
pub struct CicdRun {
    #[get(type(copy), pub)]
    pub(super) id: i32,
    #[get(type(copy), pub)]
    pub(super) pipeline_id: i32,
    #[get(type(copy), pub)]
    pub(super) run_number: i32,
    #[get(pub)]
    pub(super) status: String,
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
    #[get(pub)]
    pub(super) updated_at: Option<String>,
}

#[derive(Clone, Data, Debug, Default, Deserialize, Serialize)]
pub struct CicdJob {
    #[get(type(copy), pub)]
    pub(super) id: i32,
    #[get(type(copy), pub)]
    pub(super) run_id: i32,
    #[get(pub)]
    pub(super) name: String,
    #[get(pub)]
    pub(super) status: String,
    #[get(pub)]
    pub(super) runner: Option<String>,
    #[get(pub)]
    pub(super) started_at: Option<String>,
    #[get(pub)]
    pub(super) completed_at: Option<String>,
    #[get(type(copy), pub)]
    pub(super) duration_ms: i32,
    #[get(pub)]
    pub(super) created_at: Option<String>,
    #[get(pub)]
    pub(super) updated_at: Option<String>,
}

#[derive(Clone, Data, Debug, Default, Deserialize, Serialize)]
pub struct CicdStep {
    #[get(type(copy), pub)]
    pub(super) id: i32,
    #[get(type(copy), pub)]
    pub(super) job_id: i32,
    #[get(pub)]
    pub(super) name: String,
    #[get(pub)]
    pub(super) command: Option<String>,
    #[get(pub)]
    pub(super) status: String,
    #[get(pub)]
    pub(super) output: Option<String>,
    #[get(pub)]
    pub(super) started_at: Option<String>,
    #[get(pub)]
    pub(super) completed_at: Option<String>,
    #[get(type(copy), pub)]
    pub(super) duration_ms: i32,
    #[get(pub)]
    pub(super) created_at: Option<String>,
    #[get(pub)]
    pub(super) updated_at: Option<String>,
}

use std::collections::HashMap;

#[derive(Clone, Data, Debug, Default, Deserialize, Serialize)]
pub struct PipelineConfig {
    #[get(pub)]
    pub(super) name: Option<String>,
    #[get(pub)]
    pub(super) jobs: HashMap<String, JobConfig>,
    #[get(pub)]
    pub(super) dockerfile: Option<String>,
    #[get(pub)]
    pub(super) image: Option<String>,
    #[get(pub)]
    pub(super) context: Option<String>,
}

#[derive(Clone, Data, Debug, Default, Deserialize, Serialize)]
pub struct JobConfig {
    #[serde(rename = "runs-on")]
    #[get(pub)]
    pub(super) runs_on: Option<String>,
    #[get(pub)]
    pub(super) steps: Vec<StepConfig>,
    #[get(pub)]
    pub(super) dockerfile: Option<String>,
    #[get(pub)]
    pub(super) image: Option<String>,
}

#[derive(Clone, Data, Debug, Default, Deserialize, Serialize)]
pub struct StepConfig {
    #[get(pub)]
    pub(super) name: String,
    #[get(pub)]
    pub(super) run: Option<String>,
    #[get(pub)]
    pub(super) dockerfile: Option<String>,
}
