use super::*;

impl ActiveModel {
    #[instrument_trace]
    pub fn new(run_id: i32, name: String) -> Self {
        Self {
            run_id: ActiveValue::Set(run_id),
            name: ActiveValue::Set(name),
            status: ActiveValue::Set("pending".to_string()),
            id: ActiveValue::NotSet,
            runner: ActiveValue::NotSet,
            started_at: ActiveValue::NotSet,
            completed_at: ActiveValue::NotSet,
            duration_ms: ActiveValue::Set(0),
            created_at: ActiveValue::NotSet,
            updated_at: ActiveValue::NotSet,
        }
    }
}
