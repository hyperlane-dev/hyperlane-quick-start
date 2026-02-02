use super::*;

#[derive(Clone, Data, Debug, Default, Deserialize, Serialize)]
pub struct CicdPipeline {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub config_content: Option<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

#[derive(Clone, Data, Debug, Default, Deserialize, Serialize)]
pub struct CicdRun {
    pub id: i32,
    pub pipeline_id: i32,
    pub run_number: i32,
    pub status: String,
    pub triggered_by: Option<String>,
    pub commit_hash: Option<String>,
    pub commit_message: Option<String>,
    pub started_at: Option<String>,
    pub completed_at: Option<String>,
    pub duration_ms: i32,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

#[derive(Clone, Data, Debug, Default, Deserialize, Serialize)]
pub struct CicdJob {
    pub id: i32,
    pub run_id: i32,
    pub name: String,
    pub status: String,
    pub runner: Option<String>,
    pub started_at: Option<String>,
    pub completed_at: Option<String>,
    pub duration_ms: i32,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

#[derive(Clone, Data, Debug, Default, Deserialize, Serialize)]
pub struct CicdStep {
    pub id: i32,
    pub job_id: i32,
    pub name: String,
    pub command: Option<String>,
    pub status: String,
    pub output: Option<String>,
    pub started_at: Option<String>,
    pub completed_at: Option<String>,
    pub duration_ms: i32,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

use std::collections::HashMap;

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PipelineConfig {
    pub name: Option<String>,
    pub jobs: HashMap<String, JobConfig>,
    pub dockerfile: Option<String>,
    pub image: Option<String>,
    pub context: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct JobConfig {
    #[serde(rename = "runs-on")]
    pub runs_on: Option<String>,
    pub steps: Vec<StepConfig>,
    pub dockerfile: Option<String>,
    pub image: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct StepConfig {
    pub name: String,
    pub run: Option<String>,
    pub dockerfile: Option<String>,
}
