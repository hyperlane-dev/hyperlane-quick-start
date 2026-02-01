use super::*;

#[derive(Clone, Data, Debug, Default, Deserialize, Serialize)]
pub struct CicdPipelineDao {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub repository_url: Option<String>,
    pub branch: String,
    pub config_content: Option<String>,
    pub trigger_type: String,
    pub is_active: bool,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

#[derive(Clone, Data, Debug, Default, Deserialize, Serialize)]
pub struct CicdRunDao {
    pub id: i32,
    pub pipeline_id: i32,
    pub run_number: i32,
    pub status: String,
    pub trigger_type: String,
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
pub struct CicdJobDao {
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
pub struct CicdStepDao {
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
