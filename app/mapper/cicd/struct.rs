use super::*;

#[derive(Clone, Data, Debug, Default, Deserialize, Serialize)]
pub struct CicdPipelineDao {
    #[get(type(copy), pub(crate))]
    pub(super) id: i32,
    #[get(pub(crate))]
    pub(super) name: String,
    #[get(pub(crate))]
    pub(super) description: Option<String>,
    #[get(pub(crate))]
    pub(super) repository_url: Option<String>,
    #[get(pub(crate))]
    pub(super) branch: String,
    #[get(pub(crate))]
    pub(super) config_content: Option<String>,
    #[get(pub(crate))]
    pub(super) trigger_type: String,
    #[get(type(copy), pub(crate))]
    pub(super) is_active: bool,
    #[get(pub(crate))]
    pub(super) created_at: Option<String>,
    #[get(pub(crate))]
    pub(super) updated_at: Option<String>,
}

#[derive(Clone, Data, Debug, Default, Deserialize, Serialize)]
pub struct CicdRunDao {
    #[get(type(copy), pub(crate))]
    pub(super) id: i32,
    #[get(type(copy), pub(crate))]
    pub(super) pipeline_id: i32,
    #[get(type(copy), pub(crate))]
    pub(super) run_number: i32,
    #[get(pub(crate))]
    pub(super) status: String,
    #[get(pub(crate))]
    pub(super) trigger_type: String,
    #[get(pub(crate))]
    pub(super) triggered_by: Option<String>,
    #[get(pub(crate))]
    pub(super) commit_hash: Option<String>,
    #[get(pub(crate))]
    pub(super) commit_message: Option<String>,
    #[get(pub(crate))]
    pub(super) started_at: Option<String>,
    #[get(pub(crate))]
    pub(super) completed_at: Option<String>,
    #[get(type(copy), pub(crate))]
    pub(super) duration_ms: i32,
    #[get(pub(crate))]
    pub(super) created_at: Option<String>,
    #[get(pub(crate))]
    pub(super) updated_at: Option<String>,
}

#[derive(Clone, Data, Debug, Default, Deserialize, Serialize)]
pub struct CicdJobDao {
    #[get(type(copy), pub(crate))]
    pub(super) id: i32,
    #[get(type(copy), pub(crate))]
    pub(super) run_id: i32,
    #[get(pub(crate))]
    pub(super) name: String,
    #[get(pub(crate))]
    pub(super) status: String,
    #[get(pub(crate))]
    pub(super) runner: Option<String>,
    #[get(pub(crate))]
    pub(super) started_at: Option<String>,
    #[get(pub(crate))]
    pub(super) completed_at: Option<String>,
    #[get(type(copy), pub(crate))]
    pub(super) duration_ms: i32,
    #[get(pub(crate))]
    pub(super) created_at: Option<String>,
    #[get(pub(crate))]
    pub(super) updated_at: Option<String>,
}

#[derive(Clone, Data, Debug, Default, Deserialize, Serialize)]
pub struct CicdStepDao {
    #[get(type(copy), pub(crate))]
    pub(super) id: i32,
    #[get(type(copy), pub(crate))]
    pub(super) job_id: i32,
    #[get(pub(crate))]
    pub(super) name: String,
    #[get(pub(crate))]
    pub(super) command: Option<String>,
    #[get(pub(crate))]
    pub(super) status: String,
    #[get(pub(crate))]
    pub(super) output: Option<String>,
    #[get(pub(crate))]
    pub(super) started_at: Option<String>,
    #[get(pub(crate))]
    pub(super) completed_at: Option<String>,
    #[get(type(copy), pub(crate))]
    pub(super) duration_ms: i32,
    #[get(pub(crate))]
    pub(super) created_at: Option<String>,
    #[get(pub(crate))]
    pub(super) updated_at: Option<String>,
}
