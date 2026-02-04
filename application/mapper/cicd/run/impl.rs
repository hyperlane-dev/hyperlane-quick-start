use super::*;

impl ActiveModel {
    #[instrument_trace]
    pub fn new(
        pipeline_id: i32,
        run_number: i32,
        triggered_by: Option<String>,
        commit_hash: Option<String>,
        commit_message: Option<String>,
    ) -> Self {
        Self {
            pipeline_id: ActiveValue::Set(pipeline_id),
            run_number: ActiveValue::Set(run_number),
            status: ActiveValue::Set("pending".to_string()),
            triggered_by: ActiveValue::Set(triggered_by),
            commit_hash: ActiveValue::Set(commit_hash),
            commit_message: ActiveValue::Set(commit_message),
            id: ActiveValue::NotSet,
            started_at: ActiveValue::NotSet,
            completed_at: ActiveValue::NotSet,
            duration_ms: ActiveValue::Set(0),
            created_at: ActiveValue::NotSet,
            updated_at: ActiveValue::NotSet,
        }
    }
}
